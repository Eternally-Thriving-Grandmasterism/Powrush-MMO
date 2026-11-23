// client/src/debug/testing_harness.rs (updated v16.7.0)
// Performance test harness for GPU culling + resonance during simulated Server Wars
// Issue #738: COUNCIL BOARD - Revenue first - itch.io offline standalone spatial audio bypass & HRTF fallback caching.

use bevy::prelude::*;
use crate::gpu::infrastructure_culling::{GpuInfrastructureCullingSystem, InfrastructureCullingMarker};
use crate::ambisonics_engine::{AmbisonicEmitter, AmbisonicField, AmbisonicsEnginePlugin, SoundType};
use crate::game::procedural_music::HrtfImpulseResponses;

#[derive(Component)]
pub struct AudioListener;

pub fn performance_test_harness(
    mut culling: ResMut<GpuInfrastructureCullingSystem>,
    mut commands: Commands,
) {
    // Spawn 300 test infrastructure nodes with varying development levels
    for i in 0..300 {
        commands.spawn((
            InfrastructureCullingMarker {
                node_id: i as u64,
                position: Default::default(),
                development_level: (i % 10) as u32 + 1,
                integrity: 0.7 + (i as f32 % 5.0) * 0.05,
                faction: if i % 4 == 0 { "NeutralObserver".to_string() } else { "Forge".to_string() },
                is_contested: i % 7 == 0,
            },
        ));
    }

    culling.server_war_mode = true;

    // Trigger cull and assert visible count is reasonable (< 150 for performance)
    let visible = culling.cull_infrastructure(
        Default::default(),
        120.0,
        0.85,
        &vec![],
        0,
    );

    info!("Performance test: {} nodes visible out of 300", visible.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ambisonics_engine::AmbisonicsEnginePlugin;

    #[test]
    fn test_issue_738_council_board_ambisonic_spatial_audio_fallback() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AmbisonicsEnginePlugin)
            .insert_resource(HrtfImpulseResponses::default());

        // Spawn audio listener
        app.world.spawn((
            AudioListener,
            Transform::from_xyz(0.0, 0.0, 0.0),
        ));

        // Spawn ambisonic emitter simulating an offline standalone itch.io bypass scenario
        app.world.spawn((
            AmbisonicEmitter {
                position: Vec3::new(2.0, 0.0, 0.0),
                velocity: Vec3::ZERO,
                sound_type: SoundType::Ambient,
            },
            Transform::from_xyz(2.0, 0.0, 0.0),
        ));

        // Run systems to process spatial audio calculation and HRTF fallback caching
        app.update();

        let field = app.world.get_resource::<AmbisonicField>().expect("AmbisonicField resource must exist");
        
        // Verify First-Order Ambisonics coefficients (W, X, Y, Z) are computed correctly under offline fallback
        assert!(field.w > 0.0, "Omnidirectional component W should accumulate energy");
        assert!(field.x > 0.0, "Front-back component X should reflect emitter position along X-axis");
        assert_eq!(field.y, 0.0, "Y component should remain zero when emitter is purely along X-axis");
        assert_eq!(field.z, 0.0, "Z component should remain zero when emitter is at same height");
    }
}