//! Lived-hour Compass — Slice 14 (v23.2.18)
//!
//! Tells at live W 20 and 60. Peace silent. No extra key. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::compass;

use crate::hex_travel::HexTravelState;
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
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(92.0),
                    right: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.with_alpha(1.0).into(),
                border_color: TITLE_BORDER.with_alpha(1.0).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            CompassSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
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
            time.elapsed_seconds_f64(),
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
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    let Some(line) = yard.last else {
        return;
    };
    let painted = compass_line(travel.as_ref().map(|state| state.chip_name()), line);
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != painted {
                s.value = painted.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::HexFlag;

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
        let (border, bg) = q.single(app.world());
        let (border, bg) = (border.0.to_srgba(), bg.0.to_srgba());
        let mut t = app.world_mut().query_filtered::<&Text, With<CompassSlabText>>();
        let txt = t.single(app.world()).sections[0].style.color.to_srgba();
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
}
