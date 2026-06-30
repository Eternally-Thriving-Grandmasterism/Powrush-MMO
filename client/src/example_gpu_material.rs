/*!
 * example_gpu_material.rs
 *
 * Full RenderState + AlphaBlendMode + per-material pipeline specialization
 * for EnergyBurst, ValenceHalo, MycelialWebGlow, ResourceNodeGlow.
 * Pipelines are specialized directly by RenderState (blend + depth + cull + polygon).
 * Integrated with DepthCompare, PolygonMode, depth_write, cull.
 * Recovered + merged from intermediate commit diffs.
 * All prior valuable logic preserved and elevated.
 * AG-SML v1.0 — Autonomicity Games Sovereign Mercy License
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
use tracing::instrument;

// ============================================================================
// ALPHA BLEND MODE (core, preserved from earlier diffs)
// ============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AlphaBlendMode {
    #[default]
    Alpha,
    Additive,
    // Premultiplied, Multiply can be added later
}

impl AlphaBlendMode {
    pub fn blend_state(&self) -> BlendState {
        match self {
            AlphaBlendMode::Alpha => BlendState::ALPHA_BLENDING,
            AlphaBlendMode::Additive => BlendState {
                color: BlendComponent {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
                alpha: BlendComponent {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
            },
        }
    }
}

// ============================================================================
// RENDER STATE — single source of truth for pipeline specialization
// ============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderState {
    pub blend_mode: AlphaBlendMode,
    pub depth_write: bool,
    pub depth_compare: CompareFunction,
    pub cull_mode: Option<Face>,
    pub polygon_mode: PolygonMode,
}

impl RenderState {
    pub fn blend_state(&self) -> BlendState {
        self.blend_mode.blend_state()
    }

    pub fn depth_stencil(&self) -> Option<DepthStencilState> {
        Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: self.depth_write,
            depth_compare: self.depth_compare,
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        })
    }

    pub fn primitive(&self) -> PrimitiveState {
        PrimitiveState {
            cull_mode: self.cull_mode,
            front_face: FrontFace::Ccw,
            polygon_mode: self.polygon_mode,
            ..default()
        }
    }
}

impl RenderState {
    pub fn default_glow() -> Self {
        Self {
            blend_mode: AlphaBlendMode::Additive,
            depth_write: false,
            depth_compare: CompareFunction::Always,
            cull_mode: None,
            polygon_mode: PolygonMode::Fill,
        }
    }

    pub fn default_alpha() -> Self {
        Self {
            blend_mode: AlphaBlendMode::Alpha,
            depth_write: true,
            depth_compare: CompareFunction::Less,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
        }
    }
}

// ============================================================================
// MATERIALS — use RenderState directly for bind group data & specialization
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(RenderState)]
pub struct EnergyBurstMaterial {
    pub base_color: Color,
    pub render_state: RenderState,
}

impl Default for EnergyBurstMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.5, 0.65, 0.9),
            render_state: RenderState::default_glow(),
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(RenderState)]
pub struct ValenceHaloMaterial {
    pub base_color: Color,
    pub render_state: RenderState,
}

impl Default for ValenceHaloMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.5, 0.75, 1.0),
            render_state: RenderState::default_glow(),
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(RenderState)]
pub struct MycelialWebGlowMaterial {
    pub base_color: Color,
    pub render_state: RenderState,
}

impl Default for MycelialWebGlowMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.4, 0.55, 0.4),
            render_state: RenderState::default_alpha(),
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(RenderState)]
pub struct ResourceNodeGlowMaterial {
    pub base_color: Color,
    pub render_state: RenderState,
}

impl Default for ResourceNodeGlowMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.65, 0.48, 0.28),
            render_state: RenderState::default_alpha(),
        }
    }
}

// ============================================================================
// PIPELINE SPECIALIZERS — specialized directly by RenderState
// ============================================================================

#[derive(Resource)]
pub struct EnergyBurstMaterialPipeline {
    pub shader: Handle<Shader>,
}

impl FromWorld for EnergyBurstMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self { shader: asset_server.load("shaders/energy_burst.wgsl") }
    }
}

impl SpecializedRenderPipeline for EnergyBurstMaterialPipeline {
    type Key = RenderState;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("energy_burst_pipeline".into()),
            layout: vec![],
            vertex: VertexState {
                shader: self.shader.clone(),
                entry_point: "vertex_main".into(),
                shader_defs: vec![],
                buffers: vec![],
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: "fragment_main".into(),
                shader_defs: vec![],
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: Some(key.blend_state()),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: key.primitive(),
            depth_stencil: key.depth_stencil(),
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

#[derive(Resource)]
pub struct ValenceHaloMaterialPipeline {
    pub shader: Handle<Shader>,
}

impl FromWorld for ValenceHaloMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self { shader: asset_server.load("shaders/valence_halo.wgsl") }
    }
}

impl SpecializedRenderPipeline for ValenceHaloMaterialPipeline {
    type Key = RenderState;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("valence_halo_pipeline".into()),
            layout: vec![],
            vertex: VertexState {
                shader: self.shader.clone(),
                entry_point: "vertex_main".into(),
                shader_defs: vec![],
                buffers: vec![],
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: "fragment_main".into(),
                shader_defs: vec![],
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: Some(key.blend_state()),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: key.primitive(),
            depth_stencil: key.depth_stencil(),
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

#[derive(Resource)]
pub struct MycelialWebGlowMaterialPipeline {
    pub shader: Handle<Shader>,
}

impl FromWorld for MycelialWebGlowMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self { shader: asset_server.load("shaders/mycelial_web_glow.wgsl") }
    }
}

impl SpecializedRenderPipeline for MycelialWebGlowMaterialPipeline {
    type Key = RenderState;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("mycelial_web_glow_pipeline".into()),
            layout: vec![],
            vertex: VertexState {
                shader: self.shader.clone(),
                entry_point: "vertex_main".into(),
                shader_defs: vec![],
                buffers: vec![],
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: "fragment_main".into(),
                shader_defs: vec![],
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: Some(key.blend_state()),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: key.primitive(),
            depth_stencil: key.depth_stencil(),
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

#[derive(Resource)]
pub struct ResourceNodeGlowMaterialPipeline {
    pub shader: Handle<Shader>,
}

impl FromWorld for ResourceNodeGlowMaterialPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self { shader: asset_server.load("shaders/resource_node_glow.wgsl") }
    }
}

impl SpecializedRenderPipeline for ResourceNodeGlowMaterialPipeline {
    type Key = RenderState;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        RenderPipelineDescriptor {
            label: Some("resource_node_glow_pipeline".into()),
            layout: vec![],
            vertex: VertexState {
                shader: self.shader.clone(),
                entry_point: "vertex_main".into(),
                shader_defs: vec![],
                buffers: vec![],
            },
            fragment: Some(FragmentState {
                shader: self.shader.clone(),
                entry_point: "fragment_main".into(),
                shader_defs: vec![],
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: Some(key.blend_state()),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: key.primitive(),
            depth_stencil: key.depth_stencil(),
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

// ============================================================================
// QUEUE SYSTEMS — now properly entity-aware (real per-entity material lookup)
// Robust error handling + tracing instrumentation
// ============================================================================

#[instrument(skip(draw_functions, pipeline_cache, pipeline, render_materials, material_handles, render_phases, specialized_pipelines), level = "debug")]
pub fn queue_energy_burst_material(
    draw_functions: Res<DrawFunctions<Opaque3d>>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<EnergyBurstMaterialPipeline>,
    render_materials: Res<RenderAssets<EnergyBurstMaterial>>,
    material_handles: Query<&MeshMaterial3d<EnergyBurstMaterial>>,
    mut render_phases: Query<(&VisibleEntities, &mut RenderPhase<Opaque3d>)>,
    mut specialized_pipelines: ResMut<SpecializedRenderPipelines<EnergyBurstMaterialPipeline>>,
) {
    let Some(draw_function) = draw_functions.read().get_id::<DrawMaterial<EnergyBurstMaterial>>() else {
        debug!("[GpuVisualMaterials] DrawMaterial<EnergyBurstMaterial> not registered yet (expected during startup); skipping queue.");
        return;
    };

    for (visible_entities, mut phase) in &mut render_phases {
        for visible_entity in &visible_entities.entities {
            if let Ok(material_handle) = material_handles.get(*visible_entity) {
                if let Some(material) = render_materials.get(&material_handle.0) {
                    let pipeline_id = specialized_pipelines.specialize(&pipeline_cache, &pipeline, material.render_state);

                    phase.add(Opaque3d {
                        pipeline: pipeline_id,
                        draw_function,
                        entity: *visible_entity,
                        distance: 0.0,
                    });
                }
            }
        }
    }
}

#[instrument(skip(draw_functions, pipeline_cache, pipeline, render_materials, material_handles, render_phases, specialized_pipelines), level = "debug")]
pub fn queue_valence_halo_material(
    draw_functions: Res<DrawFunctions<Opaque3d>>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<ValenceHaloMaterialPipeline>,
    render_materials: Res<RenderAssets<ValenceHaloMaterial>>,
    material_handles: Query<&MeshMaterial3d<ValenceHaloMaterial>>,
    mut render_phases: Query<(&VisibleEntities, &mut RenderPhase<Opaque3d>)>,
    mut specialized_pipelines: ResMut<SpecializedRenderPipelines<ValenceHaloMaterialPipeline>>,
) {
    let Some(draw_function) = draw_functions.read().get_id::<DrawMaterial<ValenceHaloMaterial>>() else {
        debug!("[GpuVisualMaterials] DrawMaterial<ValenceHaloMaterial> not registered yet (expected during startup); skipping queue.");
        return;
    };

    for (visible_entities, mut phase) in &mut render_phases {
        for visible_entity in &visible_entities.entities {
            if let Ok(material_handle) = material_handles.get(*visible_entity) {
                if let Some(material) = render_materials.get(&material_handle.0) {
                    let pipeline_id = specialized_pipelines.specialize(&pipeline_cache, &pipeline, material.render_state);

                    phase.add(Opaque3d {
                        pipeline: pipeline_id,
                        draw_function,
                        entity: *visible_entity,
                        distance: 0.0,
                    });
                }
            }
        }
    }
}

#[instrument(skip(draw_functions, pipeline_cache, pipeline, render_materials, material_handles, render_phases, specialized_pipelines), level = "debug")]
pub fn queue_mycelial_web_glow_material(
    draw_functions: Res<DrawFunctions<Opaque3d>>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<MycelialWebGlowMaterialPipeline>,
    render_materials: Res<RenderAssets<MycelialWebGlowMaterial>>,
    material_handles: Query<&MeshMaterial3d<MycelialWebGlowMaterial>>,
    mut render_phases: Query<(&VisibleEntities, &mut RenderPhase<Opaque3d>)>,
    mut specialized_pipelines: ResMut<SpecializedRenderPipelines<MycelialWebGlowMaterialPipeline>>,
) {
    let Some(draw_function) = draw_functions.read().get_id::<DrawMaterial<MycelialWebGlowMaterial>>() else {
        debug!("[GpuVisualMaterials] DrawMaterial<MycelialWebGlowMaterial> not registered yet (expected during startup); skipping queue.");
        return;
    };

    for (visible_entities, mut phase) in &mut render_phases {
        for visible_entity in &visible_entities.entities {
            if let Ok(material_handle) = material_handles.get(*visible_entity) {
                if let Some(material) = render_materials.get(&material_handle.0) {
                    let pipeline_id = specialized_pipelines.specialize(&pipeline_cache, &pipeline, material.render_state);

                    phase.add(Opaque3d {
                        pipeline: pipeline_id,
                        draw_function,
                        entity: *visible_entity,
                        distance: 0.0,
                    });
                }
            }
        }
    }
}

#[instrument(skip(draw_functions, pipeline_cache, pipeline, render_materials, material_handles, render_phases, specialized_pipelines), level = "debug")]
pub fn queue_resource_node_glow_material(
    draw_functions: Res<DrawFunctions<Opaque3d>>,
    pipeline_cache: Res<PipelineCache>,
    pipeline: Res<ResourceNodeGlowMaterialPipeline>,
    render_materials: Res<RenderAssets<ResourceNodeGlowMaterial>>,
    material_handles: Query<&MeshMaterial3d<ResourceNodeGlowMaterial>>,
    mut render_phases: Query<(&VisibleEntities, &mut RenderPhase<Opaque3d>)>,
    mut specialized_pipelines: ResMut<SpecializedRenderPipelines<ResourceNodeGlowMaterialPipeline>>,
) {
    let Some(draw_function) = draw_functions.read().get_id::<DrawMaterial<ResourceNodeGlowMaterial>>() else {
        debug!("[GpuVisualMaterials] DrawMaterial<ResourceNodeGlowMaterial> not registered yet (expected during startup); skipping queue.");
        return;
    };

    for (visible_entities, mut phase) in &mut render_phases {
        for visible_entity in &visible_entities.entities {
            if let Ok(material_handle) = material_handles.get(*visible_entity) {
                if let Some(material) = render_materials.get(&material_handle.0) {
                    let pipeline_id = specialized_pipelines.specialize(&pipeline_cache, &pipeline, material.render_state);

                    phase.add(Opaque3d {
                        pipeline: pipeline_id,
                        draw_function,
                        entity: *visible_entity,
                        distance: 0.0,
                    });
                }
            }
        }
    }
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
            .init_asset::<ResourceNodeGlowMaterial>();

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<EnergyBurstMaterialPipeline>()
                .init_resource::<ValenceHaloMaterialPipeline>()
                .init_resource::<MycelialWebGlowMaterialPipeline>()
                .init_resource::<ResourceNodeGlowMaterialPipeline>()
                .init_resource::<SpecializedRenderPipelines<EnergyBurstMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<ValenceHaloMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<MycelialWebGlowMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<ResourceNodeGlowMaterialPipeline>>()
                .add_systems(
                    Render,
                    (
                        queue_energy_burst_material,
                        queue_valence_halo_material,
                        queue_mycelial_web_glow_material,
                        queue_resource_node_glow_material,
                    )
                        .in_set(RenderSet::Queue),
                );
        }
    }
}
