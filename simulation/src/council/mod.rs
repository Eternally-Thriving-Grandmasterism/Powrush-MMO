// simulation/src/council/mod.rs
// v21.79.0 — Session → decisions + RTT host mapper + sim bridge writer

pub mod proposal;
pub mod session;
pub mod decision;
pub mod plugin;
pub mod event_bus;
pub mod rtt_export;
pub mod sim_bridge_writer;

pub use proposal::CouncilProposal;
pub use session::{
    CouncilSession, CouncilSessionRegistry, CouncilArchetype,
    session_deliberation_system,
};
pub use decision::{CouncilDecision, CouncilDecisions, apply_council_decision_effects};
pub use plugin::CouncilPlugin;
pub use event_bus::{CouncilEvent, CouncilEventBus};
pub use rtt_export::{
    CouncilRttExportQueue, CouncilRttExportSignal, council_resolved_to_rtt_export_system,
};
pub use sim_bridge_writer::{SimCouncilBridgeWriterConfig, sim_council_bridge_writer_system};

// Thunder locked in. Yoi ⚡
