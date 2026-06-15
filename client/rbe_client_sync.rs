// client/rbe_client_sync.rs
// Powrush-MMO — RBE + Council + Safety Net client sync layer
// Sparse Neighbor-List Localization Implementation (v18.37)
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, Vec3Ser, DivineWhisper, SafetyNetBroadcast, SafetyNetEvent};
use std::sync::Arc;
use tokio::sync::RwLock;
use bytes::Bytes;

use crate::inventory_ui::{LocalInventory, TradeUIState, InventoryUpdated, TradeResponseReceived, HarvestResponseReceived, handle_server_message};
use crate::divine_whispers_ui::{CurrentDivineWhisper, DivineWhispersLog, DivineWhisperUI, receive_divine_whisper_from_server};

// ============================================================
// SPARSE LOCALIZATION (Neighbor-List Representation)
// Much more scalable than dense matrices for high dimensions
// ============================================================

/// Type alias for sparse localization: for each variable i, store list of (j, rho_ij)
/// where rho_ij > 0 according to the localization kernel.
pub type SparseLocalization = Vec<Vec<(usize, f32)>>;

/// Builds a sparse state-space localization structure using neighbor lists.
/// Only stores entries where Gaspari-Cohn value > 0.
/// This is far more memory efficient than dense matrices when n is large.
pub fn build_sparse_state_localization(
    distances: &[Vec<f32>],
    localization_radius: f32,
) -> SparseLocalization {
    let n = distances.len();
    let mut sparse = vec![Vec::new(); n];

    let radius = localization_radius.max(1e-6);

    for i in 0..n {
        let mut neighbors = Vec::new();
        for j in 0..n {
            if i == j {
                neighbors.push((j, 1.0));
                continue;
            }
            let d = distances[i][j];
            if d >= 2.0 * radius {
                continue;
            }
            let norm_d = d / radius;
            let rho = gaspari_cohn(norm_d);
            if rho > 0.0 {
                neighbors.push((j, rho));
            }
        }
        // Sort by index for potential future optimizations
        neighbors.sort_by_key(|&(idx, _)| idx);
        sparse[i] = neighbors;
    }

    sparse
}

/// Builds a sparse observation-space localization structure.
pub fn build_sparse_observation_localization(
    obs_state_distances: &[Vec<f32>],
    localization_radius: f32,
) -> SparseLocalization {
    let n_obs = obs_state_distances.len();
    let mut sparse = vec![Vec::new(); n_obs];

    let radius = localization_radius.max(1e-6);

    for i in 0..n_obs {
        let mut neighbors = Vec::new();
        if let Some(state_dists) = obs_state_distances.get(i) {
            for (j, &d) in state_dists.iter().enumerate() {
                if d >= 2.0 * radius {
                    continue;
                }
                let norm_d = d / radius;
                let rho = gaspari_cohn(norm_d);
                if rho > 0.0 {
                    neighbors.push((j, rho));
                }
            }
        }
        neighbors.sort_by_key(|&(idx, _)| idx);
        sparse[i] = neighbors;
    }

    sparse
}

/// Applies sparse localization to a covariance matrix using neighbor lists.
/// This is more efficient than dense Schur product when the matrix is large.
pub fn apply_sparse_localization(
    covariance: &[Vec<f32>],
    sparse_localization: &SparseLocalization,
) -> Vec<Vec<f32>> {
    let n = covariance.len();
    let mut localized = vec![vec![0.0; n]; n];

    for i in 0..n {
        for &(j, rho) in &sparse_localization[i] {
            if j < n {
                localized[i][j] = covariance[i][j] * rho;
            }
        }
    }

    localized
}

/// More efficient version: localize only the relevant parts during EnKF analysis.
/// Returns a localized covariance row for variable i.
pub fn get_localized_covariance_row(
    covariance_row: &[f32],
    sparse_localization_row: &[(usize, f32)],
) -> Vec<(usize, f32)> {
    let mut localized_row = Vec::new();
    for &(j, rho) in sparse_localization_row {
        if j < covariance_row.len() {
            let val = covariance_row[j] * rho;
            if val.abs() > 1e-12 {
                localized_row.push((j, val));
            }
        }
    }
    localized_row
}

// ============================================================
// GASPARi-COHN KERNEL
// ============================================================

pub fn gaspari_cohn(normalized_distance: f32) -> f32 {
    let z = normalized_distance.abs();
    if z >= 2.0 {
        0.0
    } else if z >= 1.0 {
        let z2 = z * z;
        let z3 = z2 * z;
        -0.25 * z3 + 0.5 * z2 + 0.625 * z
            - (5.0 / 3.0) * z2 * z2
            + (8.0 / 3.0) * z3 * z
            - 0.5 * z3 * z2
            + (1.0 / 12.0) * z2 * z2 * z
    } else {
        let z2 = z * z;
        let z3 = z2 * z;
        (4.0 / 3.0) * z3 - 2.5 * z2 + (5.0 / 8.0) * z3 * z
            - (1.0 / 12.0) * z2 * z2 + 1.0
    }
}

// ============================================================
// DENSE LOCALIZATION (kept for compatibility and smaller problems)
// ============================================================

pub fn create_state_localization_matrix(
    distances: &[Vec<f32>],
    localization_radius: f32,
) -> Vec<Vec<f32>> {
    let n = distances.len();
    let mut loc = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            loc[i][j] = gaspari_cohn(distances[i][j] / localization_radius.max(1e-6));
        }
    }
    loc
}

pub fn create_observation_localization_matrix(
    obs_state_distances: &[Vec<f32>],
    localization_radius: f32,
) -> Vec<Vec<f32>> {
    let n_obs = obs_state_distances.len();
    let n_state = if n_obs > 0 { obs_state_distances[0].len() } else { 0 };
    let mut loc = vec![vec![0.0; n_state]; n_obs];
    for i in 0..n_obs {
        for j in 0..n_state {
            loc[i][j] = gaspari_cohn(obs_state_distances[i][j] / localization_radius.max(1e-6));
        }
    }
    loc
}

pub fn apply_localization(cov: &[Vec<f32>], loc_matrix: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let n = cov.len();
    let mut out = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            out[i][j] = cov[i][j] * loc_matrix[i][j];
        }
    }
    out
}

// ============================================================
// ADVANCED ADAPTIVE LOCALIZATION
// ============================================================

pub fn compute_advanced_adaptive_radius(
    avg_residual: f32,
    ensemble_spread: f32,
    base_radius: f32,
    min_radius: f32,
    max_radius: f32,
    adaptation_strength: f32,
) -> f32 {
    let residual_factor = (1.0 / (1.0 + avg_residual * 0.8)).clamp(0.4, 1.3);
    let spread_factor = if ensemble_spread < 0.5 {
        1.15
    } else if ensemble_spread > 3.0 {
        0.9
    } else {
        1.0
    };
    let adaptive = base_radius * residual_factor * spread_factor;
    let smoothed = base_radius * (1.0 - adaptation_strength) + adaptive * adaptation_strength;
    smoothed.clamp(min_radius, max_radius)
}

#[derive(Clone, Debug)]
pub struct AdvancedAdaptiveLocalizer {
    pub current_radius: f32,
    base_radius: f32,
    min_radius: f32,
    max_radius: f32,
    adaptation_strength: f32,
    residual_history: Vec<f32>,
    spread_history: Vec<f32>,
    history_size: usize,
}

impl AdvancedAdaptiveLocalizer {
    pub fn new(
        initial_radius: f32,
        min_radius: f32,
        max_radius: f32,
        adaptation_strength: f32,
        history_size: usize,
    ) -> Self {
        Self {
            current_radius: initial_radius,
            base_radius: initial_radius,
            min_radius,
            max_radius,
            adaptation_strength,
            residual_history: Vec::with_capacity(history_size),
            spread_history: Vec::with_capacity(history_size),
            history_size,
        }
    }

    pub fn update(&mut self, new_residual: f32, new_spread: f32) {
        self.residual_history.push(new_residual);
        self.spread_history.push(new_spread);
        if self.residual_history.len() > self.history_size {
            self.residual_history.remove(0);
        }
        if self.spread_history.len() > self.history_size {
            self.spread_history.remove(0);
        }

        let avg_residual = if self.residual_history.is_empty() {
            new_residual
        } else {
            self.residual_history.iter().sum::<f32>() / self.residual_history.len() as f32
        };
        let avg_spread = if self.spread_history.is_empty() {
            new_spread
        } else {
            self.spread_history.iter().sum::<f32>() / self.spread_history.len() as f32
        };

        self.current_radius = compute_advanced_adaptive_radius(
            avg_residual,
            avg_spread,
            self.base_radius,
            self.min_radius,
            self.max_radius,
            self.adaptation_strength,
        );
    }

    pub fn get_current_radius(&self) -> f32 {
        self.current_radius
    }
}

// ============================================================
// LOCALIZED ENSEMBLE KALMAN FILTER (Foundation)
// ============================================================

pub type EnsembleMember = Vec<f32>;

#[derive(Clone, Debug)]
pub struct LocalizedEnsembleKalmanFilter {
    pub ensemble: Vec<EnsembleMember>,
    pub state_dim: usize,
    pub ensemble_size: usize,
    pub localization_radius: f32,
    pub adaptive_localizer: Option<AdvancedAdaptiveLocalizer>,
    pub last_mean: Vec<f32>,
    pub last_spread: f32,
}

impl LocalizedEnsembleKalmanFilter {
    pub fn new(
        initial_state: Vec<f32>,
        ensemble_size: usize,
        initial_localization_radius: f32,
    ) -> Self {
        let state_dim = initial_state.len();
        let mut ensemble = Vec::with_capacity(ensemble_size);

        for _ in 0..ensemble_size {
            let mut member = initial_state.clone();
            for val in &mut member {
                *val += rand::random::<f32>() * 0.5 - 0.25;
            }
            ensemble.push(member);
        }

        let mean = Self::compute_mean(&ensemble);
        let spread = Self::compute_spread(&ensemble, &mean);

        Self {
            ensemble,
            state_dim,
            ensemble_size,
            localization_radius: initial_localization_radius,
            adaptive_localizer: None,
            last_mean: mean,
            last_spread: spread,
        }
    }

    pub fn enable_adaptive_localization(
        &mut self,
        min_radius: f32,
        max_radius: f32,
        adaptation_strength: f32,
        history_size: usize,
    ) {
        self.adaptive_localizer = Some(AdvancedAdaptiveLocalizer::new(
            self.localization_radius,
            min_radius,
            max_radius,
            adaptation_strength,
            history_size,
        ));
    }

    pub fn forecast(&mut self, process_noise_std: f32) {
        for member in &mut self.ensemble {
            for val in member {
                *val += rand::random::<f32>() * process_noise_std * 2.0 - process_noise_std;
            }
        }
        self.update_statistics();
    }

    pub fn analyze(
        &mut self,
        observation: &[f32],
        obs_operator: &[Vec<f32>],
        observation_noise: f32,
        state_distances: Option<&[Vec<f32>]>,
    ) {
        let n = self.state_dim;
        let mean = Self::compute_mean(&self.ensemble);

        let mut cov = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut sum = 0.0;
                for member in &self.ensemble {
                    sum += (member[i] - mean[i]) * (member[j] - mean[j]);
                }
                cov[i][j] = sum / (self.ensemble_size as f32 - 1.0).max(1.0);
            }
        }

        let localized_cov = if let Some(distances) = state_distances {
            // Use sparse version when possible
            let sparse_loc = build_sparse_state_localization(distances, self.localization_radius);
            apply_sparse_localization(&cov, &sparse_loc)
        } else {
            cov
        };

        // Simplified gain for foundation
        let mut gain = vec![vec![0.0; observation.len()]; n];
        for i in 0..n {
            for j in 0..observation.len() {
                gain[i][j] = localized_cov[i][i] / (localized_cov[i][i] + observation_noise).max(1e-6);
            }
        }

        for member in &mut self.ensemble {
            for i in 0..n {
                let mut innovation = 0.0;
                for (j, &obs_val) in observation.iter().enumerate() {
                    innovation += gain[i][j] * (obs_val - member[i]);
                }
                member[i] += innovation;
            }
        }

        self.update_statistics();

        if let Some(ref mut localizer) = self.adaptive_localizer {
            let residual = self.compute_innovation_magnitude(observation);
            localizer.update(residual, self.last_spread);
            self.localization_radius = localizer.get_current_radius();
        }
    }

    fn compute_innovation_magnitude(&self, observation: &[f32]) -> f32 {
        let mean = &self.last_mean;
        let mut sum_sq = 0.0;
        for (i, &obs_val) in observation.iter().enumerate() {
            if i < mean.len() {
                sum_sq += (obs_val - mean[i]).powi(2);
            }
        }
        sum_sq.sqrt()
    }

    fn update_statistics(&mut self) {
        self.last_mean = Self::compute_mean(&self.ensemble);
        self.last_spread = Self::compute_spread(&self.ensemble, &self.last_mean);
    }

    fn compute_mean(ensemble: &[EnsembleMember]) -> Vec<f32> {
        let n = ensemble[0].len();
        let mut mean = vec![0.0; n];
        for member in ensemble {
            for (i, &val) in member.iter().enumerate() {
                mean[i] += val;
            }
        }
        for val in &mut mean {
            *val /= ensemble.len() as f32;
        }
        mean
    }

    fn compute_spread(ensemble: &[EnsembleMember], mean: &[f32]) -> f32 {
        let mut sum = 0.0;
        for member in ensemble {
            for (i, &val) in member.iter().enumerate() {
                sum += (val - mean[i]).powi(2);
            }
        }
        (sum / (ensemble.len() as f32 * mean.len() as f32)).sqrt()
    }

    pub fn get_mean(&self) -> &[f32] {
        &self.last_mean
    }

    pub fn get_spread(&self) -> f32 {
        self.last_spread
    }
}

// ============================================================
// SAFETY NET MONITORING EVENT
// ============================================================

#[derive(Event, Debug, Clone)]
pub struct SafetyNetMonitoringUpdate {
    pub snapshot: SafetyNetMonitoringSnapshot,
}

// ============================================================
// RTS SMOOTHED WINDOW
// ============================================================

#[derive(Debug, Clone, Default)]
pub struct RTSSmoothedWindow {
    pub smoothed_estimates: Vec<f32>,
    pub smoothed_covariances: Vec<f32>,
    pub window_size: usize,
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
    pub rts_smoothed_latency: f32,
    pub rts_vs_kalman_residual: f32,
    pub kalman_2d_latency: f32,
    pub kalman_2d_jitter: f32,
    pub kalman_2d_latency_residual: f32,
    pub kalman_2d_jitter_residual: f32,
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

        let rts_smoothed = self.rts_smoother.as_ref().map_or(0.0, |r| r.smoothed_estimate);
        let rts_vs_kalman = rts_smoothed - kalman_lat;

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
            rts_smoothed_latency: rts_smoothed,
            rts_vs_kalman_residual: rts_vs_kalman,
            kalman_2d_latency: kalman_2d_lat,
            kalman_2d_jitter: kalman_2d_jit,
            kalman_2d_latency_residual: kalman_2d_lat_res,
            kalman_2d_jitter_residual: kalman_2d_jit_res,
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

// ============================================================
// SAFETY NET STATE + HANDLERS (abbreviated for space)
// ============================================================

#[derive(Resource, Clone)]
pub struct SafetyNetState { /* ... */ }

impl Default for SafetyNetState { fn default() -> Self { /* ... */ } }

#[derive(Resource)]
pub struct RbeClientSync { /* ... */ }

impl RbeClientSync { /* ... */ }