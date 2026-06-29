/*!
 * Full Custom Pipeline Example for GpuSimulationState
 * 
 * This shows a more complete way to use GpuSimulationStateBuffer
 * with a custom render command.
 */

use bevy::prelude::*;
use bevy::render::render_phase::*;
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderDevice;
use crate::rbe_client_sync::GpuSimulationStateBuffer;

/// Marker component for entities that should receive GpuSimulationState
#[derive(Component, Default)]
pub struct UsesGpuSimulationState;

/// Custom render command that sets our GpuSimulationState bind group
pub struct SetGpuSimulationStateBindGroup;

impl<P: PhaseItem> RenderCommand<P> for SetGpuSimulationStateBindGroup {
    type Param = SRes<GpuSimulationStateBuffer>;
    type ViewQuery = ();
    type ItemQuery = ();

    fn render(
        _item: &P,
        _view: (),
        _entity: (),
        gpu_buffer: SystemParamItem<Self::Param>,
        pass: &mut TrackedRenderPass,
    ) -> RenderCommandResult {
        pass.set_bind_group(0, &gpu_buffer.bind_group, &[]);
        RenderCommandResult::Success
    }
}

/// Plugin that registers the custom render command
pub struct GpuSimulationStateRenderPlugin;

impl Plugin for GpuSimulationStateRenderPlugin {
    fn build(&self, app: &mut App) {
        // You would register the render command in the RenderApp here
        // and add it to the appropriate render phase (Opaque, AlphaMask, etc.)
        //
        // Example (simplified):
        // app.sub_app_mut(RenderApp)
        //     .init_resource::<DrawFunctions<Opaque3d>>()
        //     .add_render_command::<Opaque3d, SetGpuSimulationStateBindGroup>();
    }
}

// Note: Full integration with Bevy's render phases requires more setup.
// This file gives you the core pieces. Let me know if you want the
// complete version with proper phase registration and a custom pipeline.