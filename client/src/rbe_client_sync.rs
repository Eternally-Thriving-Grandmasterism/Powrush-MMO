/*!
 * Optimized GPU Memory Layout for GpuSimulationState
 * 
 * - #[repr(C)] for predictable layout
 * - bytemuck::Pod + Zeroable for safe wgpu buffer casting
 * - Fixed-size arrays for GPU friendliness
 * - Helper to create uploadable GPU buffer
 */

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

// ==================== GPU-OPTIMIZED STRUCTS ====================

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct HotbarSlot {
    pub count: u32,
    pub cooldown_remaining: f32,
    pub _padding: [u32; 2], // 16-byte alignment
}

#[repr(C)]
#[derive(Resource, Clone, Debug)]
pub struct GpuSimulationState {
    pub hotbar: [HotbarSlot; 8],
    pub node_confidences: [f32; 8],
}

impl Default for GpuSimulationState {
    fn default() -> Self {
        Self {
            hotbar: [HotbarSlot {
                count: 0,
                cooldown_remaining: 0.0,
                _padding: [0; 2],
            }; 8],
            node_confidences: [0.0; 8],
        }
    }
}

// ==================== GPU BUFFER HELPER ====================

/// Creates a wgpu buffer from GpuSimulationState ready for upload.
/// Usage: UNIFORM or STORAGE buffer.
pub fn create_gpu_simulation_buffer(
    device: &wgpu::Device,
    state: &GpuSimulationState,
) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("GpuSimulationState Buffer"),
        contents: bytemuck::cast_slice(std::slice::from_ref(state)),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

// Note: You can also create a STORAGE buffer variant if using it in compute shaders.