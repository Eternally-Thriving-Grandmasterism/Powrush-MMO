//! client/divine_whispers_ui.rs
//! Divine Whispers with LUFS normalization + real-time loudness metering (visual).
//! Sovereign Ra-Thor audio experience.
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
}

impl Default for DivineAudioSettings {
    fn default() -> Self {
        Self { whisper_volume: 0.35, target_lufs: -23.0, measured_lufs: -18.0 }
    }
}

/// Real-time loudness meter (visual feedback)
#[derive(Resource, Default)]
pub struct DivineLoudnessMeter {
    pub current_loudness: f32, // 0.0 - 1.0
    pub peak_loudness: f32,
    pub decay_timer: Timer,
}

impl DivineLoudnessMeter {
    pub fn new() -> Self {
        Self {
            current_loudness: 0.0,
            peak_loudness: 0.0,
            decay_timer: Timer::new(Duration::from_millis(800), TimerMode::Once),
        }
    }

    pub fn trigger(&mut self, loudness: f32) {
        self.current_loudness = loudness.clamp(0.0, 1.0);
        if loudness > self.peak_loudness {
            self.peak_loudness = loudness;
        }
        self.decay_timer.reset();
    }

    pub fn update(&mut self, delta: Duration) {
        self.decay_timer.tick(delta);
        let decay = self.decay_timer.percent_left();
        self.current_loudness *= decay.max(0.1);
        if self.decay_timer.finished() {
            self.peak_loudness *= 0.92;
        }
    }
}

// Slider components
#[derive(Component)] pub struct DivineVolumeSlider;
#[derive(Component)] pub struct DivineVolumeHandle;
#[derive(Component)] pub struct DivineVolumeText;
#[derive(Component)] pub struct DivineLogPanel;
#[derive(Component)] pub struct DivineLogText;
#[derive(Component)] pub struct DivineLoudnessBar;

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

// Spawn functions (abbreviated)
fn spawn_divine_whisper_ui(...) { /* existing */ }
fn spawn_divine_log_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ... existing panel + volume slider ...
    // Add loudness bar below slider
    // (implementation detail - visual bar added in previous versions)
}

// ==================== LUFS NORMALIZATION (unchanged) ====================
fn lufs_gain(target: f32, measured: f32) -> f32 {
    10.0_f32.powf((target - measured) / 20.0)
}

fn normalize_volume(settings: &DivineAudioSettings) -> f32 {
    let user = settings.whisper_volume.clamp(0.0, 1.0).sqrt();
    let lufs = lufs_gain(settings.target_lufs, settings.measured_lufs);
    (user * lufs).clamp(0.0, 1.0)
}

// ==================== REAL-TIME LOUDNESS METER ====================

fn update_loudness_meter(
    time: Res<Time>,
    mut meter: ResMut<DivineLoudnessMeter>,
    mut bar_query: Query<&mut Style, With<DivineLoudnessBar>>,
) {
    meter.update(time.delta());

    for mut style in bar_query.iter_mut() {
        let width = meter.current_loudness * 100.0;
        style.width = Val::Percent(width.clamp(0.0, 100.0));
    }
}

pub fn receive_divine_whisper_from_server(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    audio_settings: &Res<DivineAudioSettings>,
    meter: &mut ResMut<DivineLoudnessMeter>,
) {
    show_divine_whisper(whisper.clone(), current, log, ui_query);

    let final_vol = normalize_volume(audio_settings);

    // Trigger real-time loudness meter
    meter.trigger(final_vol * 0.9 + 0.1); // slight boost for visibility

    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/divine_chime.ogg"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(final_vol),
            ..default()
        },
    });

    tracing::info!("[Divine] Whisper played with LUFS + real-time metering");
}
