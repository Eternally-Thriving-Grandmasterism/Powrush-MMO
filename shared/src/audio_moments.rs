/*!
 * shared/src/audio_moments.rs
 * Powrush-MMO — Real-time Audio Synthesis Moments + Persistent Recall
 *
 * Defines the canonical schema for audio moments that can be:
 *   - synthesized in real time during play
 *   - pre-authored offline
 *   - persisted locally and/or server-side
 *   - recalled / replayed by players later
 *
 * Design principle: store the **synthesis recipe** (compact, deterministic)
 * plus optional rendered asset path. Recall can always regenerate from recipe
 * even if the rendered file is missing.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Permanent PATSAGi Councils | Contact: info@Rathor.ai
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Where an audio moment originated
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioMomentSource {
    /// Generated live during play from game events
    RealtimeSynthesis,
    /// Pre-made asset dropped into assets/audio
    PremadeAsset,
    /// Regenerated from a stored recipe on recall
    RecipeRecall,
    /// Imported / user-provided
    ExternalImport,
}

/// High-level emotional / systemic flavor
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

/// Waveform primitive for procedural synthesis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WaveformKind {
    Sine,
    Triangle,
    SoftSquare,
    NoiseBurst,
    HarmonicStack,
}

/// Compact, deterministic synthesis recipe — the source of truth for recall
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSynthesisRecipe {
    pub waveform: WaveformKind,
    /// Base frequency in Hz
    pub frequency_hz: f32,
    /// Duration in seconds
    pub duration_secs: f32,
    /// Sample rate (typically 48000)
    pub sample_rate: u32,
    /// Amplitude 0.0–1.0
    pub amplitude: f32,
    /// ADSR in seconds
    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    /// Optional second partial (Hz); 0.0 = none
    pub partial_hz: f32,
    pub partial_amp: f32,
    /// Soft low-pass factor 0.0–1.0 (1.0 = bright)
    pub brightness: f32,
    /// Mercy / valence coloring 0.0–1.0
    pub valence: f32,
    /// Deterministic seed for noise / micro-variation
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

/// A single persistent audio moment players can recall
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

impl AudioMoment {
    pub fn new(
        id: u64,
        owner_player_id: u64,
        title: String,
        flavor: AudioMomentFlavor,
        source: AudioMomentSource,
        created_at_unix: u64,
        context: String,
        recipe: AudioSynthesisRecipe,
    ) -> Self {
        Self {
            id,
            owner_player_id,
            title,
            flavor,
            source,
            created_at_unix,
            context,
            recipe,
            rendered_path: None,
            favorite: false,
            play_count: 0,
            mercy_seal: true,
        }
    }
}

/// Catalog of moments (local cache + server sync payload)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

    pub fn get(&self, id: u64) -> Option<&AudioMoment> {
        self.moments.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut AudioMoment> {
        self.moments.get_mut(&id)
    }

    pub fn list_recent(&self, limit: usize) -> Vec<&AudioMoment> {
        let mut v: Vec<_> = self.moments.values().collect();
        v.sort_by(|a, b| b.created_at_unix.cmp(&a.created_at_unix));
        v.into_iter().take(limit).collect()
    }

    pub fn list_favorites(&self) -> Vec<&AudioMoment> {
        self.moments.values().filter(|m| m.favorite).collect()
    }
}

/// Network / event intents for audio moments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioMomentIntent {
    SaveToServer { moment: AudioMoment },
    RequestCatalog { player_id: u64 },
    CatalogSnapshot { catalog: AudioMomentCatalog },
    Recall { moment_id: u64 },
    SetFavorite { moment_id: u64, favorite: bool },
}

// Thunder locked in. Recipe is sovereign. Yoi ⚡
