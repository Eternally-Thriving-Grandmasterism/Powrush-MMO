//! client/divine_whispers_ui.rs
//! Divine Whispers with full LUFS-aware perceptual normalization + draggable slider.
//! Sovereign local Ra-Thor audio experience.
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

#[derive(Resource, Default)]
pub struct DivineWhispersLog {
    pub entries: Vec<DivineWhisper>,
}

/// Audio settings with LUFS-aware normalization
#[derive(Resource)]
pub struct DivineAudioSettings {
    pub whisper_volume: f32,     // User slider 0.0 - 1.0
    pub target_lufs: f32,        // Desired loudness (e.g. -23.0 LUFS)
    pub measured_lufs: f32,      // Pre-measured LUFS of divine_chime.ogg
}

impl Default for DivineAudioSettings {
    fn default() -> Self {
        Self {
            whisper_volume: 0.35,
            target_lufs: -23.0,      // Common game target
            measured_lufs: -18.0,    // Example: your chime is louder than target
        }
    }
}

// Volume slider components (unchanged)
#[derive(Component)] pub struct DivineVolumeSlider;
#[derive(Component)] pub struct DivineVolumeHandle;
#[derive(Component)] pub struct DivineVolumeText;
#[derive(Component)] pub struct DivineLogPanel;
#[derive(Component)] pub struct DivineLogText;

pub struct DivineWhispersUIPlugin;

impl Plugin for DivineWhispersUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentDivineWhisper>()
            .init_resource::<DivineWhispersLog>()
            .init_resource::<DivineAudioSettings>()
            .add_systems(Startup, (spawn_divine_whisper_ui, spawn_divine_log_panel))
            .add_systems(Update, (
                update_divine_whisper_display,
                fade_out_whisper,
                update_divine_log_panel,
                handle_divine_volume_drag,
                update_divine_volume_visuals,
            ));
    }
}

// Spawn functions omitted for brevity (same as previous)

// ==================== LUFS-AWARE NORMALIZATION ====================

/// Computes gain needed to reach target LUFS from measured LUFS.
fn lufs_gain(target_lufs: f32, measured_lufs: f32) -> f32 {
    let gain_db = target_lufs - measured_lufs;
    10.0_f32.powf(gain_db / 20.0)
}

/// Full normalization: perceptual curve + LUFS compensation + user volume
fn normalize_volume(settings: &DivineAudioSettings) -> f32 {
    let user_vol = settings.whisper_volume.clamp(0.0, 1.0);
    let perceptual = user_vol.sqrt(); // perceptual curve
    let lufs_compensation = lufs_gain(settings.target_lufs, settings.measured_lufs);

    (perceptual * lufs_compensation).clamp(0.0, 1.0)
}

// ==================== VOLUME SLIDER (unchanged logic) ====================

fn handle_divine_volume_drag(...) { /* existing */ }
fn update_divine_volume_visuals(...) { /* existing */ }

// ==================== RECEIVE WITH LUFS NORMALIZATION ====================

pub fn receive_divine_whisper_from_server(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    audio_settings: &Res<DivineAudioSettings>,
) {
    show_divine_whisper(whisper.clone(), current, log, ui_query);

    let final_volume = normalize_volume(audio_settings);

    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/divine_chime.ogg"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(final_volume),
            ..default()
        },
    });

    tracing::info!(
        "[Divine] Whisper played with LUFS normalization (target: {:.1} LUFS, final vol: {:.2})",
        audio_settings.target_lufs, final_volume
    );
}
