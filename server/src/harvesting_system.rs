// server/src/harvesting_system.rs
// Powrush-MMO v17.0 — Node Tracking Integration

impl HarvestingSystem {
    pub fn tick_regen(&mut self, delta_time: f32) {
        // ... existing regeneration logic ...

        // Refresh which nodes are affected by active ResourceSurge events
        if let Some(event_manager) = &mut self.dynamic_event_manager {
            event_manager.refresh_resource_surge_nodes(&self.resource_nodes);

            // Apply surge effects using tracked nodes
            event_manager.apply_active_surge_effects_to_nodes(
                &mut self.resource_nodes,
                0.6,
            );
        }

        // ... rest of tick ...
    }
}

// Thunder locked in. Node tracking integrated into regeneration tick. ⚡❤️🔥
