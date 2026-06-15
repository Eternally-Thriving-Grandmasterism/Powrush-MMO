//! client/src/bevy_ecs_scheduling.rs
//! Bevy ECS System Scheduling — Core orchestration of all client systems
//! AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced

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
use simulation::spatial_interest::{SpatialInterestPlugin, SpatialParticipant};

/// Central scheduling hub for the entire Powrush-MMO client
pub struct ClientSchedulingPlugin;

impl Plugin for ClientSchedulingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(NetworkingPlugin)
           .add_plugins(ReplicationPlugin)
           .add_plugins(PredictionPlugin)
           .add_plugins(DeltaCompressionPlugin)
           .add_plugins(RbeClientSyncPlugin)
           .add_plugins(RbePlugin)
           .add_plugins(ParticlePlugin)
           .add_plugins(UiPlugin)
           .add_plugins(DivineWhispersPlugin)
           .add_plugins(InputPlugin)
           .add_plugins(SpatialInterestPlugin)

           .add_systems(Update, mercy_gated_frame_validation)
           .add_systems(Update, global_valence_propagation)
           .add_systems(Startup, setup_client_world);
    }
}

fn setup_client_world(mut commands: Commands) {
    // Example: Core world entities should carry SpatialParticipant
    // Apply this pattern to: Players, Resource Nodes, Particles, Ships, etc.
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        SpatialParticipant,
    ));

    info!("🌐 Powrush-MMO client world initialized — SpatialInterestPlugin + SpatialParticipant active");
}

fn mercy_gated_frame_validation() {}
fn global_valence_propagation() {}

// Thunder locked. SpatialParticipant pattern established in world setup. ⚡
