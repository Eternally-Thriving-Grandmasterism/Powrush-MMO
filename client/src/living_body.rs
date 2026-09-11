/*!
 * Living Body — v22.13.0
 *
 * Breath, carry, shade. Grove / Heartwood rest the lungs.
 *
 * H-2026-09-11-B: the same breath / heavy / winded state now names a `BodyTell`
 * so Presentation can show it on the person (PERSON_READ_SPEC §4 Stance).
 * Reads only — no new sim write, no new verb.
 *
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::harvest_feel::SoftRbePool;
use crate::input::PlayerInput;
use crate::living_practice_loop::SoftPlayerRealm;

#[derive(Resource, Debug)]
pub struct LivingBody {
    pub breath: f32,
    pub heavy: bool,
    pub winded: bool,
    pub in_shade: bool,
}

impl Default for LivingBody {
    fn default() -> Self {
        Self {
            breath: 1.0,
            heavy: false,
            winded: false,
            in_shade: false,
        }
    }
}

/// What the lungs and the load are saying, named for the body rig.
///
/// Presentation only: the sim still owns `breath` / `heavy` / `winded`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BodyTell {
    /// Rested and unladen — stand tall.
    #[default]
    Easy,
    /// Carrying a full pool — chest folds down over the load.
    Heavy,
    /// Out of breath — deepest fold, slowest, biggest chest rise.
    Winded,
}

impl BodyTell {
    /// Waist bow (radians) the load pulls out of a standing person.
    pub fn waist_load(self) -> f32 {
        match self {
            BodyTell::Easy => 0.0,
            BodyTell::Heavy => 0.17,
            BodyTell::Winded => 0.29,
        }
    }

    /// Elbows-out flare (radians) so a loaded stance is not a straight pillar.
    pub fn arm_flare(self) -> f32 {
        match self {
            BodyTell::Easy => 0.06,
            BodyTell::Heavy => 0.22,
            BodyTell::Winded => 0.17,
        }
    }

    /// Knee give (metres) under the load.
    pub fn knee_give(self) -> f32 {
        match self {
            BodyTell::Easy => 0.0,
            BodyTell::Heavy => 0.035,
            BodyTell::Winded => 0.055,
        }
    }
}

impl LivingBody {
    pub fn carry_mul(&self) -> f32 {
        if self.heavy {
            0.82
        } else {
            1.0
        }
    }

    pub fn can_sprint(&self) -> bool {
        self.breath > 0.08 && !self.winded
    }

    /// Winded reads before heavy: empty lungs are the louder tell.
    pub fn tell(&self) -> BodyTell {
        if self.winded {
            BodyTell::Winded
        } else if self.heavy {
            BodyTell::Heavy
        } else {
            BodyTell::Easy
        }
    }

    /// Chest rise per breath (metres) — spent lungs heave, rested lungs barely move.
    pub fn breath_rise(&self) -> f32 {
        let spent = 1.0 - self.breath.clamp(0.0, 1.0);
        let base = 0.012 + spent * 0.030;
        if self.winded {
            base + 0.016
        } else {
            base
        }
    }

    /// Breath cycles per second — winded is slower and heavier, not faster.
    pub fn breath_rate(&self) -> f32 {
        if self.winded {
            0.62
        } else {
            0.95 - (1.0 - self.breath.clamp(0.0, 1.0)) * 0.22
        }
    }
}

pub struct LivingBodyPlugin;

impl Plugin for LivingBodyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivingBody>()
            .add_systems(Update, breathe_and_weigh);
    }
}

fn breathe_and_weigh(
    time: Res<Time>,
    input: Res<PlayerInput>,
    pool: Res<SoftRbePool>,
    realm: Res<SoftPlayerRealm>,
    mut body: ResMut<LivingBody>,
) {
    let dt = time.delta_seconds();
    body.heavy = pool.vitality >= 3.2;
    body.in_shade = matches!(realm.current, Some(0) | Some(2));
    let moving = input.movement.length_squared() > 0.04;
    let want_sprint = input.sprint && moving && body.can_sprint();
    if want_sprint {
        body.breath = (body.breath - dt * 0.28).max(0.0);
        if body.breath <= 0.08 {
            body.winded = true;
        }
    } else {
        let mut recover = if moving { 0.16 } else { 0.28 };
        if body.in_shade {
            recover *= 1.55;
        }
        body.breath = (body.breath + dt * recover).min(1.0);
        if body.breath >= 0.42 {
            body.winded = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winded_tell_beats_heavy_tell() {
        let easy = LivingBody::default();
        assert_eq!(easy.tell(), BodyTell::Easy);
        let heavy = LivingBody {
            heavy: true,
            ..Default::default()
        };
        assert_eq!(heavy.tell(), BodyTell::Heavy);
        let both = LivingBody {
            heavy: true,
            winded: true,
            breath: 0.04,
            ..Default::default()
        };
        assert_eq!(both.tell(), BodyTell::Winded);
    }

    #[test]
    fn load_folds_the_waist_in_named_steps() {
        assert_eq!(BodyTell::Easy.waist_load(), 0.0);
        assert!(BodyTell::Heavy.waist_load() > BodyTell::Easy.waist_load());
        assert!(BodyTell::Winded.waist_load() > BodyTell::Heavy.waist_load());
        assert!(BodyTell::Winded.knee_give() > BodyTell::Heavy.knee_give());
        assert!(BodyTell::Heavy.arm_flare() > BodyTell::Easy.arm_flare());
    }

    #[test]
    fn spent_lungs_heave_slower_and_deeper() {
        let rested = LivingBody::default();
        let tired = LivingBody {
            breath: 0.20,
            ..Default::default()
        };
        let winded = LivingBody {
            breath: 0.04,
            winded: true,
            ..Default::default()
        };
        assert!(tired.breath_rise() > rested.breath_rise());
        assert!(winded.breath_rise() > tired.breath_rise());
        assert!(winded.breath_rate() < tired.breath_rate());
        assert!(tired.breath_rate() < rested.breath_rate());
    }

    #[test]
    fn carry_and_sprint_gates_are_unchanged() {
        let heavy = LivingBody {
            heavy: true,
            ..Default::default()
        };
        assert!(heavy.carry_mul() < 1.0);
        assert!(LivingBody::default().can_sprint());
        let winded = LivingBody {
            breath: 0.05,
            winded: true,
            ..Default::default()
        };
        assert!(!winded.can_sprint());
    }
}
