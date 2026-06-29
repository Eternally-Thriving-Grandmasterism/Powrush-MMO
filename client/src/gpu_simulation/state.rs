/*!
 * Core GpuSimulationState definition and plugin
 */

use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct HotbarSlot {
    pub count: u32,
    pub cooldown_remaining: f32,
    pub _padding: [u32; 2],
}

#[repr(C)]
#[derive(Resource, Clone, Debug)]
pub struct GpuSimulationState {
    pub hotbar: [HotbarSlot; 8],
    pub node_confidences: [f32; 8],
    pub global_mercy_resonance: f32,
    pub global_confidence: f32,
    pub player_position: [f32; 3],
    pub time: f32,
    pub delta_time: f32,
    pub council_valence: f32,
    pub active_council_action: u32,
    pub council_participants: u32,
    pub rbe_flow_rate: f32,
    pub total_rbe_circulating: f32,
    pub player_rbe_balance: f32,
    pub player_velocity: [f32; 3],
    pub player_mercy_attunement: f32,
    pub player_thrivability: f32,
    pub _padding: [u32; 1],
}

impl Default for GpuSimulationState {
    fn default() -> Self {
        Self {
            hotbar: [HotbarSlot { count: 0, cooldown_remaining: 0.0, _padding: [0; 2] }; 8],
            node_confidences: [0.0; 8],
            global_mercy_resonance: 0.0,
            global_confidence: 0.0,
            player_position: [0.0; 3],
            time: 0.0,
            delta_time: 0.0,
            council_valence: 0.0,
            active_council_action: 0,
            council_participants: 0,
            rbe_flow_rate: 0.0,
            total_rbe_circulating: 0.0,
            player_rbe_balance: 0.0,
            player_velocity: [0.0; 3],
            player_mercy_attunement: 0.0,
            player_thrivability: 0.0,
            _padding: [0; 1],
        }
    }
}

#[derive(Resource)]
pub struct GpuSimulationStateBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

#[derive(Resource)]
pub struct GpuSimulationStateStorageBuffer {
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

pub struct GpuSimulationStatePlugin;

impl Plugin for GpuSimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GpuSimulationState>()
            .add_systems(Startup, (
                crate::rbe_client_sync::setup_gpu_simulation_buffer,
                crate::rbe_client_sync::setup_gpu_simulation_storage_buffer,
            ))
            .add_systems(Update, (
                crate::rbe_client_sync::upload_gpu_simulation_state,
                crate::rbe_client_sync::upload_gpu_simulation_state_storage,
            ));
    }
}