// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — DynamicEventManager with Professional Effect Integration

// ... existing code ...

impl DynamicEventManager {
    // ... existing methods ...

    /// Professional integration point for HarvestingSystem.
    /// Call this during node regeneration tick to apply active ResourceSurge effects.
    pub fn apply_active_surge_effects_to_nodes(
        &self,
        nodes: &mut HashMap<u64, ResourceUpdate>,
        base_surge_multiplier: f32,
    ) {
        for event in self.get_active_events() {
            if event.event_type != EventType::ResourceSurge {
                continue;
            }

            let surge_strength = event.intensity * base_surge_multiplier;

            for (node_id, node) in nodes.iter_mut() {
                // Use distance check (assumes ResourceUpdate has position fields or we pass positions separately)
                let dx = node.position_x - event.position.x;
                let dy = node.position_y - event.position.y;
                let dz = node.position_z - event.position.z;
                let dist = (dx*dx + dy*dy + dz*dz).sqrt();

                if dist <= event.radius {
                    node.regen_rate *= 1.0 + surge_strength;
                    node.current_amount = (node.current_amount + surge_strength * 3.0).min(node.max_amount);
                }
            }
        }
    }

    /// Returns players affected by active MercyWave events.
    /// Higher layers (HarvestingSystem / reward system / mercy_bridge) can use this to grant grace.
    pub fn get_players_affected_by_mercy_waves(
        &self,
        player_positions: &HashMap<u64, Vec3Ser>,
    ) -> Vec<(u64, f32)> {
        let mut affected = Vec::new();

        for event in self.get_active_events() {
            if event.event_type != EventType::MercyWave {
                continue;
            }

            for (&player_id, pos) in player_positions {
                let dx = pos.x - event.position.x;
                let dy = pos.y - event.position.y;
                let dz = pos.z - event.position.z;
                if (dx*dx + dy*dy + dz*dz).sqrt() <= event.radius {
                    affected.push((player_id, event.intensity));
                }
            }
        }
        affected
    }
}

// Recommended integration in HarvestingSystem:
// 
// In your tick_regen or process_harvest method:
//   if let Some(event_manager) = &self.dynamic_event_manager {
//       event_manager.apply_active_surge_effects_to_nodes(&mut self.resource_nodes, 0.5);
//   }
//
// Thunder locked in. Effects ready for HarvestingSystem integration. ⚡❤️🔥
