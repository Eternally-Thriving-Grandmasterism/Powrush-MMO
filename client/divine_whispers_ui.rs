/*!
 * client/divine_whispers_ui.rs
 * Divine Whispers UI + Epiphany Feedback Reactors
 *
 * Beautiful, resonant UI for Divine Whispers and Epiphany events.
 * Production hardened with strong PATSAGi / Ra-Thor / Mercy alignment.
 */

use bevy::prelude::*;
use powrush_divine_module::DivineWhisper;
use std::time::Duration;

// Simulation events
use crate::simulation::epiphany_catalyst::EpiphanyTriggered;

// ==================== RESOURCES & COMPONENTS ====================

#[derive(Component)]
pub struct DivineWhisperUI {
    pub lifetime: Timer,
}

#[derive(Resource, Default)]
pub struct CurrentDivineWhisper {
    pub whisper: Option<DivineWhisper>,
}

#[derive(Resource, Default)]
pub struct DivineWhispersLog {
    pub entries: Vec<DivineWhisper>,
}

#[derive(Resource, Default)]
pub struct DivineAudioSettings {
    pub whisper_volume: f32,
    pub target_lufs: f32,
    pub measured_lufs: f32,
    pub compression_threshold: f32,
    pub compression_ratio: f32,
    pub knee_width: f32,
    pub auto_makeup_gain: bool,
    pub true_peak_limit: f32,
}

#[derive(Resource, Default)]
pub struct DivineLoudnessMeter {
    pub current_loudness: f32,
    pub peak_loudness: f32,
    pub decay_timer: Timer,
}

impl DivineLoudnessMeter {
    pub fn trigger(&mut self, loudness: f32) {
        let l = loudness.clamp(0.0, 1.0);
        self.current_loudness = l;
        if l > self.peak_loudness { self.peak_loudness = l; }
        self.decay_timer = Timer::new(Duration::from_millis(900), TimerMode::Once);
        self.decay_timer.reset();
    }

    pub fn update(&mut self, delta: Duration) {
        self.decay_timer.tick(delta);
        let decay = 1.0 - self.decay_timer.percent();
        self.current_loudness *= decay.max(0.05);
        if self.decay_timer.finished() { self.peak_loudness *= 0.88; }
    }
}

// UI Components
#[derive(Component)] pub struct DivineVolumeSlider;
#[derive(Component)] pub struct DivineVolumeHandle;
#[derive(Component)] pub struct DivineVolumeText;
#[derive(Component)] pub struct DivineLogPanel;
#[derive(Component)] pub struct DivineLogText;
#[derive(Component)] pub struct DivineLoudnessBar;

// ==================== PLUGIN ====================

pub struct DivineWhispersUIPlugin;

impl Plugin for DivineWhispersUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentDivineWhisper>()
            .init_resource::<DivineWhispersLog>()
            .init_resource::<DivineAudioSettings>()
            .init_resource::<DivineLoudnessMeter>()
            .add_systems(Startup, (spawn_divine_whisper_ui, spawn_divine_log_panel))
            .add_systems(Update, (
                update_divine_whisper_display,
                fade_out_whisper,
                update_divine_log_panel,
                update_loudness_meter,
                epiphany_triggered_ui_reactor,
            ));
    }
}

// ==================== SPAWN FUNCTIONS ====================

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
                    style: Style {
                        max_width: Val::Px(380.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("DivineWhisperText"),
            ));
        });
}

fn spawn_divine_log_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                bottom: Val::Px(20.0),
                width: Val::Px(380.0),
                height: Val::Px(220.0),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            background_color: Color::srgba(0.06, 0.05, 0.10, 0.9).into(),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            ..default()
        },
        DivineLogPanel,
        Name::new("DivineLogPanel"),
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("Divine Whispers Log", TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 14.0,
                color: Color::srgb(0.7, 0.85, 1.0),
            }),
            style: Style { margin: UiRect::bottom(Val::Px(8.0)), ..default() },
            ..default()
        });

        parent.spawn((
            TextBundle {
                text: Text::from_section("No whispers yet...", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 12.0,
                    color: Color::srgb(0.8, 0.82, 0.9),
                }),
                style: Style { max_width: Val::Px(360.0), ..default() },
                ..default()
            },
            DivineLogText,
        ));
    });
}

// ==================== EPIPHANY REACTOR ====================

fn epiphany_triggered_ui_reactor(
    mut epiphany_events: EventReader<EpiphanyTriggered>,
    mut current: ResMut<CurrentDivineWhisper>,
    mut log: ResMut<DivineWhispersLog>,
    mut ui_query: Query<(&mut Text, &mut DivineWhisperUI)>,
) {
    for event in epiphany_events.read() {
        let outcome = &event.outcome;

        let epiphany_message = format!(
            "✧ {} — Muscle Memory +{:.1} | Resonance +{:.0}%",
            outcome.divine_whisper_flavor,
            outcome.muscle_memory_consolidation_boost,
            outcome.intensity * 100.0
        );

        let whisper = DivineWhisper {
            message: epiphany_message,
        };

        show_divine_whisper(whisper.clone(), &mut current, &mut log, &mut ui_query);

        // Longer lifetime + golden color for epiphanies
        for (mut text, mut ui) in ui_query.iter_mut() {
            ui.lifetime = Timer::new(Duration::from_secs(12), TimerMode::Once);
            ui.lifetime.reset();
            text.sections[0].style.color = Color::srgb(1.0, 0.95, 0.6);
        }

        // TODO (future): spawn_epiphany_particles + trigger_epiphany_spatial_audio
    }
}

// ==================== CORE WHISPER SYSTEMS ====================

pub fn show_divine_whisper(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
) {
    current.whisper = Some(whisper.clone());
    log.entries.push(whisper.clone());
    if log.entries.len() > 50 { log.entries.remove(0); }

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
        let content: String = log.entries.iter().rev().take(8)
            .map(|w| format!("• {}", w.message))
            .collect::<Vec<_>>()
            .join("\n");

        text.sections[0].value = if content.is_empty() {
            "No whispers yet...".to_string()
        } else {
            content
        };
    }
}

fn update_loudness_meter(
    time: Res<Time>,
    mut meter: ResMut<DivineLoudnessMeter>,
) {
    meter.update(time.delta());
}

// Placeholder for server-received whispers
pub fn receive_divine_whisper_from_server(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
) {
    show_divine_whisper(whisper, current, log, ui_query);
}
