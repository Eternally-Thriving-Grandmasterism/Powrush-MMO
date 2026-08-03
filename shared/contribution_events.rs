// shared/contribution_events.rs
// Phase 2b Event Attachment Surface
//
// Provides a clean event type and helper that routes gameplay / RBE actions
// into the ContributionLedger. This is the recommended attachment point for
// simulation and game systems.
//
// Consistent with NEVC_POWRUSH_INTEGRATION_CODEX_v1.0.md
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use serde::{Deserialize, Serialize};

use crate::contribution_ledger::ContributionLedger;
use crate::nevc_adapter::{ContributionClass, NevcResult};

/// High-level contribution event that game / simulation systems can emit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ContributionEvent {
    /// Player performed an abundance-aligned RBE action.
    /// abundance_alignment: 0.0 ..= 1.0
    /// waste_or_harm: ≥ 0.0
    RbeAction {
        player_id: u64,
        abundance_alignment: f64,
        waste_or_harm: f64,
    },

    /// Generic sample injection (advanced / custom systems).
    RawSample {
        player_id: u64,
        valence: f64,
        grief_load: f64,
    },
}

/// Apply a contribution event to the ledger and return the resulting score.
pub fn apply_event(ledger: &mut ContributionLedger, event: ContributionEvent) -> NevcResult {
    match event {
        ContributionEvent::RbeAction {
            player_id,
            abundance_alignment,
            waste_or_harm,
        } => ledger.record_rbe_action(player_id, abundance_alignment, waste_or_harm),

        ContributionEvent::RawSample {
            player_id,
            valence,
            grief_load,
        } => {
            use crate::nevc_adapter::NevcSample;
            let t = ledger.sample_count(player_id) as u64;
            let sample = NevcSample::new(valence, grief_load, t);
            ledger.record_sample(player_id, sample)
        }
    }
}

/// Convenience: apply event and return only the resulting class.
pub fn apply_event_class(
    ledger: &mut ContributionLedger,
    event: ContributionEvent,
) -> ContributionClass {
    apply_event(ledger, event).class
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contribution_ledger::ContributionLedger;

    #[test]
    fn rbe_event_makes_contributor() {
        let mut ledger = ContributionLedger::new();
        let event = ContributionEvent::RbeAction {
            player_id: 10,
            abundance_alignment: 1.0,
            waste_or_harm: 0.0,
        };
        let class = apply_event_class(&mut ledger, event);
        assert_eq!(class, ContributionClass::ActiveEternalContributor);
        assert!(ledger.is_contributor(10));
    }

    #[test]
    fn wasteful_event_stays_zombie() {
        let mut ledger = ContributionLedger::new();
        let event = ContributionEvent::RbeAction {
            player_id: 11,
            abundance_alignment: 0.0,
            waste_or_harm: 4.0,
        };
        let class = apply_event_class(&mut ledger, event);
        assert_eq!(class, ContributionClass::ZombiePartition);
    }

    #[test]
    fn raw_sample_path_works() {
        let mut ledger = ContributionLedger::new();
        let event = ContributionEvent::RawSample {
            player_id: 12,
            valence: 0.999999,
            grief_load: 0.0,
        };
        let r = apply_event(&mut ledger, event);
        assert!(r.is_contributor());
    }
}
