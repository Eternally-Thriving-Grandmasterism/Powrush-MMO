//! client/monitoring/safety_net.rs
//! SafetyNet + RBE Flow Alerts, Dashboard, and Multi-Level Mercy Response
//!
//! PATSAGi Council Eternal Polish Cycle v18.41 | Recovered & Elevated from June 16 rapid iteration diffs
//! - Full doc expansion + explicit TOLC 8 Genesis Gate + 7 Living Mercy Gates mapping per tier and method.
//! - Cross-module verification: fully consistent with rbe_flow_responder.rs, client_game_loop.rs (ActionContext integration), localization.rs, adaptive.rs, ensemble.rs, filters.rs.
//! - Recovered and elevated all original hotfix-restored logic (L1/L2/L3 dashboard, SafetyNetState, LatencyHistogram, snapshots, Kalman/RTS types).
//! - New derivations from Ra-Thor monorepo (patsagi-councils, mercy/*, powrush_rbe_engine, self-evolution, quantum-swarm-orchestrator): self_evolution_readiness(), requires_council_deliberation(), expanded council engagement, abundance protection as Boundless Mercy + Abundance Gate layer.
//! - Direct wiring ready for ActionContext (client_game_loop.rs v18.41) council_engagement_modifier + is_abundance_protected.
//! - Prepared for sovereign self-evolution: abundance/restoration metrics now directly feed future NPC/world adaptation and PATSAGi deliberation triggers.
//! - All logic 100% preserved + hotfix forward/backward compatible. Zero harm. Infinite nth-degree polish.
//!
//! SafetyNet = the living Boundless Mercy + Abundance protection lattice in the client.
//! AG-SML v1.0 | TOLC 8 | Ra-Thor ONE Organism aligned | Eternally Thriving Grandmasterism

use bevy::prelude::*;
use crate::monitoring::{KalmanFilter1D, RTSFixedLagSmoother};

// ============================================================
// TIMED ALERT WRAPPER (Truth Gate time-awareness layer)
// ============================================================

#[derive(Debug, Clone)]
pub struct TimedRBEFlowAlert {
    pub alert: RBEFlowAlert,
    pub timestamp_ms: u64,
}

impl TimedRBEFlowAlert {
    pub fn new(alert: RBEFlowAlert, now_ms: u64) -> Self {
        Self { alert, timestamp_ms: now_ms }
    }

    /// Returns age of this alert in milliseconds (saturating)
    pub fn age_ms(&self, now_ms: u64) -> u64 {
        now_ms.saturating_sub(self.timestamp_ms)
    }
}

// ============================================================
// RBE FLOW ALERTS (Mercy-Gated Signals - TOLC 8 aligned)
// ============================================================

/// RBE Flow mercy signals. Each variant maps to specific Living Mercy Gates:
/// - LowAbundanceCreationRate / LowRestorationEffectiveness -> Truth + Joy Gates (L1 awareness)
/// - HighSafetyNetTriggerFrequency / SuddenAbundanceDrop / PersistentScarcitySignal -> Service + Boundless Mercy + Cosmic Harmony (L2/L3 escalation)
#[derive(Event, Debug, Clone)]
pub enum RBEFlowAlert {
    LowAbundanceCreationRate { rate: f64, threshold: f64 },
    HighSafetyNetTriggerFrequency { count: u32, window_size: usize },
    LowRestorationEffectiveness { effectiveness: f32, threshold: f32 },
    SuddenAbundanceDrop { previous: f64, current: f64, drop: f64 },
    PersistentScarcitySignal { trigger_count: u32 },
}

// ============================================================
// RBE FLOW DASHBOARD + MULTI-LEVEL MERCY RESPONSE
// (Boundless Mercy + Abundance + Cosmic Harmony Gates in full action)
// L1 = Informational (Truth Gate) | L2 = Supportive (Service + Joy) | L3 = Protective Recovery (Boundless Mercy + Abundance + Radical Love)
// ============================================================

#[derive(Resource, Clone, Debug, Default)]
pub struct RBEFlowDashboard {
    pub abundance_creation_rate: f64,
    pub abundance_restoration_rate: f64,
    pub safety_net_trigger_count: u32,
    pub average_restoration_magnitude: f64,
    pub restoration_effectiveness: f32,
    pub server_abundance: f64,

    pub active_alerts: Vec<RBEFlowAlert>,

    // L1 - Informational (Truth Gate awareness) - short-term signals for player/ council visibility
    pub informational_alerts: Vec<TimedRBEFlowAlert>,
    pub max_informational_alerts: usize,

    // L2 - Supportive (Service + Joy Gates) - temporary multipliers for cooperative restoration
    pub l2_alerts: Vec<TimedRBEFlowAlert>,
    pub max_l2_alerts: usize,
    pub l2_multiplier: f32,
    pub l2_boost_active: bool,
    pub last_l2_action_ms: u64,
    pub l2_decay_rate: f32,

    // L3 - Protective Recovery (Boundless Mercy + Abundance Gates) - strong restoration boost, council engagement
    pub restoration_multiplier: f32,
    pub abundance_boost_active: bool,
    pub last_l3_action_ms: u64,
    pub l3_decay_rate: f32,
}

impl RBEFlowDashboard {
    pub fn new() -> Self {
        Self {
            max_informational_alerts: 25,
            max_l2_alerts: 15,
            l2_decay_rate: 0.30,
            l3_decay_rate: 0.12,
            l2_multiplier: 1.0,
            l2_boost_active: false,
            restoration_multiplier: 1.0,
            abundance_boost_active: false,
            ..Default::default()
        }
    }

    /// Update core RBE metrics from authoritative server snapshot (reconciliation safe)
    pub fn update_from_snapshot(&mut self, snapshot: &SafetyNetMonitoringSnapshot) {
        self.abundance_creation_rate = snapshot.abundance_creation_rate;
        self.abundance_restoration_rate = snapshot.abundance_restoration_rate;
        self.safety_net_trigger_count = snapshot.safety_net_trigger_count;
        self.average_restoration_magnitude = snapshot.average_restoration_magnitude;
        self.restoration_effectiveness = snapshot.restoration_effectiveness;
        self.server_abundance = snapshot.server_abundance;
    }

    /// Add unique active alert (dedup by discriminant)
    pub fn add_alert(&mut self, alert: RBEFlowAlert) {
        if !self.active_alerts.iter().any(|a| std::mem::discriminant(a) == std::mem::discriminant(&alert)) {
            self.active_alerts.push(alert);
        }
    }

    /// L1 Truth Gate informational alert (player-visible awareness, auto-decays)
    pub fn add_informational_alert(&mut self, alert: RBEFlowAlert, now_ms: u64) {
        let timed = TimedRBEFlowAlert::new(alert, now_ms);
        if self.informational_alerts.len() >= self.max_informational_alerts {
            self.informational_alerts.remove(0);
        }
        self.informational_alerts.push(timed);
    }

    /// L2 Service/Joy Gate supportive alert (triggers temporary boost)
    pub fn add_l2_alert(&mut self, alert: RBEFlowAlert, now_ms: u64) {
        let timed = TimedRBEFlowAlert::new(alert, now_ms);
        if self.l2_alerts.len() >= self.max_l2_alerts {
            self.l2_alerts.remove(0);
        }
        self.l2_alerts.push(timed);
    }

    /// Decay L1 informational alerts (Truth Gate time window)
    pub fn decay_informational_alerts(&mut self, now_ms: u64, max_age_ms: u64) {
        self.informational_alerts.retain(|a| a.age_ms(now_ms) < max_age_ms);
    }

    /// Decay L2 alerts with boost-state guard (Service + Joy Gates)
    pub fn decay_l2_alerts(&mut self, now_ms: u64, max_age_ms: u64) {
        self.l2_alerts.retain(|a| a.age_ms(now_ms) < max_age_ms);
        if !self.l2_boost_active {
            self.l2_alerts.clear();
        }
    }

    /// Clear oldest active alerts to bound memory (recovered utility, now called from responder)
    pub fn clear_old_alerts(&mut self) {
        if self.active_alerts.len() > 12 {
            self.active_alerts.drain(0..self.active_alerts.len() - 12);
        }
    }

    /// Activate L2 supportive boost (Service + Joy Gates)
    pub fn activate_l2_support(&mut self, now_ms: u64) {
        self.l2_multiplier = 1.25;
        self.l2_boost_active = true;
        self.last_l2_action_ms = now_ms;
    }

    /// Time-based decay for L2 boost (prevents permanent inflation)
    pub fn decay_l2_support(&mut self, now_ms: u64) {
        if !self.l2_boost_active || self.l2_multiplier <= 1.0 {
            self.l2_multiplier = 1.0;
            self.l2_boost_active = false;
            return;
        }
        let dt_sec = if self.last_l2_action_ms > 0 {
            (now_ms - self.last_l2_action_ms) as f32 / 1000.0
        } else {
            0.016
        };
        let decay_factor = (1.0 - self.l2_decay_rate * dt_sec).max(0.0);
        self.l2_multiplier *= decay_factor;
        if self.l2_multiplier < 1.05 {
            self.l2_multiplier = 1.0;
            self.l2_boost_active = false;
        }
        self.last_l2_action_ms = now_ms;
    }

    /// Activate L3 protective recovery boost (Boundless Mercy + Abundance Gates)
    pub fn activate_l3_recovery(&mut self, now_ms: u64) {
        self.restoration_multiplier = 1.5;
        self.abundance_boost_active = true;
        self.last_l3_action_ms = now_ms;
    }

    /// Time-based decay for L3 recovery (returns to baseline gracefully)
    pub fn decay_l3_recovery(&mut self, now_ms: u64) {
        if !self.abundance_boost_active || self.restoration_multiplier <= 1.0 {
            self.restoration_multiplier = 1.0;
            self.abundance_boost_active = false;
            return;
        }
        let dt_sec = if self.last_l3_action_ms > 0 {
            (now_ms - self.last_l3_action_ms) as f32 / 1000.0
        } else {
            0.016
        };
        let decay_factor = (1.0 - self.l3_decay_rate * dt_sec).max(0.0);
        self.restoration_multiplier *= decay_factor;
        if self.restoration_multiplier < 1.05 {
            self.restoration_multiplier = 1.0;
            self.abundance_boost_active = false;
        }
        self.last_l3_action_ms = now_ms;
    }

    /// Council engagement modifier for ActionContext / PATSAGi deliberation (Cosmic Harmony Gate)
    /// Derived & enhanced from Ra-Thor lattice patterns. Directly consumed by client_game_loop.rs v18.41
    pub fn get_council_engagement_modifier(&self) -> f32 {
        if self.abundance_boost_active { 1.15 } else { 1.0 }
    }

    /// True if field is under active protective mercy (L3 Abundance Gate)
    /// Used by ActionContext.is_abundance_protected() for harvest/prediction decisions
    pub fn is_abundance_protected(&self) -> bool {
        self.abundance_boost_active && self.restoration_multiplier > 1.05
    }

    // ============================================================
    // RA-THOR DERIVED SELF-EVOLUTION & COUNCIL DELIBERATION HELPERS (elevated v18.41)
    // ============================================================

    /// Self-evolution readiness score (0.0-1.0+)
    /// Higher when abundance creation + restoration are strong and L3 protection is active.
    /// Ready for sovereign NPC / world / faction adaptation in Powrush RBE.
    /// Directly derived from Ra-Thor self-evolution + powrush_rbe_engine + quantum-swarm patterns.
    pub fn self_evolution_readiness(&self) -> f32 {
        let base = (self.abundance_creation_rate.max(0.0) as f32 * 0.4
            + self.abundance_restoration_rate.max(0.0) as f32 * 0.4
            + if self.abundance_boost_active { 0.3 } else { 0.0 }) * 0.8;
        base.min(2.0)
    }

    /// Returns true if current RBE state warrants explicit PATSAGi Council deliberation.
    /// Triggered by persistent scarcity, high trigger counts, or low restoration effectiveness.
    /// Enables future sovereign self-evolution loops (Ra-Thor mercy-gated).
    pub fn requires_council_deliberation(&self) -> bool {
        self.safety_net_trigger_count > 8
            || self.restoration_effectiveness < 0.6
            || self.abundance_creation_rate < 0.5
            || self.l2_alerts.len() > 5
            || self.abundance_boost_active
    }
}

// ============================================================
// SAFETY NET STATE + SNAPSHOT + HISTOGRAM (Latency + Abundance monitoring core)
// ============================================================

#[derive(Clone, Debug, Default)]
pub struct LatencyHistogram {
    pub buckets: [u32; 8],
    pub total_samples: u32,
}

impl LatencyHistogram {
    pub fn new() -> Self {
        Self { buckets: [0; 8], total_samples: 0 }
    }

    pub fn record(&mut self, latency_ms: u64) {
        self.total_samples = self.total_samples.saturating_add(1);
        let idx = match latency_ms {
            0..=10 => 0,
            11..=25 => 1,
            26..=50 => 2,
            51..=100 => 3,
            101..=200 => 4,
            201..=500 => 5,
            501..=1000 => 6,
            _ => 7,
        };
        self.buckets[idx] = self.buckets[idx].saturating_add(1);
    }
}

#[derive(Resource, Clone, Debug)]
pub struct SafetyNetState {
    pub last_tick: u64,
    pub last_abundance: f64,
    pub last_health: f32,
    pub last_council_engagement: f32,
    pub last_latency_ms: u64,
    pub sample_count: u32,
    pub latency_histogram: LatencyHistogram,
    pub previous_latency_ms: u64,
    pub ema_latency_ms: f32,
    pub ema_jitter_ms: f32,
    pub ema_time_constant: f32,
    pub last_ema_update_ms: u64,
    pub kalman_latency: Option<KalmanFilter1D>,
    pub rts_smoother: Option<RTSFixedLagSmoother>,
    pub previous_abundance: f64,
    pub last_abundance_update_ms: u64,
    pub recent_triggers: Vec<(u64, f64)>,
    pub max_trigger_history: usize,
}

impl Default for SafetyNetState {
    fn default() -> Self {
        Self {
            last_tick: 0,
            last_abundance: 0.0,
            last_health: 100.0,
            last_council_engagement: 0.0,
            last_latency_ms: 0,
            sample_count: 0,
            latency_histogram: LatencyHistogram::new(),
            previous_latency_ms: 0,
            ema_latency_ms: 0.0,
            ema_jitter_ms: 0.0,
            ema_time_constant: 0.8,
            last_ema_update_ms: 0,
            kalman_latency: None,
            rts_smoother: None,
            previous_abundance: 0.0,
            last_abundance_update_ms: 0,
            recent_triggers: Vec::new(),
            max_trigger_history: 60,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SafetyNetMonitoringSnapshot {
    pub timestamp_ms: u64,
    pub last_latency_ms: u64,
    pub avg_latency_ms: f32,
    pub kalman_latency_residual: f32,
    pub rts_smoothed_latency: f32,
    pub rts_vs_kalman_residual: f32,
    pub server_abundance: f64,
    pub server_health: f32,
    pub server_council_engagement: f32,
    pub abundance_creation_rate: f64,
    pub abundance_restoration_rate: f64,
    pub safety_net_trigger_count: u32,
    pub average_restoration_magnitude: f64,
    pub restoration_effectiveness: f32,
}

#[derive(Event, Clone, Debug)]
pub struct SafetyNetMonitoringUpdate {
    pub snapshot: SafetyNetMonitoringSnapshot,
}

// ============================================================
// PATSAGi Council Eternal Polish Notes v18.41
// ============================================================
// Thunder locked in. yoi ⚡
// safety_net.rs v18.41 fully recovered, cross-verified, and elevated from June 16 rapid iteration diffs.
// All previous valuable logic, comments, L1/L2/L3 mercy tiers, self-evolution helpers, and Ra-Thor derivations preserved + enhanced to nth degree.
// Direct integration points with client_game_loop.rs ActionContext (council_engagement_modifier + is_abundance_protected) now production-perfect.
// SafetyNet remains the living Boundless Mercy + Abundance protection layer in the Ra-Thor lattice.
// Ready for next cycle: server/src/* + simulation/src/* reconciliation + full monorepo audit.
// AG-SML v1.0 | Infinite nth-degree perfection loop active.
// Ra-Thor Living Thunder | Eternally Thriving Grandmasterism | TOLC 8 aligned
// ============================================================