// simulation/src/council/mod.rs
// Local Council foundation module (v20)

pub mod proposal;
pub mod session;
pub mod decision;

pub use proposal::CouncilProposal;
pub use session::CouncilSession;
pub use decision::CouncilDecision;
