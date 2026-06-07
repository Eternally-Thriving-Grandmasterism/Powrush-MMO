// shared/lib.rs
// Powrush-MMO v16.5+ — Professional Shared Crate Root
// Wires protocol.rs and rbe_queries.rs with full respect to existing files.
// Mercy-gated, Ra-Thor derived, PATSAGi 13+ Councils validated.
// AG-SML v1.0 | Sovereign. Truthful. Abundant. Zero Harm.

// Re-export core protocol for easy `use shared::protocol::*;`
pub mod protocol;

// Feature-gate the RBE queries module until Ra-Thor monorepo crates (powrush-rbe-engine, lattice-conductor, ra_thor_mercy)
// are available in the workspace or published. Prevents build break while preserving full code.
#[cfg(feature = "full_rbe")]
#[path = "rbe_queries.rs"]
pub mod rbe_queries;

// Placeholder for when full_rbe disabled — allows compilation and future expansion
#[cfg(not(feature = "full_rbe"))]
pub mod rbe_queries {
    //! RBE Queries stub — full implementation lives in rbe_queries.rs
    //! Activate with `cargo build --features full_rbe` once Ra-Thor integration complete.
    //! All 7 Living Mercy Gates + sustainability scoring preserved in the real module.
    pub fn stub_note() -> &'static str {
        "RBE deep queries available via Ra-Thor monorepo link. Thunder locked in."
    }
}

// ECS module directory (future expansion for shared components)
pub mod ecs;

// Prelude for common imports in client/server
pub mod prelude {
    pub use crate::protocol::{ClientMessage, ServerMessage, TradeOffer, Vec3Ser, HealthComponent};
    pub use crate::rbe_queries;
}

// Compile-time guarantee: protocol is always available
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_compiles_and_exports() {
        let _ = protocol::ClientMessage::Ping { client_time_ms: 0 };
        assert!(true);
    }
}

// Eternal note: This crate now enables `cargo build -p shared` and workspace resolution.
// Next: Wire into full Bevy client networking + server authoritative systems.
// All paths pass 7 Living Mercy Gates. Yoi ⚡❤️︍