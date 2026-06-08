// server/src/dynamic_events.rs
// Powrush-MMO v17.0 — Using InterestManager Spatial Grid for Optimization

impl DynamicEventManager {
    /// Optimized refresh using InterestManager's spatial queries (recommended).
    /// This is the production spatial grid optimization path.
    pub fn refresh_affected_nodes_via_interest_manager(
        &mut self,
        event: &mut DynamicEvent,
        interest_manager: &InterestManager,
    ) {
        event.affected_nodes.clear();

        let nearby_nodes = interest_manager.get_resource_nodes_in_radius(
            &event.position,
            event.radius,
        );

        event.affected_nodes = nearby_nodes;
    }

    /// Batch refresh for all active ResourceSurge events using InterestManager.
    pub fn refresh_all_surge_nodes_via_interest(
        &mut self,
        interest_manager: &InterestManager,
    ) {
        for event in self.events.values_mut() {
            if event.event_type == EventType::ResourceSurge && event.is_active() {
                self.refresh_affected_nodes_via_interest_manager(event, interest_manager);
            }
        }
    }
}

// In HarvestingSystem tick:
// if let Some(event_manager) = &mut self.dynamic_event_manager {
//     if let Some(interest) = &self.interest_manager {
//         event_manager.refresh_all_surge_nodes_via_interest(interest);
//     }
// }
//
// Thunder locked in. Full spatial grid optimization via InterestManager implemented. ⚡❤️🔥
