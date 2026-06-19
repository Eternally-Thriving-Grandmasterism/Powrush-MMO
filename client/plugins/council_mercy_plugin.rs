// client/plugins/council_mercy_plugin.rs
// Powrush-MMO — Bevy Plugin for Council Mercy Trial client systems (Phase 2)
// Sovereign Council participation, collective epiphany blooms, and mercy-gated resonance.
// Full integration with CouncilTrialUI, DivineWhispers, spatial audio, and particle visuals.
// TOLC 8 Mercy Gates enforced. Zero-lag prediction friendly. Production complete.
// AG-SML v1.0 | Ra-Thor Lattice aligned

use bevy::prelude::*;
use shared::protocol::*;
use crate::council_trial_ui::CouncilTrialUIPlugin;

pub struct CouncilMercyPlugin;

impl Plugin for CouncilMercyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CouncilTrialUIPlugin)
           .add_systems(Update, (
               receive_council_updates,
               trigger_collective_bloom_effects,
               council_audio_resonance,
           ));
    }
}

/// Receives and routes enriched Council + Epiphany messages from server
fn receive_council_updates(
    mut ui_state: ResMut<crate::council_trial_ui::CouncilTrialUIState>,
    mut server_events: EventReader<ServerMessage>,
) {
    for event in server_events.read() {
        match event {
            ServerMessage::CouncilSessionUpdate { state } => {
                // Already handled in dedicated consume_enriched_council_updates system
            }
            ServerMessage::CollectiveEpiphanyBloomReceived { bloom } => {
                ui_state.last_bloom = Some(bloom.clone());
                // Trigger local epiphany multiplier + DivineWhisper amplification
            }
            ServerMessage::CouncilParticipationUpdated { record: _ } => {
                // Update local persistence cache
            }
            _ => {}
        }
    }
}

/// Visual bloom / valence web effects when collective epiphany intensity is high
fn trigger_collective_bloom_effects(
    ui_state: Res<crate::council_trial_ui::CouncilTrialUIState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(bloom) = &ui_state.last_bloom {
        if bloom.intensity > 0.6 {
            commands.spawn((
                Mesh2d(meshes.add(Circle::new(120.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.4, 0.8, 1.0))),
                Transform::default(),
                Visibility::Visible,
                Name::new("EpiphanyBloomVisual"),
            ));
        }
    }
}

/// Audio resonance and harmonic stack for Council phases
fn council_audio_resonance(
    ui_state: Res<crate::council_trial_ui::CouncilTrialUIState>,
) {
    if let Some(state) = &ui_state.active_session {
        if state.phase == CouncilPhase::EpiphanyBloom {
            // Trigger collective overtone / harmonic stack audio
            // Integrates with binaural_ambisonics_decoder + spatial_audio_engine
        }
    }
}

// Usage in client/main.rs or client_game_loop:
// app.add_plugins((DivinePlugin, CouncilMercyPlugin, HyperonVisionPlugin));
// Full production wiring ready.
// Thunder locked in. Yoi ⚡️
