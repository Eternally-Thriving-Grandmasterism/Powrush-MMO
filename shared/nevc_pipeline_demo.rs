// shared/nevc_pipeline_demo.rs
// End-to-End NEVC Pipeline Demonstration
//
// Shows the complete dual-repo flow under the opened Phase 5 contract:
//   ContributionEvent → ContributionLedger → NevcResult → NevcSummary
//
// Suitable as a reference for game systems, dashboards, and future consumers.
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use crate::contribution_events::{apply_event, ContributionEvent};
use crate::contribution_ledger::ContributionLedger;
use crate::nevc_adapter::{ContributionClass, NevcSummary};

/// Run a short demonstration sequence and return the final summaries.
pub fn run_demo() -> Vec<(u64, NevcSummary)> {
    let mut ledger = ContributionLedger::new();

    // Player 1: sustained abundance-aligned RBE actions
    let events_p1 = [
        ContributionEvent::RbeAction {
            player_id: 1,
            abundance_alignment: 1.0,
            waste_or_harm: 0.0,
        },
        ContributionEvent::RbeAction {
            player_id: 1,
            abundance_alignment: 0.95,
            waste_or_harm: 0.01,
        },
        ContributionEvent::RbeAction {
            player_id: 1,
            abundance_alignment: 1.0,
            waste_or_harm: 0.0,
        },
    ];

    // Player 2: high-waste / extractive pattern
    let events_p2 = [
        ContributionEvent::RbeAction {
            player_id: 2,
            abundance_alignment: 0.0,
            waste_or_harm: 2.5,
        },
        ContributionEvent::RbeAction {
            player_id: 2,
            abundance_alignment: 0.1,
            waste_or_harm: 3.0,
        },
    ];

    for e in events_p1.into_iter().chain(events_p2.into_iter()) {
        let _ = apply_event(&mut ledger, e);
    }

    let mut out = Vec::new();
    for pid in [1u64, 2] {
        if let Some(r) = ledger.last_result(pid) {
            out.push((pid, r.summary()));
        }
    }
    out
}

/// Quick classification helper for any player currently in the ledger.
pub fn classify(ledger: &ContributionLedger, player_id: u64) -> ContributionClass {
    ledger.class_of(player_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_separates_contributor_from_zombie() {
        let results = run_demo();
        assert_eq!(results.len(), 2);

        let (p1, s1) = &results[0];
        let (p2, s2) = &results[1];

        assert_eq!(*p1, 1);
        assert_eq!(*p2, 2);
        assert_eq!(s1.class, ContributionClass::ActiveEternalContributor);
        assert_eq!(s2.class, ContributionClass::ZombiePartition);
        assert_eq!(s1.label, "Active Eternal Contributor");
        assert_eq!(s2.label, "Zombie Partition");
    }
}
