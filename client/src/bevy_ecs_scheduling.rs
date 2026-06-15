//! client/src/bevy_ecs_scheduling.rs
//! Client Scheduling with Versioned Replication Handlers

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

use simulation::spatial_interest::SpatialInterestPlugin;

use crate::prediction::{
    client_predict_local_player_movement,
    predict_interest_zone_expansion,
    handle_interest_zone_replicated,
    handle_council_bloom_state_replicated,
};

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
           .init_resource::<crate::prediction::ClientBloomState>()

           // Prediction + Versioned Replication
           .add_systems(Update, client_predict_local_player_movement)
           .add_systems(Update, predict_interest_zone_expansion)
           .add_systems(Update, handle_interest_zone_replicated)
           .add_systems(Update, handle_council_bloom_state_replicated)

           .add_systems(Update, mercy_gated_frame_validation)
           .add_systems(Update, global_valence_propagation)
           .add_systems(Startup, setup_client_world);
    }
}

fn setup_client_world(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        crate::spatial_interest::SpatialParticipant,
        crate::spatial_interest::ReplicationVersion::default(),
    ));

    info!("🌐 Powrush-MMO client initialized — Versioned replication active");
}

fn mercy_gated_frame_validation() {}
fn global_valence_propagation() {}
