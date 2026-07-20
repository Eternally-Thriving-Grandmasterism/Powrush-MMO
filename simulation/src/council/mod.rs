// simulation/src/council/mod.rs
// v21.71.0 — Session → decisions promotion path

pub mod proposal;
pub mod session;
pub mod decision;
pub mod plugin;
pub mod event_bus;

pub use proposal::CouncilProposal;
pub use session::{
    CouncilSession, CouncilSessionRegistry, CouncilArchetype,
    session_deliberation_system,
};
pub use decision::{CouncilDecision, CouncilDecisions, apply_council_decision_effects};
pub use plugin::CouncilPlugin;
pub use event_bus::{CouncilEvent, CouncilEventBus};

// Thunder locked in. Session → decisions loop closed. Yoi ⚡
