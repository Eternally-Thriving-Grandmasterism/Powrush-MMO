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
use crate::local_session_persist::LocalSessionPersist;

const LINE: &str = "What you nurture, nurtures all.";
const HOLD_SECS: f64 = 5.2;

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
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(28.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(420.0),
                    margin: UiRect::left(Val::Px(-210.0)),
                    padding: UiRect::axes(Val::Px(18.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgba(0.04, 0.07, 0.08, 0.82).into(),
                border_color: Color::srgba(0.70, 0.88, 0.62, 0.35).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            WhisperRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    LINE,
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.90, 0.98, 0.88),
                        ..default()
                    },
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
    clock.until = time.elapsed_seconds_f64() + HOLD_SECS;
    info!(target: "powrush::whisper", "one Lattice sentence — then silence");
}

fn update_whisper(
    clock: Res<WhisperClock>,
    time: Res<Time>,
    mut root: Query<&mut Visibility, With<WhisperRoot>>,
) {
    let show = clock.showing && time.elapsed_seconds_f64() < clock.until;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
