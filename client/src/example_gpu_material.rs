/*!
 * Full Custom Material + Pipeline with GpuSimulationState
 * 
 * This is a more complete example showing how to create a
 * custom material that also has access to our global GpuSimulationState.
 */

use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::*;
use bevy::pbr::{MeshPipeline, MeshPipelineKey, MeshPipelineViewLayout};
use crate::rbe_client_sync::GpuSimulationStateBuffer;

// ==================== CUSTOM MATERIAL ====================

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
#[uniform(1, CustomMaterialUniform)]
pub struct GpuStateMaterial {
    pub base_color: Color,
}

#[derive(Clone, Default, ShaderType)]
pub struct CustomMaterialUniform {
    pub base_color: [f32; 4],
}

// ==================== PIPELINE ====================

pub struct GpuStateMaterialPipeline {
    pub mesh_pipeline: MeshPipeline,
    pub gpu_state_bind_group_layout: BindGroupLayout,
}

impl FromWorld for GpuStateMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let mesh_pipeline = world.resource::<MeshPipeline>().clone();
        let gpu_buffer = world.resource::<GpuSimulationStateBuffer>();

        Self {
            mesh_pipeline,
            gpu_state_bind_group_layout: gpu_buffer.bind_group_layout.clone(),
        }
    }
}

// Note: Full implementation of SpecializedMeshPipeline + custom
// vertex/fragment shaders is quite involved.
// This file shows the key structure. A complete version would
// implement the full pipeline specialization and shader loading.
//
// If you want the complete 100+ line version with working shaders,
// let me know and I can generate it.