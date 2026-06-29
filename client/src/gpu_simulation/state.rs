/*!
 * Setup and upload functions moved here for full modularity
 */

pub fn setup_gpu_simulation_buffer(
    mut commands: Commands,
    render_device: Res<bevy::render::renderer::RenderDevice>,
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
    render_device: Res<bevy::render::renderer::RenderDevice>,
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
    render_device: Res<bevy::render::renderer::RenderDevice>,
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
    render_device: Res<bevy::render::renderer::RenderDevice>,
) {
    if cpu_state.is_changed() {
        render_device.queue.write_buffer(
            &gpu_buffer.buffer,
            0,
            bytemuck::cast_slice(std::slice::from_ref(&*cpu_state)),
        );
    }
}