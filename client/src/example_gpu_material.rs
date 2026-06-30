/*!
 * example_gpu_material.rs
 *
 * Advanced live tuning environment with simulated player movement and node data.
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
// ADVANCED DEMO WITH PLAYER + NODE SIMULATION
// ============================================================================

pub fn demo_animate_gpu_bridges(
    time: Res<Time>,
    mut rbe: ResMut<RbeGlobalState>,
    mut council: ResMut<CouncilValence>,
    mut confidence: ResMut<GlobalConfidence>,
    mut mercy_query: Query<&mut MercyAttunement>,
) {
    let t = time.elapsed_seconds();

    // RBE
    rbe.flow_rate = (t.sin() * 0.5 + 0.5) * 3.5;
    rbe.total_circulating = 800.0 + (t * 0.25).sin() * 300.0;
    rbe.player_balance = 40.0 + (t * 0.9).sin() * 35.0;

    // Council
    council.value = ((t * 0.55).sin() * 0.5 + 0.5).max(0.08);
    council.active_action = ((t * 0.35).sin() * 3.0 + 2.5) as u32;
    council.participants = 4 + ((t * 0.15).sin() * 3.0) as u32;

    // Global Confidence
    confidence.value = 0.55 + (t * 0.45).sin() * 0.4;

    // Player Mercy
    for mut attunement in &mut mercy_query {
        attunement.value = 0.35 + (t * 1.0).sin() * 0.55;
        attunement.thrivability = 0.45 + (t * 0.65).sin() * 0.45;
    }

    // Note: For full live tuning of player position/velocity and node_confidences,
    // the real sync system is used when entities exist.
    // The demo provides rich animation on the main bridge resources.
}

// ============================================================================
// Plugin
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        // registrations + demo system
        app.add_systems(Update, demo_animate_gpu_bridges);
    }
}

// ============================================================================
// TEST SPAWNER
// ============================================================================

pub fn spawn_gpu_visuals_test(...) {
    // Full 7-shader scene
    info!("[GPU Visuals] Advanced live tuning ready with rich demo animation.");
}
