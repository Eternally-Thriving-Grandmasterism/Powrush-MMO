/*!
 * shadow_render_node.rs
 * Powrush-MMO — Custom ShadowFilteringMethod for Poisson Disk PCF
 */

use bevy::prelude::*;
use bevy::render::render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext};
use bevy::render::renderer::RenderContext;
use bevy::pbr::ShadowPass;

/// Custom Shadow Filtering Method
///
/// This enum allows us to switch between different shadow filtering techniques.
/// In a full implementation, `PoissonDisk` would use our custom WGSL shader.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ShadowFilteringMethod {
    #[default]
    Hardware2x2,
    PoissonDisk,      // Our custom high-quality PCF
    Disabled,
}

/// Custom Shadow Render Node
pub struct PoissonDiskShadowNode {
    query: QueryState<&'static bevy::render::view::ViewDepthTexture>,
}

impl PoissonDiskShadowNode {
    pub fn new(world: &mut World) -> Self {
        Self {
            query: QueryState::new(world),
        }
    }
}

impl Node for PoissonDiskShadowNode {
    fn input(&self) -> Vec<bevy::render::render_graph::SlotInfo> {
        vec![]
    }

    fn output(&self) -> Vec<bevy::render::render_graph::SlotInfo> {
        vec![]
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        _render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let _shadow_pass = world.resource::<ShadowPass>();
        Ok(())
    }
}

/// Plugin that registers everything needed for custom shadow filtering
pub struct CustomShadowNodePlugin;

impl Plugin for CustomShadowNodePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(bevy::render::RenderApp);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        render_graph.add_node(
            "poisson_disk_shadow_node",
            PoissonDiskShadowNode::new(&mut render_app.world),
        );

        // When full custom filtering is ready, we would also:
        // - Register a custom ShadowFilteringMethod::PoissonDisk
        // - Provide the poisson_disk_pcf.wgsl shader to the shadow pipeline
        // - Bind the PoissonDiskUniform in the correct bind group
    }
}
