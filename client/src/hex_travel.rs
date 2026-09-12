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
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hex_travel::{
    apply_travel_named, boot_place, confirm_leave, house_embassy_on_place, house_week_footer,
    places_eligible, places_row_label, read_current_named, read_hex_named, sanctuary_fresh_climate,
    BootKind, PlaceId, TravelRefuse, PLACES_TITLE,
};
use shared::pause_ledger_face::NOT_YOUR_CHARTER;
use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

use crate::embassy::EmbassyYard;
use crate::hour_sacred::HourSacred;
use crate::lived_hour_bind::LivedHourBind;
use crate::title_screen::{
    yard_after_travel, HouseLabel, LaunchDoor, TITLE_BORDER, TITLE_BTN_BG, TITLE_BTN_FG,
    TITLE_PLATE_BG, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY,
};
use crate::ui_above_world::{LivedUiPlate, LIVED_UI_Z_PAUSE};

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
                    sync_places_visibility,
                    refresh_places_labels,
                    refresh_pause_places_row,
                    places_plate_clicks,
                    pause_places_row_clicks,
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

fn apply_place(
    travel: &mut HexTravelState,
    bind: &mut LivedHourBind,
    mut embassy: Option<&mut EmbassyYard>,
    dest: PlaceId,
    book: bool,
) {
    if (dest == PlaceId::Heartwood || dest == PlaceId::Depths) && !book {
        travel.current = PlaceId::Sanctuary;
        load_sanctuary_into(bind);
        return;
    }
    travel.current = dest;
    match dest {
        PlaceId::Sanctuary => {
            load_sanctuary_into(bind);
            restore_house_embassy(embassy.as_deref_mut());
        }
        PlaceId::Heartwood => {
            let file = read_hex_named(PlaceId::Heartwood)
                .unwrap_or_else(|| shared::hex_travel::stub_hex_file(PlaceId::Heartwood));
            bind.climate = file.climate;
            bind.standing = file.standing;
            bind.refresh_climate_slab_keep_week();
            // Hex lamp_empty is climate. Leave the house EmbassyYard seated.
        }
        PlaceId::Depths => {
            let file = read_hex_named(PlaceId::Depths)
                .unwrap_or_else(|| shared::hex_travel::stub_hex_file(PlaceId::Depths));
            bind.climate = file.climate;
            bind.standing = file.standing;
            bind.refresh_climate_slab_keep_week();
        }
    }
    maybe_sum_house_week(bind);
}

fn restore_house_embassy(embassy: Option<&mut EmbassyYard>) {
    if let Some(yard) = embassy {
        if let Some(raw) = crate::hour_sacred::read_hour_two_json() {
            let pack = shared::hour_two::HourTwoPack::from_json(&raw);
            yard.embassy = house_embassy_on_place(&pack.embassy, PlaceId::Sanctuary);
        }
    }
}

fn load_sanctuary_into(bind: &mut LivedHourBind) {
    if let Some(file) = read_hex_named(PlaceId::Sanctuary) {
        bind.climate = file.climate;
        bind.standing = file.standing;
    } else if bind.climate.hex_id == PlaceId::Heartwood.as_str()
        || bind.climate.hex_id == PlaceId::Depths.as_str()
    {
        bind.climate = sanctuary_fresh_climate();
        bind.standing = shared::hex_travel::sanctuary_fresh_standing();
    } else if bind.climate.hex_id.is_empty() || bind.climate.hex_id == "local-hex" {
        bind.climate.hex_id = PlaceId::Sanctuary.as_str().into();
        bind.standing.hex_id = PlaceId::Sanctuary.as_str().into();
    }
    bind.refresh_climate_slab_keep_week();
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
                z_index: ZIndex::Global(LIVED_UI_Z_PAUSE),
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
                padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                justify_content: JustifyContent::Center,
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
        heartwood_stub_embassy, hex_file_name, new_game_writes_heartwood, places_row_or_inert,
        travel_is_disk_only, CURRENT_HEX_FILE, ISOLATION_GAMMA, LOCAL_HEXES,
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

    /// Esc pause Places row opens the four-room door — does not dismiss pause.
    #[test]
    fn pause_places_row_click_opens_four_room_plate() {
        for place in LOCAL_HEXES {
            let mut app = yard_app(place);
            tap_escape(&mut app);
            assert!(pause_is_open(&app), "{place:?}: pause open");
            assert!(places_row_live(&mut app), "{place:?}: Places door on pause plate");
            assert!(!app.world().resource::<PlacesPlate>().open);

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
}
