// client/monitoring/localization.rs
// Ra-Thor Localization Module
// Dense + Sparse + Parallel localization with Gaspari-Cohn kernel

use rayon::prelude::*;

pub type SparseLocalization = Vec<Vec<(usize, f32)>>;

/// Gaspari-Cohn fifth-order piecewise rational function (standard EnKF localization kernel)
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
// PARALLEL SPARSE LOCALIZATION (Recommended for large state spaces)
// ============================================================

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
                let rho = gaspari_cohn(d / radius);
                if rho > 0.0 {
                    neighbors.push((j, rho));
                }
            }
            neighbors.sort_by_key(|&(idx, _)| idx);
            neighbors
        })
        .collect()
}

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
                let rho = gaspari_cohn(d / radius);
                if rho > 0.0 {
                    neighbors.push((j, rho));
                }
            }
            neighbors.sort_by_key(|&(idx, _)| idx);
            neighbors
        })
        .collect()
}

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
// SEQUENTIAL SPARSE LOCALIZATION (for smaller problems / compatibility)
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
            let rho = gaspari_cohn(d / radius);
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
    sparse_row: &[(usize, f32)],
) -> Vec<(usize, f32)> {
    sparse_row
        .iter()
        .filter_map(|&(j, rho)| {
            if j < covariance_row.len() {
                let val = covariance_row[j] * rho;
                if val.abs() > 1e-12 {
                    Some((j, val))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

// ============================================================
// DENSE LOCALIZATION (for small state spaces)
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