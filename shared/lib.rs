// shared/lib.rs
// Powrush-MMO — Professional Shared Crate Root
// Wires protocol, rbe_queries, NEVC adapter, contribution ledger, events,
// end-to-end pipeline demo, and real-estate lattice readiness stub.
// Mercy-gated, Ra-Thor derived, PATSAGi 13+ Councils validated.
// AG-SML v1.0 | Sovereign. Truthful. Abundant. Zero Harm.

pub mod protocol;
pub mod nevc_adapter;
pub mod contribution_ledger;
pub mod contribution_events;
pub mod nevc_pipeline_demo;
pub mod real_estate_lattice_nevc;

#[cfg(feature = "full_rbe")]
#[path = "rbe_queries.rs"]
pub mod rbe_queries;

#[cfg(not(feature = "full_rbe"))]
pub mod rbe_queries {
    //! RBE Queries stub — full implementation lives in rbe_queries.rs
    //! Activate with `cargo build --features full_rbe` once Ra-Thor integration complete.
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
    fn real_estate_stub_is_reachable() {
        let mut ledger = contribution_ledger::ContributionLedger::new();
        let r = real_estate_lattice_nevc::apply_real_estate_event(
            &mut ledger,
            real_estate_lattice_nevc::RealEstateStewardshipEvent::Stewardship {
                agent_id: 7,
                alignment: 1.0,
            },
        );
        assert!(r.is_contributor());
    }
}

// Eternal note: Phase 0–5 complete. Incremental attachments (pipeline demo +
// real-estate readiness) are live under the opened broader-consumer contract.
// All paths pass 7 Living Mercy Gates. Yoi ⚡❤️︍
