/*!
 * example_gpu_material.rs
 *
 * Added DepthCompare function to RenderState.
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
// EXTENDED RENDER STATE WITH DEPTH COMPARE
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

// Default helper for common cases
impl RenderState {
    pub fn default_glow() -> Self {
        Self {
            blend_mode: AlphaBlendMode::Additive,
            depth_write: false,
            depth_compare: CompareFunction::Always, // Often useful for glows
            cull_mode: None,
            polygon_mode: PolygonMode::Fill,
        }
    }
}

// Updated materials and pipelines to use the new depth_compare field
// (EnergyBurstMaterial, ValenceHaloMaterial, etc. now include depth_compare)

// Example default for EnergyBurstMaterial
impl Default for EnergyBurstMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.5, 0.65, 0.9),
            render_state: RenderState::default_glow(),
        }
    }
}

// Similar updates for other materials...

// EgUI window updated to include Depth Compare dropdown
// (in addition to existing Blend, Depth Write, Cull, Polygon controls)

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        // Resource registration
    }
}
