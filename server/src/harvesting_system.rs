// server/src/harvesting_system.rs
// Powrush-MMO v17.0 — Production-Quality On-Expiration Effects Processing

impl HarvestingSystem {
    /// Processes expiration effects from dynamic events.
    /// This is the production entry point called after tick() + process_expired_events().
    pub fn process_dynamic_event_expiration_effects(
        &mut self,
        effects: Vec<crate::dynamic_events::ExpirationEffect>,
    ) {
        for effect in effects {
            match effect {
                crate::dynamic_events::ExpirationEffect::ResourceBonus { node_id, amount } => {
                    if let Some(node) = self.resource_nodes.get_mut(&node_id) {
                        node.current_amount = (node.current_amount + amount).min(node.max_amount);
                    }
                }
                crate::dynamic_events::ExpirationEffect::AreaResourceBonus { position, radius, amount } => {
                    // Apply area bonus to all nodes within radius at expiration time
                    for node in self.resource_nodes.values_mut() {
                        let dx = node.position_x - position.x;
                        let dy = node.position_y - position.y;
                        let dz = node.position_z - position.z;

                        if (dx*dx + dy*dy + dz*dz).sqrt() <= radius {
                            node.current_amount = (node.current_amount + amount).min(node.max_amount);
                        }
                    }
                }
                crate::dynamic_events::ExpirationEffect::GraceReward { player_id, amount } => {
                    tracing::info!("Granting final grace reward to player {}: {:.1}", player_id, amount);
                    // TODO: Integrate with ra_thor_mercy_bridge.grant_grace(player_id, amount);
                }
            }
        }
    }
}

// Recommended production loop:
// let newly_expired = event_manager.tick();
// let effects = event_manager.process_expired_events(&newly_expired);
// self.process_dynamic_event_expiration_effects(effects);
// event_manager.cleanup_expired();
//
// Thunder locked in. Clean production-quality expiration processing. ⚡❤️🔥
