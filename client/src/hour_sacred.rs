//! Hour sacred — Slice 0 (v23.2.4)
//!
//! Peace: W is silent 0. Tab / G / L / Q no-op without charter_id + Frontier.
//! WASD / E / I / H / R stay the player door. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::space_law::{HexFlag, SpaceSession};

#[derive(Resource, Debug, Clone)]
pub struct HourSacred {
    pub session: SpaceSession,
}

impl Default for HourSacred {
    fn default() -> Self {
        Self {
            session: SpaceSession::default(),
        }
    }
}

impl HourSacred {
    pub fn charter_skin_live(&self) -> bool {
        self.session.charter_skin_live()
    }

    pub fn warrant_live(&self) -> f32 {
        self.session.warrant_live()
    }

    pub fn hex(&self) -> HexFlag {
        self.session.hex
    }
}

pub struct HourSacredPlugin;

impl Plugin for HourSacredPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HourSacred>()
            .add_systems(Update, swallow_charter_skin_in_peace);
    }
}

/// Tab / G / L / Q exist. In Peace they must not open Charter UI.
fn swallow_charter_skin_in_peace(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
) {
    if hour.charter_skin_live() {
        return;
    }
    let _ = keyboard.just_pressed(crate::soft_play_bindings::CHART)
        || keyboard.just_pressed(crate::soft_play_bindings::SASH)
        || keyboard.just_pressed(crate::soft_play_bindings::LEDGER)
        || keyboard.just_pressed(crate::soft_play_bindings::BUILD_WHEEL);
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::WarrantWeight;

    #[test]
    fn default_hour_hides_charter_skin_and_w() {
        let h = HourSacred::default();
        assert!(!h.charter_skin_live());
        assert_eq!(h.warrant_live(), 0.0);
        assert_eq!(h.hex(), HexFlag::Peace);
    }

    #[test]
    fn stuffed_w_still_silent_in_peace() {
        let mut h = HourSacred::default();
        h.session.warrant = WarrantWeight {
            h: 99.0,
            ..Default::default()
        };
        assert_eq!(h.warrant_live(), 0.0);
    }
}
