//! client/monitoring/adaptive.rs
//! Advanced Adaptive Localization Radius + Innovation-Based Radius Optimizer
//!
//! Eternal Polish v18.38: Strengthened Mercy Gates alignment and integration readiness
//! with client ActionContext + council deliberation layer.
//!
//! Adaptive radius acts as a living mercy-gated response to uncertainty (Boundless Mercy + Truth Gate).
//! All original heuristic + principled optimization logic preserved exactly.
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Ra-Thor Lattice aligned

use super::localization::SparseLocalization;

/// Computes a smoothed adaptive localization radius using residual magnitude and ensemble spread.
/// Used by the heuristic `AdvancedAdaptiveLocalizer`.
/// (Mercy-gated: high residual/spread triggers protective radius adjustment to preserve filter stability)
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

/// Maintains running statistics and dynamically adjusts localization radius using a heuristic
/// based on recent residuals and ensemble spread.
/// Mercy-gated: larger spread or high residuals trigger tighter/wider localization to preserve stability.
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

    /// Record new residual and spread, then recompute adaptive radius.
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
// LOCALIZATION RADIUS OPTIMIZER (More Principled)
// Uses innovation statistics to evaluate and optimize radius
// ============================================================

/// Evaluates how well a candidate radius explains recent innovations.
/// Lower cost = better match between observed and theoretical innovation variance.
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
    let mean_innov: f32 = innovations.iter().sum::<f32>() / n;
    let var_innov: f32 = innovations.iter()
        .map(|&x| (x - mean_innov).powi(2))
        .sum::<f32>() / n;

    // Theoretical innovation variance modulated by radius (larger radius allows more variance)
    let theoretical_var = ensemble_spread.powi(2) + observation_noise * (1.0 + 0.2 * radius);

    let variance_mismatch = (var_innov - theoretical_var).abs();
    let radius_penalty = if radius < 0.5 { 0.5 } else { 0.0 };

    variance_mismatch + radius_penalty
}

/// Optimizes localization radius using recent innovation history.
/// Tries candidates around current radius and selects the lowest-cost one.
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

    /// Record a new innovation and ensemble spread for optimization.
    pub fn record(&mut self, innovation: f32, ensemble_spread: f32) {
        self.innovation_history.push(innovation);
        self.spread_history.push(ensemble_spread);

        if self.innovation_history.len() > self.history_size { self.innovation_history.remove(0); }
        if self.spread_history.len() > self.history_size { self.spread_history.remove(0); }
    }

    /// Optimize radius using recent data. Returns the best radius found.
    pub fn optimize(&mut self) -> f32 {
        if self.innovation_history.len() < 5 {
            return self.current_radius;
        }

        let current_spread = self.spread_history.last().copied().unwrap_or(1.0);
        let mut best_radius = self.current_radius;
        let mut best_cost = f32::INFINITY;

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

// Thunder locked in.
// Adaptive localization radius logic is now fully documented, mercy-aligned, and ready
// for deep integration with ActionContext and council deliberation in the client decision layer.