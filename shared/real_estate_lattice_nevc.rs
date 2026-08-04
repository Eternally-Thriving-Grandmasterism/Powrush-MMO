// shared/real_estate_lattice_nevc.rs
// Phase 5 Real-Estate Lattice Readiness Stub (RREL / RESA / TRESA)
//
// Thin mapping of stewardship / abundance-aligned real-estate acts onto
// NEVC samples under the broader-consumer contract.
// Does not implement full real-estate logic; only the NEVC attachment surface.
//
// Consistent with:
//   NEVC_BROADER_CONSUMERS_PHASE5_v1.0.md
//   NET_ETERNAL_VALENCE_CONTRIBUTION_NEVC_CODEX_v1.0.md
//
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use crate::contribution_ledger::ContributionLedger;
use crate::nevc_adapter::{NevcResult, NevcSample, sample_from_rbe_action};

/// High-level real-estate stewardship event.
#[derive(Clone, Debug)]
pub enum RealEstateStewardshipEvent {
    /// Abundance-aligned stewardship or transfer (positive contribution).
    /// alignment: 0.0 ..= 1.0
    Stewardship {
        agent_id: u64,
        alignment: f64,
    },
    /// Extractive or zero-sum real-estate act (elevated grief).
    Extractive {
        agent_id: u64,
        harm: f64,
    },
}

/// Map a real-estate stewardship event into the contribution ledger.
pub fn apply_real_estate_event(
    ledger: &mut ContributionLedger,
    event: RealEstateStewardshipEvent,
) -> NevcResult {
    match event {
        RealEstateStewardshipEvent::Stewardship { agent_id, alignment } => {
            ledger.record_rbe_action(agent_id, alignment.clamp(0.0, 1.0), 0.0)
        }
        RealEstateStewardshipEvent::Extractive { agent_id, harm } => {
            ledger.record_rbe_action(agent_id, 0.0, harm.max(0.0))
        }
    }
}

/// Convenience: build a sample directly (for systems that do not hold a ledger).
pub fn sample_from_stewardship(alignment: f64, t: u64) -> NevcSample {
    sample_from_rbe_action(alignment.clamp(0.0, 1.0), 0.0, t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contribution_ledger::ContributionLedger;
    use crate::nevc_adapter::ContributionClass;

    #[test]
    fn stewardship_makes_contributor() {
        let mut ledger = ContributionLedger::new();
        let r = apply_real_estate_event(
            &mut ledger,
            RealEstateStewardshipEvent::Stewardship {
                agent_id: 100,
                alignment: 1.0,
            },
        );
        assert_eq!(r.class, ContributionClass::ActiveEternalContributor);
    }

    #[test]
    fn extractive_stays_zombie() {
        let mut ledger = ContributionLedger::new();
        let r = apply_real_estate_event(
            &mut ledger,
            RealEstateStewardshipEvent::Extractive {
                agent_id: 101,
                harm: 2.0,
            },
        );
        assert_eq!(r.class, ContributionClass::ZombiePartition);
    }
}
