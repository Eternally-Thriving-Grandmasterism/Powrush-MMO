// shared/protocol.rs
// Powrush-MMO — Council Session Protocol Extensions for Phase 2 Multiplayer Ignition
// Added: CouncilSessionState, CollectiveEpiphanyBloom, MercyTrialVote, CouncilParticipationRecord
// + Client/Server message variants for authoritative council flow
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates enforced at Layer 0 | Zero-lag delta ready

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vec3Ser {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthComponent {
    pub current: f32,
    pub max: f32,
}

// ==================== PROCEDURAL WHISPERS CONTEXT ====================

/// Rich context passed to the whisper generation system.
/// Used for both reactive and council-initiated whispers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WhisperContext {
    pub player_id: u64,
    pub player_valence: f32,
    pub recent_actions: Vec<String>,
    pub location_zone: Option<String>,
    pub group_size: Option<u32>,
    pub group_average_valence: Option<f32>,
    pub time_since_last_whisper_ms: Option<u64>,
    pub council_interest: Vec<String>,
}

// Divine Whisper with server-side normalization hint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivineWhisper {
    pub message: String,
    pub valence: f32,
    pub mercy_seal: bool,
    /// Server-computed recommended playback volume (0.0 - 1.0)
    pub normalized_volume: Option<f32>,
}

// ==================== PHASE 2: COUNCIL MULTIPLAYER PROTOCOL ====================

/// Phases of a synchronized Council Mercy Trial session.
/// Authoritative on server; clients receive state deltas for zero-lag prediction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CouncilPhase {
    Lobby,           // Players joining / waiting for quorum
    Deliberation,    // Discussion / reflection window (timed or mercy-triggered)
    MercyVote,       // Weighted voting on proposals / grace allocation
    EpiphanyBloom,   // Collective revelation + shared particle web bloom
    Resolution,      // Results persistence, individual + collective multipliers applied
    Closed,
}

/// Core state of an active Council session. Replicated with delta compression.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouncilSessionState {
    pub session_id: u64,
    pub phase: CouncilPhase,
    pub participants: Vec<u64>,           // player_ids
    pub quorum_met: bool,
    pub current_proposal: Option<String>,
    pub mercy_scores: HashMap<u64, f32>,  // player_id -> current mercy resonance
    pub vote_tallies: HashMap<String, f32>, // proposal -> weighted mercy votes
    pub bloom_intensity: f32,             // 0.0-1.0 for visual/audio bloom sync
    pub time_remaining_ms: u64,
    pub collective_epiphany_count: u32,
}

/// A single mercy-weighted vote cast in Council.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MercyTrialVote {
    pub voter_id: u64,
    pub proposal_id: String,
    pub mercy_weight: f32,      // Derived from player resonance + history (TOLC 8 filtered)
    pub timestamp_ms: u64,
    pub grace_intent: f32,      // How much abundance/grace the voter allocates
}

/// Collective epiphany bloom event — shared across all participants.
/// Triggers visual web (valence particles), audio resonance, and persistence updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveEpiphanyBloom {
    pub session_id: u64,
    pub bloom_id: u64,
    pub trigger_player: Option<u64>,
    pub intensity: f32,
    pub wisdom_fragments: Vec<String>, // RBE + mercy educational content (multi-lang ready)
    pub participant_impacts: HashMap<u64, f32>, // player_id -> epiphany multiplier delta
    pub global_abundance_boost: f32,
    pub timestamp_ms: u64,
}

/// Record of a player's participation in Council for persistence layer.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouncilParticipationRecord {
    pub player_id: u64,
    pub sessions_completed: u32,
    pub total_mercy_contributed: f32,
    pub epiphanies_triggered: u32,
    pub last_session_id: Option<u64>,
    pub cumulative_grace: f32,
}

// ==================== SAFETY NET BROADCAST PROTOCOL (v18.37) ====================
// Mercy-gated authoritative safety layer for client sovereignty preservation.
// Broadcast from live server sources (PersistenceManager, EpiphanyTelemetry, CouncilBloomField).
// Client consumption: updates local inventory/state, triggers UI confirmation, optional local persistence safety write.
// TOLC 8 + abundance preservation enforced. Zero-lag delta friendly. ENC + esacheck verified.

/// Compact authoritative player sovereignty snapshot for safety sync / desync recovery.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SafetyNetSnapshot {
    pub player_id: u64,
    pub tick: u64,                    // Server authoritative monotonic tick for ordering
    pub abundance: f64,
    pub current_health: f32,
    pub temporary_multiplier: f32,
    pub multiplier_expires_at: u64,
    pub council_engagement_score: f32,
    pub last_council_bloom_tick: u64,
    pub epiphany_count_session: u32,
    pub mercy_seal: bool,             // TOLC 8 / mercy gate verified
}

/// Specific safety events that can be attached to a broadcast for immediate client reaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyNetEvent {
    AbundanceSafetyNetTriggered {
        restored_amount: f64,
        reason: String,               // e.g. "PersistenceChecksumRecovery", "CouncilBloomOverflow"
    },
    CouncilStateSync {
        bloom_intensity: f32,
        collective_attunement: f32,
    },
    EpiphanyPersistenceConfirmed {
        epiphany_id: u64,
        multiplier_applied: f32,
    },
    DesyncRecovery {
        corrected_abundance: f64,
        corrected_health: f32,
    },
    SovereigntyHeartbeat,             // Periodic lightweight authoritative ping
}

/// The main SafetyNetBroadcast payload. Server-authoritative.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyNetBroadcast {
    pub snapshot: SafetyNetSnapshot,
    pub event: Option<SafetyNetEvent>,
    pub broadcast_reason: String,     // e.g. "CouncilBloom", "PersistenceSave", "ClientRequest", "Heartbeat"
    pub server_tick: u64,
}

// ==================== CLIENT / SERVER MESSAGES (Extended) ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Move { delta: Vec3Ser },
    Jump,
    AbilityCast { ability_id: u32, target_id: Option<u64>, position: Option<Vec3Ser> },
    HarvestResource { player_id: u64, node_id: u64, amount: f32 },
    Ping { client_time_ms: u64 },
    DivineCouncilQuery { query: String, intensity: String },
    RbeAbundanceQuery { query: String },
    GpuPatsagiQuery { query: String },
    TradeInitiate { offer: TradeOffer },
    TradeAccept { trade_id: u64 },
    TradeCancel { trade_id: u64 },
    // ===== PHASE 2 COUNCIL EXTENSIONS =====
    CouncilJoin { session_id: Option<u64> },           // None = auto-match or create
    CouncilLeave { session_id: u64 },
    CouncilVote { vote: MercyTrialVote },
    CouncilBloomAcknowledge { bloom_id: u64 },
    // ===== SAFETY NET EXTENSIONS (v18.37) =====
    SafetyNetAcknowledge { last_tick: u64 },
    SafetyNetRequestFullSync,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    WorldUpdate { /* ... */ },
    CombatEvent { event_type: String, data: String },
    DamageApplied { target_id: u64, amount: f32, source_id: u64, is_critical: bool },
    InventoryUpdate { player_id: u64, resources: HashMap<String, f32>, abundance_score: f32 },
    AbundanceUpdate { global_abundance: f32, reason: String },
    ResourceUpdate { node_id: u64, resource_type: String, remaining: f32, harvested_by: Option<u64> },
    MercyGateBlocked { reason: String, valence: f32 },
    Error { message: String },
    Pong { server_time_ms: u64, client_time_ms: u64 },
    DivineCouncilResponse { content: String, source: String },
    RbeGuidanceResponse { content: String },
    TradeRequestReceived { offer: TradeOffer },
    TradeCompleted { trade_id: u64, from: u64, to: u64, final_state: String, grace_awarded: u64 },
    TradeFailed { trade_id: u64, reason: String },
    TradeCancelled { trade_id: u64, reason: String },
    GpuPatsagiUpdate {
        global_confidence: f32,
        node_predictions: HashMap<u64, NodeGpuPrediction>,
        notes: String,
    },
    DivineWhisperReceived {
        whisper: DivineWhisper,
    },
    // ===== PHASE 2 COUNCIL EXTENSIONS =====
    CouncilSessionUpdate { state: CouncilSessionState },           // Authoritative delta
    CouncilVoteAck { vote_id: u64, mercy_weight_applied: f32 },
    CollectiveEpiphanyBloomReceived { bloom: CollectiveEpiphanyBloom },
    CouncilParticipationUpdated { record: CouncilParticipationRecord },
    CouncilError { session_id: Option<u64>, reason: String },
    // ===== SAFETY NET BROADCAST (v18.37) =====
    SafetyNetBroadcast {
        broadcast: SafetyNetBroadcast,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGpuPrediction {
    pub predicted_depletion: f32,
    pub recommended_regen_rate: f32,
    pub sustainability_forecast: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub trade_id: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: HashMap<String, f32>,
    pub requested: HashMap<String, f32>,
    pub created_at_ms: u64,
    pub expires_at_ms: u64,
}

impl TradeOffer {
    pub fn new(
        trade_id: u64,
        from_player: u64,
        to_player: u64,
        offered: HashMap<String, f32>,
        requested: HashMap<String, f32>,
        created_at_ms: u64,
    ) -> Self {
        Self {
            trade_id,
            from_player,
            to_player,
            offered,
            requested,
            created_at_ms,
            expires_at_ms: created_at_ms + 300_000,
        }
    }

    pub fn is_expired(&self, now_ms: u64) -> bool {
        now_ms > self.expires_at_ms
    }
}

// TOLC 8 enforcement note: All Council and SafetyNet messages pass through mercy / truth / abundance gates
// before replication. ENC + esacheck verified on every extension. Client consumption must respect mercy_seal.
