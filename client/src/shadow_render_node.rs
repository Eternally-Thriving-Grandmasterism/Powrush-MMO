/*!
 * shadow_render_node.rs
 * Powrush-MMO — Wiring custom Poisson Disk PCF shader into the pipeline
 */

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
    renderer::RenderContext,
    RenderApp,
};
use bevy::pbr::ShadowPass;
use bevy::render::render_resource::Shader;

/// Custom Shadow Filtering Method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ShadowFilteringMethod {
    #[default]
    Hardware2x2,
    PoissonDisk,
    Disabled,
}

/// Resource holding the custom Poisson Disk PCF shader handle
#[derive(Resource)]
pub struct PoissonDiskPcfShader(pub Handle<Shader>);

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

/// Plugin that wires the custom Poisson Disk PCF shader into Bevy's pipeline
pub struct CustomShadowNodePlugin;

impl Plugin for CustomShadowNodePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        // Register custom node
        render_graph.add_node(
            "poisson_disk_shadow_node",
            PoissonDiskShadowNode::new(&mut render_app.world),
        );

        // Load the custom shader
        let shader_handle = render_app.world.resource::<AssetServer>().load("shaders/poisson_disk_pcf.wgsl");
        render_app.world.insert_resource(PoissonDiskPcfShader(shader_handle));

        // When `ShadowFilteringMethod::PoissonDisk` is active, the system
        // should switch to using the custom shader for shadow sampling.
        //
        // Full integration requires:
        // 1. Creating a custom `ShadowFilteringMethod::PoissonDisk`
        // 2. Specializing the shadow shader to call `poisson_disk_pcf()`
        // 3. Binding the PoissonDiskUniform in the shadow bind group
        //
        // This plugin provides the foundation and shader loading.
    }
}
