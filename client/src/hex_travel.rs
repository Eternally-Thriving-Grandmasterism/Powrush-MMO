//! U2 — local hex travel (disk only) + Places plate
//!
//! Places list (Sanctuary / Heartwood) after Settled + book. Confirm leave
//! writes `powrush_hex_<id>.json` via the U1 user dir and loads the other
//! place. Heartwood is a stub: lamp disk empty, same Peace E, no hanging mesh.
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
    HouseLabel, LaunchDoor, TITLE_BORDER, TITLE_BTN_BG, TITLE_BTN_FG, TITLE_PLATE_BG,
    TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY,
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
    if dest == PlaceId::Heartwood && !book {
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
    } else if bind.climate.hex_id == PlaceId::Heartwood.as_str() {
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
    if travel.current != PlaceId::Heartwood && bind.as_ref().map(|b| b.climate.hex_id.as_str())
        != Some(PlaceId::Heartwood.as_str())
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
    mut travel: ResMut<HexTravelState>,
    mut bind: Option<ResMut<LivedHourBind>>,
    mut embassy: Option<ResMut<EmbassyYard>>,
    sanctuary: Query<&Interaction, (Changed<Interaction>, With<PlacesSanctuaryBtn>)>,
    heartwood: Query<&Interaction, (Changed<Interaction>, With<PlacesHeartwoodBtn>)>,
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
                    plate.open = false;
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
    use shared::f_book_fixture::fixture_is_not_default_door;
    use shared::hex_listen::PowrushNet;
    use shared::hex_protocol::default_client_listens;
    use shared::hex_travel::{
        heartwood_stub_embassy, hex_file_name, new_game_writes_heartwood, places_row_or_inert,
        travel_is_disk_only, CURRENT_HEX_FILE, ISOLATION_GAMMA,
    };
    use shared::space_law::HexFlag;
    use shared::user_persist::is_f_book_fixture_dir;

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
}
