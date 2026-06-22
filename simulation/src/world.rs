/*!
 * Added zone_node_cache field to SovereignWorldState.
 */

#[derive(Clone, Debug, Default)]
pub struct SovereignWorldState {
    // ... existing fields ...

    pub zone_node_cache: HashMap<u64, Vec<NodeId>>,  // NEW: reverse cache zone -> nodes
}

// In new_from_scenario, after generating zones and nodes:
// world.rebuild_zone_node_cache();

// Also call it in tick() occasionally or when zones change.
