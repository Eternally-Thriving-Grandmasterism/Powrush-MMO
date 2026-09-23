//! S0 Title + S2 House naming + D1 Pause + D2 Settings + D3 seals/heritage (v23.2.64)
//!
//! High-contrast title plate (opaque light-on-dark — soft GPU / Mesa readable).
//! Esc from InYard opens D1 pause (not Title / not quit-to-desktop); quit via
//! window close / Pause Quit. Esc again (or Resume) closes pause; Title button
//! still returns to Title with house JSON + lived persist.
//! D1: when Settings/pause opens in yard — one-line plate "the yard is waiting"
//! with Resume / Title / Quit (Title = old Esc-to-title path; Quit = AppExit).
//! D2: same plate hosts local Look / Mute / Invert-Y / Hide slabs; persist
//! `data/powrush_settings.json` beside house JSON. Online stays grey — no socket.
//! H-2026-09-12-PAUSE-TABS: Esc plate splits Comfort · Controls · Guide tabs
//! (Online stays grey).
//! CARD FLESH-GUIDE-BREATH: Guide is one sentence of the locked peak memory
//! (well · tend · week · the yard remembered). Not five stacked lines.
//! CARD FLESH-CONTINUE-LINE: the existing Continue cue on this plate speaks
//! that same peak (`yard remembered`). Shared still returns
//! `the yard remembers`; this plate says `the yard remembered` in that one
//! beat. No second HUD. No new verb.
//! H-2026-09-12-PLACES-DOOR: Settled+book **Places** door on that plate opens the
//! four-room Places plate — must not only dismiss pause.
//! H-2026-09-12-COMFORT-PRESETS: Esc Comfort Graphics · Low|Medium(default)|High
//! persists beside Grove; applies brightness/text scale/reduced motion/rumble.
//! H-2026-09-12-MESH-LOD: GraphicsPreset → mesh LOD; first-launch dismissible
//! Comfort banner ("Graphics can go Higher… Esc → Comfort"); Online grey.
//! H-2026-09-12-PLACES-OVERLAY: Esc Places opens four-room plate after Settled+book;
//! Comfort overlay / graphics banner must not linger over that Places door.
//! H-2026-09-12-PLACES-CLICK: Places row Pressed opens four-room plate; Comfort
//! SettingsStubRoot uses Display::None while places_open (soft-GPU linger fix).
//! After-D3 comfort: Brightness · Text scale on same plate; Mute-from-pause = MasterMute;
//! G0.5: Grove · off|light on same plate (persist; default off; OR with POWRUSH_GEN);
//! P3: LAN · off|loopback beside Grove (default off; 127.0.0.1 only; Title Online stays grey);
//! L1: this hex admits harm · off confirm (Settings / Q / Ledger) after Settled + book;
//! U2: Places list (Sanctuary / Heartwood) after Settled + book — dedicated plate,
//! not extra Settings rows (Title / pause / Settings stay above world);
//! Q/Pause face shows Seal · … when dressed (heritage string only).
//! D3: after Settled / skip-named — three skippable Peace-tone seals (Well · Grove ·
//! Ember) + optional heritage caption (none|human|cydruid|quellorian|draek|ambrosian);
//! rename allowed; persist seals+heritage on powrush_house.json; refuse +take/+STR.
//! MERCY_PERSONA P2/P4/P5: gated persona creator UI on Title behind `PERSONA_CREATOR_ENABLED`
//! (shared/persona.rs). Flag off → Hour 1 nameless Steward unchanged; no LLM; Online grey.
//! MechanicalRace shown as lattice module only — never Title race-lobby as power.
//! P4: Preview soft draft → **Commit** via `PersonaCommit` (validate+persist). Keep draft
//! remains local soft-caps only. Skip = nameless Steward. Online stays grey.
//! P5: story-provider picker `None | LocalTemplate | RathorOfflineShard | (opt-in online)`
//! behind `ONLINE_PICKER_ENABLED` (default false) + steward `online yes`. Never lights
//! Title Online; GrokOnline/OpenAi never first-run default. No new HUD / sockets.
//! No race select at Title lobby. No new Peace keys. No Online socket. No preview tag.
//! Fog/birds visual comfort PARKED (Title contrast law).
//! Pause→Title + Settled write data/powrush_house.json even if name skipped.
//! Continue Unnamed House + yard remembers; Online grey; SmolStr drain.
//!
//! CARD L1 GARDEN-WANT — Garden / boot plane (walkable title · God-plane, D0
//! EDEN-PLANE-LAW @ 2afff36) speaks the retargeted L1 SANCTUARY-WANT line
//! before Play lands in Sanctuary dirt. Cite PLACE_DRESS Garden≠Sanctuary ·
//! ART_BIBLE / PLAYABLE_RACES (Human — cite only). H hush still works.
//! Comfort Low is mesh LOD + text_scale, not a text gate. 0 meshes · 0 new
//! verbs · 0 Places. Title stays Play / Continue / Settings · Online grey.
//!
//! CARD L2 HOUSE-PEOPLE-GATES — Garden / boot hosts five God-plane doors
//! (D0 @ 2afff36) after Q House. Four Place landings only. Doors ignite
//! after one Tend. Crossing one-way this session. Skip House = stay light.
//! Title stays Play / Continue / Settings · Online grey. No race portraits.
//! 0 meshes · ASSET_BUDGET_COURT cite only. Not the #459 dress-token line.
//!
//! CARD L5 TITLE-GARDEN-LAND — garden_cross_landing calls the L3/L4 wire
//! (`hex_travel::try_cross_people_door_land` → apply_people_landing).
//! House + Tend → dressed Place. Skip House / no Tend → none, PlaceId
//! unchanged, Peace light-body. Title stays Play / Continue / Settings ·
//! Online grey. 0 meshes · 0 portraits · no Title lobby. Cite #465 #466
//! · D0 · C0 · PLACE_DRESS_SPEC · PLAYABLE_RACES §1.1.
//!
//! CARD L6 LANDING-WANT — Title still calls garden_boot_want_line.
//! Garden boot (no land) keeps GARDEN_WANT. After land, Want follows
//! PlaceId in first-session guidance (not Title chrome). Do not restyle
//! Play / Continue / Settings · Online grey.
//!
//! CARD S1 AFTERMATH-EVIDENCE — Title garden guidance after land (or on
//! Place Want) speaks Place-local aftermath via
//! `garden_guidance_after_land`. Skip House keeps GARDEN_WANT. Not a
//! trailer. Cite lore on tip. 0 meshes · 0 Imagine pack import.
//! Play / Continue / Settings · Online grey unchanged. L7 beat untouched.
//!
//! CARD S3 GARDEN-ROSTER — Title chrome stays Play / Continue / Settings ·
//! Online grey. Play = new ungenerated light soul (garden / God-plane),
//! unsealed. Continue = sealed souls from S2 hour-two keys, each in
//! People dress, resume at last sealed Place. Light form is only the
//! new / unsealed slot. No Title race/class lobby · no portraits grid.
//! PlaceId stays 3. Cite hour_sacred S2 helpers. No new persist schema.
//!
//! CARD F6 CONTINUE-IS-THE-BODY — Title Continue list IS the sealed soul
//! as body (People dress + last Place). Delete-soul / clear seal returns
//! that slot to light (Play). No race-portrait lobby. PlaceId stays 3.
//! Online grey. S2 disk keys reuse. S3 roster helpers cited, not rewritten.
//!
//! CARD F7 PLACE-AFTERMATH-VARIANTS — Title garden Want plate after land
//! is the existing S1 aftermath plus exactly one extra local-evidence
//! lore line per People. Not a trailer. Skip House keeps GARDEN_WANT.
//! Cite-only (Clerk exact): docs/DRIVE_LORE_ADAPTATION.md ·
//! docs/PLAYABLE_RACES.md · docs/ART_BIBLE.md. Do not invent lore.
//! PlaceId stays 3. 0 meshes. S2 / S3 / F5 / F6 WRITE unread.
//! Play / Continue / Settings · Online grey unchanged.
//! Contact: info@Rathor.ai

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use shared::house_name::{
    continue_cue_when_persist, local_persist_present, HouseName, HOUSE_PATH, HOUSE_SEALS,
    SEAL_EMBER, SEAL_GROVE, SEAL_WELL, UNNAMED,
};
use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};
use shared::persona::{
    cycle_story_provider, default_story_provider, online_picker_allows_online_rows,
    persona_copy_is_honest, resolve_story_provider, story_provider_btn_label,
    story_provider_may_select, CommitError, CustomPeople, MechanicalRace, PeopleChoice,
    PeoplePreset, Persona, PersonaCommit, Phenotype, StoryProvider, StoryShare,
    GIVEN_NAME_MAX, ONLINE_PICKER_ENABLED, PERSONA_CREATOR_ENABLED, PERSONA_PATH,
    STEWARD_ONLINE_YES, STORY_TEXT_MAX,
};

use crate::embassy::EmbassyYard;
use crate::first_session_guidance::{
    f7_garden_guidance_after_land, first_minutes_people_want_line, FirstSessionGuidance,
};
use crate::hex_travel::{
    apply_place, apply_title_boot, settings_visible_with_places, try_cross_people_door_land,
    HexTravelState, PausePlacesBtn, PlacesDoorClickSet, PlacesPlate,
};
use crate::hour_sacred::{
    continue_bodies_from_hour_two_json, continue_is_the_body, continue_sealed_souls_from_hour_two_json,
    delete_soul_returns_light, f6_title_is_race_lobby, four_place_landings_only,
    garden_roster_is_race_portrait_lobby, god_plane_doors_ignited, offer_house_peoples,
    play_new_light_soul, read_hour_two_json, skip_house_stays_light, GardenRosterSoul, HousePeople,
    HourSacred, PeopleLanding, HOUSE_PEOPLES, HOUR_TWO_PATH, L2_ASSET_BUDGET_CITE, L2_MESH_BUDGET,
};
use crate::human_presence::SoftPresence;
use crate::input::{InputMapSet, PlayerInput};
use crate::lived_hour_bind::LivedHourBind;
use crate::lived_hour_bind::{SHARD_CLIMATE_PATH, SHARD_STANDING_PATH};
use crate::local_settings::LocalSettingsState;
use crate::net_mode::SessionNetMode;
use crate::ui_above_world::{LivedUiPlate, LIVED_UI_Z_PAUSE, LIVED_UI_Z_TITLE};
use shared::hex_travel::{BootKind, PLACES_ROW};
use shared::local_settings::{
    refuse_online_socket_toggle, LocalSettings, PeaceKey, SETTINGS_PATH,
    COMFORT_GRAPHICS_BANNER_COPY,
};
#[cfg(test)]
use shared::local_settings::GraphicsPreset;
use shared::pause_ledger_face::lethal_sign_row;

/// H-2026-09-12-MESH-LOD — GraphicsPreset → mesh LOD (PATHS-only compile via title_screen).
#[path = "gltf_integration.rs"]
pub mod gltf_integration;

// --- High-contrast title palette (opaque — no alpha-on-fog) -----------------
// Soft GPU / Mesa must read Play · Continue · Online · Settings before the yard.
/// Opaque dark plate behind menu text (light-on-dark).
pub const TITLE_PLATE_BG: Color = Color::srgb(0.05, 0.07, 0.09);
/// Full-screen dimmer over the world (opaque dark — stranger reads the door).
pub const TITLE_DIM_BG: Color = Color::srgb(0.02, 0.03, 0.04);
/// Primary menu / title text — high contrast on TITLE_PLATE_BG.
pub const TITLE_TEXT_PRIMARY: Color = Color::srgb(0.96, 0.98, 0.94);
/// Secondary cue / subtitle text.
pub const TITLE_TEXT_SECONDARY: Color = Color::srgb(0.82, 0.92, 0.86);
/// Enabled button fill (opaque).
pub const TITLE_BTN_BG: Color = Color::srgb(0.12, 0.18, 0.15);
/// Enabled button label.
pub const TITLE_BTN_FG: Color = Color::srgb(0.96, 0.98, 0.95);
/// Disabled / Online-grey fill.
pub const TITLE_BTN_DISABLED_BG: Color = Color::srgb(0.10, 0.11, 0.12);
/// Disabled / Online-grey label.
pub const TITLE_BTN_DISABLED_FG: Color = Color::srgb(0.55, 0.58, 0.60);
/// Plate + button border (opaque green).
pub const TITLE_BORDER: Color = Color::srgb(0.45, 0.78, 0.58);

/// U5 Steam Deck layout contract. The title plate remains inside this surface.
pub const DECK_TITLE_WIDTH: f32 = 1280.0;
pub const DECK_TITLE_HEIGHT: f32 = 800.0;
/// Edge room for Deck/window decorations and UI scaling.
pub const TITLE_SAFE_INSET: f32 = 24.0;
/// Existing desktop title width, now used as a responsive maximum.
pub const TITLE_PLATE_MAX_WIDTH: f32 = 420.0;

/// Pure layout guard used by the Deck-sized title test.
pub fn title_plate_fits_surface(surface: Vec2, plate: Vec2, safe_inset: f32) -> bool {
    safe_inset >= 0.0
        && plate.x >= 0.0
        && plate.y >= 0.0
        && plate.x <= (surface.x - safe_inset * 2.0).max(0.0)
        && plate.y <= (surface.y - safe_inset * 2.0).max(0.0)
}

/// D1 Pause honesty one-liner (opaque plate — soft GPU readable).
pub const YARD_WAITING: &str = "the yard is waiting";

/// CARD L2 — Title chrome contract. No People-door rows. No race portraits.
pub const TITLE_CHROME_PLAY: &str = "Play — first Hands";
pub const TITLE_CHROME_CONTINUE: &str = "Continue";
pub const TITLE_CHROME_SETTINGS: &str = "Settings";

/// CARD L2 — Garden / boot (walkable title · God-plane) hosts five People-doors.
/// Law only — not Title chrome, not race portraits. Cite D0 EDEN-PLANE-LAW @ 2afff36.
pub fn garden_boot_god_plane_doors() -> [HousePeople; 5] {
    HOUSE_PEOPLES
}

/// Title stays Play / Continue / Settings · Online grey. 0 meshes. No portraits.
pub fn l2_title_chrome_holds() -> bool {
    online_row_is_honest_disabled(ONLINE_STUB_LABEL, false)
        && LaunchDoor::default() == LaunchDoor::Title
        && L2_MESH_BUDGET == 0
        && L2_ASSET_BUDGET_CITE == "docs/ASSET_BUDGET_COURT.md"
        && !title_has_race_portraits()
        && four_place_landings_only()
        && offer_house_peoples(false).is_none()
        && skip_house_stays_light(false)
        && garden_boot_god_plane_doors().len() == 5
}

/// Peoples are post-House God-plane doors, never Title portraits / lobby art.
pub fn title_has_race_portraits() -> bool {
    false
}

/// CARD S3 — Title is not a race / class lobby.
pub fn title_has_race_class_lobby() -> bool {
    false
}

/// CARD S3 — Continue is a sealed-soul list, never a portraits grid.
pub fn title_has_portraits_grid() -> bool {
    false
}

/// CARD S3 — Play / Continue / Settings chrome + Online grey + no lobby.
pub fn s3_title_chrome_holds() -> bool {
    TITLE_CHROME_PLAY == "Play — first Hands"
        && TITLE_CHROME_CONTINUE == "Continue"
        && TITLE_CHROME_SETTINGS == "Settings"
        && l2_title_chrome_holds()
        && !title_has_race_portraits()
        && !title_has_race_class_lobby()
        && !title_has_portraits_grid()
        && !garden_roster_is_race_portrait_lobby()
        && !STEWARD_ONLINE_YES
        && shared::hex_travel::LOCAL_HEXES.len() == 3
}

/// CARD S3 — Continue roster line: People dress · last sealed Place.
pub fn s3_continue_roster_line(soul: GardenRosterSoul) -> String {
    format!("{} · {}", soul.dress_line(), soul.last_place_name())
}

/// CARD S3 — Cue text for sealed souls. Empty when the S2 keys are absent.
pub fn s3_continue_roster_cue(raw: Option<&str>) -> Option<String> {
    let raw = raw?;
    let souls = continue_sealed_souls_from_hour_two_json(raw);
    if souls.is_empty() {
        return None;
    }
    Some(
        souls
            .iter()
            .copied()
            .map(s3_continue_roster_line)
            .collect::<Vec<_>>()
            .join(" · "),
    )
}

/// CARD S3 — Play boots a new light unsealed soul. Does not apply the S2 seal.
pub fn s3_play_boot(
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    hour: &HourSacred,
    embassy: Option<&mut EmbassyYard>,
) -> GardenRosterSoul {
    let soul = play_new_light_soul();
    apply_title_boot(BootKind::Play, travel, bind, hour, embassy);
    soul
}

/// CARD S3 — Continue lists S2 sealed souls and resumes at last sealed Place.
/// Missing seal keeps the existing house Continue boot.
pub fn s3_continue_boot(
    raw: Option<&str>,
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    hour: &HourSacred,
    embassy: Option<&mut EmbassyYard>,
) -> Vec<GardenRosterSoul> {
    let roster = raw
        .map(continue_sealed_souls_from_hour_two_json)
        .unwrap_or_default();
    if let Some(place) = roster.first().and_then(|soul| soul.last_place()) {
        apply_place(travel, bind, embassy, place, true);
        return roster;
    }
    apply_title_boot(BootKind::Continue, travel, bind, hour, embassy);
    roster
}

/// CARD F6 — Play / Continue / Settings chrome + Online grey + no lobby.
pub fn f6_title_chrome_holds() -> bool {
    s3_title_chrome_holds()
        && !title_has_race_portraits()
        && !title_has_race_class_lobby()
        && !title_has_portraits_grid()
        && !f6_title_is_race_lobby()
        && !STEWARD_ONLINE_YES
        && shared::hex_travel::LOCAL_HEXES.len() == 3
}

/// CARD F6 — Continue list IS the sealed soul body. Empty = Play (light).
pub fn f6_continue_bodies(raw: Option<&str>) -> Vec<GardenRosterSoul> {
    raw.map(continue_bodies_from_hour_two_json).unwrap_or_default()
}

/// CARD F6 — People dress · last Place. None when the slot is light (Play).
pub fn f6_continue_body_line(soul: GardenRosterSoul) -> Option<String> {
    if !continue_is_the_body(soul) {
        return None;
    }
    Some(s3_continue_roster_line(soul))
}

/// CARD F6 — Title cue for Continue bodies. Empty when S2 keys are absent.
pub fn f6_continue_is_the_body_cue(raw: Option<&str>) -> Option<String> {
    let bodies = f6_continue_bodies(raw);
    if bodies.is_empty() {
        return None;
    }
    let lines: Vec<String> = bodies.iter().copied().filter_map(f6_continue_body_line).collect();
    if lines.is_empty() {
        None
    } else {
        Some(lines.join(" · "))
    }
}

/// CARD F6 — Continue boot is the sealed soul body at last Place. Cites S3.
pub fn f6_continue_body_boot(
    raw: Option<&str>,
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    hour: &HourSacred,
    embassy: Option<&mut EmbassyYard>,
) -> Vec<GardenRosterSoul> {
    s3_continue_boot(raw, travel, bind, hour, embassy)
}

/// CARD F6 — delete-soul / clear seal. That Continue slot returns to light (Play).
pub fn f6_delete_soul_returns_play(raw: &str) -> (String, GardenRosterSoul) {
    delete_soul_returns_light(raw)
}

/// CARD F7 — Play / Continue / Settings chrome + Online grey. Aftermath
/// variants are Want-plate words only. 0 meshes. PlaceId stays 3.
pub fn f7_title_chrome_holds() -> bool {
    TITLE_CHROME_PLAY == "Play — first Hands"
        && TITLE_CHROME_CONTINUE == "Continue"
        && TITLE_CHROME_SETTINGS == "Settings"
        && l2_title_chrome_holds()
        && !title_has_race_portraits()
        && !STEWARD_ONLINE_YES
        && shared::hex_travel::LOCAL_HEXES.len() == 3
        && L2_MESH_BUDGET == 0
}

/// CARD F7 — Title garden Want plate: existing S1 aftermath plus exactly
/// one extra local-evidence line per People. Skip House keeps GARDEN_WANT.
/// H hush drops the line. F6 Continue WRITE unread.
pub fn f7_title_garden_want_plate(
    hush: bool,
    landing: Option<PeopleLanding>,
) -> Option<String> {
    f7_garden_guidance_after_land(true, hush, landing)
}

/// Garden door may be crossed only after House + one Tend, once this session.
pub fn garden_door_may_cross(house_live: bool, tended_once: bool, already_crossed: bool) -> bool {
    house_live && god_plane_doors_ignited(tended_once) && !already_crossed
}

/// Cross one Garden / God-plane door. One-way this session. Not Title chrome.
/// CARD L5 — same L3/L4 wire Esc→Places dress uses. Skip House / no Tend
/// returns none and does not move PlaceId (Peace light-body).
pub fn garden_cross_landing(
    house_live: bool,
    tended_once: bool,
    crossed: &mut Option<HousePeople>,
    people: HousePeople,
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    embassy: Option<&mut EmbassyYard>,
    presence: Option<&mut SoftPresence>,
) -> Option<PeopleLanding> {
    if !garden_door_may_cross(house_live, tended_once, crossed.is_some()) {
        return None;
    }
    try_cross_people_door_land(
        house_live,
        tended_once,
        crossed,
        people,
        travel,
        bind,
        embassy,
        presence,
    )
}

/// Guide tab — one sentence of the locked peak memory.
/// Well · tend · the week was the bill · quit · the yard remembered.
/// One text node (not five stacked lines). Online stays grey. No second HUD.
pub const PAUSE_GUIDE_LINE: &str =
    "I walked to a well, tended it, the week was the bill, I quit, and the yard remembered.";

/// Present-tense persist beat from shared (`the yard remembers`).
/// The Continue plate speaks the Guide past in that same beat.
const CONTINUE_PRESENT_YARD: &str = "the yard remembers";

/// Guide peak already on [`PAUSE_GUIDE_LINE`]. One beat, not a second slogan.
pub const CONTINUE_YARD_REMEMBERED: &str = "the yard remembered";

/// CARD FLESH-CONTINUE-LINE — Continue copy already on the title plate.
/// Shared still returns `the yard remembers`. This plate says `yard remembered`
/// (the Guide peak) in that one beat. No persist → the Play cue is unchanged.
/// No second HUD. No new verb.
pub fn continue_yard_remembered_line(house_cue: &str) -> String {
    match house_cue.split_once(CONTINUE_PRESENT_YARD) {
        Some((head, tail)) => {
            let mut line = String::with_capacity(house_cue.len() + 2);
            line.push_str(head);
            line.push_str(CONTINUE_YARD_REMEMBERED);
            line.push_str(tail);
            line
        }
        None => house_cue.to_string(),
    }
}

/// Relative luminance from linear-ish sRGB channels (Bevy 0.14 Color::Srgba).
pub fn title_luminance(c: Color) -> f32 {
    let s = match c {
        Color::Srgba(srgba) => srgba,
        other => other.to_srgba(),
    };
    0.2126 * s.red + 0.7152 * s.green + 0.0722 * s.blue
}

/// True when primary text is clearly brighter than the plate (soft-GPU readable).
pub fn title_contrast_is_high() -> bool {
    let plate = title_luminance(TITLE_PLATE_BG);
    let text = title_luminance(TITLE_TEXT_PRIMARY);
    text - plate >= 0.55
}

/// Alpha channel of a Color (1.0 = opaque plate / no alpha-on-fog).
pub fn title_alpha(c: Color) -> f32 {
    match c {
        Color::Srgba(srgba) => srgba.alpha,
        other => other.to_srgba().alpha,
    }
}

/// Boot door. Title until Play/Continue. Naming is optional after Settled / quit.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchDoor {
    Title,
    InYard,
    NameHouse,
    /// D3: seals + heritage caption after Settled / skip-named.
    HouseDress,
}

impl Default for LaunchDoor {
    fn default() -> Self {
        Self::Title
    }
}

#[derive(Resource, Debug, Clone)]
pub struct HouseLabel {
    pub house: HouseName,
    pub persist_present: bool,
    pub hour_two_held: bool,
    pub book_held: bool,
    pub settings_open: bool,
    pub draft: String,
    /// Offer naming once after Settled / Escape.
    pub naming_offered: bool,
    /// Offer D3 seals/heritage once after name resolved.
    pub seals_offered: bool,
}

impl Default for HouseLabel {
    fn default() -> Self {
        let house = HouseName::load_or_default();
        let hour_two_json = shared::user_persist::named_exists(HOUR_TWO_PATH)
            .then(|| shared::user_persist::read_named(HOUR_TWO_PATH).ok())
            .flatten()
            .and_then(|r| serde_json::from_str::<serde_json::Value>(&r).ok());
        let hour_two_held = hour_two_json
            .as_ref()
            .and_then(|v| v.get("complete").and_then(|c| c.as_bool()))
            .unwrap_or(false);
        let book_held = hour_two_json
            .as_ref()
            .and_then(|v| v.get("hour_three_complete").and_then(|c| c.as_bool()))
            .unwrap_or(false);
        let persist_present = local_persist_present(
            shared::user_persist::named_exists(HOUR_TWO_PATH),
            shared::user_persist::named_exists(SHARD_CLIMATE_PATH),
            shared::user_persist::named_exists(SHARD_STANDING_PATH),
            house.resolved,
        );
        let seals_offered = house.seals_resolved;
        Self {
            house,
            persist_present,
            hour_two_held,
            book_held,
            settings_open: false,
            draft: String::new(),
            naming_offered: false,
            seals_offered,
        }
    }
}

#[derive(Component)]
struct TitleRoot;
#[derive(Component)]
struct TitlePlate;
#[derive(Component)]
struct TitleCueText;
/// CARD L1 GARDEN-WANT — People + Want spoken on the walkable title / God-plane.
#[derive(Component)]
struct TitleGardenWantText;
#[derive(Component)]
struct TitleBreath;
#[derive(Component)]
struct TitlePlayBtn;
#[derive(Component)]
struct TitleContinueBtn;
#[derive(Component)]
struct TitleSettingsBtn;
#[derive(Component)]
#[allow(dead_code)]
struct TitleOnlineBtn;
#[derive(Component)]
struct SettingsStubRoot;
/// First-launch Comfort graphics banner (MESH-LOD) — title / yard pause.
#[derive(Component)]
struct ComfortGraphicsBannerRoot;
#[derive(Component)]
struct ComfortGraphicsBannerDismissBtn;
#[derive(Component)]
struct PauseCueText;
#[derive(Component)]
struct PauseResumeBtn;
#[derive(Component)]
struct PauseTitleBtn;
#[derive(Component)]
struct PauseQuitBtn;

/// Esc pause plate tabs — Comfort (default) · Controls · Guide.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PauseTab {
    #[default]
    Comfort,
    Controls,
    Guide,
}

impl PauseTab {
    const ALL: [PauseTab; 3] = [PauseTab::Comfort, PauseTab::Controls, PauseTab::Guide];

    const fn label(self) -> &'static str {
        match self {
            Self::Comfort => "Comfort",
            Self::Controls => "Controls",
            Self::Guide => "Guide",
        }
    }
}

#[derive(Component)]
struct PauseTabBtn(PauseTab);
#[derive(Component)]
struct PauseTabPanel(PauseTab);

#[derive(Component)]
struct SettingsLookBtn;
#[derive(Component)]
struct SettingsMuteBtn;
#[derive(Component)]
struct SettingsInvertBtn;
#[derive(Component)]
struct SettingsHideSlabsBtn;
#[derive(Component)]
struct SettingsLookLabel;
#[derive(Component)]
struct SettingsMuteLabel;
#[derive(Component)]
struct SettingsInvertLabel;
#[derive(Component)]
struct SettingsHideLabel;
#[derive(Component)]
struct SettingsBrightnessBtn;
#[derive(Component)]
struct SettingsBrightnessLabel;
#[derive(Component)]
struct SettingsTextScaleBtn;
#[derive(Component)]
struct SettingsTextScaleLabel;
#[derive(Component)]
struct SettingsGraphicsBtn;
#[derive(Component)]
struct SettingsGraphicsLabel;
#[derive(Component)]
struct SettingsGroveBtn;
#[derive(Component)]
struct SettingsGroveLabel;
#[derive(Component)]
struct SettingsReducedMotionBtn;
#[derive(Component)]
struct SettingsReducedMotionLabel;
#[derive(Component)]
struct SettingsRumbleBtn;
#[derive(Component)]
struct SettingsRumbleLabel;
#[derive(Component)]
struct SettingsColorblindWellsBtn;
#[derive(Component)]
struct SettingsColorblindWellsLabel;
#[derive(Component)]
struct SettingsLanBtn;
#[derive(Component)]
struct SettingsLanLabel;
#[derive(Component)]
struct SettingsLethalBtn;
#[derive(Component)]
struct SettingsLethalLabel;
#[derive(Component)]
struct SettingsSticksBtn;
#[derive(Component)]
struct SettingsSticksLabel;
#[derive(Component)]
struct SettingsTapUseBtn;
#[derive(Component)]
struct SettingsTapUseLabel;
#[derive(Component)]
struct SettingsSprintBtn;
#[derive(Component)]
struct SettingsSprintLabel;
#[derive(Component)]
struct SettingsOnlineStubBtn;
#[derive(Component)]
struct SettingsPeaceBindBtn(PeaceAction);
#[derive(Component)]
struct SettingsPeaceBindLabel(PeaceAction);
#[derive(Component)]
struct SettingsPeaceResetBtn;
#[derive(Component)]
struct NameHouseRoot;
#[derive(Component)]
struct NameDraftText;
#[derive(Component)]
struct NameConfirmBtn;
#[derive(Component)]
struct NameSkipBtn;
#[derive(Component)]
struct HouseDressRoot;
#[derive(Component)]
struct DressSealWellBtn;
#[derive(Component)]
struct DressSealGroveBtn;
#[derive(Component)]
struct DressSealEmberBtn;
#[derive(Component)]
struct DressSealWellLabel;
#[derive(Component)]
struct DressSealGroveLabel;
#[derive(Component)]
struct DressSealEmberLabel;
#[derive(Component)]
struct DressHeritageBtn;
#[derive(Component)]
struct DressHeritageLabel;
#[derive(Component)]
struct DressRenameBtn;
#[derive(Component)]
struct DressConfirmBtn;
#[derive(Component)]
struct DressSkipBtn;

// --- MERCY_PERSONA P2: gated title creator (PERSONA_CREATOR_ENABLED) ----------
#[derive(Component)]
struct TitlePersonaBtn;
#[derive(Component)]
struct PersonaCreatorRoot;
#[derive(Component)]
struct PersonaCreatorBodyText;
#[derive(Component)]
struct PersonaCreatorStepLabel;
#[derive(Component)]
struct PersonaCreatorGuidance;
#[derive(Component)]
struct PersonaModuleBtn;
#[derive(Component)]
struct PersonaModuleLabel;
#[derive(Component)]
struct PersonaPeopleBtn;
#[derive(Component)]
struct PersonaPeopleLabel;
#[derive(Component)]
struct PersonaPhenotypeBtn;
#[derive(Component)]
struct PersonaPhenotypeLabel;
#[derive(Component)]
struct PersonaStoryShareBtn;
#[derive(Component)]
struct PersonaStoryShareLabel;
#[derive(Component)]
struct PersonaStoryAiStubBtn;
#[derive(Component)]
struct PersonaOnlinePickerBtn;
#[derive(Component)]
struct PersonaOnlinePickerLabel;
#[derive(Component)]
struct PersonaNextBtn;
#[derive(Component)]
struct PersonaBackBtn;
#[derive(Component)]
struct PersonaKeepDraftBtn;
#[derive(Component)]
struct PersonaCommitBtn;
#[derive(Component)]
struct PersonaSkipNamelessBtn;
#[derive(Component)]
struct PersonaHideGuidanceBtn;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PeaceAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    Sprint,
    Use,
    Satchel,
    Hide,
    Allocate,
}

const PEACE_ACTIONS: [PeaceAction; 10] = [
    PeaceAction::MoveUp,
    PeaceAction::MoveDown,
    PeaceAction::MoveLeft,
    PeaceAction::MoveRight,
    PeaceAction::Jump,
    PeaceAction::Sprint,
    PeaceAction::Use,
    PeaceAction::Satchel,
    PeaceAction::Hide,
    PeaceAction::Allocate,
];

#[derive(Resource, Debug, Default)]
struct PeaceRebindState {
    waiting: Option<PeaceAction>,
    notice: Option<String>,
    suppress_shortcuts: bool,
}


/// Creator wizard step. Keep draft = soft-caps local; Commit = P4 PersonaCommit persist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonaCreatorStep {
    MechanicalModule,
    People,
    Phenotype,
    Story,
    Preview,
}

impl PersonaCreatorStep {
    pub fn as_label(self) -> &'static str {
        match self {
            Self::MechanicalModule => "1 · Lattice module (not people / not matchmaking)",
            Self::People => "2 · People / ethnicity (presentation)",
            Self::Phenotype => "3 · Phenotype (appearance sliders)",
            Self::Story => "4 · Story (player text · AI records only)",
            Self::Preview => "5 · Preview (Keep draft soft · Commit persists)",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::MechanicalModule => Self::People,
            Self::People => Self::Phenotype,
            Self::Phenotype => Self::Story,
            Self::Story => Self::Preview,
            Self::Preview => Self::Preview,
        }
    }

    pub fn back(self) -> Self {
        match self {
            Self::MechanicalModule => Self::MechanicalModule,
            Self::People => Self::MechanicalModule,
            Self::Phenotype => Self::People,
            Self::Story => Self::Phenotype,
            Self::Preview => Self::Story,
        }
    }
}

/// Optional starter people tags (non-exhaustive). Custom always available.
pub const PERSONA_PEOPLE_PRESETS: &[&str] = &[
    "West African",
    "East African",
    "Southern African",
    "North African / Amazigh / Egyptian",
    "Levantine / Arab",
    "Persian / Kurdish / Armenian",
    "South Asian",
    "Southeast Asian",
    "East Asian",
    "Central Asian",
    "Pacific",
    "Indigenous Americas",
    "European regional",
    "African American / Afro-Caribbean / Afro-Latino",
    "Latino / Mestizo / Indigenous-Latino",
    "Jewish",
    "Mixed / multi",
];

/// Title persona creator draft plate. Open only when `PERSONA_CREATOR_ENABLED`.
#[derive(Resource, Debug, Clone)]
pub struct PersonaCreatorState {
    pub open: bool,
    pub step: PersonaCreatorStep,
    pub draft: Persona,
    pub guidance_hidden: bool,
    pub people_mode: u8, // 0 preset, 1 custom, 2 mixed, 3 unset
    pub people_preset_ix: usize,
    pub name_draft: String,
    pub story_draft: String,
    pub kept_local: bool,
    /// True after a successful P4 PersonaCommit persist.
    pub committed: bool,
    /// P5 story-provider seat. Default `None` (records-only). Online seats
    /// require `ONLINE_PICKER_ENABLED` + steward `online yes`.
    pub story_provider: StoryProvider,
}

impl Default for PersonaCreatorState {
    fn default() -> Self {
        Self {
            open: false,
            step: PersonaCreatorStep::MechanicalModule,
            draft: Persona::nameless_steward(),
            guidance_hidden: false,
            people_mode: 3,
            people_preset_ix: 0,
            name_draft: String::new(),
            story_draft: String::new(),
            kept_local: false,
            committed: false,
            story_provider: default_story_provider(),
        }
    }
}

/// Gate: creator may open only when shared flag is on.
pub fn persona_creator_may_open(flag: bool) -> bool {
    flag
}

/// Honest title button label. Grey/disabled when gated off — never a race lobby SKU.
pub fn persona_title_btn_label(flag: bool) -> &'static str {
    if flag {
        "Persona · optional"
    } else {
        "Persona · gated (Hour 1 nameless)"
    }
}

/// Story AI stub copy — records only; P2 never calls an LLM.
pub const PERSONA_STORY_AI_STUB: &str = "Story AI · records only · no LLM this slice";

/// P5 online picker UI enabled only when feature flag on **and** steward `online yes`.
/// Even then Title Online stays grey — picker is draft-source only, not net.
pub fn online_picker_ui_enabled(flag: bool, steward_online_yes: bool) -> bool {
    online_picker_allows_online_rows(flag, steward_online_yes)
}

/// Apply picker seat under gates; never invents live LLM assist on commit/keep.
pub fn apply_story_provider_seat(state: &mut PersonaCreatorState) {
    state.story_provider = resolve_story_provider(
        state.story_provider,
        ONLINE_PICKER_ENABLED,
        STEWARD_ONLINE_YES,
    );
    // Without an allowed online seat, keep records-only (no live assist invent).
    if state.story_provider.is_offline() {
        // LocalTemplate / RathorOfflineShard may mark assist as records later;
        // P5 does not call sockets. Clear live invent when None.
        if state.story_provider == StoryProvider::None {
            state.draft.presentation.story.ai_assist_used = false;
            state.draft.presentation.story.model_id = None;
        }
    }
}

/// Guidance lines for the open step (H hides).
pub fn persona_step_guidance(step: PersonaCreatorStep) -> &'static str {
    match step {
        PersonaCreatorStep::MechanicalModule => {
            "Lattice module = how the sim works. Not ethnicity. Not Title matchmaking power."
        }
        PersonaCreatorStep::People => {
            "People / ethnicity is presentation paint. Custom allowed. No faction buff."
        }
        PersonaCreatorStep::Phenotype => {
            "Sliders are appearance only — never gather, Temper, or Hybrid cheats."
        }
        PersonaCreatorStep::Story => {
            "Write your own tale. Provider picker default-off · offline LocalTemplate/records-only · Online grey."
        }
        PersonaCreatorStep::Preview => {
            "Keep draft = soft-caps local. Commit validates + persists PersonaCommit. Skip = nameless Steward."
        }
    }
}

pub fn cycle_mechanical_module(race: MechanicalRace) -> MechanicalRace {
    match race {
        MechanicalRace::Human => MechanicalRace::Quellorian,
        MechanicalRace::Quellorian => MechanicalRace::Draek,
        MechanicalRace::Draek => MechanicalRace::Cydruid,
        MechanicalRace::Cydruid => MechanicalRace::Ambrosian,
        MechanicalRace::Ambrosian => MechanicalRace::Human,
    }
}

pub fn mechanical_module_btn_label(race: MechanicalRace) -> String {
    format!(
        "Lattice module · {} · not people · not matchmaking",
        race.as_str()
    )
}

pub fn cycle_story_share(share: StoryShare) -> StoryShare {
    match share {
        StoryShare::Private => StoryShare::Spoken,
        StoryShare::Spoken => StoryShare::Book,
        StoryShare::Book => StoryShare::Private,
    }
}

pub fn story_share_btn_label(share: StoryShare) -> String {
    let s = match share {
        StoryShare::Private => "Private",
        StoryShare::Spoken => "Spoken",
        StoryShare::Book => "Book",
    };
    format!("Story share · {s}")
}

/// Advance people paint without granting mechanical privilege.
/// Cycles every preset, then custom, mixed, unset — never a race-lobby power.
pub fn bump_people_paint(state: &mut PersonaCreatorState) {
    let n = PERSONA_PEOPLE_PRESETS.len();
    state.people_preset_ix = (state.people_preset_ix + 1) % (n + 3);
    let ix = state.people_preset_ix;
    if ix < n {
        state.people_mode = 0;
        state.draft.presentation.people = PeopleChoice::Preset(PeoplePreset {
            tag: PERSONA_PEOPLE_PRESETS[ix].to_string(),
        });
    } else if ix == n {
        state.people_mode = 1;
        let name = if state.name_draft.is_empty() {
            "Custom people".to_string()
        } else {
            state.name_draft.clone()
        };
        state.draft.presentation.people = PeopleChoice::Custom(CustomPeople {
            name,
            homelands: String::new(),
            languages: vec![],
            customs_note: String::new(),
            phenotype_seed: Phenotype::default(),
            invented: true,
        });
    } else if ix == n + 1 {
        state.people_mode = 2;
        state.draft.presentation.people = PeopleChoice::Mixed {
            parts: vec!["multi".into()],
        };
    } else {
        state.people_mode = 3;
        state.draft.presentation.people = PeopleChoice::Unset;
    }
}

pub fn people_choice_btn_label(state: &PersonaCreatorState) -> String {
    match &state.draft.presentation.people {
        PeopleChoice::Preset(p) => format!("People · preset · {}", p.tag),
        PeopleChoice::Custom(c) => format!(
            "People · custom · {}{}",
            c.name,
            if c.invented { " · invented" } else { "" }
        ),
        PeopleChoice::Mixed { parts } => format!("People · mixed · {}", parts.join("+")),
        PeopleChoice::Unset => "People · unset (nameless ok)".into(),
    }
}

/// Nudge a few phenotype knobs; clamps via soft caps later. No stats.
pub fn nudge_phenotype_paint(ph: &mut Phenotype) {
    ph.skin_melanin = (ph.skin_melanin + 0.1) % 1.0001;
    if ph.skin_melanin > 1.0 {
        ph.skin_melanin = 0.0;
    }
    ph.hair_curl = (ph.hair_curl + 0.15) % 1.0001;
    if ph.hair_curl > 1.0 {
        ph.hair_curl = 0.0;
    }
    ph.freckles = !ph.freckles;
    ph.clamp_unit_interval();
}

pub fn phenotype_btn_label(ph: &Phenotype) -> String {
    format!(
        "Phenotype · melanin {:.2} · curl {:.2} · freckles {}",
        ph.skin_melanin,
        ph.hair_curl,
        if ph.freckles { "on" } else { "off" }
    )
}

pub fn persona_preview_summary(p: &Persona) -> String {
    let people = match &p.presentation.people {
        PeopleChoice::Preset(x) => format!("preset:{}", x.tag),
        PeopleChoice::Custom(c) => format!("custom:{}", c.name),
        PeopleChoice::Mixed { parts } => format!("mixed:{}", parts.join("+")),
        PeopleChoice::Unset => "unset".into(),
    };
    format!(
        "Preview · module {} · name '{}' · people {} · story {} chars · AI assist {} · share {:?}",
        p.mechanical_race.as_str(),
        if p.presentation.given_name.is_empty() {
            "(nameless)"
        } else {
            p.presentation.given_name.as_str()
        },
        people,
        p.presentation.story.player_text.chars().count(),
        p.presentation.story.ai_assist_used,
        p.presentation.story.shared_in_world,
    )
}

/// Sync typed buffers into the soft draft (name / story). No LLM.
pub fn sync_persona_soft_draft_buffers(state: &mut PersonaCreatorState) {
    if !state.name_draft.is_empty() {
        state.draft.presentation.given_name = state.name_draft.clone();
    }
    if !state.story_draft.is_empty() {
        state.draft.presentation.story.player_text = state.story_draft.clone();
        state.draft.presentation.story.player_accepted = true;
    }
}

/// Apply soft caps and keep local draft. Does **not** PersonaCommit (P4).
pub fn keep_persona_local_draft(state: &mut PersonaCreatorState) {
    sync_persona_soft_draft_buffers(state);
    apply_story_provider_seat(state);
    // Creator UI: never invent live LLM assist — records stay offline/local.
    // P5: online picker default-off; do not call GrokOnline/OpenAi.
    if !story_provider_may_select(
        state.story_provider,
        ONLINE_PICKER_ENABLED,
        STEWARD_ONLINE_YES,
    ) || state.story_provider.is_offline()
    {
        if state.story_provider == StoryProvider::None {
            state.draft.presentation.story.ai_assist_used = false;
            state.draft.presentation.story.model_id = None;
        }
    }
    // Hard refuse live invent while gates are off (compile-time defaults).
    if !ONLINE_PICKER_ENABLED || !STEWARD_ONLINE_YES {
        state.draft.presentation.story.ai_assist_used = false;
        if state.story_provider.is_online() {
            state.story_provider = StoryProvider::None;
        }
        state.draft.presentation.story.model_id = None;
    }
    state.draft.apply_soft_caps();
    state.kept_local = true;
    state.committed = false;
    state.open = false;
}

/// Soft draft → PersonaCommit validate + persist. Title Online stays grey.
pub fn commit_persona_from_soft_draft(
    state: &mut PersonaCreatorState,
) -> Result<PersonaCommit, CommitError> {
    sync_persona_soft_draft_buffers(state);
    apply_story_provider_seat(state);
    // PersonaCommit remains the law. Commit path does not light Online /
    // sockets / live LLM — clear assist invent while P5 gates are default-off.
    if !ONLINE_PICKER_ENABLED || !STEWARD_ONLINE_YES || state.story_provider.is_offline() {
        state.draft.presentation.story.ai_assist_used = false;
        state.draft.presentation.story.model_id = None;
        if state.story_provider.is_online() {
            state.story_provider = StoryProvider::None;
        }
    }
    state.draft.apply_soft_caps();
    let committed = PersonaCommit::commit_and_persist(&state.draft)?;
    state.draft = committed.persona.clone();
    state.kept_local = true;
    state.committed = true;
    state.open = false;
    Ok(committed)
}

/// Skip → Hour 1 nameless Steward path.
pub fn skip_persona_to_nameless(state: &mut PersonaCreatorState) {
    *state = PersonaCreatorState::default();
    state.draft = Persona::nameless_steward();
    state.open = false;
    state.kept_local = false;
    state.committed = false;
}

/// Try open from Title. Returns whether the plate opened.
pub fn try_open_persona_creator(state: &mut PersonaCreatorState, flag: bool) -> bool {
    if !persona_creator_may_open(flag) {
        state.open = false;
        return false;
    }
    state.open = true;
    state.step = PersonaCreatorStep::MechanicalModule;
    if state.draft.presentation.given_name.is_empty()
        && matches!(state.draft.presentation.people, PeopleChoice::Unset)
    {
        state.draft = Persona::nameless_steward();
    }
    true
}

/// All P2 title creator strings must pass the honesty gate.
pub fn persona_creator_ui_copy_is_honest() -> bool {
    let picker_off = story_provider_btn_label(
        StoryProvider::None,
        ONLINE_PICKER_ENABLED,
        STEWARD_ONLINE_YES,
    );
    let samples = [
        persona_title_btn_label(true),
        persona_title_btn_label(false),
        PERSONA_STORY_AI_STUB,
        picker_off.as_str(),
        PersonaCreatorStep::MechanicalModule.as_label(),
        PersonaCreatorStep::People.as_label(),
        PersonaCreatorStep::Phenotype.as_label(),
        PersonaCreatorStep::Story.as_label(),
        PersonaCreatorStep::Preview.as_label(),
        persona_step_guidance(PersonaCreatorStep::MechanicalModule),
        persona_step_guidance(PersonaCreatorStep::People),
        persona_step_guidance(PersonaCreatorStep::Story),
        "Skip · nameless Steward",
        "Keep draft · local only",
        "Commit · validate + persist",
        "Lattice module · Human · not people · not matchmaking",
        persona_step_guidance(PersonaCreatorStep::Preview),
    ];
    samples.iter().all(|s| persona_copy_is_honest(s))
}

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LaunchDoor>()
            .init_resource::<HouseLabel>()
            .init_resource::<PauseTab>()
            .init_resource::<LocalSettingsState>()
            .init_resource::<PeaceRebindState>()
            .init_resource::<PersonaCreatorState>()
            .add_systems(
                Startup,
                (
                    spawn_title_screen,
                    spawn_name_house_panel,
                    spawn_house_dress_panel,
                    spawn_persona_creator_panel,
                    spawn_settings_stub,
                    spawn_comfort_graphics_banner,
                ),
            )
            .add_systems(
                Update,
                (
                    breath_title_border,
                    refresh_title_cue,
                    refresh_garden_boot_want,
                    title_button_clicks,
                    title_keyboard_shortcuts,
                    sync_title_visibility,
                    watch_settled_for_naming,
                    name_house_text_input,
                    name_house_buttons,
                    sync_name_house_visibility,
                    house_dress_buttons,
                    refresh_house_dress_labels,
                    sync_house_dress_visibility,
                    refresh_pause_cue,
                    pause_plate_clicks,
                    refresh_local_settings_labels,
                    refresh_controls_settings_labels,
                    refresh_lethal_sign_label,
                    local_settings_clicks,
                    lethal_sign_settings_clicks,
                ),
            )
            // Same-frame as Places row Pressed: hide Comfort before soft-GPU composites.
            .add_systems(
                Update,
                sync_settings_stub.after(PlacesDoorClickSet),
            )
            .add_systems(
                Update,
                (pause_tab_clicks, pause_tab_keys, sync_pause_tabs),
            )
            .add_systems(
                Update,
                (
                    persona_creator_text_input.before(name_house_text_input),
                    persona_creator_buttons,
                    refresh_persona_creator_labels,
                    sync_persona_creator_visibility,
                ),
            )
            .add_systems(
                Update,
                (
                    refresh_accessibility_settings_labels,
                    accessibility_settings_clicks,
                ),
            )
            .add_systems(
                Update,
                (
                    sync_comfort_graphics_banner.after(PlacesDoorClickSet),
                    comfort_graphics_banner_dismiss_clicks,
                ),
            )
            .add_systems(Update, peace_rebind_clicks.before(capture_peace_rebind))
            .add_systems(
                Update,
                capture_peace_rebind
                    .before(title_keyboard_shortcuts)
                    .before(esc_yard_pause),
            )
            .add_systems(
                Update,
                refresh_peace_rebind_labels.after(capture_peace_rebind),
            )
            .add_systems(Update, esc_yard_pause.after(InputMapSet));
    }
}

fn spawn_title_screen(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(14.0),
                    // U5: reserve a Deck-safe edge and constrain the centered
                    // plate instead of scaling or restyling the title.
                    padding: UiRect::all(Val::Px(TITLE_SAFE_INSET)),
                    ..default()
                },
                // Opaque dimmer — soft GPU must not alpha-blend menu into fog.
                background_color: TITLE_DIM_BG.into(),
                z_index: ZIndex::Global(LIVED_UI_Z_TITLE),
                // Consume pointer focus across the opaque title door so clicks
                // in its gaps cannot reach world UI/interactions below it.
                focus_policy: FocusPolicy::Block,
                ..default()
            },
            TitleRoot,
            TitleBreath,
            LivedUiPlate,
        ))
        .with_children(|root| {
            root.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        max_width: Val::Px(TITLE_PLATE_MAX_WIDTH),
                        max_height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(22.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
                        border: UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    background_color: TITLE_PLATE_BG.into(),
                    border_color: TITLE_BORDER.into(),
                    ..default()
                },
                TitlePlate,
            ))
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "POWRUSH",
                    TextStyle {
                        font_size: 28.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ));
                p.spawn(TextBundle::from_section(
                    "Steward House · offline first",
                    TextStyle {
                        font_size: 14.0,
                        color: TITLE_TEXT_SECONDARY,
                        ..default()
                    },
                ));
                p.spawn((
                    TextBundle::from_section(
                        first_minutes_people_want_line(),
                        TextStyle {
                            font_size: garden_boot_want_font_px(1.0),
                            color: TITLE_TEXT_PRIMARY,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        max_width: Val::Px(TITLE_PLATE_MAX_WIDTH - 44.0),
                        ..default()
                    }),
                    TitleGardenWantText,
                ));
                p.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 13.0,
                            color: TITLE_TEXT_SECONDARY,
                            ..default()
                        },
                    ),
                    TitleCueText,
                ));
                spawn_menu_btn(p, TITLE_CHROME_PLAY, TitlePlayBtn, true);
                spawn_menu_btn(p, TITLE_CHROME_CONTINUE, TitleContinueBtn, true);
                spawn_menu_btn(p, TITLE_CHROME_SETTINGS, TitleSettingsBtn, true);
                spawn_menu_btn(
                    p,
                    persona_title_btn_label(PERSONA_CREATOR_ENABLED),
                    TitlePersonaBtn,
                    PERSONA_CREATOR_ENABLED,
                );
                spawn_menu_btn(p, ONLINE_STUB_LABEL, TitleOnlineBtn, false);
                p.spawn(TextBundle::from_section(
                    "1 Play · 2 Continue · 3 Settings · Esc from yard opens pause",
                    TextStyle {
                        font_size: 11.0,
                        color: TITLE_TEXT_SECONDARY,
                        ..default()
                    },
                ));
            });
        });
}


fn spawn_comfort_graphics_banner(mut commands: Commands) {
    // Top-of-screen strip — title / yard pause first-launch only. Online stays grey.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(520.0),
                    margin: UiRect {
                        left: Val::Px(-260.0),
                        ..default()
                    },
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(LIVED_UI_Z_PAUSE + 1),
                focus_policy: FocusPolicy::Block,
                ..default()
            },
            ComfortGraphicsBannerRoot,
            LivedUiPlate,
        ))
        .with_children(|row| {
            row.spawn(TextBundle::from_section(
                COMFORT_GRAPHICS_BANNER_COPY,
                TextStyle {
                    font_size: 13.0,
                    color: TITLE_TEXT_SECONDARY,
                    ..default()
                },
            ));
            row.spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::axes(Val::Px(10.0), Val::Px(6.0)),
                        justify_content: JustifyContent::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: TITLE_BTN_BG.into(),
                    border_color: TITLE_BORDER.into(),
                    ..default()
                },
                ComfortGraphicsBannerDismissBtn,
            ))
            .with_children(|b| {
                b.spawn(TextBundle::from_section(
                    "Dismiss",
                    TextStyle {
                        font_size: 12.0,
                        color: TITLE_BTN_FG,
                        ..default()
                    },
                ));
            });
        });
}

/// Visible on Title door or when Esc Comfort plate is showing — once until dismissed.
/// Places is a leaf of pause: while the four-room door is open, Comfort banner must
/// clear (same rule as `settings_visible_with_places`) so it does not linger over Places.
pub fn comfort_graphics_banner_should_show(
    on_title: bool,
    settings_open: bool,
    dismissed: bool,
    places_open: bool,
) -> bool {
    if dismissed {
        return false;
    }
    if on_title {
        return true;
    }
    // Esc Comfort only — hide while Places four-room plate is open (PLACES-OVERLAY).
    settings_visible_with_places(settings_open, places_open)
}

fn sync_comfort_graphics_banner(
    door: Res<LaunchDoor>,
    label: Res<HouseLabel>,
    settings: Res<LocalSettingsState>,
    places: Option<Res<PlacesPlate>>,
    mut q: Query<&mut Visibility, With<ComfortGraphicsBannerRoot>>,
) {
    let on_title = *door == LaunchDoor::Title;
    let places_open = places.map(|p| p.open).unwrap_or(false);
    let show = comfort_graphics_banner_should_show(
        on_title,
        label.settings_open,
        settings.inner.comfort_graphics_banner_dismissed,
        places_open,
    );
    for mut vis in &mut q {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn comfort_graphics_banner_dismiss_clicks(
    mut settings: ResMut<LocalSettingsState>,
    clicks: Query<&Interaction, (Changed<Interaction>, With<ComfortGraphicsBannerDismissBtn>)>,
) {
    for i in &clicks {
        if *i == Interaction::Pressed {
            if settings.inner.should_show_comfort_graphics_banner() {
                settings.inner.dismiss_comfort_graphics_banner();
                settings.mark_and_persist();
            }
        }
    }
}

fn spawn_menu_btn<C: Component>(p: &mut ChildBuilder, label: &str, marker: C, enabled: bool) {
    let bg = if enabled {
        TITLE_BTN_BG
    } else {
        TITLE_BTN_DISABLED_BG
    };
    let fg = if enabled {
        TITLE_BTN_FG
    } else {
        TITLE_BTN_DISABLED_FG
    };
    p.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: bg.into(),
            border_color: TITLE_BORDER.into(),
            ..default()
        },
        marker,
    ))
    .with_children(|b| {
        b.spawn(TextBundle::from_section(
            label,
            TextStyle {
                font_size: 15.0,
                color: fg,
                ..default()
            },
        ));
    });
}

fn spawn_settings_stub(mut commands: Commands) {
    // D1 Pause honesty + D2 local settings — Comfort · Controls · Guide tabs (no second HUD).
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    // Single active tab keeps the Settings plate readable at 720p.
                    top: Val::Percent(1.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(420.0),
                    max_height: Val::Percent(98.0),
                    margin: UiRect {
                        left: Val::Px(-210.0),
                        ..default()
                    },
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(4.0),
                    border: UiRect::all(Val::Px(1.5)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(LIVED_UI_Z_PAUSE),
                // Keep pause chrome hits on this plate (Places door + tabs + Resume).
                focus_policy: FocusPolicy::Block,
                ..default()
            },
            SettingsStubRoot,
            LivedUiPlate,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    YARD_WAITING,
                    TextStyle {
                        font_size: 16.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ),
                PauseCueText,
            ));
            // Tab strip — click or Tab / [ ] while pause is open.
            p.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(6.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|tabs| {
                for tab in PauseTab::ALL {
                    spawn_pause_tab_btn(tabs, tab);
                }
            });
            // Comfort tab (default visible).
            p.spawn((
                settings_tab_panel_bundle(true),
                PauseTabPanel(PauseTab::Comfort),
            ))
            .with_children(|comfort| {
                spawn_settings_row(
                    comfort,
                    "Graphics · Medium",
                    SettingsGraphicsBtn,
                    SettingsGraphicsLabel,
                );
                spawn_settings_row(comfort, "Look · 1.00", SettingsLookBtn, SettingsLookLabel);
                spawn_settings_row(comfort, "Mute · off", SettingsMuteBtn, SettingsMuteLabel);
                spawn_settings_row(
                    comfort,
                    "Invert-Y · off",
                    SettingsInvertBtn,
                    SettingsInvertLabel,
                );
                spawn_settings_row(
                    comfort,
                    "Hide slabs · off",
                    SettingsHideSlabsBtn,
                    SettingsHideLabel,
                );
                spawn_settings_row(
                    comfort,
                    "Brightness · 1.00",
                    SettingsBrightnessBtn,
                    SettingsBrightnessLabel,
                );
                spawn_settings_row(
                    comfort,
                    "Text scale · 1.00",
                    SettingsTextScaleBtn,
                    SettingsTextScaleLabel,
                );
                spawn_settings_row(
                    comfort,
                    "Grove · off",
                    SettingsGroveBtn,
                    SettingsGroveLabel,
                );
                spawn_settings_row(
                    comfort,
                    "Reduced motion · off",
                    SettingsReducedMotionBtn,
                    SettingsReducedMotionLabel,
                );
                spawn_settings_row(
                    comfort,
                    "Rumble · on",
                    SettingsRumbleBtn,
                    SettingsRumbleLabel,
                );
                spawn_settings_row(
                    comfort,
                    "Colorblind wells · off",
                    SettingsColorblindWellsBtn,
                    SettingsColorblindWellsLabel,
                );
                // Loopback lab remains separate from disabled Title Online.
                spawn_settings_row(comfort, "LAN · off", SettingsLanBtn, SettingsLanLabel);
                spawn_settings_row(
                    comfort,
                    "this hex admits harm · off",
                    SettingsLethalBtn,
                    SettingsLethalLabel,
                );
            });
            // Controls tab — sticks + Peace remap + Reset-to-Peace.
            p.spawn((
                settings_tab_panel_bundle(false),
                PauseTabPanel(PauseTab::Controls),
            ))
            .with_children(|controls| {
                spawn_settings_row(
                    controls,
                    "Sticks · auto",
                    SettingsSticksBtn,
                    SettingsSticksLabel,
                );
                spawn_settings_row(
                    controls,
                    "Tap-to-Use · off",
                    SettingsTapUseBtn,
                    SettingsTapUseLabel,
                );
                spawn_settings_row(
                    controls,
                    "Sprint · key",
                    SettingsSprintBtn,
                    SettingsSprintLabel,
                );
                for action in PEACE_ACTIONS {
                    spawn_settings_row(
                        controls,
                        &peace_binding_label(action, &LocalSettings::peace_defaults()),
                        SettingsPeaceBindBtn(action),
                        SettingsPeaceBindLabel(action),
                    );
                }
                spawn_settings_row(
                    controls,
                    "Reset-to-Peace",
                    SettingsPeaceResetBtn,
                    SettingsPeaceResetBtn,
                );
            });
            // Guide tab — one peak-memory sentence, wrapped in this plate.
            p.spawn((
                settings_tab_panel_bundle(false),
                PauseTabPanel(PauseTab::Guide),
            ))
            .with_children(|guide| {
                guide.spawn(
                    TextBundle::from_section(
                        PAUSE_GUIDE_LINE,
                        TextStyle {
                            font_size: 14.0,
                            color: TITLE_TEXT_SECONDARY,
                            ..default()
                        },
                    )
                    .with_text_justify(JustifyText::Center)
                    .with_style(Style {
                        width: Val::Percent(100.0),
                        ..default()
                    }),
                );
            });
            // Settled+book Places door — opens four-room plate; not a Settings row.
            // Hidden until refresh_pause_places_row (hex_travel) after Settled+book.
            spawn_pause_places_door(p);
            // Online stays grey — never binds a socket from this plate.
            spawn_menu_btn(p, ONLINE_STUB_LABEL, SettingsOnlineStubBtn, false);
            spawn_menu_btn(p, "Resume", PauseResumeBtn, true);
            spawn_menu_btn(p, "Title", PauseTitleBtn, true);
            spawn_menu_btn(p, "Quit", PauseQuitBtn, true);
        });
}

fn spawn_name_house_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: TITLE_DIM_BG.into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(140),
                ..default()
            },
            NameHouseRoot,
            LivedUiPlate,
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    padding: UiRect::all(Val::Px(18.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "Name your House",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.88, 0.98, 0.92),
                        ..default()
                    },
                ));
                p.spawn(TextBundle::from_section(
                    "Optional. Skip keeps Unnamed House.",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.60, 0.78, 0.70),
                        ..default()
                    },
                ));
                p.spawn((
                    TextBundle::from_section(
                        "_",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::srgb(0.92, 0.98, 0.94),
                            ..default()
                        },
                    ),
                    NameDraftText,
                ));
                spawn_menu_btn(p, "Confirm", NameConfirmBtn, true);
                spawn_menu_btn(p, "Skip", NameSkipBtn, true);
            });
        });
}

fn spawn_house_dress_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: TITLE_DIM_BG.into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(141),
                ..default()
            },
            HouseDressRoot,
            LivedUiPlate,
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(420.0),
                    padding: UiRect::all(Val::Px(18.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "House seals · heritage",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.88, 0.98, 0.92),
                        ..default()
                    },
                ));
                p.spawn(TextBundle::from_section(
                    "Cosmetic only. Skip keeps none. No combat kits.",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.60, 0.78, 0.70),
                        ..default()
                    },
                ));
                spawn_dress_seal_row(p, "Well", DressSealWellBtn, DressSealWellLabel);
                spawn_dress_seal_row(p, "Grove", DressSealGroveBtn, DressSealGroveLabel);
                spawn_dress_seal_row(p, "Ember", DressSealEmberBtn, DressSealEmberLabel);
                p.spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            border: UiRect::all(Val::Px(1.0)),
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: TITLE_BTN_BG.into(),
                        border_color: TITLE_BORDER.into(),
                        ..default()
                    },
                    DressHeritageBtn,
                ))
                .with_children(|b| {
                    b.spawn((
                        TextBundle::from_section(
                            "Heritage · none",
                            TextStyle {
                                font_size: 15.0,
                                color: TITLE_BTN_FG,
                                ..default()
                            },
                        ),
                        DressHeritageLabel,
                    ));
                });
                spawn_menu_btn(p, "Rename House", DressRenameBtn, true);
                spawn_menu_btn(p, "Confirm seals", DressConfirmBtn, true);
                spawn_menu_btn(p, "Skip seals", DressSkipBtn, true);
            });
        });
}

fn spawn_dress_seal_row<B: Component, L: Component>(
    p: &mut ChildBuilder,
    label: &str,
    btn: B,
    text_marker: L,
) {
    p.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                width: Val::Percent(100.0),
                ..default()
            },
            background_color: TITLE_BTN_BG.into(),
            border_color: TITLE_BORDER.into(),
            ..default()
        },
        btn,
    ))
    .with_children(|b| {
        b.spawn((
            TextBundle::from_section(
                format!("Seal · {label} · off"),
                TextStyle {
                    font_size: 15.0,
                    color: TITLE_BTN_FG,
                    ..default()
                },
            ),
            text_marker,
        ));
    });
}

fn spawn_persona_cycle_btn<B: Component, L: Component>(
    p: &mut ChildBuilder,
    initial: &str,
    btn: B,
    text_marker: L,
) {
    p.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                width: Val::Percent(100.0),
                ..default()
            },
            background_color: TITLE_BTN_BG.into(),
            border_color: TITLE_BORDER.into(),
            ..default()
        },
        btn,
    ))
    .with_children(|b| {
        b.spawn((
            TextBundle::from_section(
                initial.to_string(),
                TextStyle {
                    font_size: 14.0,
                    color: TITLE_BTN_FG,
                    ..default()
                },
            ),
            text_marker,
        ));
    });
}

fn spawn_persona_creator_panel(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: TITLE_DIM_BG.into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(142),
                focus_policy: FocusPolicy::Block,
                ..default()
            },
            PersonaCreatorRoot,
            LivedUiPlate,
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Px(440.0),
                    max_height: Val::Percent(92.0),
                    padding: UiRect::all(Val::Px(18.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "Persona creator · optional",
                    TextStyle {
                        font_size: 18.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ));
                p.spawn((
                    TextBundle::from_section(
                        PersonaCreatorStep::MechanicalModule.as_label(),
                        TextStyle {
                            font_size: 13.0,
                            color: TITLE_TEXT_SECONDARY,
                            ..default()
                        },
                    ),
                    PersonaCreatorStepLabel,
                ));
                p.spawn((
                    TextBundle::from_section(
                        persona_step_guidance(PersonaCreatorStep::MechanicalModule),
                        TextStyle {
                            font_size: 12.0,
                            color: TITLE_TEXT_SECONDARY,
                            ..default()
                        },
                    ),
                    PersonaCreatorGuidance,
                ));
                p.spawn((
                    TextBundle::from_section(
                        "Name · (empty = nameless Steward)",
                        TextStyle {
                            font_size: 14.0,
                            color: TITLE_TEXT_PRIMARY,
                            ..default()
                        },
                    ),
                    PersonaCreatorBodyText,
                ));
                spawn_persona_cycle_btn(
                    p,
                    &mechanical_module_btn_label(MechanicalRace::Human),
                    PersonaModuleBtn,
                    PersonaModuleLabel,
                );
                spawn_persona_cycle_btn(
                    p,
                    "People · unset (nameless ok)",
                    PersonaPeopleBtn,
                    PersonaPeopleLabel,
                );
                spawn_persona_cycle_btn(
                    p,
                    &phenotype_btn_label(&Phenotype::default()),
                    PersonaPhenotypeBtn,
                    PersonaPhenotypeLabel,
                );
                spawn_persona_cycle_btn(
                    p,
                    &story_share_btn_label(StoryShare::Private),
                    PersonaStoryShareBtn,
                    PersonaStoryShareLabel,
                );
                // Disabled stub — records only; never lights an LLM or Online.
                spawn_menu_btn(p, PERSONA_STORY_AI_STUB, PersonaStoryAiStubBtn, false);
                // P5 online picker — feature default off; Online stays grey.
                {
                    let label = story_provider_btn_label(
                        default_story_provider(),
                        ONLINE_PICKER_ENABLED,
                        STEWARD_ONLINE_YES,
                    );
                    let enabled = online_picker_ui_enabled(
                        ONLINE_PICKER_ENABLED,
                        STEWARD_ONLINE_YES,
                    );
                    if enabled {
                        spawn_persona_cycle_btn(
                            p,
                            &label,
                            PersonaOnlinePickerBtn,
                            PersonaOnlinePickerLabel,
                        );
                    } else {
                        // Explicitly off / available-later — no online rows lit.
                        spawn_menu_btn(p, &label, PersonaOnlinePickerBtn, false);
                    }
                }
                spawn_menu_btn(p, "Back", PersonaBackBtn, true);
                spawn_menu_btn(p, "Next", PersonaNextBtn, true);
                spawn_menu_btn(p, "Keep draft · local only", PersonaKeepDraftBtn, true);
                spawn_menu_btn(p, "Commit · validate + persist", PersonaCommitBtn, true);
                spawn_menu_btn(p, "Skip · nameless Steward", PersonaSkipNamelessBtn, true);
                spawn_menu_btn(p, "H · hide guidance", PersonaHideGuidanceBtn, true);
                p.spawn(TextBundle::from_section(
                    "Title Online stays grey · no matchmaking power · presentation only",
                    TextStyle {
                        font_size: 11.0,
                        color: TITLE_TEXT_SECONDARY,
                        ..default()
                    },
                ));
            });
        });
}

fn breath_title_border(
    time: Res<Time>,
    door: Res<LaunchDoor>,
    mut q: Query<&mut BorderColor, With<TitleBreath>>,
) {
    if *door != LaunchDoor::Title {
        return;
    }
    let pulse = 0.45 + (time.elapsed_seconds() * 1.2).sin() * 0.12;
    for mut border in &mut q {
        // TitleBreath is on the full-screen root (no border) — keep noop-safe.
        let _ = pulse; // opaque plate — no alpha breath on soft GPU
        *border = TITLE_BORDER.into();
    }
}

fn refresh_title_cue(label: Res<HouseLabel>, mut q: Query<&mut Text, With<TitleCueText>>) {
    let house_cue = continue_cue_when_persist(label.persist_present, &label.house)
        .unwrap_or_else(|| "Play opens the yard · no account wall".into());
    // CARD FLESH-CONTINUE-LINE — same beat, Guide peak (`yard remembered`).
    let house_cue = continue_yard_remembered_line(&house_cue);
    // CARD S3 — sealed souls shown as People dress · last Place. Not a portraits grid.
    let cue = match s3_continue_roster_cue(read_hour_two_json().as_deref()) {
        Some(roster) => format!("{house_cue} · {roster}"),
        None => house_cue,
    };
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != cue {
                s.value = cue.clone();
            }
        }
    }
}

/// Comfort Low bumps `text_scale` to 1.10 — Garden Want stays words, not a mesh.
pub fn garden_boot_want_font_px(text_scale: f32) -> f32 {
    (14.0 * text_scale.clamp(1.0, 1.6)).clamp(13.0, 22.0)
}

/// CARD L1 GARDEN-WANT / CARD S1 / CARD F7 — Title / God-plane speaks
/// People + Want until H hushes. After a sealed People-door land, the
/// same plate speaks Place-local aftermath plus one F7 variant (not a
/// trailer). Skip House keeps GARDEN_WANT. Does not require
/// LaunchDoor::InYard or Sanctuary dirt.
fn refresh_garden_boot_want(
    door: Res<LaunchDoor>,
    guidance: Option<Res<FirstSessionGuidance>>,
    settings: Res<LocalSettingsState>,
    mut q: Query<(&mut Text, &mut Visibility), With<TitleGardenWantText>>,
) {
    let hush = guidance
        .as_ref()
        .map(|g| !g.speaks_people_want())
        .unwrap_or(false);
    let landing = guidance.as_ref().and_then(|g| g.people_landing);
    let spoken = if *door == LaunchDoor::Title {
        f7_title_garden_want_plate(hush, landing)
    } else {
        None
    };
    let font = garden_boot_want_font_px(settings.inner.text_scale);
    for (mut text, mut vis) in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            let value = spoken.clone().unwrap_or_default();
            if s.value != value {
                s.value = value;
            }
            if (s.style.font_size - font).abs() > 0.01 {
                s.style.font_size = font;
            }
        }
        *vis = if spoken.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn enter_yard(door: &mut LaunchDoor, label: &mut HouseLabel) {
    *door = LaunchDoor::InYard;
    label.settings_open = false;
}

fn title_button_clicks(
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    mut persona: ResMut<PersonaCreatorState>,
    mut travel: Option<ResMut<HexTravelState>>,
    mut bind: Option<ResMut<LivedHourBind>>,
    hour: Option<Res<HourSacred>>,
    mut embassy: Option<ResMut<EmbassyYard>>,
    play: Query<&Interaction, (Changed<Interaction>, With<TitlePlayBtn>)>,
    cont: Query<&Interaction, (Changed<Interaction>, With<TitleContinueBtn>)>,
    settings: Query<&Interaction, (Changed<Interaction>, With<TitleSettingsBtn>)>,
    persona_btn: Query<&Interaction, (Changed<Interaction>, With<TitlePersonaBtn>)>,
) {
    if *door != LaunchDoor::Title {
        return;
    }
    if persona.open {
        return;
    }
    for i in &play {
        if *i == Interaction::Pressed {
            if let (Some(travel), Some(bind), Some(hour)) =
                (travel.as_mut(), bind.as_mut(), hour.as_ref())
            {
                let _ = s3_play_boot(travel, bind, hour, embassy.as_deref_mut());
            }
            enter_yard(&mut door, &mut label);
            return;
        }
    }
    for i in &cont {
        if *i == Interaction::Pressed {
            if label.persist_present {
                if let (Some(travel), Some(bind), Some(hour)) =
                    (travel.as_mut(), bind.as_mut(), hour.as_ref())
                {
                    let raw = read_hour_two_json();
                    let _ = s3_continue_boot(
                        raw.as_deref(),
                        travel,
                        bind,
                        hour,
                        embassy.as_deref_mut(),
                    );
                }
                enter_yard(&mut door, &mut label);
            }
            return;
        }
    }
    for i in &settings {
        if *i == Interaction::Pressed {
            label.settings_open = !label.settings_open;
            return;
        }
    }
    for i in &persona_btn {
        if *i == Interaction::Pressed {
            if try_open_persona_creator(&mut persona, PERSONA_CREATOR_ENABLED) {
                label.settings_open = false;
            }
            return;
        }
    }
}

fn title_keyboard_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    rebind: Res<PeaceRebindState>,
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    persona: Res<PersonaCreatorState>,
    mut travel: Option<ResMut<HexTravelState>>,
    mut bind: Option<ResMut<LivedHourBind>>,
    hour: Option<Res<HourSacred>>,
    mut embassy: Option<ResMut<EmbassyYard>>,
    mut places: Option<ResMut<PlacesPlate>>,
) {
    if rebind.waiting.is_some() || rebind.suppress_shortcuts {
        return;
    }
    if persona.open {
        return;
    }
    match *door {
        LaunchDoor::Title => {
            if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Enter) {
                if let (Some(travel), Some(bind), Some(hour)) =
                    (travel.as_mut(), bind.as_mut(), hour.as_ref())
                {
                    let _ = s3_play_boot(travel, bind, hour, embassy.as_deref_mut());
                }
                enter_yard(&mut door, &mut label);
            } else if keyboard.just_pressed(KeyCode::Digit2) {
                if label.persist_present {
                    if let (Some(travel), Some(bind), Some(hour)) =
                        (travel.as_mut(), bind.as_mut(), hour.as_ref())
                    {
                        let raw = read_hour_two_json();
                        let _ = s3_continue_boot(
                            raw.as_deref(),
                            travel,
                            bind,
                            hour,
                            embassy.as_deref_mut(),
                        );
                    }
                    enter_yard(&mut door, &mut label);
                }
            } else if keyboard.just_pressed(KeyCode::Digit3) {
                label.settings_open = !label.settings_open;
            } else if keyboard.just_pressed(KeyCode::Escape) {
                // Esc-from-title: close settings / persona plate. Never wipe house/climate/standing/book.
                label.settings_open = false;
                // Persona Esc handled in persona_creator_buttons when open.
            }
        }
        LaunchDoor::InYard => {
            // Esc opens/closes D1 pause (esc_yard_pause) — never Title / never quit.
            // Digit3 walks the same verb, so both keys land the same plate.
            if keyboard.just_pressed(KeyCode::Digit3) {
                apply_yard_pause_press(LaunchDoor::InYard, &mut label, places.as_deref_mut());
            }
        }
        LaunchDoor::NameHouse | LaunchDoor::HouseDress => {}
    }
}

fn sync_title_visibility(
    door: Res<LaunchDoor>,
    label: Res<HouseLabel>,
    mut root: Query<&mut Visibility, With<TitleRoot>>,
    mut cont_style: Query<(&mut BackgroundColor, &mut BorderColor), With<TitleContinueBtn>>,
    net: Res<SessionNetMode>,
) {
    let show = *door == LaunchDoor::Title;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    // Continue dims when no persist
    let enabled = label.persist_present;
    for (mut bg, mut border) in &mut cont_style {
        *bg = if enabled {
            TITLE_BTN_BG.into()
        } else {
            TITLE_BTN_DISABLED_BG.into()
        };
        *border = TITLE_BORDER.into();
    }
    let _ = net.mode; // Online stays Offline; peer count never shown
}

fn sync_settings_stub(
    label: Res<HouseLabel>,
    door: Res<LaunchDoor>,
    persona: Res<PersonaCreatorState>,
    places: Option<Res<PlacesPlate>>,
    mut q: Query<(&mut Visibility, &mut Style), With<SettingsStubRoot>>,
) {
    let places_open = places.map(|p| p.open).unwrap_or(false);
    let show = settings_visible_with_places(label.settings_open, places_open)
        && *door != LaunchDoor::NameHouse
        && *door != LaunchDoor::HouseDress
        && !persona.open;
    // Display::None (not only Hidden): soft-GPU / same-z Comfort must not linger
    // over the four-room Places plate after Places row Pressed (PLACES-CLICK).
    for (mut vis, mut style) in &mut q {
        style.display = if show {
            Display::Flex
        } else {
            Display::None
        };
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Test/helper: Comfort pause chrome is laid out (not Display::None).
pub(crate) fn settings_stub_is_showing(world: &mut bevy::prelude::World) -> bool {
    let mut q = world.query_filtered::<(&Style, &Visibility), With<SettingsStubRoot>>();
    q.iter(world)
        .any(|(style, vis)| style.display == Display::Flex && *vis == Visibility::Visible)
}

fn watch_settled_for_naming(
    hour: Option<Res<HourSacred>>,
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
) {
    if *door != LaunchDoor::InYard {
        return;
    }
    let Some(hour) = hour else {
        return;
    };
    if !hour.complete {
        return;
    }
    // Name first (skippable) — then D3 seals/heritage after Settled.
    if !label.house.resolved && !label.naming_offered {
        label.naming_offered = true;
        // Soft-write Unnamed so Continue works even if they quit mid-name panel.
        ensure_house_file_written(&mut label);
        label.draft.clear();
        *door = LaunchDoor::NameHouse;
        return;
    }
    if label.house.resolved && !label.house.seals_resolved && !label.seals_offered {
        label.seals_offered = true;
        ensure_house_file_written(&mut label);
        *door = LaunchDoor::HouseDress;
    }
}

/// Esc while InYard toggles D1 pause plate (stranger-pass / E1 + Wave H).
/// Pause closed → open (*the yard is waiting*); pause open → close (Resume).
/// Same plate on every local hex — the door gates this, the hex never does.
/// Does **not** set Title and does **not** quit-to-desktop — Title button /
/// `return_yard_to_title` still writes house JSON + lived persist; Quit = AppExit.
fn esc_yard_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_input: Res<PlayerInput>,
    rebind: Res<PeaceRebindState>,
    door: Res<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    mut places: Option<ResMut<PlacesPlate>>,
) {
    if rebind.waiting.is_some() || rebind.suppress_shortcuts {
        return;
    }
    let esc = keyboard.just_pressed(KeyCode::Escape);
    // Start/Options = same pause toggle as Esc (INPUT_CANON).
    let start = player_input.pause_toggle;
    if !esc && !start {
        return;
    }
    // Never LaunchDoor::Title / never AppExit from Esc/Start.
    apply_yard_pause_press(*door, &mut label, places.as_deref_mut());
}

/// Run one yard pause press (Esc / Start / key 3 / touch chip) through the
/// single verb, so every entry point agrees on every local hex.
pub(crate) fn apply_yard_pause_press(
    door: LaunchDoor,
    label: &mut HouseLabel,
    places: Option<&mut PlacesPlate>,
) -> bool {
    let places_was_open = places.as_ref().map(|p| p.open).unwrap_or(false);
    let Some(step) = yard_pause_step(door, label.settings_open, places_was_open) else {
        return false;
    };
    apply_yard_pause(step, label, places);
    true
}

fn apply_yard_pause(step: YardPause, label: &mut HouseLabel, places: Option<&mut PlacesPlate>) {
    label.settings_open = step.pause_open;
    if let Some(places) = places {
        if places.open != step.places_open {
            if step.places_open {
                places.open_door();
            } else {
                places.close_door();
            }
        }
    }
}

/// After Settled or quit-to-title: house JSON exists (name may be null / Unnamed).
fn ensure_house_file_written(label: &mut HouseLabel) {
    if !label.house.resolved {
        label.house.skip();
        label.naming_offered = true;
    }
    label.house.persist();
    label.persist_present = true;
}

fn refresh_pause_cue(
    door: Res<LaunchDoor>,
    label: Res<HouseLabel>,
    mut q: Query<&mut Text, With<PauseCueText>>,
) {
    if !label.settings_open {
        return;
    }
    let line = pause_plate_line(*door).unwrap_or("Local settings · Esc closes");
    for mut text in &mut q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.to_string();
            }
        }
    }
}

fn pause_plate_clicks(
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    bind: Option<Res<LivedHourBind>>,
    mut places: Option<ResMut<PlacesPlate>>,
    mut exit: EventWriter<AppExit>,
    resume: Query<&Interaction, (Changed<Interaction>, With<PauseResumeBtn>)>,
    title: Query<&Interaction, (Changed<Interaction>, With<PauseTitleBtn>)>,
    quit: Query<&Interaction, (Changed<Interaction>, With<PauseQuitBtn>)>,
) {
    if !label.settings_open {
        return;
    }
    for i in &resume {
        if *i == Interaction::Pressed {
            label.settings_open = false;
            if let Some(places) = places.as_mut() {
                places.close_door();
            }
            return;
        }
    }
    for i in &title {
        if *i == Interaction::Pressed {
            if let Some(places) = places.as_mut() {
                places.close_door();
            }
            return_yard_to_title(&mut door, &mut label, bind.as_ref());
            return;
        }
    }
    for i in &quit {
        if *i == Interaction::Pressed {
            // Same window-close / Settings quit path — not Esc.
            label.settings_open = false;
            if let Some(places) = places.as_mut() {
                places.close_door();
            }
            exit.send(AppExit::Success);
            return;
        }
    }
}

/// Shared Pause→Title path (Title button): house JSON + lived persist, then Title.
fn return_yard_to_title(
    door: &mut LaunchDoor,
    label: &mut HouseLabel,
    bind: Option<&Res<LivedHourBind>>,
) {
    label.settings_open = false;
    if *door != LaunchDoor::InYard {
        return;
    }
    ensure_house_file_written(label);
    if let Some(bind) = bind {
        bind.persist();
    }
    *door = LaunchDoor::Title;
}

fn settings_tab_panel_bundle(visible: bool) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            align_items: AlignItems::Stretch,
            // Inactive tabs must leave the flex flow (Hidden still occupies space).
            display: if visible {
                Display::Flex
            } else {
                Display::None
            },
            ..default()
        },
        visibility: if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        },
        ..default()
    }
}

fn spawn_pause_places_door(p: &mut ChildBuilder) {
    p.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                // Settled+book only — refresh_pause_places_row flips to Flex.
                display: Display::None,
                ..default()
            },
            background_color: TITLE_BTN_BG.into(),
            border_color: TITLE_BORDER.into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        PausePlacesBtn,
        Name::new("PausePlacesRow"),
    ))
    .with_children(|b| {
        b.spawn(TextBundle::from_section(
            PLACES_ROW,
            TextStyle {
                font_size: 15.0,
                color: TITLE_BTN_FG,
                ..default()
            },
        ));
    });
}

fn spawn_pause_tab_btn(p: &mut ChildBuilder, tab: PauseTab) {
    p.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                flex_grow: 1.0,
                ..default()
            },
            background_color: TITLE_BTN_BG.into(),
            border_color: TITLE_BORDER.into(),
            ..default()
        },
        PauseTabBtn(tab),
    ))
    .with_children(|b| {
        b.spawn(TextBundle::from_section(
            tab.label(),
            TextStyle {
                font_size: 14.0,
                color: TITLE_BTN_FG,
                ..default()
            },
        ));
    });
}

fn pause_tab_clicks(
    label: Res<HouseLabel>,
    mut tab: ResMut<PauseTab>,
    q: Query<(&Interaction, &PauseTabBtn), Changed<Interaction>>,
) {
    if !label.settings_open {
        return;
    }
    for (interaction, btn) in &q {
        if *interaction == Interaction::Pressed {
            *tab = btn.0;
        }
    }
}

fn pause_tab_keys(
    keyboard: Res<ButtonInput<KeyCode>>,
    label: Res<HouseLabel>,
    rebind: Res<PeaceRebindState>,
    mut tab: ResMut<PauseTab>,
) {
    if !label.settings_open || rebind.waiting.is_some() || rebind.suppress_shortcuts {
        return;
    }
    // Digits stay Title Play/Continue/Settings and InYard Digit3 pause — use Tab / [].
    let next = (keyboard.just_pressed(KeyCode::Tab)
        && !keyboard.pressed(KeyCode::ShiftLeft)
        && !keyboard.pressed(KeyCode::ShiftRight))
        || keyboard.just_pressed(KeyCode::BracketRight);
    let prev = (keyboard.just_pressed(KeyCode::Tab)
        && (keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight)))
        || keyboard.just_pressed(KeyCode::BracketLeft);
    if next {
        *tab = match *tab {
            PauseTab::Comfort => PauseTab::Controls,
            PauseTab::Controls => PauseTab::Guide,
            PauseTab::Guide => PauseTab::Comfort,
        };
    } else if prev {
        *tab = match *tab {
            PauseTab::Comfort => PauseTab::Guide,
            PauseTab::Controls => PauseTab::Comfort,
            PauseTab::Guide => PauseTab::Controls,
        };
    }
}

fn sync_pause_tabs(
    tab: Res<PauseTab>,
    mut panels: Query<(&PauseTabPanel, &mut Visibility, &mut Style)>,
    mut btns: Query<(&PauseTabBtn, &mut BackgroundColor, &mut BorderColor)>,
) {
    for (panel, mut vis, mut style) in &mut panels {
        let active = panel.0 == *tab;
        *vis = if active {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        style.display = if active {
            Display::Flex
        } else {
            Display::None
        };
    }
    for (btn, mut bg, mut border) in &mut btns {
        let active = btn.0 == *tab;
        *bg = if active {
            Color::srgb(0.16, 0.26, 0.20).into()
        } else {
            TITLE_BTN_BG.into()
        };
        *border = if active {
            TITLE_BORDER.into()
        } else {
            Color::srgb(0.28, 0.40, 0.32).into()
        };
    }
}

fn spawn_settings_row<B: Component, L: Component>(
    p: &mut ChildBuilder,
    label: &str,
    btn: B,
    text_marker: L,
) {
    p.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(1.0)),
                width: Val::Percent(100.0),
                ..default()
            },
            background_color: TITLE_BTN_BG.into(),
            border_color: TITLE_BORDER.into(),
            ..default()
        },
        btn,
    ))
    .with_children(|b| {
        b.spawn((
            TextBundle::from_section(
                label,
                TextStyle {
                    font_size: 15.0,
                    color: TITLE_BTN_FG,
                    ..default()
                },
            ),
            text_marker,
        ));
    });
}

fn on_off(v: bool) -> &'static str {
    if v {
        "on"
    } else {
        "off"
    }
}

pub fn look_btn_label(s: &LocalSettings) -> String {
    format!("Look · {:.2}", s.look_sensitivity)
}

pub fn mute_btn_label(s: &LocalSettings) -> String {
    format!("Mute · {}", on_off(s.mute))
}

pub fn invert_btn_label(s: &LocalSettings) -> String {
    format!("Invert-Y · {}", on_off(s.invert_y))
}

pub fn hide_slabs_btn_label(s: &LocalSettings) -> String {
    format!("Hide slabs · {}", on_off(s.hide_slabs))
}

pub fn brightness_btn_label(s: &LocalSettings) -> String {
    format!("Brightness · {:.2}", s.brightness)
}

pub fn text_scale_btn_label(s: &LocalSettings) -> String {
    format!("Text scale · {:.2}", s.text_scale)
}

pub fn graphics_preset_btn_label(s: &LocalSettings) -> String {
    format!("Graphics · {}", s.graphics_preset.label())
}

pub fn grove_btn_label(s: &LocalSettings) -> String {
    let g = if s.grove_is_light() { "light" } else { "off" };
    format!("Grove · {g}")
}

pub fn reduced_motion_btn_label(s: &LocalSettings) -> String {
    format!("Reduced motion · {}", on_off(s.reduced_motion))
}

pub fn rumble_btn_label(s: &LocalSettings) -> String {
    format!("Rumble · {}", on_off(s.rumble))
}

pub fn colorblind_wells_btn_label(s: &LocalSettings) -> String {
    format!("Colorblind wells · {}", s.colorblind_wells_label())
}

pub fn lan_btn_label(s: &LocalSettings) -> String {
    format!("LAN · {}", s.lan_label())
}

/// L1 Settings confirm row. Standing hex sign — not a LocalSettings persist field.
pub fn lethal_sign_btn_label(
    settled: bool,
    book_held: bool,
    charter_live: bool,
    declared: bool,
) -> &'static str {
    lethal_sign_row(settled, book_held, charter_live, declared)
}

pub fn sticks_btn_label(s: &LocalSettings) -> String {
    format!("Sticks · {}", s.on_screen_sticks)
}

pub fn tap_use_btn_label(s: &LocalSettings) -> String {
    format!("Tap-to-Use · {}", on_off(s.tap_to_use))
}

pub fn sprint_btn_label(s: &LocalSettings) -> String {
    format!("Sprint · {}", s.sprint_mode)
}

impl PeaceAction {
    const fn label(self) -> &'static str {
        match self {
            Self::MoveUp => "Move up",
            Self::MoveDown => "Move down",
            Self::MoveLeft => "Move left",
            Self::MoveRight => "Move right",
            Self::Jump => "Jump",
            Self::Sprint => "Sprint",
            Self::Use => "Use",
            Self::Satchel => "Satchel",
            Self::Hide => "Hide",
            Self::Allocate => "Allocate",
        }
    }

    fn key(self, settings: &LocalSettings) -> PeaceKey {
        match self {
            Self::MoveUp => settings.key_move_up,
            Self::MoveDown => settings.key_move_down,
            Self::MoveLeft => settings.key_move_left,
            Self::MoveRight => settings.key_move_right,
            Self::Jump => settings.key_jump,
            Self::Sprint => settings.key_sprint,
            Self::Use => settings.key_use,
            Self::Satchel => settings.key_satchel,
            Self::Hide => settings.key_hide,
            Self::Allocate => settings.key_allocate,
        }
    }

    fn set_key(self, settings: &mut LocalSettings, key: PeaceKey) {
        match self {
            Self::MoveUp => settings.key_move_up = key,
            Self::MoveDown => settings.key_move_down = key,
            Self::MoveLeft => settings.key_move_left = key,
            Self::MoveRight => settings.key_move_right = key,
            Self::Jump => settings.key_jump = key,
            Self::Sprint => settings.key_sprint = key,
            Self::Use => settings.key_use = key,
            Self::Satchel => settings.key_satchel = key,
            Self::Hide => settings.key_hide = key,
            Self::Allocate => settings.key_allocate = key,
        }
    }
}

fn peace_binding_label(action: PeaceAction, settings: &LocalSettings) -> String {
    let key = action.key(settings);
    let key_label = if action == PeaceAction::Sprint && key == PeaceKey::LeftShift {
        "either Shift"
    } else {
        key.display_label()
    };
    format!("{} · {key_label}", action.label())
}

fn bound_peace_action(
    settings: &LocalSettings,
    key: PeaceKey,
    except: PeaceAction,
) -> Option<PeaceAction> {
    PEACE_ACTIONS.into_iter().find(|action| {
        if *action == except {
            return false;
        }
        let bound = action.key(settings);
        bound == key
            || (*action == PeaceAction::Sprint
                && bound == PeaceKey::LeftShift
                && key == PeaceKey::RightShift)
    })
}

fn try_peace_rebind(
    settings: &mut LocalSettings,
    action: PeaceAction,
    key: PeaceKey,
) -> Result<(), PeaceAction> {
    if let Some(bound) = bound_peace_action(settings, key, action) {
        return Err(bound);
    }
    action.set_key(settings, key);
    Ok(())
}

fn reset_peace_bindings(settings: &mut LocalSettings) {
    let defaults = LocalSettings::peace_defaults();
    for action in PEACE_ACTIONS {
        action.set_key(settings, action.key(&defaults));
    }
}

fn peace_key_from_key_code(key: KeyCode) -> Option<PeaceKey> {
    Some(match key {
        KeyCode::KeyA => PeaceKey::A,
        KeyCode::KeyB => PeaceKey::B,
        KeyCode::KeyC => PeaceKey::C,
        KeyCode::KeyD => PeaceKey::D,
        KeyCode::KeyE => PeaceKey::E,
        KeyCode::KeyF => PeaceKey::F,
        KeyCode::KeyG => PeaceKey::G,
        KeyCode::KeyH => PeaceKey::H,
        KeyCode::KeyI => PeaceKey::I,
        KeyCode::KeyJ => PeaceKey::J,
        KeyCode::KeyK => PeaceKey::K,
        KeyCode::KeyL => PeaceKey::L,
        KeyCode::KeyM => PeaceKey::M,
        KeyCode::KeyN => PeaceKey::N,
        KeyCode::KeyO => PeaceKey::O,
        KeyCode::KeyP => PeaceKey::P,
        KeyCode::KeyQ => PeaceKey::Q,
        KeyCode::KeyR => PeaceKey::R,
        KeyCode::KeyS => PeaceKey::S,
        KeyCode::KeyT => PeaceKey::T,
        KeyCode::KeyU => PeaceKey::U,
        KeyCode::KeyV => PeaceKey::V,
        KeyCode::KeyW => PeaceKey::W,
        KeyCode::KeyX => PeaceKey::X,
        KeyCode::KeyY => PeaceKey::Y,
        KeyCode::KeyZ => PeaceKey::Z,
        KeyCode::Digit0 => PeaceKey::Digit0,
        KeyCode::Digit1 => PeaceKey::Digit1,
        KeyCode::Digit2 => PeaceKey::Digit2,
        KeyCode::Digit3 => PeaceKey::Digit3,
        KeyCode::Digit4 => PeaceKey::Digit4,
        KeyCode::Digit5 => PeaceKey::Digit5,
        KeyCode::Digit6 => PeaceKey::Digit6,
        KeyCode::Digit7 => PeaceKey::Digit7,
        KeyCode::Digit8 => PeaceKey::Digit8,
        KeyCode::Digit9 => PeaceKey::Digit9,
        KeyCode::ArrowUp => PeaceKey::ArrowUp,
        KeyCode::ArrowDown => PeaceKey::ArrowDown,
        KeyCode::ArrowLeft => PeaceKey::ArrowLeft,
        KeyCode::ArrowRight => PeaceKey::ArrowRight,
        KeyCode::Space => PeaceKey::Space,
        KeyCode::ShiftLeft => PeaceKey::LeftShift,
        KeyCode::ShiftRight => PeaceKey::RightShift,
        KeyCode::ControlLeft => PeaceKey::LeftControl,
        KeyCode::ControlRight => PeaceKey::RightControl,
        KeyCode::AltLeft => PeaceKey::LeftAlt,
        KeyCode::AltRight => PeaceKey::RightAlt,
        KeyCode::Tab => PeaceKey::Tab,
        KeyCode::Enter => PeaceKey::Enter,
        KeyCode::Backspace => PeaceKey::Backspace,
        _ => return None,
    })
}

fn peace_rebind_clicks(
    label: Res<HouseLabel>,
    mut state: ResMut<PeaceRebindState>,
    mut settings: ResMut<LocalSettingsState>,
    binds: Query<(&Interaction, &SettingsPeaceBindBtn), Changed<Interaction>>,
    reset: Query<&Interaction, (Changed<Interaction>, With<SettingsPeaceResetBtn>)>,
) {
    if !label.settings_open {
        state.waiting = None;
        state.notice = None;
        return;
    }
    for (interaction, bind) in &binds {
        if *interaction == Interaction::Pressed {
            state.waiting = Some(bind.0);
            state.notice = None;
            return;
        }
    }
    for interaction in &reset {
        if *interaction == Interaction::Pressed {
            reset_peace_bindings(&mut settings.inner);
            settings.mark_and_persist();
            state.waiting = None;
            state.notice = None;
            return;
        }
    }
}

fn capture_peace_rebind(
    label: Res<HouseLabel>,
    mut state: ResMut<PeaceRebindState>,
    mut settings: ResMut<LocalSettingsState>,
    mut keys: EventReader<KeyboardInput>,
) {
    state.suppress_shortcuts = false;
    if !label.settings_open || state.waiting.is_none() {
        for _ in keys.read() {}
        return;
    }
    for event in keys.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }
        state.suppress_shortcuts = true;
        if event.key_code == KeyCode::Escape {
            state.waiting = None;
            state.notice = None;
            return;
        }
        let Some(key) = peace_key_from_key_code(event.key_code) else {
            state.notice = Some("unsupported key".into());
            return;
        };
        let action = state.waiting.expect("checked above");
        match try_peace_rebind(&mut settings.inner, action, key) {
            Ok(()) => {
                settings.mark_and_persist();
                state.waiting = None;
                state.notice = None;
            }
            Err(bound) => {
                state.notice = Some(format!("taken by {}", bound.label()));
            }
        }
        return;
    }
}

fn refresh_peace_rebind_labels(
    label: Res<HouseLabel>,
    settings: Res<LocalSettingsState>,
    state: Res<PeaceRebindState>,
    mut texts: Query<(&SettingsPeaceBindLabel, &mut Text)>,
) {
    if !label.settings_open {
        return;
    }
    let font = (15.0 * settings.inner.text_scale).clamp(11.0, 22.0);
    for (marker, mut text) in &mut texts {
        let value = if state.waiting == Some(marker.0) {
            format!(
                "{} · {}",
                marker.0.label(),
                state.notice.as_deref().unwrap_or("press a key")
            )
        } else {
            peace_binding_label(marker.0, &settings.inner)
        };
        set_btn_section_text(&mut text, &value);
        set_btn_section_font(&mut text, font);
    }
}

fn set_btn_section_text(text: &mut Text, value: &str) {
    if let Some(s) = text.sections.get_mut(0) {
        if s.value != value {
            s.value = value.to_string();
        }
    }
}

fn set_btn_section_font(text: &mut Text, size: f32) {
    if let Some(s) = text.sections.get_mut(0) {
        if (s.style.font_size - size).abs() > 0.01 {
            s.style.font_size = size;
        }
    }
}

fn refresh_local_settings_labels(
    label: Res<HouseLabel>,
    settings: Res<LocalSettingsState>,
    mut texts: ParamSet<(
        Query<&mut Text, With<SettingsLookLabel>>,
        Query<&mut Text, With<SettingsMuteLabel>>,
        Query<&mut Text, With<SettingsInvertLabel>>,
        Query<&mut Text, With<SettingsHideLabel>>,
        Query<&mut Text, With<SettingsBrightnessLabel>>,
        Query<&mut Text, With<SettingsTextScaleLabel>>,
        Query<&mut Text, With<SettingsGroveLabel>>,
        Query<&mut Text, With<SettingsLanLabel>>,
    )>,
) {
    if !label.settings_open {
        return;
    }
    let s = &settings.inner;
    let look = look_btn_label(s);
    let mute = mute_btn_label(s);
    let invert = invert_btn_label(s);
    let hide = hide_slabs_btn_label(s);
    let bright = brightness_btn_label(s);
    let scale = text_scale_btn_label(s);
    let grove = grove_btn_label(s);
    let lan = lan_btn_label(s);
    let font = (15.0 * s.text_scale).clamp(11.0, 22.0);
    for mut text in &mut texts.p0() {
        set_btn_section_text(&mut text, &look);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut texts.p1() {
        set_btn_section_text(&mut text, &mute);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut texts.p2() {
        set_btn_section_text(&mut text, &invert);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut texts.p3() {
        set_btn_section_text(&mut text, &hide);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut texts.p4() {
        set_btn_section_text(&mut text, &bright);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut texts.p5() {
        set_btn_section_text(&mut text, &scale);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut texts.p6() {
        set_btn_section_text(&mut text, &grove);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut texts.p7() {
        set_btn_section_text(&mut text, &lan);
        set_btn_section_font(&mut text, font);
    }
}

fn refresh_accessibility_settings_labels(
    label: Res<HouseLabel>,
    settings: Res<LocalSettingsState>,
    mut graphics: Query<&mut Text, With<SettingsGraphicsLabel>>,
    mut reduced_motion: Query<
        &mut Text,
        (
            With<SettingsReducedMotionLabel>,
            Without<SettingsGraphicsLabel>,
        ),
    >,
    mut rumble: Query<
        &mut Text,
        (
            With<SettingsRumbleLabel>,
            Without<SettingsReducedMotionLabel>,
            Without<SettingsGraphicsLabel>,
        ),
    >,
    mut colorblind: Query<
        &mut Text,
        (
            With<SettingsColorblindWellsLabel>,
            Without<SettingsReducedMotionLabel>,
            Without<SettingsRumbleLabel>,
            Without<SettingsGraphicsLabel>,
        ),
    >,
) {
    if !label.settings_open {
        return;
    }
    let s = &settings.inner;
    let graphics_label = graphics_preset_btn_label(s);
    let reduced_motion_label = reduced_motion_btn_label(s);
    let rumble_label = rumble_btn_label(s);
    let colorblind_label = colorblind_wells_btn_label(s);
    let font = (15.0 * s.text_scale).clamp(11.0, 22.0);
    for mut text in &mut graphics {
        set_btn_section_text(&mut text, &graphics_label);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut reduced_motion {
        set_btn_section_text(&mut text, &reduced_motion_label);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut rumble {
        set_btn_section_text(&mut text, &rumble_label);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut colorblind {
        set_btn_section_text(&mut text, &colorblind_label);
        set_btn_section_font(&mut text, font);
    }
}

fn local_settings_clicks(
    label: Res<HouseLabel>,
    mut settings: ResMut<LocalSettingsState>,
    look: Query<&Interaction, (Changed<Interaction>, With<SettingsLookBtn>)>,
    mute: Query<&Interaction, (Changed<Interaction>, With<SettingsMuteBtn>)>,
    invert: Query<&Interaction, (Changed<Interaction>, With<SettingsInvertBtn>)>,
    hide: Query<&Interaction, (Changed<Interaction>, With<SettingsHideSlabsBtn>)>,
    bright: Query<&Interaction, (Changed<Interaction>, With<SettingsBrightnessBtn>)>,
    scale: Query<&Interaction, (Changed<Interaction>, With<SettingsTextScaleBtn>)>,
    grove: Query<&Interaction, (Changed<Interaction>, With<SettingsGroveBtn>)>,
    lan: Query<&Interaction, (Changed<Interaction>, With<SettingsLanBtn>)>,
    sticks: Query<&Interaction, (Changed<Interaction>, With<SettingsSticksBtn>)>,
    tap_use: Query<&Interaction, (Changed<Interaction>, With<SettingsTapUseBtn>)>,
    sprint: Query<&Interaction, (Changed<Interaction>, With<SettingsSprintBtn>)>,
    online: Query<&Interaction, (Changed<Interaction>, With<SettingsOnlineStubBtn>)>,
) {
    if !label.settings_open {
        return;
    }
    let mut changed = false;
    for i in &look {
        if *i == Interaction::Pressed {
            settings.inner.bump_look();
            changed = true;
        }
    }
    for i in &mute {
        if *i == Interaction::Pressed {
            // Same MasterMute / mute flag as D2 — pause plate Mute is not a second system.
            settings.inner.toggle_mute();
            changed = true;
        }
    }
    for i in &invert {
        if *i == Interaction::Pressed {
            settings.inner.toggle_invert_y();
            changed = true;
        }
    }
    for i in &hide {
        if *i == Interaction::Pressed {
            settings.inner.toggle_hide_slabs();
            changed = true;
        }
    }
    for i in &bright {
        if *i == Interaction::Pressed {
            settings.inner.bump_brightness();
            changed = true;
        }
    }
    for i in &scale {
        if *i == Interaction::Pressed {
            settings.inner.bump_text_scale();
            changed = true;
        }
    }
    for i in &grove {
        if *i == Interaction::Pressed {
            // G0.5 — same light-gen path as POWRUSH_GEN=light (OR at door).
            settings.inner.cycle_grove();
            changed = true;
        }
    }
    for i in &lan {
        if *i == Interaction::Pressed {
            // P3 — off ↔ loopback. Does not touch Title Online or POWRUSH_NET=on.
            settings.inner.cycle_lan();
            changed = true;
        }
    }
    for i in &sticks {
        if *i == Interaction::Pressed {
            settings.inner.cycle_on_screen_sticks();
            changed = true;
        }
    }
    for i in &tap_use {
        if *i == Interaction::Pressed {
            settings.inner.toggle_tap_to_use();
            changed = true;
        }
    }
    for i in &sprint {
        if *i == Interaction::Pressed {
            settings.inner.cycle_sprint_mode();
            changed = true;
        }
    }
    for i in &online {
        if *i == Interaction::Pressed {
            // Hard refuse — Online never binds a socket from Settings.
            // LAN is a separate row. Title Online stays grey.
            let _refused = refuse_online_socket_toggle(true);
            debug_assert!(_refused);
            debug_assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        }
    }
    if changed {
        settings.mark_and_persist();
    }
}

fn accessibility_settings_clicks(
    label: Res<HouseLabel>,
    mut settings: ResMut<LocalSettingsState>,
    graphics: Query<&Interaction, (Changed<Interaction>, With<SettingsGraphicsBtn>)>,
    reduced_motion: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<SettingsReducedMotionBtn>,
            Without<SettingsGraphicsBtn>,
        ),
    >,
    rumble: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<SettingsRumbleBtn>,
            Without<SettingsReducedMotionBtn>,
            Without<SettingsGraphicsBtn>,
        ),
    >,
    colorblind: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<SettingsColorblindWellsBtn>,
            Without<SettingsReducedMotionBtn>,
            Without<SettingsRumbleBtn>,
            Without<SettingsGraphicsBtn>,
        ),
    >,
) {
    if !label.settings_open {
        return;
    }
    let mut changed = false;
    for i in &graphics {
        if *i == Interaction::Pressed {
            // Comfort graphics preset — Low|Medium|High beside Grove persist.
            settings.inner.cycle_graphics_preset();
            changed = true;
        }
    }
    for i in &reduced_motion {
        if *i == Interaction::Pressed {
            settings.inner.toggle_reduced_motion();
            changed = true;
        }
    }
    for i in &rumble {
        if *i == Interaction::Pressed {
            settings.inner.toggle_rumble();
            changed = true;
        }
    }
    for i in &colorblind {
        if *i == Interaction::Pressed {
            settings.inner.cycle_colorblind_wells();
            changed = true;
        }
    }
    if changed {
        settings.mark_and_persist();
    }
}

fn refresh_lethal_sign_label(
    label: Res<HouseLabel>,
    hour: Option<Res<HourSacred>>,
    bind: Option<Res<LivedHourBind>>,
    mut texts: Query<&mut Text, With<SettingsLethalLabel>>,
    settings: Res<LocalSettingsState>,
) {
    if !label.settings_open {
        return;
    }
    // hour.complete is the Settled pack flag — no LedgerYard import (avoids a module cycle).
    let settled = hour.as_ref().map(|h| h.complete).unwrap_or(false);
    let book = hour
        .as_ref()
        .map(|h| h.hour_three_complete)
        .unwrap_or(false);
    let charter = hour
        .as_ref()
        .map(|h| h.charter_skin_live())
        .unwrap_or(false);
    let declared = bind
        .as_ref()
        .map(|b| b.standing.declared_lethal)
        .unwrap_or(false);
    let line = lethal_sign_btn_label(settled, book, charter, declared);
    let font = (15.0 * settings.inner.text_scale).clamp(11.0, 22.0);
    for mut text in &mut texts {
        set_btn_section_text(&mut text, line);
        set_btn_section_font(&mut text, font);
    }
}

fn lethal_sign_settings_clicks(
    label: Res<HouseLabel>,
    hour: Option<Res<HourSacred>>,
    mut bind: Option<ResMut<LivedHourBind>>,
    lethal: Query<&Interaction, (Changed<Interaction>, With<SettingsLethalBtn>)>,
) {
    if !label.settings_open {
        return;
    }
    let Some(hour) = hour else {
        return;
    };
    let Some(bind) = bind.as_mut() else {
        return;
    };
    let mut pressed = false;
    for i in &lethal {
        if *i == Interaction::Pressed {
            pressed = true;
        }
    }
    if !pressed {
        return;
    }
    let settled = hour.complete;
    if bind.standing.declared_lethal {
        if bind.standing.clear_lethal() {
            bind.refresh_climate_slab();
            bind.persist();
        }
        return;
    }
    if !bind
        .standing
        .confirm_hex_sign(settled, hour.hour_three_complete)
    {
        // Book / Settled missing — row already shows wait / not your charter.
        return;
    }
    let _paid = bind.climate.on_lethal_declare();
    bind.refresh_climate_slab();
    bind.persist();
}

fn refresh_controls_settings_labels(
    label: Res<HouseLabel>,
    settings: Res<LocalSettingsState>,
    mut texts: ParamSet<(
        Query<&mut Text, With<SettingsSticksLabel>>,
        Query<&mut Text, With<SettingsTapUseLabel>>,
        Query<&mut Text, With<SettingsSprintLabel>>,
    )>,
) {
    if !label.settings_open {
        return;
    }
    let s = &settings.inner;
    let sticks = sticks_btn_label(s);
    let tap = tap_use_btn_label(s);
    let sprint = sprint_btn_label(s);
    let font = (15.0 * s.text_scale).clamp(11.0, 22.0);
    for mut text in &mut texts.p0() {
        set_btn_section_text(&mut text, &sticks);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut texts.p1() {
        set_btn_section_text(&mut text, &tap);
        set_btn_section_font(&mut text, font);
    }
    for mut text in &mut texts.p2() {
        set_btn_section_text(&mut text, &sprint);
        set_btn_section_font(&mut text, font);
    }
}

/// D1: pause plate one-liner when Settings/pause is open in the yard.
pub fn pause_plate_line(door: LaunchDoor) -> Option<&'static str> {
    match door {
        LaunchDoor::InYard => Some(YARD_WAITING),
        LaunchDoor::Title | LaunchDoor::NameHouse | LaunchDoor::HouseDress => None,
    }
}

/// Pure helper: Resume keeps InYard (unpause only).
pub fn resume_keeps_door(from: LaunchDoor) -> LaunchDoor {
    from
}

/// Pure helper: Esc from InYard toggles pause open/closed; door stays InYard.
/// Never yields Title. `pause_was_open` is settings_open before the Esc press.
pub fn esc_from_inyard_toggles_pause(from: LaunchDoor, pause_was_open: bool) -> (LaunchDoor, bool) {
    match from {
        LaunchDoor::InYard => (LaunchDoor::InYard, !pause_was_open),
        other => (other, pause_was_open),
    }
}

/// Pause plate state after one yard pause press.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YardPause {
    pub pause_open: bool,
    pub places_open: bool,
}

/// One yard pause verb for Esc, pad Start, key 3, and the touch pause chip.
///
/// The door decides, never the hex: Sanctuary, Heartwood, and any later local
/// hex reach the same plate. Places is a leaf of that plate, so a press with
/// Places open walks back to pause instead of eating the press and leaving the
/// walker on a bare yard.
pub fn yard_pause_step(
    door: LaunchDoor,
    pause_was_open: bool,
    places_was_open: bool,
) -> Option<YardPause> {
    if door != LaunchDoor::InYard {
        return None;
    }
    if places_was_open {
        return Some(YardPause {
            pause_open: true,
            places_open: false,
        });
    }
    Some(YardPause {
        pause_open: !pause_was_open,
        places_open: false,
    })
}

/// A confirmed leave lands the walker in the new hex's yard — no plate held
/// over from the hex before, so the next Esc there opens pause.
pub const fn yard_after_travel() -> YardPause {
    YardPause {
        pause_open: false,
        places_open: false,
    }
}

/// Pure helper: Title button from pause / InYard yields Title (house JSON path).
pub fn title_from_pause_returns_title(from: LaunchDoor) -> LaunchDoor {
    match from {
        LaunchDoor::InYard => LaunchDoor::Title,
        other => other,
    }
}

fn name_house_text_input(
    door: Res<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    persona: Res<PersonaCreatorState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut chars: EventReader<ReceivedCharacter>,
    mut draft_q: Query<&mut Text, With<NameDraftText>>,
) {
    if *door != LaunchDoor::NameHouse {
        // Drain SmolStr ReceivedCharacter so Continue/name does not flicker with stale input.
        // Leave the stream alone while the gated persona creator owns typing.
        if !persona.open {
            chars.clear();
        }
        return;
    }
    // Bevy 0.14: ReceivedCharacter.char is SmolStr (deprecated API; still compiles).
    for ev in chars.read() {
        for c in ev.char.chars() {
            if c.is_control() {
                continue;
            }
            if (c.is_alphanumeric() || c == ' ' || c == '-' || c == '\'') && label.draft.len() < 32
            {
                label.draft.push(c);
            }
        }
    }
    if keyboard.just_pressed(KeyCode::Backspace) {
        label.draft.pop();
    }
    let shown = if label.draft.is_empty() {
        format!("_  ({UNNAMED})")
    } else {
        label.draft.clone()
    };
    for mut text in &mut draft_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != shown {
                s.value = shown.clone();
            }
        }
    }
}

fn name_house_buttons(
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    confirm: Query<&Interaction, (Changed<Interaction>, With<NameConfirmBtn>)>,
    skip: Query<&Interaction, (Changed<Interaction>, With<NameSkipBtn>)>,
) {
    if *door != LaunchDoor::NameHouse {
        return;
    }
    let mut do_confirm = keyboard.just_pressed(KeyCode::Enter);
    let mut do_skip = keyboard.just_pressed(KeyCode::Escape);
    for i in &confirm {
        if *i == Interaction::Pressed {
            do_confirm = true;
        }
    }
    for i in &skip {
        if *i == Interaction::Pressed {
            do_skip = true;
        }
    }
    if do_confirm {
        let draft = label.draft.clone();
        // Rename path: already resolved → rename keeps seals/heritage.
        if label.house.resolved && label.house.seals_resolved {
            label.house.rename(&draft);
        } else {
            label.house.confirm(&draft);
        }
        label.house.persist();
        label.persist_present = true;
        *door = advance_after_naming(&mut label);
    } else if do_skip {
        if label.house.seals_resolved {
            // Rename cancel after dress — keep existing name, back to yard.
            *door = LaunchDoor::InYard;
        } else if label.house.resolved && label.seals_offered {
            // Mid-dress rename cancel — keep name, return to seals panel.
            *door = LaunchDoor::HouseDress;
        } else {
            label.house.skip();
            label.house.persist();
            label.persist_present = true;
            *door = advance_after_naming(&mut label);
        }
    }
}

/// After naming: D3 dress if seals not yet resolved; else yard.
fn advance_after_naming(label: &mut HouseLabel) -> LaunchDoor {
    if label.house.resolved && !label.house.seals_resolved {
        label.seals_offered = true;
        LaunchDoor::HouseDress
    } else {
        LaunchDoor::InYard
    }
}

fn sync_name_house_visibility(
    door: Res<LaunchDoor>,
    mut q: Query<&mut Visibility, With<NameHouseRoot>>,
) {
    let show = *door == LaunchDoor::NameHouse;
    for mut vis in &mut q {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn seal_btn_label(house: &HouseName, id: &str) -> String {
    let on = house.seals.iter().any(|s| s == id);
    format!(
        "Seal · {} · {}",
        HouseName::seal_label(id),
        if on { "on" } else { "off" }
    )
}

pub fn heritage_btn_label(house: &HouseName) -> String {
    format!("Heritage · {}", house.heritage)
}

fn refresh_house_dress_labels(
    door: Res<LaunchDoor>,
    label: Res<HouseLabel>,
    mut texts: ParamSet<(
        Query<&mut Text, With<DressSealWellLabel>>,
        Query<&mut Text, With<DressSealGroveLabel>>,
        Query<&mut Text, With<DressSealEmberLabel>>,
        Query<&mut Text, With<DressHeritageLabel>>,
    )>,
) {
    if *door != LaunchDoor::HouseDress {
        return;
    }
    let h = &label.house;
    let well = seal_btn_label(h, SEAL_WELL);
    let grove = seal_btn_label(h, SEAL_GROVE);
    let ember = seal_btn_label(h, SEAL_EMBER);
    let heritage = heritage_btn_label(h);
    for mut text in &mut texts.p0() {
        set_btn_section_text(&mut text, &well);
    }
    for mut text in &mut texts.p1() {
        set_btn_section_text(&mut text, &grove);
    }
    for mut text in &mut texts.p2() {
        set_btn_section_text(&mut text, &ember);
    }
    for mut text in &mut texts.p3() {
        set_btn_section_text(&mut text, &heritage);
    }
}

fn house_dress_buttons(
    mut door: ResMut<LaunchDoor>,
    mut label: ResMut<HouseLabel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    well: Query<&Interaction, (Changed<Interaction>, With<DressSealWellBtn>)>,
    grove: Query<&Interaction, (Changed<Interaction>, With<DressSealGroveBtn>)>,
    ember: Query<&Interaction, (Changed<Interaction>, With<DressSealEmberBtn>)>,
    heritage: Query<&Interaction, (Changed<Interaction>, With<DressHeritageBtn>)>,
    rename: Query<&Interaction, (Changed<Interaction>, With<DressRenameBtn>)>,
    confirm: Query<&Interaction, (Changed<Interaction>, With<DressConfirmBtn>)>,
    skip: Query<&Interaction, (Changed<Interaction>, With<DressSkipBtn>)>,
) {
    if *door != LaunchDoor::HouseDress {
        return;
    }
    for i in &well {
        if *i == Interaction::Pressed {
            label.house.toggle_seal(SEAL_WELL);
        }
    }
    for i in &grove {
        if *i == Interaction::Pressed {
            label.house.toggle_seal(SEAL_GROVE);
        }
    }
    for i in &ember {
        if *i == Interaction::Pressed {
            label.house.toggle_seal(SEAL_EMBER);
        }
    }
    for i in &heritage {
        if *i == Interaction::Pressed {
            label.house.bump_heritage();
        }
    }
    for i in &rename {
        if *i == Interaction::Pressed {
            // Rename allowed — return to name panel with current name as draft.
            label.draft = if label.house.name.is_empty() {
                String::new()
            } else {
                label.house.name.clone()
            };
            // Mark seals resolved temporarily? No — keep dress state; name confirm
            // with seals_resolved false would re-enter dress. Set seals_resolved
            // only on Confirm/Skip. For rename mid-dress, go to NameHouse and
            // come back via advance_after_naming.
            *door = LaunchDoor::NameHouse;
            return;
        }
    }
    let mut do_confirm = keyboard.just_pressed(KeyCode::Enter);
    let mut do_skip = keyboard.just_pressed(KeyCode::Escape);
    for i in &confirm {
        if *i == Interaction::Pressed {
            do_confirm = true;
        }
    }
    for i in &skip {
        if *i == Interaction::Pressed {
            do_skip = true;
        }
    }
    if do_confirm {
        label.house.confirm_seals();
        // Heritage already set via bump; persist caption as-is.
        if !HouseName::is_valid_heritage(&label.house.heritage) {
            label.house.skip_heritage();
        }
        label.house.persist();
        label.persist_present = true;
        *door = LaunchDoor::InYard;
    } else if do_skip {
        label.house.skip_seals();
        label.house.skip_heritage();
        label.house.persist();
        label.persist_present = true;
        *door = LaunchDoor::InYard;
    }
}

fn sync_house_dress_visibility(
    door: Res<LaunchDoor>,
    mut q: Query<&mut Visibility, With<HouseDressRoot>>,
) {
    let show = *door == LaunchDoor::HouseDress;
    for mut vis in &mut q {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}


fn sync_persona_creator_visibility(
    persona: Res<PersonaCreatorState>,
    mut q: Query<&mut Visibility, With<PersonaCreatorRoot>>,
) {
    let show = persona.open && PERSONA_CREATOR_ENABLED;
    for mut vis in &mut q {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn refresh_persona_creator_labels(
    persona: Res<PersonaCreatorState>,
    mut texts: ParamSet<(
        Query<&mut Text, With<PersonaCreatorStepLabel>>,
        Query<&mut Text, With<PersonaCreatorGuidance>>,
        Query<&mut Text, With<PersonaCreatorBodyText>>,
        Query<&mut Text, With<PersonaModuleLabel>>,
        Query<&mut Text, With<PersonaPeopleLabel>>,
        Query<&mut Text, With<PersonaPhenotypeLabel>>,
        Query<&mut Text, With<PersonaStoryShareLabel>>,
        Query<&mut Text, With<PersonaOnlinePickerLabel>>,
    )>,
) {
    if !persona.open || !PERSONA_CREATOR_ENABLED {
        return;
    }
    let step = persona.step.as_label().to_string();
    let guidance = if persona.guidance_hidden {
        String::new()
    } else {
        persona_step_guidance(persona.step).to_string()
    };
    let body = match persona.step {
        PersonaCreatorStep::Story | PersonaCreatorStep::Preview => {
            let name = if persona.name_draft.is_empty() {
                "(nameless Steward)".into()
            } else {
                persona.name_draft.clone()
            };
            let story = if persona.story_draft.is_empty() {
                "(no story yet)".into()
            } else {
                let t: String = persona.story_draft.chars().take(80).collect();
                format!("{t}…")
            };
            if persona.step == PersonaCreatorStep::Preview {
                persona_preview_summary(&persona.draft)
            } else {
                format!("Name · {name} · Story · {story}")
            }
        }
        _ => {
            if persona.name_draft.is_empty() {
                "Name · (type when on Story · empty = nameless Steward)".into()
            } else {
                format!("Name · {}", persona.name_draft)
            }
        }
    };
    let module = mechanical_module_btn_label(persona.draft.mechanical_race);
    let people = people_choice_btn_label(&persona);
    let pheno = phenotype_btn_label(&persona.draft.presentation.phenotype);
    let share = story_share_btn_label(persona.draft.presentation.story.shared_in_world);
    let picker = story_provider_btn_label(
        persona.story_provider,
        ONLINE_PICKER_ENABLED,
        STEWARD_ONLINE_YES,
    );

    for mut text in &mut texts.p0() {
        set_btn_section_text(&mut text, &step);
    }
    for mut text in &mut texts.p1() {
        set_btn_section_text(&mut text, &guidance);
    }
    for mut text in &mut texts.p2() {
        set_btn_section_text(&mut text, &body);
    }
    for mut text in &mut texts.p3() {
        set_btn_section_text(&mut text, &module);
    }
    for mut text in &mut texts.p4() {
        set_btn_section_text(&mut text, &people);
    }
    for mut text in &mut texts.p5() {
        set_btn_section_text(&mut text, &pheno);
    }
    for mut text in &mut texts.p6() {
        set_btn_section_text(&mut text, &share);
    }
    for mut text in &mut texts.p7() {
        set_btn_section_text(&mut text, &picker);
    }
}

fn persona_creator_text_input(
    mut persona: ResMut<PersonaCreatorState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut chars: EventReader<ReceivedCharacter>,
) {
    if !persona.open || !PERSONA_CREATOR_ENABLED {
        return;
    }
    // Type into name on early steps; story text on Story/Preview.
    let into_story = matches!(
        persona.step,
        PersonaCreatorStep::Story | PersonaCreatorStep::Preview
    );
    for ev in chars.read() {
        for c in ev.char.chars() {
            if c.is_control() {
                continue;
            }
            if into_story {
                if persona.story_draft.chars().count() < STORY_TEXT_MAX
                    && (c.is_alphanumeric()
                        || c.is_whitespace()
                        || matches!(c, '.' | ',' | '\'' | '-' | '!' | '?' | ';'))
                {
                    persona.story_draft.push(c);
                }
            } else if persona.name_draft.chars().count() < GIVEN_NAME_MAX
                && (c.is_alphanumeric() || c == ' ' || c == '-' || c == '\'')
            {
                persona.name_draft.push(c);
            }
        }
    }
    if keyboard.just_pressed(KeyCode::Backspace) {
        if into_story {
            persona.story_draft.pop();
        } else {
            persona.name_draft.pop();
        }
    }
}

fn persona_creator_buttons(
    mut persona: ResMut<PersonaCreatorState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    module: Query<&Interaction, (Changed<Interaction>, With<PersonaModuleBtn>)>,
    people: Query<&Interaction, (Changed<Interaction>, With<PersonaPeopleBtn>)>,
    pheno: Query<&Interaction, (Changed<Interaction>, With<PersonaPhenotypeBtn>)>,
    share: Query<&Interaction, (Changed<Interaction>, With<PersonaStoryShareBtn>)>,
    picker: Query<&Interaction, (Changed<Interaction>, With<PersonaOnlinePickerBtn>)>,
    next: Query<&Interaction, (Changed<Interaction>, With<PersonaNextBtn>)>,
    back: Query<&Interaction, (Changed<Interaction>, With<PersonaBackBtn>)>,
    keep: Query<&Interaction, (Changed<Interaction>, With<PersonaKeepDraftBtn>)>,
    commit_btn: Query<&Interaction, (Changed<Interaction>, With<PersonaCommitBtn>)>,
    skip_btn: Query<&Interaction, (Changed<Interaction>, With<PersonaSkipNamelessBtn>)>,
    hide: Query<&Interaction, (Changed<Interaction>, With<PersonaHideGuidanceBtn>)>,
) {
    if !persona.open || !PERSONA_CREATOR_ENABLED {
        return;
    }

    for i in &module {
        if *i == Interaction::Pressed {
            persona.draft.mechanical_race = cycle_mechanical_module(persona.draft.mechanical_race);
        }
    }
    for i in &people {
        if *i == Interaction::Pressed {
            bump_people_paint(&mut persona);
        }
    }
    for i in &pheno {
        if *i == Interaction::Pressed {
            nudge_phenotype_paint(&mut persona.draft.presentation.phenotype);
        }
    }
    for i in &share {
        if *i == Interaction::Pressed {
            persona.draft.presentation.story.shared_in_world =
                cycle_story_share(persona.draft.presentation.story.shared_in_world);
        }
    }
    for i in &picker {
        if *i == Interaction::Pressed {
            // Online rows only when flag + steward online yes; else offline cycle
            // (or no-op when UI spawned disabled). Never lights Title Online.
            if online_picker_ui_enabled(ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES)
                || ONLINE_PICKER_ENABLED
            {
                persona.story_provider = cycle_story_provider(
                    persona.story_provider,
                    ONLINE_PICKER_ENABLED,
                    STEWARD_ONLINE_YES,
                );
                apply_story_provider_seat(&mut persona);
            }
        }
    }
    for i in &hide {
        if *i == Interaction::Pressed {
            persona.guidance_hidden = !persona.guidance_hidden;
        }
    }
    if keyboard.just_pressed(KeyCode::KeyH) {
        persona.guidance_hidden = !persona.guidance_hidden;
    }

    let mut do_next = false;
    let mut do_back = false;
    let mut do_keep = false;
    let mut do_commit = false;
    let mut do_skip = keyboard.just_pressed(KeyCode::Escape);
    for i in &next {
        if *i == Interaction::Pressed {
            do_next = true;
        }
    }
    for i in &back {
        if *i == Interaction::Pressed {
            do_back = true;
        }
    }
    for i in &keep {
        if *i == Interaction::Pressed {
            do_keep = true;
        }
    }
    for i in &commit_btn {
        if *i == Interaction::Pressed {
            do_commit = true;
        }
    }
    for i in &skip_btn {
        if *i == Interaction::Pressed {
            do_skip = true;
        }
    }
    // Preview Enter = Commit (P4). Keep draft remains a separate soft path.
    if keyboard.just_pressed(KeyCode::Enter) && persona.step == PersonaCreatorStep::Preview {
        do_commit = true;
    } else if keyboard.just_pressed(KeyCode::Enter) {
        do_next = true;
    }

    if do_skip {
        skip_persona_to_nameless(&mut persona);
        return;
    }
    if do_commit {
        let _ = commit_persona_from_soft_draft(&mut persona);
        return;
    }
    if do_keep {
        keep_persona_local_draft(&mut persona);
        return;
    }
    if do_next {
        // Sync typed buffers into draft before leaving Story.
        if !persona.name_draft.is_empty() {
            let mut n = persona.name_draft.clone();
            if n.chars().count() > GIVEN_NAME_MAX {
                n = n.chars().take(GIVEN_NAME_MAX).collect();
            }
            persona.draft.presentation.given_name = n;
        }
        if !persona.story_draft.is_empty() {
            persona.draft.presentation.story.player_text = persona.story_draft.clone();
            persona.draft.presentation.story.player_accepted = true;
        }
        persona.step = persona.step.next();
    } else if do_back {
        persona.step = persona.step.back();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::house_name::{
        continue_cue, continue_cue_when_persist, esc_from_title_preserves_persist, YARD_REMEMBERS,
    };
    use shared::stranger_loop_proof::{hour_two_held_fixture, peace_fixture};
    use shared::title_house_proof::online_row_is_honest_disabled;

    #[test]
    fn play_does_not_require_house_name() {
        let house = HouseName::default();
        assert!(!house.blocks_hands());
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
        assert!(continue_cue(false, &house).is_none());
    }

    #[test]
    fn continue_cue_unnamed_house_yard_remembers() {
        assert!(local_persist_present(true, false, false, false));
        let mut house = HouseName::default();
        house.skip();
        let cue = continue_cue_when_persist(true, &house).unwrap();
        assert_eq!(cue, "Unnamed House · the yard remembers");
        assert!(cue.contains(UNNAMED));
        assert!(cue.contains(YARD_REMEMBERS));
        let spoken = continue_yard_remembered_line(&cue);
        assert_eq!(spoken, "Unnamed House · the yard remembered");
        assert!(spoken.contains("yard remembered"));
        assert_eq!(CONTINUE_YARD_REMEMBERED, "the yard remembered");
        let mut named = HouseName::default();
        named.confirm("Yard");
        let cue2 = continue_cue(true, &named).unwrap();
        assert_eq!(cue2, "Yard · the yard remembers");
        assert_eq!(
            continue_yard_remembered_line(&cue2),
            "Yard · the yard remembered"
        );
    }

    /// CARD FLESH-CONTINUE-LINE — Continue speaks the Guide peak. One beat.
    #[test]
    fn flesh_continue_line_speaks_yard_remembered() {
        let mut house = HouseName::default();
        house.skip();
        let cue = continue_cue_when_persist(true, &house).unwrap();
        let spoken = continue_yard_remembered_line(&cue);
        assert!(spoken.contains("yard remembered"));
        assert_eq!(spoken, "Unnamed House · the yard remembered");
        assert!(!spoken.contains("remembers"));
        assert!(PAUSE_GUIDE_LINE.contains("yard remembered"));
        assert!(PAUSE_GUIDE_LINE.contains(CONTINUE_YARD_REMEMBERED));
        // Sealed roster stays on the same line — not a second plate.
        let with_roster =
            continue_yard_remembered_line("Unnamed House · the yard remembers · Human · Sanctuary");
        assert_eq!(
            with_roster,
            "Unnamed House · the yard remembered · Human · Sanctuary"
        );
        assert!(with_roster.contains("yard remembered"));
        // First-run Play cue does not grow the peak.
        let play = "Play opens the yard · no account wall";
        assert_eq!(continue_yard_remembered_line(play), play);
        assert!(!continue_yard_remembered_line(play).contains("yard remembered"));
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
    }

    #[test]
    fn esc_from_title_does_not_clear_persist_files() {
        let mut house = HouseName::default();
        house.confirm("Keep Me");
        let house_json = house.to_json().unwrap();
        let climate = r#"{"harmony":1.0}"#;
        let standing = r#"{"declared_lethal":false}"#;
        let book = r#"{"complete":true}"#;
        // Esc-from-title only flips settings_open=false; payloads identical.
        let mut settings_open = true;
        settings_open = false; // Esc on Title
        assert!(!settings_open);
        assert!(esc_from_title_preserves_persist(
            &house_json,
            &house_json,
            climate,
            climate,
            standing,
            standing,
            book,
            book,
        ));
        let back = HouseName::from_json(&house_json).unwrap();
        assert!(back.resolved);
        assert_eq!(back.display_name(), "Keep Me");
    }

    #[test]
    fn online_row_visible_disabled_honest() {
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        let net = SessionNetMode::default();
        assert!(net.mode.peer_count_for_peace_boot().is_none());
    }

    #[test]
    fn house_path_constant() {
        assert_eq!(HOUSE_PATH, "data/powrush_house.json");
    }

    #[test]
    fn peace_fixture_lethal_untouched() {
        let (_p, _c, standing, _) = peace_fixture();
        assert!(!standing.declared_lethal);
        let h2 = hour_two_held_fixture();
        assert!(h2.complete);
    }

    #[test]
    fn title_default_is_launch_door_title() {
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
    }

    /// CARD L1 GARDEN-WANT — stranger on Title / God-plane hears Want before Play.
    #[test]
    fn garden_boot_title_speaks_want_before_play() {
        use crate::first_session_guidance::{
            first_minutes_people_want_line, garden_boot_want_line, FirstSessionGuidance,
            GARDEN_PEOPLE, GARDEN_WANT, SANCTUARY_WANT,
        };
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
        let spoken = garden_boot_want_line(true, false).expect("Want on Garden boot");
        assert_eq!(spoken, first_minutes_people_want_line());
        assert!(spoken.contains(GARDEN_PEOPLE));
        assert!(spoken.contains(GARDEN_WANT));
        assert!(spoken.contains(SANCTUARY_WANT));
        assert!(spoken.contains("tend"));
        assert!(spoken.contains("the well goes quiet"));
        // Sanctuary dirt / InYard is not required for the boot-plane line.
        assert!(garden_boot_want_line(false, false).is_none());
        let mut g = FirstSessionGuidance::default();
        assert!(g.speaks_people_want());
        g.dismiss();
        assert!(garden_boot_want_line(true, !g.speaks_people_want()).is_none());
        // Title chrome unchanged · Online grey · 0 Places.
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
    }

    /// CARD L1 GARDEN-WANT — Title spawn speaks Want; Comfort Low stays readable.
    #[test]
    fn garden_boot_title_spawn_shows_want_comfort_low_readable() {
        use crate::first_session_guidance::{GARDEN_WANT, SANCTUARY_PEOPLE};
        let mut app = App::new();
        app.add_systems(Startup, spawn_title_screen);
        app.update();

        let world = app.world_mut();
        let text = world
            .query_filtered::<&Text, With<TitleGardenWantText>>()
            .single(world);
        let line = &text.sections[0].value;
        assert!(line.contains(SANCTUARY_PEOPLE));
        assert!(line.contains(GARDEN_WANT));
        assert!(line.contains("the yard needs tending or the well goes quiet"));
        assert_eq!(
            text.sections[0].style.color,
            TITLE_TEXT_PRIMARY,
            "Garden Want uses primary title text for Comfort Low"
        );
        assert!(
            title_luminance(TITLE_TEXT_PRIMARY) > title_luminance(TITLE_PLATE_BG),
            "Comfort Low readable: Want stays light-on-opaque-dark"
        );
        assert!(title_contrast_is_high());
        let mut low = LocalSettings::peace_defaults();
        low.set_graphics_preset(GraphicsPreset::Low);
        assert!(low.text_scale >= 1.10);
        assert!(garden_boot_want_font_px(low.text_scale) >= garden_boot_want_font_px(1.0));
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

    /// CARD L2 HOUSE-PEOPLE-GATES — Title chrome holds; Garden hosts 5 doors.
    /// Not a race lobby. Not the #459 dress-token prove-line. Online grey.
    #[test]
    fn l2_title_chrome_play_continue_settings_online_grey() {
        assert!(l2_title_chrome_holds());
        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!title_has_race_portraits());
        assert_eq!(garden_boot_god_plane_doors(), HOUSE_PEOPLES);
        assert_eq!(garden_boot_god_plane_doors().len(), 5);
        assert!(four_place_landings_only());
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
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
        assert_eq!(L2_MESH_BUDGET, 0);
        assert_eq!(L2_ASSET_BUDGET_CITE, "docs/ASSET_BUDGET_COURT.md");
        // Refuse old #459 dress-token prove-line.
        for people in HOUSE_PEOPLES {
            assert!(!people.people_line().contains("Sanctuary tint"));
            assert!(!people.people_line().contains("dress token"));
        }

        let mut app = App::new();
        app.add_systems(Startup, spawn_title_screen);
        app.update();
        let world = app.world_mut();
        assert_eq!(
            world
                .query_filtered::<(), With<TitlePlayBtn>>()
                .iter(world)
                .count(),
            1
        );
        assert_eq!(
            world
                .query_filtered::<(), With<TitleContinueBtn>>()
                .iter(world)
                .count(),
            1
        );
        assert_eq!(
            world
                .query_filtered::<(), With<TitleSettingsBtn>>()
                .iter(world)
                .count(),
            1
        );
        assert_eq!(
            world
                .query_filtered::<(), With<TitleOnlineBtn>>()
                .iter(world)
                .count(),
            1
        );
    }

    /// CARD L2 — Garden doors stay dark until Tend; skip House stays light;
    /// one-way cross this session. Ambrosian shares Sanctuary.
    #[test]
    fn l2_garden_doors_ignite_after_tend_one_way() {
        assert!(skip_house_stays_light(false));
        assert!(offer_house_peoples(false).is_none());
        assert!(!garden_door_may_cross(false, true, false));
        assert!(!garden_door_may_cross(true, false, false));
        assert!(garden_door_may_cross(true, true, false));
        assert!(!garden_door_may_cross(true, true, true));
        let mut crossed = None;
        let mut travel = HexTravelState {
            current: shared::hex_travel::PlaceId::Sanctuary,
        };
        let mut bind = l5_demo_bind();
        assert!(garden_cross_landing(
            true,
            false,
            &mut crossed,
            HousePeople::Draek,
            &mut travel,
            &mut bind,
            None,
            None,
        )
        .is_none());
        assert!(garden_cross_landing(
            false,
            true,
            &mut crossed,
            HousePeople::Draek,
            &mut travel,
            &mut bind,
            None,
            None,
        )
        .is_none());
        let land = garden_cross_landing(
            true,
            true,
            &mut crossed,
            HousePeople::Draek,
            &mut travel,
            &mut bind,
            None,
            None,
        )
        .expect("House + Tend opens Draek door");
        assert_eq!(land, PeopleLanding::DepthsTealWayHome);
        assert_eq!(land.landing_line(), "Depths (teal way-home)");
        assert!(garden_cross_landing(
            true,
            true,
            &mut crossed,
            HousePeople::Ambrosian,
            &mut travel,
            &mut bind,
            None,
            None,
        )
        .is_none());
        assert_eq!(
            HousePeople::Ambrosian.landing().place_name(),
            HousePeople::Human.landing().place_name()
        );
        assert_eq!(
            HousePeople::Ambrosian.landing().landing_line(),
            "Sanctuary well-from-above"
        );
        assert!(l2_title_chrome_holds());
        assert_eq!(L2_MESH_BUDGET, 0);
    }

    fn l5_demo_bind() -> LivedHourBind {
        LivedHourBind {
            hour: shared::climate_node::LivedHour::new_demo(),
            climate: Default::default(),
            standing: Default::default(),
            week: Default::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        }
    }

    fn l5_garden_land(
        house: bool,
        tend: bool,
        people: HousePeople,
        start: shared::hex_travel::PlaceId,
        mut presence: Option<&mut SoftPresence>,
    ) -> (Option<PeopleLanding>, shared::hex_travel::PlaceId) {
        let mut travel = HexTravelState { current: start };
        let mut bind = l5_demo_bind();
        let mut crossed = None;
        let land = garden_cross_landing(
            house,
            tend,
            &mut crossed,
            people,
            &mut travel,
            &mut bind,
            None,
            presence.as_deref_mut(),
        );
        (land, travel.current)
    }

    /// CARD L5 — skip House garden cross → none, PlaceId unchanged.
    #[test]
    fn l5_skip_house_garden_cross_none_place_id_unchanged() {
        use shared::hex_travel::PlaceId;
        let start = PlaceId::Sanctuary;
        let (land, now) = l5_garden_land(false, true, HousePeople::Human, start, None);
        assert!(land.is_none());
        assert_eq!(now, start);
        let (land, now) = l5_garden_land(true, false, HousePeople::Draek, start, None);
        assert!(land.is_none());
        assert_eq!(now, start);
        assert!(skip_house_stays_light(false));
    }

    /// CARD L5 — Human garden cross after House+Tend → Sanctuary + Sanctuary Prime dress.
    #[test]
    fn l5_human_garden_cross_sanctuary_prime_dress() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) = l5_garden_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryYard));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(dress_token_for_place(now), "Sanctuary Prime");
    }

    /// CARD L5 — Cydruid → Heartwood + Verdant Heartwood + people_line human-in-frame.
    #[test]
    fn l5_cydruid_garden_cross_heartwood_verdant_human_in_frame() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) = l5_garden_land(true, true, HousePeople::Cydruid, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::Heartwood));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(dress_token_for_place(now), "Verdant Heartwood");
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
    }

    /// CARD L5 — Quellorian → Heartwood + threshold_use_in_reach.
    #[test]
    fn l5_quellorian_garden_cross_heartwood_threshold_use_in_reach() {
        use crate::climate_plane::dress_token_for_place;
        use crate::human_presence::people_landing_wake;
        use shared::hex_travel::PlaceId;
        use shared::threshold_shelf::threshold_use_in_reach;

        let mut presence = SoftPresence::default();
        let (land, now) = l5_garden_land(
            true,
            true,
            HousePeople::Quellorian,
            PlaceId::Sanctuary,
            Some(&mut presence),
        );
        assert_eq!(land, Some(PeopleLanding::Threshold));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(dress_token_for_place(now), "Verdant Heartwood");
        let wake = people_landing_wake(PeopleLanding::Threshold);
        assert_eq!(presence.position, wake);
        assert!(threshold_use_in_reach(
            now,
            presence.position.x,
            presence.position.z
        ));
    }

    /// CARD L5 — Draek → Depths + Abyssal Depths.
    #[test]
    fn l5_draek_garden_cross_depths_abyssal() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) = l5_garden_land(true, true, HousePeople::Draek, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::DepthsTealWayHome));
        assert_eq!(now, PlaceId::Depths);
        assert_eq!(dress_token_for_place(now), "Abyssal Depths");
    }

    /// CARD L5 — Ambrosian → Sanctuary PlaceId same as Human.
    #[test]
    fn l5_ambrosian_garden_cross_sanctuary_same_as_human() {
        use crate::climate_plane::dress_token_for_place;
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_garden_land(true, true, HousePeople::Ambrosian, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryWellFromAbove));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(now, HousePeople::Human.landing().place_id());
        assert_eq!(dress_token_for_place(now), "Sanctuary Prime");
        assert_eq!(
            HousePeople::Ambrosian.landing().place_id(),
            HousePeople::Human.landing().place_id()
        );
    }

    /// CARD L5 — Title chrome strings still Play / Continue / Settings.
    #[test]
    fn l5_title_chrome_play_continue_settings() {
        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(l2_title_chrome_holds());
        assert!(!title_has_race_portraits());
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
    }

    /// CARD L5 — STEWARD_ONLINE_YES stays false. Title Online grey.
    #[test]
    fn l5_steward_online_yes_stays_false() {
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
        assert!(!online_picker_ui_enabled(
            ONLINE_PICKER_ENABLED,
            STEWARD_ONLINE_YES
        ));
    }

    /// CARD L6 — skip House / Garden boot keep GARDEN_WANT · SANCTUARY_WANT.
    #[test]
    fn l6_skip_house_garden_boot_want_unchanged() {
        use crate::first_session_guidance::{
            garden_boot_want_line, want_after_people_landing, GARDEN_WANT, SANCTUARY_WANT,
        };
        use shared::hex_travel::PlaceId;

        assert_eq!(GARDEN_WANT, SANCTUARY_WANT);
        let spoken = garden_boot_want_line(true, false).expect("Want on Garden boot");
        assert!(spoken.contains(GARDEN_WANT));
        assert!(spoken.contains(SANCTUARY_WANT));
        let (land, now) = l5_garden_land(false, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert!(land.is_none());
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(want_after_people_landing(land), GARDEN_WANT);
        assert_eq!(garden_boot_want_line(true, false).expect("still Garden"), spoken);
    }

    /// CARD L6 — Human land → Sanctuary Want. Title still calls garden_boot_want_line.
    #[test]
    fn l6_human_land_sanctuary_want() {
        use crate::first_session_guidance::{
            garden_boot_want_line, want_after_people_landing, SANCTUARY_WANT,
        };
        use shared::hex_travel::PlaceId;

        let (land, now) = l5_garden_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryYard));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(want_after_people_landing(land), SANCTUARY_WANT);
        assert!(garden_boot_want_line(true, false)
            .expect("Garden boot unchanged")
            .contains(SANCTUARY_WANT));
    }

    /// CARD L6 — Ambrosian land → same Want as Human.
    #[test]
    fn l6_ambrosian_land_same_want_as_human() {
        use crate::first_session_guidance::want_after_people_landing;
        use shared::hex_travel::PlaceId;

        let (human, _) = l5_garden_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        let (ambrosian, now) =
            l5_garden_land(true, true, HousePeople::Ambrosian, PlaceId::Sanctuary, None);
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(
            want_after_people_landing(ambrosian),
            want_after_people_landing(human)
        );
    }

    /// CARD L6 — Cydruid land → Heartwood Want · people_line human-in-frame (NOT treant).
    #[test]
    fn l6_cydruid_land_heartwood_want_human_in_frame() {
        use crate::first_session_guidance::{want_after_people_landing, HEARTWOOD_WANT};
        use shared::hex_travel::PlaceId;

        let (land, now) = l5_garden_land(true, true, HousePeople::Cydruid, PlaceId::Sanctuary, None);
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(want_after_people_landing(land), HEARTWOOD_WANT);
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
    }

    /// CARD L6 — Quellorian land → Heartwood Want (not Sanctuary well).
    #[test]
    fn l6_quellorian_land_heartwood_want_not_sanctuary_well() {
        use crate::first_session_guidance::{
            want_after_people_landing, HEARTWOOD_WANT, SANCTUARY_WANT,
        };
        use shared::hex_travel::PlaceId;

        let (land, now) =
            l5_garden_land(true, true, HousePeople::Quellorian, PlaceId::Sanctuary, None);
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(want_after_people_landing(land), HEARTWOOD_WANT);
        assert_ne!(want_after_people_landing(land), SANCTUARY_WANT);
        assert!(!want_after_people_landing(land).contains("the well goes quiet"));
    }

    /// CARD L6 — Draek land → Depths Want (restore, not Take).
    #[test]
    fn l6_draek_land_depths_want_restore_not_take() {
        use crate::first_session_guidance::{want_after_people_landing, DEPTHS_WANT};
        use shared::hex_travel::PlaceId;

        let (land, now) = l5_garden_land(true, true, HousePeople::Draek, PlaceId::Sanctuary, None);
        assert_eq!(now, PlaceId::Depths);
        assert_eq!(want_after_people_landing(land), DEPTHS_WANT);
        assert!(want_after_people_landing(land).contains("restored"));
        assert!(!want_after_people_landing(land).contains("Take"));
    }

    /// CARD L6 — TITLE_CHROME_PLAY / Continue / Settings unchanged.
    #[test]
    fn l6_title_chrome_play_continue_settings() {
        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(l2_title_chrome_holds());
        assert!(!title_has_race_portraits());
    }

    /// CARD L6 — STEWARD_ONLINE_YES stays false. Online grey.
    #[test]
    fn l6_steward_online_yes_stays_false() {
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
    }

    /// CARD S1 — skip House Title garden keeps GARDEN_WANT / Sanctuary boot Want.
    #[test]
    fn s1_skip_house_title_garden_keeps_garden_sanctuary_boot_want() {
        use crate::first_session_guidance::{
            aftermath_after_people_landing, garden_guidance_after_land, want_after_people_landing,
            GARDEN_WANT, SANCTUARY_WANT,
        };
        use shared::hex_travel::PlaceId;

        assert_eq!(GARDEN_WANT, SANCTUARY_WANT);
        let spoken = garden_guidance_after_land(true, false, None).expect("Garden boot");
        assert!(spoken.contains(GARDEN_WANT));
        let (land, now) = l5_garden_land(false, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert!(land.is_none());
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(want_after_people_landing(land), GARDEN_WANT);
        assert_eq!(aftermath_after_people_landing(land), GARDEN_WANT);
        assert_eq!(
            garden_guidance_after_land(true, false, land).expect("still Garden"),
            spoken
        );
    }

    /// CARD S1 — Human Title garden after land: yard teaching · war rumor at well.
    /// Want stays Sanctuary. Chrome unchanged.
    #[test]
    fn s1_human_title_garden_aftermath_yard_teaching_war_rumor() {
        use crate::first_session_guidance::{
            aftermath_after_people_landing, first_minutes_aftermath_line, garden_boot_want_line,
            garden_guidance_after_land, want_after_people_landing, HUMAN_AFTERMATH, SANCTUARY_WANT,
        };
        use shared::hex_travel::PlaceId;

        let (land, now) = l5_garden_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryYard));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(want_after_people_landing(land), SANCTUARY_WANT);
        assert_eq!(aftermath_after_people_landing(land), HUMAN_AFTERMATH);
        let spoken = garden_guidance_after_land(true, false, land).expect("after land");
        assert_eq!(spoken, first_minutes_aftermath_line(land));
        assert!(spoken.contains(SANCTUARY_WANT));
        assert!(spoken.contains("yard still teaching"));
        assert!(spoken.contains("war is rumor at the well"));
        assert!(garden_boot_want_line(true, false)
            .expect("boot Want unchanged")
            .contains(SANCTUARY_WANT));
    }

    /// CARD S1 — Ambrosian Title garden: Want stays Sanctuary; aftermath is
    /// same disk · thinner fog · no hull (Fork A). Differs from Human.
    #[test]
    fn s1_ambrosian_title_garden_aftermath_thinner_fog_no_hull() {
        use crate::first_session_guidance::{
            aftermath_after_people_landing, garden_guidance_after_land, want_after_people_landing,
            AMBROSIAN_AFTERMATH, HUMAN_AFTERMATH, SANCTUARY_WANT,
        };
        use shared::hex_travel::PlaceId;

        let (human, _) = l5_garden_land(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        let (ambrosian, now) =
            l5_garden_land(true, true, HousePeople::Ambrosian, PlaceId::Sanctuary, None);
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(want_after_people_landing(ambrosian), SANCTUARY_WANT);
        assert_eq!(
            want_after_people_landing(ambrosian),
            want_after_people_landing(human)
        );
        assert_eq!(aftermath_after_people_landing(ambrosian), AMBROSIAN_AFTERMATH);
        assert_ne!(
            aftermath_after_people_landing(ambrosian),
            aftermath_after_people_landing(human)
        );
        let spoken = garden_guidance_after_land(true, false, ambrosian).expect("after land");
        assert!(spoken.contains(SANCTUARY_WANT));
        assert!(spoken.contains("same disk"));
        assert!(spoken.contains("thinner fog"));
        assert!(spoken.contains("no hull"));
        assert!(!spoken.contains(HUMAN_AFTERMATH));
    }

    /// CARD S1 — Cydruid Title garden: human-in-frame · nature is practice.
    /// Want stays Heartwood. NOT treant.
    #[test]
    fn s1_cydruid_title_garden_aftermath_human_in_frame_nature_is_practice() {
        use crate::first_session_guidance::{
            aftermath_after_people_landing, garden_guidance_after_land, want_after_people_landing,
            CYDRUID_AFTERMATH, HEARTWOOD_WANT,
        };
        use shared::hex_travel::PlaceId;

        let (land, now) = l5_garden_land(true, true, HousePeople::Cydruid, PlaceId::Sanctuary, None);
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(want_after_people_landing(land), HEARTWOOD_WANT);
        assert_eq!(aftermath_after_people_landing(land), CYDRUID_AFTERMATH);
        let spoken = garden_guidance_after_land(true, false, land).expect("after land");
        assert!(spoken.contains(HEARTWOOD_WANT));
        assert!(spoken.contains("human-in-frame"));
        assert!(spoken.contains("nature is practice"));
        assert!(!spoken.contains("treant"));
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
    }

    /// CARD S1 — Quellorian Title garden: seam remembers the leaving.
    /// Want stays Heartwood. Aftermath differs from Cydruid.
    #[test]
    fn s1_quellorian_title_garden_aftermath_seam_remembers_the_leaving() {
        use crate::first_session_guidance::{
            aftermath_after_people_landing, garden_guidance_after_land, want_after_people_landing,
            CYDRUID_AFTERMATH, HEARTWOOD_WANT, QUELLORIAN_AFTERMATH, SANCTUARY_WANT,
        };
        use shared::hex_travel::PlaceId;

        let (cydruid, _) =
            l5_garden_land(true, true, HousePeople::Cydruid, PlaceId::Sanctuary, None);
        let (land, now) =
            l5_garden_land(true, true, HousePeople::Quellorian, PlaceId::Sanctuary, None);
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(want_after_people_landing(land), HEARTWOOD_WANT);
        assert_eq!(
            want_after_people_landing(land),
            want_after_people_landing(cydruid)
        );
        assert_eq!(aftermath_after_people_landing(land), QUELLORIAN_AFTERMATH);
        assert_ne!(
            aftermath_after_people_landing(land),
            aftermath_after_people_landing(cydruid)
        );
        let spoken = garden_guidance_after_land(true, false, land).expect("after land");
        assert!(spoken.contains("seam remembers the leaving"));
        assert!(!spoken.contains(CYDRUID_AFTERMATH));
        assert!(!spoken.contains(SANCTUARY_WANT));
    }

    /// CARD S1 — Draek Title garden: consume-scar · teal way-home.
    /// Want stays Depths restore, not Take.
    #[test]
    fn s1_draek_title_garden_aftermath_consume_scar_teal_way_home() {
        use crate::first_session_guidance::{
            aftermath_after_people_landing, garden_guidance_after_land, want_after_people_landing,
            DEPTHS_WANT, DRAEK_AFTERMATH,
        };
        use shared::hex_travel::PlaceId;

        let (land, now) = l5_garden_land(true, true, HousePeople::Draek, PlaceId::Sanctuary, None);
        assert_eq!(now, PlaceId::Depths);
        assert_eq!(want_after_people_landing(land), DEPTHS_WANT);
        assert_eq!(aftermath_after_people_landing(land), DRAEK_AFTERMATH);
        let spoken = garden_guidance_after_land(true, false, land).expect("after land");
        assert!(spoken.contains(DEPTHS_WANT));
        assert!(spoken.contains("consume-scar"));
        assert!(spoken.contains("teal way-home"));
        assert!(spoken.contains("restored"));
        assert!(!spoken.contains("Take"));
    }

    /// CARD S1 — TITLE_CHROME_PLAY / Continue / Settings unchanged.
    #[test]
    fn s1_title_chrome_play_continue_settings() {
        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(l2_title_chrome_holds());
        assert!(!title_has_race_portraits());
    }

    /// CARD S1 — STEWARD_ONLINE_YES stays false. Online grey.
    #[test]
    fn s1_steward_online_yes_stays_false() {
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
    }

    /// CARD S3 — Title chrome Play / Continue / Settings unchanged.
    #[test]
    fn s3_title_chrome_play_continue_settings() {
        assert_eq!(TITLE_CHROME_PLAY, "Play — first Hands");
        assert_eq!(TITLE_CHROME_CONTINUE, "Continue");
        assert_eq!(TITLE_CHROME_SETTINGS, "Settings");
        assert!(s3_title_chrome_holds());
        assert!(l2_title_chrome_holds());
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
    }

    /// CARD S3 — Play starts a new ungenerated light unsealed soul (garden / God-plane).
    #[test]
    fn s3_play_starts_light_unsealed_soul() {
        use crate::hour_sacred::{
            doors_are_unsealed, gate_seal_from_hour_two_json, merge_gate_seal_into_hour_two_json,
            soul_is_light,
        };
        use shared::hex_travel::PlaceId;

        let sealed = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        assert_eq!(
            gate_seal_from_hour_two_json(&sealed),
            Some((HousePeople::Cydruid, PeopleLanding::Heartwood))
        );

        let soul = play_new_light_soul();
        assert_eq!(soul, GardenRosterSoul::LightUnsealed);
        assert!(soul.is_light());
        assert!(soul.is_unsealed());
        assert!(soul_is_light(soul.sealed_pair()));
        assert!(doors_are_unsealed(soul.sealed_pair()));
        assert_eq!(soul.dress_line(), "light");
        assert!(soul.last_place().is_none());
        assert_eq!(soul.last_place_name(), "Garden");

        let mut travel = HexTravelState {
            current: PlaceId::Depths,
        };
        let mut bind = l5_demo_bind();
        let hour = HourSacred::default();
        let booted = s3_play_boot(&mut travel, &mut bind, &hour, None);
        assert!(booted.is_light());
        assert_eq!(travel.current, PlaceId::Sanctuary);
        assert_ne!(
            travel.current,
            PlaceId::Heartwood,
            "Play is a new light soul — does not resume the S2 seal"
        );
    }

    /// CARD S3 — Continue lists sealed souls with People dress + last Place.
    #[test]
    fn s3_continue_lists_sealed_souls_dress_last_place() {
        use crate::hour_sacred::merge_gate_seal_into_hour_two_json;
        use shared::hex_travel::PlaceId;

        assert!(s3_continue_roster_cue(None).is_none());
        assert!(s3_continue_roster_cue(Some("{}")).is_none());
        assert!(continue_sealed_souls_from_hour_two_json("{}").is_empty());

        let json = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        let list = continue_sealed_souls_from_hour_two_json(&json);
        assert_eq!(list.len(), 1);
        assert_eq!(s3_continue_roster_line(list[0]), "Cydruid · human-in-frame · Heartwood");
        assert_eq!(
            s3_continue_roster_cue(Some(&json)).as_deref(),
            Some("Cydruid · human-in-frame · Heartwood")
        );
        assert!(!list[0].is_light());
        assert_eq!(list[0].last_place(), Some(PlaceId::Heartwood));

        let mut travel = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        let mut bind = l5_demo_bind();
        let hour = HourSacred::default();
        assert!(!hour.hour_three_complete, "seal resume is not the book boot");
        let roster = s3_continue_boot(Some(&json), &mut travel, &mut bind, &hour, None);
        assert_eq!(roster.len(), 1);
        assert_eq!(travel.current, PlaceId::Heartwood);

        let human = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Human,
            PeopleLanding::SanctuaryYard,
        );
        assert_eq!(
            s3_continue_roster_cue(Some(&human)).as_deref(),
            Some("Human · Sanctuary")
        );
        let ambrosian = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Ambrosian,
            PeopleLanding::SanctuaryWellFromAbove,
        );
        assert_eq!(
            s3_continue_roster_cue(Some(&ambrosian)).as_deref(),
            Some("Ambrosian · Sanctuary")
        );
        let draek = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Draek,
            PeopleLanding::DepthsTealWayHome,
        );
        let mut travel = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        let roster = s3_continue_boot(Some(&draek), &mut travel, &mut bind, &hour, None);
        assert_eq!(s3_continue_roster_line(roster[0]), "Draek · Depths");
        assert_eq!(travel.current, PlaceId::Depths);
    }

    /// CARD S3 — no Title race/class lobby · no portraits grid.
    #[test]
    fn s3_no_race_portrait_lobby() {
        use crate::hour_sacred::merge_gate_seal_into_hour_two_json;

        assert!(!title_has_race_portraits());
        assert!(!title_has_race_class_lobby());
        assert!(!title_has_portraits_grid());
        assert!(!garden_roster_is_race_portrait_lobby());
        assert_ne!(TITLE_CHROME_PLAY, "Human");
        assert_ne!(TITLE_CHROME_CONTINUE, "Race");
        let play = play_new_light_soul();
        assert!(play.is_light(), "new slot is light, not a race picker");
        assert!(!play.dress_line().contains("portrait"));
        assert!(!play.dress_line().contains("class"));
        let empty = continue_sealed_souls_from_hour_two_json("{}");
        assert!(empty.is_empty());
        assert_ne!(empty.len(), HOUSE_PEOPLES.len());
        let json = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        let list = continue_sealed_souls_from_hour_two_json(&json);
        assert_eq!(list.len(), 1, "sealed list is souls, not five portraits");
        assert!(!list[0].dress_line().contains("Sanctuary tint"));
        assert!(!list[0].dress_line().contains("dress token"));
        assert!(!list[0].dress_line().contains("portrait"));
        assert!(s3_title_chrome_holds());
    }

    /// CARD S3 — STEWARD_ONLINE_YES stays false. Title Online grey.
    #[test]
    fn s3_steward_online_yes_stays_false() {
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
        assert!(!online_picker_ui_enabled(
            ONLINE_PICKER_ENABLED,
            STEWARD_ONLINE_YES
        ));
        assert!(s3_title_chrome_holds());
    }

    /// CARD S3 — PlaceId / LOCAL_HEXES len == 3. No fifth Place.
    #[test]
    fn s3_place_id_local_hexes_len_three() {
        use shared::hex_travel::PlaceId;

        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        assert!(four_place_landings_only());
        assert_eq!(PlaceId::Sanctuary.as_str(), "sanctuary");
        assert_eq!(PlaceId::Heartwood.as_str(), "heartwood");
        assert_eq!(PlaceId::Depths.as_str(), "depths");
        let light = play_new_light_soul();
        assert_eq!(light.last_place_name(), "Garden");
        assert!(light.last_place().is_none());
        for place in shared::hex_travel::LOCAL_HEXES {
            assert_ne!(place.display_name(), "Garden");
        }
    }

    /// CARD S3 — S2 seal keys still round-trip; Continue reads the same pair.
    #[test]
    fn s3_s2_seal_keys_still_round_trip() {
        use crate::hour_sacred::{
            gate_seal_from_hour_two_json, merge_gate_seal_into_hour_two_json,
            preserve_gate_seal_in_hour_two_json, SEALED_LANDING_KEY, SEALED_PEOPLE_KEY,
        };
        use shared::hex_travel::PlaceId;
        use shared::hour_two::HourTwoPack;
        use shared::space_law::HexFlag;

        let existing = r#"{"charter_id":"house-local","hex":"Frontier","kind":"House","warrant":{"h":0.0,"i":0.0,"c":0.0,"f":0.0,"x":0.0,"repair":0.0,"return_cargo":0.0,"council":0.0,"tend_spill":0.0}}"#;
        let json = merge_gate_seal_into_hour_two_json(
            existing,
            HousePeople::Draek,
            PeopleLanding::DepthsTealWayHome,
        );
        assert_eq!(
            gate_seal_from_hour_two_json(&json),
            Some((HousePeople::Draek, PeopleLanding::DepthsTealWayHome))
        );
        let loaded = HourTwoPack::from_json(&json);
        assert_eq!(loaded.session.charter_id.as_deref(), Some("house-local"));
        assert_eq!(loaded.session.hex, HexFlag::Frontier);
        assert!(json.contains(SEALED_PEOPLE_KEY));
        assert!(json.contains(SEALED_LANDING_KEY));
        let kept = preserve_gate_seal_in_hour_two_json(
            &json,
            r#"{"complete":true,"hour_three_complete":false}"#,
        );
        assert_eq!(
            gate_seal_from_hour_two_json(&kept),
            Some((HousePeople::Draek, PeopleLanding::DepthsTealWayHome))
        );
        let list = continue_sealed_souls_from_hour_two_json(&kept);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].dress_line(), "Draek");
        assert_eq!(list[0].last_place(), Some(PlaceId::Depths));
        assert_eq!(HOUR_TWO_PATH, "data/powrush_hour_two.json");
    }

    /// CARD F6 — Continue shows sealed souls in People dress @ last Place.
    #[test]
    fn f6_continue_shows_sealed_souls_people_dress_last_place() {
        use crate::hour_sacred::merge_gate_seal_into_hour_two_json;
        use shared::hex_travel::PlaceId;

        assert!(f6_continue_is_the_body_cue(None).is_none());
        assert!(f6_continue_is_the_body_cue(Some("{}")).is_none());
        assert!(f6_continue_bodies(Some("{}")).is_empty());
        assert!(f6_continue_body_line(play_new_light_soul()).is_none());

        let json = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        let list = f6_continue_bodies(Some(&json));
        assert_eq!(list.len(), 1);
        assert!(continue_is_the_body(list[0]));
        assert_eq!(
            f6_continue_body_line(list[0]).as_deref(),
            Some("Cydruid · human-in-frame · Heartwood")
        );
        assert_eq!(
            f6_continue_is_the_body_cue(Some(&json)).as_deref(),
            Some("Cydruid · human-in-frame · Heartwood")
        );
        assert!(!list[0].is_light());
        assert_eq!(list[0].last_place(), Some(PlaceId::Heartwood));

        let mut travel = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        let mut bind = l5_demo_bind();
        let hour = HourSacred::default();
        let roster = f6_continue_body_boot(Some(&json), &mut travel, &mut bind, &hour, None);
        assert_eq!(roster.len(), 1);
        assert!(continue_is_the_body(roster[0]));
        assert_eq!(travel.current, PlaceId::Heartwood);

        let human = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Human,
            PeopleLanding::SanctuaryYard,
        );
        assert_eq!(
            f6_continue_is_the_body_cue(Some(&human)).as_deref(),
            Some("Human · Sanctuary")
        );
        let draek = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Draek,
            PeopleLanding::DepthsTealWayHome,
        );
        let mut travel = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        let roster = f6_continue_body_boot(Some(&draek), &mut travel, &mut bind, &hour, None);
        assert_eq!(
            f6_continue_body_line(roster[0]).as_deref(),
            Some("Draek · Depths")
        );
        assert_eq!(travel.current, PlaceId::Depths);
    }

    /// CARD F6 — delete-soul / clear seal → that slot returns to light (Play).
    #[test]
    fn f6_delete_soul_clear_seal_slot_returns_light_play() {
        use crate::hour_sacred::{
            gate_seal_from_hour_two_json, merge_gate_seal_into_hour_two_json, SEALED_LANDING_KEY,
            SEALED_PEOPLE_KEY,
        };
        use shared::hex_travel::PlaceId;
        use shared::hour_two::HourTwoPack;

        let existing = r#"{"charter_id":"house-local","hex":"Frontier","kind":"House","warrant":{"h":0.0,"i":0.0,"c":0.0,"f":0.0,"x":0.0,"repair":0.0,"return_cargo":0.0,"council":0.0,"tend_spill":0.0}}"#;
        let json = merge_gate_seal_into_hour_two_json(
            existing,
            HousePeople::Ambrosian,
            PeopleLanding::SanctuaryWellFromAbove,
        );
        assert_eq!(
            f6_continue_is_the_body_cue(Some(&json)).as_deref(),
            Some("Ambrosian · Sanctuary")
        );

        let (cleared, slot) = f6_delete_soul_returns_play(&json);
        assert_eq!(slot, GardenRosterSoul::LightUnsealed);
        assert!(slot.is_light());
        assert!(slot.is_unsealed());
        assert_eq!(slot.dress_line(), "light");
        assert!(slot.last_place().is_none());
        assert_eq!(slot.last_place_name(), "Garden");
        assert!(!continue_is_the_body(slot));
        assert!(f6_continue_bodies(Some(&cleared)).is_empty());
        assert!(f6_continue_is_the_body_cue(Some(&cleared)).is_none());
        assert!(gate_seal_from_hour_two_json(&cleared).is_none());
        assert!(!cleared.contains(SEALED_PEOPLE_KEY));
        assert!(!cleared.contains(SEALED_LANDING_KEY));
        let loaded = HourTwoPack::from_json(&cleared);
        assert_eq!(loaded.session.charter_id.as_deref(), Some("house-local"));

        let mut travel = HexTravelState {
            current: PlaceId::Depths,
        };
        let mut bind = l5_demo_bind();
        let hour = HourSacred::default();
        let play = s3_play_boot(&mut travel, &mut bind, &hour, None);
        assert!(play.is_light());
        assert_eq!(travel.current, PlaceId::Sanctuary);
        assert_eq!(HOUR_TWO_PATH, "data/powrush_hour_two.json");
    }

    /// CARD F6 — no race-portrait lobby.
    #[test]
    fn f6_no_race_portrait_lobby() {
        use crate::hour_sacred::merge_gate_seal_into_hour_two_json;

        assert!(!title_has_race_portraits());
        assert!(!title_has_race_class_lobby());
        assert!(!title_has_portraits_grid());
        assert!(!garden_roster_is_race_portrait_lobby());
        assert!(!f6_title_is_race_lobby());
        assert_ne!(TITLE_CHROME_PLAY, "Human");
        assert_ne!(TITLE_CHROME_CONTINUE, "Race");
        let play = play_new_light_soul();
        assert!(play.is_light(), "new slot is light, not a race picker");
        assert!(f6_continue_body_line(play).is_none());
        assert!(!play.dress_line().contains("portrait"));
        assert!(!play.dress_line().contains("class"));
        let empty = f6_continue_bodies(Some("{}"));
        assert!(empty.is_empty());
        assert_ne!(empty.len(), HOUSE_PEOPLES.len());
        let json = merge_gate_seal_into_hour_two_json(
            "{}",
            HousePeople::Cydruid,
            PeopleLanding::Heartwood,
        );
        let list = f6_continue_bodies(Some(&json));
        assert_eq!(list.len(), 1, "sealed list is souls, not five portraits");
        assert!(!list[0].dress_line().contains("Sanctuary tint"));
        assert!(!list[0].dress_line().contains("dress token"));
        assert!(!list[0].dress_line().contains("portrait"));
        assert!(f6_title_chrome_holds());
    }

    /// CARD F6 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f6_place_id_local_hexes_len_three() {
        use shared::hex_travel::PlaceId;

        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        assert!(four_place_landings_only());
        assert_eq!(PlaceId::Sanctuary.as_str(), "sanctuary");
        assert_eq!(PlaceId::Heartwood.as_str(), "heartwood");
        assert_eq!(PlaceId::Depths.as_str(), "depths");
        let light = play_new_light_soul();
        assert_eq!(light.last_place_name(), "Garden");
        assert!(light.last_place().is_none());
        for place in shared::hex_travel::LOCAL_HEXES {
            assert_ne!(place.display_name(), "Garden");
            assert_ne!(place.as_str(), "market");
        }
        assert!(f6_title_chrome_holds());
    }

    /// CARD F6 — STEWARD_ONLINE_YES false.
    #[test]
    fn f6_steward_online_yes_false() {
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
        assert!(!online_picker_ui_enabled(
            ONLINE_PICKER_ENABLED,
            STEWARD_ONLINE_YES
        ));
        assert!(f6_title_chrome_holds());
    }

    /// CARD F7 — each People gets exactly one extra aftermath lore line
    /// on the existing S1 Title garden Want plate.
    #[test]
    fn f7_each_people_exactly_one_extra_aftermath_lore_line() {
        use crate::first_session_guidance::{
            aftermath_after_people_landing, f7_aftermath_variant_for_landing,
            f7_first_minutes_aftermath_line, first_minutes_aftermath_line, AMBROSIAN_AFTERMATH,
            AMBROSIAN_AFTERMATH_VARIANT, CYDRUID_AFTERMATH, CYDRUID_AFTERMATH_VARIANT,
            DRAEK_AFTERMATH, DRAEK_AFTERMATH_VARIANT, GARDEN_WANT, HEARTWOOD_WANT, HUMAN_AFTERMATH,
            HUMAN_AFTERMATH_VARIANT, QUELLORIAN_AFTERMATH, QUELLORIAN_AFTERMATH_VARIANT,
            SANCTUARY_WANT,
        };
        use shared::hex_travel::PlaceId;

        let boot = f7_title_garden_want_plate(false, None).expect("Garden boot");
        assert!(boot.contains(GARDEN_WANT));
        assert_eq!(boot, first_minutes_aftermath_line(None));
        assert!(f7_title_garden_want_plate(true, None).is_none());

        let pairs = [
            (
                HousePeople::Human,
                PeopleLanding::SanctuaryYard,
                HUMAN_AFTERMATH,
                HUMAN_AFTERMATH_VARIANT,
                SANCTUARY_WANT,
            ),
            (
                HousePeople::Ambrosian,
                PeopleLanding::SanctuaryWellFromAbove,
                AMBROSIAN_AFTERMATH,
                AMBROSIAN_AFTERMATH_VARIANT,
                SANCTUARY_WANT,
            ),
            (
                HousePeople::Cydruid,
                PeopleLanding::Heartwood,
                CYDRUID_AFTERMATH,
                CYDRUID_AFTERMATH_VARIANT,
                HEARTWOOD_WANT,
            ),
            (
                HousePeople::Quellorian,
                PeopleLanding::Threshold,
                QUELLORIAN_AFTERMATH,
                QUELLORIAN_AFTERMATH_VARIANT,
                HEARTWOOD_WANT,
            ),
            (
                HousePeople::Draek,
                PeopleLanding::DepthsTealWayHome,
                DRAEK_AFTERMATH,
                DRAEK_AFTERMATH_VARIANT,
                crate::first_session_guidance::DEPTHS_WANT,
            ),
        ];
        for (people, landing, s1, variant, want) in pairs {
            let (land, now) = l5_garden_land(true, true, people, PlaceId::Sanctuary, None);
            assert_eq!(land, Some(landing));
            assert_eq!(aftermath_after_people_landing(land), s1);
            assert_eq!(f7_aftermath_variant_for_landing(landing), variant);
            let s1_plate = first_minutes_aftermath_line(land);
            let spoken = f7_title_garden_want_plate(false, land).expect("after land");
            assert_eq!(spoken, f7_first_minutes_aftermath_line(land));
            assert_eq!(spoken, format!("{s1_plate} · {variant}"));
            assert!(spoken.contains(want));
            assert!(spoken.contains(s1));
            assert_eq!(spoken.matches(variant).count(), 1);
            assert!(!s1_plate.contains(variant));
            assert_ne!(now.as_str(), "market");
        }
        assert_eq!(HOUSE_PEOPLES.len(), 5);
        assert!(f7_title_chrome_holds());
    }

    /// CARD F7 — extra lines are local evidence, not a trailer / plot dump.
    #[test]
    fn f7_aftermath_variants_are_local_evidence_not_trailer() {
        use crate::first_session_guidance::{
            AMBROSIAN_AFTERMATH_VARIANT, CYDRUID_AFTERMATH_VARIANT, DRAEK_AFTERMATH_VARIANT,
            HUMAN_AFTERMATH_VARIANT, QUELLORIAN_AFTERMATH_VARIANT,
        };

        let variants = [
            HUMAN_AFTERMATH_VARIANT,
            AMBROSIAN_AFTERMATH_VARIANT,
            CYDRUID_AFTERMATH_VARIANT,
            QUELLORIAN_AFTERMATH_VARIANT,
            DRAEK_AFTERMATH_VARIANT,
        ];
        for variant in variants {
            let lower = variant.to_lowercase();
            assert!(variant.len() < 48, "{variant} is a manifesto");
            assert!(!lower.contains("trailer"));
            assert!(!lower.contains("cutscene"));
            assert!(!lower.contains("imagine"));
            assert!(!lower.contains("mothership"));
            assert!(!lower.contains("crownstone"));
            assert!(!lower.contains("plot dump"));
            assert!(!variant.contains("Market"));
            assert!(!variant.contains(".glb"));
        }
        assert!(HUMAN_AFTERMATH_VARIANT.contains("well"));
        assert!(AMBROSIAN_AFTERMATH_VARIANT.contains("prism"));
        assert!(CYDRUID_AFTERMATH_VARIANT.contains("amber lamp"));
        assert!(!CYDRUID_AFTERMATH_VARIANT.contains("treant"));
        assert!(QUELLORIAN_AFTERMATH_VARIANT.contains("tend seam"));
        assert!(DRAEK_AFTERMATH_VARIANT.contains("teal Peace"));
        assert!(!DRAEK_AFTERMATH_VARIANT.contains("Take"));
        let hush = f7_title_garden_want_plate(true, Some(PeopleLanding::SanctuaryYard));
        assert!(hush.is_none());
    }

    /// CARD F7 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f7_place_id_local_hexes_len_three() {
        use shared::hex_travel::PlaceId;

        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
        assert!(four_place_landings_only());
        assert_eq!(PlaceId::Sanctuary.as_str(), "sanctuary");
        assert_eq!(PlaceId::Heartwood.as_str(), "heartwood");
        assert_eq!(PlaceId::Depths.as_str(), "depths");
        let light = play_new_light_soul();
        assert_eq!(light.last_place_name(), "Garden");
        assert!(light.last_place().is_none());
        for place in shared::hex_travel::LOCAL_HEXES {
            assert_ne!(place.display_name(), "Garden");
            assert_ne!(place.as_str(), "market");
        }
        assert!(f7_title_chrome_holds());
    }

    /// CARD F7 — STEWARD_ONLINE_YES false.
    #[test]
    fn f7_steward_online_yes_false() {
        assert!(!STEWARD_ONLINE_YES);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
        assert!(!online_picker_ui_enabled(
            ONLINE_PICKER_ENABLED,
            STEWARD_ONLINE_YES
        ));
        assert!(f7_title_chrome_holds());
    }

    /// CARD F7 — no mesh / glb adds. Want plate stays words.
    #[test]
    fn f7_no_mesh_glb_adds() {
        use crate::first_session_guidance::{
            f7_first_minutes_aftermath_line, AMBROSIAN_AFTERMATH_VARIANT,
            CYDRUID_AFTERMATH_VARIANT, DRAEK_AFTERMATH_VARIANT, HUMAN_AFTERMATH_VARIANT,
            QUELLORIAN_AFTERMATH_VARIANT,
        };

        assert_eq!(L2_MESH_BUDGET, 0);
        assert_eq!(L2_ASSET_BUDGET_CITE, "docs/ASSET_BUDGET_COURT.md");
        assert!(f7_title_chrome_holds());
        for variant in [
            HUMAN_AFTERMATH_VARIANT,
            AMBROSIAN_AFTERMATH_VARIANT,
            CYDRUID_AFTERMATH_VARIANT,
            QUELLORIAN_AFTERMATH_VARIANT,
            DRAEK_AFTERMATH_VARIANT,
        ] {
            assert!(!variant.contains(".glb"));
            assert!(!variant.contains(".gltf"));
            assert!(!variant.to_lowercase().contains("mesh"));
        }
        let plate = f7_first_minutes_aftermath_line(Some(PeopleLanding::Heartwood));
        assert!(!plate.contains(".glb"));
        assert!(!plate.to_lowercase().contains("mesh"));
    }

    #[test]
    fn title_contrast_light_on_opaque_dark() {
        assert!(
            title_contrast_is_high(),
            "primary text must out-luminance plate"
        );
        let a = title_alpha(TITLE_PLATE_BG);
        assert!(
            (a - 1.0).abs() < 0.01,
            "plate must be opaque, got alpha={a}"
        );
        let a2 = title_alpha(TITLE_DIM_BG);
        assert!(
            (a2 - 1.0).abs() < 0.01,
            "dimmer must be opaque, got alpha={a2}"
        );
        assert!(title_luminance(TITLE_TEXT_PRIMARY) > title_luminance(TITLE_PLATE_BG));
        assert!(title_luminance(TITLE_BTN_FG) > title_luminance(TITLE_BTN_BG));
    }

    #[test]
    fn u5_deck_title_plate_is_inside_surface_and_above_world() {
        let surface = Vec2::new(DECK_TITLE_WIDTH, DECK_TITLE_HEIGHT);
        let available = surface - Vec2::splat(TITLE_SAFE_INSET * 2.0);
        assert!(title_plate_fits_surface(
            surface,
            Vec2::new(TITLE_PLATE_MAX_WIDTH, available.y),
            TITLE_SAFE_INSET,
        ));
        assert!(crate::ui_above_world::ui_camera_draws_above_world());

        let mut app = App::new();
        app.add_systems(Startup, spawn_title_screen);
        app.update();

        let world = app.world_mut();
        let root = world
            .query_filtered::<(&Style, &FocusPolicy), With<TitleRoot>>()
            .single(world);
        assert_eq!(root.0.padding, UiRect::all(Val::Px(TITLE_SAFE_INSET)));
        assert_eq!(*root.1, FocusPolicy::Block);

        let plate = world
            .query_filtered::<&Style, With<TitlePlate>>()
            .single(world);
        assert_eq!(plate.width, Val::Percent(100.0));
        assert_eq!(plate.max_width, Val::Px(TITLE_PLATE_MAX_WIDTH));
        assert_eq!(plate.max_height, Val::Percent(100.0));
    }

    #[test]
    fn esc_from_inyard_opens_pause_not_title() {
        // Esc with pause closed → open pause; door stays InYard (not Title).
        assert_eq!(
            super::esc_from_inyard_toggles_pause(LaunchDoor::InYard, false),
            (LaunchDoor::InYard, true)
        );
        // Esc with pause open → close pause (Resume); still InYard.
        assert_eq!(
            super::esc_from_inyard_toggles_pause(LaunchDoor::InYard, true),
            (LaunchDoor::InYard, false)
        );
        // Other doors unchanged.
        assert_eq!(
            super::esc_from_inyard_toggles_pause(LaunchDoor::Title, false),
            (LaunchDoor::Title, false)
        );
        assert_eq!(
            super::esc_from_inyard_toggles_pause(LaunchDoor::NameHouse, true),
            (LaunchDoor::NameHouse, true)
        );
        assert_eq!(
            super::esc_from_inyard_toggles_pause(LaunchDoor::HouseDress, false),
            (LaunchDoor::HouseDress, false)
        );
    }

    #[test]
    fn yard_pause_verb_is_the_same_on_every_local_hex() {
        // The verb reads the door, never the hex — one plate for all of them.
        assert_eq!(
            super::yard_pause_step(LaunchDoor::InYard, false, false),
            Some(YardPause {
                pause_open: true,
                places_open: false
            })
        );
        assert_eq!(
            super::yard_pause_step(LaunchDoor::InYard, true, false),
            Some(YardPause {
                pause_open: false,
                places_open: false
            })
        );
        // Places is a leaf of the pause plate: the press walks back, never eats.
        assert_eq!(
            super::yard_pause_step(LaunchDoor::InYard, true, true),
            Some(YardPause {
                pause_open: true,
                places_open: false
            })
        );
        assert_eq!(
            super::yard_pause_step(LaunchDoor::InYard, false, true),
            Some(YardPause {
                pause_open: true,
                places_open: false
            })
        );
        // Off the yard the verb is silent — Title / naming / dress keep their keys.
        assert_eq!(
            super::yard_pause_step(LaunchDoor::Title, false, false),
            None
        );
        assert_eq!(
            super::yard_pause_step(LaunchDoor::NameHouse, true, false),
            None
        );
        assert_eq!(
            super::yard_pause_step(LaunchDoor::HouseDress, false, true),
            None
        );
    }

    #[test]
    fn confirmed_leave_lands_on_a_bare_yard() {
        let landed = super::yard_after_travel();
        assert!(!landed.pause_open, "no plate carried into the new hex");
        assert!(!landed.places_open);
        // So the next press on the new hex opens pause instead of closing.
        assert_eq!(
            super::yard_pause_step(LaunchDoor::InYard, landed.pause_open, landed.places_open),
            Some(YardPause {
                pause_open: true,
                places_open: false
            })
        );
    }

    #[test]
    fn d1_pause_plate_yard_waiting_line() {
        assert_eq!(
            super::pause_plate_line(LaunchDoor::InYard),
            Some(YARD_WAITING)
        );
        assert_eq!(super::pause_plate_line(LaunchDoor::Title), None);
        assert_eq!(super::pause_plate_line(LaunchDoor::NameHouse), None);
        assert_eq!(super::pause_plate_line(LaunchDoor::HouseDress), None);
        assert_eq!(YARD_WAITING, "the yard is waiting");
    }

    #[test]
    fn pause_tab_defaults_to_comfort() {
        assert_eq!(PauseTab::default(), PauseTab::Comfort);
        assert_eq!(PauseTab::Comfort.label(), "Comfort");
        assert_eq!(PauseTab::Controls.label(), "Controls");
        assert_eq!(PauseTab::Guide.label(), "Guide");
        assert_eq!(PauseTab::ALL.len(), 3);
    }

    #[test]
    fn pause_guide_breathes_peak_memory_one_sentence() {
        let line = PAUSE_GUIDE_LINE;
        let lower = line.to_ascii_lowercase();
        // Locked peak memory: well · tend · week · the yard remembered.
        assert!(lower.contains("well"));
        assert!(lower.contains("tend"));
        assert!(lower.contains("week"));
        assert!(lower.contains("yard remembered"));
        // One readable sentence — not five stacked Guide lines.
        assert!(!line.contains('\n'));
        assert_eq!(line.chars().filter(|c| *c == '.').count(), 1);
        assert!(line.ends_with('.'));
        assert_eq!(
            line,
            "I walked to a well, tended it, the week was the bill, I quit, and the yard remembered."
        );
        // No new chrome on this card.
        assert!(!lower.contains("online"));
        assert!(!lower.contains("market"));
        assert!(!lower.contains("socket"));
        assert!(!lower.contains("ultra"));
        assert!(!lower.contains("0.0.0.0"));
        assert!(!lower.contains("f1"));
        assert!(!lower.contains("f2"));
        assert!(!lower.contains("f-row"));
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(ONLINE_STUB_LABEL, "Online — off (no listen)");
        assert_eq!(PauseTab::ALL.len(), 3, "Comfort · Controls · Guide preserved");
    }

    #[test]
    fn pause_plate_spawns_places_door_before_online() {
        // Places door is on the tabbed pause plate (not a floating chip).
        assert_eq!(PLACES_ROW, "Places");
        assert_eq!(PauseTab::ALL.len(), 3, "Comfort · Controls · Guide preserved");
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

    #[test]
    fn d1_resume_keeps_inyard() {
        assert_eq!(
            super::resume_keeps_door(LaunchDoor::InYard),
            LaunchDoor::InYard
        );
        // Esc close-pause path is Resume-equivalent (stays InYard).
        assert_eq!(
            super::esc_from_inyard_toggles_pause(LaunchDoor::InYard, true),
            (LaunchDoor::InYard, false)
        );
    }

    #[test]
    fn title_from_pause_maps_to_title() {
        assert_eq!(
            super::title_from_pause_returns_title(LaunchDoor::InYard),
            LaunchDoor::Title
        );
        assert_eq!(
            super::title_from_pause_returns_title(LaunchDoor::Title),
            LaunchDoor::Title
        );
        assert_eq!(
            super::title_from_pause_returns_title(LaunchDoor::NameHouse),
            LaunchDoor::NameHouse
        );
    }

    #[test]
    fn ensure_house_written_on_skip_path() {
        // In-memory skip path (no disk write in unit test).
        let mut label = HouseLabel {
            house: HouseName::default(),
            persist_present: false,
            hour_two_held: false,
            book_held: false,
            settings_open: false,
            draft: String::new(),
            naming_offered: false,
            seals_offered: false,
        };
        assert!(!label.house.resolved);
        if !label.house.resolved {
            label.house.skip();
            label.naming_offered = true;
        }
        label.persist_present = true;
        assert!(label.house.resolved);
        assert_eq!(label.house.display_name(), UNNAMED);
        assert!(label.persist_present);
        assert!(label.naming_offered);
        let raw = label.house.to_json().unwrap();
        assert!(raw.contains("powrush_house_v1"));
        // Shared persist beat stays present tense. The plate speaks the Guide peak.
        let cue = continue_cue_when_persist(true, &label.house).unwrap();
        assert_eq!(cue, "Unnamed House · the yard remembers");
        assert_eq!(
            continue_yard_remembered_line(&cue),
            "Unnamed House · the yard remembered"
        );
        assert!(continue_yard_remembered_line(&cue).contains("yard remembered"));
        // Title-from-pause helper still maps InYard → Title (house JSON path).
        assert_eq!(
            super::title_from_pause_returns_title(LaunchDoor::InYard),
            LaunchDoor::Title
        );
        // Esc itself only toggles pause — never Title.
        assert_eq!(
            super::esc_from_inyard_toggles_pause(LaunchDoor::InYard, false),
            (LaunchDoor::InYard, true)
        );
    }

    #[test]
    fn d2_local_settings_defaults_and_labels() {
        let s = LocalSettings::peace_defaults();
        assert!(!s.mute && !s.invert_y && !s.hide_slabs);
        assert!((s.brightness - 1.0).abs() < f32::EPSILON);
        assert!((s.text_scale - 1.0).abs() < f32::EPSILON);
        assert_eq!(look_btn_label(&s), "Look · 1.00");
        assert_eq!(mute_btn_label(&s), "Mute · off");
        assert_eq!(invert_btn_label(&s), "Invert-Y · off");
        assert_eq!(hide_slabs_btn_label(&s), "Hide slabs · off");
        assert_eq!(brightness_btn_label(&s), "Brightness · 1.00");
        assert_eq!(text_scale_btn_label(&s), "Text scale · 1.00");
        assert_eq!(graphics_preset_btn_label(&s), "Graphics · Medium");
        assert_eq!(s.graphics_preset, GraphicsPreset::Medium);
        assert_eq!(grove_btn_label(&s), "Grove · off");
        assert_eq!(s.grove, "off");
        assert_eq!(reduced_motion_btn_label(&s), "Reduced motion · off");
        assert_eq!(rumble_btn_label(&s), "Rumble · on");
        assert_eq!(colorblind_wells_btn_label(&s), "Colorblind wells · off");
        assert_eq!(lan_btn_label(&s), "LAN · off");
        assert_eq!(s.lan, "off");
        assert_eq!(SETTINGS_PATH, "data/powrush_settings.json");
    }

    #[test]
    fn b4_settings_rows_show_current_peace_bindings() {
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(peace_binding_label(PeaceAction::MoveUp, &s), "Move up · W");
        assert_eq!(peace_binding_label(PeaceAction::Jump, &s), "Jump · Space");
        assert_eq!(
            peace_binding_label(PeaceAction::Sprint, &s),
            "Sprint · either Shift"
        );
        assert_eq!(peace_binding_label(PeaceAction::Use, &s), "Use · E");

        s.key_move_up = PeaceKey::ArrowUp;
        s.key_use = PeaceKey::F;
        assert_eq!(peace_binding_label(PeaceAction::MoveUp, &s), "Move up · Up");
        assert_eq!(peace_binding_label(PeaceAction::Use, &s), "Use · F");
    }

    #[test]
    fn b4_settings_rebind_rejects_a_taken_peace_key() {
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(
            try_peace_rebind(&mut s, PeaceAction::Use, PeaceKey::W),
            Err(PeaceAction::MoveUp)
        );
        assert_eq!(s.key_use, PeaceKey::E);
        assert_eq!(
            try_peace_rebind(&mut s, PeaceAction::Use, PeaceKey::RightShift),
            Err(PeaceAction::Sprint),
            "default sprint claims either Shift"
        );

        assert_eq!(
            try_peace_rebind(&mut s, PeaceAction::Use, PeaceKey::F),
            Ok(())
        );
        assert_eq!(s.key_use, PeaceKey::F);
        let round_trip = LocalSettings::from_json(&s.to_json().unwrap()).unwrap();
        assert_eq!(round_trip.key_use, PeaceKey::F);
    }

    #[test]
    fn b4_reset_to_peace_restores_all_bindings_only() {
        let mut s = LocalSettings::peace_defaults();
        s.grove = "light".into();
        s.key_move_up = PeaceKey::ArrowUp;
        s.key_move_down = PeaceKey::ArrowDown;
        s.key_move_left = PeaceKey::ArrowLeft;
        s.key_move_right = PeaceKey::ArrowRight;
        s.key_jump = PeaceKey::J;
        s.key_sprint = PeaceKey::RightControl;
        s.key_use = PeaceKey::F;
        s.key_satchel = PeaceKey::B;
        s.key_hide = PeaceKey::V;
        s.key_allocate = PeaceKey::N;

        reset_peace_bindings(&mut s);

        assert!(s.peace_keys_are_default());
        assert_eq!(s.grove, "light");
        assert_eq!(
            PEACE_ACTIONS.map(|action| action.key(&s)),
            [
                PeaceKey::W,
                PeaceKey::S,
                PeaceKey::A,
                PeaceKey::D,
                PeaceKey::Space,
                PeaceKey::LeftShift,
                PeaceKey::E,
                PeaceKey::I,
                PeaceKey::H,
                PeaceKey::R,
            ]
        );
    }

    #[test]
    fn b4_supported_physical_keys_map_to_persisted_keys() {
        assert_eq!(peace_key_from_key_code(KeyCode::KeyQ), Some(PeaceKey::Q));
        assert_eq!(
            peace_key_from_key_code(KeyCode::ShiftRight),
            Some(PeaceKey::RightShift)
        );
        assert_eq!(
            peace_key_from_key_code(KeyCode::ControlLeft),
            Some(PeaceKey::LeftControl)
        );
        assert_eq!(peace_key_from_key_code(KeyCode::F1), None);
    }

    #[test]
    fn d2_refuse_online_socket_from_settings_plate() {
        assert!(refuse_online_socket_toggle(true));
        assert!(!refuse_online_socket_toggle(false));
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

    #[test]
    fn d2_settings_round_trip_labels_follow_state() {
        let mut s = LocalSettings::default();
        s.mute = true;
        s.invert_y = true;
        s.hide_slabs = true;
        s.look_sensitivity = 1.50;
        s.brightness = 1.25;
        s.text_scale = 1.10;
        s.graphics_preset = GraphicsPreset::High;
        s.grove = "light".into();
        s.reduced_motion = true;
        s.rumble = false;
        s.colorblind_wells = "shape_only".into();
        s.lan = "loopback".into();
        let raw = s.to_json().unwrap();
        let back = LocalSettings::from_json(&raw).unwrap();
        assert_eq!(mute_btn_label(&back), "Mute · on");
        assert_eq!(invert_btn_label(&back), "Invert-Y · on");
        assert_eq!(hide_slabs_btn_label(&back), "Hide slabs · on");
        assert_eq!(look_btn_label(&back), "Look · 1.50");
        assert_eq!(brightness_btn_label(&back), "Brightness · 1.25");
        assert_eq!(text_scale_btn_label(&back), "Text scale · 1.10");
        assert_eq!(graphics_preset_btn_label(&back), "Graphics · High");
        assert_eq!(grove_btn_label(&back), "Grove · light");
        assert_eq!(reduced_motion_btn_label(&back), "Reduced motion · on");
        assert_eq!(rumble_btn_label(&back), "Rumble · off");
        assert_eq!(colorblind_wells_btn_label(&back), "Colorblind wells · shape_only");
        assert_eq!(lan_btn_label(&back), "LAN · loopback");
    }

    #[test]
    fn g05_grove_cycles_off_light() {
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(grove_btn_label(&s), "Grove · off");
        s.cycle_grove();
        assert_eq!(grove_btn_label(&s), "Grove · light");
        s.cycle_grove();
        assert_eq!(grove_btn_label(&s), "Grove · off");
    }

    #[test]
    fn comfort_graphics_presets_cycle_labels_medium_default() {
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(s.graphics_preset, GraphicsPreset::Medium);
        assert_eq!(graphics_preset_btn_label(&s), "Graphics · Medium");
        s.cycle_graphics_preset();
        assert_eq!(graphics_preset_btn_label(&s), "Graphics · High");
        assert!((s.brightness - 1.25).abs() < 0.01);
        assert!(!s.reduced_motion);
        s.cycle_graphics_preset();
        assert_eq!(graphics_preset_btn_label(&s), "Graphics · Low");
        assert!(s.reduced_motion);
        assert!(!s.rumble_enabled());
        s.cycle_graphics_preset();
        assert_eq!(graphics_preset_btn_label(&s), "Graphics · Medium");
        // Online stays grey — presets never bind a socket.
        assert!(refuse_online_socket_toggle(true));
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        let raw = s.to_json().unwrap();
        let back = LocalSettings::from_json(&raw).unwrap();
        assert_eq!(graphics_preset_btn_label(&back), "Graphics · Medium");
    }

    #[test]
    fn mesh_lod_banner_copy_and_once_dismiss() {
        assert_eq!(
            COMFORT_GRAPHICS_BANNER_COPY,
            "Graphics can go Higher on this machine — Esc → Comfort"
        );
        assert!(!COMFORT_GRAPHICS_BANNER_COPY.contains("assets/"));
        assert!(!COMFORT_GRAPHICS_BANNER_COPY.contains(".glb"));

        let mut s = LocalSettings::peace_defaults();
        assert!(s.should_show_comfort_graphics_banner());
        assert!(comfort_graphics_banner_should_show(true, false, false, false));
        assert!(comfort_graphics_banner_should_show(false, true, false, false));
        assert!(!comfort_graphics_banner_should_show(false, false, false, false));
        assert!(!comfort_graphics_banner_should_show(true, true, true, false));

        s.dismiss_comfort_graphics_banner();
        assert!(!s.should_show_comfort_graphics_banner());
        assert!(!comfort_graphics_banner_should_show(
            true,
            true,
            s.comfort_graphics_banner_dismissed,
            false
        ));
        // Online stays grey — banner never lights a socket.
        assert!(refuse_online_socket_toggle(true));
    }

    #[test]
    fn places_open_clears_comfort_overlay_banner() {
        // Pause stays armed (settings_open) while Places door is open — Comfort
        // graphics banner must not linger over the four-room Places plate.
        assert!(
            !comfort_graphics_banner_should_show(false, true, false, true),
            "Places open ⇒ Comfort banner off"
        );
        assert!(
            comfort_graphics_banner_should_show(false, true, false, false),
            "Esc Comfort with Places closed still shows banner"
        );
        assert!(
            !settings_visible_with_places(true, true),
            "Comfort pause plate also hides under Places"
        );
        // Pause armed + Places closed ⇒ Comfort chrome may show; Places open ⇒ hide.
        assert!(settings_visible_with_places(true, false));
        assert!(!settings_visible_with_places(false, true));
        assert_eq!(PLACES_ROW, "Places");
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(refuse_online_socket_toggle(true));
    }

    #[test]
    fn mesh_lod_plan_from_comfort_graphics_preset() {
        use gltf_integration::{mesh_lod_for_preset, plan_for_preset, MeshLod};
        assert_eq!(mesh_lod_for_preset(GraphicsPreset::Medium), MeshLod::Medium);
        let low = plan_for_preset(GraphicsPreset::Low);
        assert!(low.primitives_only);
        let high = plan_for_preset(GraphicsPreset::High);
        assert!(high.persona_commit_dress);
        assert!(refuse_online_socket_toggle(true));
    }

    #[test]
    fn b3_colorblind_wells_cycles_beside_grove_online_grey() {
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(colorblind_wells_btn_label(&s), "Colorblind wells · off");
        assert!(!s.colorblind_wells_shapes());
        s.cycle_colorblind_wells();
        assert_eq!(colorblind_wells_btn_label(&s), "Colorblind wells · deuteranopia");
        assert!(s.colorblind_wells_shapes());
        s.cycle_colorblind_wells();
        assert_eq!(colorblind_wells_btn_label(&s), "Colorblind wells · protanopia");
        s.cycle_colorblind_wells();
        assert_eq!(colorblind_wells_btn_label(&s), "Colorblind wells · tritanopia");
        s.cycle_colorblind_wells();
        assert_eq!(colorblind_wells_btn_label(&s), "Colorblind wells · shape_only");
        s.cycle_colorblind_wells();
        assert_eq!(colorblind_wells_btn_label(&s), "Colorblind wells · off");
        // Grove independent; Online stub still hard-refuses.
        assert!(!s.grove_is_light());
        assert!(refuse_online_socket_toggle(true));
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

    #[test]
    fn p3_lan_row_cycles_off_loopback_online_stays_grey() {
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(lan_btn_label(&s), "LAN · off");
        assert!(!s.lan_is_loopback());
        s.cycle_lan();
        assert_eq!(lan_btn_label(&s), "LAN · loopback");
        assert!(s.lan_is_loopback());
        // Grove stays default off. Online stub still hard-refuses. Title does not bind.
        assert!(!s.grove_is_light());
        assert!(refuse_online_socket_toggle(true));
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        s.cycle_lan();
        assert_eq!(lan_btn_label(&s), "LAN · off");
    }

    #[test]
    fn l1_hex_sign_row_default_off_online_grey_lan_untouched() {
        use shared::pause_ledger_face::{
            HEX_ADMITS_HARM, HEX_ADMITS_HARM_OFF, LEDGER_WAITS, NOT_YOUR_CHARTER,
        };
        use shared::shard_climate::ShardClimate;
        use shared::shard_standing::ShardStanding;
        let s = LocalSettings::peace_defaults();
        assert_eq!(lan_btn_label(&s), "LAN · off");
        assert!(!s.lan_is_loopback());
        assert_eq!(
            lethal_sign_btn_label(false, false, false, false),
            NOT_YOUR_CHARTER
        );
        assert_eq!(
            lethal_sign_btn_label(true, false, true, false),
            LEDGER_WAITS
        );
        assert_eq!(
            lethal_sign_btn_label(true, true, true, false),
            HEX_ADMITS_HARM_OFF
        );
        assert_eq!(
            lethal_sign_btn_label(true, true, true, true),
            HEX_ADMITS_HARM
        );
        let (pack, _c, standing, _w, _h) = shared::f_book_fixture::load_f_book_disk();
        assert!(pack.complete && pack.hour_three_complete);
        assert!(!standing.declared_lethal);
        assert_eq!(
            lethal_sign_btn_label(true, true, true, standing.declared_lethal),
            HEX_ADMITS_HARM_OFF
        );
        let mut standing = ShardStanding::default();
        let mut climate = ShardClimate::default();
        climate.tons_moved = 2;
        assert!(!standing.confirm_hex_sign(false, false));
        assert!(!standing.declared_lethal);
        assert!(standing.confirm_hex_sign(true, true));
        let _ = climate.on_lethal_declare();
        assert_eq!(climate.tons_moved, 2);
        assert!(standing.declared_lethal);
        // Confirm is not Title Online and does not flip LAN.
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(refuse_online_socket_toggle(true));
        assert_eq!(s.lan, "off");
        assert_eq!(soft_play_bindings_e_is_use(), true);
    }

    #[test]
    fn u2_play_and_no_book_continue_stay_sanctuary_places_hidden() {
        use shared::hex_travel::{boot_place, places_row_label, BootKind, PlaceId};
        assert_eq!(
            boot_place(BootKind::Play, true, Some(PlaceId::Heartwood)),
            PlaceId::Sanctuary
        );
        assert_eq!(
            boot_place(BootKind::Continue, false, Some(PlaceId::Heartwood)),
            PlaceId::Sanctuary
        );
        assert_eq!(places_row_label(false, false), None);
        assert!(settings_visible_with_places(true, false));
        assert!(!settings_visible_with_places(true, true));
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert_eq!(soft_play_bindings_e_is_use(), true);
    }

    fn soft_play_bindings_e_is_use() -> bool {
        crate::soft_play_bindings::INTERACT == bevy::prelude::KeyCode::KeyE
    }

    #[test]
    fn after_d3_mute_from_pause_is_master_mute() {
        // Pause plate Mute · uses LocalSettings.mute → MasterMuteGain (not a second audio system).
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(mute_btn_label(&s), "Mute · off");
        assert!((s.master_gain() - 1.0).abs() < f32::EPSILON);
        s.toggle_mute();
        assert_eq!(mute_btn_label(&s), "Mute · on");
        assert!((s.master_gain() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn after_d3_title_contrast_unchanged_with_brightness() {
        // Brightness/text_scale persist must not regress opaque Title plate law.
        assert!(title_contrast_is_high());
        assert!((title_alpha(TITLE_PLATE_BG) - 1.0).abs() < 0.01);
        let mut s = LocalSettings::default();
        s.brightness = 1.50;
        s.text_scale = 1.35;
        let _ = s; // settings do not mutate TITLE_* constants
        assert!(title_contrast_is_high());
    }

    #[test]
    fn after_d3_q_plate_shows_seal_when_dressed() {
        let mut house = HouseName::default();
        house.confirm("Ridge");
        house.set_seals(&[SEAL_GROVE]);
        house.confirm_seals();
        house.set_heritage("draek");
        let line = house.dress_line_for_plate().unwrap();
        assert_eq!(line, "Seal · Grove · draek");
        assert!(!house.heritage_grants_stats());
        assert!(!house.seals_grant_combat());
    }

    #[test]
    fn d3_seals_skippable_after_settled() {
        let mut house = HouseName::default();
        house.skip(); // Settled / skip-named
        assert!(house.resolved);
        assert!(!house.seals_resolved);
        house.skip_seals();
        assert!(house.seals_resolved);
        assert!(house.seals.is_empty());
        assert!(!house.seals_grant_combat());
        // Choose Peace-tone seals only
        let mut h2 = HouseName::default();
        h2.confirm("Grove House");
        for id in HOUSE_SEALS {
            assert!(h2.toggle_seal(id));
        }
        assert_eq!(h2.seals.len(), 3);
        h2.confirm_seals();
        assert!(h2.seals_resolved);
        let raw = h2.to_json().unwrap();
        assert!(raw.contains("well") && raw.contains("grove") && raw.contains("ember"));
        assert!(!raw.contains("+STR") && !raw.contains("+take"));
    }

    #[test]
    fn d3_heritage_string_only_no_stats() {
        let mut house = HouseName::default();
        house.skip();
        assert_eq!(heritage_btn_label(&house), "Heritage · none");
        assert!(house.set_heritage("human"));
        assert_eq!(heritage_btn_label(&house), "Heritage · human");
        assert!(house.set_heritage("ambrosian"));
        assert!(!house.heritage_grants_stats());
        assert!(!house.set_heritage("+STR"));
        let raw = house.to_json().unwrap();
        assert!(raw.contains("heritage"));
        assert!(!raw.contains("str_bonus") && !raw.contains("+take"));
    }

    #[test]
    fn d3_rename_ok_keeps_seals_heritage() {
        let mut house = HouseName::default();
        house.confirm("Old");
        house.set_seals(&[SEAL_WELL, SEAL_GROVE]);
        house.confirm_seals();
        house.set_heritage("quellorian");
        house.rename("New Ridge");
        assert_eq!(house.display_name(), "New Ridge");
        assert_eq!(house.seals.len(), 2);
        assert_eq!(house.heritage, "quellorian");
        assert!(house.seals_resolved);
    }

    #[test]
    fn d3_refuse_combat_mods() {
        assert!(HouseName::refuse_combat_mod("+take"));
        assert!(HouseName::refuse_combat_mod("+STR"));
        let house = HouseName::default();
        assert!(house.apply_combat_mod("+take").is_err());
        assert!(house.apply_combat_mod("+STR").is_err());
        assert!(!house.seals_grant_combat());
        assert!(!house.heritage_grants_stats());
        // advance_after_naming offers dress when seals pending
        let mut label = HouseLabel {
            house: HouseName::default(),
            persist_present: true,
            hour_two_held: false,
            book_held: false,
            settings_open: false,
            draft: String::new(),
            naming_offered: true,
            seals_offered: false,
        };
        label.house.skip();
        assert_eq!(
            super::advance_after_naming(&mut label),
            LaunchDoor::HouseDress
        );
        assert!(label.seals_offered);
        label.house.confirm_seals();
        assert_eq!(super::advance_after_naming(&mut label), LaunchDoor::InYard);
    }

    // --- MERCY_PERSONA P2 ----------------------------------------------------

    #[test]
    fn mercy_persona_p2_flag_on_lights_optional_creator() {
        assert!(PERSONA_CREATOR_ENABLED);
        assert!(persona_creator_may_open(PERSONA_CREATOR_ENABLED));
        let mut state = PersonaCreatorState::default();
        assert!(try_open_persona_creator(&mut state, PERSONA_CREATOR_ENABLED));
        assert!(state.open);
        let p = Persona::nameless_steward();
        assert!(p.presentation.given_name.is_empty());
        assert!(matches!(p.presentation.people, PeopleChoice::Unset));
        assert_eq!(LaunchDoor::default(), LaunchDoor::Title);
        // Play path does not require persona plate; Title Persona lights optional.
        assert_eq!(
            persona_title_btn_label(PERSONA_CREATOR_ENABLED),
            "Persona · optional"
        );
    }

    #[test]
    fn mercy_persona_p2_gate_on_opens_creator_without_online() {
        let mut state = PersonaCreatorState::default();
        assert!(try_open_persona_creator(&mut state, true));
        assert!(state.open);
        assert_eq!(state.step, PersonaCreatorStep::MechanicalModule);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!state.draft.presentation.story.ai_assist_used);
        assert_eq!(state.draft.presentation.story.model_id, None);
        assert_eq!(PERSONA_STORY_AI_STUB, "Story AI · records only · no LLM this slice");
    }

    #[test]
    fn mercy_persona_p2_module_not_people_not_lobby_power() {
        let mut race = MechanicalRace::Human;
        race = cycle_mechanical_module(race);
        assert_eq!(race, MechanicalRace::Quellorian);
        let label = mechanical_module_btn_label(race);
        assert!(label.contains("not people"));
        assert!(label.contains("not matchmaking"));
        assert!(persona_copy_is_honest(&label));
        // People paint independent of module.
        let mut state = PersonaCreatorState::default();
        state.draft.mechanical_race = MechanicalRace::Quellorian;
        bump_people_paint(&mut state);
        assert_eq!(state.draft.mechanical_race, MechanicalRace::Quellorian);
        assert!(matches!(
            state.draft.presentation.people,
            PeopleChoice::Preset(_)
        ));
        // From ix=1 preset, advance until custom (ix == preset_len).
        let mut guard = 0;
        while !matches!(state.draft.presentation.people, PeopleChoice::Custom(_)) {
            bump_people_paint(&mut state);
            guard += 1;
            assert!(guard <= PERSONA_PEOPLE_PRESETS.len() + 2, "never reached custom");
        }
        if let PeopleChoice::Custom(c) = &state.draft.presentation.people {
            assert!(c.invented);
            assert_ne!(c.name, state.draft.mechanical_race.as_str());
        } else {
            panic!("expected custom after presets");
        }
    }

    #[test]
    fn mercy_persona_p2_phenotype_nudge_no_stats_story_records_only() {
        let mut ph = Phenotype::default();
        let before = ph.skin_melanin;
        nudge_phenotype_paint(&mut ph);
        assert!(ph.skin_melanin >= 0.0 && ph.skin_melanin <= 1.0);
        assert_ne!(ph.skin_melanin, before);
        let mut state = PersonaCreatorState::default();
        state.story_draft = "I tend wells gently.".into();
        state.name_draft = "Mira".into();
        state.draft.mechanical_race = MechanicalRace::Cydruid;
        bump_people_paint(&mut state);
        keep_persona_local_draft(&mut state);
        assert!(state.kept_local);
        assert!(!state.open);
        assert!(!state.draft.presentation.story.ai_assist_used);
        assert_eq!(state.draft.presentation.story.model_id, None);
        assert_eq!(state.draft.presentation.given_name, "Mira");
        assert_eq!(state.draft.mechanical_race, MechanicalRace::Cydruid);
        skip_persona_to_nameless(&mut state);
        assert!(matches!(state.draft.presentation.people, PeopleChoice::Unset));
        assert!(state.draft.presentation.given_name.is_empty());
    }

    #[test]
    fn mercy_persona_p2_ui_copy_honest_and_online_grey() {
        assert!(persona_creator_ui_copy_is_honest());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(persona_copy_is_honest(&persona_preview_summary(
            &Persona::nameless_steward()
        )));
        let refuse = "race lobby ranked gold mall";
        assert!(!persona_copy_is_honest(refuse));
        // Step walk Preview: soft Keep + Commit persist — never Online required.
        let mut step = PersonaCreatorStep::MechanicalModule;
        for _ in 0..4 {
            step = step.next();
        }
        assert_eq!(step, PersonaCreatorStep::Preview);
        assert!(persona_step_guidance(step).contains("soft-caps local"));
        assert!(persona_step_guidance(step).contains("PersonaCommit"));
        assert!(!persona_step_guidance(step).to_lowercase().contains("online required"));
    }

    #[test]
    fn mercy_persona_p2_share_cycle_and_title_btn_labels() {
        let mut s = StoryShare::Private;
        s = cycle_story_share(s);
        assert_eq!(s, StoryShare::Spoken);
        s = cycle_story_share(s);
        assert_eq!(s, StoryShare::Book);
        s = cycle_story_share(s);
        assert_eq!(s, StoryShare::Private);
        assert!(story_share_btn_label(s).starts_with("Story share"));
        assert_eq!(persona_title_btn_label(true), "Persona · optional");
        assert!(persona_copy_is_honest(persona_title_btn_label(true)));
        assert!(persona_copy_is_honest(persona_title_btn_label(false)));
    }


    // --- MERCY_PERSONA P4 ----------------------------------------------------

    #[test]
    fn mercy_persona_p4_soft_draft_commit_persists_when_flag_on() {
        // Flag on: Commit works when the creator path is exercised.
        assert!(PERSONA_CREATOR_ENABLED);
        let dir = std::env::temp_dir().join(format!(
            "powrush-p4-client-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).expect("temp user dir");
        std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &dir);
        let mut state = PersonaCreatorState::default();
        assert!(try_open_persona_creator(&mut state, true));
        state.name_draft = "Mira".into();
        state.story_draft = "I tend wells gently.".into();
        state.draft.mechanical_race = MechanicalRace::Quellorian;
        bump_people_paint(&mut state);
        let commit = commit_persona_from_soft_draft(&mut state).expect("commit");
        assert!(state.committed);
        assert!(state.kept_local);
        assert!(!state.open);
        assert_eq!(state.draft.presentation.given_name, "Mira");
        assert_eq!(commit.persona.mechanical_race, MechanicalRace::Quellorian);
        assert_eq!(commit.schema, shared::persona::PERSONA_SCHEMA);
        assert_eq!(PERSONA_PATH, "data/powrush_persona.json");
        assert!(dir.join(shared::persona::PERSONA_FILE_NAME).is_file());
        // Soft Keep still available and does not claim Online.
        let mut keep_state = PersonaCreatorState::default();
        keep_state.name_draft = "Soft".into();
        keep_persona_local_draft(&mut keep_state);
        assert!(keep_state.kept_local);
        assert!(!keep_state.committed);
        skip_persona_to_nameless(&mut state);
        assert!(!state.committed);
        assert!(state.draft.presentation.given_name.is_empty());
        std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn mercy_persona_p4_commit_refuses_dishonest_and_stays_offline() {
        let mut state = PersonaCreatorState::default();
        state.story_draft = "race lobby ranked gold mall".into();
        let err = commit_persona_from_soft_draft(&mut state).unwrap_err();
        assert_eq!(err, CommitError::DishonestCopy);
        assert!(!state.committed);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(persona_creator_ui_copy_is_honest());
        assert!(persona_copy_is_honest("Commit · validate + persist"));
        assert!(!persona_copy_is_honest("Grok Online required"));
    }

    // --- MERCY_PERSONA P5 online picker (default off) -----------------------

    #[test]
    fn mercy_persona_p5_online_picker_default_off_hour_unchanged() {
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(!STEWARD_ONLINE_YES);
        assert!(!online_picker_ui_enabled(
            ONLINE_PICKER_ENABLED,
            STEWARD_ONLINE_YES
        ));
        let state = PersonaCreatorState::default();
        assert_eq!(state.story_provider, StoryProvider::None);
        assert_eq!(state.story_provider, default_story_provider());
        assert!(!state.story_provider.is_online());
        // Creator lights; picker stays off. Hour 1 nameless still skip-able.
        assert!(PERSONA_CREATOR_ENABLED);
        assert!(persona_creator_may_open(PERSONA_CREATOR_ENABLED));
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

    #[test]
    fn mercy_persona_p5_picker_lists_offline_and_gates_online() {
        // Offline seats listed; online opt-in only with flag + steward yes.
        assert!(story_provider_may_select(
            StoryProvider::None,
            false,
            false
        ));
        assert!(story_provider_may_select(
            StoryProvider::LocalTemplate,
            false,
            false
        ));
        assert!(story_provider_may_select(
            StoryProvider::RathorOfflineShard,
            false,
            false
        ));
        assert!(!story_provider_may_select(
            StoryProvider::GrokOnline,
            ONLINE_PICKER_ENABLED,
            STEWARD_ONLINE_YES
        ));
        assert!(!story_provider_may_select(
            StoryProvider::OpenAiCompatible,
            true,
            false
        ));
        assert!(story_provider_may_select(
            StoryProvider::RathorOnline,
            true,
            true
        ));
        assert_eq!(
            resolve_story_provider(
                StoryProvider::GrokOnline,
                ONLINE_PICKER_ENABLED,
                STEWARD_ONLINE_YES
            ),
            StoryProvider::None
        );
        // Cycle under defaults never lands on online seats.
        let mut p = StoryProvider::None;
        for _ in 0..9 {
            p = cycle_story_provider(p, ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES);
            assert!(p.is_offline());
        }
    }

    #[test]
    fn mercy_persona_p5_labels_honest_online_grey_no_llm_product() {
        let label = story_provider_btn_label(
            StoryProvider::None,
            ONLINE_PICKER_ENABLED,
            STEWARD_ONLINE_YES,
        );
        assert!(label.contains("online picker off"));
        assert!(persona_copy_is_honest(&label));
        assert!(persona_creator_ui_copy_is_honest());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(persona_step_guidance(PersonaCreatorStep::Story).contains("Online grey"));
        // Honesty refuses mall/P2W/gold/lobby/LLM-as-product; face≠class.
        assert!(!persona_copy_is_honest("gold mall P2W"));
        assert!(!persona_copy_is_honest("race lobby ranked"));
        assert!(!persona_copy_is_honest("LLM product upsell"));
        assert!(!persona_copy_is_honest("Grok Online required"));
        let module = mechanical_module_btn_label(MechanicalRace::Human);
        assert!(module.contains("not people"));
        assert!(module.contains("not matchmaking"));
    }

    #[test]
    fn mercy_persona_p5_commit_stays_law_no_online_default() {
        let mut state = PersonaCreatorState::default();
        state.story_provider = StoryProvider::GrokOnline; // attempted; gates refuse
        state.story_draft = "I tend wells gently.".into();
        state.name_draft = "Mira".into();
        apply_story_provider_seat(&mut state);
        assert_eq!(state.story_provider, StoryProvider::None);
        keep_persona_local_draft(&mut state);
        assert!(!state.draft.presentation.story.ai_assist_used);
        assert_eq!(state.draft.presentation.story.model_id, None);
        assert_eq!(state.story_provider, StoryProvider::None);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

}
