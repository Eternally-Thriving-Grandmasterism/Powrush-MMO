/*!
 * Example GpuSimulationStateMaterial
 * 
 * Demonstrates how to create a Bevy material that reads from GpuSimulationState.
 * This is a starting point you can extend.
 */

use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use crate::rbe_client_sync::GpuSimulationStateBuffer;

/// Example material that has access to GpuSimulationState
#[derive(Asset, TypePath, AsBindGroup, Clone, Debug, Default)]
#[uniform(0, GpuSimulationStateUniform)]
pub struct GpuSimulationStateMaterial {
    // You can add your own material properties here
    pub base_color: Color,
}

#[derive(Clone, Default, ShaderType)]
pub struct GpuSimulationStateUniform {
    pub base_color: [f32; 4],
}

impl Material for GpuSimulationStateMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/example_gpu_state.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

// Note:
// To properly inject our custom GpuSimulationStateBuffer bind group,
// you will usually need to use a custom render pipeline or
// ExtendedMaterial + a custom MaterialPlugin.
//
// For a quick start, you can manually set the bind group in a
// custom render pass or use bevy's render graph extensions.
//
// This file gives you the foundation. Let me know if you want
// the full custom pipeline version.