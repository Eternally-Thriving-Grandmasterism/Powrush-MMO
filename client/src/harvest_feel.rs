/*!
 * Harvest Feel — soft RBE pool + pad rumble (v22.2.0)
 *
 * Take grows vitality. Tend grows harmony. Allocate spends a stack.
 * Rumble is confirmation, never punishment.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use std::time::Duration;

use bevy::input::gamepad::{GamepadRumbleIntensity, GamepadRumbleRequest};
use bevy::prelude::*;

use crate::lived_hour_support::RbeGlobalState;
use crate::rbe_allocate_choice::AllocatePath;

#[derive(Resource, Debug)]
pub struct SoftRbePool {
    pub vitality: f32,
    pub harmony: f32,
    pub joy: f32,
    pub last_credit: f32,
    pub harvests: u32,
    pub tends: u32,
}

impl Default for SoftRbePool {
    fn default() -> Self {
        Self {
            vitality: 0.0,
            harmony: 0.0,
            joy: 0.0,
            last_credit: 0.0,
            harvests: 0,
            tends: 0,
        }
    }
}

impl SoftRbePool {
    pub fn credit_mercy_harvest(&mut self, node_vitality: f32) -> f32 {
        let amount = (0.85 + node_vitality * 0.35).clamp(0.6, 1.4);
        self.vitality += amount;
        self.harmony += amount * 0.45;
        self.joy += amount * 0.30;
        self.last_credit = amount;
        self.harvests = self.harvests.saturating_add(1);
        amount
    }

    pub fn credit_tend(&mut self, node_vitality: f32) -> f32 {
        let amount = (0.28 + node_vitality * 0.12).clamp(0.22, 0.48);
        self.harmony += amount;
        self.joy += amount * 0.70;
        self.vitality += amount * 0.20;
        self.last_credit = amount;
        self.tends = self.tends.saturating_add(1);
        amount
    }

    pub fn spend_allocate(&mut self, path: AllocatePath, want: f32) -> f32 {
        match path {
            AllocatePath::FlowOutward => {
                let n = want.min(self.vitality.max(0.0));
                self.vitality = (self.vitality - n).max(0.0);
                n
            }
            AllocatePath::StewardReserve => {
                let n = want.min(self.harmony.max(0.0));
                self.harmony = (self.harmony - n).max(0.0);
                n
            }
        }
    }

    pub fn line(&self) -> String {
        format!(
            "Pool  vitality {:.1} · harmony {:.1} · joy {:.1}",
            self.vitality, self.harmony, self.joy
        )
    }
}

pub fn credit_soft_and_global(
    pool: &mut SoftRbePool,
    global: Option<&mut RbeGlobalState>,
    node_vitality: f32,
) -> f32 {
    let amount = pool.credit_mercy_harvest(node_vitality);
    if let Some(g) = global {
        g.total_abundance += amount;
        g.global_harmony_score = (g.global_harmony_score + 0.015).min(1.0);
    }
    amount
}

pub fn rumble_mercy_harvest(
    rumble: &mut EventWriter<GamepadRumbleRequest>,
    gamepads: &Gamepads,
) {
    for gamepad in gamepads.iter() {
        rumble.send(GamepadRumbleRequest::Add {
            gamepad,
            intensity: GamepadRumbleIntensity {
                strong_motor: 0.12,
                weak_motor: 0.28,
            },
            duration: Duration::from_millis(90),
        });
    }
}

pub struct HarvestFeelPlugin;

impl Plugin for HarvestFeelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SoftRbePool>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harvest_grows_pool_without_emptying() {
        let mut pool = SoftRbePool::default();
        let a = pool.credit_mercy_harvest(1.0);
        let b = pool.credit_mercy_harvest(0.5);
        assert!(a > 0.0 && b > 0.0);
        assert_eq!(pool.harvests, 2);
        assert!(pool.vitality > a);
    }

    #[test]
    fn tend_grows_harmony_more_than_vitality() {
        let mut pool = SoftRbePool::default();
        pool.credit_tend(1.0);
        assert!(pool.harmony > pool.vitality);
        assert_eq!(pool.tends, 1);
    }

    #[test]
    fn allocate_spends_the_named_stack() {
        let mut pool = SoftRbePool::default();
        pool.vitality = 3.0;
        pool.harmony = 2.0;
        let spent = pool.spend_allocate(AllocatePath::FlowOutward, 1.0);
        assert_eq!(spent, 1.0);
        assert!((pool.vitality - 2.0).abs() < 0.01);
    }
}
