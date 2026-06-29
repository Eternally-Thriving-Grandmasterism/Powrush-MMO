/*!
 * Exposing BindGroup for materials and custom render passes
 */

impl GpuSimulationStateBuffer {
    /// Returns a reference to the BindGroup for use in materials or render passes.
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    /// Returns the BindGroupLayout (useful when creating custom pipelines).
    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

impl GpuSimulationStateStorageBuffer {
    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}

// Usage example in a custom material or render pass:
// let bind_group = gpu_sim_buffer.bind_group();
// render_pass.set_bind_group(0, bind_group, &[]);