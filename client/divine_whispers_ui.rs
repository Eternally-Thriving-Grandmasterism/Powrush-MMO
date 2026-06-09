//! client/divine_whispers_ui.rs
//! Elegant, mercy-themed UI for Divine Whispers + persistent history log + subtle audio chime.
//! Fully local Ra-Thor sovereign experience.
//! AG-SML | One Lattice

use bevy::prelude::*;
use powrush_divine_module::DivineWhisper;
use std::time::Duration;

#[derive(Component)]
pub struct DivineWhisperUI {
    pub lifetime: Timer,
}

#[derive(Resource, Default)]
pub struct CurrentDivineWhisper {
    pub whisper: Option<DivineWhisper>,
}

/// Persistent history of received whispers
#[derive(Resource, Default)]
pub struct DivineWhispersLog {
    pub entries: Vec<DivineWhisper>,
}

#[derive(Component)]
pub struct DivineLogPanel;

#[derive(Component)]
pub struct DivineLogText;

pub struct DivineWhispersUIPlugin;

impl Plugin for DivineWhispersUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentDivineWhisper>()
            .init_resource::<DivineWhispersLog>()
            .add_systems(Startup, (spawn_divine_whisper_ui, spawn_divine_log_panel))
            .add_systems(Update, (
                update_divine_whisper_display,
                fade_out_whisper,
                update_divine_log_panel,
            ));
    }
}

fn spawn_divine_whisper_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    bottom: Val::Px(80.0),
                    width: Val::Px(420.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.06, 0.12, 0.85).into(),
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            DivineWhisperUI {
                lifetime: Timer::new(Duration::from_secs(8), TimerMode::Once),
            },
            Name::new("DivineWhisperPanel"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "The Lattice is silent...",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 18.0,
                            color: Color::srgb(0.95, 0.92, 1.0),
                        },
                    ),
                    style: Style { max_width: Val::Px(380.0), ..default() },
                    ..default()
                },
                Name::new("DivineWhisperText"),
            ));
        });
}

/// Persistent scrollable log panel (top-right)
fn spawn_divine_log_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    width: Val::Px(380.0),
                    height: Val::Px(220.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.04, 0.08, 0.9).into(),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            DivineLogPanel,
            Name::new("DivineWhispersLog"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Divine Whispers Log",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 16.0,
                            color: Color::srgb(0.7, 0.85, 1.0),
                        },
                    ),
                    ..default()
                },
                Name::new("LogTitle"),
            ));
            parent.spawn((
                TextBundle {
                    text: Text::default(),
                    style: Style {
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                    ..default()
                },
                DivineLogText,
                Name::new("LogContent"),
            ));
        });
}

pub fn show_divine_whisper(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
) {
    current.whisper = Some(whisper.clone());
    log.entries.push(whisper.clone());
    if log.entries.len() > 50 {
        log.entries.remove(0); // keep last 50
    }

    for (mut text, mut ui) in ui_query.iter_mut() {
        text.sections[0].value = whisper.message.clone();
        ui.lifetime = Timer::new(Duration::from_secs(8), TimerMode::Once);
        ui.lifetime.reset();
    }
}

fn update_divine_whisper_display(
    current: Res<CurrentDivineWhisper>,
    mut query: Query<&mut Text, With<DivineWhisperUI>>,
) {
    if let Some(whisper) = &current.whisper {
        for mut text in query.iter_mut() {
            if text.sections[0].value != whisper.message {
                text.sections[0].value = whisper.message.clone();
            }
        }
    }
}

fn fade_out_whisper(
    time: Res<Time>,
    mut query: Query<(&mut DivineWhisperUI, &mut Visibility)>,
    mut current: ResMut<CurrentDivineWhisper>,
) {
    for (mut ui, mut visibility) in query.iter_mut() {
        ui.lifetime.tick(time.delta());
        if ui.lifetime.finished() {
            *visibility = Visibility::Hidden;
            current.whisper = None;
        }
    }
}

fn update_divine_log_panel(
    log: Res<DivineWhispersLog>,
    mut query: Query<&mut Text, With<DivineLogText>>,
) {
    for mut text in query.iter_mut() {
        let content = log.entries
            .iter()
            .rev()
            .take(8)
            .map(|w| format!(“• {}”, w.message))
            .collect::<Vec<_>>()
            .join("
");
        text.sections[0].value = if content.is_empty() {
            "No whispers yet...".to_string()
        } else {
            content
        };
    }
}

/// Call this when receiving DivineWhisperReceived from server.
/// Plays a subtle divine chime for audio feedback.
pub fn receive_divine_whisper_from_server(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    show_divine_whisper(whisper.clone(), current, log, ui_query);

    // === Audio feedback: subtle divine chime ===
    // Place your sound file at assets/sounds/divine_chime.ogg (or .wav)
    // The chime should be soft, high, and merciful — not startling.
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/divine_chime.ogg"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(0.35), // gentle, not loud
            ..default()
        },
    });

    tracing::info!("[Divine] New whisper received — audio chime played");
}
