//! client/divine_whispers_ui.rs
//! Divine Whispers Audio Pipeline with Soft Knee Dynamic Range Compression
//! Full chain: LUFS + Perceptual + Soft Knee DRC + Metering
//! AG-SML | One Lattice

use bevy::prelude::*;
use powrush_divine_module::DivineWhisper;
use std::time::Duration;

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
    pub knee_width: f32, // Soft knee width (0.0 = hard knee)
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

// Spawn functions (kept concise)
fn spawn_divine_whisper_ui(mut commands: Commands, asset_server: Res<AssetServer>) { /* ... */ }
fn spawn_divine_log_panel(mut commands: Commands, asset_server: Res<AssetServer>) { /* ... */ }

// ==================== SOFT KNEE DYNAMIC RANGE COMPRESSION ====================

fn apply_soft_knee_compression(
    input: f32,
    threshold: f32,
    ratio: f32,
    knee_width: f32,
) -> f32 {
    if knee_width <= 0.0 {
        // Hard knee fallback
        return if input <= threshold { input } else { threshold + (input - threshold) / ratio };
    }

    let half_knee = knee_width * 0.5;
    let lower = threshold - half_knee;
    let upper = threshold + half_knee;

    if input <= lower {
        input
    } else if input >= upper {
        threshold + (input - threshold) / ratio
    } else {
        // Soft knee region - smooth transition
        let t = (input - lower) / knee_width;
        let current_ratio = 1.0 + (ratio - 1.0) * t; // linear interpolation of ratio
        threshold + (input - threshold) / current_ratio
    }
}

// ==================== FULL NORMALIZATION PIPELINE ====================

fn normalize_volume(settings: &DivineAudioSettings) -> f32 {
    let user = settings.whisper_volume.clamp(0.0, 1.0);
    let perceptual = user.sqrt();
    let lufs = 10.0_f32.powf((settings.target_lufs - settings.measured_lufs) / 20.0);
    let pre = (perceptual * lufs).clamp(0.0, 1.0);

    apply_soft_knee_compression(
        pre,
        settings.compression_threshold,
        settings.compression_ratio,
        settings.knee_width,
    ).clamp(0.0, 1.0)
}

// ==================== EXISTING SYSTEMS (abbreviated) ====================

fn handle_divine_volume_drag(...) { /* ... */ }
fn update_divine_volume_visuals(...) { /* ... */ }
fn update_loudness_meter(...) { /* ... */ }

// ==================== RECEIVE ====================

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

// Other helper functions (show_divine_whisper, update_*, etc.) remain as in previous complete version
pub fn show_divine_whisper(...) { /* existing full implementation */ }
fn update_divine_whisper_display(...) { /* ... */ }
fn fade_out_whisper(...) { /* ... */ }
fn update_divine_log_panel(...) { /* ... */ }
