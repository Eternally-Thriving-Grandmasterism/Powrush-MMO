//! U2 — local hex travel (disk only) + Places plate
//!
//! Settled + book opens Places as a **door**, not a list-row teleport
//! (`SETTLED_DOOR_CLARITY` rung C). Four disk rooms: Sanctuary / Heartwood /
//! Threshold / Depths. Threshold rides Heartwood's hex file (no fifth PlaceId).
//! Confirm is leave-this-hex / enter-that-room. Without the book the pause row
//! stays hidden (or *Not your charter*). Heartwood stub: lamp disk empty, same
//! Peace E. Play always boots Sanctuary. Continue without the book boots
//! Sanctuary. Dedicated Places plate (LivedUiPlate / Camera2d) — not extra
//! Settings rows. After PAUSE-TABS the Settled+book **Places** door sits on the
//! Esc pause plate (Comfort · Controls · Guide kept); sticks cull when open.
//! H-2026-09-12-PLACES-CLICK: Places row Pressed must open the four-room plate
//! above Comfort (z+2) and keep pause armed — not Comfort/settings linger.
//! H-2026-09-12-PLACES-FAT: four-room + Confirm/Back hit targets ≥44 logical dp
//! (fat-tap / lavapipe click-clean). Peace tone; no second HUD.
//!
//! CARD L3 PEOPLE-DOOR-LAND — try_cross success → apply_place / lived bind.
//! PeopleLanding maps onto the three existing PlaceId variants. Threshold
//! rides Heartwood disk. Cite L3_SPAWN_RESEARCH §3 · PLAYABLE_RACES §1.1.
//! CARD L4 PLACE-DRESS-ON-LAND — after apply_people_landing, the same
//! apply_place path Esc→Places uses turns on the authored Place dress
//! (climate_plane look_for). 0 meshes. Cite PLACE_DRESS_SPEC · ART_BIBLE.
//! CARD L5 TITLE-GARDEN-LAND — Title garden_cross_landing and first-session
//! People-door call [`apply_people_landing`] here. No PlaceId remap.
//! CARD L7 ARRIVAL-BEAT — after apply_people_landing, one beat keyed by
//! PeopleLanding (fog / camera / SoftPresence already on tip). No PlaceId remap.
//! Skip House → no beat. Ambrosian = lift on Sanctuary disk (FORK A).
//! CARD S2 GATE-SEAL — decline / wrong door restores garden PlaceId (no net
//! change) and does not run the L7 beat. Confirm is existing E/Q in
//! hour_sacred; this file only restores PlaceId. PlaceId stays 3.
//! CARD F5 WRONG-DOOR-BOUNCE — unsealed light may take a People-door and
//! feel the existing L7 beat (apply_people_landing already arms it). Land
//! is not a seal. Decline / wrong door restores garden PlaceId + garden
//! wake; no L7 beat on bounce. PlaceId stays 3. L7 fog WRITE unread.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hex_travel::{
    apply_travel_named, boot_place, confirm_leave, house_embassy_on_place, house_week_footer,
    places_eligible, places_row_label, read_current_named, read_hex_named,
    BootKind, PlaceId, TravelRefuse, PLACES_TITLE,
};
use shared::pause_ledger_face::NOT_YOUR_CHARTER;
use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

use crate::embassy::EmbassyYard;
use crate::hour_sacred::{
    decline_or_wrong_door, try_cross_people_door, unsealed_light_may_take_people_door,
    HousePeople, HourSacred, PeopleLanding,
};
use crate::human_presence::{
    arrival_beat_after_land, run_arrival_beat, wake_garden_bounce, wake_people_landing,
    ArrivalBeat, SoftPresence,
};
use crate::lived_hour_bind::LivedHourBind;
use crate::title_screen::{
    yard_after_travel, HouseLabel, LaunchDoor, TITLE_BORDER, TITLE_BTN_BG, TITLE_BTN_FG,
    TITLE_PLATE_BG, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY,
};
use crate::ui_above_world::{LivedUiPlate, LIVED_UI_Z_PAUSE};

/// Soft-GPU stack: four-room Places door draws above pause Comfort (+ banner at +1).
pub const PLACES_PLATE_Z: i32 = LIVED_UI_Z_PAUSE + 2;

/// Minimum Places deck / Confirm / Back hit target (logical px) — INPUT_CANON ≥44dp.
pub const PLACES_HIT_MIN: f32 = 44.0;

/// Click → open_door runs before pause/settings visibility sync (same frame).
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlacesDoorClickSet;

#[derive(Resource, Debug, Clone)]
pub struct HexTravelState {
    pub current: PlaceId,
}

impl Default for HexTravelState {
    fn default() -> Self {
        Self {
            current: read_current_named().unwrap_or(PlaceId::Sanctuary),
        }
    }
}

impl HexTravelState {
    pub fn chip_name(&self) -> &'static str {
        self.current.chip_name()
    }
}

/// Confirm button — threshold language, not a teleport tap.
const DOOR_CONFIRM_BTN: &str = "Leave · enter";
/// Idle cue on the open Places plate (no dest chosen yet).
const DOOR_IDLE_CUE: &str = "Leave this hex · enter that room";

/// One of the four Offline rooms named on the Places door.
/// Threshold shares Heartwood's disk file (`OFFLINE_SKU`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlacesRoom {
    Sanctuary,
    Heartwood,
    Threshold,
    Depths,
}

impl PlacesRoom {
    const fn display_name(self) -> &'static str {
        match self {
            PlacesRoom::Sanctuary => "Sanctuary",
            PlacesRoom::Heartwood => "Heartwood",
            PlacesRoom::Threshold => "Threshold",
            PlacesRoom::Depths => "Depths",
        }
    }

    /// Disk hex this room loads. Threshold → Heartwood file.
    const fn place_id(self) -> PlaceId {
        match self {
            PlacesRoom::Sanctuary => PlaceId::Sanctuary,
            PlacesRoom::Heartwood | PlacesRoom::Threshold => PlaceId::Heartwood,
            PlacesRoom::Depths => PlaceId::Depths,
        }
    }
}

/// Leave / enter confirm copy for a chosen room.
fn door_confirm_cue(room: PlacesRoom) -> String {
    format!("Leave this hex · enter {}?", room.display_name())
}

#[derive(Resource, Debug, Default, Clone)]
pub struct PlacesPlate {
    pub open: bool,
    pub selected: Option<PlaceId>,
    pub confirm_pending: bool,
    /// Door name for cue (Threshold may share Heartwood's PlaceId).
    selected_room: Option<&'static str>,
}

impl PlacesPlate {
    /// Open the four-room door from the Esc pause Places row — pause stays armed.
    pub fn open_door(&mut self) {
        self.open = true;
        self.selected = None;
        self.selected_room = None;
        self.confirm_pending = false;
    }

    /// Close the Places door (Back / Esc leaf / Resume) without touching pause.
    pub fn close_door(&mut self) {
        self.open = false;
        self.selected = None;
        self.selected_room = None;
        self.confirm_pending = false;
    }
}

#[derive(Component)]
struct PlacesRoot;
#[derive(Component)]
struct PlacesCueText;
#[derive(Component)]
struct PlacesSanctuaryBtn;
#[derive(Component)]
struct PlacesHeartwoodBtn;
#[derive(Component)]
struct PlacesThresholdBtn;
#[derive(Component)]
struct PlacesDepthsBtn;
#[derive(Component)]
struct PlacesConfirmBtn;
#[derive(Component)]
struct PlacesBackBtn;
/// Pause-plate Places door (Settled+book). Spawned on the Esc plate after PAUSE-TABS.
#[derive(Component)]
pub struct PausePlacesBtn;

pub struct HexTravelPlugin;

impl Plugin for HexTravelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HexTravelState>()
            .init_resource::<PlacesPlate>()
            .add_systems(Startup, spawn_places_plate)
            .add_systems(
                Update,
                (
                    boot_guard_no_book_stays_sanctuary,
                    pause_places_row_clicks.in_set(PlacesDoorClickSet),
                    places_plate_clicks,
                    sync_places_visibility.after(PlacesDoorClickSet),
                    refresh_places_labels.after(PlacesDoorClickSet),
                    refresh_pause_places_row.after(PlacesDoorClickSet),
                ),
            );
    }
}

/// Play / Continue boot. Play always Sanctuary. No-book Continue: Sanctuary.
/// Play does not rewrite the last-hex pointer — Continue + book still loads it.
pub fn apply_title_boot(
    kind: BootKind,
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    hour: &HourSacred,
    embassy: Option<&mut EmbassyYard>,
) {
    let book = hour.hour_three_complete;
    let last = read_current_named().or(Some(travel.current));
    let dest = boot_place(kind, book, last);
    apply_place(travel, bind, embassy, dest, book);
}

/// Places / title-boot climate + hex. Heartwood / Depths still need the book
/// on this path. People-door land uses [`apply_people_landing`] (House + Tend).
pub fn apply_place(
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    mut embassy: Option<&mut EmbassyYard>,
    dest: PlaceId,
    book: bool,
) {
    if (dest == PlaceId::Heartwood || dest == PlaceId::Depths) && !book {
        travel.current = PlaceId::Sanctuary;
        bind.apply_place(PlaceId::Sanctuary);
        return;
    }
    travel.current = dest;
    bind.apply_place(dest);
    if dest == PlaceId::Sanctuary {
        restore_house_embassy(embassy.as_deref_mut());
    }
    maybe_sum_house_week(bind);
}

/// CARD L3 — try_cross success → apply_place / lived bind. House + Tend
/// authorizes Heartwood / Depths (not Places book). One-way stays L2.
/// Threshold disk is Heartwood (no fourth PlaceId).
/// CARD L4 — same apply_place dress path Esc→Places uses for that PlaceId
/// (climate swap + HexTravelState). climate_plane syncs look_for from
/// travel.current — no second dresser.
/// CARD L7 — after land, one arrival beat keyed by PeopleLanding (not a new
/// hex). Ambrosian = lift on Sanctuary disk (FORK A). mothership-over-Earth
/// PRESENTATION; no hull mesh. Skip House never reaches here → no beat.
pub fn apply_people_landing(
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    embassy: Option<&mut EmbassyYard>,
    landing: PeopleLanding,
    presence: Option<&mut SoftPresence>,
) {
    let dest = landing.place_id();
    apply_place(travel, bind, embassy, dest, true);
    if let Some(p) = presence {
        wake_people_landing(p, landing);
    }
    // CARD L7 — primitives / fog / camera already in ClimatePlanePlugin only.
    let _beat: ArrivalBeat = run_arrival_beat(landing);
}

/// CARD L3 — L2 door then land. Skip House / no Tend / already crossed = no move.
pub fn try_cross_people_door_land(
    house_live: bool,
    tended_once: bool,
    crossed: &mut Option<HousePeople>,
    people: HousePeople,
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    embassy: Option<&mut EmbassyYard>,
    presence: Option<&mut SoftPresence>,
) -> Option<PeopleLanding> {
    let landing = try_cross_people_door(house_live, tended_once, crossed, people)?;
    apply_people_landing(travel, bind, embassy, landing, presence);
    Some(landing)
}

/// CARD S2 — decline / wrong door returns the body to garden. PlaceId is
/// restored to `garden` (boot disk, not a fifth Place). No L7 arrival beat.
/// Session cross is cleared. Does not unseal a confirmed soul — caller checks.
pub fn decline_people_door_land(
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    embassy: Option<&mut EmbassyYard>,
    garden: PlaceId,
    crossed: &mut Option<HousePeople>,
) {
    *crossed = None;
    apply_place(travel, bind, embassy, garden, true);
}

/// CARD F5 — unsealed light takes a People-door and feels the existing L7 beat.
/// Land is not a seal. Skip House / sealed / no Tend / already crossed = none.
/// Does not recook L7 fog — [`apply_people_landing`] already arms the beat.
pub fn try_unsealed_light_people_door_land(
    house_live: bool,
    tended_once: bool,
    sealed: Option<(HousePeople, PeopleLanding)>,
    crossed: &mut Option<HousePeople>,
    people: HousePeople,
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    embassy: Option<&mut EmbassyYard>,
    presence: Option<&mut SoftPresence>,
) -> Option<(PeopleLanding, ArrivalBeat)> {
    if !unsealed_light_may_take_people_door(house_live, tended_once, sealed) {
        return None;
    }
    let landing = try_cross_people_door_land(
        house_live,
        tended_once,
        crossed,
        people,
        travel,
        bind,
        embassy,
        presence,
    )?;
    Some((landing, arrival_beat_after_land(Some(landing))))
}

/// CARD F5 — decline / wrong door: garden PlaceId, still light, no L7 beat.
/// Restores SoftPresence to the garden wake (Sanctuary yard helper), not a land.
pub fn bounce_wrong_door_to_garden(
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    embassy: Option<&mut EmbassyYard>,
    garden: PlaceId,
    crossed: &mut Option<HousePeople>,
    pending_landing: &mut Option<PeopleLanding>,
    sealed: Option<(HousePeople, PeopleLanding)>,
    presence: Option<&mut SoftPresence>,
) -> bool {
    if !decline_or_wrong_door(crossed, pending_landing, sealed) {
        return false;
    }
    decline_people_door_land(travel, bind, embassy, garden, crossed);
    if let Some(p) = presence {
        wake_garden_bounce(p);
    }
    true
}

/// CARD F5 — bounce does not arm the L7 arrival beat.
pub fn wrong_door_bounce_arrival_beat() -> ArrivalBeat {
    arrival_beat_after_land(None)
}

fn restore_house_embassy(embassy: Option<&mut EmbassyYard>) {
    if let Some(yard) = embassy {
        if let Some(raw) = crate::hour_sacred::read_hour_two_json() {
            let pack = shared::hour_two::HourTwoPack::from_json(&raw);
            yard.embassy = house_embassy_on_place(&pack.embassy, PlaceId::Sanctuary);
        }
    }
}

fn maybe_sum_house_week(bind: &mut LivedHourBind) {
    let mut climates = Vec::new();
    if let Some(s) = read_hex_named(PlaceId::Sanctuary) {
        climates.push(s.climate);
    }
    if let Some(h) = read_hex_named(PlaceId::Heartwood) {
        climates.push(h.climate);
    }
    if let Some(d) = read_hex_named(PlaceId::Depths) {
        climates.push(d.climate);
    }
    if climates.is_empty() {
        return;
    }
    if !climates.iter().any(|c| c.hex_id == bind.climate.hex_id) {
        climates.push(bind.climate.clone());
    }
    bind.week = house_week_footer(&climates);
    bind.refresh_climate_slab_keep_week();
}

/// First Continue / boot without the book cannot sit on Heartwood.
fn boot_guard_no_book_stays_sanctuary(
    hour: Option<Res<HourSacred>>,
    mut travel: ResMut<HexTravelState>,
    mut bind: Option<ResMut<LivedHourBind>>,
    mut embassy: Option<ResMut<EmbassyYard>>,
    door: Res<LaunchDoor>,
) {
    let Some(hour) = hour else {
        return;
    };
    if hour.hour_three_complete {
        return;
    }
    if travel.current != PlaceId::Heartwood
        && travel.current != PlaceId::Depths
        && bind.as_ref().map(|b| b.climate.hex_id.as_str()) != Some(PlaceId::Heartwood.as_str())
        && bind.as_ref().map(|b| b.climate.hex_id.as_str()) != Some(PlaceId::Depths.as_str())
    {
        if travel.current != PlaceId::Sanctuary && *door == LaunchDoor::Title {
            travel.current = PlaceId::Sanctuary;
        }
        return;
    }
    let Some(bind) = bind.as_mut() else {
        travel.current = PlaceId::Sanctuary;
        return;
    };
    apply_place(
        &mut travel,
        bind,
        embassy.as_deref_mut(),
        PlaceId::Sanctuary,
        false,
    );
}

fn spawn_places_plate(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(18.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(400.0),
                    margin: UiRect {
                        left: Val::Px(-200.0),
                        ..default()
                    },
                    padding: UiRect::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    border: UiRect::all(Val::Px(1.5)),
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                // Above pause Comfort (130) and Comfort graphics banner (131).
                z_index: ZIndex::Global(PLACES_PLATE_Z),
                focus_policy: bevy::ui::FocusPolicy::Block,
                ..default()
            },
            PlacesRoot,
            LivedUiPlate,
            Name::new("PlacesPlate"),
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                PLACES_TITLE,
                TextStyle {
                    font_size: 18.0,
                    color: TITLE_TEXT_PRIMARY,
                    ..default()
                },
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
                PlacesCueText,
            ));
            spawn_places_btn(p, "Sanctuary", PlacesSanctuaryBtn);
            spawn_places_btn(p, "Heartwood", PlacesHeartwoodBtn);
            spawn_places_btn(p, "Threshold", PlacesThresholdBtn);
            spawn_places_btn(p, "Depths", PlacesDepthsBtn);
            spawn_places_btn(p, DOOR_CONFIRM_BTN, PlacesConfirmBtn);
            spawn_places_btn(p, "Back", PlacesBackBtn);
        });
}

fn spawn_places_btn<C: Component>(p: &mut ChildBuilder, label: &str, marker: C) {
    p.spawn((
        ButtonBundle {
            style: Style {
                // Fat-tap ≥44dp (lavapipe click-clean); Peace tone, stretch width.
                min_height: Val::Px(PLACES_HIT_MIN),
                padding: UiRect::axes(Val::Px(14.0), Val::Px(12.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: TITLE_BTN_BG.into(),
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
                color: TITLE_BTN_FG,
                ..default()
            },
        ));
    });
}

/// Places door is spawned on the Esc pause plate in `title_screen::spawn_settings_stub`
/// (PAUSE-TABS layout). This module owns clicks + Settled+book visibility only.

fn book_flags(hour: Option<&HourSacred>) -> (bool, bool) {
    let settled = hour.map(|h| h.complete).unwrap_or(false);
    let book = hour.map(|h| h.hour_three_complete).unwrap_or(false);
    (settled, book)
}

fn sync_places_visibility(
    plate: Res<PlacesPlate>,
    mut q: Query<&mut Visibility, With<PlacesRoot>>,
) {
    let vis = if plate.open {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    for mut v in &mut q {
        if *v != vis {
            *v = vis;
        }
    }
}

fn refresh_pause_places_row(
    label: Res<HouseLabel>,
    door: Res<LaunchDoor>,
    hour: Option<Res<HourSacred>>,
    plate: Res<PlacesPlate>,
    mut q: Query<(&mut Style, &mut Visibility), With<PausePlacesBtn>>,
) {
    // On-plate door: hidden without Settled+book so Comfort/Controls/Guide height holds.
    let (settled, book) = book_flags(hour.as_deref());
    let live = places_eligible(settled, book)
        && label.settings_open
        && !plate.open
        && *door == LaunchDoor::InYard;
    for (mut style, mut vis) in &mut q {
        style.display = if live { Display::Flex } else { Display::None };
        *vis = if live {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn refresh_places_labels(
    plate: Res<PlacesPlate>,
    mut cue: Query<&mut Text, With<PlacesCueText>>,
) {
    if !plate.open {
        return;
    }
    // Door threshold copy — not a roster chip / teleport toast.
    // Arrival names the room on the climate slab (PLACE_CLARITY), not a second HUD.
    let line = if plate.confirm_pending {
        match plate.selected_room {
            Some(name) => format!("Leave this hex · enter {name}?"),
            None => DOOR_CONFIRM_BTN.to_string(),
        }
    } else {
        DOOR_IDLE_CUE.to_string()
    };
    for mut text in &mut cue {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
        }
    }
}

fn pause_places_row_clicks(
    label: Res<HouseLabel>,
    hour: Option<Res<HourSacred>>,
    mut plate: ResMut<PlacesPlate>,
    clicks: Query<&Interaction, (Changed<Interaction>, With<PausePlacesBtn>)>,
) {
    // Places is a leaf of the open pause plate — never dismiss pause here.
    // Pressed opens the four-room door; Comfort chrome hides via
    // settings_visible_with_places + sync after PlacesDoorClickSet.
    if !label.settings_open || plate.open {
        return;
    }
    let (settled, book) = book_flags(hour.as_deref());
    if !places_eligible(settled, book) {
        return;
    }
    for i in &clicks {
        if *i == Interaction::Pressed {
            plate.open_door();
            return;
        }
    }
}

fn places_plate_clicks(
    hour: Option<Res<HourSacred>>,
    mut plate: ResMut<PlacesPlate>,
    mut label: ResMut<HouseLabel>,
    mut travel: ResMut<HexTravelState>,
    mut bind: Option<ResMut<LivedHourBind>>,
    mut embassy: Option<ResMut<EmbassyYard>>,
    sanctuary: Query<&Interaction, (Changed<Interaction>, With<PlacesSanctuaryBtn>)>,
    heartwood: Query<&Interaction, (Changed<Interaction>, With<PlacesHeartwoodBtn>)>,
    threshold: Query<&Interaction, (Changed<Interaction>, With<PlacesThresholdBtn>)>,
    depths: Query<&Interaction, (Changed<Interaction>, With<PlacesDepthsBtn>)>,
    confirm: Query<&Interaction, (Changed<Interaction>, With<PlacesConfirmBtn>)>,
    back: Query<&Interaction, (Changed<Interaction>, With<PlacesBackBtn>)>,
) {
    if !plate.open {
        return;
    }
    for i in &back {
        if *i == Interaction::Pressed {
            plate.close_door();
            return;
        }
    }
    let (settled, book) = book_flags(hour.as_deref());
    if !places_eligible(settled, book) {
        plate.close_door();
        return;
    }
    for i in &sanctuary {
        if *i == Interaction::Pressed {
            select_room(&mut plate, travel.current, PlacesRoom::Sanctuary);
            return;
        }
    }
    for i in &heartwood {
        if *i == Interaction::Pressed {
            select_room(&mut plate, travel.current, PlacesRoom::Heartwood);
            return;
        }
    }
    for i in &threshold {
        if *i == Interaction::Pressed {
            select_room(&mut plate, travel.current, PlacesRoom::Threshold);
            return;
        }
    }
    for i in &depths {
        if *i == Interaction::Pressed {
            select_room(&mut plate, travel.current, PlacesRoom::Depths);
            return;
        }
    }
    for i in &confirm {
        if *i == Interaction::Pressed {
            if !plate.confirm_pending {
                return;
            }
            let Some(to) = plate.selected else {
                return;
            };
            let Some(bind) = bind.as_mut() else {
                return;
            };
            match confirm_leave(settled, book, travel.current, to) {
                Ok(_) => {
                    // Depths save-on-exit: Peace restore ink must hit disk before
                    // the body sits Sanctuary, or House week stays Sanctuary-only.
                    if travel.current == PlaceId::Depths {
                        let file = shared::hex_travel::HexClimateFile::from_parts(
                            PlaceId::Depths,
                            bind.climate.clone(),
                            bind.standing.clone(),
                        );
                        let _ = shared::hex_travel::write_hex_named(&file);
                    }
                    if let Ok(loaded) = apply_travel_named(
                        settled,
                        book,
                        travel.current,
                        to,
                        &bind.climate,
                        &bind.standing,
                    ) {
                        travel.current = to;
                        bind.climate = loaded.climate;
                        bind.standing = loaded.standing;
                        bind.refresh_climate_slab_keep_week();
                        maybe_sum_house_week(bind);
                        bind.persist();
                        if to == PlaceId::Sanctuary {
                            restore_house_embassy(embassy.as_deref_mut());
                        }
                        // Heartwood: leave the house embassy seated. Stub lamp is hex climate.
                    }
                    // Land in the new hex's yard: no plate carried over, so Esc
                    // there opens pause instead of closing the plate from before.
                    let landed = yard_after_travel();
                    label.settings_open = landed.pause_open;
                    if landed.places_open {
                        plate.open_door();
                    } else {
                        plate.close_door();
                    }
                }
                Err(TravelRefuse::NotYourCharter) | Err(TravelRefuse::SamePlace) => {
                    plate.selected = None;
                    plate.selected_room = None;
                    plate.confirm_pending = false;
                }
            }
            return;
        }
    }
}

fn select_room(plate: &mut PlacesPlate, current: PlaceId, room: PlacesRoom) {
    let dest = room.place_id();
    if dest == current {
        plate.selected = None;
        plate.selected_room = None;
        plate.confirm_pending = false;
        return;
    }
    plate.selected = Some(dest);
    plate.selected_room = Some(room.display_name());
    plate.confirm_pending = true;
}

/// Test / walk helper: pick a disk PlaceId under its own room name.
fn select_dest(plate: &mut PlacesPlate, current: PlaceId, dest: PlaceId) {
    let room = match dest {
        PlaceId::Sanctuary => PlacesRoom::Sanctuary,
        PlaceId::Heartwood => PlacesRoom::Heartwood,
        PlaceId::Depths => PlacesRoom::Depths,
    };
    select_room(plate, current, room);
}

/// Sticks / grove cull when the Places plate is open (same spirit as pause).
pub fn places_plate_open(plate: &PlacesPlate) -> bool {
    plate.open
}

pub fn places_culls_sticks(plate: &PlacesPlate) -> bool {
    plate.open
}

/// HouseLabel + Places: pause/Settings hide while Places is open so they are not buried.
pub fn settings_visible_with_places(settings_open: bool, places_open: bool) -> bool {
    settings_open && !places_open
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::keyboard::{Key, KeyboardInput};
    use bevy::input::ButtonState;
    use bevy::input::InputPlugin as BevyInputPlugin;
    use bevy::MinimalPlugins;
    use shared::f_book_fixture::fixture_is_not_default_door;
    use shared::hex_listen::PowrushNet;
    use shared::hex_protocol::default_client_listens;
    use shared::hex_travel::{
        confirm_leave, heartwood_stub_embassy, hex_file_name, new_game_writes_heartwood,
        places_row_or_inert, travel_is_disk_only, CURRENT_HEX_FILE, ISOLATION_GAMMA, LOCAL_HEXES,
        TravelRefuse,
    };
    use shared::house_name::HouseName;
    use shared::space_law::HexFlag;
    use shared::user_persist::is_f_book_fixture_dir;

    use crate::title_screen::TitleScreenPlugin;

    /// Yard on `place` with Settled + book, pause and Places both closed.
    /// Same door the walk reaches after Continue; no disk write.
    fn yard_app(place: PlaceId) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(BevyInputPlugin);
        app.add_event::<bevy::window::ReceivedCharacter>();
        let mut house = HouseName::default();
        house.skip();
        house.skip_seals();
        house.skip_heritage();
        app.insert_resource(HexTravelState { current: place });
        app.insert_resource(HourSacred {
            session: Default::default(),
            complete: true,
            hour_three_complete: true,
        });
        app.insert_resource(LaunchDoor::InYard);
        app.insert_resource(HouseLabel {
            house,
            persist_present: true,
            hour_two_held: true,
            book_held: true,
            settings_open: false,
            draft: String::new(),
            naming_offered: true,
            seals_offered: true,
        });
        app.insert_resource(PlacesPlate::default());
        app.add_plugins(crate::net_mode::NetModePlugin);
        app.add_plugins(crate::input::InputPlugin);
        app.add_plugins(TitleScreenPlugin);
        app.add_plugins(HexTravelPlugin);
        app.update();
        app
    }

    /// One real key edge: winit press event, frame, release event, frame.
    fn tap_key(app: &mut App, code: KeyCode, key: Key) {
        let window = Entity::PLACEHOLDER;
        app.world_mut().send_event(KeyboardInput {
            key_code: code,
            logical_key: key.clone(),
            state: ButtonState::Pressed,
            window,
        });
        app.update();
        app.world_mut().send_event(KeyboardInput {
            key_code: code,
            logical_key: key,
            state: ButtonState::Released,
            window,
        });
        app.update();
    }

    fn tap_escape(app: &mut App) {
        tap_key(app, KeyCode::Escape, Key::Escape);
    }

    fn pause_is_open(app: &App) -> bool {
        app.world().resource::<HouseLabel>().settings_open
    }

    /// Places row is laid out (not `Display::None`) on the open pause plate.
    fn places_row_live(app: &mut App) -> bool {
        let world = app.world_mut();
        let mut q = world.query_filtered::<(&Style, &Visibility), With<PausePlacesBtn>>();
        q.iter(world)
            .any(|(style, vis)| style.display == Display::Flex && *vis == Visibility::Visible)
    }

    fn settings_stub_showing(app: &mut App) -> bool {
        use crate::title_screen::settings_stub_is_showing;
        settings_stub_is_showing(app.world_mut())
    }

    fn places_plate_showing(app: &mut App) -> bool {
        let world = app.world_mut();
        let mut q = world.query_filtered::<&Visibility, With<PlacesRoot>>();
        q.iter(world).any(|vis| *vis == Visibility::Visible)
    }

    #[test]
    fn esc_opens_pause_on_every_local_hex() {
        for place in LOCAL_HEXES {
            let mut app = yard_app(place);
            assert!(
                !pause_is_open(&app),
                "{place:?}: yard starts without the pause plate"
            );
            tap_escape(&mut app);
            assert!(
                pause_is_open(&app),
                "{place:?}: Esc from the yard must open pause"
            );
            assert_eq!(
                *app.world().resource::<LaunchDoor>(),
                LaunchDoor::InYard,
                "{place:?}: Esc never walks to Title"
            );
            assert_eq!(
                crate::title_screen::pause_plate_line(LaunchDoor::InYard),
                Some(crate::title_screen::YARD_WAITING)
            );
            assert!(
                places_row_live(&mut app),
                "{place:?}: Places stays on the pause plate after Settled + book"
            );
            // Esc again is Resume — still the yard, never Title.
            tap_escape(&mut app);
            assert!(!pause_is_open(&app), "{place:?}: Esc closes pause again");
            assert_eq!(*app.world().resource::<LaunchDoor>(), LaunchDoor::InYard);
        }
    }

    #[test]
    fn esc_and_key_three_agree_on_every_local_hex() {
        for place in LOCAL_HEXES {
            let mut app = yard_app(place);
            tap_key(&mut app, KeyCode::Digit3, Key::Character("3".into()));
            assert!(pause_is_open(&app), "{place:?}: key 3 opens pause");
            tap_key(&mut app, KeyCode::Digit3, Key::Character("3".into()));
            assert!(!pause_is_open(&app), "{place:?}: key 3 closes pause");
            tap_escape(&mut app);
            assert!(pause_is_open(&app), "{place:?}: Esc opens the same plate");
        }
    }

    /// An open Places plate must not eat the Esc edge — it walks back to pause.
    #[test]
    fn esc_from_places_returns_to_the_pause_plate() {
        for place in LOCAL_HEXES {
            let mut app = yard_app(place);
            tap_escape(&mut app);
            app.world_mut().resource_mut::<PlacesPlate>().open = true;
            app.update();
            tap_escape(&mut app);
            assert!(
                !app.world().resource::<PlacesPlate>().open,
                "{place:?}: Esc closes Places"
            );
            assert!(
                pause_is_open(&app),
                "{place:?}: and lands back on the pause plate"
            );
            assert!(places_row_live(&mut app));
        }
    }

    /// The walk that failed: Places → other hex → Esc there opens pause.
    #[test]
    fn leave_this_hex_lands_in_the_yard_and_esc_opens_pause_there() {
        let dir = std::env::temp_dir().join(format!(
            "powrush-hex-walk-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::create_dir_all(&dir);
        std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &dir);

        let mut app = yard_app(PlaceId::Sanctuary);
        app.insert_resource(LivedHourBind::default());
        app.update();

        // Esc opens pause, the Places row opens the plate, Heartwood is picked.
        tap_escape(&mut app);
        assert!(pause_is_open(&app));
        {
            let mut plate = app.world_mut().resource_mut::<PlacesPlate>();
            plate.open = true;
            select_dest(&mut plate, PlaceId::Sanctuary, PlaceId::Heartwood);
        }
        app.update();
        assert!(app.world().resource::<PlacesPlate>().confirm_pending);

        // Leave this hex.
        let mut confirm_q = app
            .world_mut()
            .query_filtered::<Entity, With<PlacesConfirmBtn>>();
        let confirm = confirm_q
            .iter(app.world())
            .next()
            .expect("confirm button");
        *app.world_mut().get_mut::<Interaction>(confirm).unwrap() = Interaction::Pressed;
        app.update();
        *app.world_mut().get_mut::<Interaction>(confirm).unwrap() = Interaction::None;
        app.update();

        assert_eq!(
            app.world().resource::<HexTravelState>().current,
            PlaceId::Heartwood,
            "Leave this hex seats Heartwood"
        );
        assert!(
            !app.world().resource::<PlacesPlate>().open,
            "Places closes on arrival"
        );
        assert!(
            !pause_is_open(&app),
            "arrival lands in the yard — no plate held over from Sanctuary"
        );

        // The fail: Esc on Heartwood needed a click or key 3. Now it opens pause.
        tap_escape(&mut app);
        assert!(pause_is_open(&app), "Esc on Heartwood opens pause");
        assert!(
            places_row_live(&mut app),
            "Places stays eligible on the Heartwood pause plate"
        );

        // And Places → Sanctuary → Leave this hex still walks home.
        {
            let mut plate = app.world_mut().resource_mut::<PlacesPlate>();
            plate.open = true;
            select_dest(&mut plate, PlaceId::Heartwood, PlaceId::Sanctuary);
        }
        app.update();
        *app.world_mut().get_mut::<Interaction>(confirm).unwrap() = Interaction::Pressed;
        app.update();
        *app.world_mut().get_mut::<Interaction>(confirm).unwrap() = Interaction::None;
        app.update();
        assert_eq!(
            app.world().resource::<HexTravelState>().current,
            PlaceId::Sanctuary
        );
        assert!(!pause_is_open(&app));
        tap_escape(&mut app);
        assert!(pause_is_open(&app), "Esc on Sanctuary opens pause");

        // Book stayed seated through the walk — Online never woke.
        let hour = app.world().resource::<HourSacred>();
        assert!(hour.complete && hour.hour_three_complete);
        assert!(!default_client_listens());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn play_and_no_book_continue_stay_sanctuary() {
        assert_eq!(
            boot_place(BootKind::Play, true, Some(PlaceId::Heartwood)),
            PlaceId::Sanctuary
        );
        assert_eq!(
            boot_place(BootKind::Continue, false, Some(PlaceId::Heartwood)),
            PlaceId::Sanctuary
        );
    }

    #[test]
    fn places_hidden_without_book() {
        assert_eq!(places_row_label(false, false), None);
        assert_eq!(places_row_or_inert(false, false, false), NOT_YOUR_CHARTER);
        assert!(!places_eligible(true, false));
    }

    /// Playtest H2-TAB: E on the visitor ridge is *Not your charter*, not harvest
    /// and not a Places leave. Settled+book is Hour 3 — Embassy is not required.
    #[test]
    fn h2_tab_visitor_ridge_e_is_not_your_charter() {
        assert_eq!(
            confirm_leave(false, false, PlaceId::Sanctuary, PlaceId::Heartwood),
            Err(TravelRefuse::NotYourCharter)
        );
        assert_eq!(
            places_row_or_inert(false, false, false),
            NOT_YOUR_CHARTER
        );
        assert!(!places_eligible(false, false));
    }

    #[test]
    fn travel_file_names_and_gamma() {
        assert_eq!(hex_file_name(PlaceId::Sanctuary), "powrush_hex_sanctuary.json");
        assert_eq!(hex_file_name(PlaceId::Heartwood), "powrush_hex_heartwood.json");
        assert_eq!(ISOLATION_GAMMA, 0.0);
        assert!(travel_is_disk_only());
        assert!(!new_game_writes_heartwood());
        assert_eq!(CURRENT_HEX_FILE, "powrush_hex_current.json");
    }

    #[test]
    fn f_book_not_default_door_and_online_grey() {
        assert!(fixture_is_not_default_door());
        assert!(!is_f_book_fixture_dir(
            &shared::user_persist::os_user_data_dir()
        ));
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(!default_client_listens());
    }

    #[test]
    fn heartwood_stub_peace_e_lamp_empty() {
        assert_eq!(PlaceId::Heartwood.peace_hex(), HexFlag::Peace);
        let e = heartwood_stub_embassy();
        assert!(!e.lamp_live);
        assert!(places_culls_sticks(&PlacesPlate { open: true, ..Default::default() }));
        assert!(!places_culls_sticks(&PlacesPlate::default()));
        assert!(!settings_visible_with_places(true, true));
        assert!(settings_visible_with_places(true, false));
        assert_eq!(LIVED_UI_Z_PAUSE, 130);
        assert_eq!(PLACES_PLATE_Z, LIVED_UI_Z_PAUSE + 2);
        assert!(PLACES_PLATE_Z > LIVED_UI_Z_PAUSE + 1, "Places above Comfort banner");
    }

    #[test]
    fn plate_stays_on_ui_camera_order() {
        assert!(crate::ui_above_world::ui_camera_draws_above_world());
        assert!(crate::ui_above_world::pause_z_above_title());
    }

    #[test]
    fn house_embassy_stays_on_heartwood_stub() {
        use shared::hex_travel::house_embassy_on_place;
        use shared::stranger_loop_proof::hour_three_held_fixture;
        let house = hour_three_held_fixture();
        let on_stub = house_embassy_on_place(&house.embassy, PlaceId::Heartwood);
        assert!(on_stub.seated);
        assert_eq!(on_stub, house.embassy);
        assert_ne!(on_stub, heartwood_stub_embassy());
        assert!(places_eligible(house.complete, house.hour_three_complete));
    }

    #[test]
    fn heartwood_lamp_spatial_is_shared_not_embassy_mesh() {
        use shared::heartwood_lamp::{
            embassy_lamp_is_spatial_gate, heartwood_mesh_on_sanctuary, hanging_mesh_shipped,
            try_place_building, BuildRefuse, LAMP_DISK_CENTER, WATER_POND_CENTER,
        };
        assert_eq!(
            try_place_building(
                PlaceId::Heartwood,
                WATER_POND_CENTER[0],
                WATER_POND_CENTER[1]
            ),
            Err(BuildRefuse::Water)
        );
        assert_eq!(
            try_place_building(
                PlaceId::Heartwood,
                LAMP_DISK_CENTER[0],
                LAMP_DISK_CENTER[1]
            ),
            Err(BuildRefuse::LampDisk)
        );
        assert!(!embassy_lamp_is_spatial_gate());
        assert!(!heartwood_mesh_on_sanctuary());
        assert!(hanging_mesh_shipped());
        assert_eq!(PlaceId::Heartwood.peace_hex(), HexFlag::Peace);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!default_client_listens());
    }

    /// Esc pause Places row opens the four-room door — does not dismiss pause
    /// and must clear Comfort/settings chrome (not linger over Sanctuary·…).
    #[test]
    fn pause_places_row_click_opens_four_room_plate() {
        for place in LOCAL_HEXES {
            let mut app = yard_app(place);
            tap_escape(&mut app);
            assert!(pause_is_open(&app), "{place:?}: pause open");
            assert!(places_row_live(&mut app), "{place:?}: Places door on pause plate");
            assert!(!app.world().resource::<PlacesPlate>().open);
            assert!(
                settings_stub_showing(&mut app),
                "{place:?}: Comfort pause chrome visible before Places click"
            );

            let mut q = app
                .world_mut()
                .query_filtered::<Entity, With<PausePlacesBtn>>();
            let btn = q
                .iter(app.world())
                .next()
                .expect("PausePlacesBtn on pause plate");
            *app.world_mut().get_mut::<Interaction>(btn).unwrap() = Interaction::Pressed;
            app.update();
            *app.world_mut().get_mut::<Interaction>(btn).unwrap() = Interaction::None;
            app.update();

            assert!(
                app.world().resource::<PlacesPlate>().open,
                "{place:?}: Places door must open the four-room plate"
            );
            assert!(
                pause_is_open(&app),
                "{place:?}: Places click must NOT dismiss/close pause (settings_open stays)"
            );
            assert!(
                !places_row_live(&mut app),
                "{place:?}: Places row hides while the door plate is open"
            );
            assert!(
                !settings_visible_with_places(true, true),
                "{place:?}: Comfort chrome hide rule while places_open"
            );
            assert!(
                !settings_stub_showing(&mut app),
                "{place:?}: SettingsStubRoot must hide (Display::None) — no Comfort linger"
            );
            assert!(
                places_plate_showing(&mut app),
                "{place:?}: PlacesRoot four-room plate must be Visible"
            );
            let world = app.world_mut();
            assert_eq!(
                world
                    .query_filtered::<(), With<PlacesSanctuaryBtn>>()
                    .iter(world)
                    .count(),
                1
            );
            assert_eq!(
                world
                    .query_filtered::<(), With<PlacesThresholdBtn>>()
                    .iter(world)
                    .count(),
                1
            );
            assert_eq!(
                world
                    .query_filtered::<(), With<PlacesHeartwoodBtn>>()
                    .iter(world)
                    .count(),
                1
            );
            assert_eq!(
                world
                    .query_filtered::<(), With<PlacesDepthsBtn>>()
                    .iter(world)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn open_door_helper_keeps_threshold_clear() {
        let mut plate = PlacesPlate::default();
        plate.open_door();
        assert!(plate.open);
        assert!(plate.selected.is_none());
        assert!(plate.selected_room.is_none());
        assert!(!plate.confirm_pending);
        plate.selected = Some(PlaceId::Depths);
        plate.selected_room = Some("Depths");
        plate.confirm_pending = true;
        plate.close_door();
        assert!(!plate.open);
        assert!(plate.selected.is_none());
        assert!(plate.selected_room.is_none());
        assert!(!plate.confirm_pending);
    }

    /// Rung C: confirm cue is leave / enter, not a list-row teleport.
    #[test]
    fn door_confirm_cue_reads_leave_enter() {
        assert_eq!(
            door_confirm_cue(PlacesRoom::Sanctuary),
            "Leave this hex · enter Sanctuary?"
        );
        assert_eq!(
            door_confirm_cue(PlacesRoom::Heartwood),
            "Leave this hex · enter Heartwood?"
        );
        assert_eq!(
            door_confirm_cue(PlacesRoom::Threshold),
            "Leave this hex · enter Threshold?"
        );
        assert_eq!(
            door_confirm_cue(PlacesRoom::Depths),
            "Leave this hex · enter Depths?"
        );
        assert_eq!(DOOR_IDLE_CUE, "Leave this hex · enter that room");
        assert_eq!(DOOR_CONFIRM_BTN, "Leave · enter");
    }

    /// Four Offline rooms on the door; Threshold shares Heartwood's file.
    #[test]
    fn four_disk_rooms_threshold_shares_heartwood() {
        assert_eq!(PlacesRoom::Sanctuary.place_id(), PlaceId::Sanctuary);
        assert_eq!(PlacesRoom::Heartwood.place_id(), PlaceId::Heartwood);
        assert_eq!(PlacesRoom::Threshold.place_id(), PlaceId::Heartwood);
        assert_eq!(PlacesRoom::Depths.place_id(), PlaceId::Depths);
        assert_eq!(PlacesRoom::Threshold.display_name(), "Threshold");
        // No Market / fifth PlaceId on this plate.
        assert_eq!(LOCAL_HEXES.len(), 3, "disk hexes stay three; Threshold rides Heartwood");
    }

    #[test]
    fn select_threshold_pending_names_threshold_door() {
        let mut plate = PlacesPlate::default();
        select_room(&mut plate, PlaceId::Sanctuary, PlacesRoom::Threshold);
        assert!(plate.confirm_pending);
        assert_eq!(plate.selected, Some(PlaceId::Heartwood));
        assert_eq!(plate.selected_room, Some("Threshold"));
        // Already on Heartwood: Threshold is same disk room — no confirm teleport.
        select_room(&mut plate, PlaceId::Heartwood, PlacesRoom::Threshold);
        assert!(!plate.confirm_pending);
        assert!(plate.selected.is_none());
    }

    #[test]
    fn places_plate_spawns_four_room_buttons() {
        let mut app = yard_app(PlaceId::Sanctuary);
        let world = app.world_mut();
        let sanctuary = world
            .query_filtered::<(), With<PlacesSanctuaryBtn>>()
            .iter(world)
            .count();
        let heartwood = world
            .query_filtered::<(), With<PlacesHeartwoodBtn>>()
            .iter(world)
            .count();
        let threshold = world
            .query_filtered::<(), With<PlacesThresholdBtn>>()
            .iter(world)
            .count();
        let depths = world
            .query_filtered::<(), With<PlacesDepthsBtn>>()
            .iter(world)
            .count();
        assert_eq!(sanctuary, 1);
        assert_eq!(heartwood, 1);
        assert_eq!(threshold, 1, "Threshold must sit on the Places door");
        assert_eq!(depths, 1);
    }

    /// Fat-tap / lavapipe: Places deck + Confirm/Back min hit ≥44 logical dp.
    #[test]
    fn places_deck_hit_targets_meet_44dp() {
        assert!(PLACES_HIT_MIN >= 44.0);
        let mut app = yard_app(PlaceId::Sanctuary);
        let world = app.world_mut();
        let mut heights: Vec<f32> = Vec::new();
        for style in world
            .query_filtered::<&Style, With<PlacesSanctuaryBtn>>()
            .iter(world)
        {
            match style.min_height {
                Val::Px(h) => heights.push(h),
                other => panic!("Sanctuary btn min_height expected Px, got {other:?}"),
            }
        }
        for style in world
            .query_filtered::<&Style, With<PlacesHeartwoodBtn>>()
            .iter(world)
        {
            match style.min_height {
                Val::Px(h) => heights.push(h),
                other => panic!("Heartwood btn min_height expected Px, got {other:?}"),
            }
        }
        for style in world
            .query_filtered::<&Style, With<PlacesThresholdBtn>>()
            .iter(world)
        {
            match style.min_height {
                Val::Px(h) => heights.push(h),
                other => panic!("Threshold btn min_height expected Px, got {other:?}"),
            }
        }
        for style in world
            .query_filtered::<&Style, With<PlacesDepthsBtn>>()
            .iter(world)
        {
            match style.min_height {
                Val::Px(h) => heights.push(h),
                other => panic!("Depths btn min_height expected Px, got {other:?}"),
            }
        }
        for style in world
            .query_filtered::<&Style, With<PlacesConfirmBtn>>()
            .iter(world)
        {
            match style.min_height {
                Val::Px(h) => heights.push(h),
                other => panic!("Confirm btn min_height expected Px, got {other:?}"),
            }
        }
        for style in world
            .query_filtered::<&Style, With<PlacesBackBtn>>()
            .iter(world)
        {
            match style.min_height {
                Val::Px(h) => heights.push(h),
                other => panic!("Back btn min_height expected Px, got {other:?}"),
            }
        }
        assert_eq!(heights.len(), 6, "four rooms + Confirm + Back");
        for h in heights {
            assert!(
                h >= 44.0,
                "Places deck hit target min_height {h} < 44 logical dp"
            );
        }
        // PLACES-CLICK stack intact — fat-tap does not lower z or add a HUD.
        assert_eq!(PLACES_PLATE_Z, LIVED_UI_Z_PAUSE + 2);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
    }

    fn demo_bind() -> LivedHourBind {
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

    fn land_people(
        house: bool,
        tend: bool,
        people: HousePeople,
        start: PlaceId,
    ) -> (Option<PeopleLanding>, PlaceId) {
        land_people_climate(house, tend, people, start, None).0
    }

    /// CARD L4 — land plus climate hex_id (Esc→Places apply_place swap).
    fn land_people_climate(
        house: bool,
        tend: bool,
        people: HousePeople,
        start: PlaceId,
        mut presence: Option<&mut SoftPresence>,
    ) -> ((Option<PeopleLanding>, PlaceId), String) {
        let mut travel = HexTravelState { current: start };
        let mut bind = demo_bind();
        let hex_before = bind.climate.hex_id.clone();
        let mut crossed = None;
        let land = try_cross_people_door_land(
            house,
            tend,
            &mut crossed,
            people,
            &mut travel,
            &mut bind,
            None,
            presence.as_deref_mut(),
        );
        let hex_after = bind.climate.hex_id.clone();
        if land.is_none() {
            assert_eq!(hex_after, hex_before, "skip House must not swap climate");
        }
        ((land, travel.current), hex_after)
    }

    /// CARD L3 — skip House → no PlaceId change.
    #[test]
    fn skip_house_no_place_id_change() {
        let start = PlaceId::Sanctuary;
        let (land, now) = land_people(false, true, HousePeople::Human, start);
        assert!(land.is_none());
        assert_eq!(now, start);
        let (land, now) = land_people(false, true, HousePeople::Draek, PlaceId::Sanctuary);
        assert!(land.is_none());
        assert_eq!(now, PlaceId::Sanctuary);
    }

    /// CARD L3 — Human door → Sanctuary.
    #[test]
    fn human_door_lands_sanctuary() {
        let (land, now) = land_people(true, true, HousePeople::Human, PlaceId::Sanctuary);
        assert_eq!(land, Some(PeopleLanding::SanctuaryYard));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(PeopleLanding::SanctuaryYard.place_id(), PlaceId::Sanctuary);
    }

    /// CARD L3 — Cydruid door → Heartwood. C0: human-in-frame, no treant.
    #[test]
    fn cydruid_door_lands_heartwood() {
        let (land, now) = land_people(true, true, HousePeople::Cydruid, PlaceId::Sanctuary);
        assert_eq!(land, Some(PeopleLanding::Heartwood));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
        assert!(!HousePeople::Cydruid.people_line().contains("bark"));
    }

    /// CARD L3 — Quellorian door → Heartwood disk (no new PlaceId variant).
    #[test]
    fn quellorian_door_lands_heartwood_disk() {
        let (land, now) = land_people(true, true, HousePeople::Quellorian, PlaceId::Sanctuary);
        assert_eq!(land, Some(PeopleLanding::Threshold));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(PeopleLanding::Threshold.place_id(), PlaceId::Heartwood);
        assert_eq!(
            LOCAL_HEXES.len(),
            3,
            "disk hexes stay three; Threshold rides Heartwood"
        );
        match now {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
    }

    /// CARD L3 — Draek door → Depths.
    #[test]
    fn draek_door_lands_depths() {
        let (land, now) = land_people(true, true, HousePeople::Draek, PlaceId::Sanctuary);
        assert_eq!(land, Some(PeopleLanding::DepthsTealWayHome));
        assert_eq!(now, PlaceId::Depths);
        assert_eq!(
            PeopleLanding::DepthsTealWayHome.place_id(),
            PlaceId::Depths
        );
    }

    /// CARD L3 — Ambrosian door → Sanctuary (same hex as Human, not a 5th room).
    #[test]
    fn ambrosian_door_lands_sanctuary() {
        let (land, now) = land_people(true, true, HousePeople::Ambrosian, PlaceId::Sanctuary);
        assert_eq!(land, Some(PeopleLanding::SanctuaryWellFromAbove));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(
            PeopleLanding::SanctuaryWellFromAbove.place_id(),
            PlaceId::Sanctuary
        );
        assert_eq!(
            HousePeople::Ambrosian.landing().place_id(),
            HousePeople::Human.landing().place_id()
        );
        assert_ne!(
            HousePeople::Ambrosian.landing(),
            HousePeople::Human.landing()
        );
    }

    /// CARD L4 — skip House → climate / place unchanged.
    #[test]
    fn skip_house_climate_place_unchanged() {
        use crate::climate_plane::dress_token_for_place;

        let start = PlaceId::Sanctuary;
        let ((land, now), hex) =
            land_people_climate(false, true, HousePeople::Human, start, None);
        assert!(land.is_none());
        assert_eq!(now, start);
        assert_eq!(hex, "local-hex");
        assert_eq!(dress_token_for_place(now), dress_token_for_place(start));

        let ((land, now), hex) =
            land_people_climate(false, true, HousePeople::Draek, PlaceId::Sanctuary, None);
        assert!(land.is_none());
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(hex, "local-hex");
    }

    /// CARD L4 — Human land → Sanctuary dress token / PlaceId::Sanctuary.
    #[test]
    fn human_land_sanctuary_dress_token() {
        use crate::climate_plane::{dress_token_for_landing, dress_token_for_place};

        let ((land, now), hex) =
            land_people_climate(true, true, HousePeople::Human, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryYard));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(hex, PlaceId::Sanctuary.as_str());
        assert_eq!(dress_token_for_place(now), "Sanctuary Prime");
        assert_eq!(
            dress_token_for_landing(land.expect("Human land")),
            "Sanctuary Prime"
        );
    }

    /// CARD L4 — Cydruid land → Heartwood dress. C0: person stays human-in-frame.
    #[test]
    fn cydruid_land_heartwood_dress() {
        use crate::climate_plane::dress_token_for_place;

        let ((land, now), hex) =
            land_people_climate(true, true, HousePeople::Cydruid, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::Heartwood));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(hex, PlaceId::Heartwood.as_str());
        assert_eq!(dress_token_for_place(now), "Verdant Heartwood");
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
    }

    /// CARD L4 — Quellorian land → Heartwood PlaceId + Threshold shelf flag/reach.
    #[test]
    fn quellorian_land_heartwood_threshold_shelf_reach() {
        use crate::climate_plane::dress_token_for_place;
        use crate::human_presence::{people_landing_wake, SoftPresence};
        use shared::threshold_shelf::threshold_use_in_reach;

        let mut presence = SoftPresence::default();
        let ((land, now), hex) = land_people_climate(
            true,
            true,
            HousePeople::Quellorian,
            PlaceId::Sanctuary,
            Some(&mut presence),
        );
        assert_eq!(land, Some(PeopleLanding::Threshold));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(hex, PlaceId::Heartwood.as_str());
        assert_eq!(dress_token_for_place(now), "Verdant Heartwood");
        let wake = people_landing_wake(PeopleLanding::Threshold);
        assert_eq!(presence.position, wake);
        assert!(
            threshold_use_in_reach(now, presence.position.x, presence.position.z),
            "Quellorian wake must reach the existing Threshold shelf"
        );
        assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
    }

    /// CARD L4 — Draek land → Depths dress. DepthsPeaceTend restore-not-Take owns Use.
    #[test]
    fn draek_land_depths_dress() {
        use crate::climate_plane::dress_token_for_place;

        let ((land, now), hex) =
            land_people_climate(true, true, HousePeople::Draek, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::DepthsTealWayHome));
        assert_eq!(now, PlaceId::Depths);
        assert_eq!(hex, PlaceId::Depths.as_str());
        assert_eq!(dress_token_for_place(now), "Abyssal Depths");
        // DepthsPeaceTend restore-not-Take already guards on this hex_id
        // (`restore_depths_hex` in depths_landing — restore, not Take).
        assert_eq!(hex, "depths");
    }

    /// CARD L4 — Ambrosian land → Sanctuary PlaceId (same as Human). No new hex.
    #[test]
    fn ambrosian_land_sanctuary_same_as_human() {
        use crate::climate_plane::dress_token_for_landing;

        let ((land, now), hex) =
            land_people_climate(true, true, HousePeople::Ambrosian, PlaceId::Sanctuary, None);
        assert_eq!(land, Some(PeopleLanding::SanctuaryWellFromAbove));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(hex, PlaceId::Sanctuary.as_str());
        assert_eq!(
            dress_token_for_landing(PeopleLanding::SanctuaryWellFromAbove),
            dress_token_for_landing(PeopleLanding::SanctuaryYard)
        );
        assert_eq!(now, HousePeople::Human.landing().place_id());
    }

    /// CARD L7 — skip House → no PlaceId change · no beat flag.
    #[test]
    fn l7_skip_house_no_place_id_change_no_beat_flag() {
        use crate::human_presence::{arrival_beat_after_land, SoftPresence};

        let start = PlaceId::Sanctuary;
        let mut presence = SoftPresence::default();
        let ((land, now), _) =
            land_people_climate(false, true, HousePeople::Human, start, Some(&mut presence));
        assert!(land.is_none());
        assert_eq!(now, start);
        let beat = arrival_beat_after_land(land);
        assert!(!beat.armed);
        assert!(beat.landing.is_none());
        assert_eq!(presence.position, SoftPresence::default().position);

        let ((land, now), _) = land_people_climate(
            false,
            true,
            HousePeople::Draek,
            PlaceId::Sanctuary,
            None,
        );
        assert!(land.is_none());
        assert_eq!(now, PlaceId::Sanctuary);
        assert!(!arrival_beat_after_land(land).armed);
    }

    /// CARD L7 — Human land → PlaceId::Sanctuary · wake == Sanctuary yard helper.
    #[test]
    fn l7_human_land_sanctuary_yard_wake() {
        use crate::human_presence::{
            arrival_beat_after_land, people_landing_wake, sanctuary_yard_wake, SoftPresence,
        };

        let mut presence = SoftPresence::default();
        let ((land, now), hex) = land_people_climate(
            true,
            true,
            HousePeople::Human,
            PlaceId::Sanctuary,
            Some(&mut presence),
        );
        assert_eq!(land, Some(PeopleLanding::SanctuaryYard));
        assert_eq!(now, PlaceId::Sanctuary);
        assert_eq!(hex, PlaceId::Sanctuary.as_str());
        let wake = people_landing_wake(PeopleLanding::SanctuaryYard);
        assert_eq!(wake, sanctuary_yard_wake());
        assert_eq!(presence.position, wake);
        assert!(arrival_beat_after_land(land).armed);
        assert!(crate::climate_plane::arrival_garden_title_light_yields(
            PeopleLanding::SanctuaryYard
        ));
    }

    /// CARD L7 — Ambrosian land → PlaceId::Sanctuary (eq Human) · wake Y > Human wake Y.
    #[test]
    fn l7_ambrosian_land_sanctuary_wake_y_above_human() {
        use crate::first_session_guidance::SANCTUARY_WANT;
        use crate::human_presence::{
            arrival_beat_after_land, arrival_camera_ease, people_landing_wake, ArrivalCameraEase,
            SoftPresence,
        };

        let mut human_p = SoftPresence::default();
        let ((human_land, human_now), _) = land_people_climate(
            true,
            true,
            HousePeople::Human,
            PlaceId::Sanctuary,
            Some(&mut human_p),
        );
        let mut amb_p = SoftPresence::default();
        let ((amb_land, amb_now), _) = land_people_climate(
            true,
            true,
            HousePeople::Ambrosian,
            PlaceId::Sanctuary,
            Some(&mut amb_p),
        );
        assert_eq!(amb_land, Some(PeopleLanding::SanctuaryWellFromAbove));
        assert_eq!(amb_now, PlaceId::Sanctuary);
        assert_eq!(amb_now, human_now);
        assert_eq!(
            HousePeople::Ambrosian.landing().place_id(),
            HousePeople::Human.landing().place_id()
        );
        let human_wake = people_landing_wake(PeopleLanding::SanctuaryYard);
        let amb_wake = people_landing_wake(PeopleLanding::SanctuaryWellFromAbove);
        assert!(amb_wake.y > human_wake.y);
        assert_eq!(amb_p.position.y, amb_wake.y);
        assert!(amb_p.position.y > human_p.position.y);
        assert_eq!(
            arrival_camera_ease(PeopleLanding::SanctuaryWellFromAbove),
            ArrivalCameraEase::Up
        );
        assert!(arrival_beat_after_land(amb_land).armed);
        assert_eq!(
            crate::first_session_guidance::want_after_people_landing(amb_land),
            SANCTUARY_WANT
        );
        assert_eq!(human_land, Some(PeopleLanding::SanctuaryYard));
    }

    /// CARD L7 — Cydruid land → Heartwood · human-in-frame line (NOT treant).
    #[test]
    fn l7_cydruid_land_heartwood_human_in_frame() {
        use crate::human_presence::{arrival_beat_after_land, people_landing_wake, SoftPresence};

        let mut presence = SoftPresence::default();
        let ((land, now), hex) = land_people_climate(
            true,
            true,
            HousePeople::Cydruid,
            PlaceId::Sanctuary,
            Some(&mut presence),
        );
        assert_eq!(land, Some(PeopleLanding::Heartwood));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(hex, PlaceId::Heartwood.as_str());
        assert_eq!(presence.position, people_landing_wake(PeopleLanding::Heartwood));
        assert_eq!(HousePeople::Cydruid.people_line(), "Cydruid · human-in-frame");
        assert!(!HousePeople::Cydruid.people_line().contains("treant"));
        assert!(!HousePeople::Cydruid.people_line().contains("bark"));
        assert!(arrival_beat_after_land(land).armed);
        assert_eq!(
            crate::climate_plane::dress_mood_for_place(now),
            crate::climate_plane::PlaceMood::HeartwoodCanopy
        );
    }

    /// CARD L7 — Quellorian land → Heartwood · threshold_use_in_reach. No extra lift.
    #[test]
    fn l7_quellorian_land_heartwood_threshold_use_in_reach() {
        use crate::human_presence::{
            arrival_beat_after_land, arrival_camera_ease, people_landing_wake, ArrivalCameraEase,
            SoftPresence,
        };
        use shared::threshold_shelf::threshold_use_in_reach;

        let mut presence = SoftPresence::default();
        let ((land, now), hex) = land_people_climate(
            true,
            true,
            HousePeople::Quellorian,
            PlaceId::Sanctuary,
            Some(&mut presence),
        );
        assert_eq!(land, Some(PeopleLanding::Threshold));
        assert_eq!(now, PlaceId::Heartwood);
        assert_eq!(hex, PlaceId::Heartwood.as_str());
        let wake = people_landing_wake(PeopleLanding::Threshold);
        assert_eq!(presence.position, wake);
        assert_eq!(
            wake,
            Vec3::from_array(shared::threshold_shelf::THRESHOLD_SHELF_CENTER)
        );
        assert!(
            threshold_use_in_reach(now, presence.position.x, presence.position.z),
            "Quellorian wake must reach the existing Threshold shelf"
        );
        assert_eq!(arrival_camera_ease(PeopleLanding::Threshold), ArrivalCameraEase::None);
        assert!(arrival_beat_after_land(land).armed);
    }

    /// CARD L7 — Draek land → Depths. Existing wet-stone fog / teal Peace.
    #[test]
    fn l7_draek_land_depths() {
        use crate::climate_plane::{dress_mood_for_place, dress_token_for_place, PlaceMood};
        use crate::human_presence::{arrival_beat_after_land, SoftPresence};

        let mut presence = SoftPresence::default();
        let ((land, now), hex) = land_people_climate(
            true,
            true,
            HousePeople::Draek,
            PlaceId::Sanctuary,
            Some(&mut presence),
        );
        assert_eq!(land, Some(PeopleLanding::DepthsTealWayHome));
        assert_eq!(now, PlaceId::Depths);
        assert_eq!(hex, PlaceId::Depths.as_str());
        assert_eq!(dress_token_for_place(now), "Abyssal Depths");
        assert_eq!(dress_mood_for_place(now), PlaceMood::DepthsWetStone);
        assert!(arrival_beat_after_land(land).armed);
    }

    /// CARD L7 — disk hexes stay three. No fourth PlaceId.
    #[test]
    fn l7_local_hexes_stay_three() {
        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert_eq!(
            PeopleLanding::Threshold.place_id(),
            PlaceId::Heartwood
        );
        assert_eq!(
            PeopleLanding::SanctuaryWellFromAbove.place_id(),
            PlaceId::Sanctuary
        );
    }

    /// CARD L7 — Title Online stays grey.
    #[test]
    fn l7_steward_online_yes_stays_false() {
        use shared::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!shared::hex_protocol::default_client_listens());
        assert!(!PowrushNet::Off.title_online_enabled());
    }

    /// CARD S2 — decline / wrong door restores garden PlaceId · still light.
    #[test]
    fn s2_decline_wrong_door_restores_garden_place_id() {
        use crate::hour_sacred::decline_or_wrong_door;

        let start = PlaceId::Sanctuary;
        let (land, now) = land_people(true, true, HousePeople::Draek, start);
        assert_eq!(land, Some(PeopleLanding::DepthsTealWayHome));
        assert_eq!(now, PlaceId::Depths);

        let mut travel = HexTravelState { current: start };
        let mut bind = demo_bind();
        let mut crossed = None;
        let land = try_cross_people_door_land(
            true,
            true,
            &mut crossed,
            HousePeople::Draek,
            &mut travel,
            &mut bind,
            None,
            None,
        );
        assert_eq!(land, Some(PeopleLanding::DepthsTealWayHome));
        assert_eq!(travel.current, PlaceId::Depths);
        let mut pending = land;
        assert!(decline_or_wrong_door(&mut crossed, &mut pending, None));
        decline_people_door_land(&mut travel, &mut bind, None, start, &mut crossed);
        assert!(crossed.is_none());
        assert!(pending.is_none());
        assert_eq!(travel.current, start);
        assert_eq!(travel.current, PlaceId::Sanctuary);
    }

    /// CARD S2 — Skip House → no PlaceId change · not sealed.
    #[test]
    fn s2_skip_house_place_id_unchanged_not_sealed() {
        use crate::hour_sacred::{confirm_gate_seal, soul_is_light};
        use shared::local_settings::PeaceKey;

        let start = PlaceId::Sanctuary;
        let (land, now) = land_people(false, true, HousePeople::Human, start);
        assert!(land.is_none());
        assert_eq!(now, start);
        assert!(soul_is_light(None));
        assert!(confirm_gate_seal(PeaceKey::E, None).is_none());
        assert!(confirm_gate_seal(PeaceKey::Q, None).is_none());
        assert_eq!(LOCAL_HEXES.len(), 3);
    }

    /// CARD F5 — unsealed light may cross door → L7 beat armed · still unsealed until E/Q.
    #[test]
    fn f5_unsealed_light_may_cross_door_l7_beat_armed_still_unsealed_until_eq() {
        use crate::hour_sacred::{confirm_gate_seal, still_unsealed_until_eq, soul_is_light};
        use crate::human_presence::SoftPresence;
        use shared::local_settings::PeaceKey;

        let mut travel = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        let mut bind = demo_bind();
        let mut presence = SoftPresence::default();
        let mut crossed = None;
        let (land, beat) = try_unsealed_light_people_door_land(
            true,
            true,
            None,
            &mut crossed,
            HousePeople::Draek,
            &mut travel,
            &mut bind,
            None,
            Some(&mut presence),
        )
        .expect("unsealed light may take a People-door");
        assert_eq!(land, PeopleLanding::DepthsTealWayHome);
        assert_eq!(travel.current, PlaceId::Depths);
        assert!(beat.armed);
        assert_eq!(beat.landing, Some(PeopleLanding::DepthsTealWayHome));
        assert!(still_unsealed_until_eq(None));
        assert!(soul_is_light(None));
        assert!(confirm_gate_seal(PeaceKey::Digit1, Some((HousePeople::Draek, land))).is_none());
        let sealed = confirm_gate_seal(PeaceKey::E, Some((HousePeople::Draek, land))).expect("E");
        assert!(!still_unsealed_until_eq(Some(sealed)));
        assert!(!soul_is_light(Some(sealed)));

        let mut skip = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        let mut skip_bind = demo_bind();
        let mut skip_cross = None;
        assert!(try_unsealed_light_people_door_land(
            false,
            true,
            None,
            &mut skip_cross,
            HousePeople::Human,
            &mut skip,
            &mut skip_bind,
            None,
            None,
        )
        .is_none());
        assert_eq!(skip.current, PlaceId::Sanctuary);
    }

    /// CARD F5 — decline / wrong door → garden light · not sealed · PlaceId garden.
    #[test]
    fn f5_decline_wrong_door_garden_light_not_sealed_place_id_garden() {
        use crate::hour_sacred::{still_unsealed_until_eq, soul_is_light};
        use crate::human_presence::{garden_bounce_wake, SoftPresence};

        let garden = PlaceId::Sanctuary;
        let mut travel = HexTravelState { current: garden };
        let mut bind = demo_bind();
        let mut presence = SoftPresence::default();
        let mut crossed = None;
        let (land, beat) = try_unsealed_light_people_door_land(
            true,
            true,
            None,
            &mut crossed,
            HousePeople::Ambrosian,
            &mut travel,
            &mut bind,
            None,
            Some(&mut presence),
        )
        .expect("land");
        assert_eq!(land, PeopleLanding::SanctuaryWellFromAbove);
        assert!(beat.armed);
        assert!(presence.position.y > garden_bounce_wake().y);

        let mut pending = Some(land);
        assert!(bounce_wrong_door_to_garden(
            &mut travel,
            &mut bind,
            None,
            garden,
            &mut crossed,
            &mut pending,
            None,
            Some(&mut presence),
        ));
        assert!(crossed.is_none());
        assert!(pending.is_none());
        assert_eq!(travel.current, garden);
        assert_eq!(travel.current, PlaceId::Sanctuary);
        assert_eq!(presence.position, garden_bounce_wake());
        assert!(!wrong_door_bounce_arrival_beat().armed);
        assert!(soul_is_light(None));
        assert!(still_unsealed_until_eq(None));
    }

    /// CARD F5 — Peace recall does not clear seal (if sealed) / vision home for light.
    #[test]
    fn f5_peace_recall_does_not_clear_seal_vision_home_for_light() {
        use crate::hour_sacred::{peace_recall, PEACE_RECALL_VISION_HOME};

        let sealed = Some((HousePeople::Quellorian, PeopleLanding::Threshold));
        let (after, line) = peace_recall(sealed);
        assert_eq!(after, sealed);
        assert_eq!(line, PEACE_RECALL_VISION_HOME);

        let garden = PlaceId::Sanctuary;
        let mut travel = HexTravelState {
            current: PlaceId::Heartwood,
        };
        let mut bind = demo_bind();
        let mut crossed = Some(HousePeople::Quellorian);
        let mut pending = Some(PeopleLanding::Threshold);
        assert!(!bounce_wrong_door_to_garden(
            &mut travel,
            &mut bind,
            None,
            garden,
            &mut crossed,
            &mut pending,
            sealed,
            None,
        ));
        assert_eq!(travel.current, PlaceId::Heartwood);
        assert_eq!(crossed, Some(HousePeople::Quellorian));

        let (light_after, light_line) = peace_recall(None);
        assert!(light_after.is_none());
        assert_eq!(light_line, "vision home");
    }

    /// CARD F5 — PlaceId / LOCAL_HEXES len == 3.
    #[test]
    fn f5_place_id_local_hexes_len_three() {
        assert_eq!(LOCAL_HEXES.len(), 3);
        match PlaceId::Sanctuary {
            PlaceId::Sanctuary | PlaceId::Heartwood | PlaceId::Depths => {}
        }
        assert_eq!(PeopleLanding::Threshold.place_id(), PlaceId::Heartwood);
    }

    /// CARD F5 — STEWARD_ONLINE_YES false.
    #[test]
    fn f5_steward_online_yes_false() {
        use shared::persona::{ONLINE_PICKER_ENABLED, STEWARD_ONLINE_YES};

        assert!(!STEWARD_ONLINE_YES);
        assert!(!ONLINE_PICKER_ENABLED);
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!default_client_listens());
        assert!(!PowrushNet::Off.title_online_enabled());
    }

    /// CARD F5 — no fifth Place · no Title race lobby.
    #[test]
    fn f5_no_fifth_place_no_title_race_lobby() {
        use crate::hour_sacred::{f5_title_is_race_lobby, garden_roster_is_race_portrait_lobby};
        use crate::human_presence::presence_opens_race_lobby;

        assert_eq!(LOCAL_HEXES.len(), 3);
        assert!(!f5_title_is_race_lobby());
        assert!(!garden_roster_is_race_portrait_lobby());
        assert!(!presence_opens_race_lobby());
        for place in LOCAL_HEXES {
            assert_ne!(place.as_str(), "garden");
            assert_ne!(place.as_str(), "market");
        }
    }
}
