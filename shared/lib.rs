// shared/lib.rs
// Powrush-MMO — Shared Crate Root
// Phase 0–11 NEVC surfaces (including real-estate lattice attachment).
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai

pub mod protocol;
pub mod nevc_adapter;
pub mod contribution_ledger;
pub mod contribution_events;
pub mod nevc_pipeline_demo;
pub mod real_estate_lattice_nevc;
pub mod nevc_game_loop;
pub mod nevc_persistence;
pub mod nevc_bridge;
pub mod nevc_visibility;

#[cfg(feature = "full_rbe")]
#[path = "rbe_queries.rs"]
pub mod rbe_queries;

#[cfg(not(feature = "full_rbe"))]
pub mod rbe_queries {
    pub fn stub_note() -> &'static str {
        "RBE deep queries available via Ra-Thor monorepo link. Thunder locked in."
    }
}

pub mod prelude {
    pub use crate::protocol::HotbarSlot;
    pub use crate::rbe_queries;
    pub use crate::nevc_adapter::{ContributionClass, NevcSample, NevcResult, NevcConfig, NevcSummary, compute_nevc, score_instant, sample_from_rbe_action};
    pub use crate::contribution_ledger::{ContributionLedger, PlayerContribution};
    pub use crate::contribution_events::{ContributionEvent, apply_event, apply_event_class};
    pub use crate::nevc_pipeline_demo::{run_demo, classify};
    pub use crate::real_estate_lattice_nevc::{RealEstateStewardshipEvent, RealEstateNevcLedger, apply_real_estate_event, sample_from_stewardship, sample_from_event};
    pub use crate::nevc_game_loop::{HarvestNevcInput, harvest_to_event, apply_harvest_to_ledger, apply_harvest_class, apply_harvest_summary};
    pub use crate::nevc_persistence::{NevcPlayerRecord, NevcPersistenceStore};
    pub use crate::nevc_bridge::{compute_nevc_bridged, score_instant_bridged, summary_bridged, active_mode};
    pub use crate::nevc_visibility::{HorizonPreset, status_line, badge_text, summary_from_result, panel_fields};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visibility_helpers_reachable() {
        let r = nevc_adapter::score_instant(0.999999, 0.0);
        let s = nevc_visibility::summary_from_result(&r);
        assert_eq!(nevc_visibility::badge_text(s.class), "Contributor");
    }

    #[test]
    fn real_estate_ledger_reachable() {
        let mut rrel = real_estate_lattice_nevc::RealEstateNevcLedger::new();
        let r = rrel.apply(real_estate_lattice_nevc::RealEstateStewardshipEvent::Stewardship {
            agent_id: 1,
            alignment: 1.0,
        });
        assert!(r.is_contributor());
    }
}
