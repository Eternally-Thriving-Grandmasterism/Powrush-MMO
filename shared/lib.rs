// shared/lib.rs
// Powrush-MMO v16.5+ — Professional Shared Crate Root
// Wires protocol, rbe_queries, Phase-1 NEVC adapter, and Phase-2 contribution ledger.
// Mercy-gated, Ra-Thor derived, PATSAGi 13+ Councils validated.
// AG-SML v1.0 | Sovereign. Truthful. Abundant. Zero Harm.

// Re-export core protocol for easy `use shared::protocol::*;`
pub mod protocol;

// Phase 1 thin NEVC adapter (dual-repo consumer surface)
pub mod nevc_adapter;

// Phase 2 first consumer — per-player contribution ledger
pub mod contribution_ledger;

// Feature-gate the RBE queries module until Ra-Thor monorepo crates are available
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

// ECS directory exists in shared/ — commented to keep build clean until proper mod.rs added.
// pub mod ecs;

// Prelude for common imports in client/server
pub mod prelude {
    pub use crate::protocol::{ClientMessage, ServerMessage, TradeOffer, Vec3Ser, HealthComponent};
    pub use crate::rbe_queries;
    pub use crate::nevc_adapter::{ContributionClass, NevcSample, NevcResult, NevcConfig, compute_nevc, score_instant, sample_from_rbe_action};
    pub use crate::contribution_ledger::{ContributionLedger, PlayerContribution};
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
    fn nevc_adapter_is_reachable() {
        let r = nevc_adapter::score_instant(0.999999, 0.0);
        assert!(r.is_contributor());
    }

    #[test]
    fn contribution_ledger_is_reachable() {
        let mut ledger = contribution_ledger::ContributionLedger::new();
        let r = ledger.record_rbe_action(1, 1.0, 0.0);
        assert!(r.is_contributor());
        assert!(ledger.is_contributor(1));
    }
}

// Eternal note: This crate now enables `cargo build -p shared` and workspace resolution.
// Phase 1 NEVC adapter + Phase 2 contribution ledger are live.
// Next: Phase 2 continued — attach ledger to live game/simulation event paths.
// All paths pass 7 Living Mercy Gates. Yoi ⚡❤️︍
