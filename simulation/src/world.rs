/*!
 * Automatic periodic rebuild of zone_node_cache in tick().
 */

impl SovereignWorldState {
    pub fn tick(&mut self, dt_ms: u64) -> Result<(), MercyViolation> {
        self.sim_time += dt_ms;

        // ... existing biome logic ...

        // === Periodic spatial cache rebuild for policy performance ===
        // Rebuild every 200 ticks (~3-4 seconds at 60 TPS) to keep queries fast
        // without significant overhead.
        if self.sim_time % 200 == 0 {
            self.rebuild_zone_node_cache();
        }

        self.apply_and_decay_policies();

        Ok(())
    }
}
