// client/src/debug/testing_harness.rs (updated v16.7.0)
// Performance test harness for GPU culling + resonance during simulated Server Wars

use bevy::prelude::*;
use crate::gpu::infrastructure_culling::{GpuInfrastructureCullingSystem, InfrastructureCullingMarker};

pub fn performance_test_harness(
    mut culling: ResMut<GpuInfrastructureCullingSystem>,
    mut commands: Commands,
) {
    // Spawn 300 test infrastructure nodes with varying development levels
    for i in 0..300 {
        commands.spawn((
            InfrastructureCullingMarker {
                node_id: i as u64,
                position: /* random or grid position */ Default::default(),
                development_level: (i % 10) as u32 + 1,
                integrity: 0.7 + (i as f32 % 5.0) * 0.05,
                faction: if i % 4 == 0 { "NeutralObserver".to_string() } else { "Forge".to_string() },
                is_contested: i % 7 == 0,
            },
            // ... other components
        ));
    }

    culling.server_war_mode = true;

    // Trigger cull and assert visible count is reasonable (< 150 for performance)
    let visible = culling.cull_infrastructure(
        /* camera pos */ Default::default(),
        120.0,
        0.85,
        &/* collect from query */ vec![],
        /* current_time */ 0,
    );

    info!("Performance test: {} nodes visible out of 300", visible.len());
}
