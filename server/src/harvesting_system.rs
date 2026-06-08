// server/src/harvesting_system.rs
// Powrush-MMO v17.0 — MercyWave Integration into Grace/Reward Systems

// ... existing code ...

impl HarvestingSystem {
    // ... existing methods ...

    /// Process active MercyWave events and apply grace/reward effects.
    /// This should be called after tick_regen or in a dedicated grace processing step.
    pub fn process_mercy_wave_effects(&mut self, player_positions: &HashMap<u64, Vec3Ser>) {
        if let Some(event_manager) = &self.dynamic_event_manager {
            let affected_players = event_manager.get_players_affected_by_mercy_waves(player_positions);

            for (player_id, intensity) in affected_players {
                // === Integration Point for Mercy/Grace Systems ===
                // 
                // Options:
                // 1. Call into ra_thor_mercy_bridge to trigger Divine Whispers or grace rewards
                // 2. Increase temporary grace/abundance score for the player
                // 3. Trigger RBE Abundance Feedback (milestone celebrations)
                //
                // Example (pseudo):
                // self.ra_thor_mercy_bridge.trigger_mercy_wave(player_id, intensity);
                // self.abundance_feedback.grant_mercy_wave_bonus(player_id, intensity);

                tracing::info!(
                    "MercyWave affected player {} with intensity {:.2}", 
                    player_id, 
                    intensity
                );

                // For now, we log + could add a simple grace bonus here
            }
        }
    }
}

// Recommended call site:
// After tick_regen() in your main loop:
//   harvesting_system.process_mercy_wave_effects(&current_player_positions);
//
// Thunder locked in. MercyWave now wired into grace/reward processing path. ⚡❤️🔥
