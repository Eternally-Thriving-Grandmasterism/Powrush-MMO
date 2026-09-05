//! Hour sacred — Slice 0 (v23.2.4) + hour-two door (v23.2.28) + pack (v23.2.29)
//!
//! Peace: W is silent 0. Tab / G / L / Q no-op without charter_id + Frontier.
//! After a first-hour allocate, Tab steps the ridge. Q founds the House.
//! Pack persist keeps factory + I2 + Ledger across quit.
//! WASD / E / I / H / R stay the player door. Contact: info@Rathor.ai

use std::fs;
use std::path::Path;

use bevy::prelude::*;

use shared::hour_two::HourTwoPack;
use shared::space_law::{HexFlag, SpaceSession};

use crate::infra_spill::EvidenceYard;
use crate::ledger_bind::LedgerYard;
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::vertical_factory::FactoryYard;

pub const HOUR_TWO_PATH: &str = "data/powrush_hour_two.json";

#[derive(Resource, Debug, Clone)]
pub struct HourSacred {
    pub session: SpaceSession,
    pub complete: bool,
}

impl Default for HourSacred {
    fn default() -> Self {
        Self::load_or_peace()
    }
}

impl HourSacred {
    pub fn load_or_peace() -> Self {
        if let Ok(raw) = fs::read_to_string(HOUR_TWO_PATH) {
            let pack = HourTwoPack::from_json(&raw);
            return Self {
                session: pack.session,
                complete: pack.complete,
            };
        }
        Self {
            session: SpaceSession::default(),
            complete: false,
        }
    }

    pub fn persist_pack(
        &self,
        factory: &FactoryYard,
        evidence: &EvidenceYard,
        ledger: &LedgerYard,
    ) {
        let mut pack = HourTwoPack {
            session: self.session.clone(),
            factory: factory.factory.clone(),
            witness: evidence.witness.clone(),
            board: ledger.board.clone(),
            complete: self.complete,
        };
        pack.mark_complete();
        if let Some(parent) = Path::new(HOUR_TWO_PATH).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&pack) {
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
        app.init_resource::<HourSacred>().add_systems(
            Update,
            (
                take_ridge_door,
                swallow_charter_skin_in_peace,
                persist_hour_two_pack,
            ),
        );
    }
}

/// Tab after allocate: Peace → Frontier visitor. Q still founds.
fn take_ridge_door(
    keyboard: Res<ButtonInput<KeyCode>>,
    bind: Option<Res<LivedHourBind>>,
    mut hour: ResMut<HourSacred>,
) {
    if hour.hex() != HexFlag::Peace {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::CHART) {
        return;
    }
    let ready = bind
        .map(|b| {
            SpaceSession::hour_two_door_ready(b.hour.allocation.flow, b.hour.allocation.reserve)
        })
        .unwrap_or(false);
    if !ready {
        return;
    }
    let _ = hour.session.take_frontier_ridge();
}

/// Tab / G / L / Q exist. In Peace they must not open Charter UI.
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

fn persist_hour_two_pack(
    hour: Res<HourSacred>,
    factory: Option<Res<FactoryYard>>,
    evidence: Option<Res<EvidenceYard>>,
    ledger: Option<Res<LedgerYard>>,
) {
    let changed = hour.is_changed()
        || factory.as_ref().map(|f| f.is_changed()).unwrap_or(false)
        || evidence.as_ref().map(|e| e.is_changed()).unwrap_or(false)
        || ledger.as_ref().map(|l| l.is_changed()).unwrap_or(false);
    if !changed {
        return;
    }
    let Some(factory) = factory else {
        return;
    };
    let Some(evidence) = evidence else {
        return;
    };
    let Some(ledger) = ledger else {
        return;
    };
    hour.persist_pack(&factory, &evidence, &ledger);
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::WarrantWeight;

    #[test]
    fn default_hour_hides_charter_skin_and_w() {
        let h = HourSacred {
            session: SpaceSession::default(),
            complete: false,
        };
        assert!(!h.charter_skin_live());
        assert_eq!(h.warrant_live(), 0.0);
        assert_eq!(h.hex(), HexFlag::Peace);
        assert!(!h.complete);
    }

    #[test]
    fn stuffed_w_still_silent_in_peace() {
        let mut h = HourSacred {
            session: SpaceSession::default(),
            complete: false,
        };
        h.session.warrant = WarrantWeight {
            h: 99.0,
            ..Default::default()
        };
        assert_eq!(h.warrant_live(), 0.0);
    }

    #[test]
    fn ridge_is_a_visitor_until_q() {
        let mut h = HourSacred {
            session: SpaceSession::default(),
            complete: false,
        };
        assert!(h.session.take_frontier_ridge());
        assert_eq!(h.hex(), HexFlag::Frontier);
        assert!(h.session.peace_visitor_on_frontier());
        assert!(!h.charter_skin_live());
    }

    #[test]
    fn hour_two_path_is_local() {
        assert_eq!(HOUR_TWO_PATH, "data/powrush_hour_two.json");
    }
}
