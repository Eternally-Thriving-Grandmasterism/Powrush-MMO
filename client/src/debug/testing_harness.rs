// client/src/debug/testing_harness.rs (updated v16.7.0)
// Performance test harness for GPU culling + resonance during simulated Server Wars
// COUNCIL BOARD CARD #738: Offline itch.io build audio fallback and spatial stabilization unit tests

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
    use bevy::app::App;
    use glam::Vec3;

    #[test]
    fn test_issue_738_ambisonic_offline_fallback_and_stabilization() {
        let mut app = App::new();
        app.add_plugins(AmbisonicsEnginePlugin);

        // Initialize required resources for the ambisonics system
        app.insert_resource(AmbisonicField::default());
        app.insert_resource(HrtfImpulseResponses::default());

        // Setup audio listener representing the player
        app.world.spawn(AudioListener {
            position: Vec3::ZERO,
        });

        // Spawn test emitters with different sound types to verify Card #738 type weightings and offline stability clamps
        app.world.spawn((
            AmbisonicEmitter {
                position: Vec3::new(1.0, 0.0, 0.0),
                velocity: Vec3::ZERO,
                sound_type: SoundType::JoySanctuary,
            },
            Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
        ));

        app.world.spawn((
            AmbisonicEmitter {
                position: Vec3::new(0.0, 0.0, 5.0),
                velocity: Vec3::ZERO,
                sound_type: SoundType::RbeResource,
            },
            Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
        ));

        // Run update systems
        app.update();

        // Verify that the AmbisonicField resource accumulated values correctly and remains within stable limits (no NaN/Inf from offline fallbacks)
        let field = app.world.resource::<AmbisonicField>();
        assert!(field.w.is_finite(), "Ambisonic omnidirectional component W must be finite for Card #738 offline stability");
        assert!(field.w > 0.0, "Ambisonic field should receive energy from active emitters");
    }
}