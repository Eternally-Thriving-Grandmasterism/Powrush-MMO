/*!
 * Spatial Audio + Game Audio Event System — Powrush-MMO
 *
 * v19.5 — Cleaned up after Step 3 move. Now imports ClientInterestState,
 * InterestUpdateEvent, and HighSalienceAudio from simulation_integration.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::simulation_integration::{ClientInterestState, InterestUpdateEvent, HighSalienceAudio};
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

fn handle_game_audio_events(
    mut events: EventReader<GameAudioEvent>,
    interest: Res<ClientInterestState>,
    mut commands: Commands,
) {
    for event in events.read() {
        let entity_id = match event {
            GameAudioEvent::Epiphany { entity_id, .. } => *entity_id,
            GameAudioEvent::Harvest { entity_id, .. } => *entity_id,
            GameAudioEvent::CouncilTrial { entity_id, .. } => *entity_id,
            GameAudioEvent::RbeNode { entity_id, .. } => *entity_id,
        };

        if let Some(id) = entity_id {
            if !interest.is_visible(id) {
                continue;
            }
        }

        match event {
            GameAudioEvent::Epiphany { position, intensity, .. } => {
                let is_high_salience = *intensity > 0.9;
                let mut entity = commands.spawn_empty();
                if is_high_salience {
                    entity.insert(HighSalienceAudio { priority: 2, gain_boost: 0.25 });
                }
                entity.insert(Name::new("SpatialAudio_Epiphany"));
            }
            GameAudioEvent::CouncilTrial { position, intensity, .. } => {
                let is_high_salience = *intensity > 0.7;
                let mut entity = commands.spawn_empty();
                if is_high_salience {
                    entity.insert(HighSalienceAudio::default());
                }
                entity.insert(Name::new("SpatialAudio_Council"));
            }
            GameAudioEvent::Harvest { .. } => {
                commands.spawn_empty().insert(Name::new("SpatialAudio_Harvest"));
            }
            GameAudioEvent::RbeNode { .. } => {}
        }
    }
}

// End of production file v19.5
// Interest types moved to simulation_integration.rs (Step 3).
// Spatial audio remains clean and focused on audio logic.
// Thunder locked in. Yoi ⚡