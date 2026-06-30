/*!
 * example_gpu_material.rs
 *
 * Extended AlphaBlendMode support to more materials.
 *
 * AG-SML v1.0
 */

// ... (previous AlphaBlendMode, EnergyBurst, ValenceHalo code remains) ...

// ============================================================================
// MYCELIAL WEB GLOW MATERIAL + PIPELINE
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(MycelialWebGlowKey)]
pub struct MycelialWebGlowMaterial {
    pub base_color: Color,
    pub blend_mode: AlphaBlendMode,
}

impl Default for MycelialWebGlowMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.4, 0.55, 0.4),
            blend_mode: AlphaBlendMode::Alpha, // Web looks good with standard alpha
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MycelialWebGlowKey {
    blend_mode: AlphaBlendMode,
}

impl From<&MycelialWebGlowMaterial> for MycelialWebGlowKey {
    fn from(material: &MycelialWebGlowMaterial) -> Self {
        Self { blend_mode: material.blend_mode }
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
    type Key = MycelialWebGlowKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let blend = key.blend_mode.blend_state();

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
                    blend: Some(blend),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

// ============================================================================
// RESOURCE NODE GLOW MATERIAL + PIPELINE
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(ResourceNodeGlowKey)]
pub struct ResourceNodeGlowMaterial {
    pub base_color: Color,
    pub blend_mode: AlphaBlendMode,
}

impl Default for ResourceNodeGlowMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.65, 0.48, 0.28),
            blend_mode: AlphaBlendMode::Alpha,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ResourceNodeGlowKey {
    blend_mode: AlphaBlendMode,
}

impl From<&ResourceNodeGlowMaterial> for ResourceNodeGlowKey {
    fn from(material: &ResourceNodeGlowMaterial) -> Self {
        Self { blend_mode: material.blend_mode }
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
    type Key = ResourceNodeGlowKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let blend = key.blend_mode.blend_state();

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
                    blend: Some(blend),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

// ============================================================================
// PLUGIN UPDATE
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<MycelialWebGlowMaterial>()
            .init_asset::<ResourceNodeGlowMaterial>();

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<MycelialWebGlowMaterialPipeline>()
                .init_resource::<ResourceNodeGlowMaterialPipeline>()
                .init_resource::<SpecializedRenderPipelines<MycelialWebGlowMaterialPipeline>>()
                .init_resource::<SpecializedRenderPipelines<ResourceNodeGlowMaterialPipeline>>();
        }
    }
}
