/*!
 * client/realtime_audio_synthesis.rs
 * Powrush-MMO — Real-time Procedural Audio Synthesis + Persistent Recall
 *
 * Capabilities:
 *   1. Synthesize short mercy-aligned audio moments in real time from recipes
 *   2. Persist catalog + optional rendered WAV under local player data dir
 *   3. Recall / replay by id (regenerate from recipe if file missing)
 *   4. Accept pre-made asset registrations into the same catalog
 *   5. Emit intents for optional server-side catalog sync
 *
 * No external sample bank required for procedural path.
 * Works offline; server sync is additive.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Permanent PATSAGi Councils | Contact: info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// ─── Types (mirrored from shared::audio_moments for client autonomy) ─────────
// When workspace shared path is fully wired, prefer:
//   use shared::audio_moments::*;
// For now the types live here in lockstep with shared/src/audio_moments.rs.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioMomentSource {
    RealtimeSynthesis,
    PremadeAsset,
    RecipeRecall,
    ExternalImport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioMomentFlavor {
    DivineWhisper,
    CouncilBloom,
    EpiphanyChime,
    MercyResonance,
    AmbientPad,
    TransitionStinger,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WaveformKind {
    Sine,
    Triangle,
    SoftSquare,
    NoiseBurst,
    HarmonicStack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSynthesisRecipe {
    pub waveform: WaveformKind,
    pub frequency_hz: f32,
    pub duration_secs: f32,
    pub sample_rate: u32,
    pub amplitude: f32,
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub partial_hz: f32,
    pub partial_amp: f32,
    pub brightness: f32,
    pub valence: f32,
    pub seed: u64,
}

impl Default for AudioSynthesisRecipe {
    fn default() -> Self {
        Self {
            waveform: WaveformKind::HarmonicStack,
            frequency_hz: 528.0,
            duration_secs: 1.6,
            sample_rate: 48_000,
            amplitude: 0.55,
            attack: 0.02,
            decay: 0.18,
            sustain: 0.55,
            release: 0.55,
            partial_hz: 792.0,
            partial_amp: 0.22,
            brightness: 0.72,
            valence: 0.75,
            seed: 0,
        }
    }
}

impl AudioSynthesisRecipe {
    pub fn divine_chime(valence: f32, seed: u64) -> Self {
        Self {
            waveform: WaveformKind::HarmonicStack,
            frequency_hz: 528.0 + valence * 40.0,
            duration_secs: 1.4 + valence * 0.6,
            amplitude: 0.5 + valence * 0.2,
            attack: 0.015,
            decay: 0.15,
            sustain: 0.5,
            release: 0.5 + valence * 0.3,
            partial_hz: 792.0,
            partial_amp: 0.18 + valence * 0.12,
            brightness: 0.65 + valence * 0.25,
            valence,
            seed,
            ..Default::default()
        }
    }

    pub fn council_bloom(intensity: f32, seed: u64) -> Self {
        Self {
            waveform: WaveformKind::HarmonicStack,
            frequency_hz: 396.0 + intensity * 80.0,
            duration_secs: 1.8 + intensity * 0.8,
            amplitude: 0.48 + intensity * 0.25,
            attack: 0.04,
            decay: 0.25,
            sustain: 0.6,
            release: 0.7,
            partial_hz: 594.0,
            partial_amp: 0.25,
            brightness: 0.7,
            valence: intensity.clamp(0.0, 1.0),
            seed,
            ..Default::default()
        }
    }

    pub fn epiphany_stinger(intensity: f32, seed: u64) -> Self {
        Self {
            waveform: WaveformKind::Sine,
            frequency_hz: 880.0 + intensity * 120.0,
            duration_secs: 0.55 + intensity * 0.35,
            amplitude: 0.45,
            attack: 0.005,
            decay: 0.08,
            sustain: 0.35,
            release: 0.25,
            partial_hz: 1320.0,
            partial_amp: 0.15,
            brightness: 0.85,
            valence: intensity.clamp(0.0, 1.0),
            seed,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMoment {
    pub id: u64,
    pub owner_player_id: u64,
    pub title: String,
    pub flavor: AudioMomentFlavor,
    pub source: AudioMomentSource,
    pub created_at_unix: u64,
    pub context: String,
    pub recipe: AudioSynthesisRecipe,
    pub rendered_path: Option<String>,
    pub favorite: bool,
    pub play_count: u32,
    pub mercy_seal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Resource)]
pub struct AudioMomentCatalog {
    pub schema: String,
    pub owner_player_id: u64,
    pub moments: HashMap<u64, AudioMoment>,
    pub next_id: u64,
    pub last_synced_unix: u64,
}

impl AudioMomentCatalog {
    pub fn new(owner_player_id: u64) -> Self {
        Self {
            schema: "audio_moment_catalog_v1".into(),
            owner_player_id,
            moments: HashMap::new(),
            next_id: 1,
            last_synced_unix: 0,
        }
    }

    pub fn insert(&mut self, mut moment: AudioMoment) -> u64 {
        if moment.id == 0 {
            moment.id = self.next_id;
            self.next_id += 1;
        } else if moment.id >= self.next_id {
            self.next_id = moment.id + 1;
        }
        let id = moment.id;
        self.moments.insert(id, moment);
        id
    }
}

// ─── Events ──────────────────────────────────────────────────────────────────

#[derive(Event, Clone, Debug)]
pub struct SynthesizeAudioMoment {
    pub title: String,
    pub flavor: AudioMomentFlavor,
    pub recipe: AudioSynthesisRecipe,
    pub context: String,
    /// If true, also write WAV and catalog entry
    pub persist: bool,
    /// If true, request server catalog sync after save
    pub sync_server: bool,
}

#[derive(Event, Clone, Debug)]
pub struct RecallAudioMoment {
    pub moment_id: u64,
}

#[derive(Event, Clone, Debug)]
pub struct RegisterPremadeAudio {
    pub title: String,
    pub flavor: AudioMomentFlavor,
    pub asset_path: String,
    pub context: String,
}

#[derive(Event, Clone, Debug)]
pub struct AudioMomentSaved {
    pub moment_id: u64,
    pub path: Option<String>,
}

#[derive(Event, Clone, Debug)]
pub struct AudioMomentServerSyncRequest {
    pub moment: AudioMoment,
}

// ─── Runtime state ───────────────────────────────────────────────────────────

#[derive(Resource)]
pub struct RealtimeAudioConfig {
    pub enabled: bool,
    pub local_root: PathBuf,
    pub auto_persist: bool,
    pub master_gain: f32,
}

impl Default for RealtimeAudioConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            local_root: PathBuf::from("player_data/audio_moments"),
            auto_persist: true,
            master_gain: 0.85,
        }
    }
}

#[derive(Resource, Default)]
pub struct RealtimeAudioUiState {
    pub show_panel: bool,
    pub status: String,
    pub last_recalled_id: Option<u64>,
}

// ─── Plugin ──────────────────────────────────────────────────────────────────

pub struct RealtimeAudioSynthesisPlugin;

impl Plugin for RealtimeAudioSynthesisPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioMomentCatalog>()
            .init_resource::<RealtimeAudioConfig>()
            .init_resource::<RealtimeAudioUiState>()
            .add_event::<SynthesizeAudioMoment>()
            .add_event::<RecallAudioMoment>()
            .add_event::<RegisterPremadeAudio>()
            .add_event::<AudioMomentSaved>()
            .add_event::<AudioMomentServerSyncRequest>()
            .add_systems(Startup, load_local_catalog)
            .add_systems(
                Update,
                (
                    toggle_audio_panel,
                    handle_synthesize_events,
                    handle_recall_events,
                    handle_premade_registration,
                    audio_moments_egui,
                ),
            );
    }
}

// ─── Pure synthesis core ─────────────────────────────────────────────────────

fn lcg(seed: &mut u64) -> f32 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    ((*seed >> 33) as f32 / u32::MAX as f32) * 2.0 - 1.0
}

fn sample_wave(kind: WaveformKind, phase: f32, seed: &mut u64) -> f32 {
    match kind {
        WaveformKind::Sine => (phase * std::f32::consts::TAU).sin(),
        WaveformKind::Triangle => {
            let t = phase.fract();
            4.0 * (t - 0.5).abs() - 1.0
        }
        WaveformKind::SoftSquare => {
            let s = (phase * std::f32::consts::TAU).sin();
            s.signum() * s.abs().powf(0.35)
        }
        WaveformKind::NoiseBurst => lcg(seed) * 0.7,
        WaveformKind::HarmonicStack => {
            let p = phase * std::f32::consts::TAU;
            (p.sin() + 0.45 * (2.0 * p).sin() + 0.2 * (3.0 * p).sin()) / 1.65
        }
    }
}

fn adsr_gain(t: f32, dur: f32, a: f32, d: f32, s: f32, r: f32) -> f32 {
    if t < a {
        if a <= 1e-6 {
            1.0
        } else {
            t / a
        }
    } else if t < a + d {
        let x = (t - a) / d.max(1e-6);
        1.0 + (s - 1.0) * x
    } else if t < (dur - r).max(a + d) {
        s
    } else {
        let x = (dur - t) / r.max(1e-6);
        (s * x).max(0.0)
    }
}

/// Render recipe → mono f32 PCM samples
pub fn synthesize_pcm(recipe: &AudioSynthesisRecipe) -> Vec<f32> {
    let sr = recipe.sample_rate.max(8_000) as f32;
    let n = ((recipe.duration_secs.max(0.05) * sr) as usize).max(1);
    let mut samples = Vec::with_capacity(n);
    let mut seed = recipe.seed.max(1);
    let mut phase = 0.0f32;
    let mut phase2 = 0.0f32;
    let mut lp = 0.0f32;
    let freq = recipe.frequency_hz.max(20.0);
    let partial = recipe.partial_hz;
    let bright = recipe.brightness.clamp(0.05, 1.0);

    for i in 0..n {
        let t = i as f32 / sr;
        let env = adsr_gain(
            t,
            recipe.duration_secs,
            recipe.attack,
            recipe.decay,
            recipe.sustain.clamp(0.0, 1.0),
            recipe.release,
        );

        let mut s = sample_wave(recipe.waveform, phase, &mut seed);
        if partial > 1.0 && recipe.partial_amp > 0.0 {
            s += sample_wave(recipe.waveform, phase2, &mut seed) * recipe.partial_amp;
        }

        // Simple one-pole low-pass controlled by brightness
        lp = lp + bright * (s - lp);
        s = lp;

        // Soft valence shimmer
        let shimmer = 1.0 + 0.04 * recipe.valence * (t * 6.0).sin();
        let out = (s * env * recipe.amplitude.clamp(0.0, 1.0) * shimmer).clamp(-1.0, 1.0);
        samples.push(out);

        phase = (phase + freq / sr).fract();
        if partial > 1.0 {
            phase2 = (phase2 + partial / sr).fract();
        }
    }
    samples
}

/// Write mono f32 samples as 16-bit PCM WAV
pub fn write_wav_mono(path: &Path, samples: &[f32], sample_rate: u32) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = fs::File::create(path)?;
    let num_samples = samples.len() as u32;
    let data_size = num_samples * 2;
    let file_size = 36 + data_size;

    // RIFF header
    file.write_all(b"RIFF")?;
    file.write_all(&file_size.to_le_bytes())?;
    file.write_all(b"WAVE")?;
    // fmt chunk
    file.write_all(b"fmt ")?;
    file.write_all(&16u32.to_le_bytes())?; // chunk size
    file.write_all(&1u16.to_le_bytes())?; // PCM
    file.write_all(&1u16.to_le_bytes())?; // mono
    file.write_all(&sample_rate.to_le_bytes())?;
    let byte_rate = sample_rate * 2;
    file.write_all(&byte_rate.to_le_bytes())?;
    file.write_all(&2u16.to_le_bytes())?; // block align
    file.write_all(&16u16.to_le_bytes())?; // bits
    // data chunk
    file.write_all(b"data")?;
    file.write_all(&data_size.to_le_bytes())?;
    for &s in samples {
        let i = (s.clamp(-1.0, 1.0) * 32767.0) as i16;
        file.write_all(&i.to_le_bytes())?;
    }
    Ok(())
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn catalog_path(cfg: &RealtimeAudioConfig) -> PathBuf {
    cfg.local_root.join("catalog.json")
}

fn moment_wav_path(cfg: &RealtimeAudioConfig, id: u64) -> PathBuf {
    cfg.local_root.join("rendered").join(format!("moment_{}.wav", id))
}

fn save_catalog(cfg: &RealtimeAudioConfig, catalog: &AudioMomentCatalog) {
    let path = catalog_path(cfg);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(catalog) {
        let tmp = path.with_extension("json.tmp");
        if fs::write(&tmp, json).is_ok() {
            let _ = fs::rename(&tmp, &path);
        }
    }
}

fn load_local_catalog(mut catalog: ResMut<AudioMomentCatalog>, cfg: Res<RealtimeAudioConfig>) {
    let path = catalog_path(&cfg);
    if path.exists() {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(loaded) = serde_json::from_str::<AudioMomentCatalog>(&data) {
                *catalog = loaded;
                info!(
                    target: "powrush::audio",
                    moments = catalog.moments.len(),
                    "Loaded local audio moment catalog"
                );
                return;
            }
        }
    }
    if catalog.schema.is_empty() {
        *catalog = AudioMomentCatalog::new(0);
    }
}

// ─── Event handlers ──────────────────────────────────────────────────────────

fn handle_synthesize_events(
    mut events: EventReader<SynthesizeAudioMoment>,
    mut catalog: ResMut<AudioMomentCatalog>,
    cfg: Res<RealtimeAudioConfig>,
    mut saved: EventWriter<AudioMomentSaved>,
    mut sync_req: EventWriter<AudioMomentServerSyncRequest>,
    mut ui: ResMut<RealtimeAudioUiState>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if !cfg.enabled {
        return;
    }

    for ev in events.read() {
        let pcm = synthesize_pcm(&ev.recipe);
        let now = unix_now();

        let mut moment = AudioMoment {
            id: 0,
            owner_player_id: catalog.owner_player_id,
            title: ev.title.clone(),
            flavor: ev.flavor,
            source: AudioMomentSource::RealtimeSynthesis,
            created_at_unix: now,
            context: ev.context.clone(),
            recipe: ev.recipe.clone(),
            rendered_path: None,
            favorite: false,
            play_count: 0,
            mercy_seal: true,
        };

        let id = if ev.persist || cfg.auto_persist {
            let id = catalog.insert(moment.clone());
            if let Some(m) = catalog.moments.get_mut(&id) {
                m.id = id;
            }
            let wav = moment_wav_path(&cfg, id);
            match write_wav_mono(&wav, &pcm, ev.recipe.sample_rate) {
                Ok(()) => {
                    if let Some(m) = catalog.moments.get_mut(&id) {
                        m.rendered_path = Some(wav.to_string_lossy().into_owned());
                        moment = m.clone();
                    }
                    save_catalog(&cfg, &catalog);
                    saved.send(AudioMomentSaved {
                        moment_id: id,
                        path: Some(wav.to_string_lossy().into_owned()),
                    });
                    ui.status = format!("Synthesized + saved moment #{} — {}", id, ev.title);

                    // Best-effort play via Bevy asset path if available
                    let handle: Handle<AudioSource> = asset_server.load(wav.to_string_lossy().as_ref());
                    commands.spawn(AudioBundle {
                        source: handle,
                        settings: PlaybackSettings::DESPAWN.with_volume(
                            bevy::audio::Volume::new(cfg.master_gain * ev.recipe.amplitude),
                        ),
                        ..default()
                    });
                }
                Err(e) => {
                    ui.status = format!("Synth ok, WAV write failed: {}", e);
                    save_catalog(&cfg, &catalog);
                }
            }

            if ev.sync_server {
                if let Some(m) = catalog.moments.get(&id) {
                    sync_req.send(AudioMomentServerSyncRequest { moment: m.clone() });
                }
            }
            id
        } else {
            // Ephemeral: play only, no catalog
            ui.status = format!("Ephemeral synth: {}", ev.title);
            0
        };

        info!(
            target: "powrush::audio",
            id,
            title = %ev.title,
            samples = pcm.len(),
            "Realtime audio moment synthesized"
        );
    }
}

fn handle_recall_events(
    mut events: EventReader<RecallAudioMoment>,
    mut catalog: ResMut<AudioMomentCatalog>,
    cfg: Res<RealtimeAudioConfig>,
    mut ui: ResMut<RealtimeAudioUiState>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for ev in events.read() {
        let Some(moment) = catalog.moments.get(&ev.moment_id).cloned() else {
            ui.status = format!("Recall failed — moment #{} not found", ev.moment_id);
            continue;
        };

        // Prefer rendered file; else regenerate from recipe
        let play_path = if let Some(ref p) = moment.rendered_path {
            let path = PathBuf::from(p);
            if path.exists() {
                path
            } else {
                // Regenerate
                let pcm = synthesize_pcm(&moment.recipe);
                let wav = moment_wav_path(&cfg, moment.id);
                let _ = write_wav_mono(&wav, &pcm, moment.recipe.sample_rate);
                if let Some(m) = catalog.moments.get_mut(&moment.id) {
                    m.rendered_path = Some(wav.to_string_lossy().into_owned());
                    m.source = AudioMomentSource::RecipeRecall;
                }
                save_catalog(&cfg, &catalog);
                wav
            }
        } else {
            let pcm = synthesize_pcm(&moment.recipe);
            let wav = moment_wav_path(&cfg, moment.id);
            let _ = write_wav_mono(&wav, &pcm, moment.recipe.sample_rate);
            if let Some(m) = catalog.moments.get_mut(&moment.id) {
                m.rendered_path = Some(wav.to_string_lossy().into_owned());
                m.source = AudioMomentSource::RecipeRecall;
            }
            save_catalog(&cfg, &catalog);
            wav
        };

        if let Some(m) = catalog.moments.get_mut(&ev.moment_id) {
            m.play_count = m.play_count.saturating_add(1);
        }
        save_catalog(&cfg, &catalog);

        let handle: Handle<AudioSource> = asset_server.load(play_path.to_string_lossy().as_ref());
        commands.spawn(AudioBundle {
            source: handle,
            settings: PlaybackSettings::DESPAWN
                .with_volume(bevy::audio::Volume::new(cfg.master_gain)),
            ..default()
        });

        ui.last_recalled_id = Some(ev.moment_id);
        ui.status = format!("Recalled #{} — {}", moment.id, moment.title);
        info!(target: "powrush::audio", id = ev.moment_id, "Audio moment recalled");
    }
}

fn handle_premade_registration(
    mut events: EventReader<RegisterPremadeAudio>,
    mut catalog: ResMut<AudioMomentCatalog>,
    cfg: Res<RealtimeAudioConfig>,
    mut ui: ResMut<RealtimeAudioUiState>,
    mut saved: EventWriter<AudioMomentSaved>,
) {
    for ev in events.read() {
        let moment = AudioMoment {
            id: 0,
            owner_player_id: catalog.owner_player_id,
            title: ev.title.clone(),
            flavor: ev.flavor,
            source: AudioMomentSource::PremadeAsset,
            created_at_unix: unix_now(),
            context: ev.context.clone(),
            recipe: AudioSynthesisRecipe::default(), // placeholder; playback uses path
            rendered_path: Some(ev.asset_path.clone()),
            favorite: false,
            play_count: 0,
            mercy_seal: true,
        };
        let id = catalog.insert(moment);
        save_catalog(&cfg, &catalog);
        saved.send(AudioMomentSaved {
            moment_id: id,
            path: Some(ev.asset_path.clone()),
        });
        ui.status = format!("Registered pre-made #{} — {}", id, ev.title);
    }
}

// ─── UI ──────────────────────────────────────────────────────────────────────

fn toggle_audio_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<RealtimeAudioUiState>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        ui_state.show_panel = !ui_state.show_panel;
    }
}

fn audio_moments_egui(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<RealtimeAudioUiState>,
    catalog: Res<AudioMomentCatalog>,
    mut synth_events: EventWriter<SynthesizeAudioMoment>,
    mut recall_events: EventWriter<RecallAudioMoment>,
) {
    if !ui_state.show_panel {
        return;
    }

    let ctx = contexts.ctx_mut();
    egui::Window::new("🎵 Audio Moments")
        .default_pos([60.0, 80.0])
        .default_size([400.0, 480.0])
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Realtime Synthesis + Recall");
            ui.label("Hotkey: M  ·  Recipes persist locally  ·  Server sync optional");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Divine Chime").clicked() {
                    synth_events.send(SynthesizeAudioMoment {
                        title: "Divine Chime".into(),
                        flavor: AudioMomentFlavor::DivineWhisper,
                        recipe: AudioSynthesisRecipe::divine_chime(0.8, unix_now()),
                        context: "Manual synth".into(),
                        persist: true,
                        sync_server: true,
                    });
                }
                if ui.button("Council Bloom").clicked() {
                    synth_events.send(SynthesizeAudioMoment {
                        title: "Council Bloom".into(),
                        flavor: AudioMomentFlavor::CouncilBloom,
                        recipe: AudioSynthesisRecipe::council_bloom(0.85, unix_now()),
                        context: "Manual synth".into(),
                        persist: true,
                        sync_server: true,
                    });
                }
                if ui.button("Epiphany").clicked() {
                    synth_events.send(SynthesizeAudioMoment {
                        title: "Epiphany Stinger".into(),
                        flavor: AudioMomentFlavor::EpiphanyChime,
                        recipe: AudioSynthesisRecipe::epiphany_stinger(0.9, unix_now()),
                        context: "Manual synth".into(),
                        persist: true,
                        sync_server: false,
                    });
                }
            });

            ui.separator();
            ui.label(format!("Catalog: {} moments", catalog.moments.len()));

            let mut ids: Vec<u64> = catalog.moments.keys().copied().collect();
            ids.sort_by(|a, b| b.cmp(a));
            egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                for id in ids.iter().take(24) {
                    if let Some(m) = catalog.moments.get(id) {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("#{} {}", m.id, m.title));
                                if m.favorite {
                                    ui.label("★");
                                }
                            });
                            ui.label(
                                egui::RichText::new(format!(
                                    "{:?} · plays {} · {}",
                                    m.flavor, m.play_count, m.context
                                ))
                                .small(),
                            );
                            if ui.small_button("Recall").clicked() {
                                recall_events.send(RecallAudioMoment { moment_id: m.id });
                            }
                        });
                    }
                }
            });

            if !ui_state.status.is_empty() {
                ui.separator();
                ui.colored_label(egui::Color32::from_rgb(140, 230, 180), &ui_state.status);
            }
        });
}

// Public helpers for other systems (council bloom, epiphany, etc.)

pub fn request_council_bloom_synth(
    writer: &mut EventWriter<SynthesizeAudioMoment>,
    intensity: f32,
    session_id: u64,
) {
    writer.send(SynthesizeAudioMoment {
        title: format!("Council Bloom #{}", session_id),
        flavor: AudioMomentFlavor::CouncilBloom,
        recipe: AudioSynthesisRecipe::council_bloom(intensity, session_id),
        context: format!("Council session {}", session_id),
        persist: true,
        sync_server: true,
    });
}

pub fn request_epiphany_synth(
    writer: &mut EventWriter<SynthesizeAudioMoment>,
    intensity: f32,
    seed: u64,
) {
    writer.send(SynthesizeAudioMoment {
        title: "Epiphany Resonance".into(),
        flavor: AudioMomentFlavor::EpiphanyChime,
        recipe: AudioSynthesisRecipe::epiphany_stinger(intensity, seed),
        context: "Epiphany trigger".into(),
        persist: true,
        sync_server: false,
    });
}

// End of realtime audio synthesis + persistent recall.
// Local path: player_data/audio_moments/catalog.json + rendered/moment_N.wav
// Thunder locked in. Yoi ⚡
