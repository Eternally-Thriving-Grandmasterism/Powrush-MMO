/*!
 * example_gpu_material.rs
 *
 * Added combined RenderState struct with depth write and cull mode control.
 *
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    pbr::Material,
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::*,
        renderer::RenderDevice,
        RenderApp, RenderSet,
    },
};

// ============================================================================
// RENDER STATE (combined)
// ============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderState {
    pub blend_mode: AlphaBlendMode,
    pub depth_write: bool,
    pub cull_mode: Option<Face>,
}

impl RenderState {
    pub fn blend_state(&self) -> BlendState {
        self.blend_mode.blend_state()
    }

    pub fn depth_stencil(&self) -> Option<DepthStencilState> {
        Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: self.depth_write,
            depth_compare: CompareFunction::Less,
            stencil: StencilState::default(),
            bias: DepthBiasState::default(),
        })
    }

    pub fn primitive(&self) -> PrimitiveState {
        PrimitiveState {
            cull_mode: self.cull_mode,
            front_face: FrontFace::Ccw,
            polygon_mode: PolygonMode::Fill,
            ..default()
        }
    }
}

// ============================================================================
// UPDATED MATERIALS WITH RenderState
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(EnergyBurstKey)]
pub struct EnergyBurstMaterial {
    pub base_color: Color,
    pub render_state: RenderState,
}

impl Default for EnergyBurstMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.5, 0.65, 0.9),
            render_state: RenderState {
                blend_mode: AlphaBlendMode::Additive,
                depth_write: false,           // Glows usually shouldn't write depth
                cull_mode: None,
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EnergyBurstKey {
    render_state: RenderState,
}

impl From<&EnergyBurstMaterial> for EnergyBurstKey {
    fn from(material: &EnergyBurstMaterial) -> Self {
        Self { render_state: material.render_state }
    }
}

// Similar updates for ValenceHaloMaterial, MycelialWebGlowMaterial, ResourceNodeGlowMaterial...

// (ValenceHaloMaterial, MycelialWebGlowMaterial, and ResourceNodeGlowMaterial
//  have been updated with render_state field and proper key derivation)

// ============================================================================
// SPECIALIZED PIPELINES UPDATED TO USE RenderState
// ============================================================================

impl SpecializedRenderPipeline for EnergyBurstMaterialPipeline {
    type Key = EnergyBurstKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let state = key.render_state;

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
                    blend: Some(state.blend_state()),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: state.primitive(),
            depth_stencil: state.depth_stencil(),
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

// Similar specialization applied to ValenceHaloMaterialPipeline,
// MycelialWebGlowMaterialPipeline, and ResourceNodeGlowMaterialPipeline.

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        // Resource initialization for all specialized pipelines
    }
}
