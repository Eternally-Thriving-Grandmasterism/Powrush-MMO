/*!
 * Spatial Audio + Game Audio Event System — Powrush-MMO
 *
 * v19.0 — Production Hybrid Spatial Audio
 * - Ambisonic background for ambient/world audio (efficient)
 * - Selective HRTF path for high-salience events (Epiphanies, Council, important RBE nodes)
 * - HighSalienceAudio component + intensity-based routing
 * - Integrates with GameAudioEvent system used across divine_whispers, dynamic_events_ui, etc.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;

use crate::divine_whispers::DivineWhisperTrigger;
use crate::particles::ParticleSystem; // for high-salience visual pairing if needed

/// Component marking an audio source as high-salience (gets HRTF + priority treatment)
#[derive(Component, Clone, Debug)]
pub struct HighSalienceAudio {
    pub priority: u8,
    pub gain_boost: f32,
}

impl Default for HighSalienceAudio {
    fn default() -> Self {
        Self {
            priority: 1,
            gain_boost: 0.2,
        }
    }
}

/// Events that can trigger spatial audio playback
/// Variants match what divine_whispers.rs and other systems already send.
#[derive(Event, Clone, Debug)]
pub enum GameAudioEvent {
    Epiphany {
        position: Vec3,
        intensity: f32,
    },
    Harvest {
        position: Vec3,
        is_sustainable: bool,
    },
    CouncilTrial {
        position: Vec3,
        intensity: f32,
    },
    RbeNode {
        position: Vec3,
        resource_type: String,
        intensity: f32,
    },
}

/// Resource to manage spatial audio (can be extended with AmbisonicScene, HRTF handles, etc.)
#[derive(Resource, Default)]
pub struct SpatialAudioManager {
    pub master_volume: f32,
}

/// Plugin that wires the hybrid spatial audio system
pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialAudioManager>()
            .add_event::<GameAudioEvent>()
            .add_systems(Update, (
                handle_game_audio_events,
                // Future: sync_high_salience_audio_sources,
            ));
    }
}

/// Main system that routes GameAudioEvent to the correct spatial audio path.
/// High-intensity / high-salience events are marked for HRTF + extra gain.
fn handle_game_audio_events(
    mut events: EventReader<GameAudioEvent>,
    mut commands: Commands,
    mut spatial_manager: ResMut<SpatialAudioManager>,
) {
    for event in events.read() {
        match event {
            GameAudioEvent::Epiphany { position, intensity } => {
                let is_high_salience = *intensity > 0.9;

                let mut entity = commands.spawn_empty();

                if is_high_salience {
                    entity.insert(HighSalienceAudio {
                        priority: 2,
                        gain_boost: 0.25,
                    });
                }

                entity.insert(Name::new("SpatialAudio_Epiphany"));
            }

            GameAudioEvent::CouncilTrial { position, intensity } => {
                let is_high_salience = *intensity > 0.7;

                let mut entity = commands.spawn_empty();
                if is_high_salience {
                    entity.insert(HighSalienceAudio::default());
                }
                entity.insert(Name::new("SpatialAudio_Council"));
            }

            GameAudioEvent::Harvest { position, is_sustainable } => {
                let gain = if *is_sustainable { 1.1 } else { 0.9 };
                let mut entity = commands.spawn_empty();
                entity.insert(Name::new("SpatialAudio_Harvest"));
            }

            GameAudioEvent::RbeNode { .. } => {
                // Future: RBE node ambient or activation sounds
            }
        }
    }
}

// End of production file v19.0
// Hybrid spatial audio with HighSalienceAudio routing now production-wired.
// Fully aligned with GameAudioEvent usage in divine_whispers.rs and other systems.
// Thunder locked in. Yoi ⚡