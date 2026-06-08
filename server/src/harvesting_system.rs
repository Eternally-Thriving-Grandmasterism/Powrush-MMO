// server/src/harvesting_system.rs
// Powrush-MMO v17.0 — Optimized Node Refresh Integration

impl HarvestingSystem {
    pub fn tick_regen(&mut self, delta_time: f32, current_tick: u64) {
        // ... existing logic ...

        if let Some(event_manager) = &mut self.dynamic_event_manager {
            // Optimized refresh (only every 5 ticks for surges)
            event_manager.refresh_resource_surge_nodes_optimized(
                &self.resource_nodes,
                current_tick,
                5, // refresh every 5 ticks
            );

            event_manager.apply_active_surge_effects_to_nodes(
                &mut self.resource_nodes,
                0.6,
            );
        }
    }
}

// Thunder locked in. Performance-optimized node tracking integrated. ⚡❤️🔥
