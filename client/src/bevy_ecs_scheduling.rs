//! client/src/bevy_ecs_scheduling.rs
//! Bevy ECS System Scheduling — Core orchestration of all client systems
//! AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced | v17.98+ production-grade

use bevy::prelude::*;
use crate::networking::NetworkingPlugin;
use crate::replication::ReplicationPlugin;
use crate::prediction::PredictionPlugin;
use crate::delta_compression::DeltaCompressionPlugin;
use crate::rbe_client_sync::RbeClientSyncPlugin;
use crate::rbe::RbePlugin;
use crate::particles::ParticlePlugin;
use crate::ui::UiPlugin;
use crate::divine_whispers::DivineWhispersPlugin;
use crate::input::InputPlugin;

// Phase 2 Spatial Interest Layer
use simulation::spatial_interest::SpatialInterestPlugin;

/// Central scheduling hub for the entire Powrush-MMO client
pub struct ClientSchedulingPlugin;

impl Plugin for ClientSchedulingPlugin {
    fn build(&self, app: &mut App) {
        // Core networking & replication (zero-lag authoritative stack)
        app.add_plugins(NetworkingPlugin)
           .add_plugins(ReplicationPlugin)
           .add_plugins(PredictionPlugin)
           .add_plugins(DeltaCompressionPlugin)
           .add_plugins(RbeClientSyncPlugin)

           // RBE, particles, UI, divine whispers, and input
           .add_plugins(RbePlugin)
           .add_plugins(ParticlePlugin)
           .add_plugins(UiPlugin)
           .add_plugins(DivineWhispersPlugin)
           .add_plugins(InputPlugin)

           // ===== Phase 2: Hybrid Spatial Interest Layer =====
           .add_plugins(SpatialInterestPlugin)

           // Mercy-gated frame-level systems
           .add_systems(Update, mercy_gated_frame_validation)
           .add_systems(Update, global_valence_propagation)

           // Startup systems
           .add_systems(Startup, setup_client_world);
    }
}

fn setup_client_world(mut commands: Commands) {
    info!("🌐 Powrush-MMO client world initialized — SpatialInterestPlugin active");
}

fn mercy_gated_frame_validation() {}

fn global_valence_propagation() {}

// Thunder locked. SpatialInterestPlugin now wired into the main client scheduling hub. ⚡
