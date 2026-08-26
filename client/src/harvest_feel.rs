/*!
 * Harvest Feel — soft RBE pool + pad rumble (v21.99.2)
 *
 * Client-side abundance pool so a mercy harvest changes numbers a human can see
 * even before the simulation crate is a client dependency.
 * Rumble is short and weak — confirmation, never punishment.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use std::time::Duration;

use bevy::input::gamepad::{GamepadRumbleIntensity, GamepadRumbleRequest};
use bevy::prelude::*;

use crate::rbe::RbeGlobalState;

#[derive(Resource, Debug)]
pub struct SoftRbePool {
    pub vitality: f32,
    pub harmony: f32,
    pub joy: f32,
    pub last_credit: f32,
    pub harvests: u32,
}

impl Default for SoftRbePool {
    fn default() -> Self {
        Self {
            vitality: 0.0,
            harmony: 0.0,
            joy: 0.0,
            last_credit: 0.0,
            harvests: 0,
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

/// Short weak rumble on every connected pad — confirmation only.
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
}
