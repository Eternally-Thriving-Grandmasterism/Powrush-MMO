//! client/divine_whispers_ui.rs
//! Divine Whispers with Rubato-based Oversampled True Peak Limiting (Prototype)
//! AG-SML | One Lattice

use bevy::prelude::*;
use powrush_divine_module::DivineWhisper;
use std::time::Duration;
use rubato::{Resampler, SincFixedIn, SincInterpolationType, SincInterpolationParameters, WindowFunction};

// ==================== RESOURCES ====================

#[derive(Component)] pub struct DivineWhisperUI { pub lifetime: Timer }
#[derive(Resource, Default)] pub struct CurrentDivineWhisper { pub whisper: Option<DivineWhisper> }
#[derive(Resource, Default)] pub struct DivineWhispersLog { pub entries: Vec<DivineWhisper> }

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

// Components
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

// Spawn functions
fn spawn_divine_whisper_ui(mut commands: Commands, asset_server: Res<AssetServer>) { /* ... */ }
fn spawn_divine_log_panel(mut commands: Commands, asset_server: Res<AssetServer>) { /* ... */ }

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

// ==================== FULL PIPELINE WITH OVERSAMPLING ====================

fn normalize_volume(settings: &DivineAudioSettings) -> f32 {
    let user = settings.whisper_volume.clamp(0.0, 1.0);
    let perceptual = user.sqrt();
    let lufs = 10.0_f32.powf((settings.target_lufs - settings.measured_lufs) / 20.0);
    let pre = (perceptual * lufs).clamp(0.0, 1.0);

    let compressed = apply_soft_knee_compression(
        pre,
        settings.compression_threshold,
        settings.compression_ratio,
        settings.knee_width,
    );

    let with_makeup = apply_auto_gain_compensation(compressed, pre, settings.auto_makeup_gain);

    // Oversampled True Peak Protection (prototype using rubato)
    let mut limiter = OversampledTruePeakLimiter::new(4);
    limiter.process(with_makeup, 10.0_f32.powf(settings.true_peak_limit / 20.0))
}

// Helper functions (previous implementations)
fn apply_soft_knee_compression(input: f32, threshold: f32, ratio: f32, knee: f32) -> f32 { /* ... */ }
fn apply_auto_gain_compensation(compressed: f32, original: f32, enabled: bool) -> f32 { /* ... */ }

fn handle_divine_volume_drag(...) { /* ... */ }
fn update_divine_volume_visuals(...) { /* ... */ }
fn update_loudness_meter(...) { /* ... */ }

pub fn receive_divine_whisper_from_server(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_q: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    settings: &Res<DivineAudioSettings>,
    meter: &mut ResMut<DivineLoudnessMeter>,
) {
    show_divine_whisper(whisper.clone(), current, log, ui_q);
    let vol = normalize_volume(settings);
    meter.trigger(vol);

    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/divine_chime.ogg"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(vol),
            ..default()
        },
    });
}

// Other functions
pub fn show_divine_whisper(...) { /* ... */ }
fn update_divine_whisper_display(...) { /* ... */ }
fn fade_out_whisper(...) { /* ... */ }
fn update_divine_log_panel(...) { /* ... */ }
