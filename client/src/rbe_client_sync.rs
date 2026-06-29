/*!
 * Dirty-checked GPU upload for GpuSimulationState
 */

/// Only uploads to GPU when the CPU state actually changed.
/// This is much more efficient than uploading every frame.
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

// The upload system is now dirty-checked and will only write to GPU when needed.