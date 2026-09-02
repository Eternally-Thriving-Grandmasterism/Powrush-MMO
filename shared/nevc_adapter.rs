// shared/nevc_adapter.rs
// Phase 1 Thin NEVC Adapter + Phase 4 visibility + Finish Pass D recovery parity
//
// Mirrors Ra-Thor `mercy_tolc_operator_algebra::nevc`.
// Authoritative: Ra-Thor NEVC Codex + crates/mercy_tolc_operator_algebra/src/nevc.rs
//
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use serde::{Deserialize, Serialize};

/// Binary partition defined by the NEVC Codex.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContributionClass {
    ActiveEternalContributor,
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

/// Finish Pass D — Compassion-gate recovery (Codex §6).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompassionRecoveryState {
    Open,
    Sealed,
}

impl Default for CompassionRecoveryState {
    fn default() -> Self {
        CompassionRecoveryState::Open
    }
}

impl CompassionRecoveryState {
    pub fn is_open(self) -> bool {
        matches!(self, CompassionRecoveryState::Open)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NevcSample {
    pub valence: f64,
    pub grief_load: f64,
    pub mercy_components: Vec<f64>,
    pub t: u64,
    #[serde(default)]
    pub transient: bool,
}

impl NevcSample {
    pub fn new(valence: f64, grief_load: f64, t: u64) -> Self {
        Self {
            valence: valence.clamp(0.0, 1.0),
            grief_load: grief_load.max(0.0),
            mercy_components: Vec::new(),
            t,
            transient: false,
        }
    }

    pub fn with_mercy(mut self, components: Vec<f64>) -> Self {
        self.mercy_components = components;
        self
    }

    pub fn transient(mut self) -> Self {
        self.transient = true;
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NevcResult {
    pub score: f64,
    pub class: ContributionClass,
    pub sample_count: usize,
    pub mean_valence: f64,
    pub total_grief: f64,
    pub recovery: CompassionRecoveryState,
}

impl NevcResult {
    pub fn is_contributor(&self) -> bool {
        self.class.is_contributor()
    }

    pub fn recovery_open(&self) -> bool {
        self.recovery.is_open()
    }

    pub fn summary(&self) -> NevcSummary {
        NevcSummary::from(self)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NevcSummary {
    pub class: ContributionClass,
    pub score: f64,
    pub sample_count: usize,
    pub mean_valence: f64,
    pub total_grief: f64,
    pub label: &'static str,
    pub recovery: CompassionRecoveryState,
}

impl From<&NevcResult> for NevcSummary {
    fn from(r: &NevcResult) -> Self {
        let label = match r.class {
            ContributionClass::ActiveEternalContributor => "Active Eternal Contributor",
            ContributionClass::ZombiePartition => "Zombie Partition",
        };
        Self {
            class: r.class,
            score: r.score,
            sample_count: r.sample_count,
            mean_valence: r.mean_valence,
            total_grief: r.total_grief,
            label,
            recovery: r.recovery,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NevcConfig {
    pub positive_weight: f64,
    pub grief_penalty: f64,
    pub horizon_emphasis: f64,
    pub valence_floor: f64,
    pub respect_transient: bool,
}

impl Default for NevcConfig {
    fn default() -> Self {
        Self {
            positive_weight: 1.0,
            grief_penalty: 1.0,
            horizon_emphasis: 1.0,
            valence_floor: 0.999999,
            respect_transient: true,
        }
    }
}

impl NevcConfig {
    pub fn neutral() -> Self {
        Self {
            horizon_emphasis: 0.0,
            ..Default::default()
        }
    }

    pub fn forward_emphasis() -> Self {
        Self::default()
    }

    pub fn eternal_tilt() -> Self {
        Self {
            horizon_emphasis: 1.5,
            ..Default::default()
        }
    }
}

pub fn compute_nevc(samples: &[NevcSample], config: &NevcConfig) -> NevcResult {
    if samples.is_empty() {
        return NevcResult {
            score: 0.0,
            class: ContributionClass::ZombiePartition,
            sample_count: 0,
            mean_valence: 0.0,
            total_grief: 0.0,
            recovery: CompassionRecoveryState::Open,
        };
    }

    let n = samples.len() as f64;
    let mut score = 0.0;
    let mut sum_v = 0.0;
    let mut total_grief = 0.0;
    let mut any_transient = false;

    let t_max = samples.iter().map(|s| s.t).max().unwrap_or(1).max(1) as f64;

    for s in samples {
        any_transient |= s.transient;
        let v = s.valence;
        sum_v += v;
        total_grief += s.grief_load;

        // Inclusive HIGH floor: valence at the floor is already contributor-grade.
        // Do not scale proximity from 0 at the floor (that made HIGH+0 grief a zombie).
        let positive = if v >= config.valence_floor {
            config.positive_weight
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
    let class = ContributionClass::from_score(score);

    let recovery = if class.is_contributor() {
        CompassionRecoveryState::Open
    } else if config.respect_transient && any_transient {
        CompassionRecoveryState::Open
    } else if samples.len() >= 3 && score < -0.5 {
        CompassionRecoveryState::Sealed
    } else {
        CompassionRecoveryState::Open
    };

    NevcResult {
        score,
        class,
        sample_count: samples.len(),
        mean_valence,
        total_grief,
        recovery,
    }
}

pub fn score_instant(valence: f64, grief_load: f64) -> NevcResult {
    let sample = NevcSample::new(valence, grief_load, 0);
    compute_nevc(&[sample], &NevcConfig::default())
}

pub fn sample_from_rbe_action(
    abundance_alignment: f64,
    waste_or_harm: f64,
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
        assert!(r.recovery_open());
    }

    #[test]
    fn zero_valence_high_grief_is_zombie() {
        let samples = vec![NevcSample::new(0.0, 2.5, 0)];
        let r = compute_nevc(&samples, &NevcConfig::default());
        assert!(!r.is_contributor());
        assert_eq!(r.class, ContributionClass::ZombiePartition);
    }

    #[test]
    fn empty_is_zombie_open_recovery() {
        let r = compute_nevc(&[], &NevcConfig::default());
        assert_eq!(r.class, ContributionClass::ZombiePartition);
        assert!(r.recovery_open());
    }

    #[test]
    fn transient_keeps_recovery_open() {
        let samples = vec![
            NevcSample::new(0.0, 2.0, 0).transient(),
            NevcSample::new(0.0, 2.0, 1).transient(),
            NevcSample::new(0.0, 2.0, 2).transient(),
        ];
        let r = compute_nevc(&samples, &NevcConfig::default());
        assert!(!r.is_contributor());
        assert!(r.recovery_open());
    }

    #[test]
    fn rbe_helper_produces_reasonable_sample() {
        let s = sample_from_rbe_action(1.0, 0.0, 42);
        assert!(s.valence >= 0.999999);
        assert_eq!(s.grief_load, 0.0);
        assert_eq!(s.t, 42);
    }

    #[test]
    fn summary_includes_recovery() {
        let r = score_instant(0.999999, 0.0);
        let s = r.summary();
        assert_eq!(s.label, "Active Eternal Contributor");
        assert_eq!(s.recovery, CompassionRecoveryState::Open);
    }
}
