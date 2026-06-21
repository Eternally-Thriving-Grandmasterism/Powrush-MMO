// simulation/src/council/mod.rs

pub mod proposal;
pub mod session;
pub mod decision;
pub mod plugin;
pub mod event_bus;

pub use proposal::CouncilProposal;
pub use session::CouncilSession;
pub use decision::{CouncilDecision, CouncilDecisions, apply_council_decision_effects};
pub use plugin::CouncilPlugin;
pub use event_bus::{CouncilEvent, CouncilEventBus};
