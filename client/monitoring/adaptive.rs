// client/monitoring/adaptive.rs
// Ra-Thor Advanced Adaptive + Optimized Localization Radius

use super::localization::SparseLocalization;

/// Computes adaptive localization radius using residual magnitude and ensemble spread.
pub fn compute_advanced_adaptive_radius(
    avg_residual: f32,
    ensemble_spread: f32,
    base_radius: f32,
    min_radius: f32,
    max_radius: f32,
    adaptation_strength: f32,
) -> f32 {
    let residual_factor = (1.0 / (1.0 + avg_residual * 0.8)).clamp(0.4, 1.3);
    let spread_factor = if ensemble_spread < 0.5 { 1.15 } else if ensemble_spread > 3.0 { 0.9 } else { 1.0 };
    let adaptive = base_radius * residual_factor * spread_factor;
    let smoothed = base_radius * (1.0 - adaptation_strength) + adaptive * adaptation_strength;
    smoothed.clamp(min_radius, max_radius)
}

/// Maintains running statistics and dynamically adjusts localization radius (heuristic).
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

        if self.residual_history.len() > self.history_size { self.residual_history.remove(0); }
        if self.spread_history.len() > self.history_size { self.spread_history.remove(0); }

        let avg_residual = if self.residual_history.is_empty() { new_residual } else {
            self.residual_history.iter().sum::<f32>() / self.residual_history.len() as f32
        };
        let avg_spread = if self.spread_history.is_empty() { new_spread } else {
            self.spread_history.iter().sum::<f32>() / self.spread_history.len() as f32
        };

        self.current_radius = compute_advanced_adaptive_radius(
            avg_residual, avg_spread, self.base_radius, self.min_radius, self.max_radius, self.adaptation_strength,
        );
    }

    pub fn get_current_radius(&self) -> f32 { self.current_radius }
}

// ============================================================
// LOCALIZATION RADIUS OPTIMIZER
// More principled optimization based on innovation statistics
// ============================================================

/// Evaluates how well a given localization radius explains recent innovations.
/// Lower cost = better radius.
pub fn evaluate_radius_cost(
    innovations: &[f32],
    ensemble_spread: f32,
    radius: f32,
    observation_noise: f32,
) -> f32 {
    if innovations.is_empty() {
        return 0.0;
    }

    let n = innovations.len() as f32;

    // Observed innovation variance
    let mean_innov: f32 = innovations.iter().sum::<f32>() / n;
    let var_innov: f32 = innovations.iter()
        .map(|&x| (x - mean_innov).powi(2))
        .sum::<f32>() / n;

    // Theoretical innovation variance under current radius
    // We use a simple model: theoretical_var ≈ ensemble_spread^2 + observation_noise
    // modulated by radius (larger radius → more variance allowed)
    let theoretical_var = ensemble_spread.powi(2) + observation_noise * (1.0 + 0.2 * radius);

    // Cost = absolute difference between observed and theoretical variance
    // + small penalty for very small radii (to avoid over-localization)
    let variance_mismatch = (var_innov - theoretical_var).abs();
    let radius_penalty = if radius < 0.5 { 0.5 } else { 0.0 };

    variance_mismatch + radius_penalty
}

/// Optimizes the localization radius using recent innovations.
/// Tries a small number of candidates around the current radius.
#[derive(Clone, Debug)]
pub struct LocalizationRadiusOptimizer {
    pub current_radius: f32,
    min_radius: f32,
    max_radius: f32,
    observation_noise: f32,
    innovation_history: Vec<f32>,
    spread_history: Vec<f32>,
    history_size: usize,
    search_step: f32,
}

impl LocalizationRadiusOptimizer {
    pub fn new(
        initial_radius: f32,
        min_radius: f32,
        max_radius: f32,
        observation_noise: f32,
        history_size: usize,
        search_step: f32,
    ) -> Self {
        Self {
            current_radius: initial_radius,
            min_radius,
            max_radius,
            observation_noise,
            innovation_history: Vec::with_capacity(history_size),
            spread_history: Vec::with_capacity(history_size),
            history_size,
            search_step,
        }
    }

    /// Record a new innovation and ensemble spread
    pub fn record(&mut self, innovation: f32, ensemble_spread: f32) {
        self.innovation_history.push(innovation);
        self.spread_history.push(ensemble_spread);

        if self.innovation_history.len() > self.history_size { self.innovation_history.remove(0); }
        if self.spread_history.len() > self.history_size { self.spread_history.remove(0); }
    }

    /// Optimize the radius using recent data.
    /// Returns the best radius found.
    pub fn optimize(&mut self) -> f32 {
        if self.innovation_history.len() < 5 {
            return self.current_radius; // not enough data
        }

        let current_spread = self.spread_history.last().copied().unwrap_or(1.0);
        let mut best_radius = self.current_radius;
        let mut best_cost = f32::INFINITY;

        // Try a few candidates around current radius
        let candidates = [
            self.current_radius,
            (self.current_radius - self.search_step).max(self.min_radius),
            (self.current_radius + self.search_step).min(self.max_radius),
            (self.current_radius - 2.0 * self.search_step).max(self.min_radius),
            (self.current_radius + 2.0 * self.search_step).min(self.max_radius),
        ];

        for &r in &candidates {
            let cost = evaluate_radius_cost(
                &self.innovation_history,
                current_spread,
                r,
                self.observation_noise,
            );
            if cost < best_cost {
                best_cost = cost;
                best_radius = r;
            }
        }

        self.current_radius = best_radius.clamp(self.min_radius, self.max_radius);
        self.current_radius
    }

    pub fn get_current_radius(&self) -> f32 {
        self.current_radius
    }
}