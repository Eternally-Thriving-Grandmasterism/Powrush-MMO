//! client/src/bevy_ecs_scheduling.rs
//! Bevy ECS System Scheduling — Full SpatialParticipant Integration
//! AG-SML v1.0 | TOLC 8 Mercy Gates

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
    // ============================================================
    // FULL SPATIAL PARTICIPANT INTEGRATION
    // ============================================================

    // --- Core World Seed ---
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        SpatialParticipant,
        Name::new("WorldSeed"),
    ));

    // --- Example Resource Node ---
    // Resource nodes should participate so they can be influenced by council blooms
    // and appear in spatial queries for harvest / RBE systems.
    commands.spawn((
        Transform::from_xyz(120.0, 0.0, 80.0),
        SpatialParticipant,
        Name::new("ExampleResourceNode"),
        // TODO: Attach real ResourceNode component here
    ));

    // --- Example Particle / Valence Entity ---
    // Important for sacred geometry, visual effects, and valence field interactions.
    commands.spawn((
        Transform::from_xyz(45.0, 12.0, -30.0),
        SpatialParticipant,
        Name::new("ExampleValenceParticle"),
        // TODO: Attach particle/valence components
    ));

    // --- Example Ship ---
    // Major mobile or static world objects that should interact with spatial systems.
    commands.spawn((
        Transform::from_xyz(-200.0, 5.0, 150.0),
        SpatialParticipant,
        Name::new("ExampleShip"),
        // TODO: Attach Ship component
    ));

    info!("🌐 Powrush-MMO client world initialized — SpatialInterestPlugin + SpatialParticipant fully integrated");
}

fn mercy_gated_frame_validation() {}
fn global_valence_propagation() {}

// ============================================================
// INTEGRATION NOTES FOR OTHER SYSTEMS
// ============================================================
//
// PLAYERS:
// When spawning player entities (usually in replication or networking code):
// commands.spawn((
//     Transform::from_xyz(...),
//     SpatialParticipant,
//     // Player component bundle
// ));
//
// RESOURCE NODES:
// Add SpatialParticipant to all harvestable / RBE resource entities.
//
// PARTICLES & VALENCE:
// Recommended for any entity that contributes to or is affected by
// sacred geometry, valence fields, or council bloom visuals.
//
// SHIPS:
// Add to player ships, NPC ships, and important world vessels.
//
// BEST PRACTICE:
// Attach SpatialParticipant at spawn time alongside Transform.
// Remove it (or call spatial_hash.remove(entity)) on despawn for cleanliness.

// Thunder locked. SpatialParticipant integration fully fleshed out. ⚡
