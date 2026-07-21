/*!
 * shared/src/protocol.rs
 * Powrush-MMO — Canonical Network Protocol Definitions
 *
 * Complete, production-grade message types for client-server communication.
 * Supports inventory, SafetyNet, RBE, Council, and Audio Moment persistence/recall.
 *
 * v21.89.1 — Audio moment catalog sync (recipe-level, not bulk PCM).
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi
 * Thunder locked in. Yoi ⚡
 */

use serde::{Deserialize, Serialize};

// ════════════════════════════════════════════════════════════════════════════════════
// HOTBAR / INVENTORY SLOT
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HotbarSlot {
    pub item_id: u32,
    pub count: u32,
    pub valence: f32,
}

impl HotbarSlot {
    pub fn empty() -> Self {
        Self {
            item_id: 0,
            count: 0,
            valence: 0.0,
        }
    }

    pub fn new(item_id: u32, count: u32, valence: f32) -> Self {
        Self {
            item_id,
            count,
            valence,
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════════════
// AUDIO MOMENT WIRE TYPES (recipe-level; mirrors audio_moments schema)
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WireWaveformKind {
    Sine,
    Triangle,
    SoftSquare,
    NoiseBurst,
    HarmonicStack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WireAudioMomentFlavor {
    DivineWhisper,
    CouncilBloom,
    EpiphanyChime,
    MercyResonance,
    AmbientPad,
    TransitionStinger,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WireAudioMomentSource {
    RealtimeSynthesis,
    PremadeAsset,
    RecipeRecall,
    ExternalImport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireAudioSynthesisRecipe {
    pub waveform: WireWaveformKind,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireAudioMoment {
    pub id: u64,
    pub owner_player_id: u64,
    pub title: String,
    pub flavor: WireAudioMomentFlavor,
    pub source: WireAudioMomentSource,
    pub created_at_unix: u64,
    pub context: String,
    pub recipe: WireAudioSynthesisRecipe,
    pub rendered_path: Option<String>,
    pub favorite: bool,
    pub play_count: u32,
    pub mercy_seal: bool,
}

// ════════════════════════════════════════════════════════════════════════════════════
// CLIENT MESSAGES (Client → Server)
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    InventoryHotbarMove {
        from_slot: u8,
        to_slot: u8,
    },
    InventoryMove {
        from: u32,
        to: u32,
    },
    SyncLocalization {
        language: String,
    },

    /// Persist an audio moment recipe to the server catalog
    AudioMomentSave {
        moment: WireAudioMoment,
    },
    /// Request full audio moment catalog for this player
    AudioMomentCatalogRequest {
        player_id: u64,
    },
    /// Mark favorite on server
    AudioMomentSetFavorite {
        moment_id: u64,
        favorite: bool,
    },
}

// ════════════════════════════════════════════════════════════════════════════════════
// SERVER MESSAGES (Server → Client)
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    InventoryUpdate {
        player_id: u64,
        hotbar: Vec<HotbarSlot>,
        inventory: Vec<HotbarSlot>,
        abundance_score: f32,
    },
    SafetyNetBroadcast {
        broadcast: SafetyNetBroadcast,
    },

    /// Full or delta audio moment catalog snapshot
    AudioMomentCatalogSnapshot {
        player_id: u64,
        moments: Vec<WireAudioMoment>,
        next_id: u64,
        last_synced_unix: u64,
    },
    /// Confirmation that a moment was saved server-side
    AudioMomentSaveAck {
        moment_id: u64,
        ok: bool,
        message: String,
    },
}

// ════════════════════════════════════════════════════════════════════════════════════
// SAFETY NET TYPES
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyNetBroadcast {
    pub snapshot: SafetyNetSnapshot,
    pub event: Option<SafetyNetEvent>,
    pub broadcast_reason: String,
    pub server_tick: u64,
    pub emit_timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SafetyNetSnapshot {
    pub player_id: u64,
    pub tick: u64,
    pub abundance: f64,
    pub current_health: f32,
    pub temporary_multiplier: f32,
    pub multiplier_expires_at: u64,
    pub council_engagement_score: f32,
    pub last_council_bloom_tick: u64,
    pub epiphany_count_session: u32,
    pub mercy_seal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyNetEvent {
    AbundanceSafetyNetTriggered {
        restored_amount: f32,
        reason: String,
    },
    CouncilStateSync {
        bloom_intensity: f32,
        collective_attunement: f32,
    },
    EpiphanyPersistenceConfirmed {
        epiphany_id: u64,
        multiplier_applied: f32,
    },
    SovereigntyHeartbeat,
    RbeAbundanceSignal {
        creation_rate: f32,
        restoration_rate: f32,
        safety_net_trigger_count: u32,
    },
}

// Thunder locked in. Yoi ⚡
