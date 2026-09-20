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
//!
//! CARD S2 GATE-SEAL — new soul is light on the God-plane (garden); doors
//! stay unsealed until existing E/Q confirm at landing. Confirm writes
//! People+Place onto the existing hour-two disk (`powrush_hour_two.json`)
//! — extra keys, not a new schema file. Decline / wrong door returns to
//! garden still light, PlaceId unchanged. Skip House stays light (L7).
//! Peace recall = vision home, does not unseal. PlaceId stays 3. S1
//! aftermath lines untouched. No new verb · no fifth Place · Online grey.
//!
//! CARD S3 GARDEN-ROSTER — Title Play is a new ungenerated light soul on
//! the garden / God-plane (unsealed). Continue lists sealed souls from
//! the S2 hour-two extra keys (`sealed_people` / `sealed_landing`), each
//! in People dress, resume at last sealed Place. Light form is only the
//! new / unsealed slot — never a race-portrait lobby. PlaceId stays 3.
//! Reads S2 helpers; does not invent schema. S2 WRITE untouched.
//!
//! CARD F5 WRONG-DOOR-BOUNCE — unsealed light may take a People-door and
//! feel the existing L7 arrival beat. Land is not a seal (still unsealed
//! until E/Q). Decline / wrong door returns to garden still light, PlaceId
//! garden. Peace recall stays vision home and does not unseal. PlaceId
//! stays 3. S1 aftermath / S2 seal / S3 roster WRITE unread. No fifth
//! Place · no Title race lobby · Online grey.
//!
//! CARD F6 CONTINUE-IS-THE-BODY — Continue list is sealed souls as body
//! (People dress + last Place). Light / unsealed is only the Play slot.
//! Delete-soul / clear seal removes S2 extra keys on the existing
//! hour-two disk; that slot returns to light (Play). Not a race-portrait
//! lobby. PlaceId stays 3. S2 keys only — no new schema. S1 / S2 / S3 / F5
//! WRITE unread. Online grey.

use std::path::PathBuf;

use bevy::prelude::*;

use shared::hex_travel::PlaceId;
use shared::hour_two::HourTwoPack;
use shared::local_settings::PeaceKey;
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

    /// CARD S2 — persist key on existing hour-two disk. Not a new PlaceId.
    pub const fn persist_name(self) -> &'static str {
        match self {
            Self::SanctuaryYard => "sanctuary_yard",
            Self::Heartwood => "heartwood",
            Self::Threshold => "threshold",
            Self::DepthsTealWayHome => "depths_teal_way_home",
            Self::SanctuaryWellFromAbove => "sanctuary_well_from_above",
        }
    }

    pub fn from_persist(raw: &str) -> Option<Self> {
        match raw.trim() {
            "sanctuary_yard" => Some(Self::SanctuaryYard),
            "heartwood" => Some(Self::Heartwood),
            "threshold" => Some(Self::Threshold),
            "depths_teal_way_home" => Some(Self::DepthsTealWayHome),
            "sanctuary_well_from_above" => Some(Self::SanctuaryWellFromAbove),
            _ => None,
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

    /// CARD S2 — persist key on existing hour-two disk. Same as [`Self::as_str`].
    pub fn from_persist(raw: &str) -> Option<Self> {
        match raw.trim() {
            "Human" => Some(Self::Human),
            "Cydruid" => Some(Self::Cydruid),
            "Quellorian" => Some(Self::Quellorian),
            "Draek" => Some(Self::Draek),
            "Ambrosian" => Some(Self::Ambrosian),
            _ => None,
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
/// `crossed` is session-local until CARD S2 E/Q confirm writes People+Place
/// onto the existing hour-two disk. CARD L3: success is PlaceId + apply_place
/// / lived bind (hex_travel). Land is not a seal.
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

/// CARD S2 — extra keys on existing `powrush_hour_two.json`. Not a new file.
pub const SEALED_PEOPLE_KEY: &str = "sealed_people";
pub const SEALED_LANDING_KEY: &str = "sealed_landing";

/// CARD S2 — Peace recall is vision home, not a hub and not an unseal.
pub const PEACE_RECALL_VISION_HOME: &str = "vision home";

/// CARD S2 — new soul is light until People+Place are sealed.
pub fn soul_is_light(sealed: Option<(HousePeople, PeopleLanding)>) -> bool {
    sealed.is_none()
}

/// CARD S2 — God-plane doors stay unsealed until E/Q confirm at landing.
pub fn doors_are_unsealed(sealed: Option<(HousePeople, PeopleLanding)>) -> bool {
    sealed.is_none()
}

/// CARD S2 — existing E / Q only. No new verb.
pub fn is_gate_seal_confirm_verb(key: PeaceKey) -> bool {
    matches!(key, PeaceKey::E | PeaceKey::Q)
}

/// CARD S2 — confirm at landing with existing E/Q seals this People+Place.
/// Other keys do nothing. Missing pending land does nothing.
pub fn confirm_gate_seal(
    key: PeaceKey,
    pending: Option<(HousePeople, PeopleLanding)>,
) -> Option<(HousePeople, PeopleLanding)> {
    if !is_gate_seal_confirm_verb(key) {
        return None;
    }
    pending
}

/// CARD S2 — decline / wrong door: still light, not tied. Clears session cross.
/// Caller restores garden PlaceId (no net PlaceId change).
pub fn decline_or_wrong_door(
    crossed: &mut Option<HousePeople>,
    pending_landing: &mut Option<PeopleLanding>,
    sealed: Option<(HousePeople, PeopleLanding)>,
) -> bool {
    if sealed.is_some() {
        return false;
    }
    *crossed = None;
    *pending_landing = None;
    true
}

/// CARD S2 — Peace recall = vision home. Seal stays.
pub fn peace_recall(
    sealed: Option<(HousePeople, PeopleLanding)>,
) -> (Option<(HousePeople, PeopleLanding)>, &'static str) {
    (sealed, PEACE_RECALL_VISION_HOME)
}

/// CARD F5 — unsealed light (House + Tend, no E/Q seal) may take a People-door.
/// Skip House / already sealed / no Tend stay off the door. Land is not a seal.
pub fn unsealed_light_may_take_people_door(
    house_live: bool,
    tended_once: bool,
    sealed: Option<(HousePeople, PeopleLanding)>,
) -> bool {
    soul_is_light(sealed) && doors_are_unsealed(sealed) && house_live && tended_once
}

/// CARD F5 — after a People-door land the soul stays light until existing E/Q.
pub fn still_unsealed_until_eq(sealed: Option<(HousePeople, PeopleLanding)>) -> bool {
    soul_is_light(sealed) && doors_are_unsealed(sealed)
}

/// CARD F5 — Title is not a race / class lobby. S3 roster READ only.
pub fn f5_title_is_race_lobby() -> bool {
    garden_roster_is_race_portrait_lobby()
}

/// CARD S2 — read People+Place seal from existing hour-two JSON extra keys.
pub fn gate_seal_from_hour_two_json(raw: &str) -> Option<(HousePeople, PeopleLanding)> {
    let value = serde_json::from_str::<serde_json::Value>(raw).ok()?;
    let people = value
        .get(SEALED_PEOPLE_KEY)?
        .as_str()
        .and_then(HousePeople::from_persist)?;
    let landing = value
        .get(SEALED_LANDING_KEY)?
        .as_str()
        .and_then(PeopleLanding::from_persist)?;
    Some((people, landing))
}

/// CARD S2 — write People+Place onto existing hour-two JSON. Extra keys only.
pub fn merge_gate_seal_into_hour_two_json(
    raw: &str,
    people: HousePeople,
    landing: PeopleLanding,
) -> String {
    let mut value = serde_json::from_str::<serde_json::Value>(raw)
        .unwrap_or_else(|_| serde_json::json!({}));
    if !value.is_object() {
        value = serde_json::json!({});
    }
    if let Some(obj) = value.as_object_mut() {
        obj.insert(
            SEALED_PEOPLE_KEY.to_string(),
            serde_json::Value::String(people.as_str().to_string()),
        );
        obj.insert(
            SEALED_LANDING_KEY.to_string(),
            serde_json::Value::String(landing.persist_name().to_string()),
        );
    }
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| raw.to_string())
}

/// CARD S2 — keep extra seal keys when the hour-two pack is rewritten.
pub fn preserve_gate_seal_in_hour_two_json(prior: &str, next: &str) -> String {
    match gate_seal_from_hour_two_json(prior) {
        Some((people, landing)) => merge_gate_seal_into_hour_two_json(next, people, landing),
        None => next.to_string(),
    }
}

/// CARD S3 — Title garden roster row. In-memory view of S2 keys, not a schema.
/// Light is only the Play / new / unsealed slot. Sealed rows wear People dress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GardenRosterSoul {
    /// Play — ungenerated light on the garden / God-plane. Doors unsealed.
    LightUnsealed,
    /// Continue — S2 sealed People+Place. Dress + last Place. Not a portrait.
    Sealed {
        people: HousePeople,
        landing: PeopleLanding,
    },
}

impl GardenRosterSoul {
    pub const fn is_light(self) -> bool {
        matches!(self, Self::LightUnsealed)
    }

    pub const fn is_unsealed(self) -> bool {
        self.is_light()
    }

    /// People dress for sealed souls. Light form never wears a race portrait.
    pub const fn dress_line(self) -> &'static str {
        match self {
            Self::LightUnsealed => "light",
            Self::Sealed { people, .. } => people.people_line(),
        }
    }

    /// Last sealed Place. Garden / God-plane is not a PlaceId landing.
    pub const fn last_place(self) -> Option<PlaceId> {
        match self {
            Self::LightUnsealed => None,
            Self::Sealed { landing, .. } => Some(landing.place_id()),
        }
    }

    pub const fn last_place_name(self) -> &'static str {
        match self {
            Self::LightUnsealed => "Garden",
            Self::Sealed { landing, .. } => landing.place_name(),
        }
    }

    pub const fn sealed_pair(self) -> Option<(HousePeople, PeopleLanding)> {
        match self {
            Self::LightUnsealed => None,
            Self::Sealed { people, landing } => Some((people, landing)),
        }
    }
}

/// CARD S3 — Play starts a new ungenerated light soul. Does not read the seal.
pub fn play_new_light_soul() -> GardenRosterSoul {
    GardenRosterSoul::LightUnsealed
}

/// CARD S3 — Continue lists sealed souls from S2 hour-two extra keys.
/// One pair of keys → zero or one row. Not a five-portrait race lobby.
pub fn continue_sealed_souls_from_hour_two_json(raw: &str) -> Vec<GardenRosterSoul> {
    match gate_seal_from_hour_two_json(raw) {
        Some((people, landing)) => vec![GardenRosterSoul::Sealed { people, landing }],
        None => Vec::new(),
    }
}

/// CARD S3 — Title garden roster is souls, never a race-portrait lobby.
pub fn garden_roster_is_race_portrait_lobby() -> bool {
    false
}

/// CARD F6 — Continue is the sealed soul as body (People dress + last Place).
/// Light / unsealed is Play, never a Continue body. Not a race portrait.
pub fn continue_is_the_body(soul: GardenRosterSoul) -> bool {
    matches!(soul, GardenRosterSoul::Sealed { .. })
        && !soul.is_light()
        && soul.last_place().is_some()
        && !garden_roster_is_race_portrait_lobby()
}

/// CARD F6 — Continue bodies from S2 hour-two extra keys. Empty = Play (light).
pub fn continue_bodies_from_hour_two_json(raw: &str) -> Vec<GardenRosterSoul> {
    continue_sealed_souls_from_hour_two_json(raw)
}

/// CARD F6 — dress · last Place line for a Continue body. None for Play light.
pub fn continue_body_line(soul: GardenRosterSoul) -> Option<String> {
    if !continue_is_the_body(soul) {
        return None;
    }
    Some(format!("{} · {}", soul.dress_line(), soul.last_place_name()))
}

/// CARD F6 — remove S2 extra keys from existing hour-two JSON. Not a new file.
pub fn clear_gate_seal_from_hour_two_json(raw: &str) -> String {
    let mut value = serde_json::from_str::<serde_json::Value>(raw)
        .unwrap_or_else(|_| serde_json::json!({}));
    if !value.is_object() {
        value = serde_json::json!({});
    }
    if let Some(obj) = value.as_object_mut() {
        obj.remove(SEALED_PEOPLE_KEY);
        obj.remove(SEALED_LANDING_KEY);
    }
    serde_json::to_string_pretty(&value).unwrap_or_else(|_| raw.to_string())
}

/// CARD F6 — delete-soul / clear seal. That Continue body slot returns to light (Play).
/// Reuses S2 extra keys on the existing hour-two disk. No new schema file.
pub fn delete_soul_returns_light(raw: &str) -> (String, GardenRosterSoul) {
    let cleared = clear_gate_seal_from_hour_two_json(raw);
    (cleared, play_new_light_soul())
}

/// CARD F6 — Title is not a race / class lobby. S3 roster READ only.
pub fn f6_title_is_race_lobby() -> bool {
    garden_roster_is_race_portrait_lobby()
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
            let json = match read_hour_two_json() {
                Some(prior) => preserve_gate_seal_in_hour_two_json(&prior, &json),
                None => json,
            };
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

    /// CARD F5 — House + Tend + unsealed may take one People-door. Not a seal.
    pub fn unsealed_light_may_take_people_door(
        &self,
        tended_once: bool,
        sealed: Option<(HousePeople, PeopleLanding)>,
    ) -> bool {
        unsealed_light_may_take_people_door(self.charter_skin_live(), tended_once, sealed)
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

    /// CARD S2 — new soul starts light · doors unsealed.
    #[test]
    fn s2_new_soul_starts_light_doors_unsealed() {
        let h = HourSacred::default();
        assert!(soul_is_light(None));
        assert!(doors_are_unsealed(None));
        assert!(h.stays_light_peace());
        assert!(skip_house_stays_light(false));
        assert!(gate_seal_from_hour_two_json("{}").is_none());
        assert!(gate_seal_from_hour_two_json(r#"{"hex":"Peace"}"#).is_none());
        let mut crossed = None;
        assert!(h
            .try_cross_people_door(true, &mut crossed, HousePeople::Human)
            .is_none());
        assert!(crossed.is_none(), "unsealed light soul is not tied");
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
    }

    /// CARD S2 — confirm at landing with existing E/Q seals People+Place
    /// onto the existing hour-two disk (extra keys, not a new schema).
    #[test]
    fn s2_confirm_at_landing_seals_people_place_persisted() {
        let existing = r#"{"charter_id":"house-local","hex":"Frontier","kind":"House","warrant":{"h":0.0,"i":0.0,"c":0.0,"f":0.0,"x":0.0,"repair":0.0,"return_cargo":0.0,"council":0.0,"tend_spill":0.0}}"#;
        let pack = HourTwoPack::from_json(existing);
        assert_eq!(pack.session.charter_id.as_deref(), Some("house-local"));
        assert!(gate_seal_from_hour_two_json(existing).is_none());

        let pending = Some((HousePeople::Cydruid, PeopleLanding::Heartwood));
        assert!(confirm_gate_seal(PeaceKey::Digit1, pending).is_none());
        assert!(soul_is_light(None));

        let sealed = confirm_gate_seal(PeaceKey::E, pending).expect("E confirms");
        assert_eq!(sealed, (HousePeople::Cydruid, PeopleLanding::Heartwood));
        assert!(!soul_is_light(Some(sealed)));
        assert!(!doors_are_unsealed(Some(sealed)));

        let q_sealed = confirm_gate_seal(PeaceKey::Q, pending).expect("Q confirms");
        assert_eq!(q_sealed, sealed);

        let json = merge_gate_seal_into_hour_two_json(
            existing,
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        assert_eq!(
            gate_seal_from_hour_two_json(&json),
            Some((HousePeople::Cydruid, PeopleLanding::Heartwood))
        );
        let loaded = HourTwoPack::from_json(&json);
        assert_eq!(loaded.session.charter_id.as_deref(), Some("house-local"));
        assert_eq!(loaded.session.hex, HexFlag::Frontier);
        assert!(json.contains(SEALED_PEOPLE_KEY));
        assert!(json.contains("Cydruid"));
        assert!(json.contains("heartwood"));
        assert_eq!(HOUR_TWO_PATH, "data/powrush_hour_two.json");
    }

    /// CARD S2 — Peace recall is vision home and does not clear the seal.
    #[test]
    fn s2_peace_recall_does_not_clear_seal() {
        let sealed = Some((HousePeople::Draek, PeopleLanding::DepthsTealWayHome));
        let json = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Draek,
            PeopleLanding::DepthsTealWayHome,
        );
        let (after, line) = peace_recall(sealed);
        assert_eq!(after, sealed);
        assert_eq!(line, PEACE_RECALL_VISION_HOME);
        assert_eq!(line, "vision home");
        assert_eq!(
            gate_seal_from_hour_two_json(&json),
            Some((HousePeople::Draek, PeopleLanding::DepthsTealWayHome))
        );
        let rewritten = r#"{"complete":true,"hour_three_complete":false}"#;
        let kept = preserve_gate_seal_in_hour_two_json(&json, rewritten);
        assert_eq!(
            gate_seal_from_hour_two_json(&kept),
            Some((HousePeople::Draek, PeopleLanding::DepthsTealWayHome))
        );
        let mut crossed = Some(HousePeople::Draek);
        let mut pending = Some(PeopleLanding::DepthsTealWayHome);
        assert!(!decline_or_wrong_door(&mut crossed, &mut pending, sealed));
        assert_eq!(crossed, Some(HousePeople::Draek));
        assert_eq!(pending, Some(PeopleLanding::DepthsTealWayHome));
    }

    /// CARD S3 — Play starts a new ungenerated light soul (garden / God-plane).
    #[test]
    fn s3_play_starts_light_unsealed_soul() {
        let soul = play_new_light_soul();
        assert_eq!(soul, GardenRosterSoul::LightUnsealed);
        assert!(soul.is_light());
        assert!(soul.is_unsealed());
        assert!(soul_is_light(soul.sealed_pair()));
        assert!(doors_are_unsealed(soul.sealed_pair()));
        assert_eq!(soul.dress_line(), "light");
        assert!(soul.last_place().is_none(), "garden is not a Place landing");
        assert_eq!(soul.last_place_name(), "Garden");
        assert_ne!(soul.last_place_name(), "Sanctuary");
        assert!(!soul.dress_line().contains("portrait"));
        assert!(!garden_roster_is_race_portrait_lobby());
    }

    /// CARD S3 — Continue lists sealed souls from S2 keys · People dress · last Place.
    #[test]
    fn s3_continue_lists_sealed_souls_dress_last_place() {
        assert!(continue_sealed_souls_from_hour_two_json("{}").is_empty());
        assert!(continue_sealed_souls_from_hour_two_json(r#"{"hex":"Peace"}"#).is_empty());

        let json = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        let list = continue_sealed_souls_from_hour_two_json(&json);
        assert_eq!(list.len(), 1, "S2 keys are one sealed soul, not a portraits grid");
        assert_ne!(list.len(), HOUSE_PEOPLES.len());
        let soul = list[0];
        assert!(!soul.is_light());
        assert_eq!(soul.dress_line(), "Cydruid · human-in-frame");
        assert!(!soul.dress_line().contains("treant"));
        assert!(!soul.dress_line().contains("portrait"));
        assert_eq!(soul.last_place(), Some(PlaceId::Heartwood));
        assert_eq!(soul.last_place_name(), "Heartwood");
        assert_eq!(
            soul.sealed_pair(),
            Some((HousePeople::Cydruid, PeopleLanding::Heartwood))
        );

        let draek = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Draek,
            PeopleLanding::DepthsTealWayHome,
        );
        let list = continue_sealed_souls_from_hour_two_json(&draek);
        assert_eq!(list[0].dress_line(), "Draek");
        assert_eq!(list[0].last_place(), Some(PlaceId::Depths));
        assert_eq!(list[0].last_place_name(), "Depths");
    }

    /// CARD S3 — S2 seal keys still round-trip on the existing hour-two disk.
    #[test]
    fn s3_s2_seal_keys_still_round_trip() {
        let existing = r#"{"charter_id":"house-local","hex":"Frontier","kind":"House","warrant":{"h":0.0,"i":0.0,"c":0.0,"f":0.0,"x":0.0,"repair":0.0,"return_cargo":0.0,"council":0.0,"tend_spill":0.0}}"#;
        let json = merge_gate_seal_into_hour_two_json(
            existing,
            HousePeople::Quellorian,
            PeopleLanding::Threshold,
        );
        assert_eq!(
            gate_seal_from_hour_two_json(&json),
            Some((HousePeople::Quellorian, PeopleLanding::Threshold))
        );
        let loaded = HourTwoPack::from_json(&json);
        assert_eq!(loaded.session.charter_id.as_deref(), Some("house-local"));
        assert_eq!(loaded.session.hex, HexFlag::Frontier);
        assert!(json.contains(SEALED_PEOPLE_KEY));
        assert!(json.contains(SEALED_LANDING_KEY));
        assert!(json.contains("Quellorian"));
        assert!(json.contains("threshold"));
        let rewritten = r#"{"complete":true,"hour_three_complete":false}"#;
        let kept = preserve_gate_seal_in_hour_two_json(&json, rewritten);
        assert_eq!(
            gate_seal_from_hour_two_json(&kept),
            Some((HousePeople::Quellorian, PeopleLanding::Threshold))
        );
        let roster = continue_sealed_souls_from_hour_two_json(&kept);
        assert_eq!(roster.len(), 1);
        assert_eq!(roster[0].dress_line(), "Quellorian");
        assert_eq!(roster[0].last_place(), Some(PlaceId::Heartwood));
        assert_eq!(HOUR_TWO_PATH, "data/powrush_hour_two.json");
    }

    /// CARD S3 — PlaceId / LOCAL_HEXES stay three. Garden is not a fifth Place.
    #[test]
    fn s3_place_id_local_hexes_len_three() {
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        assert!(four_place_landings_only());
        let light = play_new_light_soul();
        assert!(light.last_place().is_none());
        assert_eq!(light.last_place_name(), "Garden");
        for place in shared::hex_travel::LOCAL_HEXES {
            assert_ne!(place.as_str(), "garden");
        }
    }

    /// CARD F5 — unsealed light may cross a People-door · still unsealed until E/Q.
    #[test]
    fn f5_unsealed_light_may_cross_door_still_unsealed_until_eq() {
        let mut h = peace_hour();
        let mut factory = VerticalFactory::default();
        assert!(!h.unsealed_light_may_take_people_door(true, None));
        assert!(try_ridge_tab(&mut h, true));
        assert!(try_plant_house(&mut h, &mut factory));
        assert!(h.unsealed_light_may_take_people_door(true, None));
        assert!(!h.unsealed_light_may_take_people_door(false, None));
        let sealed = Some((HousePeople::Human, PeopleLanding::SanctuaryYard));
        assert!(!h.unsealed_light_may_take_people_door(true, sealed));

        let mut crossed = None;
        let land = h
            .try_cross_people_door(true, &mut crossed, HousePeople::Human)
            .expect("unsealed light may take a People-door");
        assert_eq!(land, PeopleLanding::SanctuaryYard);
        assert_eq!(crossed, Some(HousePeople::Human));
        assert!(still_unsealed_until_eq(None));
        assert!(soul_is_light(None));
        assert!(doors_are_unsealed(None));
        assert!(confirm_gate_seal(PeaceKey::Digit1, Some((HousePeople::Human, land))).is_none());
        let sealed = confirm_gate_seal(PeaceKey::E, Some((HousePeople::Human, land))).expect("E");
        assert!(!still_unsealed_until_eq(Some(sealed)));
        assert!(!soul_is_light(Some(sealed)));
    }

    /// CARD F5 — decline / wrong door → garden light · not sealed · PlaceId garden.
    #[test]
    fn f5_decline_wrong_door_garden_light_not_sealed_place_id_garden() {
        let mut crossed = Some(HousePeople::Draek);
        let mut pending = Some(PeopleLanding::DepthsTealWayHome);
        assert!(decline_or_wrong_door(&mut crossed, &mut pending, None));
        assert!(crossed.is_none());
        assert!(pending.is_none());
        assert!(soul_is_light(None));
        assert!(doors_are_unsealed(None));
        assert!(still_unsealed_until_eq(None));
        assert_eq!(PeopleLanding::SanctuaryYard.place_id(), PlaceId::Sanctuary);
        for place in shared::hex_travel::LOCAL_HEXES {
            assert_ne!(place.as_str(), "garden");
        }
    }

    /// CARD F5 — Peace recall does not clear seal (if sealed) / vision home for light.
    #[test]
    fn f5_peace_recall_does_not_clear_seal_vision_home_for_light() {
        let sealed = Some((HousePeople::Cydruid, PeopleLanding::Heartwood));
        let (after, line) = peace_recall(sealed);
        assert_eq!(after, sealed);
        assert_eq!(line, PEACE_RECALL_VISION_HOME);
        assert_eq!(line, "vision home");
        assert!(!soul_is_light(after));

        let (light_after, light_line) = peace_recall(None);
        assert!(light_after.is_none());
        assert_eq!(light_line, PEACE_RECALL_VISION_HOME);
        assert!(soul_is_light(light_after));
        assert!(still_unsealed_until_eq(light_after));
    }

    /// CARD F5 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f5_place_id_local_hexes_len_three() {
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert_eq!(PeopleLanding::Threshold.place_id(), PlaceId::Heartwood);
        assert_eq!(
            PeopleLanding::SanctuaryWellFromAbove.place_id(),
            PlaceId::Sanctuary
        );
        assert!(four_place_landings_only());
    }

    /// CARD F5 — STEWARD_ONLINE_YES false.
    #[test]
    fn f5_steward_online_yes_false() {
        use shared::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!shared::hex_protocol::default_client_listens());
    }

    /// CARD F5 — no fifth Place · no Title race lobby.
    #[test]
    fn f5_no_fifth_place_no_title_race_lobby() {
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        assert!(four_place_landings_only());
        assert!(!f5_title_is_race_lobby());
        assert!(!garden_roster_is_race_portrait_lobby());
        let light = play_new_light_soul();
        assert!(light.last_place().is_none());
        assert_eq!(light.last_place_name(), "Garden");
        assert!(!light.dress_line().contains("portrait"));
        for place in shared::hex_travel::LOCAL_HEXES {
            assert_ne!(place.as_str(), "garden");
            assert_ne!(place.as_str(), "market");
        }
    }

    /// CARD F6 — Continue shows sealed souls in People dress @ last Place.
    #[test]
    fn f6_continue_shows_sealed_souls_people_dress_last_place() {
        assert!(continue_bodies_from_hour_two_json("{}").is_empty());
        assert!(continue_body_line(play_new_light_soul()).is_none());
        assert!(!continue_is_the_body(play_new_light_soul()));

        let json = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        let list = continue_bodies_from_hour_two_json(&json);
        assert_eq!(list.len(), 1, "S2 keys are one sealed body, not a portraits grid");
        assert_ne!(list.len(), HOUSE_PEOPLES.len());
        let soul = list[0];
        assert!(continue_is_the_body(soul));
        assert!(!soul.is_light());
        assert_eq!(soul.dress_line(), "Cydruid · human-in-frame");
        assert!(!soul.dress_line().contains("treant"));
        assert!(!soul.dress_line().contains("portrait"));
        assert_eq!(soul.last_place(), Some(PlaceId::Heartwood));
        assert_eq!(soul.last_place_name(), "Heartwood");
        assert_eq!(
            continue_body_line(soul).as_deref(),
            Some("Cydruid · human-in-frame · Heartwood")
        );
        assert_eq!(
            soul.sealed_pair(),
            Some((HousePeople::Cydruid, PeopleLanding::Heartwood))
        );

        let draek = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Draek,
            PeopleLanding::DepthsTealWayHome,
        );
        let list = continue_bodies_from_hour_two_json(&draek);
        assert!(continue_is_the_body(list[0]));
        assert_eq!(list[0].dress_line(), "Draek");
        assert_eq!(list[0].last_place(), Some(PlaceId::Depths));
        assert_eq!(
            continue_body_line(list[0]).as_deref(),
            Some("Draek · Depths")
        );
    }

    /// CARD F6 — delete-soul / clear seal → that slot returns to light (Play).
    #[test]
    fn f6_delete_soul_clear_seal_slot_returns_light_play() {
        let existing = r#"{"charter_id":"house-local","hex":"Frontier","kind":"House"}"#;
        let json = merge_gate_seal_into_hour_two_json(
            existing,
            HousePeople::Quellorian,
            PeopleLanding::Threshold,
        );
        assert!(continue_is_the_body(
            continue_bodies_from_hour_two_json(&json)[0]
        ));
        assert_eq!(
            gate_seal_from_hour_two_json(&json),
            Some((HousePeople::Quellorian, PeopleLanding::Threshold))
        );

        let (cleared, slot) = delete_soul_returns_light(&json);
        assert_eq!(slot, GardenRosterSoul::LightUnsealed);
        assert!(slot.is_light());
        assert!(slot.is_unsealed());
        assert_eq!(slot.dress_line(), "light");
        assert!(slot.last_place().is_none());
        assert_eq!(slot.last_place_name(), "Garden");
        assert!(!continue_is_the_body(slot), "cleared slot is Play, not Continue");
        assert!(continue_bodies_from_hour_two_json(&cleared).is_empty());
        assert!(gate_seal_from_hour_two_json(&cleared).is_none());
        assert!(!cleared.contains(SEALED_PEOPLE_KEY));
        assert!(!cleared.contains(SEALED_LANDING_KEY));
        assert!(cleared.contains("house-local"), "S2 pack fields stay; only seal keys leave");
        let loaded = HourTwoPack::from_json(&cleared);
        assert_eq!(loaded.session.charter_id.as_deref(), Some("house-local"));
        assert_eq!(HOUR_TWO_PATH, "data/powrush_hour_two.json");

        let twice = clear_gate_seal_from_hour_two_json(&cleared);
        assert!(gate_seal_from_hour_two_json(&twice).is_none());
        assert!(continue_bodies_from_hour_two_json(&twice).is_empty());
    }

    /// CARD F6 — no race-portrait lobby.
    #[test]
    fn f6_no_race_portrait_lobby() {
        assert!(!f6_title_is_race_lobby());
        assert!(!garden_roster_is_race_portrait_lobby());
        assert!(!f5_title_is_race_lobby());
        let light = play_new_light_soul();
        assert!(light.is_light(), "new slot is light, not a race picker");
        assert!(!light.dress_line().contains("portrait"));
        assert!(!light.dress_line().contains("class"));
        let empty = continue_bodies_from_hour_two_json("{}");
        assert!(empty.is_empty());
        assert_ne!(empty.len(), HOUSE_PEOPLES.len());
        let json = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        let list = continue_bodies_from_hour_two_json(&json);
        assert_eq!(list.len(), 1, "sealed list is souls, not five portraits");
        assert!(!list[0].dress_line().contains("Sanctuary tint"));
        assert!(!list[0].dress_line().contains("dress token"));
        assert!(!list[0].dress_line().contains("portrait"));
    }

    /// CARD F6 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f6_place_id_local_hexes_len_three() {
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert_eq!(PeopleLanding::Threshold.place_id(), PlaceId::Heartwood);
        assert_eq!(
            PeopleLanding::SanctuaryWellFromAbove.place_id(),
            PlaceId::Sanctuary
        );
        assert!(four_place_landings_only());
        let light = play_new_light_soul();
        assert!(light.last_place().is_none());
        assert_eq!(light.last_place_name(), "Garden");
        for place in shared::hex_travel::LOCAL_HEXES {
            assert_ne!(place.as_str(), "garden");
            assert_ne!(place.as_str(), "market");
        }
    }

    /// CARD F6 — STEWARD_ONLINE_YES false.
    #[test]
    fn f6_steward_online_yes_false() {
        use shared::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!shared::hex_protocol::default_client_listens());
    }
}
