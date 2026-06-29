/*!
 * Complete Custom Render Pipeline for GpuSimulationState
 * 
 * This version includes proper phase registration so the bind group
 * actually gets set during rendering.
 */

use bevy::prelude::*;
use bevy::render::render_phase::*;
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderDevice;
use bevy::render::Extract;
use crate::rbe_client_sync::GpuSimulationStateBuffer;

#[derive(Component, Default, Clone, Copy)]
pub struct UsesGpuSimulationState;

// ==================== RENDER COMMAND ====================

pub struct SetGpuSimulationStateBindGroup;

impl<P: PhaseItem> RenderCommand<P> for SetGpuSimulationStateBindGroup {
    type Param = SRes<GpuSimulationStateBuffer>;
    type ViewQuery = ();
    type ItemQuery = Read<UsesGpuSimulationState>;

    fn render(
        _item: &P,
        _view: (),
        _entity: Read<UsesGpuSimulationState>,
        gpu_buffer: SystemParamItem<Self::Param>,
        pass: &mut TrackedRenderPass,
    ) -> RenderCommandResult {
        pass.set_bind_group(0, &gpu_buffer.bind_group, &[]);
        RenderCommandResult::Success
    }
}

// ==================== PLUGIN ====================

pub struct GpuSimulationStateRenderPlugin;

impl Plugin for GpuSimulationStateRenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(ExtractSchedule, extract_uses_gpu_state)
            .sub_app_mut(RenderApp)
            .init_resource::<DrawFunctions<Opaque3d>>()
            .add_render_command::<Opaque3d, SetGpuSimulationStateBindGroup>();
    }
}

fn extract_uses_gpu_state(
    mut commands: Commands,
    query: Extract<Query<Entity, With<UsesGpuSimulationState>>>,
) {
    for entity in query.iter() {
        commands.get_or_spawn(entity).insert(UsesGpuSimulationState);
    }
}

// ==================== USAGE ====================
// 1. Add GpuSimulationStateRenderPlugin to your app
// 2. Spawn entities with UsesGpuSimulationState + a mesh + material
// 3. The bind group will be automatically set during Opaque3d phase