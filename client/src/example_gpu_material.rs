/*!
 * example_gpu_material.rs
 *
 * Full RenderState-driven visual materials + live egui tuning + optimized GPU compute dispatch with barriers.
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    log::debug,
    pbr::{Material, MeshMaterial3d},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::*,
        renderer::RenderDevice,
        RenderApp, RenderSet,
    },
};
use bevy_egui::EguiContexts;
use tracing::instrument;

// ... [previous code remains] ...

// ============================================================================
// GPU COMPUTE — Optimized Dispatch with Barriers
// ============================================================================

#[derive(Resource)]
pub struct VisualComputeOutput {
    pub buffer: Buffer,
}

impl FromWorld for VisualComputeOutput {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("visual_compute_output".into()),
            size: 1024 * 4,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        Self { buffer }
    }
}

#[derive(Resource)]
pub struct VisualComputeBindGroup {
    pub bind_group: BindGroup,
}

pub fn prepare_visual_compute_bind_group(
    mut commands: Commands,
    pipeline: Res<VisualComputePipeline>,
    render_device: Res<RenderDevice>,
    // TODO: Replace with actual simulation state buffer resource when available
    simulation_state_buffer: Option<Res<crate::gpu::GpuSimulationStateBuffer>>,
    output: Res<VisualComputeOutput>,
) {
    let Some(sim_buffer) = simulation_state_buffer else { return; };

    let bind_group = render_device.create_bind_group(
        "visual_compute_bind_group",
        &pipeline.bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: sim_buffer.buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: output.buffer.as_entire_binding(),
            },
        ],
    );

    commands.insert_resource(VisualComputeBindGroup { bind_group });
}

pub fn dispatch_visual_compute(
    pipeline: Res<VisualComputePipeline>,
    bind_group: Option<Res<VisualComputeBindGroup>>,
    pipeline_cache: Res<PipelineCache>,
    mut commands: Commands,
) {
    let Some(bind_group) = bind_group else { return; };
    let Some(compute_pipeline) = pipeline_cache.get_compute_pipeline(pipeline.pipeline) else { return; };

    commands.add(move |world: &mut World| {
        let render_context = world.resource::<RenderContext>();
        let mut encoder = render_context.command_encoder();

        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("visual_compute_pass".into()),
            });

            pass.set_pipeline(compute_pipeline);
            pass.set_bind_group(0, &bind_group.bind_group, &[]);
            pass.dispatch_workgroups(16, 1, 1);
        }

        // Optimize: Insert memory barrier so subsequent render passes see the written data
        encoder.insert_barrier(BarrierType::StorageBuffer);
    });

    debug!("[VisualCompute] Optimized dispatch with barrier executed.");
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<EnergyBurstMaterial>()
            .init_asset::<ValenceHaloMaterial>()
            .init_asset::<MycelialWebGlowMaterial>()
            .init_asset::<ResourceNodeGlowMaterial>()
            .init_resource::<GpuVisualMaterialSettings>()
            .init_resource::<VisualComputeOutput>();

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<EnergyBurstMaterialPipeline>()
                .init_resource::<ValenceHaloMaterialPipeline>()
                .init_resource::<MycelialWebGlowMaterialPipeline>()
                .init_resource::<ResourceNodeGlowMaterialPipeline>()
                .init_resource::<VisualComputePipeline>()
                .init_resource::<SpecializedRenderPipelines<EnergyBurstMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<ValenceHaloMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<MycelialWebGlowMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<ResourceNodeGlowMaterialPipeline>>()
                .add_systems(
                    Render,
                    (
                        prepare_visual_compute_bind_group,
                        dispatch_visual_compute,
                    )
                        .chain()  // Ensure prepare runs before dispatch
                        .in_set(RenderSet::Queue),
                );
        }

        app.add_systems(Update, tune_gpu_visual_materials);
    }
}
