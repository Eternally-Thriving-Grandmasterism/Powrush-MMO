// shared/nevc_adapter.rs
// Phase 1 Thin NEVC Adapter — Powrush-MMO dual-repo consumer surface
//
// Mirrors the essential types from Ra-Thor `mercy_tolc_operator_algebra::nevc`
// so that game systems can emit samples and consume scores without a hard
// compile-time dependency on the full Ra-Thor monorepo.
//
// Authoritative definition remains in:
//   https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor
//     NET_ETERNAL_VALENCE_CONTRIBUTION_NEVC_CODEX_v1.0.md
//     crates/mercy_tolc_operator_algebra/src/nevc.rs
//
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use serde::{Deserialize, Serialize};

/// Binary partition defined by the NEVC Codex.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContributionClass {
    /// NEVC > 0 — raises net value to all life under infinite forward time.
    ActiveEternalContributor,
    /// NEVC ≤ 0 — mindless mental waste / entropy increase (zombie partition).
    ZombiePartition,
}

impl ContributionClass {
    pub fn from_score(score: f64) -> Self {
        if score > 0.0 {
            ContributionClass::ActiveEternalContributor
        } else {
            ContributionClass::ZombiePartition
        }
    }

    pub fn is_contributor(self) -> bool {
        matches!(self, ContributionClass::ActiveEternalContributor)
    }
}

/// A single timed sample of an agent’s effect on the valence field.
/// Maps 1:1 onto Ra-Thor `NevcSample`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NevcSample {
    /// Instantaneous valence after the action / state (ideally ≥ 0.999999).
    pub valence: f64,
    /// Grief / orthogonal load induced.
    pub grief_load: f64,
    /// Optional per-gate mercy vector components (length ≤ 8).
    pub mercy_components: Vec<f64>,
    /// Discrete time index (monotonic non-decreasing).
    pub t: u64,
}

impl NevcSample {
    pub fn new(valence: f64, grief_load: f64, t: u64) -> Self {
        Self {
            valence: valence.clamp(0.0, 1.0),
            grief_load: grief_load.max(0.0),
            mercy_components: Vec::new(),
            t,
        }
    }

    pub fn with_mercy(mut self, components: Vec<f64>) -> Self {
        self.mercy_components = components;
        self
    }
}

/// Result of an NEVC evaluation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NevcResult {
    pub score: f64,
    pub class: ContributionClass,
    pub sample_count: usize,
    pub mean_valence: f64,
    pub total_grief: f64,
}

impl NevcResult {
    pub fn is_contributor(&self) -> bool {
        self.class.is_contributor()
    }
}

/// Configuration for the discrete NEVC integrator (mirrors Ra-Thor defaults).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NevcConfig {
    pub positive_weight: f64,
    pub grief_penalty: f64,
    pub horizon_emphasis: f64,
    pub valence_floor: f64,
}

impl Default for NevcConfig {
    fn default() -> Self {
        Self {
            positive_weight: 1.0,
            grief_penalty: 1.0,
            horizon_emphasis: 1.0,
            valence_floor: 0.999999,
        }
    }
}

/// Local discrete approximation (identical algorithm to Ra-Thor `compute_nevc`).
/// When a published Ra-Thor interface becomes available this function can
/// become a thin forwarder; until then it is the self-contained Phase-1 surface.
pub fn compute_nevc(samples: &[NevcSample], config: &NevcConfig) -> NevcResult {
    if samples.is_empty() {
        return NevcResult {
            score: 0.0,
            class: ContributionClass::ZombiePartition,
            sample_count: 0,
            mean_valence: 0.0,
            total_grief: 0.0,
        };
    }

    let n = samples.len() as f64;
    let mut score = 0.0;
    let mut sum_v = 0.0;
    let mut total_grief = 0.0;

    let t_max = samples.iter().map(|s| s.t).max().unwrap_or(1).max(1) as f64;

    for s in samples {
        let v = s.valence;
        sum_v += v;
        total_grief += s.grief_load;

        let positive = if v >= config.valence_floor {
            let proximity = (v - config.valence_floor) / (1.0 - config.valence_floor).max(1e-12);
            config.positive_weight * proximity
        } else {
            0.0
        };

        let t_norm = (s.t as f64) / t_max;
        let w = 1.0 + config.horizon_emphasis * t_norm;

        let mercy_bonus = if s.mercy_components.is_empty() {
            1.0
        } else {
            let mean_m: f64 = s.mercy_components.iter().sum::<f64>()
                / (s.mercy_components.len().max(1) as f64);
            (0.5 + 0.5 * mean_m.clamp(0.0, 1.0)).clamp(0.5, 1.5)
        };

        let term = w * mercy_bonus * (positive - config.grief_penalty * s.grief_load);
        score += term;
    }

    score /= n;
    let mean_valence = sum_v / n;

    NevcResult {
        score,
        class: ContributionClass::from_score(score),
        sample_count: samples.len(),
        mean_valence,
        total_grief,
    }
}

/// Convenience single-state evaluation.
pub fn score_instant(valence: f64, grief_load: f64) -> NevcResult {
    let sample = NevcSample::new(valence, grief_load, 0);
    compute_nevc(&[sample], &NevcConfig::default())
}

/// Helper: map a typical RBE contribution event into a sample.
/// Positive contribution → high valence, near-zero grief.
/// Extractive / wasteful act → lower valence, elevated grief.
pub fn sample_from_rbe_action(
    abundance_alignment: f64, // 0.0 ..= 1.0
    waste_or_harm: f64,       // ≥ 0.0
    t: u64,
) -> NevcSample {
    let valence = (0.999999 + 0.000001 * abundance_alignment.clamp(0.0, 1.0)).min(1.0);
    NevcSample::new(valence, waste_or_harm.max(0.0), t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn high_valence_low_grief_is_contributor() {
        let samples = vec![
            NevcSample::new(0.999999, 0.0, 0),
            NevcSample::new(0.9999995, 0.001, 1),
        ];
        let r = compute_nevc(&samples, &NevcConfig::default());
        assert!(r.is_contributor(), "score={}", r.score);
    }

    #[test]
    fn zero_valence_high_grief_is_zombie() {
        let samples = vec![NevcSample::new(0.0, 2.5, 0)];
        let r = compute_nevc(&samples, &NevcConfig::default());
        assert!(!r.is_contributor());
        assert_eq!(r.class, ContributionClass::ZombiePartition);
    }

    #[test]
    fn empty_is_zombie() {
        let r = compute_nevc(&[], &NevcConfig::default());
        assert_eq!(r.class, ContributionClass::ZombiePartition);
    }

    #[test]
    fn rbe_helper_produces_reasonable_sample() {
        let s = sample_from_rbe_action(1.0, 0.0, 42);
        assert!(s.valence >= 0.999999);
        assert_eq!(s.grief_load, 0.0);
        assert_eq!(s.t, 42);
    }
}
