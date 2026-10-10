/*!
 * First Whisper — v22.3.0
 *
 * One sentence from the Lattice primer. Then silence.
 * Never a manifesto overlay.
 *
 * Source: content/rbe_onboarding_education.md — "What you nurture, nurtures all."
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hud_anchor_registry::{HudSlab, WHISPER};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};
use crate::hex_travel::HexTravelState;
use crate::local_session_persist::LocalSessionPersist;

const LINE: &str = "What you nurture, nurtures all.";
const HOLD_SECS: f64 = 5.2;

/// CARD FLESH-WHISPER-PLACE — the one Lattice sentence may name the Place.
/// Absent travel keeps `LINE` byte for byte. One sentence, then silence.
fn whisper_line(place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {LINE}"),
        None => LINE.to_string(),
    }
}

#[derive(Resource, Debug, Default)]
struct WhisperClock {
    until: f64,
    showing: bool,
}

#[derive(Component)]
struct WhisperRoot;
#[derive(Component)]
struct WhisperText;

pub struct FirstWhisperPlugin;

impl Plugin for FirstWhisperPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WhisperClock>()
            .add_systems(Startup, spawn_whisper)
            .add_systems(Update, (maybe_speak, update_whisper));
    }
}

fn spawn_whisper(mut commands: Commands) {
    commands
        .spawn((
            (
                Node {
                    position_type: PositionType::Absolute,
                    top: WHISPER.top(),
                    left: WHISPER.left(),
                    width: Val::Px(420.0),
                    margin: WHISPER.margin(),
                    padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(TITLE_PLATE_BG),
                BorderColor(TITLE_BORDER),
                Visibility::Hidden,
            ),
            WhisperRoot,
            HudSlab(WHISPER.id),
        ))
        .with_children(|p| {
            p.spawn((
                (
Text::new(LINE),
TextFont { font_size: 18.0 / 1.2, ..default() },
TextColor(TITLE_TEXT_PRIMARY),
),
                WhisperText,
            ));
        });
}

fn maybe_speak(
    harvest: Res<FirstHarvestEpiphany>,
    mut persist: ResMut<LocalSessionPersist>,
    mut clock: ResMut<WhisperClock>,
    time: Res<Time>,
    travel: Option<Res<HexTravelState>>,
    mut whisper: Query<&mut Text, With<WhisperText>>,
) {
    if persist.whisper_lived || clock.showing {
        return;
    }
    if !harvest.first_harvest_lived {
        return;
    }
    persist.whisper_lived = true;
    persist.dirty = true;
    clock.showing = true;
    clock.until = time.elapsed_secs_f64() + HOLD_SECS;
    if let Some(state) = travel.as_ref() {
        let dressed = whisper_line(Some(state.chip_name()));
        for mut text in &mut whisper {
            **text = dressed.clone();
        }
    }
    info!(target: "powrush::whisper", "one Lattice sentence — then silence");
}

fn update_whisper(
    clock: Res<WhisperClock>,
    time: Res<Time>,
    mut root: Query<&mut Visibility, With<WhisperRoot>>,
) {
    let show = clock.showing && time.elapsed_secs_f64() < clock.until;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::hex_travel::PlaceId;

    /// CARD HUD-ANCHOR-REGISTRY-2B — Whisper position is the registry, Style is the coded literal.
    #[test]
    fn whisper_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_whisper);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Node, With<WhisperRoot>>();
        let style = q.single(app.world()).unwrap().clone();
        let coded = Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(28.0),
            left: Val::Percent(50.0),
            width: Val::Px(420.0),
            margin: UiRect::left(Val::Px(-210.0)),
            padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)),
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, WHISPER.top());
        assert_eq!(style.left, WHISPER.left());
        assert_eq!(style.margin, WHISPER.margin());
        assert_eq!(style.width, Val::Px(WHISPER.width));
    }

    /// CARD FLESH-WHISPER-PLACE — Place prefixes the sentence; no travel keeps LINE.
    #[test]
    fn flesh_whisper_place_names_chip_or_keeps_line() {
        assert_eq!(whisper_line(None), LINE);
        assert_eq!(whisper_line(None), "What you nurture, nurtures all.");

        let travel = HexTravelState {
            current: PlaceId::Sanctuary,
        };
        let dressed = whisper_line(Some(travel.chip_name()));
        assert_eq!(dressed, "Sanctuary Prime · What you nurture, nurtures all.");
        assert!(dressed.ends_with(LINE));
        let low = dressed.to_lowercase();
        assert!(!low.contains("threshold"));
        assert!(!low.contains("gold"));
        assert!(!low.contains("market"));
        assert!(!low.contains("xp"));
    }
}
