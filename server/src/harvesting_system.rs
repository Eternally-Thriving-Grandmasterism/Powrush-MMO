// server/src/harvesting_system.rs
// Powrush-MMO v17.0 — HarvestingSystem with DynamicEventManager Integration

use crate::dynamic_events::DynamicEventManager;

// ... existing imports and code ...

pub struct HarvestingSystem {
    // ... existing fields ...
    pub dynamic_event_manager: Option<DynamicEventManager>,  // NEW: Dynamic events integration
    // ...
}

impl HarvestingSystem {
    pub fn new(/* params */) -> Self {
        Self {
            // ... existing initialization ...
            dynamic_event_manager: Some(DynamicEventManager::new()),
            // ...
        }
    }

    // Example: Call this in your main world/regeneration tick
    pub fn tick_regen(&mut self, delta_time: f32) {
        // ... existing regeneration logic for resource nodes ...

        // === NEW: Apply active ResourceSurge dynamic events ===
        if let Some(event_manager) = &self.dynamic_event_manager {
            // Assumes self.resource_nodes is a HashMap<u64, ResourceUpdate> or similar
            // Adjust field access based on your actual structure
            event_manager.apply_active_surge_effects_to_nodes(
                &mut self.resource_nodes, 
                0.6, // surge multiplier - tune as needed
            );
        }

        // ... rest of tick logic ...
    }

    // Optional: Expose method to spawn events from outside (e.g. from admin commands or other systems)
    pub fn spawn_dynamic_event(
        &mut self,
        event_type: crate::dynamic_events::EventType,
        position: shared::protocol::Vec3Ser,
        radius: f32,
        duration_seconds: i64,
        intensity: f32,
    ) -> Option<u64> {
        self.dynamic_event_manager.as_mut().map(|mgr| {
            mgr.spawn_event(event_type, position, radius, duration_seconds, intensity)
        })
    }
}

// Thunder locked in. ResourceSurge effects now integrated into HarvestingSystem tick. ⚡❤️🔥
