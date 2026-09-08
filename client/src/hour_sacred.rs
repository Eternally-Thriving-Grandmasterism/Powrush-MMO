//! Hour sacred — Slice 0 (v23.2.4) + hour-two door (v23.2.28) + pack (v23.2.29)
//!
//! Peace: W is silent 0. Tab / G / L / Q no-op without charter_id + Frontier.
//! After a first-hour allocate, Tab steps the ridge. Q founds the House.
//! Pack persist keeps factory + I2 + Ledger + fabricator + Embassy across quit.
//! WASD / E / I / H / R stay the player door. Contact: info@Rathor.ai

use std::path::PathBuf;

use bevy::prelude::*;

use shared::hour_two::HourTwoPack;
use shared::space_law::{HexFlag, SpaceSession};

use crate::infra_spill::EvidenceYard;
use crate::ledger_bind::LedgerYard;
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::embassy::EmbassyYard;
use crate::fabricator::FabricatorYard;
use crate::vertical_factory::FactoryYard;

pub const HOUR_TWO_PATH: &str = "data/powrush_hour_two.json";

/// Resolved user-dir path for the hour-two / book pack.
pub fn hour_two_disk() -> PathBuf {
    shared::user_persist::persist_path(HOUR_TWO_PATH)
}

/// Soft-read the hour-two pack JSON from the user dir.
pub fn read_hour_two_json() -> Option<String> {
    shared::user_persist::read_named(HOUR_TWO_PATH).ok()
}

#[derive(Resource, Debug, Clone)]
pub struct HourSacred {
    pub session: SpaceSession,
    pub complete: bool,
    pub hour_three_complete: bool,
}

impl Default for HourSacred {
    fn default() -> Self {
        Self::load_or_peace()
    }
}

impl HourSacred {
    pub fn load_or_peace() -> Self {
        if let Some(raw) = read_hour_two_json() {
            let pack = HourTwoPack::from_json(&raw);
            return Self {
                session: pack.session,
                complete: pack.complete,
                hour_three_complete: pack.hour_three_complete,
            };
        }
        Self {
            session: SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        }
    }

    pub fn persist_pack(
        &mut self,
        factory: &FactoryYard,
        evidence: &EvidenceYard,
        ledger: &LedgerYard,
        fab: &FabricatorYard,
        embassy: &EmbassyYard,
    ) {
        let mut pack = HourTwoPack {
            session: self.session.clone(),
            factory: factory.factory.clone(),
            witness: evidence.witness.clone(),
            board: ledger.board.clone(),
            fabricator: fab.fab.clone(),
            embassy: embassy.embassy.clone(),
            complete: self.complete,
            hour_three_complete: self.hour_three_complete,
        };
        pack.mark_complete();
        // Hex travel must not persist a Heartwood stub seat over the house book.
        if let Some(raw) = read_hour_two_json() {
            let prior = HourTwoPack::from_json(&raw);
            pack.keep_house_book_over_hex_stub(&prior);
        }
        pack.mark_hour_three();
        self.complete = pack.complete;
        self.hour_three_complete = pack.hour_three_complete;
        if let Ok(json) = serde_json::to_string_pretty(&pack) {
            let _ = shared::user_persist::write_named(HOUR_TWO_PATH, json);
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
    mut hour: ResMut<HourSacred>,
    factory: Option<Res<FactoryYard>>,
    evidence: Option<Res<EvidenceYard>>,
    ledger: Option<Res<LedgerYard>>,
    fab: Option<Res<FabricatorYard>>,
    embassy: Option<Res<EmbassyYard>>,
) {
    let changed = hour.is_changed()
        || factory.as_ref().map(|f| f.is_changed()).unwrap_or(false)
        || evidence.as_ref().map(|e| e.is_changed()).unwrap_or(false)
        || ledger.as_ref().map(|l| l.is_changed()).unwrap_or(false)
        || fab.as_ref().map(|f| f.is_changed()).unwrap_or(false)
        || embassy.as_ref().map(|e| e.is_changed()).unwrap_or(false);
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
    let Some(fab) = fab else {
        return;
    };
    let Some(embassy) = embassy else {
        return;
    };
    hour.persist_pack(&factory, &evidence, &ledger, &fab, &embassy);
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
            hour_three_complete: false,
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
            hour_three_complete: false,
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
            hour_three_complete: false,
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

    #[test]
    fn f_book_fixture_is_not_default_boot() {
        use shared::f_book_fixture::{
            fixture_is_not_default_door, load_f_book_disk, F_BOOK_DATA_REL, F_BOOK_REL_DIR,
        };
        use shared::pause_ledger_face::{lethal_sign_row, HEX_ADMITS_HARM_OFF};
        assert_eq!(HOUR_TWO_PATH, "data/powrush_hour_two.json");
        assert_eq!(F_BOOK_DATA_REL, "tests/fixtures/f-book/data");
        assert!(!HOUR_TWO_PATH.contains("f-book"));
        assert!(!HOUR_TWO_PATH.starts_with(F_BOOK_REL_DIR));
        assert!(fixture_is_not_default_door());
        let (pack, climate, standing, week, _house) = load_f_book_disk();
        assert!(pack.complete && pack.hour_three_complete && pack.ledger_settled());
        assert!(!standing.declared_lethal);
        assert_eq!(week.tons_moved, climate.tons_moved);
        assert_eq!(
            lethal_sign_row(true, true, true, standing.declared_lethal),
            HEX_ADMITS_HARM_OFF
        );
        // Stranger first hour is not the F-book pack (empty book ≠ Settled+book).
        let fresh = HourTwoPack::default();
        assert!(!fresh.complete);
        assert!(!fresh.hour_three_complete);
        assert!(!fresh.ledger_settled());
        let resolved = hour_two_disk();
        assert!(!resolved.to_string_lossy().contains("f-book"));
        assert_eq!(
            resolved.file_name().and_then(|s| s.to_str()),
            Some("powrush_hour_two.json")
        );
        assert!(shared::user_persist::is_writable_user_dir_rule(&resolved));
    }

    #[test]
    fn persist_pack_keeps_house_book_over_heartwood_stub() {
        use shared::hex_travel::{heartwood_stub_embassy, places_eligible};
        use shared::stranger_loop_proof::hour_three_held_fixture;
        let prior = hour_three_held_fixture();
        let mut pack = prior.clone();
        pack.embassy = heartwood_stub_embassy();
        pack.mark_complete();
        pack.keep_house_book_over_hex_stub(&prior);
        pack.mark_hour_three();
        assert!(pack.hour_three_complete);
        assert!(pack.embassy.seated);
        assert_eq!(pack.embassy, prior.embassy);
        assert!(places_eligible(pack.complete, pack.hour_three_complete));
    }
}
