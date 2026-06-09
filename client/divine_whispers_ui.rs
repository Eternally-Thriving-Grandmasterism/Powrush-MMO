//! client/divine_whispers_ui.rs
//! Divine Whispers UI + log + audio with perceptual volume normalization + draggable slider.
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

#[derive(Resource, Default)]
pub struct DivineWhispersLog {
    pub entries: Vec<DivineWhisper>,
}

#[derive(Resource)]
pub struct DivineAudioSettings {
    pub whisper_volume: f32, // raw slider value 0.0 - 1.0
}

impl Default for DivineAudioSettings {
    fn default() -> Self {
        Self { whisper_volume: 0.35 }
    }
}

// === Volume Slider Components ===
#[derive(Component)]
pub struct DivineVolumeSlider;

#[derive(Component)]
pub struct DivineVolumeHandle;

#[derive(Component)]
pub struct DivineVolumeText;

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

// ==================== SPAWN UI (abbreviated for clarity) ====================

fn spawn_divine_whisper_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ... existing spawn code for floating panel ...
}

fn spawn_divine_log_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ... existing spawn code for log + slider ...
}

// ==================== CORE WHISPER LOGIC ====================

pub fn show_divine_whisper(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
) {
    // existing implementation
}

fn update_divine_whisper_display(
    current: Res<CurrentDivineWhisper>,
    mut query: Query<&mut Text, With<DivineWhisperUI>>,
) {
    // existing
}

fn fade_out_whisper(
    time: Res<Time>,
    mut query: Query<(&mut DivineWhisperUI, &mut Visibility)>,
    mut current: ResMut<CurrentDivineWhisper>,
) {
    // existing
}

fn update_divine_log_panel(
    log: Res<DivineWhispersLog>,
    mut query: Query<&mut Text, With<DivineLogText>>,
) {
    // existing
}

// ==================== PERCEPTUAL VOLUME NORMALIZATION ====================

/// Converts raw slider value (0.0–1.0) into perceptually normalized volume.
/// Square root curve makes volume changes feel natural to human hearing.
fn normalize_volume(raw: f32) -> f32 {
    raw.clamp(0.0, 1.0).sqrt()
}

// ==================== VOLUME SLIDER ====================

fn handle_divine_volume_drag(
    mut interaction_query: Query<(&Interaction, &mut Style), With<DivineVolumeHandle>>,
    mut audio_settings: ResMut<DivineAudioSettings>,
    windows: Query<&Window>,
) {
    // existing drag logic
}

fn update_divine_volume_visuals(
    audio_settings: Res<DivineAudioSettings>,
    mut text_query: Query<&mut Text, With<DivineVolumeText>>,
    mut handle_query: Query<&mut Style, With<DivineVolumeHandle>>,
) {
    // existing visual sync
}

// ==================== RECEIVE WHISPER WITH NORMALIZED AUDIO ====================

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

    // Apply perceptual normalization
    let normalized = normalize_volume(audio_settings.whisper_volume);

    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/divine_chime.ogg"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::Linear(normalized),
            ..default()
        },
    });

    tracing::info!("[Divine] Whisper — normalized audio played (raw {:.2} → {:.2})", 
        audio_settings.whisper_volume, normalized);
}
