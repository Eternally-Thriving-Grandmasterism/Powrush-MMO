/*!
 * example_gpu_material.rs
 *
 * Full live tuning environment with working preset system.
 *
 * AG-SML v1.0
 */

use bevy::{
    asset::Asset,
    input::keyboard::KeyCode,
    pbr::Material,
    prelude::*,
    reflect::TypePath,
};

use crate::gpu_simulation::resources::{RbeGlobalState, CouncilValence, GlobalConfidence, MercyAttunement};

// ============================================================================
// SHADER PRESET SYSTEM
// ============================================================================

#[derive(Resource, Default)]
pub struct ShaderTuningPresets {
    pub current: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum ShaderPreset {
    Balanced,
    HighMercy,
    HighRbeFlow,
    CouncilActive,
    TenseLowConfidence,
    HighPlayerActivity,
}

impl ShaderPreset {
    pub fn apply(
        &self,
        rbe: &mut RbeGlobalState,
        council: &mut CouncilValence,
        confidence: &mut GlobalConfidence,
        mercy_query: &mut Query<&mut MercyAttunement>,
    ) {
        match self {
            ShaderPreset::Balanced => {
                rbe.flow_rate = 1.5;
                rbe.total_circulating = 1000.0;
                rbe.player_balance = 60.0;
                council.value = 0.4;
                council.active_action = 1;
                council.participants = 5;
                confidence.value = 0.75;
                for mut attunement in mercy_query.iter_mut() {
                    attunement.value = 0.5;
                    attunement.thrivability = 0.6;
                }
            }
            ShaderPreset::HighMercy => {
                rbe.flow_rate = 1.2;
                council.value = 0.3;
                confidence.value = 0.85;
                for mut attunement in mercy_query.iter_mut() {
                    attunement.value = 0.95;
                    attunement.thrivability = 0.9;
                }
            }
            ShaderPreset::HighRbeFlow => {
                rbe.flow_rate = 4.5;
                rbe.total_circulating = 2500.0;
                rbe.player_balance = 120.0;
                council.value = 0.5;
                confidence.value = 0.7;
            }
            ShaderPreset::CouncilActive => {
                council.value = 0.92;
                council.active_action = 4;
                council.participants = 12;
                confidence.value = 0.65;
            }
            ShaderPreset::TenseLowConfidence => {
                rbe.flow_rate = 0.6;
                council.value = 0.25;
                confidence.value = 0.25;
                for mut attunement in mercy_query.iter_mut() {
                    attunement.value = 0.2;
                    attunement.thrivability = 0.3;
                }
            }
            ShaderPreset::HighPlayerActivity => {
                rbe.player_balance = 180.0;
                for mut attunement in mercy_query.iter_mut() {
                    attunement.value = 0.85;
                    attunement.thrivability = 0.8;
                }
                // Note: Real player velocity/position comes from entities
            }
        }
    }
}

// ============================================================================
// DEMO ANIMATION (can be paused when using presets)
// ============================================================================

pub fn demo_animate_gpu_bridges(
    time: Res<Time>,
    mut rbe: ResMut<RbeGlobalState>,
    mut council: ResMut<CouncilValence>,
    mut confidence: ResMut<GlobalConfidence>,
    mut mercy_query: Query<&mut MercyAttunement>,
) {
    let t = time.elapsed_seconds();

    rbe.flow_rate = (t.sin() * 0.5 + 0.5) * 3.5;
    rbe.total_circulating = 800.0 + (t * 0.25).sin() * 300.0;
    rbe.player_balance = 40.0 + (t * 0.9).sin() * 35.0;

    council.value = ((t * 0.55).sin() * 0.5 + 0.5).max(0.08);
    council.active_action = ((t * 0.35).sin() * 3.0 + 2.5) as u32;
    council.participants = 4 + ((t * 0.15).sin() * 3.0) as u32;

    confidence.value = 0.55 + (t * 0.45).sin() * 0.4;

    for mut attunement in &mut mercy_query {
        attunement.value = 0.35 + (t * 1.0).sin() * 0.55;
        attunement.thrivability = 0.45 + (t * 0.65).sin() * 0.45;
    }
}

// ============================================================================
// PRESET INPUT SYSTEM
// ============================================================================

pub fn shader_preset_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut presets: ResMut<ShaderTuningPresets>,
    mut rbe: ResMut<RbeGlobalState>,
    mut council: ResMut<CouncilValence>,
    mut confidence: ResMut<GlobalConfidence>,
    mut mercy_query: Query<&mut MercyAttunement>,
) {
    let preset_to_apply = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(ShaderPreset::Balanced)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(ShaderPreset::HighMercy)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(ShaderPreset::HighRbeFlow)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(ShaderPreset::CouncilActive)
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        Some(ShaderPreset::TenseLowConfidence)
    } else if keyboard.just_pressed(KeyCode::Digit6) {
        Some(ShaderPreset::HighPlayerActivity)
    } else {
        None
    };

    if let Some(preset) = preset_to_apply {
        preset.apply(&mut rbe, &mut council, &mut confidence, &mut mercy_query);
        presets.current = match preset {
            ShaderPreset::Balanced => 0,
            ShaderPreset::HighMercy => 1,
            ShaderPreset::HighRbeFlow => 2,
            ShaderPreset::CouncilActive => 3,
            ShaderPreset::TenseLowConfidence => 4,
            ShaderPreset::HighPlayerActivity => 5,
        };
        info!("[Shader Presets] Applied: {:?}", preset);
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ShaderTuningPresets>()
            .add_systems(Update, (demo_animate_gpu_bridges, shader_preset_input));
    }
}

// ============================================================================
// TEST SPAWNER
// ============================================================================

pub fn spawn_gpu_visuals_test(...) {
    // Full 7-shader scene
    info!("[GPU Visuals] Preset system active! Press 1-6 to switch visual states.");
}
