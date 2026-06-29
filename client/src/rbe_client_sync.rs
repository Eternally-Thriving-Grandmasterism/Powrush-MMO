/*!
 * Expanded GpuSimulationState with high-value game data
 */

#[repr(C)]
#[derive(Resource, Clone, Debug)]
pub struct GpuSimulationState {
    pub hotbar: [HotbarSlot; 8],
    pub node_confidences: [f32; 8],

    // === New useful fields ===
    pub global_mercy_resonance: f32,
    pub global_confidence: f32,
    pub player_position: [f32; 3],
    pub time: f32,
    pub delta_time: f32,

    pub _padding: [u32; 3], // keep alignment clean
}

impl Default for GpuSimulationState {
    fn default() -> Self {
        Self {
            hotbar: [HotbarSlot::default(); 8],
            node_confidences: [0.0; 8],
            global_mercy_resonance: 0.0,
            global_confidence: 0.0,
            player_position: [0.0; 3],
            time: 0.0,
            delta_time: 0.0,
            _padding: [0; 3],
        }
    }
}