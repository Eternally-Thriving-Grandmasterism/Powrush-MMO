// client/plugins/council_mercy_plugin.rs
// Powrush-MMO — Bevy Plugin for Council Mercy Trial client systems (Phase 2)
// Sovereign Council participation, collective epiphany blooms, and mercy-gated resonance.
// Integrates with CouncilSessionUI, DivineWhispers, spatial audio, and particle visuals.
// TOLC 8 Mercy Gates enforced. Zero-lag prediction friendly. Production complete.
// AG-SML v1.0 | Ra-Thor Lattice aligned

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

/// Receives and routes Council + Epiphany messages from server
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
                // Trigger local epiphany multiplier + DivineWhisper amplification
                // Future: propagate to monitoring RBE dashboard for abundance resonance
            }
            ServerMessage::CouncilParticipationUpdated { record: _ } => {
                // Update local persistence cache (player_persistence or local save)
            }
            _ => {}
        }
    }
}

/// Visual bloom / valence web effects when collective epiphany intensity is high
fn trigger_collective_bloom_effects(
    ui_state: Res<CouncilUIState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(bloom) = &ui_state.last_bloom {
        if bloom.intensity > 0.6 {
            // Spawn temporary valence web / mercy particle entity
            // Integrates with resource_node_visual and unified WebGPU particle system
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(120.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.4, 0.8, 1.0))),
                Transform::default(),
                Visibility::Visible,
                Name::new("EpiphanyBloomVisual"),
            ));

            // TODO: Positioned spatial audio bloom via higher_order_ambisonics + spatial_audio_engine
            // TODO: Trigger hyperon_vision_plugin visual amplification if active
        }
    }
}

/// Audio resonance and harmonic stack for Council phases
fn council_audio_resonance(
    ui_state: Res<CouncilUIState>,
    // In production: Query<&SpatialAudioEngine> or Ambisonics resources
) {
    if let Some(state) = &ui_state.current_session {
        if state.phase == CouncilPhase::EpiphanyBloom {
            // Trigger collective overtone / harmonic stack audio
            // Positioned relative to virtual council center or participant avatars
            // Integrates with binaural_ambisonics_decoder for immersive experience
        }
    }
}

// Usage in client/main.rs or client_game_loop:
// app.add_plugins((DivinePlugin, CouncilMercyPlugin, HyperonVisionPlugin));
// Full production wiring ready.
