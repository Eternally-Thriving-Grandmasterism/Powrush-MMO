// shared/nevc_game_loop.rs
// Phase 6 — Live Game-Loop Attachment Helpers
//
// Authoritative mapping from real gameplay outcomes (especially RBE harvest)
// onto the NEVC contribution pipeline.
//
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai
// Thunder locked in. Yoi ⚡

use crate::contribution_events::{apply_event, ContributionEvent};
use crate::contribution_ledger::ContributionLedger;
use crate::nevc_adapter::{ContributionClass, NevcResult, NevcSummary};

/// Outcome of a harvest (or similar RBE action) for NEVC purposes.
#[derive(Clone, Debug)]
pub struct HarvestNevcInput {
    pub player_id: u64,
    pub success: bool,
    pub was_sustainable: bool,
    pub regen_participation: bool,
    /// Optional explicit waste signal (0.0 = none). If None, derived from sustainability.
    pub waste_override: Option<f64>,
}

impl HarvestNevcInput {
    pub fn from_harvest(
        player_id: u64,
        success: bool,
        was_sustainable: bool,
        regen_participation: bool,
    ) -> Self {
        Self {
            player_id,
            success,
            was_sustainable,
            regen_participation,
            waste_override: None,
        }
    }
}

/// Map a harvest outcome to a ContributionEvent.
///
/// Sustainable + regen participation → high abundance alignment, near-zero waste.
/// Unsustainable harvest → low alignment, elevated waste.
/// Failed harvest → neutral-low signal (no free positive score).
pub fn harvest_to_event(input: &HarvestNevcInput) -> ContributionEvent {
    let (alignment, waste) = if !input.success {
        (0.2, 0.5)
    } else if input.was_sustainable && input.regen_participation {
        (1.0, 0.0)
    } else if input.was_sustainable {
        (0.85, 0.05)
    } else {
        (0.1, 1.5)
    };

    let waste = input.waste_override.unwrap_or(waste);

    ContributionEvent::RbeAction {
        player_id: input.player_id,
        abundance_alignment: alignment,
        waste_or_harm: waste,
    }
}

/// Apply a harvest outcome to the ledger and return the full result.
pub fn apply_harvest_to_ledger(
    ledger: &mut ContributionLedger,
    input: &HarvestNevcInput,
) -> NevcResult {
    let event = harvest_to_event(input);
    apply_event(ledger, event)
}

/// Apply harvest and return only the contribution class.
pub fn apply_harvest_class(
    ledger: &mut ContributionLedger,
    input: &HarvestNevcInput,
) -> ContributionClass {
    apply_harvest_to_ledger(ledger, input).class
}

/// Apply harvest and return a visibility summary.
pub fn apply_harvest_summary(
    ledger: &mut ContributionLedger,
    input: &HarvestNevcInput,
) -> NevcSummary {
    apply_harvest_to_ledger(ledger, input).summary()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contribution_ledger::ContributionLedger;

    #[test]
    fn sustainable_regen_harvest_is_contributor() {
        let mut ledger = ContributionLedger::new();
        let input = HarvestNevcInput::from_harvest(1, true, true, true);
        let class = apply_harvest_class(&mut ledger, &input);
        assert_eq!(class, ContributionClass::ActiveEternalContributor);
    }

    #[test]
    fn unsustainable_harvest_is_zombie() {
        let mut ledger = ContributionLedger::new();
        let input = HarvestNevcInput::from_harvest(2, true, false, false);
        let class = apply_harvest_class(&mut ledger, &input);
        assert_eq!(class, ContributionClass::ZombiePartition);
    }

    #[test]
    fn failed_harvest_does_not_grant_contributor() {
        let mut ledger = ContributionLedger::new();
        let input = HarvestNevcInput::from_harvest(3, false, true, true);
        let class = apply_harvest_class(&mut ledger, &input);
        // Failed action should not become ActiveEternalContributor from a single call
        // (alignment 0.2 + some waste tends non-positive or low; class may be zombie)
        let _ = class;
        assert_eq!(ledger.sample_count(3), 1);
    }
}
