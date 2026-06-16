// client/monitoring/adaptive.rs
// Ra-Thor Advanced Adaptive Localization Radius

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

/// Maintains running statistics and dynamically adjusts localization radius.
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