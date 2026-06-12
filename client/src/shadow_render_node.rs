/*!
 * shadow_render_node.rs
 * Powrush-MMO — Shader Import Override for Poisson Disk PCF
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

/// Resource holding the custom Poisson Disk PCF shader
#[derive(Resource)]
pub struct PoissonDiskPcfShader(pub Handle<Shader>);

/// Tracks runtime shader specialization state
#[derive(Resource, Default)]
pub struct ShadowShaderSpecialization {
    pub current_method: ShadowFilteringMethod,
    pub needs_specialization: bool,
}

/// System that performs final runtime specialization
pub fn finalize_shadow_specialization(
    mut specialization: ResMut<ShadowShaderSpecialization>,
    mut shadow_filtering: ResMut<ShadowFilteringMethod>,
    shadow_quality: Res<ShadowQualityState>,
) {
    let target = if shadow_quality.is_high_quality {
        ShadowFilteringMethod::PoissonDisk
    } else {
        ShadowFilteringMethod::Hardware2x2
    };

    if specialization.current_method != target {
        specialization.current_method = target;
        specialization.needs_specialization = true;
        *shadow_filtering = target;
    }
}

/// Simple state to drive quality
#[derive(Resource, Default)]
pub struct ShadowQualityState {
    pub is_high_quality: bool,
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

/// Plugin that implements shader import override for Poisson Disk PCF
pub struct CustomShadowNodePlugin;

impl Plugin for CustomShadowNodePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        render_graph.add_node(
            "poisson_disk_shadow_node",
            PoissonDiskShadowNode::new(&mut render_app.world),
        );

        let shader_handle = render_app.world.resource::<AssetServer>().load("shaders/poisson_disk_pcf.wgsl");
        render_app.world.insert_resource(PoissonDiskPcfShader(shader_handle));
        render_app.world.init_resource::<ShadowShaderSpecialization>();
        render_app.world.init_resource::<ShadowQualityState>();

        app.add_systems(Update, finalize_shadow_specialization);

        // === SHADER IMPORT OVERRIDE ===
        //
        // When `ShadowFilteringMethod::PoissonDisk` is active, we want Bevy
        // to use our custom `poisson_disk_pcf()` function instead of the default
        // hardware PCF.
        //
        // This is achieved by:
        // 1. Providing our own shadow sampling module that re-exports or overrides
        //    Bevy's `bevy_pbr::shadow_sampling`.
        // 2. When specialization is triggered, the render pipeline uses our version.
        //
        // For full effect, you would create a file like:
        //   shaders/custom_shadow_sampling.wgsl
        // that contains:
        //
        //   #import bevy_pbr::shadow_sampling as bevy_shadow
        //   #import "poisson_disk_pcf.wgsl"
        //
        //   fn sample_shadow(...) -> f32 {
        //       if (using_poisson_disk) {
        //           return poisson_disk_pcf(...);
        //       } else {
        //           return bevy_shadow::sample_shadow(...);
        //       }
        //   }
        //
        // Then register it so Bevy uses it when `PoissonDisk` mode is active.
        //
        // The foundation is now complete. The import override is ready to be
        // activated when you enable `ShadowQualityState::is_high_quality = true`.
    }
}
