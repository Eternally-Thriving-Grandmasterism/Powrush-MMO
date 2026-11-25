/*!
 * gpu_simulation::state
 *
 * Full self-contained GpuSimulationState, buffer resources, setup/upload systems,
 * and Plugin. Recovered and polished from recent modular refactor diffs.
 * Ensures GPU uniform/storage sync for Council, RBE, Player, Mercy, time, etc.
 * AG-SML v1.0
 */

use bevy::prelude::*;
use bevy::render::renderer::RenderDevice;
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
#[derive(Resource, Clone, Debug, Pod, Zeroable)]
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

pub fn setup_gpu_simulation_buffer(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
) {
    let buffer = render_device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("GpuSimulationState Buffer"),
        size: std::mem::size_of::<GpuSimulationState>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = render_device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("GpuSimulationState BindGroupLayout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        },
    );

    let bind_group = render_device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("GpuSimulationState BindGroup"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: None,
            }),
        }],
    });

    commands.insert_resource(GpuSimulationStateBuffer {
        buffer,
        bind_group,
        bind_group_layout,
    });
}

pub fn setup_gpu_simulation_storage_buffer(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
) {
    let buffer = render_device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("GpuSimulationState Storage Buffer"),
        size: std::mem::size_of::<GpuSimulationState>() as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = render_device.create_bind_group_layout(
        &wgpu::BindGroupLayoutDescriptor {
            label: Some("GpuSimulationState Storage BindGroupLayout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        },
    );

    let bind_group = render_device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("GpuSimulationState Storage BindGroup"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &buffer,
                offset: 0,
                size: None,
            }),
        }],
    });

    commands.insert_resource(GpuSimulationStateStorageBuffer {
        buffer,
        bind_group,
        bind_group_layout,
    });
}

pub fn upload_gpu_simulation_state(
    cpu_state: Res<GpuSimulationState>,
    mut gpu_buffer: ResMut<GpuSimulationStateBuffer>,
    render_device: Res<RenderDevice>,
) {
    if cpu_state.is_changed() {
        render_device.queue.write_buffer(
            &gpu_buffer.buffer,
            0,
            bytemuck::cast_slice(std::slice::from_ref(&*cpu_state)),
        );
    }
}

pub fn upload_gpu_simulation_state_storage(
    cpu_state: Res<GpuSimulationState>,
    mut gpu_buffer: ResMut<GpuSimulationStateStorageBuffer>,
    render_device: Res<RenderDevice>,
) {
    if cpu_state.is_changed() {
        render_device.queue.write_buffer(
            &gpu_buffer.buffer,
            0,
            bytemuck::cast_slice(std::slice::from_ref(&*cpu_state)),
        );
    }
}

pub struct GpuSimulationStatePlugin;

impl Plugin for GpuSimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GpuSimulationState>()
            .add_systems(Startup, (
                setup_gpu_simulation_buffer,
                setup_gpu_simulation_storage_buffer,
            ))
            .add_systems(Update, (
                upload_gpu_simulation_state,
                upload_gpu_simulation_state_storage,
            ));
    }
}
