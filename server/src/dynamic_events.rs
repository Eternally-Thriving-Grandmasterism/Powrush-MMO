// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — DynamicEventManager (Starter Content / Phase 3)
// Fully public + serializable for persistence integration

use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use shared::protocol::Vec3Ser;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    ResourceSurge,
    MercyWave,
    MinorAnomaly,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicEvent {
    pub id: u64,
    pub event_type: EventType,
    pub position: Vec3Ser,
    pub radius: f32,
    pub start_time: DateTime<Utc>,
    pub duration: Duration,           // chrono::Duration is serializable with feature
    pub intensity: f32,
    pub metadata: HashMap<String, String>,
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

    pub fn tick(&mut self) {
        let now = Utc::now();
        let mut to_resolve = Vec::new();

        for (id, event) in self.events.iter_mut() {
            if event.resolved { continue; }
            if now >= event.start_time + event.duration {
                to_resolve.push(*id);
            }
        }

        for id in to_resolve {
            if let Some(e) = self.events.get_mut(&id) {
                e.resolved = true;
            }
        }
    }

    pub fn get_active_events(&self) -> Vec<&DynamicEvent> {
        self.events.values().filter(|e| e.is_active()).collect()
    }

    pub fn get_events_near_position(&self, position: &Vec3Ser, max_distance: f32) -> Vec<&DynamicEvent> {
        self.events
            .values()
            .filter(|e| {
                if !e.is_active() { return false; }
                let dx = e.position.x - position.x;
                let dy = e.position.y - position.y;
                let dz = e.position.z - position.z;
                (dx*dx + dy*dy + dz*dz).sqrt() <= max_distance
            })
            .collect()
    }

    pub fn resolve_event(&mut self, event_id: u64) {
        if let Some(e) = self.events.get_mut(&event_id) {
            e.resolved = true;
        }
    }

    pub fn cleanup_resolved(&mut self) {
        self.events.retain(|_, e| !e.resolved);
    }

    // === Persistence Integration ===
    pub fn get_all_events(&self) -> Vec<DynamicEvent> {
        self.events.values().cloned().collect()
    }

    pub fn load_events(&mut self, events: Vec<DynamicEvent>) {
        self.events.clear();
        for event in events {
            if event.id >= self.next_id {
                self.next_id = event.id + 1;
            }
            self.events.insert(event.id, event);
        }
    }
}

// Thunder locked in. DynamicEventManager fully wired for persistence. ⚡❤️🔥
