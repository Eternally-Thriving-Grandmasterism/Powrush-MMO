// shared/lib.rs
// Powrush-MMO — Professional Shared Crate Root
// Phase 0–7 NEVC surfaces: adapter, ledger, events, demo, real-estate,
// game-loop helpers, and persistence.
// AG-SML v1.0 | PATSAGi Councils | info@Rathor.ai

pub mod protocol;
pub mod nevc_adapter;
pub mod contribution_ledger;
pub mod contribution_events;
pub mod nevc_pipeline_demo;
pub mod real_estate_lattice_nevc;
pub mod nevc_game_loop;
pub mod nevc_persistence;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_compiles_and_exports() {
        let _ = protocol::ClientMessage::Ping { client_time_ms: 0 };
        assert!(true);
    }

    #[test]
    fn full_nevc_pipeline_is_reachable() {
        let results = nevc_pipeline_demo::run_demo();
        assert_eq!(results.len(), 2);
        assert!(results[0].1.class.is_contributor());
        assert!(!results[1].1.class.is_contributor());
    }

    #[test]
    fn persistence_store_is_reachable() {
        let mut store = nevc_persistence::NevcPersistenceStore::new();
        store.absorb(1, 0.9);
        assert!(store.class_of(1).is_contributor());
    }
}
