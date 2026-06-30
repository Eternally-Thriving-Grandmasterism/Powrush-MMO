/*!
 * example_gpu_material.rs
 *
 * Test scene with strong live tuning support.
 *
 * === Live Tuning ===
 * 1. Add bevy_inspector_egui to your project.
 * 2. The bridge resources (RbeGlobalState, CouncilValence, GlobalConfidence, MercyAttunement)
 *    are already Resources/Components and will appear in the inspector.
 * 3. Modify them live while the demo animation or real systems are running.
 * 4. Watch the shaders react in real time.
 *
 * The demo_animate_gpu_bridges system provides baseline animation so you can see
 * effects even without real game systems running.
 *
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    pbr::Material,
    prelude::*,
    reflect::TypePath,
};

use crate::gpu_simulation::resources::{RbeGlobalState, CouncilValence, GlobalConfidence, MercyAttunement};

// ============================================================================
// Materials (abbreviated for brevity - full definitions exist above in actual file)
// ... (materials remain the same)
// ============================================================================

// ============================================================================
// ENHANCED DEMO ANIMATION FOR LIVE TUNING
// ============================================================================

pub fn demo_animate_gpu_bridges(
    time: Res<Time>,
    mut rbe: ResMut<RbeGlobalState>,
    mut council: ResMut<CouncilValence>,
    mut confidence: ResMut<GlobalConfidence>,
    mut mercy_query: Query<&mut MercyAttunement>,
) {
    let t = time.elapsed_seconds();

    // RBE flow - pulsing economic activity
    rbe.flow_rate = (t.sin() * 0.5 + 0.5) * 3.0;
    rbe.total_circulating = 1000.0 + (t * 0.3).sin() * 200.0;
    rbe.player_balance = 50.0 + (t * 0.8).sin() * 30.0;

    // Council valence - activity level
    council.value = ((t * 0.6).sin() * 0.5 + 0.5).max(0.05);
    council.active_action = ((t * 0.4).sin() * 2.0 + 2.0) as u32;
    council.participants = 3 + ((t * 0.2).sin() * 2.0) as u32;

    // Global confidence
    confidence.value = 0.6 + (t * 0.5).sin() * 0.35;

    // Player mercy attunement (if any entity has the component)
    for mut attunement in &mut mercy_query {
        attunement.value = 0.4 + (t * 1.1).sin() * 0.5;
        attunement.thrivability = 0.5 + (t * 0.7).sin() * 0.4;
    }
}

// ============================================================================
// Plugin
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        // ... asset registrations ...
        app.add_systems(Update, demo_animate_gpu_bridges);

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            // ...
        }
    }
}

// ============================================================================
// TEST SPAWNER (unchanged - full 7 shader coverage)
// ============================================================================

pub fn spawn_gpu_visuals_test(...) {
    // ... same as previous version ...
    info!("[GPU Visuals] Full test scene ready. Use inspector to live-tune bridge resources.");
}
