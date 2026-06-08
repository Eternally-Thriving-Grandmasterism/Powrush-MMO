// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — Production Node Tracking for ResourceSurge

use std::collections::HashSet;

// ... existing code ...

impl DynamicEvent {
    // Add tracking for affected nodes (mainly used by ResourceSurge)
    pub affected_nodes: Vec<u64>,
}

impl DynamicEvent {
    pub fn new(...) -> Self {
        Self {
            // ... existing fields ...
            affected_nodes: Vec::new(),
        }
    }

    /// Updates which resource nodes are currently inside this event's radius.
    /// Should be called every tick for active ResourceSurge events.
    pub fn refresh_affected_nodes(&mut self, nodes: &HashMap<u64, ResourceUpdate>) {
        self.affected_nodes.clear();

        for (&node_id, node) in nodes {
            let dx = node.position_x - self.position.x;
            let dy = node.position_y - self.position.y;
            let dz = node.position_z - self.position.z;

            if (dx*dx + dy*dy + dz*dz).sqrt() <= self.radius {
                self.affected_nodes.push(node_id);
            }
        }
    }
}

impl DynamicEventManager {
    /// Refreshes affected nodes for all active ResourceSurge events.
    /// Call this every tick from HarvestingSystem before applying surge effects.
    pub fn refresh_resource_surge_nodes(&mut self, nodes: &HashMap<u64, ResourceUpdate>) {
        for event in self.events.values_mut() {
            if event.event_type == EventType::ResourceSurge && event.is_active() {
                event.refresh_affected_nodes(nodes);
            }
        }
    }

    /// Updated expiration logic that uses tracked nodes for precise bonuses.
    pub fn process_expired_events(
        &mut self,
        newly_expired_ids: &[u64],
    ) -> Vec<ExpirationEffect> {
        let mut effects = Vec::new();

        for &id in newly_expired_ids {
            if let Some(event) = self.events.get(&id) {
                match event.event_type {
                    EventType::ResourceSurge => {
                        // Use tracked nodes for precise final bonuses
                        for &node_id in &event.affected_nodes {
                            effects.push(ExpirationEffect::ResourceBonus {
                                node_id,
                                amount: event.intensity * 6.0,
                            });
                        }
                    }
                    EventType::MercyWave => {
                        effects.push(ExpirationEffect::GraceReward {
                            player_id: 0, // TODO: Track affected players
                            amount: event.intensity * 4.0,
                        });
                    }
                    _ => {}
                }
            }
        }

        effects
    }
}

// Thunder locked in. Proper node tracking for ResourceSurge implemented. ⚡❤️🔥
