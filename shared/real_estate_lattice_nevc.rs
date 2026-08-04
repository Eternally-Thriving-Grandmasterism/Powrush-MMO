// shared/real_estate_lattice_nevc.rs
// Phase 11 — Real-Estate Lattice NEVC Attachment (RREL / RESA / TRESA)
//
// Maps stewardship, listing integrity, and abundance-aligned real-estate acts
// onto the NEVC contribution pipeline under the Phase 5 broader-consumer contract.
//
// Consistent with:
//   NEVC_BROADER_CONSUMERS_PHASE5_v1.0.md
//   NET_ETERNAL_VALENCE_CONTRIBUTION_NEVC_CODEX_v1.0.md
//
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use crate::contribution_ledger::ContributionLedger;
use crate::nevc_adapter::{ContributionClass, NevcResult, NevcSample, NevcSummary, sample_from_rbe_action};
use crate::nevc_visibility::{badge_text, status_line};

/// Real-estate lattice action types for NEVC scoring.
#[derive(Clone, Debug)]
pub enum RealEstateStewardshipEvent {
    /// Abundance-aligned stewardship (care, maintenance, community benefit).
    /// alignment: 0.0 ..= 1.0
    Stewardship {
        agent_id: u64,
        alignment: f64,
    },
    /// Honest listing / disclosure integrity (Truth + Order gates).
    /// integrity: 0.0 ..= 1.0
    ListingIntegrity {
        agent_id: u64,
        integrity: f64,
    },
    /// Abundance-aligned transfer (non-extractive conveyance).
    /// alignment: 0.0 ..= 1.0
    AbundanceTransfer {
        agent_id: u64,
        alignment: f64,
    },
    /// Extractive or zero-sum real-estate act (elevated grief).
    Extractive {
        agent_id: u64,
        harm: f64,
    },
}

impl RealEstateStewardshipEvent {
    pub fn agent_id(&self) -> u64 {
        match self {
            Self::Stewardship { agent_id, .. }
            | Self::ListingIntegrity { agent_id, .. }
            | Self::AbundanceTransfer { agent_id, .. }
            | Self::Extractive { agent_id, .. } => *agent_id,
        }
    }

    /// Map to (abundance_alignment, waste_or_harm) for the NEVC pipeline.
    pub fn to_rbe_signals(&self) -> (f64, f64) {
        match self {
            Self::Stewardship { alignment, .. } => (alignment.clamp(0.0, 1.0), 0.0),
            Self::ListingIntegrity { integrity, .. } => {
                let i = integrity.clamp(0.0, 1.0);
                // Low integrity → elevated waste/harm signal
                (i, (1.0 - i) * 2.0)
            }
            Self::AbundanceTransfer { alignment, .. } => (alignment.clamp(0.0, 1.0), 0.0),
            Self::Extractive { harm, .. } => (0.0, harm.max(0.0)),
        }
    }
}

/// Map a real-estate event into a shared contribution ledger.
pub fn apply_real_estate_event(
    ledger: &mut ContributionLedger,
    event: RealEstateStewardshipEvent,
) -> NevcResult {
    let agent_id = event.agent_id();
    let (alignment, waste) = event.to_rbe_signals();
    ledger.record_rbe_action(agent_id, alignment, waste)
}

/// Convenience: build a sample directly (systems without a ledger).
pub fn sample_from_stewardship(alignment: f64, t: u64) -> NevcSample {
    sample_from_rbe_action(alignment.clamp(0.0, 1.0), 0.0, t)
}

pub fn sample_from_event(event: &RealEstateStewardshipEvent, t: u64) -> NevcSample {
    let (alignment, waste) = event.to_rbe_signals();
    sample_from_rbe_action(alignment, waste, t)
}

/// Domain-scoped real-estate NEVC ledger (Phase 11).
/// Thin wrapper over ContributionLedger for RREL pathways.
#[derive(Clone, Debug, Default)]
pub struct RealEstateNevcLedger {
    inner: ContributionLedger,
}

impl RealEstateNevcLedger {
    pub fn new() -> Self {
        Self {
            inner: ContributionLedger::new(),
        }
    }

    pub fn apply(&mut self, event: RealEstateStewardshipEvent) -> NevcResult {
        apply_real_estate_event(&mut self.inner, event)
    }

    pub fn class_of(&self, agent_id: u64) -> ContributionClass {
        self.inner.class_of(agent_id)
    }

    pub fn is_contributor(&self, agent_id: u64) -> bool {
        self.inner.is_contributor(agent_id)
    }

    pub fn summary_of(&self, agent_id: u64) -> Option<NevcSummary> {
        self.inner.last_result(agent_id).map(|r| r.summary())
    }

    pub fn status_line_of(&self, agent_id: u64) -> Option<String> {
        self.summary_of(agent_id).map(|s| status_line(&s))
    }

    pub fn badge_of(&self, agent_id: u64) -> &'static str {
        badge_text(self.class_of(agent_id))
    }

    pub fn inner(&self) -> &ContributionLedger {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut ContributionLedger {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stewardship_makes_contributor() {
        let mut ledger = RealEstateNevcLedger::new();
        let r = ledger.apply(RealEstateStewardshipEvent::Stewardship {
            agent_id: 100,
            alignment: 1.0,
        });
        assert_eq!(r.class, ContributionClass::ActiveEternalContributor);
        assert!(ledger.is_contributor(100));
        assert_eq!(ledger.badge_of(100), "Contributor");
    }

    #[test]
    fn high_integrity_listing_contributor() {
        let mut ledger = RealEstateNevcLedger::new();
        let r = ledger.apply(RealEstateStewardshipEvent::ListingIntegrity {
            agent_id: 101,
            integrity: 1.0,
        });
        assert!(r.is_contributor());
    }

    #[test]
    fn low_integrity_listing_not_contributor() {
        let mut ledger = RealEstateNevcLedger::new();
        let r = ledger.apply(RealEstateStewardshipEvent::ListingIntegrity {
            agent_id: 102,
            integrity: 0.0,
        });
        assert!(!r.is_contributor());
    }

    #[test]
    fn abundance_transfer_contributor() {
        let mut ledger = RealEstateNevcLedger::new();
        let r = ledger.apply(RealEstateStewardshipEvent::AbundanceTransfer {
            agent_id: 103,
            alignment: 0.95,
        });
        assert!(r.is_contributor());
    }

    #[test]
    fn extractive_stays_zombie() {
        let mut ledger = RealEstateNevcLedger::new();
        let r = ledger.apply(RealEstateStewardshipEvent::Extractive {
            agent_id: 104,
            harm: 2.0,
        });
        assert_eq!(r.class, ContributionClass::ZombiePartition);
        assert_eq!(ledger.badge_of(104), "Zombie");
    }
}
