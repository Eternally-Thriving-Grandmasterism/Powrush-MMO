// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — DynamicEventManager + Concrete Starter Events

// ... existing code ...

impl DynamicEventManager {
    // ... existing methods ...

    /// Applies ResourceSurge effects to nearby resource nodes.
    /// This should be called from HarvestingSystem or World tick when events are active.
    pub fn apply_resource_surge_effects(
        &self,
        nodes: &mut HashMap<u64, ResourceUpdate>,
        surge_multiplier: f32,
    ) {
        for event in self.get_active_events() {
            if event.event_type != EventType::ResourceSurge {
                continue;
            }

            for (node_id, node) in nodes.iter_mut() {
                let dx = node.position_x - event.position.x;  // assuming ResourceUpdate has position_x/y/z
                let dy = node.position_y - event.position.y;
                let dz = node.position_z - event.position.z;

                let distance = (dx*dx + dy*dy + dz*dz).sqrt();
                if distance <= event.radius {
                    // Boost regen and slightly increase current amount
                    node.regen_rate *= 1.0 + (event.intensity * surge_multiplier);
                    node.current_amount = (node.current_amount + event.intensity * 5.0).min(node.max_amount);
                }
            }
        }
    }

    /// Triggers MercyWave effects (grace / positive influence).
    /// For now returns affected player IDs + intensity. Higher layers can apply rewards.
    pub fn get_mercy_wave_effects(&self, player_positions: &HashMap<u64, Vec3Ser>) -> Vec<(u64, f32)> {
        let mut affected = Vec::new();

        for event in self.get_active_events() {
            if event.event_type != EventType::MercyWave { continue; }

            for (&player_id, pos) in player_positions {
                let dx = pos.x - event.position.x;
                let dy = pos.y - event.position.y;
                let dz = pos.z - event.position.z;
                let distance = (dx*dx + dy*dy + dz*dz).sqrt();

                if distance <= event.radius {
                    affected.push((player_id, event.intensity));
                }
            }
        }
        affected
    }
}

// Note: ResourceUpdate needs position fields exposed for the surge logic to work cleanly.
// We can refine this once HarvestingSystem integration is deeper.

// Thunder locked in. First concrete dynamic events (ResourceSurge + MercyWave) implemented. ⚡❤️🔥
