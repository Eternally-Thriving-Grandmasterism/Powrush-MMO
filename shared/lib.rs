// shared/lib.rs
// Powrush-MMO — Shared Crate Root
// Phase 0–8 NEVC: adapter, ledger, events, demo, real-estate, game-loop,
// persistence, dual-repo bridge.
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
    pub use crate::protocol::{ClientMessage, ServerMessage, TradeOffer, Vec3Ser, HealthComponent};
    pub use crate::rbe_queries;
    pub use crate::nevc_adapter::{ContributionClass, NevcSample, NevcResult, NevcConfig, NevcSummary, compute_nevc, score_instant, sample_from_rbe_action};
    pub use crate::contribution_ledger::{ContributionLedger, PlayerContribution};
    pub use crate::contribution_events::{ContributionEvent, apply_event, apply_event_class};
    pub use crate::nevc_pipeline_demo::{run_demo, classify};
    pub use crate::real_estate_lattice_nevc::{RealEstateStewardshipEvent, apply_real_estate_event, sample_from_stewardship};
    pub use crate::nevc_game_loop::{HarvestNevcInput, harvest_to_event, apply_harvest_to_ledger, apply_harvest_class, apply_harvest_summary};
    pub use crate::nevc_persistence::{NevcPlayerRecord, NevcPersistenceStore};
    pub use crate::nevc_bridge::{compute_nevc_bridged, score_instant_bridged, summary_bridged, active_mode};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_is_reachable() {
        let r = nevc_bridge::score_instant_bridged(0.999999, 0.0);
        assert!(r.is_contributor());
        assert!(!nevc_bridge::active_mode().is_empty());
    }
}
