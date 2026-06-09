//! client/divine_whispers_ui.rs
//! Divine Whispers UI + Professional Audio Pipeline for Powrush-MMO
//!
//! ARCHITECTURAL DECISION (PATSAGi Councils – June 2026):
//! 
//! Primary Quality Layer:
//!   Master `divine_chime.ogg` OFFLINE using high-quality true peak limiting
//!   + oversampling (target ≤ -1.0 dBTP). This is the highest-leverage action.
//!
//! Runtime Safety Net (lightweight & sovereign):
//!   - LUFS Normalization
//!   - Perceptual Volume Curve
//!   - Soft Knee Dynamic Range Compression
//!   - Auto Gain Compensation (Makeup Gain)
//!   - True Peak Protection
//!
//! The `OversampledTruePeakLimiter` (using rubato) is kept as a refined prototype
//! for future dynamic/real-time generated content. It is not the default path
//! for the static chime.
//!
//! This architecture honors both technical excellence and practical wisdom.
//! AG-SML v1.0 | One Lattice

use bevy::prelude::*;
use powrush_divine_module::DivineWhisper;
use std::time::Duration;
use rubato::{Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction};

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

#[derive(Resource)]
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

impl Default for DivineAudioSettings {
    fn default() -> Self {
        Self {
            whisper_volume: 0.35,
            target_lufs: -23.0,
            measured_lufs: -18.0,
            compression_threshold: 0.75,
            compression_ratio: 3.0,
            knee_width: 0.12,
            auto_makeup_gain: true,
            true_peak_limit: -1.0,
        }
    }
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
                handle_divine_volume_drag,
                update_divine_volume_visuals,
                update_loudness_meter,
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
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    width: Val::Px(380.0),
                    height: Val::Px(280.0),
                    padding: UiRect::all(Val::Px(12.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.04, 0.08, 0.92).into(),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            DivineLogPanel,
            Name::new("DivineWhispersLog"),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Divine Whispers Log",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::srgb(0.7, 0.85, 1.0),
                    },
                ),
                ..default()
            });

            // Log content
            parent.spawn((
                TextBundle {
                    text: Text::default(),
                    style: Style { margin: UiRect::top(Val::Px(6.0)), ..default() },
                    ..default()
                },
                DivineLogText,
            ));

            // Volume slider
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(32.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
            )).with_children(|row| {
                row.spawn(TextBundle {
                    text: Text::from_section("Volume", TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.85, 0.9),
                    }),
                    style: Style { width: Val::Px(50.0), ..default() },
                    ..default()
                });

                row.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(220.0),
                            height: Val::Px(8.0),
                            margin: UiRect::horizontal(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.2, 0.2, 0.25).into(),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        ..default()
                    },
                    DivineVolumeSlider,
                )).with_children(|bar| {
                    bar.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(16.0),
                                height: Val::Px(16.0),
                                position_type: PositionType::Absolute,
                                left: Val::Px(70.0),
                                top: Val::Px(-4.0),
                                border_radius: BorderRadius::MAX,
                                ..default()
                            },
                            background_color: Color::srgb(0.6, 0.75, 1.0).into(),
                            ..default()
                        },
                        DivineVolumeHandle,
                        Interaction::default(),
                    ));
                });

                row.spawn((
                    TextBundle {
                        text: Text::from_section("35%", TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 12.0,
                            color: Color::srgb(0.7, 0.85, 1.0),
                        }),
                        style: Style { width: Val::Px(40.0), ..default() },
                        ..default()
                    },
                    DivineVolumeText,
                ));
            });

            // Loudness meter bar
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(6.0),
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.35, 0.25).into(),
                    border_radius: BorderRadius::all(Val::Px(3.0)),
                    ..default()
                },
                DivineLoudnessBar,
            ));
        });
}

// ==================== WHISPER DISPLAY LOGIC ====================

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
        let content: String = log.entries.iter().rev().take(8).map(|w| format!(• {}, w.message)).collect::<Vec<_>>().join("
");
        text.sections[0].value = if content.is_empty() { "No whispers yet...".to_string() } else { content };
    }
}

// ==================== AUDIO PROCESSING ====================

fn apply_soft_knee_compression(input: f32, threshold: f32, ratio: f32, knee_width: f32) -> f32 {
    if knee_width <= 0.0 {
        return if input <= threshold { input } else { threshold + (input - threshold) / ratio };
    }
    let half_knee = knee_width * 0.5;
    let lower = threshold - half_knee;
    let upper = threshold + half_knee;

    if input <= lower { input }
    else if input >= upper { threshold + (input - threshold) / ratio }
    else {
        let t = (input - lower) / knee_width;
        let current_ratio = 1.0 + (ratio - 1.0) * t;
        threshold + (input - threshold) / current_ratio
    }
}

fn apply_auto_gain_compensation(compressed: f32, original: f32, enabled: bool) -> f32 {
    if !enabled || original <= 0.0 { return compressed; }
    let reduction = (original - compressed).max(0.0);
    (compressed * (1.0 + reduction * 0.65)).clamp(0.0, 1.0)
}

fn apply_true_peak_protection(input: f32, limit_db: f32) -> f32 {
    let limit = 10.0_f32.powf(limit_db / 20.0);
    if input <= limit { input } else { limit + (input - limit) * 0.3 }
}

// ==================== OVERSAMPLED TRUE PEAK LIMITER (PROTOTYPE) ====================

pub struct OversampledTruePeakLimiter {
    resampler: SincFixedIn<f32>,
    oversampling_factor: usize,
}

impl OversampledTruePeakLimiter {
    pub fn new(oversampling_factor: usize) -> Self {
        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor,
            window: WindowFunction::BlackmanHarris2,
        };

        let resampler = SincFixedIn::<f32>::new(
            oversampling_factor as f64,
            2.0,
            params,
            1,
            512,
        ).expect("Failed to create resampler");

        Self { resampler, oversampling_factor }
    }

    pub fn process(&mut self, input: f32, limit: f32) -> f32 {
        if input <= limit { return input; }

        let buffer: Vec<f32> = vec![input; 64];
        let waves_in = vec![buffer];

        let waves_out = self.resampler.process(&waves_in, None).unwrap();
        let oversampled = &waves_out[0];

        let true_peak = oversampled.iter().fold(0.0_f32, |max, &v| max.max(v.abs()));

        if true_peak > limit {
            let gain_reduction = limit / true_peak;
            input * gain_reduction.sqrt()
        } else {
            input
        }
    }
}

// ==================== FULL NORMALIZATION PIPELINE ====================

fn normalize_volume(settings: &DivineAudioSettings) -> f32 {
    let user = settings.whisper_volume.clamp(0.0, 1.0);
    let perceptual = user.sqrt();
    let lufs = 10.0_f32.powf((settings.target_lufs - settings.measured_lufs) / 20.0);
    let pre = (perceptual * lufs).clamp(0.0, 1.0);

    let compressed = apply_soft_knee_compression(pre, settings.compression_threshold, settings.compression_ratio, settings.knee_width);
    let with_makeup = apply_auto_gain_compensation(compressed, pre, settings.auto_makeup_gain);

    // Final stage: Oversampled True Peak Protection (prototype)
    let mut limiter = OversampledTruePeakLimiter::new(4);
    limiter.process(with_makeup, 10.0_f32.powf(settings.true_peak_limit / 20.0))
}

// ==================== SYSTEMS ====================

fn handle_divine_volume_drag(
    mut interaction_query: Query<(&Interaction, &mut Style), With<DivineVolumeHandle>>,
    mut settings: ResMut<DivineAudioSettings>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.get_single() else { return };
    for (interaction, mut style) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed || *interaction == Interaction::Dragged {
            if let Some(cursor_pos) = window.cursor_position() {
                let relative_x = (cursor_pos.x - 90.0).clamp(0.0, 220.0);
                settings.whisper_volume = relative_x / 220.0;
                style.left = Val::Px(relative_x - 8.0);
            }
        }
    }
}

fn update_divine_volume_visuals(
    settings: Res<DivineAudioSettings>,
    mut text_query: Query<&mut Text, With<DivineVolumeText>>,
    mut handle_query: Query<&mut Style, With<DivineVolumeHandle>>,
) {
    let vol = settings.whisper_volume.clamp(0.0, 1.0);
    let percent = (vol * 100.0) as u32;

    for mut text in text_query.iter_mut() { text.sections[0].value = format!("{}%", percent); }
    for mut style in handle_query.iter_mut() { style.left = Val::Px(vol * 220.0 - 8.0); }
}

fn update_loudness_meter(
    time: Res<Time>,
    mut meter: ResMut<DivineLoudnessMeter>,
    mut bar_query: Query<&mut Style, With<DivineLoudnessBar>>,
) {
    meter.update(time.delta());
    for mut style in bar_query.iter_mut() { style.width = Val::Percent((meter.current_loudness * 100.0).clamp(0.0, 100.0)); }
}

// ==================== RECEIVE FUNCTION ====================

pub fn receive_divine_whisper_from_server(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    settings: &Res<DivineAudioSettings>,
    meter: &mut ResMut<DivineLoudnessMeter>,
) {
    show_divine_whisper(whisper.clone(), current, log, ui_query);
    let final_volume = normalize_volume(settings);
    meter.trigger(final_volume);

    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/divine_chime.ogg"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(final_volume),
            ..default()
        },
    });
}
