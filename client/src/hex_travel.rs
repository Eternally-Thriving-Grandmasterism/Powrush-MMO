//! U2 — local hex travel (disk only) + Places plate
//!
//! Places list (Sanctuary / Heartwood) after Settled + book. Confirm leave
//! writes `powrush_hex_<id>.json` via the U1 user dir and loads the other
//! place. Heartwood is a stub: lamp disk empty, same Peace E, no hanging mesh.
//! U3 spatial refuse (water / lamp disk) is shared/heartwood_lamp, not this plate.
//! Play always boots Sanctuary. Continue without the book boots Sanctuary.
//! Dedicated Places plate (LivedUiPlate / Camera2d) — not extra Settings rows,
//! so Title / pause / Settings stay above the world. Sticks cull when open.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hex_travel::{
    apply_travel_named, boot_place, confirm_leave, house_embassy_on_place, house_week_footer,
    places_eligible, places_row_label, read_current_named, read_hex_named, sanctuary_fresh_climate,
    BootKind, PlaceId, TravelRefuse, LEAVE_CONFIRM, PLACES_ROW, PLACES_TITLE,
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

#[derive(Resource, Debug, Default, Clone)]
pub struct PlacesPlate {
    pub open: bool,
    pub selected: Option<PlaceId>,
    pub confirm_pending: bool,
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
struct PlacesDepthsBtn;
#[derive(Component)]
struct PlacesConfirmBtn;
#[derive(Component)]
struct PlacesBackBtn;
#[derive(Component)]
struct PausePlacesBtn;
#[derive(Component)]
struct PausePlacesLabel;

pub struct HexTravelPlugin;

impl Plugin for HexTravelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HexTravelState>()
            .init_resource::<PlacesPlate>()
            .add_systems(Startup, (spawn_places_plate, spawn_pause_places_row_marker).chain())
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
            spawn_places_btn(p, "Depths", PlacesDepthsBtn);
            spawn_places_btn(p, LEAVE_CONFIRM, PlacesConfirmBtn);
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

/// Places row lives as its own LivedUiPlate chip under pause — not a Settings row.
/// Hidden (no layout) without Settled+book so the 720p pause plate stays the same height.
fn spawn_pause_places_row_marker(mut commands: Commands) {
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(4.0),
                    left: Val::Px(16.0),
                    width: Val::Px(140.0),
                    padding: UiRect::axes(Val::Px(10.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    display: Display::None,
                    ..default()
                },
                background_color: TITLE_BTN_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(LIVED_UI_Z_PAUSE),
                ..default()
            },
            PausePlacesBtn,
            LivedUiPlate,
            Name::new("PausePlacesRow"),
        ))
        .with_children(|b| {
            b.spawn((
                TextBundle::from_section(
                    PLACES_ROW,
                    TextStyle {
                        font_size: 14.0,
                        color: TITLE_BTN_FG,
                        ..default()
                    },
                ),
                PausePlacesLabel,
            ));
        });
}

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
    travel: Res<HexTravelState>,
    mut cue: Query<&mut Text, With<PlacesCueText>>,
) {
    if !plate.open {
        return;
    }
    let line = if plate.confirm_pending {
        match plate.selected {
            Some(PlaceId::Heartwood) => "Leave this hex · Heartwood?",
            Some(PlaceId::Sanctuary) => "Leave this hex · Sanctuary?",
            Some(PlaceId::Depths) => "Leave this hex · Depths?",
            None => LEAVE_CONFIRM,
        }
    } else {
        travel.current.chip_name()
    };
    for mut text in &mut cue {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.to_string();
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
    if !label.settings_open {
        return;
    }
    let (settled, book) = book_flags(hour.as_deref());
    if !places_eligible(settled, book) {
        return;
    }
    for i in &clicks {
        if *i == Interaction::Pressed {
            plate.open = true;
            plate.selected = None;
            plate.confirm_pending = false;
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
    depths: Query<&Interaction, (Changed<Interaction>, With<PlacesDepthsBtn>)>,
    confirm: Query<&Interaction, (Changed<Interaction>, With<PlacesConfirmBtn>)>,
    back: Query<&Interaction, (Changed<Interaction>, With<PlacesBackBtn>)>,
) {
    if !plate.open {
        return;
    }
    for i in &back {
        if *i == Interaction::Pressed {
            plate.open = false;
            plate.selected = None;
            plate.confirm_pending = false;
            return;
        }
    }
    let (settled, book) = book_flags(hour.as_deref());
    if !places_eligible(settled, book) {
        plate.open = false;
        return;
    }
    for i in &sanctuary {
        if *i == Interaction::Pressed {
            select_dest(&mut plate, travel.current, PlaceId::Sanctuary);
            return;
        }
    }
    for i in &heartwood {
        if *i == Interaction::Pressed {
            select_dest(&mut plate, travel.current, PlaceId::Heartwood);
            return;
        }
    }
    for i in &depths {
        if *i == Interaction::Pressed {
            select_dest(&mut plate, travel.current, PlaceId::Depths);
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
                    plate.open = landed.places_open;
                    plate.selected = None;
                    plate.confirm_pending = false;
                }
                Err(TravelRefuse::NotYourCharter) | Err(TravelRefuse::SamePlace) => {
                    plate.confirm_pending = false;
                    plate.selected = None;
                }
            }
            return;
        }
    }
}

fn select_dest(plate: &mut PlacesPlate, current: PlaceId, dest: PlaceId) {
    if dest == current {
        plate.selected = None;
        plate.confirm_pending = false;
        return;
    }
    plate.selected = Some(dest);
    plate.confirm_pending = true;
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
}
