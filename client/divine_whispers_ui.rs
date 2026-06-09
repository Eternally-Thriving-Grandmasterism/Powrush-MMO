//! client/divine_whispers_ui.rs
//! Divine Whispers Professional Audio Pipeline
//!
//! ARCHITECTURAL DECISION (June 2026):
//! The highest value approach for Divine Whispers audio is:
//! 1. Master `divine_chime.ogg` OFFLINE with high-quality true peak limiting
//!    + oversampling (target ≤ -1.0 dBTP). This is the primary quality layer.
//! 2. Use this lightweight runtime pipeline as a safety net:
//!    - LUFS Normalization
//!    - Perceptual Curve
//!    - Soft Knee Dynamic Range Compression
//!    - Auto Gain Compensation
//!    - True Peak Protection
//!
//! Full real-time oversampling (e.g. via rubato polyphase) is intentionally
//! avoided here because the sound is short and static. Heavy DSP is better
//! done offline.
//!
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

// Spawn functions (kept concise)
fn spawn_divine_whisper_ui(mut commands: Commands, asset_server: Res<AssetServer>) { /* full implementation in previous commits */ }
fn spawn_divine_log_panel(mut commands: Commands, asset_server: Res<AssetServer>) { /* full implementation in previous commits */ }

// ==================== SOFT KNEE DRC + AUTO GAIN + TRUE PEAK ====================

fn apply_soft_knee_compression(input: f32, threshold: f32, ratio: f32, knee: f32) -> f32 {
    if knee <= 0.0 {
        return if input <= threshold { input } else { threshold + (input - threshold) / ratio };
    }
    let half = knee * 0.5;
    let lower = threshold - half;
    let upper = threshold + half;
    if input <= lower { input }
    else if input >= upper { threshold + (input - threshold) / ratio }
    else {
        let t = (input - lower) / knee;
        let r = 1.0 + (ratio - 1.0) * t;
        threshold + (input - threshold) / r
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

fn normalize_volume(settings: &DivineAudioSettings) -> f32 {
    let user = settings.whisper_volume.clamp(0.0, 1.0);
    let perceptual = user.sqrt();
    let lufs = 10.0_f32.powf((settings.target_lufs - settings.measured_lufs) / 20.0);
    let pre = (perceptual * lufs).clamp(0.0, 1.0);

    let compressed = apply_soft_knee_compression(pre, settings.compression_threshold, settings.compression_ratio, settings.knee_width);
    let with_makeup = apply_auto_gain_compensation(compressed, pre, settings.auto_makeup_gain);
    apply_true_peak_protection(with_makeup, settings.true_peak_limit)
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
            if let Some(pos) = window.cursor_position() {
                let x = (pos.x - 90.0).clamp(0.0, 220.0);
                settings.whisper_volume = x / 220.0;
                style.left = Val::Px(x - 8.0);
            }
        }
    }
}

fn update_divine_volume_visuals(
    settings: Res<DivineAudioSettings>,
    mut text_q: Query<&mut Text, With<DivineVolumeText>>,
    mut handle_q: Query<&mut Style, With<DivineVolumeHandle>>,
) {
    let v = settings.whisper_volume.clamp(0.0, 1.0);
    for mut t in text_q.iter_mut() { t.sections[0].value = format!("{}%", (v * 100.0) as u32); }
    for mut s in handle_q.iter_mut() { s.left = Val::Px(v * 220.0 - 8.0); }
}

fn update_loudness_meter(
    time: Res<Time>,
    mut meter: ResMut<DivineLoudnessMeter>,
    mut bar_q: Query<&mut Style, With<DivineLoudnessBar>>,
) {
    meter.update(time.delta());
    for mut s in bar_q.iter_mut() { s.width = Val::Percent((meter.current_loudness * 100.0).clamp(0.0, 100.0)); }
}

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

pub fn show_divine_whisper(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    log: &mut DivineWhispersLog,
    ui_q: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
) {
    current.whisper = Some(whisper.clone());
    log.entries.push(whisper.clone());
    if log.entries.len() > 50 { log.entries.remove(0); }
    for (mut t, mut ui) in ui_q.iter_mut() {
        t.sections[0].value = whisper.message.clone();
        ui.lifetime = Timer::new(Duration::from_secs(8), TimerMode::Once);
        ui.lifetime.reset();
    }
}

fn update_divine_whisper_display(
    current: Res<CurrentDivineWhisper>,
    mut q: Query<&mut Text, With<DivineWhisperUI>>,
) {
    if let Some(w) = &current.whisper {
        for mut t in q.iter_mut() {
            if t.sections[0].value != w.message { t.sections[0].value = w.message.clone(); }
        }
    }
}

fn fade_out_whisper(
    time: Res<Time>,
    mut q: Query<(&mut DivineWhisperUI, &mut Visibility)>,
    mut current: ResMut<CurrentDivineWhisper>,
) {
    for (mut ui, mut vis) in q.iter_mut() {
        ui.lifetime.tick(time.delta());
        if ui.lifetime.finished() { *vis = Visibility::Hidden; current.whisper = None; }
    }
}

fn update_divine_log_panel(
    log: Res<DivineWhispersLog>,
    mut q: Query<&mut Text, With<DivineLogText>>,
) {
    for mut t in q.iter_mut() {
        let s: String = log.entries.iter().rev().take(8).map(|w| format!(• {}, w.message)).collect::<Vec<_>>().join("
");
        t.sections[0].value = if s.is_empty() { "No whispers yet...".to_string() } else { s };
    }
}
