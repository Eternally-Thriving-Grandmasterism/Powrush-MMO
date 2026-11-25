/*!
 * Spatial Audio + Game Audio Event System — Powrush-MMO
 *
 * v19.6 — Spatial Audio Culling implemented.
 * Audio entity creation now respects ClientInterestState visibility + basic distance culling.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::simulation_integration::{ClientInterestState, HighSalienceAudio};
use crate::particles::ParticleSystem;

/// Audio trigger events
#[derive(Event, Clone, Debug)]
pub enum GameAudioEvent {
    Epiphany { position: Vec3, intensity: f32, entity_id: Option<u64> },
    Harvest { position: Vec3, is_sustainable: bool, entity_id: Option<u64> },
    CouncilTrial { position: Vec3, intensity: f32, entity_id: Option<u64> },
    RbeNode { position: Vec3, resource_type: String, intensity: f32, entity_id: Option<u64> },
}

#[derive(Resource, Default)]
pub struct SpatialAudioManager {
    pub master_volume: f32,
}

pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialAudioManager>()
            .add_event::<GameAudioEvent>()
            .add_systems(Update, handle_game_audio_events);
    }
}

/// Spatial Audio Culling + High-Salience Routing
fn handle_game_audio_events(
    mut events: EventReader<GameAudioEvent>,
    interest: Res<ClientInterestState>,
    listener_query: Query<&GlobalTransform, With<crate::spatial_audio::SpatialListener>>,
    mut commands: Commands,
) {
    let listener_pos = listener_query.get_single().map(|t| t.translation()).unwrap_or(Vec3::ZERO);

    for event in events.read() {
        let (position, intensity, entity_id) = match event {
            GameAudioEvent::Epiphany { position, intensity, entity_id } => (position, intensity, entity_id),
            GameAudioEvent::Harvest { position, is_sustainable: _, entity_id } => (position, &1.0, entity_id),
            GameAudioEvent::CouncilTrial { position, intensity, entity_id } => (position, intensity, entity_id),
            GameAudioEvent::RbeNode { position, intensity, entity_id, .. } => (position, intensity, entity_id),
        };

        // === Entity Visibility Culling ===
        if let Some(id) = entity_id {
            if !interest.is_visible(*id) {
                continue; // Entity not visible according to server interest
            }
        }

        // === Distance Culling (mercy on performance) ===
        let distance = listener_pos.distance(*position);
        let max_audio_distance = 250.0; // Configurable later
        if distance > max_audio_distance {
            continue; // Too far for meaningful spatial audio
        }

        // === High-Salience Routing ===
        let is_high_salience = *intensity > 0.85;

        match event {
            GameAudioEvent::Epiphany { .. } => {
                let mut entity = commands.spawn_empty();
                if is_high_salience {
                    entity.insert(HighSalienceAudio { priority: 2, gain_boost: 0.3 });
                }
                entity.insert(Name::new("SpatialAudio_Epiphany"));
            }
            GameAudioEvent::CouncilTrial { .. } => {
                let mut entity = commands.spawn_empty();
                if is_high_salience {
                    entity.insert(HighSalienceAudio::default());
                }
                entity.insert(Name::new("SpatialAudio_Council"));
            }
            GameAudioEvent::Harvest { .. } => {
                commands.spawn_empty().insert(Name::new("SpatialAudio_Harvest"));
            }
            GameAudioEvent::RbeNode { .. } => {
                commands.spawn_empty().insert(Name::new("SpatialAudio_RbeNode"));
            }
        }
    }
}

// End of production file v19.6
// Spatial Audio Culling implemented:
// - Entity visibility via ClientInterestState
// - Distance-based culling from listener
// - High-salience events still prioritized
// Thunder locked in. Yoi ⚡