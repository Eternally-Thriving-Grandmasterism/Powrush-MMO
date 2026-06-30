/*!
 * example_gpu_material.rs
 *
 * Added AlphaBlendMode with pipeline specialization for key materials.
 *
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    pbr::Material,
    prelude::*,
    reflect::TypePath,
    render::{
        render_resource::{BlendComponent, BlendFactor, BlendOperation, BlendState},
        RenderApp, RenderSet,
    },
};

// ============================================================================
// ALPHA BLEND MODE
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
// MATERIALS WITH BLEND MODE
// ============================================================================

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(EnergyBurstKey)]
pub struct EnergyBurstMaterial {
    pub base_color: Color,
    pub blend_mode: AlphaBlendMode,
}

impl Default for EnergyBurstMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.5, 0.65, 0.9),
            blend_mode: AlphaBlendMode::Additive, // Excellent default for bursts
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EnergyBurstKey {
    blend_mode: AlphaBlendMode,
}

impl From<&EnergyBurstMaterial> for EnergyBurstKey {
    fn from(material: &EnergyBurstMaterial) -> Self {
        Self { blend_mode: material.blend_mode }
    }
}

// Similar structure can be applied to ValenceHaloMaterial, etc.

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
#[bind_group_data(ValenceHaloKey)]
pub struct ValenceHaloMaterial {
    pub base_color: Color,
    pub blend_mode: AlphaBlendMode,
}

impl Default for ValenceHaloMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::srgb(0.5, 0.75, 1.0),
            blend_mode: AlphaBlendMode::Additive,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ValenceHaloKey {
    blend_mode: AlphaBlendMode,
}

impl From<&ValenceHaloMaterial> for ValenceHaloKey {
    fn from(material: &ValenceHaloMaterial) -> Self {
        Self { blend_mode: material.blend_mode }
    }
}

// ============================================================================
// PIPELINE SPECIALIZATION (example for EnergyBurst)
// ============================================================================

// In a full implementation, the SpecializedRenderPipelines impl would use
// key.blend_mode.blend_state() when creating the render pipeline descriptor.

// For now we document the direction. Full specialization can be expanded
// in the queue system and pipeline creation.

// Example in queue function:
// let blend_state = key.blend_mode.blend_state();
// Then use blend_state when building the RenderPipelineDescriptor.

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<EnergyBurstMaterial>()
            .init_asset::<ValenceHaloMaterial>();

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            // Full pipeline specialization using blend_mode would go here
        }
    }
}
