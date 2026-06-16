// client/monitoring/ensemble.rs
// Ra-Thor Localized Ensemble Kalman Filter Foundation
// Clean dual-path implementation (Standard + Information Form) (v18.37)

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

    /// Standard localized Kalman gain analysis.
    /// Uses K = P_loc H^T (H P_loc H^T + R)^{-1} form (or scalar approximation).
    pub fn analyze(
        &mut self,
        observation: &[f32],
        obs_operator: &[Vec<f32>],
        observation_noise: f32,
        state_distances: Option<&[Vec<f32>]>,
    ) {
        let n = self.state_dim;
        let m = observation.len();
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
            let sparse_loc = build_sparse_state_localization(distances, self.localization_radius);
            apply_sparse_localization(&cov, &sparse_loc)
        } else {
            cov
        };

        // Scalar localized gain (practical foundation)
        let mut gain = vec![0.0; n];
        for i in 0..n {
            let p_ii = localized_cov[i][i];
            gain[i] = p_ii / (p_ii + observation_noise);
        }

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

    /// Matrix Inversion Lemma / Information Form analysis.
    /// K = (P^{-1} + H^T R^{-1} H)^{-1} H^T R^{-1}
    /// Preferred when R is diagonal and we want information-space updates.
    pub fn analyze_information_form(
        &mut self,
        observation: &[f32],
        obs_operator: &[Vec<f32>],
        observation_noise: f32,
        state_distances: Option<&[Vec<f32>]>,
    ) {
        let n = self.state_dim;
        let m = observation.len();
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
            let sparse_loc = build_sparse_state_localization(distances, self.localization_radius);
            apply_sparse_localization(&cov, &sparse_loc)
        } else {
            cov
        };

        // Information form gain (practical diagonal approximation)
        let mut gain = vec![0.0; n];
        for i in 0..n {
            let p_ii = localized_cov[i][i];
            let info = 1.0 / p_ii.max(1e-6);
            let updated_info = info + (1.0 / observation_noise);
            gain[i] = (1.0 / updated_info) / observation_noise;
        }

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