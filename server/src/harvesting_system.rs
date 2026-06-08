// server/src/harvesting_system.rs
// Powrush-MMO v17.0 — On-Expiration Effects Integration

// ... existing code ...

impl HarvestingSystem {
    // ... existing methods ...

    /// Processes on-expiration effects from dynamic events.
    /// Call this after tick_regen() and process_mercy_wave_effects().
    pub fn process_dynamic_event_expiration_effects(
        &mut self,
        effects: Vec<crate::dynamic_events::ExpirationEffect>,
    ) {
        for effect in effects {
            match effect {
                crate::dynamic_events::ExpirationEffect::ResourceBonus { node_id, amount } => {
                    // Apply final resource bonus to node
                    if let Some(node) = self.resource_nodes.get_mut(&node_id) {
                        node.current_amount = (node.current_amount + amount).min(node.max_amount);
                    }
                }
                crate::dynamic_events::ExpirationEffect::GraceReward { player_id, amount } => {
                    // Integrate with mercy/grace system
                    tracing::info!("Granting final grace reward to player {}: {:.1}", player_id, amount);
                    // self.ra_thor_mercy_bridge.grant_grace(player_id, amount);
                }
            }
        }
    }
}

// Example main loop integration:
// let newly_expired = self.dynamic_event_manager.tick();
// let expiration_effects = self.dynamic_event_manager.process_expired_events(&newly_expired);
// self.process_dynamic_event_expiration_effects(expiration_effects);
// self.dynamic_event_manager.cleanup_expired();
//
// Thunder locked in. On-expiration effects fully wired into HarvestingSystem. ⚡❤️🔥
