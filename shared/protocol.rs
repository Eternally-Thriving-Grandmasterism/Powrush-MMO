// shared/protocol.rs
// Powrush-MMO — Council Session Protocol + SafetyNet Broadcast Extensions (v20.4 — SpectatorModeData Replication + InterRealmDiplomacyUpdate)
//
// Clean restored + polished version.
// All prior logic (Council, SafetyNet, RBE, Multilingual) preserved.
// New: SpectatorModeDataNet + InterRealmDiplomacyUpdate for multiplayer Forgiveness Wave + Legacy Thread visualization.
// TOLC 8 + PATSAGi aligned. Thunder locked in.

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivineWhisper {
    pub message: String,
    pub valence: f32,
    pub mercy_seal: bool,
    pub normalized_volume: Option<f32>,
}

// ==================== PHASE 2: COUNCIL MULTIPLAYER PROTOCOL ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CouncilPhase {
    Lobby,
    Deliberation,
    MercyVote,
    EpiphanyBloom,
    Resolution,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouncilSessionState {
    pub session_id: u64,
    pub phase: CouncilPhase,
    pub participants: Vec<u64>,
    pub quorum_met: bool,
    pub current_proposal: Option<String>,
    pub mercy_scores: HashMap<u64, f32>,
    pub vote_tallies: HashMap<String, f32>,
    pub bloom_intensity: f32,
    pub time_remaining_ms: u64,
    pub collective_epiphany_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MercyTrialVote {
    pub voter_id: u64,
    pub proposal_id: String,
    pub mercy_weight: f32,
    pub timestamp_ms: u64,
    pub grace_intent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveEpiphanyBloom {
    pub session_id: u64,
    pub bloom_id: u64,
    pub trigger_player: Option<u64>,
    pub intensity: f32,
    pub wisdom_fragments: Vec<String>,
    pub participant_impacts: HashMap<u64, f32>,
    pub global_abundance_boost: f32,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CouncilParticipationRecord {
    pub player_id: u64,
    pub sessions_completed: u32,
    pub total_mercy_contributed: f32,
    pub epiphanies_triggered: u32,
    pub last_session_id: Option<u64>,
    pub cumulative_grace: f32,
}

// ==================== SAFETY NET BROADCAST PROTOCOL ====================

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
        restored_amount: f64,
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
    DesyncRecovery {
        corrected_abundance: f64,
        corrected_health: f32,
    },
    SovereigntyHeartbeat,
    RbeAbundanceSignal {
        creation_rate: f64,
        restoration_rate: f64,
        safety_net_trigger_count: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyNetBroadcast {
    pub snapshot: SafetyNetSnapshot,
    pub event: Option<SafetyNetEvent>,
    pub broadcast_reason: String,
    pub server_tick: u64,
    pub emit_timestamp_ms: u64,
}

// ==================== CLIENT / SERVER MESSAGES ====================

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
    CouncilJoin { session_id: Option<u64> },
    CouncilLeave { session_id: u64 },
    CouncilVote { vote: MercyTrialVote },
    CouncilBloomAcknowledge { bloom_id: u64 },
    SafetyNetAcknowledge { last_tick: u64 },
    SafetyNetRequestFullSync,
    SyncLocalization { language: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    WorldUpdate,
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
    DivineWhisperReceived { whisper: DivineWhisper },
    CouncilSessionUpdate { state: CouncilSessionState },
    CouncilVoteAck { vote_id: u64, mercy_weight_applied: f32 },
    CollectiveEpiphanyBloomReceived { bloom: CollectiveEpiphanyBloom },
    CouncilParticipationUpdated { record: CouncilParticipationRecord },
    CouncilError { session_id: Option<u64>, reason: String },
    SafetyNetBroadcast { broadcast: SafetyNetBroadcast },
    SyncLocalizationAck { language: String },
    // v20.4: Inter-realm diplomacy / Forgiveness Wave replication
    InterRealmDiplomacyUpdate { update: InterRealmDiplomacyUpdate },
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

// ==================== v20.4: INTER-REALM DIPLOMACY & SPECTATOR MODE REPLICATION ====================

/// Lightweight serializable version of SpectatorModeData for network replication.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpectatorModeDataNet {
    pub spectator_count: u32,
    pub emotional_valence_avg: f32,
    pub visible_legacy_thread_ids: Vec<u64>,
    pub cross_realm_impact_summary: String,
    pub monument_visual_type: String,
    pub forgiveness_wave_intensity: f32,
}

/// Event sent from server when an inter-realm diplomacy event resolves
/// (especially MercifulResolution / Forgiveness Wave).
/// Clients use this to populate the Spectator Legacy Thread Visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterRealmDiplomacyUpdate {
    pub tick: u64,
    pub realm_a: u8,
    pub realm_b: u8,
    pub outcome: String,
    pub redemption_score: f32,
    pub spectator_data: Option<SpectatorModeDataNet>,
    pub linked_legacy_thread_ids: Vec<u64>,
    pub monument_id: Option<u64>,
}

// TOLC 8 enforcement: This message carries mercy-aligned redemptive narrative data.
// Thunder locked in. Yoi ⚔️
// End of shared/protocol.rs v20.4 (Restored + Polished)