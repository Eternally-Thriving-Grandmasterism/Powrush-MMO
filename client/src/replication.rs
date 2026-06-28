/// Re-exports for convenience so other modules can still use:
/// use crate::replication::{CouncilBloomPayload, CouncilBloomReceived};
pub use crate::council_bloom_feedback::{
    CouncilBloomEffect,
    CouncilBloomFeedbackPlugin,
    CouncilBloomParticleAssets,
};

// Note: Full Council Bloom implementation (particles, events, systems)
// has been moved to client/src/council_bloom_feedback.rs for better separation.
// Add CouncilBloomFeedbackPlugin to your client app to enable rich bloom feedback.
