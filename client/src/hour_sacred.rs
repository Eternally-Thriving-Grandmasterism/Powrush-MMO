//! Hour sacred — Slice 0 (v23.2.4) + hour-two door (v23.2.28) + pack (v23.2.29)
//!
//! Peace: W is silent 0. Tab / G / L / Q no-op without charter_id + Frontier.
//! After a first-hour allocate, Tab steps the ridge. Q founds the House.
//! Pack persist keeps factory + I2 + Ledger + fabricator + Embassy across quit.
//! WASD / E / I / H / R stay the player door. Contact: info@Rathor.ai
//!
//! CARD L2 HOUSE-PEOPLE-GATES — Q House offers five Peoples as God-plane doors
//! (Garden / boot · D0 EDEN-PLANE-LAW @ 2afff36). Four Place landings only
//! (PLAYABLE_RACES §1.1). Doors ignite after one Tend. Crossing is one-way
//! this session. Skip House = stay light / Peace default. C0 Cydruid =
//! human-in-frame, not treant. 0 meshes · ASSET_BUDGET_COURT cite only.
//! Not the #459 dress-token prove-line. Not a Title race lobby.
//!
//! CARD L3 PEOPLE-DOOR-LAND — try_cross success maps PeopleLanding → existing
//! PlaceId (three disk variants) and apply_place / lived bind. Cite
//! docs/L3_SPAWN_RESEARCH.md §3 · PLAYABLE_RACES §1.1 · C0 · D0 ·
//! ACityGamesInc/status/2101247905218568248 (stills only). 0 meshes.

use std::path::PathBuf;

use bevy::prelude::*;

use shared::hex_travel::PlaceId;
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

/// CARD L2 — cite only. No mesh / pack cargo. [`docs/ASSET_BUDGET_COURT.md`].
pub const L2_ASSET_BUDGET_CITE: &str = "docs/ASSET_BUDGET_COURT.md";
/// CARD L2 mesh budget. Hands stay dark; 0 meshes this card.
pub const L2_MESH_BUDGET: u32 = 0;

/// CARD L2 HOUSE-PEOPLE-GATES — five Peoples after Q House.
/// Cite PLAYABLE_RACES §1.1 · D0 EDEN-PLANE-LAW @ 2afff36 · C0 not-treant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HousePeople {
    Human,
    Cydruid,
    Quellorian,
    Draek,
    Ambrosian,
}

/// Steward table order: Human · Cydruid · Quellorian · Draek · Ambrosian.
pub const HOUSE_PEOPLES: [HousePeople; 5] = [
    HousePeople::Human,
    HousePeople::Cydruid,
    HousePeople::Quellorian,
    HousePeople::Draek,
    HousePeople::Ambrosian,
];

/// Four Place landings only. Ambrosian shares Sanctuary (not a 5th room).
/// Threshold is one of the four Places (rides Heartwood disk — no fifth PlaceId).
/// Garden / boot is the God-plane door host, not a landing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeopleLanding {
    SanctuaryYard,
    Heartwood,
    Threshold,
    DepthsTealWayHome,
    SanctuaryWellFromAbove,
}

impl PeopleLanding {
    pub const fn place_name(self) -> &'static str {
        match self {
            Self::SanctuaryYard | Self::SanctuaryWellFromAbove => "Sanctuary",
            Self::Heartwood => "Heartwood",
            Self::Threshold => "Threshold",
            Self::DepthsTealWayHome => "Depths",
        }
    }

    pub const fn landing_line(self) -> &'static str {
        match self {
            Self::SanctuaryYard => "Sanctuary yard",
            Self::Heartwood => "Heartwood",
            Self::Threshold => "Threshold",
            Self::DepthsTealWayHome => "Depths (teal way-home)",
            Self::SanctuaryWellFromAbove => "Sanctuary well-from-above",
        }
    }

    /// CARD L3 — disk PlaceId already used by Places travel / title boot.
    /// Threshold rides Heartwood (no fourth variant). Ambrosian = Sanctuary.
    pub const fn place_id(self) -> PlaceId {
        match self {
            Self::SanctuaryYard | Self::SanctuaryWellFromAbove => PlaceId::Sanctuary,
            Self::Heartwood | Self::Threshold => PlaceId::Heartwood,
            Self::DepthsTealWayHome => PlaceId::Depths,
        }
    }
}

impl HousePeople {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Human => "Human",
            Self::Cydruid => "Cydruid",
            Self::Quellorian => "Quellorian",
            Self::Draek => "Draek",
            Self::Ambrosian => "Ambrosian",
        }
    }

    /// C0: Cydruid is a human housed in a cyborg frame — not a treant.
    pub const fn people_line(self) -> &'static str {
        match self {
            Self::Cydruid => "Cydruid · human-in-frame",
            other => other.as_str(),
        }
    }

    /// PLAYABLE_RACES §1.1 door → landing. Ambrosian = same Sanctuary Place.
    pub const fn landing(self) -> PeopleLanding {
        match self {
            Self::Human => PeopleLanding::SanctuaryYard,
            Self::Cydruid => PeopleLanding::Heartwood,
            Self::Quellorian => PeopleLanding::Threshold,
            Self::Draek => PeopleLanding::DepthsTealWayHome,
            Self::Ambrosian => PeopleLanding::SanctuaryWellFromAbove,
        }
    }
}

/// Q House → offer the five Peoples. Skip House / ridge visitor → none.
pub fn offer_house_peoples(house_live: bool) -> Option<[HousePeople; 5]> {
    if house_live {
        Some(HOUSE_PEOPLES)
    } else {
        None
    }
}

/// Skip House = stay light-body / Peace default. No People offer. No landing.
pub fn skip_house_stays_light(house_live: bool) -> bool {
    !house_live
}

/// Five God-plane doors stay dark until the stranger Tends once.
pub fn god_plane_doors_ignited(tended_once: bool) -> bool {
    tended_once
}

/// Distinct Place names among the five landings — must be exactly four.
/// Garden is not a landing. Market is not a Place.
pub fn four_place_landings_only() -> bool {
    let mut seen = [""; 5];
    let mut n = 0;
    for people in HOUSE_PEOPLES {
        let name = people.landing().place_name();
        if !seen[..n].contains(&name) {
            seen[n] = name;
            n += 1;
        }
    }
    n == 4
        && seen[..n].contains(&"Sanctuary")
        && seen[..n].contains(&"Heartwood")
        && seen[..n].contains(&"Threshold")
        && seen[..n].contains(&"Depths")
        && !seen[..n].contains(&"Garden")
        && !seen[..n].contains(&"Market")
}

/// Cross one God-plane door. One-way this session. Needs House + one Tend.
/// `crossed` is session-local — not written to the hour-two pack.
/// CARD L3: success is PlaceId + apply_place / lived bind (hex_travel).
pub fn try_cross_people_door(
    house_live: bool,
    tended_once: bool,
    crossed: &mut Option<HousePeople>,
    people: HousePeople,
) -> Option<PeopleLanding> {
    if !house_live || !tended_once || crossed.is_some() {
        return None;
    }
    *crossed = Some(people);
    Some(people.landing())
}

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
    /// Peace only. Must not read live user-dir hour-two — CAPTURE persist
    /// poisons `--lib` Peace tests. Boot/plugin uses [`Self::load_or_peace`].
    fn default() -> Self {
        Self {
            session: SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        }
    }
}

impl HourSacred {
    /// Soft-load the user-dir pack. Boot / plugin only — not `Default`.
    /// Does not wipe steward saves; missing or unreadable JSON stays Peace.
    pub fn load_or_peace() -> Self {
        if let Some(raw) = read_hour_two_json() {
            let pack = HourTwoPack::from_json(&raw);
            return Self {
                session: pack.session,
                complete: pack.complete,
                hour_three_complete: pack.hour_three_complete,
            };
        }
        Self::default()
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

    /// CARD L2 — Q House offers five Peoples. Skip House stays light / Peace.
    pub fn offers_five_peoples(&self) -> bool {
        self.charter_skin_live()
    }

    /// Skip House / ridge visitor: stay light-body / Peace default. No People offer.
    pub fn stays_light_peace(&self) -> bool {
        skip_house_stays_light(self.charter_skin_live())
    }

    /// Five Peoples after House. None while Peace / visitor (no Q).
    pub fn house_people_offer(&self) -> Option<[HousePeople; 5]> {
        offer_house_peoples(self.charter_skin_live())
    }

    /// Cross one ignited God-plane door. One-way this session. Not persisted.
    pub fn try_cross_people_door(
        &self,
        tended_once: bool,
        crossed: &mut Option<HousePeople>,
        people: HousePeople,
    ) -> Option<PeopleLanding> {
        try_cross_people_door(self.charter_skin_live(), tended_once, crossed, people)
    }
}

pub struct HourSacredPlugin;

impl Plugin for HourSacredPlugin {
    fn build(&self, app: &mut App) {
        // Persist is a boot path. `init_resource` would use Default (Peace).
        app.insert_resource(HourSacred::load_or_peace()).add_systems(
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

/// House + spill seen + Bind Settled latches Hour two held.
pub fn try_mark_hour_two_held(hour: &mut HourSacred, witness_seen: bool, settled: bool) -> bool {
    if hour.complete {
        return false;
    }
    if hour.charter_skin_live() && witness_seen && settled {
        hour.complete = true;
        return true;
    }
    false
}

/// Proof Pack + Embassy seat latches Hour three / the book.
pub fn try_mark_hour_three_held(hour: &mut HourSacred, pack_unlocked: bool, seated: bool) -> bool {
    if hour.hour_three_complete {
        return false;
    }
    if pack_unlocked && seated {
        hour.hour_three_complete = true;
        return true;
    }
    false
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
        let h = HourSacred::default();
        assert!(!h.charter_skin_live());
        assert_eq!(h.warrant_live(), 0.0);
        assert_eq!(h.hex(), HexFlag::Peace);
        assert!(!h.complete);
        assert!(!h.hour_three_complete);
    }

    /// PLAYTEST/CI: live `powrush_hour_two.json` must not poison Default.
    #[test]
    fn default_is_peace_without_reading_hour_two() {
        let h = HourSacred::default();
        assert_eq!(h.hex(), HexFlag::Peace);
        assert!(!h.complete);
        assert!(!h.hour_three_complete);
        assert!(!h.session.peace_visitor_on_frontier());
        assert!(!h.charter_skin_live());

        // CAPTURE persist (if present) loads only via load_or_peace — Default stays Peace.
        if let Some(raw) = read_hour_two_json() {
            let pack = HourTwoPack::from_json(&raw);
            let loaded = HourSacred::load_or_peace();
            assert_eq!(loaded.hex(), pack.session.hex);
            assert_eq!(loaded.complete, pack.complete);
            assert_eq!(loaded.hour_three_complete, pack.hour_three_complete);
            if pack.session.hex != HexFlag::Peace || pack.complete {
                assert_eq!(h.hex(), HexFlag::Peace);
                assert!(!h.complete);
                assert_ne!(
                    loaded.complete, h.complete,
                    "Default ignored live hour-two complete pack"
                );
            }
        }
    }

    /// Plugin boot still loads persist. Default stays Peace even if disk is CAPTURE.
    #[test]
    fn plugin_boot_uses_load_or_peace() {
        let mut app = App::new();
        app.add_plugins(HourSacredPlugin);
        let hour = app.world().resource::<HourSacred>();
        let loaded = HourSacred::load_or_peace();
        assert_eq!(hour.hex(), loaded.hex());
        assert_eq!(hour.complete, loaded.complete);
        assert_eq!(hour.hour_three_complete, loaded.hour_three_complete);
        assert_eq!(HourSacred::default().hex(), HexFlag::Peace);
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
        HourSacred::default()
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

    /// CARD L2 HOUSE-PEOPLE-GATES — Q House offers five Peoples.
    /// Not the #459 dress-token prove-line. C0 Cydruid = human-in-frame.
    #[test]
    fn q_house_offers_five_peoples() {
        let mut h = peace_hour();
        let mut factory = VerticalFactory::default();
        assert!(h.stays_light_peace());
        assert!(h.house_people_offer().is_none());
        assert!(try_ridge_tab(&mut h, true));
        assert!(h.stays_light_peace(), "ridge visitor is not House");
        assert!(h.house_people_offer().is_none());
        assert!(try_plant_house(&mut h, &mut factory));
        assert!(h.offers_five_peoples());
        assert!(!h.stays_light_peace());
        let offer = h.house_people_offer().expect("five Peoples after Q House");
        assert_eq!(offer, HOUSE_PEOPLES);
        assert_eq!(offer.len(), 5);
        assert_eq!(HousePeople::Human.landing(), PeopleLanding::SanctuaryYard);
        assert_eq!(HousePeople::Cydruid.landing(), PeopleLanding::Heartwood);
        assert_eq!(HousePeople::Quellorian.landing(), PeopleLanding::Threshold);
        assert_eq!(
            HousePeople::Draek.landing(),
            PeopleLanding::DepthsTealWayHome
        );
        assert_eq!(
            HousePeople::Ambrosian.landing(),
            PeopleLanding::SanctuaryWellFromAbove
        );
        assert_eq!(
            HousePeople::Draek.landing().landing_line(),
            "Depths (teal way-home)"
        );
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
        assert!(!HousePeople::Cydruid.people_line().contains("bark"));
        // Refuse old #459 dress-token prove-line.
        for people in offer {
            assert!(!people.people_line().contains("Sanctuary tint"));
            assert!(!people.people_line().contains("dress token"));
        }
        assert_eq!(L2_MESH_BUDGET, 0);
        assert_eq!(L2_ASSET_BUDGET_CITE, "docs/ASSET_BUDGET_COURT.md");
    }

    /// CARD L2 — skip House / Peace / visitor stays light · Peace default.
    #[test]
    fn skip_house_stays_light_peace() {
        let mut h = peace_hour();
        assert_eq!(h.hex(), HexFlag::Peace);
        assert!(h.stays_light_peace());
        assert!(!h.offers_five_peoples());
        assert!(skip_house_stays_light(false));
        assert!(try_ridge_tab(&mut h, true));
        assert_eq!(h.hex(), HexFlag::Frontier);
        assert!(h.stays_light_peace());
        assert!(h.house_people_offer().is_none());
        let mut crossed = None;
        assert!(h
            .try_cross_people_door(true, &mut crossed, HousePeople::Human)
            .is_none());
        assert!(crossed.is_none());
    }

    /// CARD L2 — 5 God-plane doors · 4 Place landings only.
    /// Garden ≠ Sanctuary. Ambrosian shares Sanctuary (not a 5th room).
    #[test]
    fn five_god_plane_doors_four_place_landings() {
        assert_eq!(HOUSE_PEOPLES.len(), 5);
        assert!(four_place_landings_only());
        assert_eq!(
            HousePeople::Human.landing().place_name(),
            HousePeople::Ambrosian.landing().place_name()
        );
        assert_eq!(HousePeople::Human.landing().place_name(), "Sanctuary");
        assert_ne!(
            HousePeople::Human.landing(),
            HousePeople::Ambrosian.landing()
        );
        assert_eq!(
            HousePeople::Human.landing().landing_line(),
            "Sanctuary yard"
        );
        assert_eq!(
            HousePeople::Ambrosian.landing().landing_line(),
            "Sanctuary well-from-above"
        );
        for people in HOUSE_PEOPLES {
            assert_ne!(people.landing().place_name(), "Garden");
            assert_ne!(people.landing().place_name(), "Eden");
            assert_ne!(people.landing().place_name(), "Market");
        }
        assert_eq!(L2_MESH_BUDGET, 0);
    }

    /// CARD L2 — doors ignite after one Tend; crossing is one-way this session.
    #[test]
    fn doors_ignite_after_tend_cross_one_way() {
        let mut h = peace_hour();
        let mut factory = VerticalFactory::default();
        assert!(try_ridge_tab(&mut h, true));
        assert!(try_plant_house(&mut h, &mut factory));
        assert!(!god_plane_doors_ignited(false));
        let mut crossed = None;
        assert!(
            h.try_cross_people_door(false, &mut crossed, HousePeople::Quellorian)
                .is_none(),
            "doors stay dark until one Tend"
        );
        assert!(god_plane_doors_ignited(true));
        let land = h
            .try_cross_people_door(true, &mut crossed, HousePeople::Quellorian)
            .expect("House + Tend opens one door");
        assert_eq!(land, PeopleLanding::Threshold);
        assert_eq!(crossed, Some(HousePeople::Quellorian));
        assert!(
            h.try_cross_people_door(true, &mut crossed, HousePeople::Draek)
                .is_none(),
            "crossing is one-way this session"
        );
        assert_eq!(crossed, Some(HousePeople::Quellorian));
        assert_eq!(L2_MESH_BUDGET, 0);
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

    /// Playtest H2-L: Bind (not Digit3) + spill seen latches Hour two held.
    #[test]
    fn bind_settled_marks_hour_two_held() {
        let mut h = peace_hour();
        assert!(try_ridge_tab(&mut h, true));
        let mut factory = VerticalFactory::default();
        assert!(try_plant_house(&mut h, &mut factory));
        assert!(!try_mark_hour_two_held(&mut h, true, false));
        assert!(!h.complete);
        assert!(try_mark_hour_two_held(&mut h, true, true));
        assert!(h.complete);
        assert!(!try_mark_hour_two_held(&mut h, true, true), "latch once");
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
