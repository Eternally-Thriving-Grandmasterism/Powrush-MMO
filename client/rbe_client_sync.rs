// client/rbe_client_sync.rs
// Powrush-MMO — RBE + Council + Safety Net client sync layer
// Handles authoritative ServerMessage consumption including SafetyNetBroadcast (v18.37)
// Full monitoring state with RTS Fixed-Lag Backward Smoother (v18.37)
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, Vec3Ser, DivineWhisper, SafetyNetBroadcast, SafetyNetEvent};
use std::sync::Arc;
use tokio::sync::RwLock;
use bytes::Bytes;

use crate::inventory_ui::{LocalInventory, TradeUIState, InventoryUpdated, TradeResponseReceived, HarvestResponseReceived, handle_server_message};
use crate::divine_whispers_ui::{CurrentDivineWhisper, DivineWhispersLog, DivineWhisperUI, receive_divine_whisper_from_server};

// ============================================================
// SAFETY NET MONITORING EVENT
// ============================================================

#[derive(Event, Debug, Clone)]
pub struct SafetyNetMonitoringUpdate {
    pub snapshot: SafetyNetMonitoringSnapshot,
}

// ============================================================
// SAFETY NET MONITORING SNAPSHOT
// ============================================================

#[derive(Debug, Clone, Default)]
pub struct SafetyNetMonitoringSnapshot {
    pub timestamp_ms: u64,
    pub last_latency_ms: u64,
    pub avg_latency_ms: f32,
    pub min_latency_ms: u64,
    pub max_latency_ms: u64,
    pub sample_count: u32,
    pub last_jitter_ms: u64,
    pub avg_jitter_ms: f32,
    pub max_jitter_ms: u64,
    pub p50_ms: u64,
    pub p95_ms: u64,
    pub p99_ms: u64,
    pub ema_latency_ms: f32,
    pub ema_jitter_ms: f32,
    pub kalman_latency_estimate: f32,
    pub kalman_latency_velocity: f32,
    pub kalman_latency_residual: f32,
    pub kalman_2d_latency: f32,
    pub kalman_2d_jitter: f32,
    pub kalman_2d_latency_residual: f32,
    pub kalman_2d_jitter_residual: f32,
    pub smoothed_latency: f32,
    pub server_abundance: f64,
    pub server_health: f32,
    pub server_council_engagement: f32,
}

impl SafetyNetState {
    pub fn get_monitoring_snapshot(&self, now_ms: u64) -> SafetyNetMonitoringSnapshot {
        let histogram = &self.latency_histogram;

        let kalman_lat = self.kalman_latency.as_ref().map_or(0.0, |k| k.estimate);
        let kalman_vel = self.kalman_latency.as_ref().map_or(0.0, |k| k.velocity);
        let kalman_res = self.kalman_latency.as_ref().map_or(0.0, |k| k.last_residual);

        let kalman_2d_lat = self.kalman_2d.as_ref().map_or(0.0, |k| k.latency);
        let kalman_2d_jit = self.kalman_2d.as_ref().map_or(0.0, |k| k.jitter);
        let kalman_2d_lat_res = self.kalman_2d.as_ref().map_or(0.0, |k| k.last_latency_residual);
        let kalman_2d_jit_res = self.kalman_2d.as_ref().map_or(0.0, |k| k.last_jitter_residual);

        let smoothed = self.rts_smoother.as_ref()
            .map_or_else(
                || self.smoother_latency.as_ref().map_or(0.0, |s| s.smoothed_estimate),
                |r| r.smoothed_estimate
            );

        SafetyNetMonitoringSnapshot {
            timestamp_ms: now_ms,
            last_latency_ms: self.last_latency_ms,
            avg_latency_ms: self.avg_latency_ms,
            min_latency_ms: self.min_latency_ms,
            max_latency_ms: self.max_latency_ms,
            sample_count: self.sample_count,
            last_jitter_ms: self.last_jitter_ms,
            avg_jitter_ms: self.avg_jitter_ms,
            max_jitter_ms: self.max_jitter_ms,
            p50_ms: histogram.p50(),
            p95_ms: histogram.p95(),
            p99_ms: histogram.p99(),
            ema_latency_ms: self.ema_latency_ms,
            ema_jitter_ms: self.ema_jitter_ms,
            kalman_latency_estimate: kalman_lat,
            kalman_latency_velocity: kalman_vel,
            kalman_latency_residual: kalman_res,
            kalman_2d_latency: kalman_2d_lat,
            kalman_2d_jitter: kalman_2d_jit,
            kalman_2d_latency_residual: kalman_2d_lat_res,
            kalman_2d_jitter_residual: kalman_2d_jit_res,
            smoothed_latency: smoothed,
            server_abundance: self.last_abundance,
            server_health: self.last_health,
            server_council_engagement: self.last_council_engagement,
        }
    }
}

// ============================================================
// CORE MONITORING STRUCTS
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

    pub fn p50(&self) -> u64 { self.percentile(0.5) }
    pub fn p95(&self) -> u64 { self.percentile(0.95) }
    pub fn p99(&self) -> u64 { self.percentile(0.99) }

    fn percentile(&self, p: f32) -> u64 {
        if self.total_samples == 0 { return 0; }
        let target = (self.total_samples as f32 * p.clamp(0.0, 1.0)) as u32;
        let mut cum = 0u32;
        let edges = [10u64, 25, 50, 100, 200, 500, 1000, u64::MAX];
        for (i, &c) in self.buckets.iter().enumerate() {
            cum += c;
            if cum >= target { return edges[i]; }
        }
        edges.last().copied().unwrap_or(0)
    }
}

// 1D Kalman with residual tracking
#[derive(Clone, Debug)]
pub struct KalmanFilter1D {
    pub estimate: f32,
    pub velocity: f32,
    pub last_residual: f32,
    process_noise: f32,
    measurement_noise: f32,
    error_estimate: f32,
    error_velocity: f32,
}

impl KalmanFilter1D {
    pub fn new(initial: f32) -> Self {
        Self {
            estimate: initial,
            velocity: 0.0,
            last_residual: 0.0,
            process_noise: 0.1,
            measurement_noise: 15.0,
            error_estimate: 1.0,
            error_velocity: 1.0,
        }
    }

    pub fn update(&mut self, measurement: f32, dt: f32) -> f32 {
        self.estimate += self.velocity * dt;
        self.error_estimate += dt * (self.error_velocity + self.process_noise);
        self.error_velocity += self.process_noise;

        let residual = measurement - self.estimate;
        self.last_residual = residual;

        let gain = self.error_estimate / (self.error_estimate + self.measurement_noise);
        self.estimate += gain * residual;
        self.velocity += gain * (residual / dt.max(0.001));
        self.error_estimate *= (1.0 - gain);

        self.estimate
    }
}

// 2D Kalman with residual tracking
#[derive(Clone, Debug)]
pub struct KalmanFilter2D {
    pub latency: f32,
    pub jitter: f32,
    pub last_latency_residual: f32,
    pub last_jitter_residual: f32,
    process_noise: f32,
    measurement_noise: f32,
    error_cov: f32,
}

impl KalmanFilter2D {
    pub fn new(lat: f32, jit: f32) -> Self {
        Self {
            latency: lat,
            jitter: jit,
            last_latency_residual: 0.0,
            last_jitter_residual: 0.0,
            process_noise: 0.15,
            measurement_noise: 20.0,
            error_cov: 1.0,
        }
    }

    pub fn update(&mut self, m_lat: f32, m_jit: f32, dt: f32) {
        let alpha = 1.0 - (-dt / 0.6).exp().clamp(0.0, 0.95);

        let res_lat = m_lat - self.latency;
        let res_jit = m_jit - self.jitter;

        self.last_latency_residual = res_lat;
        self.last_jitter_residual = res_jit;

        self.latency += alpha * res_lat + 0.1 * alpha * res_jit;
        self.jitter += alpha * res_jit + 0.1 * alpha * res_lat;
    }
}

// Heuristic Fixed-Lag Smoother (legacy)
#[derive(Clone, Debug)]
pub struct FixedLagKalmanSmoother {
    pub smoothed_estimate: f32,
    history: Vec<f32>,
    lag: usize,
}

impl FixedLagKalmanSmoother {
    pub fn new(lag: usize) -> Self {
        Self { smoothed_estimate: 0.0, history: Vec::with_capacity(lag + 1), lag }
    }

    pub fn update(&mut self, new_est: f32) {
        self.history.push(new_est);
        if self.history.len() > self.lag { self.history.remove(0); }
        if self.history.len() < 3 { self.smoothed_estimate = new_est; return; }
        let mut s = *self.history.last().unwrap();
        for &v in self.history.iter().rev().skip(1) { s = 0.7 * s + 0.3 * v; }
        self.smoothed_estimate = s;
    }
}

// RTS Fixed-Lag Backward Smoother
#[derive(Clone, Debug)]
pub struct RTSFixedLagSmoother {
    pub smoothed_estimate: f32,
    history: Vec<RTSState>,
    lag: usize,
}

#[derive(Clone, Debug)]
struct RTSState {
    estimate: f32,
    predicted: f32,
    covariance: f32,
    transition: f32,
}

impl RTSFixedLagSmoother {
    pub fn new(lag: usize) -> Self {
        Self { smoothed_estimate: 0.0, history: Vec::with_capacity(lag + 1), lag }
    }

    pub fn update(&mut self, new_estimate: f32, new_covariance: f32, dt: f32) {
        let transition = 1.0;

        let predicted = if let Some(last) = self.history.last() {
            last.estimate * transition
        } else {
            new_estimate
        };

        let state = RTSState {
            estimate: new_estimate,
            predicted,
            covariance: new_covariance.max(0.1),
            transition,
        };

        self.history.push(state);
        if self.history.len() > self.lag { self.history.remove(0); }

        if self.history.len() < 2 {
            self.smoothed_estimate = new_estimate;
            return;
        }

        // RTS Backward Pass
        let mut smoothed = self.history.last().unwrap().estimate;
        let mut smoothed_cov = self.history.last().unwrap().covariance;

        for i in (0..self.history.len() - 1).rev() {
            let curr = &self.history[i];
            let next = &self.history[i + 1];

            let pred_cov = curr.covariance * curr.transition * curr.transition + 0.1;
            let smoother_gain = curr.covariance * curr.transition / pred_cov.max(0.01);

            smoothed = curr.estimate + smoother_gain * (smoothed - next.predicted);
            smoothed_cov = curr.covariance + smoother_gain * smoother_gain * (smoothed_cov - pred_cov);
        }

        self.smoothed_estimate = smoothed;
    }
}

// ============================================================
// SAFETY NET STATE
// ============================================================

#[derive(Resource, Clone)]
pub struct SafetyNetState {
    pub last_tick: u64,
    pub last_abundance: f64,
    pub last_health: f32,
    pub last_council_engagement: f32,
    pub pending_events: Vec<String>,

    pub last_latency_ms: u64,
    pub avg_latency_ms: f32,
    pub max_latency_ms: u64,
    pub min_latency_ms: u64,
    pub sample_count: u32,

    pub latency_histogram: LatencyHistogram,
    pub last_jitter_ms: u64,
    pub avg_jitter_ms: f32,
    pub max_jitter_ms: u64,
    previous_latency_ms: u64,

    pub ema_latency_ms: f32,
    pub ema_jitter_ms: f32,
    ema_time_constant: f32,
    last_ema_update_ms: u64,

    pub kalman_latency: Option<KalmanFilter1D>,
    pub kalman_jitter: Option<KalmanFilter1D>,
    pub kalman_2d: Option<KalmanFilter2D>,
    pub smoother_latency: Option<FixedLagKalmanSmoother>,
    pub rts_smoother: Option<RTSFixedLagSmoother>,
}

impl Default for SafetyNetState {
    fn default() -> Self {
        Self {
            last_tick: 0,
            last_abundance: 0.0,
            last_health: 100.0,
            last_council_engagement: 0.0,
            pending_events: Vec::new(),
            last_latency_ms: 0,
            avg_latency_ms: 0.0,
            max_latency_ms: 0,
            min_latency_ms: u64::MAX,
            sample_count: 0,
            latency_histogram: LatencyHistogram::new(),
            last_jitter_ms: 0,
            avg_jitter_ms: 0.0,
            max_jitter_ms: 0,
            previous_latency_ms: 0,
            ema_latency_ms: 0.0,
            ema_jitter_ms: 0.0,
            ema_time_constant: 0.8,
            last_ema_update_ms: 0,
            kalman_latency: None,
            kalman_jitter: None,
            kalman_2d: None,
            smoother_latency: None,
            rts_smoother: None,
        }
    }
}

#[derive(Resource)]
pub struct RbeClientSync {
    pub local_inventory: Arc<RwLock<LocalInventory>>,
    pub trade_state: Arc<RwLock<TradeUIState>>,
    pub gpu_state: Arc<RwLock<GpuSimulationState>>,
    pub safety_net_state: Arc<RwLock<SafetyNetState>>,
}

impl RbeClientSync {
    pub fn new() -> Self {
        Self {
            local_inventory: Arc::new(RwLock::new(LocalInventory::default())),
            trade_state: Arc::new(RwLock::new(TradeUIState::default())),
            gpu_state: Arc::new(RwLock::new(GpuSimulationState::default())),
            safety_net_state: Arc::new(RwLock::new(SafetyNetState::default())),
        }
    }

    pub async fn handle_server_binary_message(
        &self,
        data: Bytes,
        inventory_events: &mut EventWriter<InventoryUpdated>,
        trade_events: &mut EventWriter<TradeResponseReceived>,
        harvest_events: &mut EventWriter<HarvestResponseReceived>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        divine_current: &mut CurrentDivineWhisper,
        divine_log: &mut DivineWhispersLog,
        divine_ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
        mut monitoring_events: EventWriter<SafetyNetMonitoringUpdate>,
    ) {
        if let Ok(msg) = bincode::deserialize::<ServerMessage>(&data) {
            let mut inv = self.local_inventory.write().await;
            let mut trade = self.trade_state.write().await;

            handle_server_message(&msg, &mut inv, &mut trade, inventory_events, trade_events, harvest_events);

            if let ServerMessage::GpuPatsagiUpdate { global_confidence, node_predictions, notes } = &msg {
                let mut gpu = self.gpu_state.write().await;
                gpu.global_confidence = *global_confidence;
                gpu.node_predictions = node_predictions.clone();
                gpu.last_update_notes = notes.clone();
            }

            if let ServerMessage::DivineWhisperReceived { whisper } = &msg {
                receive_divine_whisper_from_server(whisper.clone(), divine_current, divine_log, divine_ui_query, commands, asset_server);
            }

            if let ServerMessage::SafetyNetBroadcast { broadcast } = &msg {
                self.handle_safety_net_broadcast(broadcast, &mut monitoring_events).await;
            }
        }
    }

    async fn handle_safety_net_broadcast(
        &self,
        broadcast: &SafetyNetBroadcast,
        monitoring_events: &mut EventWriter<SafetyNetMonitoringUpdate>,
    ) {
        let mut safety = self.safety_net_state.write().await;

        safety.last_tick = broadcast.snapshot.tick;
        safety.last_abundance = broadcast.snapshot.abundance;
        safety.last_health = broadcast.snapshot.current_health;
        safety.last_council_engagement = broadcast.snapshot.council_engagement_score;

        let now_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
        let latency_ms = if broadcast.emit_timestamp_ms > 0 { now_ms.saturating_sub(broadcast.emit_timestamp_ms) } else { 0 };
        let jitter_ms = if safety.previous_latency_ms > 0 { (latency_ms as i64 - safety.previous_latency_ms as i64).unsigned_abs() } else { 0 };

        safety.last_latency_ms = latency_ms;
        safety.sample_count = safety.sample_count.saturating_add(1);

        if safety.sample_count == 1 {
            safety.avg_latency_ms = latency_ms as f32;
            safety.min_latency_ms = latency_ms;
            safety.max_latency_ms = latency_ms;

            safety.ema_latency_ms = latency_ms as f32;
            safety.ema_jitter_ms = 0.0;
            safety.last_ema_update_ms = now_ms;

            safety.kalman_latency = Some(KalmanFilter1D::new(latency_ms as f32));
            safety.kalman_jitter = Some(KalmanFilter1D::new(jitter_ms as f32));
            safety.kalman_2d = Some(KalmanFilter2D::new(latency_ms as f32, jitter_ms as f32));
            safety.smoother_latency = Some(FixedLagKalmanSmoother::new(8));
            safety.rts_smoother = Some(RTSFixedLagSmoother::new(8));
        } else {
            safety.avg_latency_ms = (safety.avg_latency_ms * (safety.sample_count - 1) as f32 + latency_ms as f32) / safety.sample_count as f32;
            if latency_ms < safety.min_latency_ms { safety.min_latency_ms = latency_ms; }
            if latency_ms > safety.max_latency_ms { safety.max_latency_ms = latency_ms; }

            let dt_ms = now_ms.saturating_sub(safety.last_ema_update_ms);
            let dt_sec = dt_ms as f32 / 1000.0;
            let tau = safety.ema_time_constant;
            let alpha = if dt_sec > 0.0 { 1.0 - (-dt_sec / tau).exp() } else { 0.0 };

            safety.ema_latency_ms = alpha * (latency_ms as f32) + (1.0 - alpha) * safety.ema_latency_ms;
            safety.ema_jitter_ms = alpha * (jitter_ms as f32) + (1.0 - alpha) * safety.ema_jitter_ms;
            safety.last_ema_update_ms = now_ms;

            let mut kalman_estimate = latency_ms as f32;
            let mut kalman_cov = 1.0;

            if let Some(k) = &mut safety.kalman_latency {
                kalman_estimate = k.update(latency_ms as f32, dt_sec.max(0.001));
                kalman_cov = k.error_estimate.max(0.1);
            }
            if let Some(k) = &mut safety.kalman_jitter { k.update(jitter_ms as f32, dt_sec.max(0.001)); }
            if let Some(k2d) = &mut safety.kalman_2d { k2d.update(latency_ms as f32, jitter_ms as f32, dt_sec.max(0.001)); }

            if let Some(s) = &mut safety.smoother_latency { s.update(kalman_estimate); }
            if let Some(rts) = &mut safety.rts_smoother {
                rts.update(kalman_estimate, kalman_cov, dt_sec.max(0.001));
            }
        }

        safety.last_jitter_ms = jitter_ms;
        if safety.sample_count > 1 {
            if safety.sample_count == 2 {
                safety.avg_jitter_ms = jitter_ms as f32;
                safety.max_jitter_ms = jitter_ms;
            } else {
                safety.avg_jitter_ms = (safety.avg_jitter_ms * (safety.sample_count - 2) as f32 + jitter_ms as f32) / (safety.sample_count - 1) as f32;
                if jitter_ms > safety.max_jitter_ms { safety.max_jitter_ms = jitter_ms; }
            }
        }

        safety.previous_latency_ms = latency_ms;
        safety.latency_histogram.record(latency_ms);

        if safety.sample_count % 5 == 0 {
            let snapshot = safety.get_monitoring_snapshot(now_ms);
            monitoring_events.send(SafetyNetMonitoringUpdate { snapshot });

            let rts_val = safety.rts_smoother.as_ref().map_or(0.0, |r| r.smoothed_estimate);
            tracing::info!(
                "[SafetyNet][RTS] RTS={:.1} | res={:.1} | samples={}",
                rts_val,
                snapshot.kalman_latency_residual,
                snapshot.sample_count
            );
        }

        if latency_ms > 150 || jitter_ms > 50 {
            tracing::warn!("[SafetyNet] High latency/jitter detected");
        }

        if let Some(event) = &broadcast.event {
            if let SafetyNetEvent::AbundanceSafetyNetTriggered { restored_amount, .. } = event {
                tracing::warn!("[SafetyNet] Abundance safety net triggered: +{:.2}", restored_amount);
            }
        }
    }

    // ... other methods remain the same ...
}