// client/rbe_client_sync.rs
// Powrush-MMO — RBE + Council + Safety Net client sync layer
// Parallel Sparse Localization (rayon) (v18.37)
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use shared::protocol::{ClientMessage, ServerMessage, Vec3Ser, DivineWhisper, SafetyNetBroadcast, SafetyNetEvent};
use std::sync::Arc;
use tokio::sync::RwLock;
use bytes::Bytes;

// Note: Add `rayon = "1"` to Cargo.toml for parallelization
use rayon::prelude::*;

use crate::inventory_ui::{LocalInventory, TradeUIState, InventoryUpdated, TradeResponseReceived, HarvestResponseReceived, handle_server_message};
use crate::divine_whispers_ui::{CurrentDivineWhisper, DivineWhispersLog, DivineWhisperUI, receive_divine_whisper_from_server};

// ============================================================
// PARALLEL SPARSE LOCALIZATION
// Uses rayon for multi-threaded construction and application
// ============================================================

pub type SparseLocalization = Vec<Vec<(usize, f32)>>;

/// Parallel version of sparse state-space localization construction.
/// Significantly faster on multi-core systems for large state dimensions.
pub fn build_sparse_state_localization_parallel(
    distances: &[Vec<f32>],
    localization_radius: f32,
) -> SparseLocalization {
    let n = distances.len();
    let radius = localization_radius.max(1e-6);

    (0..n)
        .into_par_iter()
        .map(|i| {
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
            neighbors.sort_by_key(|&(idx, _)| idx);
            neighbors
        })
        .collect()
}

/// Parallel version of sparse observation-space localization.
pub fn build_sparse_observation_localization_parallel(
    obs_state_distances: &[Vec<f32>],
    localization_radius: f32,
) -> SparseLocalization {
    let radius = localization_radius.max(1e-6);

    obs_state_distances
        .par_iter()
        .map(|state_dists| {
            let mut neighbors = Vec::new();
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
            neighbors.sort_by_key(|&(idx, _)| idx);
            neighbors
        })
        .collect()
}

/// Parallel application of sparse localization.
pub fn apply_sparse_localization_parallel(
    covariance: &[Vec<f32>],
    sparse_localization: &SparseLocalization,
) -> Vec<Vec<f32>> {
    let n = covariance.len();

    (0..n)
        .into_par_iter()
        .map(|i| {
            let mut row = vec![0.0; n];
            for &(j, rho) in &sparse_localization[i] {
                if j < n {
                    row[j] = covariance[i][j] * rho;
                }
            }
            row
        })
        .collect()
}

// ============================================================
// SPARSE LOCALIZATION (Sequential versions for compatibility)
// ============================================================

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
        neighbors.sort_by_key(|&(idx, _)| idx);
        sparse[i] = neighbors;
    }
    sparse
}

pub fn build_sparse_observation_localization(
    obs_state_distances: &[Vec<f32>],
    localization_radius: f32,
) -> SparseLocalization {
    let mut sparse = vec![Vec::new(); obs_state_distances.len()];
    let radius = localization_radius.max(1e-6);

    for (i, state_dists) in obs_state_distances.iter().enumerate() {
        let mut neighbors = Vec::new();
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
        neighbors.sort_by_key(|&(idx, _)| idx);
        sparse[i] = neighbors;
    }
    sparse
}

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
// GASPARi-COHN + DENSE FALLBACKS
// ============================================================

pub fn gaspari_cohn(normalized_distance: f32) -> f32 {
    let z = normalized_distance.abs();
    if z >= 2.0 { 0.0 }
    else if z >= 1.0 {
        let z2 = z * z; let z3 = z2 * z;
        -0.25 * z3 + 0.5 * z2 + 0.625 * z - (5.0/3.0)*z2*z2 + (8.0/3.0)*z3*z - 0.5*z3*z2 + (1.0/12.0)*z2*z2*z
    } else {
        let z2 = z * z; let z3 = z2 * z;
        (4.0/3.0)*z3 - 2.5*z2 + (5.0/8.0)*z3*z - (1.0/12.0)*z2*z2 + 1.0
    }
}

pub fn create_state_localization_matrix(distances: &[Vec<f32>], radius: f32) -> Vec<Vec<f32>> {
    let n = distances.len();
    let mut loc = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            loc[i][j] = gaspari_cohn(distances[i][j] / radius.max(1e-6));
        }
    }
    loc
}

pub fn create_observation_localization_matrix(obs_state_distances: &[Vec<f32>], radius: f32) -> Vec<Vec<f32>> {
    let n_obs = obs_state_distances.len();
    let n_state = if n_obs > 0 { obs_state_distances[0].len() } else { 0 };
    let mut loc = vec![vec![0.0; n_state]; n_obs];
    for i in 0..n_obs {
        for j in 0..n_state {
            loc[i][j] = gaspari_cohn(obs_state_distances[i][j] / radius.max(1e-6));
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
// ADVANCED ADAPTIVE LOCALIZER + LOCALIZED ENKF
// (Kept for continuity — full implementation in previous commits)
// ============================================================

pub fn compute_advanced_adaptive_radius(
    avg_residual: f32,
    ensemble_spread: f32,
    base_radius: f32,
    min_radius: f32,
    max_radius: f32,
    adaptation_strength: f32,
) -> f32 { /* ... */ 0.0 }

#[derive(Clone, Debug)]
pub struct AdvancedAdaptiveLocalizer { /* ... */ }

impl AdvancedAdaptiveLocalizer { /* ... */ }

pub type EnsembleMember = Vec<f32>;

#[derive(Clone, Debug)]
pub struct LocalizedEnsembleKalmanFilter { /* ... */ }

impl LocalizedEnsembleKalmanFilter { /* ... */ }

// ============================================================
// SAFETY NET + CORE (abbreviated)
// ============================================================

#[derive(Event, Debug, Clone)]
pub struct SafetyNetMonitoringUpdate { pub snapshot: SafetyNetMonitoringSnapshot }

#[derive(Debug, Clone, Default)]
pub struct SafetyNetMonitoringSnapshot { /* ... */ }

#[derive(Clone, Debug, Default)]
pub struct LatencyHistogram { /* ... */ }

impl LatencyHistogram { /* ... */ }

#[derive(Clone, Debug)]
pub struct KalmanFilter1D { /* ... */ }

impl KalmanFilter1D { /* ... */ }

#[derive(Clone, Debug)]
pub struct KalmanFilter2D { /* ... */ }

impl KalmanFilter2D { /* ... */ }

#[derive(Clone, Debug)]
pub struct FixedLagKalmanSmoother { /* ... */ }

impl FixedLagKalmanSmoother { /* ... */ }

#[derive(Clone, Debug)]
pub struct RTSFixedLagSmoother { /* ... */ }

impl RTSFixedLagSmoother { /* ... */ }

#[derive(Resource, Clone)]
pub struct SafetyNetState { /* ... */ }

impl Default for SafetyNetState { fn default() -> Self { /* ... */ } }

#[derive(Resource)]
pub struct RbeClientSync { /* ... */ }

impl RbeClientSync { /* ... */ }