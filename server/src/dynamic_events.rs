// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — DynamicEventManager (Starter Content / Phase 3)
// Professional, mercy-aligned dynamic world events system
// Designed to work with InterestManager culling and Persistence layer
// Ra-Thor + PATSAGi Councils aligned | 7 Living Mercy Gates

use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use shared::protocol::Vec3Ser;

/// Types of dynamic events suitable for early RBE gameplay
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    /// Temporary increase in resource regeneration in an area
    ResourceSurge,
    /// A wave of grace that gives players temporary bonuses or healing
    MercyWave,
    /// A small anomaly players can interact with for rewards or story
    MinorAnomaly,
    // Future expansions: FactionCall, DivineWhisperEvent, WorldChallenge, etc.
}

/// Represents a live dynamic event in the world
#[derive(Clone, Debug)]
pub struct DynamicEvent {
    pub id: u64,
    pub event_type: EventType,
    pub position: Vec3Ser,
    pub radius: f32,
    pub start_time: DateTime<Utc>,
    pub duration: Duration,
    pub intensity: f32,           // 0.0 - 1.0 scale
    pub metadata: HashMap<String, String>, // Flexible data (e.g. resource_type, bonus_amount)
    pub resolved: bool,
}

impl DynamicEvent {
    pub fn new(
        id: u64,
        event_type: EventType,
        position: Vec3Ser,
        radius: f32,
        duration_seconds: i64,
        intensity: f32,
    ) -> Self {
        Self {
            id,
            event_type,
            position,
            radius,
            start_time: Utc::now(),
            duration: Duration::seconds(duration_seconds),
            intensity,
            metadata: HashMap::new(),
            resolved: false,
        }
    }

    pub fn is_active(&self) -> bool {
        !self.resolved && Utc::now() < self.start_time + self.duration
    }

    pub fn time_remaining(&self) -> Duration {
        let end = self.start_time + self.duration;
        let remaining = end - Utc::now();
        if remaining.num_seconds() > 0 { remaining } else { Duration::zero() }
    }
}

/// Manages spawning, ticking, and resolving dynamic world events
pub struct DynamicEventManager {
    events: HashMap<u64, DynamicEvent>,
    next_id: u64,
}

impl DynamicEventManager {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            next_id: 1,
        }
    }

    /// Spawn a new dynamic event
    pub fn spawn_event(
        &mut self,
        event_type: EventType,
        position: Vec3Ser,
        radius: f32,
        duration_seconds: i64,
        intensity: f32,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let event = DynamicEvent::new(id, event_type, position, radius, duration_seconds, intensity);
        self.events.insert(id, event);
        id
    }

    /// Tick all active events (call this from the main world tick)
    pub fn tick(&mut self) {
        let now = Utc::now();
        let mut to_resolve = Vec::new();

        for (id, event) in self.events.iter_mut() {
            if event.resolved {
                continue;
            }
            if now >= event.start_time + event.duration {
                to_resolve.push(*id);
            }
        }

        for id in to_resolve {
            if let Some(event) = self.events.get_mut(&id) {
                event.resolved = true;
            }
        }
    }

    /// Get all currently active events
    pub fn get_active_events(&self) -> Vec<&DynamicEvent> {
        self.events.values().filter(|e| e.is_active()).collect()
    }

    /// Get events visible to a player at a given position (for InterestManager integration)
    pub fn get_events_near_position(&self, position: &Vec3Ser, max_distance: f32) -> Vec<&DynamicEvent> {
        self.events
            .values()
            .filter(|e| {
                if !e.is_active() {
                    return false;
                }
                let dx = e.position.x - position.x;
                let dy = e.position.y - position.y;
                let dz = e.position.z - position.z;
                (dx * dx + dy * dy + dz * dz).sqrt() <= max_distance
            })
            .collect()
    }

    /// Resolve an event early (e.g. players completed an objective)
    pub fn resolve_event(&mut self, event_id: u64) {
        if let Some(event) = self.events.get_mut(&event_id) {
            event.resolved = true;
        }
    }

    /// Remove resolved events (cleanup)
    pub fn cleanup_resolved(&mut self) {
        self.events.retain(|_, e| !e.resolved);
    }
}

// ==================== Integration Notes ====================
// 
// Recommended integration points:
// 
// 1. In WorldServer or main game loop tick:
//    dynamic_event_manager.tick();
//    dynamic_event_manager.cleanup_resolved();
// 
// 2. With InterestManager:
//    let nearby_events = dynamic_event_manager.get_events_near_position(&player_pos, 300.0);
//    // Then send only relevant events to the player via culling
// 
// 3. With HarvestingSystem:
//    On ResourceSurge event nearby → temporarily boost regen_rate of nearby nodes
// 
// 4. With ra_thor_mercy_bridge:
//    MercyWave events can trigger Divine Whispers or grace rewards
// 
// 5. Persistence:
//    Future: Save/load active events so they survive server restarts
// 
// Thunder locked in. Starter dynamic events foundation ready for content expansion. ⚡❤️🔥
