//! Lived-hour Compass — Slice 14 (v23.2.18)
//!
//! Tells at live W 20 and 60. Peace silent. No extra key. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::compass;

use crate::hex_travel::HexTravelState;
use crate::hud_anchor_registry::{HudSlab, COMPASS};
use crate::hour_sacred::HourSacred;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};

#[derive(Resource, Debug, Default)]
pub struct CompassYard {
    pub last: Option<&'static str>,
    pub fired: bool,
}

#[derive(Component)]
struct CompassSlabRoot;
#[derive(Component)]
struct CompassSlabText;

pub struct CompassPlugin;

impl Plugin for CompassPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CompassYard>()
            .add_systems(Startup, spawn_compass_slab)
            .add_systems(Update, (update_compass, update_compass_slab));
    }
}

fn spawn_compass_slab(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: COMPASS.bottom(),
                    right: COMPASS.right(),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG.with_alpha(1.0)),
                BorderColor::all(TITLE_BORDER.with_alpha(1.0)),
                Visibility::Hidden,
            ),
            CompassSlabRoot,
            HudSlab(COMPASS.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(""),
TextFont { font_size: FontSize::Px(14.0 / 1.2), ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                CompassSlabText,
            ));
        });
}

fn update_compass(
    hour: Res<HourSacred>,
    mut yard: ResMut<CompassYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    let line = compass::tell(&hour.session.warrant, hour.hex());
    if line.is_some() && yard.last.is_none() && !yard.fired {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstCompass,
            time.elapsed_secs_f64(),
        );
        yard.fired = true;
    }
    yard.last = line;
}

/// CARD FLESH-COMPASS-LINE — the compass tell may name the Place.
/// Absent travel keeps the tell line byte for byte.
fn compass_line(place: Option<&str>, line: &str) -> String {
    match place {
        Some(place) => format!("{place} · {line}"),
        None => line.to_string(),
    }
}

fn update_compass_slab(
    yard: Res<CompassYard>,
    travel: Option<Res<HexTravelState>>,
    mut root: Query<&mut Visibility, With<CompassSlabRoot>>,
    mut text_q: Query<&mut Text, With<CompassSlabText>>,
) {
    let show = yard.last.is_some();
    for mut vis in &mut root {
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
    }
    let Some(line) = yard.last else {
        return;
    };
    let painted = compass_line(travel.as_ref().map(|state| state.chip_name()), line);
    for mut text in &mut text_q {
        if text.as_str() != painted {
            **text = painted.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::HexFlag;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Compass position is the registry, Style is the coded literal.
    #[test]
    fn compass_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_compass_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<CompassSlabRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(92.0),
            right: Val::Px(16.0),
            width: Val::Px(420.0),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
            justify_content: JustifyContent::FlexStart,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.bottom, COMPASS.bottom());
        assert_eq!(style.right, COMPASS.right());
        assert_eq!(style.margin, UiRect::default());
        assert_eq!(style.width, Val::Px(COMPASS.width));
    }

    /// CARD VP-SLABS-REGAL-1 — the compass slab rests on the title palette: opaque
    /// TITLE_PLATE_BG plate, TITLE_BORDER rim at alpha 1, TITLE_TEXT_PRIMARY text.
    #[test]
    fn slabs_regal_compass_slab_rests_on_title_palette() {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .add_systems(Startup, spawn_compass_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<(&BorderColor, &BackgroundColor), With<CompassSlabRoot>>();
        let (border, bg) = q.single(app.world()).unwrap();
        let (border, bg) = (({
            assert_eq!(border.top, border.right, "border edges");
            assert_eq!(border.top, border.bottom, "border edges");
            assert_eq!(border.top, border.left, "border edges");
            border.top
        }).to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&TextColor, With<CompassSlabText>>();
        let txt = t.single(app.world()).unwrap().0.to_srgba();
        for (got, want, what) in [
            (bg, TITLE_PLATE_BG.to_srgba(), "plate"),
            (border, TITLE_BORDER.to_srgba(), "rim"),
            (txt, TITLE_TEXT_PRIMARY.to_srgba(), "text"),
        ] {
            assert!((got.red - want.red).abs() < 1e-6, "{what} red");
            assert!((got.green - want.green).abs() < 1e-6, "{what} green");
            assert!((got.blue - want.blue).abs() < 1e-6, "{what} blue");
            assert_eq!(got.alpha, 1.0, "{what} alpha");
        }
    }

    #[test]
    fn peace_hides_compass() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        assert_eq!(compass::tell(&hour.session.warrant, hour.hex()), None);
    }

    /// CARD FLESH-COMPASS-LINE — Place prefixes the tell; no place keeps the line.
    #[test]
    fn compass_line_names_place_or_keeps_line() {
        let line = "Compass · 20 — a cited wind";
        let dressed = compass_line(Some("Heartwood"), line);
        assert_eq!(dressed, "Heartwood · Compass · 20 — a cited wind");
        let bare = compass_line(None, line);
        assert_eq!(bare, line);
        for sample in [&dressed, &bare] {
            let low = sample.to_lowercase();
            assert!(!low.contains("threshold"));
            assert!(!low.contains("gold"));
            assert!(!low.contains("market"));
            assert!(!low.contains("xp"));
        }
    }

    /// COMPASS-NEQ-1 CP1. Real HudLayoutPlugin: a steady slab stays unmarked; the R5 yield and the Classic clean hide still win, and restore wins (M7).
    #[derive(Resource, Default)] struct CpTape(Vec<(Visibility, bool)>);
    fn cp_tape(q: Query<Ref<Visibility>, With<CompassSlabRoot>>, mut t: ResMut<CpTape>) { t.0.extend(q.iter().map(|v| (*v, v.is_changed()))); }
    #[test]
    fn compass_neq_1_cp1_steady_slab_unmarked_yield_clean_and_restore_still_win() {
        use crate::{hex_travel::PlacesPlate, hud_presets::{ActiveHudPreset, HudLayoutPlugin, HudPresetId, HudYielded}, title_screen::LaunchDoor};
        let (v, h) = (Visibility::Visible, Visibility::Hidden);
        // Actions: 0 none, 1 last = Some, 2 last = None, 3 Places open, 4 Places closed.
        let run = |preset: HudPresetId, steps: &[(u8, (Visibility, bool))]| {
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, HudLayoutPlugin)).insert_resource(ActiveHudPreset { id: Some(preset) }).insert_resource(LaunchDoor::InYard).init_resource::<CompassYard>().init_resource::<PlacesPlate>().init_resource::<CpTape>().add_systems(Startup, spawn_compass_slab).add_systems(Update, update_compass_slab).add_systems(Last, cp_tape);
            app.update(); app.update(); app.world_mut().resource_mut::<CpTape>().0.clear();
            for (i, (act, want)) in steps.iter().enumerate() {
                match act {
                    1 => app.world_mut().resource_mut::<CompassYard>().last = Some("Compass · 20"),
                    2 => app.world_mut().resource_mut::<CompassYard>().last = None,
                    3 | 4 => app.world_mut().resource_mut::<PlacesPlate>().open = *act == 3,
                    _ => {}
                }
                app.update();
                assert_eq!(*app.world_mut().query_filtered::<&Visibility, With<CompassSlabRoot>>().single(app.world()).unwrap(), want.0, "CP1 {preset:?} final Visibility, step {}", i + 1);
                if preset == HudPresetId::Classic { assert!(app.world().resource::<HudYielded>().0.is_empty(), "CP1 C: the clean hide is not R2, step {}", i + 1); }
            }
            (app.world().resource::<CpTape>().0.clone(), steps.iter().map(|s| s.1).collect::<Vec<_>>())
        };
        let m = run(HudPresetId::Management, &[(0, (h, false)), (0, (h, false)), (1, (v, true)), (0, (v, false)), (3, (h, true)), (0, (h, true)), (4, (v, true)), (0, (v, false)), (2, (h, true)), (3, (h, false)), (4, (h, false))]);
        let c = run(HudPresetId::Classic, &[(0, (h, false)), (1, (h, true)), (0, (h, true)), (2, (h, true)), (0, (h, false))]);
        assert_eq!((m.0, c.0), (m.1, c.1), "CP1 tapes (Case M, Case C): (final Visibility, is_changed) per tick");
    }
}
