// shared/nevc_bridge.rs
// Phase 8 — Dual-Repo Published Interface Bridge
//
// Mode A (feature = "nevc_rathor"): forward to Ra-Thor mercy_tolc_operator_algebra::nevc
// Mode B (default): use local nevc_adapter (algorithm-identical sovereign fallback)
//
// Contract: Ra-Thor NEVC_DUAL_REPO_INTERFACE_v1.0.md
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use crate::nevc_adapter::{self, ContributionClass, NevcConfig, NevcResult, NevcSample, NevcSummary};

/// Unified entry point: compute NEVC under the active dual-repo mode.
pub fn compute_nevc_bridged(samples: &[NevcSample], config: &NevcConfig) -> NevcResult {
    #[cfg(feature = "nevc_rathor")]
    {
        // Mode A — Ra-Thor path dependency
        use mercy_tolc_operator_algebra::nevc as rathor;
        let converted: Vec<rathor::NevcSample> = samples
            .iter()
            .map(|s| {
                let mut sample = rathor::NevcSample::new(
                    mercy_tolc_operator_algebra::Valence::new(s.valence),
                    s.grief_load,
                    s.t,
                );
                if !s.mercy_components.is_empty() {
                    sample = sample.with_mercy(s.mercy_components.clone());
                }
                sample
            })
            .collect();
        let cfg = rathor::NevcConfig {
            positive_weight: config.positive_weight,
            grief_penalty: config.grief_penalty,
            horizon_emphasis: config.horizon_emphasis,
            valence_floor: config.valence_floor,
            ..Default::default()
        };
        let r = rathor::compute_nevc(&converted, &cfg);
        return NevcResult {
            score: r.score,
            class: if r.class.is_contributor() {
                ContributionClass::ActiveEternalContributor
            } else {
                ContributionClass::ZombiePartition
            },
            sample_count: r.sample_count,
            mean_valence: r.mean_valence,
            total_grief: r.total_grief,
        };
    }

    #[cfg(not(feature = "nevc_rathor"))]
    {
        // Mode B — local sovereign adapter
        nevc_adapter::compute_nevc(samples, config)
    }
}

/// Instant score under the active mode.
pub fn score_instant_bridged(valence: f64, grief_load: f64) -> NevcResult {
    let sample = NevcSample::new(valence, grief_load, 0);
    compute_nevc_bridged(&[sample], &NevcConfig::default())
}

/// Visibility summary under the active mode.
pub fn summary_bridged(samples: &[NevcSample], config: &NevcConfig) -> NevcSummary {
    compute_nevc_bridged(samples, config).summary()
}

/// Which dual-repo mode is active at compile time.
pub fn active_mode() -> &'static str {
    #[cfg(feature = "nevc_rathor")]
    {
        "Mode A — Ra-Thor path dependency (nevc_rathor)"
    }
    #[cfg(not(feature = "nevc_rathor"))]
    {
        "Mode B — local nevc_adapter (sovereign fallback)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridged_high_valence_is_contributor() {
        let samples = vec![NevcSample::new(0.999999, 0.0, 0)];
        let r = compute_nevc_bridged(&samples, &NevcConfig::default());
        assert!(r.is_contributor());
    }

    #[test]
    fn bridged_zero_is_zombie() {
        let samples = vec![NevcSample::new(0.0, 2.0, 0)];
        let r = compute_nevc_bridged(&samples, &NevcConfig::default());
        assert!(!r.is_contributor());
    }

    #[test]
    fn mode_string_is_nonempty() {
        assert!(!active_mode().is_empty());
    }
}
