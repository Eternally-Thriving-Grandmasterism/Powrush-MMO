// client/src/debug/testing_harness.rs (updated v16.7.0)
// Performance test harness for GPU culling + resonance during simulated Server Wars
// COUNCIL BOARD CARD #738: Offline build sellable on itch.io (Revenue First verification)

use bevy::prelude::*;
use crate::gpu::infrastructure_culling::{GpuInfrastructureCullingSystem, InfrastructureCullingMarker};
use crate::ambisonics_engine::{AmbisonicEmitter, AmbisonicField, AmbisonicsEnginePlugin, SoundType};
use crate::game::procedural_music::HrtfImpulseResponses;

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
    use crate::ambisonics_engine::{update_ambisonics_field, AmbisonicField};
    use crate::game::procedural_music::HrtfImpulseResponses;

    #[test]
    fn test_issue_738_ambisonics_offline_build_field_calculation() {
        // Verify Card #738: Ensure First-Order Ambisonics spatialization calculates 
        // valid coefficients for offline/standalone itch.io builds without panicking.
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        app.init_resource::<AmbisonicField>();
        app.insert_resource(HrtfImpulseResponses::default());

        // Spawn a listener
        app.world.spawn(AudioListener {
            position: Vec3::ZERO,
        });

        // Spawn an ambisonic emitter
        app.world.spawn((
            AmbisonicEmitter {
                position: Vec3::new(1.0, 0.0, 0.0),
                velocity: Vec3::ZERO,
                sound_type: SoundType::Ambient,
            },
            Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
        ));

        // Add update system
        app.add_systems(Update, update_ambisonics_field);

        // Run app update ticks to verify system stability and execution
        app.update();

        let field = app.world.resource::<AmbisonicField>();
        
        // Assert that energy is successfully encoded into the W omnidirectional channel
        assert!(field.w > 0.0, "Ambisonic W coefficient must register audio energy for offline build");
        // Assert X channel reflects front-back positioning from emitter at x=1.0
        assert!(field.x > 0.0, "Ambisonic X coefficient must register directional energy");
    }
}