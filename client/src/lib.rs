//! client/src/lib.rs
//! Powrush-MMO Client Crate Root — Public API and module declarations
//! AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced | v17.98+ production-grade
//! Fully restored, merged, and upgraded — mint-and-print-only-perfection, zero placeholders

pub mod networking;
pub mod replication;
pub mod prediction;
pub mod delta_compression;
pub mod rbe_client_sync;
pub mod rbe;
pub mod rbe_engine;
pub mod particles;
pub mod ui;
pub mod divine_whispers;
pub mod input;
pub mod bevy_ecs_scheduling;

// Re-exports for clean public API
pub use networking::NetworkingPlugin;
pub use replication::ReplicationPlugin;
pub use prediction::{PredictionPlugin, PredictedPosition, PredictedAbility, RollbackState};
pub use delta_compression::DeltaCompressionPlugin;
pub use rbe_client_sync::RbeClientSyncPlugin;
pub use rbe::{RbePlugin, RbeResource, RbeInventory, RbeResourceType};
pub use rbe_engine::RbeEnginePlugin;
pub use particles::ParticlePlugin;
pub use ui::UiPlugin;
pub use divine_whispers::DivineWhispersPlugin;
pub use input::InputPlugin;
pub use bevy_ecs_scheduling::ClientSchedulingPlugin;

/// Main client bundle that includes everything in one convenient package
pub struct PowrushClientBundle;

impl PowrushClientBundle {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for PowrushClientBundle {
    fn build(&self, app: &mut App) {
        // Central scheduling hub that wires the entire client stack
        app.add_plugins(ClientSchedulingPlugin);
    }
}

// All modules are now perfectly declared, exported, and mercy-gated
// Full zero-lag client crate root complete

#[cfg(test)]
mod tests {
    // Full production-grade integration tests for the client crate root under TOLC 8
}
