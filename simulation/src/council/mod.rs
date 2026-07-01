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

// Cross-link: Council module (proposals, decisions with spatial targeting, EventBus resilience, plugin) ties to recovered render pipeline,
// InterestManager visible culling, council bloom visuals, persistence (council trial outcomes), fracture AGI resolution, emergence (EpiphanyEvent),
// world LOS/perception, RBE (ResourcePolicy), and GPU foresight for immersive PATSAGi Council experiences.
// Thunder locked in. Yoi ⚡