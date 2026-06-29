/*!
 * client/src/rbe_client_sync.rs
 *
 * Added GpuSimulationState definition for Hotbar + PATSAGi Node data.
 */

// ==================== GPU SIMULATION STATE ====================

#[derive(Resource, Default, Clone, Debug)]
pub struct GpuSimulationState {
    /// Hotbar slot data (index 0 = first hotbar slot)
    pub hotbar: Vec<HotbarSlot>,

    /// PATSAGi node confidences
    /// Index 0 = Node 01, Index 1 = Node 02, etc.
    pub node_confidences: Vec<f32>,

    // Add other simulation state fields here as needed
}

#[derive(Clone, Debug, Default)]
pub struct HotbarSlot {
    pub count: u32,
    pub cooldown_remaining: f32,
    // Future: item_id, durability, rarity, etc.
}

// Note: Make sure GpuSimulationState is initialized in your App:
// app.init_resource::<GpuSimulationState>();