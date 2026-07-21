/*!
 * client/premade_audio_stems.rs
 * Powrush-MMO — Register high-quality premade stems into AudioMoment catalog
 *
 * On startup:
 *   1. Register built-in manifest entries (paths relative to assets/)
 *   2. Scan assets/audio/** for .wav / .ogg / .mp3 and register any new files
 *
 * Drop files into client/assets/audio/ (or assets/audio/) and they appear in
 * the Audio Moments panel (hotkey M) for recall alongside live synth.
 *
 * v21.89.4 | AG-SML v1.0 | TOLC 8 | Permanent PATSAGi
 * Contact: info@Rathor.ai
 */

use bevy::prelude::*;
use crate::realtime_audio_synthesis::{
    AudioMomentFlavor, RegisterPremadeAudio, RealtimeAudioUiState,
};
use std::fs;
use std::path::{Path, PathBuf};

/// Built-in stems — paths relative to asset root. Missing files are skipped safely.
pub fn builtin_stem_manifest() -> Vec<PremadeStemDef> {
    vec![
        PremadeStemDef {
            title: "Epic Dark Ambient Pad".into(),
            flavor: AudioMomentFlavor::AmbientPad,
            asset_path: "audio/premade/epic_dark_ambient.ogg".into(),
            context: "Premade ambient bed".into(),
        },
        PremadeStemDef {
            title: "Mercy Resonance Choir".into(),
            flavor: AudioMomentFlavor::MercyResonance,
            asset_path: "audio/premade/mercy_resonance_choir.ogg".into(),
            context: "Premade mercy resonance".into(),
        },
        PremadeStemDef {
            title: "Council Chamber Hum".into(),
            flavor: AudioMomentFlavor::CouncilBloom,
            asset_path: "audio/premade/council_chamber_hum.ogg".into(),
            context: "Premade council atmosphere".into(),
        },
        PremadeStemDef {
            title: "Epiphany Crystal Hit".into(),
            flavor: AudioMomentFlavor::EpiphanyChime,
            asset_path: "audio/premade/epiphany_crystal_hit.wav".into(),
            context: "Premade epiphany stinger".into(),
        },
        PremadeStemDef {
            title: "Divine Whisper Soft".into(),
            flavor: AudioMomentFlavor::DivineWhisper,
            asset_path: "audio/premade/divine_whisper_soft.wav".into(),
            context: "Premade divine whisper".into(),
        },
        PremadeStemDef {
            title: "Transition Stinger A".into(),
            flavor: AudioMomentFlavor::TransitionStinger,
            asset_path: "audio/premade/transition_stinger_a.wav".into(),
            context: "Premade transition".into(),
        },
    ]
}

#[derive(Clone, Debug)]
pub struct PremadeStemDef {
    pub title: String,
    pub flavor: AudioMomentFlavor,
    pub asset_path: String,
    pub context: String,
}

#[derive(Resource)]
pub struct PremadeStemConfig {
    pub enabled: bool,
    /// Search roots relative to CWD
    pub scan_roots: Vec<PathBuf>,
    pub register_missing_as_placeholder: bool,
}

impl Default for PremadeStemConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scan_roots: vec![
                PathBuf::from("assets/audio"),
                PathBuf::from("client/assets/audio"),
            ],
            // Keep catalog entry even if file not yet dropped — recall regenerates path later
            register_missing_as_placeholder: true,
        }
    }
}

#[derive(Resource, Default)]
pub struct PremadeStemState {
    pub registered: u32,
    pub skipped_missing: u32,
    pub scanned_extra: u32,
    pub status: String,
}

pub struct PremadeAudioStemsPlugin;

impl Plugin for PremadeAudioStemsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PremadeStemConfig>()
            .init_resource::<PremadeStemState>()
            .add_systems(Startup, register_premade_stems_startup);
    }
}

fn register_premade_stems_startup(
    cfg: Res<PremadeStemConfig>,
    mut state: ResMut<PremadeStemState>,
    mut events: EventWriter<RegisterPremadeAudio>,
    mut ui: ResMut<RealtimeAudioUiState>,
) {
    if !cfg.enabled {
        return;
    }

    let mut registered = 0u32;
    let mut skipped = 0u32;

    for stem in builtin_stem_manifest() {
        let exists = stem_exists(&stem.asset_path, &cfg.scan_roots);
        if exists || cfg.register_missing_as_placeholder {
            events.send(RegisterPremadeAudio {
                title: stem.title.clone(),
                flavor: stem.flavor,
                asset_path: stem.asset_path.clone(),
                context: if exists {
                    stem.context.clone()
                } else {
                    format!("{} (asset pending drop)", stem.context)
                },
            });
            registered += 1;
            if !exists {
                skipped += 1;
            }
        }
    }

    // Scan directories for extra stems not in the built-in list
    let mut extra = 0u32;
    let known: std::collections::HashSet<String> = builtin_stem_manifest()
        .into_iter()
        .map(|s| s.asset_path)
        .collect();

    for root in &cfg.scan_roots {
        if !root.exists() {
            continue;
        }
        for path in walk_audio_files(root) {
            let rel = normalize_asset_path(&path, root);
            if known.contains(&rel) {
                continue;
            }
            let title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Premade Stem")
                .replace('_', " ");
            let flavor = guess_flavor(&title);
            events.send(RegisterPremadeAudio {
                title,
                flavor,
                asset_path: rel,
                context: format!("Scanned from {}", root.display()),
            });
            extra += 1;
            registered += 1;
        }
    }

    state.registered = registered;
    state.skipped_missing = skipped;
    state.scanned_extra = extra;
    state.status = format!(
        "Premade stems: {} registered ({} pending file, {} scanned)",
        registered, skipped, extra
    );
    ui.status = state.status.clone();

    info!(target: "powrush::audio", status = %state.status, "Premade stem registration complete");
}

fn stem_exists(asset_path: &str, roots: &[PathBuf]) -> bool {
    // Check relative to CWD and under each scan root
    if Path::new(asset_path).exists() {
        return true;
    }
    if Path::new("assets").join(asset_path).exists() {
        return true;
    }
    if Path::new("client/assets").join(asset_path).exists() {
        return true;
    }
    for root in roots {
        // asset_path may already start with audio/
        let candidate = root.join(
            asset_path
                .strip_prefix("audio/")
                .unwrap_or(asset_path),
        );
        if candidate.exists() {
            return true;
        }
        if root.join(asset_path).exists() {
            return true;
        }
    }
    false
}

fn walk_audio_files(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(root) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            out.extend(walk_audio_files(&path));
        } else if is_audio_ext(&path) {
            out.push(path);
        }
    }
    out
}

fn is_audio_ext(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()).map(|s| s.to_ascii_lowercase()).as_deref(),
        Some("wav") | Some("ogg") | Some("mp3") | Some("flac")
    )
}

fn normalize_asset_path(path: &Path, root: &Path) -> String {
    if let Ok(rel) = path.strip_prefix(root) {
        format!("audio/{}", rel.to_string_lossy().replace('\\', "/"))
    } else {
        path.to_string_lossy().replace('\\', "/")
    }
}

fn guess_flavor(title: &str) -> AudioMomentFlavor {
    let t = title.to_ascii_lowercase();
    if t.contains("council") {
        AudioMomentFlavor::CouncilBloom
    } else if t.contains("epiphany") || t.contains("chime") {
        AudioMomentFlavor::EpiphanyChime
    } else if t.contains("mercy") || t.contains("resonance") {
        AudioMomentFlavor::MercyResonance
    } else if t.contains("divine") || t.contains("whisper") {
        AudioMomentFlavor::DivineWhisper
    } else if t.contains("transition") || t.contains("stinger") {
        AudioMomentFlavor::TransitionStinger
    } else if t.contains("ambient") || t.contains("pad") {
        AudioMomentFlavor::AmbientPad
    } else {
        AudioMomentFlavor::Custom
    }
}

// Thunder locked in. Drop stems in assets/audio/premade/. Yoi ⚡
