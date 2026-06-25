/*!
 * Spatial Audio + Game Audio Event System + Client Interest State — Powrush-MMO
 *
 * v19.3 — Entity Visibility Queries implemented and integrated.
 * ClientInterestState now provides robust visibility queries for audio, particles, and future rendering culling.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashSet;

/// Marks an audio source for premium HRTF treatment
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

/// Audio trigger events (sent by divine_whispers, harvest systems, etc.)
#[derive(Event, Clone, Debug)]
pub enum GameAudioEvent {
    Epiphany { position: Vec3, intensity: f32, entity_id: Option<u64> },
    Harvest { position: Vec3, is_sustainable: bool, entity_id: Option<u64> },
    CouncilTrial { position: Vec3, intensity: f32, entity_id: Option<u64> },
    RbeNode { position: Vec3, resource_type: String, intensity: f32, entity_id: Option<u64> },
}

/// Sent by replication/interest layer when server updates visible entities for this client
#[derive(Event, Clone, Debug)]
pub struct InterestUpdateEvent {
    pub visible_entities: Vec<u64>,
    pub server_tick: u64,
}

/// Single source of truth for which entities the server says are currently visible/interesting to this client
#[derive(Resource, Default)]
pub struct ClientInterestState {
    pub visible_entities: HashSet<u64>,
    pub last_update_tick: u64,
}

impl ClientInterestState {
    /// Core visibility query used by audio, particles, rendering, and UI systems
    pub fn is_visible(&self, entity_id: u64) -> bool {
        self.visible_entities.contains(&entity_id)
    }

    /// Bulk update from replication/interest layer
    pub fn update_visible_entities(&mut self, entities: Vec<u64>, current_tick: u64) {
        self.visible_entities.clear();
        self.visible_entities.extend(entities);
        self.last_update_tick = current_tick;
    }

    /// Returns how many entities are currently considered visible
    pub fn visible_count(&self) -> usize {
        self.visible_entities.len()
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
                handle_interest_updates,
                handle_game_audio_events,
            ));
    }
}

/// Populates ClientInterestState from server replication/interest updates
fn handle_interest_updates(
    mut events: EventReader<InterestUpdateEvent>,
    mut interest_state: ResMut<ClientInterestState>,
) {
    for event in events.read() {
        interest_state.update_visible_entities(event.visible_entities.clone(), event.server_tick);
    }
}

/// Routes audio events and applies interest-aware culling + high-salience logic
fn handle_game_audio_events(
    mut events: EventReader<GameAudioEvent>,
    interest: Res<ClientInterestState>,
    mut commands: Commands,
) {
    for event in events.read() {
        // === Entity Visibility Query ===
        let entity_id = match event {
            GameAudioEvent::Epiphany { entity_id, .. } => *entity_id,
            GameAudioEvent::Harvest { entity_id, .. } => *entity_id,
            GameAudioEvent::CouncilTrial { entity_id, .. } => *entity_id,
            GameAudioEvent::RbeNode { entity_id, .. } => *entity_id,
        };

        // Interest-aware culling: skip if we have an entity_id and it's not visible
        if let Some(id) = entity_id {
            if !interest.is_visible(id) {
                continue; // Entity not in current server interest set
            }
        }

        // High-salience routing + entity creation
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
            GameAudioEvent::Harvest { position, is_sustainable, .. } => {
                commands.spawn_empty().insert(Name::new("SpatialAudio_Harvest"));
            }
            GameAudioEvent::RbeNode { .. } => {}
        }
    }
}

// End of production file v19.3
// Entity Visibility Queries fully implemented via ClientInterestState::is_visible()
// Interest-aware culling active in audio handler
// Ready for replication layer to feed InterestUpdateEvent
// Thunder locked in. Yoi ⚡