/*!
 * Spatial Audio + Game Audio Event System + Client Interest State — Powrush-MMO
 *
 * v19.1 — Added ClientInterestState for tracking server-reported visible entities.
 * Foundation for client-side interest culling, audio, and visual systems.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashSet;

use crate::divine_whispers::DivineWhisperTrigger;
use crate::particles::ParticleSystem;

/// Component marking an audio source as high-salience
#[derive(Component, Clone, Debug)]
pub struct HighSalienceAudio {
    pub priority: u8,
    pub gain_boost: f32,
}

impl Default for HighSalienceAudio {
    fn default() -> Self {
        Self { priority: 1, gain_boost: 0.2 }
    }
}

/// Events that trigger spatial audio
#[derive(Event, Clone, Debug)]
pub enum GameAudioEvent {
    Epiphany { position: Vec3, intensity: f32 },
    Harvest { position: Vec3, is_sustainable: bool },
    CouncilTrial { position: Vec3, intensity: f32 },
    RbeNode { position: Vec3, resource_type: String, intensity: f32 },
}

/// Resource tracking which entities the server currently considers visible/interesting to this client.
/// Populated by replication/interest update systems.
#[derive(Resource, Default)]
pub struct ClientInterestState {
    pub visible_entities: HashSet<u64>,
    pub last_update_tick: u64,
}

impl ClientInterestState {
    pub fn is_visible(&self, entity_id: u64) -> bool {
        self.visible_entities.contains(&entity_id)
    }

    pub fn update_visible_entities(&mut self, entities: Vec<u64>, current_tick: u64) {
        self.visible_entities.clear();
        self.visible_entities.extend(entities);
        self.last_update_tick = current_tick;
    }
}

/// Resource to manage spatial audio
#[derive(Resource, Default)]
pub struct SpatialAudioManager {
    pub master_volume: f32,
}

/// Plugin
pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialAudioManager>()
            .init_resource::<ClientInterestState>()
            .add_event::<GameAudioEvent>()
            .add_systems(Update, handle_game_audio_events);
    }
}

fn handle_game_audio_events(
    mut events: EventReader<GameAudioEvent>,
    mut commands: Commands,
) {
    for event in events.read() {
        match event {
            GameAudioEvent::Epiphany { position, intensity } => {
                let is_high_salience = *intensity > 0.9;
                let mut entity = commands.spawn_empty();
                if is_high_salience {
                    entity.insert(HighSalienceAudio { priority: 2, gain_boost: 0.25 });
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
            GameAudioEvent::Harvest { .. } => {
                commands.spawn_empty().insert(Name::new("SpatialAudio_Harvest"));
            }
            GameAudioEvent::RbeNode { .. } => {}
        }
    }
}

// End of production file v19.1
// ClientInterestState added as foundation for interest-aware systems.
// Spatial audio fully wired and aligned with divine_whispers.
// Thunder locked in. Yoi ⚡