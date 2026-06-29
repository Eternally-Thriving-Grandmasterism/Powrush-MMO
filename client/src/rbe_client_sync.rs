/*!
 * STORAGE buffer variant for compute shaders
 */

#[derive(Resource)]
pub struct GpuSimulationStateStorageBuffer {
    pub buffer: Buffer,
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
}

pub fn setup_gpu_simulation_storage_buffer(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
) {
    let buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("GpuSimulationState Storage Buffer"),
        size: std::mem::size_of::<GpuSimulationState>() as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = render_device.create_bind_group_layout(
        &BindGroupLayoutDescriptor {
            label: Some("GpuSimulationState Storage BindGroupLayout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        },
    );

    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: Some("GpuSimulationState Storage BindGroup"),
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

    commands.insert_resource(GpuSimulationStateStorageBuffer {
        buffer,
        bind_group,
        bind_group_layout,
    });
}

/// Dirty-checked upload to STORAGE buffer
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