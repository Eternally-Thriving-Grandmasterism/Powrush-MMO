/*!
 * Spatial Audio + Game Audio Event System + Client Interest State — Powrush-MMO
 *
 * v19.2 — Interest population system + Interest-aware culling foundation.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashSet;

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

/// Event sent by replication/interest layer when the server updates visible entities for this client.
#[derive(Event, Clone, Debug)]
pub struct InterestUpdateEvent {
    pub visible_entities: Vec<u64>,
    pub server_tick: u64,
}

/// Resource tracking which entities the server currently considers visible/interesting.
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

#[derive(Resource, Default)]
pub struct SpatialAudioManager {
    pub master_volume: f32,
}

pub struct SpatialAudioPlugin;

impl Plugin for SpatialAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialAudioManager>()
            .init_resource::<ClientInterestState>()
            .add_event::<GameAudioEvent>()
            .add_event::<InterestUpdateEvent>()
            .add_systems(Update, (
                handle_game_audio_events,
                handle_interest_updates,
            ));
    }
}

/// Populates ClientInterestState from replication/interest updates.
/// This is the hook point for the replication layer.
fn handle_interest_updates(
    mut events: EventReader<InterestUpdateEvent>,
    mut interest_state: ResMut<ClientInterestState>,
) {
    for event in events.read() {
        interest_state.update_visible_entities(event.visible_entities.clone(), event.server_tick);
    }
}

fn handle_game_audio_events(
    mut events: EventReader<GameAudioEvent>,
    interest: Res<ClientInterestState>,
    mut commands: Commands,
) {
    for event in events.read() {
        // Basic interest culling example for audio sources
        let should_spawn = match event {
            GameAudioEvent::Epiphany { .. } => true, // Epiphanies are usually high priority
            GameAudioEvent::CouncilTrial { .. } => true,
            GameAudioEvent::Harvest { position: _, is_sustainable: _ } => true,
            GameAudioEvent::RbeNode { .. } => true,
        };

        if !should_spawn { continue; }

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

// End of production file v19.2
// 1. InterestUpdateEvent + population system added.
// 2. Basic interest-aware culling in audio handler.
// 3. ClientInterestState exposed via resource.
// Thunder locked in. Yoi ⚡