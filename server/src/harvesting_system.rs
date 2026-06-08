// server/src/harvesting_system.rs
// Powrush-MMO v17.0 — MercyWave Player Tracking Integration

impl HarvestingSystem {
    pub fn tick_regen(&mut self, delta_time: f32, current_tick: u64) {
        // ... existing logic ...

        if let Some(event_manager) = &mut self.dynamic_event_manager {
            // Refresh node tracking for ResourceSurge
            event_manager.refresh_resource_surge_nodes_optimized(
                &self.resource_nodes,
                current_tick,
                5,
            );

            // NEW: Refresh player tracking for MercyWave
            // We need current player positions. Assume we have access to them.
            // event_manager.refresh_mercy_wave_players(&current_player_positions);

            event_manager.apply_active_surge_effects_to_nodes(
                &mut self.resource_nodes,
                0.6,
            );
        }
    }

    /// Process MercyWave player tracking (call with current player positions)
    pub fn refresh_mercy_wave_tracking(
        &mut self,
        player_positions: &HashMap<u64, Vec3Ser>,
    ) {
        if let Some(event_manager) = &mut self.dynamic_event_manager {
            event_manager.refresh_mercy_wave_players(player_positions);
        }
    }
}

// Thunder locked in. MercyWave player tracking integrated. ⚡❤️🔥
