// client/monitoring/ensemble.rs
// Ra-Thor Localized Ensemble Kalman Filter Foundation

use super::localization::{build_sparse_state_localization, apply_sparse_localization, SparseLocalization};
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

    pub fn analyze(
        &mut self,
        observation: &[f32],
        _obs_operator: &[Vec<f32>],
        observation_noise: f32,
        state_distances: Option<&[Vec<f32>]>,
    ) {
        let mean = Self::compute_mean(&self.ensemble);

        let mut cov = vec![vec![0.0; self.state_dim]; self.state_dim];
        for i in 0..self.state_dim {
            for j in 0..self.state_dim {
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

        // Simplified gain computation for foundation version
        let mut gain = vec![0.0; self.state_dim];
        for i in 0..self.state_dim {
            gain[i] = localized_cov[i][i] / (localized_cov[i][i] + observation_noise).max(1e-6);
        }

        for member in &mut self.ensemble {
            for i in 0..self.state_dim {
                let mut innovation = 0.0;
                for (k, &obs_val) in observation.iter().enumerate() {
                    if k < gain.len() {
                        innovation += gain[i] * (obs_val - member[i]);
                    }
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