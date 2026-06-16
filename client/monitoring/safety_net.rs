// client/monitoring/safety_net.rs
// Ra-Thor SafetyNet + RBE Flow Alerts & Dashboard (v18.37)

use bevy::prelude::*;
use crate::monitoring::{KalmanFilter1D, RTSFixedLagSmoother};

// ============================================================
// RBE FLOW ALERTS
// ============================================================

#[derive(Event, Debug, Clone)]
pub enum RBEFlowAlert {
    LowAbundanceCreationRate { rate: f64, threshold: f64 },
    HighSafetyNetTriggerFrequency { count: u32, window_size: usize },
    LowRestorationEffectiveness { effectiveness: f32, threshold: f32 },
    SuddenAbundanceDrop { previous: f64, current: f64, drop: f64 },
    PersistentScarcitySignal { trigger_count: u32 },
}

// ============================================================
// RBE FLOW DASHBOARD (with clean L1 / L2 / L3 separation)
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

    // Level 1 - Informational
    pub informational_alerts: Vec<RBEFlowAlert>,
    pub max_informational_alerts: usize,

    // Level 2 - Supportive Alerts
    pub l2_alerts: Vec<RBEFlowAlert>,
    pub max_l2_alerts: usize,

    // Level 2 Supportive State
    pub l2_multiplier: f32,
    pub l2_boost_active: bool,
    pub last_l2_action_ms: u64,
    pub l2_decay_rate: f32,

    // Level 3 Recovery State
    pub restoration_multiplier: f32,
    pub abundance_boost_active: bool,
    pub last_l3_action_ms: u64,
    pub l3_decay_rate: f32,
}

impl RBEFlowDashboard {
    pub fn new() -> Self {
        Self {
            max_informational_alerts: 20,
            max_l2_alerts: 15,
            l2_multiplier: 1.0,
            l2_boost_active: false,
            l2_decay_rate: 0.25,
            restoration_multiplier: 1.0,
            abundance_boost_active: false,
            l3_decay_rate: 0.15,
            ..Default::default()
        }
    }

    pub fn update_from_snapshot(&mut self, snapshot: &SafetyNetMonitoringSnapshot) {
        self.abundance_creation_rate = snapshot.abundance_creation_rate;
        self.abundance_restoration_rate = snapshot.abundance_restoration_rate;
        self.safety_net_trigger_count = snapshot.safety_net_trigger_count;
        self.average_restoration_magnitude = snapshot.average_restoration_magnitude;
        self.restoration_effectiveness = snapshot.restoration_effectiveness;
        self.server_abundance = snapshot.server_abundance;
    }

    pub fn add_alert(&mut self, alert: RBEFlowAlert) {
        if !self.active_alerts.iter().any(|a| std::mem::discriminant(a) == std::mem::discriminant(&alert)) {
            self.active_alerts.push(alert);
        }
    }

    /// Level 1 - Purely informational
    pub fn add_informational_alert(&mut self, alert: RBEFlowAlert) {
        if self.informational_alerts.len() >= self.max_informational_alerts {
            self.informational_alerts.remove(0);
        }
        self.informational_alerts.push(alert);
    }

    /// Level 2 - Supportive alerts (trigger mild automated assistance)
    pub fn add_l2_alert(&mut self, alert: RBEFlowAlert) {
        if self.l2_alerts.len() >= self.max_l2_alerts {
            self.l2_alerts.remove(0);
        }
        self.l2_alerts.push(alert);
    }

    pub fn clear_old_alerts(&mut self) {
        if self.active_alerts.len() > 10 {
            self.active_alerts.drain(0..self.active_alerts.len() - 10);
        }
        if self.informational_alerts.len() > self.max_informational_alerts {
            self.informational_alerts.drain(0..self.informational_alerts.len() - self.max_informational_alerts);
        }
        if self.l2_alerts.len() > self.max_l2_alerts {
            self.l2_alerts.drain(0..self.l2_alerts.len() - self.max_l2_alerts);
        }
    }

    // Level 2 Support
    pub fn activate_l2_support(&mut self, now_ms: u64) {
        self.l2_multiplier = 1.25;
        self.l2_boost_active = true;
        self.last_l2_action_ms = now_ms;
    }

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

    // Level 3 Recovery
    pub fn activate_l3_recovery(&mut self, now_ms: u64) {
        self.restoration_multiplier = 1.5;
        self.abundance_boost_active = true;
        self.last_l3_action_ms = now_ms;
    }

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
}

// ============================================================
// SAFETY NET MONITORING
// ============================================================

#[derive(Event, Debug, Clone)]
pub struct SafetyNetMonitoringUpdate {
    pub snapshot: SafetyNetMonitoringSnapshot,
}

#[derive(Debug, Clone, Default)]
pub struct SafetyNetMonitoringSnapshot {
    pub timestamp_ms: u64,

    // Network Health
    pub last_latency_ms: u64,
    pub avg_latency_ms: f32,
    pub kalman_latency_residual: f32,
    pub rts_smoothed_latency: f32,
    pub rts_vs_kalman_residual: f32,

    // Server State
    pub server_abundance: f64,
    pub server_health: f32,
    pub server_council_engagement: f32,

    // RBE Flow Dynamics
    pub abundance_creation_rate: f64,
    pub abundance_restoration_rate: f64,
    pub safety_net_trigger_count: u32,
    pub average_restoration_magnitude: f64,
    pub restoration_effectiveness: f32,
}

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

#[derive(Resource, Clone)]
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

    // RBE Flow Tracking
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