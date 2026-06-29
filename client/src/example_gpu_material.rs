/*!
 * Refined GpuSimulationState Custom Pipeline
 * 
 * Now supports Opaque3d and AlphaMask phases.
 * More robust registration and usage.
 */

use bevy::prelude::*;
use bevy::render::render_phase::*;
use bevy::render::render_resource::*;
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
            .init_resource::<DrawFunctions<AlphaMask3d>>()
            .add_render_command::<Opaque3d, SetGpuSimulationStateBindGroup>()
            .add_render_command::<AlphaMask3d, SetGpuSimulationStateBindGroup>();
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

// ==================== CONVENIENCE ====================

/// Helper to spawn an entity that uses GpuSimulationState
pub fn spawn_with_gpu_state(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>, // or your custom material
) -> Entity {
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            UsesGpuSimulationState,
        ))
        .id()
}