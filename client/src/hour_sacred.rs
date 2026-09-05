//! Hour sacred — Slice 0 (v23.2.4) + hour-two door (v23.2.28)
//!
//! Peace: W is silent 0. Tab / G / L / Q no-op without charter_id + Frontier.
//! After a first-hour allocate, Tab takes a local House on Frontier.
//! WASD / E / I / H / R stay the player door. Contact: info@Rathor.ai

use std::fs;
use std::path::Path;

use bevy::prelude::*;

use shared::space_law::{HexFlag, SpaceSession};

use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;

pub const HOUR_TWO_PATH: &str = "data/powrush_hour_two.json";

#[derive(Resource, Debug, Clone)]
pub struct HourSacred {
    pub session: SpaceSession,
}

impl Default for HourSacred {
    fn default() -> Self {
        Self::load_or_peace()
    }
}

impl HourSacred {
    pub fn load_or_peace() -> Self {
        if let Ok(raw) = fs::read_to_string(HOUR_TWO_PATH) {
            if let Ok(session) = serde_json::from_str::<SpaceSession>(&raw) {
                return Self { session };
            }
        }
        Self {
            session: SpaceSession::default(),
        }
    }

    pub fn persist(&self) {
        if let Some(parent) = Path::new(HOUR_TWO_PATH).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.session) {
            let _ = fs::write(HOUR_TWO_PATH, json);
        }
    }

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
            .add_systems(Update, (take_charter_door, swallow_charter_skin_in_peace));
    }
}

/// Tab after allocate: Peace → Frontier House. Existing slabs speak.
fn take_charter_door(
    keyboard: Res<ButtonInput<KeyCode>>,
    bind: Option<Res<LivedHourBind>>,
    mut hour: ResMut<HourSacred>,
) {
    if hour.charter_skin_live() {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::CHART) {
        return;
    }
    let ready = bind
        .map(|b| SpaceSession::hour_two_door_ready(b.hour.allocation.flow, b.hour.allocation.reserve))
        .unwrap_or(false);
    if !ready {
        return;
    }
    if hour.session.take_frontier_charter() {
        hour.persist();
    }
}

/// Tab / G / L / Q exist. In Peace they must not open Charter UI
/// unless the hour-two door just fired (Tab handled above).
fn swallow_charter_skin_in_peace(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
) {
    if hour.charter_skin_live() {
        return;
    }
    let _ = keyboard.just_pressed(soft_play_bindings::CHART)
        || keyboard.just_pressed(soft_play_bindings::SASH)
        || keyboard.just_pressed(soft_play_bindings::LEDGER)
        || keyboard.just_pressed(soft_play_bindings::BUILD_WHEEL);
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::{CharterKind, WarrantWeight};

    #[test]
    fn default_hour_hides_charter_skin_and_w() {
        let h = HourSacred {
            session: SpaceSession::default(),
        };
        assert!(!h.charter_skin_live());
        assert_eq!(h.warrant_live(), 0.0);
        assert_eq!(h.hex(), HexFlag::Peace);
    }

    #[test]
    fn stuffed_w_still_silent_in_peace() {
        let mut h = HourSacred {
            session: SpaceSession::default(),
        };
        h.session.warrant = WarrantWeight {
            h: 99.0,
            ..Default::default()
        };
        assert_eq!(h.warrant_live(), 0.0);
    }

    #[test]
    fn door_writes_frontier_house() {
        let mut h = HourSacred {
            session: SpaceSession::default(),
        };
        assert!(h.session.take_frontier_charter());
        assert_eq!(h.hex(), HexFlag::Frontier);
        assert_eq!(h.session.kind, CharterKind::House);
        assert!(h.charter_skin_live());
    }

    #[test]
    fn hour_two_path_is_local() {
        assert_eq!(HOUR_TWO_PATH, "data/powrush_hour_two.json");
    }
}
