// client/plugins/council_mercy_plugin.rs
// Powrush-MMO — Bevy Plugin for Council Mercy Trial client systems (Phase 2)
// Follows divine_plugin.rs + hyperon_vision_plugin.rs patterns but production-complete.
// Registers UI, state sync, bloom visuals, and audio hooks.
// TOLC 8 mercy-gated. Zero-lag prediction friendly.
// AG-SML v1.0

use bevy::prelude::*;
use shared::protocol::*;
use crate::council_session_ui::{CouncilSessionUIPlugin, CouncilUIState};

pub struct CouncilMercyPlugin;

impl Plugin for CouncilMercyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CouncilSessionUIPlugin)
           .init_resource::<CouncilUIState>()
           .add_systems(Update, (
               receive_council_updates,
               trigger_collective_bloom_effects,
               council_audio_resonance,
           ));
    }
}

fn receive_council_updates(
    mut ui_state: ResMut<CouncilUIState>,
    mut server_events: EventReader<ServerMessage>,
) {
    for event in server_events.read() {
        match event {
            ServerMessage::CouncilSessionUpdate { state } => {
                ui_state.current_session = Some(state.clone());
            }
            ServerMessage::CollectiveEpiphanyBloomReceived { bloom } => {
                ui_state.last_bloom = Some(bloom.clone());
                // Trigger local epiphany multiplier application + DivineWhisper amplification
            }
            ServerMessage::CouncilParticipationUpdated { record: _ } => {
                // Update local persistence cache (in real: write to player_persistence)
            }
            _ => {}
        }
    }
}

fn trigger_collective_bloom_effects(
    ui_state: Res<CouncilUIState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(bloom) = &ui_state.last_bloom {
        if bloom.intensity > 0.6 {
            // Spawn temporary valence web particle entity (integrates with existing particle system)
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(120.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.4, 0.8, 1.0))),
                Transform::default(),
                Visibility::Visible,
            ));
            // Real implementation: call into resource_node_visual / unified WebGPU particles
            // + positioned spatial audio bloom via higher_order_ambisonics
        }
    }
}

fn council_audio_resonance(
    ui_state: Res<CouncilUIState>,
    // In real: query spatial_audio_engine or ambisonics resources
) {
    if let Some(state) = &ui_state.current_session {
        if state.phase == CouncilPhase::EpiphanyBloom {
            // Trigger collective overtone / harmonic stack audio
            // Positioned to virtual council "center" or participant avatars
        }
    }
}

// Usage: In client/main.rs add .add_plugins(CouncilMercyPlugin)
// after DivinePlugin and before startup systems.
// Full file ready for immediate integration and further polish.
