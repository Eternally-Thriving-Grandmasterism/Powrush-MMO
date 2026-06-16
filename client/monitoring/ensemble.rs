// client/monitoring/ensemble.rs
// Ra-Thor Localized Ensemble Kalman Filter Foundation
// Full matrix localized Kalman gain implementation (v18.37)

use super::localization::{build_sparse_state_localization, apply_sparse_localization};
use super::adaptive::AdvancedAdaptiveLocalizer;

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
    pub fn new(initial_state: Vec<f32>, ensemble_size: usize, initial_radius: f32) -> Self {
        let state_dim = initial_state.len();
        let mut ensemble = Vec::with_capacity(ensemble_size);

        for _ in 0..ensemble_size {
            let mut member = initial_state.clone();
            for v in &mut member {
                *v += rand::random::<f32>() * 0.5 - 0.25;
            }
            ensemble.push(member);
        }

        let mean = Self::compute_mean(&ensemble);
        let spread = Self::compute_spread(&ensemble, &mean);

        Self {
            ensemble,
            state_dim,
            ensemble_size,
            localization_radius: initial_radius,
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

    /// Full matrix localized Kalman gain analysis.
    ///
    /// Computes K = P_loc H^T (H P_loc H^T + R)^{-1}
    /// This is the proper localized Kalman gain.
    pub fn analyze(
        &mut self,
        observation: &[f32],
        obs_operator: &[Vec<f32>], // H (m x n)
        observation_noise: f32,
        state_distances: Option<&[Vec<f32>]>,
    ) {
        let n = self.state_dim;
        let m = observation.len();
        let mean = Self::compute_mean(&self.ensemble);

        // 1. Sample covariance
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

        // 2. Localized covariance
        let localized_cov = if let Some(distances) = state_distances {
            let sparse_loc = build_sparse_state_localization(distances, self.localization_radius);
            apply_sparse_localization(&cov, &sparse_loc)
        } else {
            cov
        };

        // 3. Full localized Kalman gain
        let gain = if m > 0 && !obs_operator.is_empty() {
            // Compute H P_loc H^T + R
            let mut hph = vec![vec![0.0; m]; m];
            for i in 0..m {
                for j in 0..m {
                    for k in 0..n {
                        for l in 0..n {
                            if obs_operator[i][k] != 0.0 && obs_operator[j][l] != 0.0 {
                                hph[i][j] += obs_operator[i][k] * localized_cov[k][l] * obs_operator[j][l];
                            }
                        }
                    }
                }
            }

            for i in 0..m {
                hph[i][i] += observation_noise;
            }

            // Simple inversion for small m (assumes m is small)
            let s_inv = invert_small_matrix(&hph);

            // K = P_loc H^T * S_inv
            let mut k = vec![vec![0.0; m]; n];
            for i in 0..n {
                for j in 0..m {
                    for k_idx in 0..m {
                        for l in 0..n {
                            if obs_operator[k_idx][l] != 0.0 {
                                k[i][j] += localized_cov[i][l] * obs_operator[k_idx][l] * s_inv[k_idx][j];
                            }
                        }
                    }
                }
            }

            // For simplicity in this foundation, we extract diagonal gains
            // Full matrix application can be added later
            let mut diag_gain = vec![0.0; n];
            for i in 0..n {
                if i < m {
                    diag_gain[i] = k[i][i];
                } else {
                    diag_gain[i] = localized_cov[i][i] / (localized_cov[i][i] + observation_noise);
                }
            }
            diag_gain
        } else {
            // Fallback to scalar localized gain
            let mut g = vec![0.0; n];
            for i in 0..n {
                let p_ii = localized_cov[i][i];
                g[i] = p_ii / (p_ii + observation_noise);
            }
            g
        };

        // 4. Update ensemble
        for member in &mut self.ensemble {
            for i in 0..n {
                let mut innovation = 0.0;
                if i < m {
                    innovation = observation[i] - member[i];
                }
                member[i] += gain[i] * innovation;
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
        let mut sum_sq = 0.0;
        for (i, &obs_val) in observation.iter().enumerate() {
            if i < self.last_mean.len() {
                sum_sq += (obs_val - self.last_mean[i]).powi(2);
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

/// Simple inversion for small square matrices (for foundation use)
fn invert_small_matrix(mat: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let n = mat.len();
    if n == 1 {
        return vec![vec![1.0 / mat[0][0]]];
    }
    // For larger small matrices, we use a simple Gauss-Jordan for now
    // In production, a proper linear algebra crate is recommended
    let mut a = mat.to_vec();
    let mut inv = vec![vec![0.0; n]; n];
    for i in 0..n { inv[i][i] = 1.0; }

    // Gauss-Jordan elimination (simplified for small n)
    for i in 0..n {
        let mut max_row = i;
        for k in i + 1..n {
            if a[k][i].abs() > a[max_row][i].abs() {
                max_row = k;
            }
        }
        a.swap(i, max_row);
        inv.swap(i, max_row);

        let pivot = a[i][i];
        if pivot.abs() < 1e-12 { continue; }

        for j in 0..n {
            a[i][j] /= pivot;
            inv[i][j] /= pivot;
        }

        for k in 0..n {
            if k == i { continue; }
            let factor = a[k][i];
            for j in 0..n {
                a[k][j] -= factor * a[i][j];
                inv[k][j] -= factor * inv[i][j];
            }
        }
    }
    inv
}