/*!
 * GpuSimulationState GPU Upload + BindGroup Integration
 */

use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderDevice;

// ==================== GPU BUFFER RESOURCE ====================

#[derive(Resource)]
pub struct GpuSimulationStateBuffer {
    pub buffer: Buffer,
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
}

// ==================== UPLOAD SYSTEM ====================

/// System that uploads Cpu GpuSimulationState to GPU every frame.
pub fn upload_gpu_simulation_state(
    cpu_state: Res<GpuSimulationState>,
    mut gpu_buffer: ResMut<GpuSimulationStateBuffer>,
    render_device: Res<RenderDevice>,
) {
    // Only upload if changed (simple version — can be improved with change detection)
    render_device.queue.write_buffer(
        &gpu_buffer.buffer,
        0,
        bytemuck::cast_slice(std::slice::from_ref(&*cpu_state)),
    );
}

// ==================== INITIALIZATION ====================

pub fn setup_gpu_simulation_buffer(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
) {
    let buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("GpuSimulationState Buffer"),
        size: std::mem::size_of::<GpuSimulationState>() as u64,
        usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = render_device.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
            label: Some("GpuSimulationState BindGroupLayout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        },
    );

    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("GpuSimulationState BindGroup"),
        layout: &bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::Buffer(BufferBinding {
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

// Usage in App:
// app.add_systems(Startup, setup_gpu_simulation_buffer)
//     .add_systems(Update, upload_gpu_simulation_state);