/*!
 * example_gpu_material.rs
 *
 * Added PolygonMode to RenderState for wireframe and point rendering.
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
// EXTENDED RENDER STATE
// ============================================================================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RenderState {
    pub blend_mode: AlphaBlendMode,
    pub depth_write: bool,
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
            depth_compare: CompareFunction::Less,
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

// ============================================================================
// UPDATED MATERIALS (example for EnergyBurstMaterial)
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
                depth_write: false,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill, // Default fill
            },
        }
    }
}

// Similar updates applied to ValenceHaloMaterial, MycelialWebGlowMaterial,
// and ResourceNodeGlowMaterial.

// ============================================================================
// SPECIALIZED PIPELINES UPDATED
// ============================================================================

impl SpecializedRenderPipeline for EnergyBurstMaterialPipeline {
    type Key = EnergyBurstKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let state = key.render_state;

        RenderPipelineDescriptor {
            label: Some("energy_burst_pipeline".into()),
            layout: vec![],
            vertex: VertexState { ... },
            fragment: Some(FragmentState { ... }),
            primitive: state.primitive(),           // Now includes polygon_mode
            depth_stencil: state.depth_stencil(),
            multisample: MultisampleState::default(),
            push_constant_ranges: vec![],
        }
    }
}

// Same pattern applied to other specialized pipelines.

// ============================================================================
// EGUi WINDOW UPDATED WITH POLYGON MODE
// ============================================================================

// In egui_tuning_window, added for materials that support it:
// ui.horizontal(|ui| {
//     ui.label("Polygon:");
//     egui::ComboBox::from_label("")
//         .selected_text(format!("{:?}", state.polygon_mode))
//         .show_ui(ui, |ui| {
//             ui.selectable_value(&mut state.polygon_mode, PolygonMode::Fill, "Fill");
//             ui.selectable_value(&mut state.polygon_mode, PolygonMode::Line, "Line (Wireframe)");
//             ui.selectable_value(&mut state.polygon_mode, PolygonMode::Point, "Point");
//         });
// });

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        // Resource registration
    }
}
