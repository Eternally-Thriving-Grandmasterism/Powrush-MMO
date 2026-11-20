/*!
 * Human Soft Panels — M Mercy Journey · Z Realm Travel (v21.99.3)
 *
 * M / Z taught. F2 / F3 still heard as aliases.
 *
 * PATSAGi + TOLC 8 | Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::abundance_journey_echo::AbundanceJourneyEcho;
use crate::hud_anchor_registry::{HudSlab, MERCY, REALM};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY, TITLE_TEXT_SECONDARY};
use crate::hex_travel::HexTravelState;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::soft_play_bindings;

#[derive(Resource, Debug, Default)]
pub struct HumanSoftPanels {
    pub mercy_open: bool,
    pub realm_open: bool,
}

#[derive(Component)]
struct MercySoftRoot;
#[derive(Component)]
struct MercySoftBody;
#[derive(Component)]
struct RealmSoftRoot;
#[derive(Component)]
struct RealmSoftBody;

const REALMS: [(u8, &str); 5] = [
    (0, "Sanctuary Prime"),
    (1, "Synthetic Lattice"),
    (2, "Verdant Bloom"),
    (3, "Harmonic Chorus"),
    (4, "Voidfarer Horizon"),
];

pub struct HumanSoftPanelsPlugin;

impl Plugin for HumanSoftPanelsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HumanSoftPanels>()
            .add_systems(Startup, spawn_soft_panels)
            .add_systems(
                Update,
                (
                    toggle_soft_panels,
                    digit_realm_travel,
                    update_soft_visibility,
                    update_soft_bodies,
                ),
            );
    }
}

fn spawn_soft_panels(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: MERCY.top(),
                    right: MERCY.right(),
                    width: Val::Px(360.0),
                    max_height: Val::Px(320.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.5)),
                    overflow: Overflow::clip_y(),
                    overflow_clip_margin: OverflowClipMargin::border_box(),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                BorderColor::all(TITLE_BORDER),
                crate::ui_frame::plate_frame(),
                Visibility::Hidden,
            ),
            MercySoftRoot,
            HudSlab(MERCY.id),
        ))
        .with_children(|p| {
            p.spawn((
Text::new("MY MERCY JOURNEY"),
TextFont { font_size: FontSize::Px(15.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_SECONDARY),
));
            p.spawn((
                (
Text::new("Acts of thriving will gather here"),
TextFont { font_size: FontSize::Px(13.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                MercySoftBody,
            ));
            p.spawn((
                Text::new(""),
                TextFont { font_size: FontSize::Px(12.0 / 1.2), ..default() },
                TextColor(TITLE_TEXT_PRIMARY),
                crate::skill_xp::SkillsPanelText,
            ));
            p.spawn((
Text::new("M toggle · J also opens the echo"),
TextFont { font_size: FontSize::Px(11.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_SECONDARY),
));
        });

    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: REALM.top(),
                    left: REALM.left(),
                    width: Val::Px(300.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.5)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                BorderColor::all(TITLE_BORDER),
                crate::ui_frame::plate_frame(),
                Visibility::Hidden,
            ),
            RealmSoftRoot,
            HudSlab(REALM.id),
        ))
        .with_children(|p| {
            p.spawn((
Text::new("REALM TRAVEL"),
TextFont { font_size: FontSize::Px(15.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_SECONDARY),
));
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(13.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                RealmSoftBody,
            ));
            p.spawn((
Text::new("Z toggle · 1–5 choose climate"),
TextFont { font_size: FontSize::Px(11.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_SECONDARY),
));
        });
}

pub(crate) fn toggle_soft_panels(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut panels: ResMut<HumanSoftPanels>,
) {
    if soft_play_bindings::mercy_journey_just_pressed(&keyboard) {
        panels.mercy_open = !panels.mercy_open;
    }
    if soft_play_bindings::realm_travel_just_pressed(&keyboard) {
        panels.realm_open = !panels.realm_open;
    }
}

fn digit_realm_travel(
    keyboard: Res<ButtonInput<KeyCode>>,
    panels: Res<HumanSoftPanels>,
    mut soft_realm: ResMut<SoftPlayerRealm>,
    mut echo: ResMut<AbundanceJourneyEcho>,
) {
    if !panels.realm_open {
        return;
    }
    let pick = if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Digit1) {
        Some(0u8)
    } else if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else if keyboard.just_pressed(KeyCode::Digit4) || keyboard.just_pressed(KeyCode::Digit4) {
        Some(3)
    } else if keyboard.just_pressed(KeyCode::Digit5) || keyboard.just_pressed(KeyCode::Digit5) {
        Some(4)
    } else {
        None
    };
    let Some(id) = pick else {
        return;
    };
    if soft_realm.current == Some(id) {
        return;
    }
    soft_realm.current = Some(id);
    let name = REALMS[id as usize].1;
    echo.push(
        crate::abundance_journey_echo::JourneyKind::Note,
        format!("Traveled to {name}"),
    );
    info!(target: "powrush::realm", id, name, "soft realm travel");
}

fn update_soft_visibility(
    panels: Res<HumanSoftPanels>,
    mut mercy: Query<&mut Visibility, (With<MercySoftRoot>, Without<RealmSoftRoot>)>,
    mut realm: Query<&mut Visibility, (With<RealmSoftRoot>, Without<MercySoftRoot>)>,
) {
    for mut vis in &mut mercy {
        *vis = if panels.mercy_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    for mut vis in &mut realm {
        *vis = if panels.realm_open {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// CARD FLESH-SOFT-PANEL — the realm body may name the Place.
/// Absent travel keeps the first line byte for byte.
fn realm_head(place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · 1–5 choose a climate"),
        None => "1–5 choose a climate".to_string(),
    }
}

fn update_soft_bodies(
    echo: Res<AbundanceJourneyEcho>,
    soft_realm: Res<SoftPlayerRealm>,
    travel: Option<Res<HexTravelState>>,
    mut mercy: Query<&mut Text, (With<MercySoftBody>, Without<RealmSoftBody>)>,
    mut realm: Query<&mut Text, (With<RealmSoftBody>, Without<MercySoftBody>)>,
) {
    let mercy_body = if echo.lines.is_empty() {
        "Acts of thriving will gather here".to_string()
    } else {
        echo.lines
            .iter()
            .rev()
            .take(8)
            .map(|l| format!("· {}", l.text))
            .collect::<Vec<_>>()
            .join("\n")
            + &crate::abundance_journey_echo::more_hint(echo.lines.len(), 8)
    };
    for mut text in &mut mercy {
        if text.as_str() != mercy_body {
            **text = mercy_body.clone();
        }
    }

    let current = soft_realm.current.unwrap_or(0);
    let mut realm_body = format!(
        "{}\n",
        realm_head(travel.as_ref().map(|state| state.chip_name()))
    );
    for (id, name) in REALMS {
        let mark = if id == current { ">" } else { " " };
        realm_body.push_str(&format!("{mark} [{}] {name}\n", id + 1));
    }
    for mut text in &mut realm {
        if text.as_str() != realm_body {
            **text = realm_body.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Mercy and Realm positions are the registry. Styles stay coded.
    #[test]
    fn mercy_and_realm_styles_byte_identical_to_coded_places() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_soft_panels);
        app.update();

        let mut mercy_q = app
            .world_mut()
            .query_filtered::<&Node, With<MercySoftRoot>>();
        let mercy = mercy_q.single(app.world()).unwrap().clone();
        let mercy_coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(10.0),
            right: Val::Percent(2.0),
            width: Val::Px(360.0),
            max_height: Val::Px(320.0),
            padding: UiRect::all(Val::Px(14.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            border: UiRect::all(Val::Px(1.5)),
            overflow: Overflow::clip_y(),
            overflow_clip_margin: OverflowClipMargin::border_box(),
            ..default()
        };
        assert_eq!(mercy, mercy_coded);
        assert_eq!(mercy.top, MERCY.top());
        assert_eq!(mercy.right, MERCY.right());
        assert_eq!(mercy.margin, UiRect::default());
        assert_eq!(mercy.width, Val::Px(MERCY.width));

        let mut realm_q = app
            .world_mut()
            .query_filtered::<&Node, With<RealmSoftRoot>>();
        let realm = realm_q.single(app.world()).unwrap().clone();
        let realm_coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(18.0),
            left: Val::Percent(2.0),
            width: Val::Px(300.0),
            padding: UiRect::all(Val::Px(14.0)),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(6.0),
            border: UiRect::all(Val::Px(1.5)),
            ..default()
        };
        assert_eq!(realm, realm_coded);
        assert_eq!(realm.top, REALM.top());
        assert_eq!(realm.left, REALM.left());
        assert_eq!(realm.margin, UiRect::default());
        assert_eq!(realm.width, Val::Px(REALM.width));
    }

    /// CARD FLESH-SOFT-PANEL — Place prefixes the realm head; no place keeps the line.
    #[test]
    fn realm_head_names_place_or_keeps_line() {
        let dressed = realm_head(Some("Heartwood"));
        assert_eq!(dressed, "Heartwood · 1–5 choose a climate");
        let bare = realm_head(None);
        assert_eq!(bare, "1–5 choose a climate");
        for sample in [&dressed, &bare] {
            let low = sample.to_lowercase();
            assert!(!low.contains("threshold"));
            assert!(!low.contains("gold"));
            assert!(!low.contains("market"));
            assert!(!low.contains("xp"));
        }
    }
    fn hp3_bare(assets: bool) -> App {
        let mut app = App::new(); app.add_plugins(MinimalPlugins);
        if assets { app.add_plugins(bevy::asset::AssetPlugin::default()).init_asset::<Image>(); }
        app.add_systems(Startup, spawn_soft_panels).update(); app
    }
    fn hp3_open(w: u32, h: u32, s: f32, scale: f32, mercy: bool) -> App {
        let mut app = App::new(); app.insert_resource(crate::local_settings::LocalSettingsState { inner: shared::local_settings::LocalSettings { text_scale: scale, ..shared::local_settings::LocalSettings::peace_defaults() }, dirty: false }).add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default(), bevy::input::InputPlugin, bevy::text::TextPlugin, bevy::ui::UiPlugin, crate::ui_font::UiFontPlugin)).init_asset::<Image>().init_asset::<TextureAtlasLayout>().add_message::<bevy::picking::pointer::PointerInput>().add_message::<bevy::picking::backend::PointerHits>().init_resource::<bevy::picking::hover::HoverMap>().init_resource::<bevy::picking::events::PointerState>().insert_resource(HumanSoftPanels { mercy_open: mercy, realm_open: !mercy }).insert_resource(AbundanceJourneyEcho::default()).insert_resource(SoftPlayerRealm::default()).add_systems(Startup, spawn_soft_panels).add_systems(Update, (update_soft_visibility, update_soft_bodies));
        app.world_mut().spawn((Camera2d, bevy::ui::IsDefaultUiCamera, Camera { computed: bevy::camera::ComputedCameraValues { target_info: Some(bevy::camera::RenderTargetInfo { physical_size: UVec2::new(w, h), scale_factor: s }), ..default() }, ..default() }));
        (app.update(), app.update()); app
    }
    fn hp3_root(app: &mut App, mercy: bool) -> Entity { let w = app.world_mut(); if mercy { w.query_filtered::<Entity, With<MercySoftRoot>>().single(w).unwrap() } else { w.query_filtered::<Entity, With<RealmSoftRoot>>().single(w).unwrap() } }
    #[test] fn hud_plate_hp3_a_mercy_and_realm_carry_sliced_frame() {
        for assets in [false, true] {
            let mut app = hp3_bare(assets);
            if assets { assert!(app.world().resource::<Assets<Image>>().get(crate::ui_frame::FRAME_IMAGE.id()).is_none(), "HP3-a assets"); }
            for mercy in [true, false] {
                let root = hp3_root(&mut app, mercy);
                assert_eq!(app.world().get::<ImageNode>(root).cloned().map(|i| (i.image.id(), i.visual_box, i.image_mode)), Some((crate::ui_frame::FRAME_IMAGE.id(), VisualBox::PaddingBox, NodeImageMode::Sliced(crate::ui_frame::frame_slicer()))), "HP3-a frame missing mercy={mercy} assets={assets}");
            }
        }
    }
    #[test] fn hud_plate_hp3_b_open_padding_box_covers_frame() {
        for scale in [0.85_f32, shared::local_settings::TEXT_SCALE_MAX] {
            for mercy in [true, false] {
                let mut app = hp3_open(1280, 720, 1., scale, mercy); let root = hp3_root(&mut app, mercy);
                let (n, vis) = (app.world().get::<ComputedNode>(root).unwrap(), app.world().get::<Visibility>(root).unwrap()); let pad = n.padding_box().size() * n.inverse_scale_factor;
                assert!(*vis == Visibility::Visible && pad.x >= 24. && pad.y >= 24. && (pad / crate::ui_frame::FRAME_PX as f32).min_element().min(crate::ui_frame::frame_slicer().max_corner_scale) == 1., "HP3-b mercy={mercy} {scale} {pad}");
            }
        }
    }
    #[test] fn hud_plate_hp3_c_layout_matches_without_image() {
        for (w, h, s) in [(1280u32, 720u32, 1f32), (2560, 1440, 2.)] {
            for mercy in [true, false] {
                let mut app = hp3_open(w, h, s, 1., mercy); let root = hp3_root(&mut app, mercy);
                let at = |app: &App| (app.world().get::<ComputedNode>(root).unwrap().size(), app.world().get::<bevy::ui::UiGlobalTransform>(root).unwrap().translation); let framed = at(&app);
                app.world_mut().entity_mut(root).remove::<ImageNode>(); app.update(); assert_eq!(at(&app), framed, "HP3-c mercy={mercy} {w}x{h}");
            }
        }
    }
    #[test] fn hud_plate_hp3_d_text_children_carry_no_image_and_two_roots() {
        let mut app = hp3_bare(false);
        for (mercy, kids) in [(true, 4), (false, 3)] {
            let root = hp3_root(&mut app, mercy); let w = app.world();
            assert_eq!(w.get::<Children>(root).unwrap().iter().map(|c| (w.get::<Text>(c).is_some(), w.get::<ImageNode>(c).is_some())).collect::<Vec<_>>(), vec![(true, false); kids], "HP3-d mercy={mercy}");
        }
        let w = app.world_mut(); assert_eq!(w.query::<&ImageNode>().iter(w).count(), 2, "HP3-d root count");
    }
}
