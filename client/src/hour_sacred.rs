//! Hour sacred — Slice 0 (v23.2.4) + hour-two door (v23.2.28) + pack (v23.2.29)
//!
//! Peace: W is silent 0. Tab / G / L / Q no-op without charter_id + Frontier.
//! After a first-hour allocate, Tab steps the ridge. Q founds the House.
//! Pack persist keeps factory + I2 + Ledger + fabricator + Embassy across quit.
//! WASD / E / I / H / R stay the player door. Contact: info@Rathor.ai

use std::path::PathBuf;

use bevy::prelude::*;

use shared::hour_two::HourTwoPack;
use shared::space_law::{CharterKind, HexFlag, SpaceSession};
use shared::vertical_factory::VerticalFactory;

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

/// Tab after first-hour allocate. Peace → Frontier visitor. No-op without a bank.
pub fn try_ridge_tab(hour: &mut HourSacred, door_ready: bool) -> bool {
    if hour.hex() != HexFlag::Peace {
        return false;
    }
    if !door_ready {
        return false;
    }
    hour.session.take_frontier_ridge()
}

/// Q off Peace plants `house-local`. Peace is a no-op for founding.
pub fn try_plant_house(hour: &mut HourSacred, factory: &mut VerticalFactory) -> bool {
    if hour.hex() == HexFlag::Peace {
        return false;
    }
    if hour.session.charter_id.is_some() {
        return false;
    }
    factory.found_house();
    hour.session.charter_id = Some("house-local".into());
    hour.session.kind = CharterKind::House;
    true
}

/// Tab after allocate: Peace → Frontier visitor. Q still founds.
fn take_ridge_door(
    keyboard: Res<ButtonInput<KeyCode>>,
    bind: Option<Res<LivedHourBind>>,
    mut hour: ResMut<HourSacred>,
) {
    if !keyboard.just_pressed(soft_play_bindings::CHART) {
        return;
    }
    let ready = bind
        .map(|b| {
            SpaceSession::hour_two_door_ready(b.hour.allocation.flow, b.hour.allocation.reserve)
        })
        .unwrap_or(false);
    let _ = try_ridge_tab(&mut hour, ready);
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

    fn peace_hour() -> HourSacred {
        HourSacred {
            session: SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        }
    }

    /// Playtest H2-TAB: Tab is a no-op until flow or reserve is banked.
    #[test]
    fn tab_without_allocate_stays_peace() {
        let mut h = peace_hour();
        assert!(!SpaceSession::hour_two_door_ready(0, 0));
        assert!(!try_ridge_tab(&mut h, false));
        assert_eq!(h.hex(), HexFlag::Peace);
        assert!(!h.session.peace_visitor_on_frontier());
    }

    /// Playtest H2-TAB: after allocate, Tab leaves Peace as a visitor.
    /// E on the ridge is *Not your charter*; Q still founds.
    #[test]
    fn tab_after_allocate_leaves_peace() {
        let mut h = peace_hour();
        assert!(SpaceSession::hour_two_door_ready(1, 0));
        assert!(try_ridge_tab(&mut h, true));
        assert_eq!(h.hex(), HexFlag::Frontier);
        assert!(h.session.peace_visitor_on_frontier());
        assert!(!h.charter_skin_live());
        assert_eq!(
            h.session.hour_two_line(true),
            "Not your charter · Q plant a House stake"
        );
        assert!(!try_ridge_tab(&mut h, true), "second Tab is a no-op");
        assert_eq!(h.hex(), HexFlag::Frontier);
    }

    /// Playtest H2-TAB: the Tab key after allocate is the door, not a direct call.
    #[test]
    fn tab_key_after_allocate_leaves_peace() {
        use bevy::input::keyboard::{Key, KeyboardInput};
        use bevy::input::ButtonState;
        use bevy::input::InputPlugin as BevyInputPlugin;
        use crate::lived_hour_bind::LivedHourBind;

        let mut hour_lived = shared::climate_node::LivedHour::new_demo();
        hour_lived.allocation.flow = 1;
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(BevyInputPlugin);
        app.insert_resource(peace_hour());
        app.insert_resource(LivedHourBind {
            hour: hour_lived,
            climate: Default::default(),
            standing: Default::default(),
            week: Default::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        });
        app.add_systems(Update, take_ridge_door);
        app.update();
        assert_eq!(app.world().resource::<HourSacred>().hex(), HexFlag::Peace);

        let window = Entity::PLACEHOLDER;
        app.world_mut().send_event(KeyboardInput {
            key_code: soft_play_bindings::CHART,
            logical_key: Key::Tab,
            state: ButtonState::Pressed,
            window,
        });
        app.update();

        let hour = app.world().resource::<HourSacred>();
        assert_eq!(hour.hex(), HexFlag::Frontier);
        assert!(hour.session.peace_visitor_on_frontier());
        assert_eq!(
            hour.session.hour_two_line(true),
            "Not your charter · Q plant a House stake"
        );
    }

    /// Playtest fail-closed: Tab before allocate must not leave Peace.
    #[test]
    fn tab_key_without_allocate_stays_peace() {
        use bevy::input::keyboard::{Key, KeyboardInput};
        use bevy::input::ButtonState;
        use bevy::input::InputPlugin as BevyInputPlugin;
        use crate::lived_hour_bind::LivedHourBind;

        let mut hour_lived = shared::climate_node::LivedHour::new_demo();
        hour_lived.allocation.flow = 0;
        hour_lived.allocation.reserve = 0;
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(BevyInputPlugin);
        app.insert_resource(peace_hour());
        app.insert_resource(LivedHourBind {
            hour: hour_lived,
            climate: Default::default(),
            standing: Default::default(),
            week: Default::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        });
        app.add_systems(Update, take_ridge_door);
        app.update();

        let window = Entity::PLACEHOLDER;
        app.world_mut().send_event(KeyboardInput {
            key_code: soft_play_bindings::CHART,
            logical_key: Key::Tab,
            state: ButtonState::Pressed,
            window,
        });
        app.update();

        let hour = app.world().resource::<HourSacred>();
        assert_eq!(hour.hex(), HexFlag::Peace);
        assert!(!hour.session.peace_visitor_on_frontier());
    }

    /// Playtest H2-Q: Q in Peace does not found a House.
    #[test]
    fn q_on_peace_does_not_found() {
        let mut h = peace_hour();
        let mut factory = VerticalFactory::default();
        assert!(!try_plant_house(&mut h, &mut factory));
        assert_eq!(h.hex(), HexFlag::Peace);
        assert!(!factory.founded);
        assert!(!h.charter_skin_live());
    }

    /// Playtest H2-Q: Q off Peace (visitor ridge) plants house-local.
    #[test]
    fn q_off_peace_plants_house() {
        let mut h = peace_hour();
        assert!(try_ridge_tab(&mut h, true));
        assert!(h.session.peace_visitor_on_frontier());
        let mut factory = VerticalFactory::default();
        assert!(try_plant_house(&mut h, &mut factory));
        assert!(factory.founded);
        assert_eq!(h.session.charter_id.as_deref(), Some("house-local"));
        assert!(h.charter_skin_live());
        assert!(!h.session.peace_visitor_on_frontier());
        assert!(!try_plant_house(&mut h, &mut factory), "second Q does not re-found");
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
