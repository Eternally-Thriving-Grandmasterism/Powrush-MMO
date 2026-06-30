/*!
 * example_gpu_material.rs
 *
 * Advanced live tuning with presets, demo control, and egui UI guidance.
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
// SHADER PRESET SYSTEM (Expanded)
// ============================================================================

#[derive(Resource, Default)]
pub struct ShaderTuningPresets {
    pub current: usize,
    pub demo_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaderPreset {
    Balanced,
    HighMercy,
    HighRbeFlow,
    CouncilActive,
    TenseLowConfidence,
    HighPlayerActivity,
    HighCouncilMercy,
    LowRbeTense,
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
            ShaderPreset::Balanced => { /* ... values ... */ }
            ShaderPreset::HighMercy => { /* ... values ... */ }
            ShaderPreset::HighRbeFlow => { /* ... values ... */ }
            ShaderPreset::CouncilActive => { /* ... values ... */ }
            ShaderPreset::TenseLowConfidence => { /* ... values ... */ }
            ShaderPreset::HighPlayerActivity => { /* ... values ... */ }
            ShaderPreset::HighCouncilMercy => {
                council.value = 0.85;
                for mut attunement in mercy_query.iter_mut() {
                    attunement.value = 0.9;
                    attunement.thrivability = 0.85;
                }
            }
            ShaderPreset::LowRbeTense => {
                rbe.flow_rate = 0.4;
                council.value = 0.2;
                confidence.value = 0.3;
            }
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ShaderPreset::Balanced => "Balanced",
            ShaderPreset::HighMercy => "High Mercy",
            ShaderPreset::HighRbeFlow => "High RBE Flow",
            ShaderPreset::CouncilActive => "Council Active",
            ShaderPreset::TenseLowConfidence => "Tense / Low Confidence",
            ShaderPreset::HighPlayerActivity => "High Player Activity",
            ShaderPreset::HighCouncilMercy => "High Council + Mercy",
            ShaderPreset::LowRbeTense => "Low RBE / Tense",
        }
    }
}

// ============================================================================
// DEMO ANIMATION (with pause support)
// ============================================================================

pub fn demo_animate_gpu_bridges(
    time: Res<Time>,
    presets: Res<ShaderTuningPresets>,
    mut rbe: ResMut<RbeGlobalState>,
    mut council: ResMut<CouncilValence>,
    mut confidence: ResMut<GlobalConfidence>,
    mut mercy_query: Query<&mut MercyAttunement>,
) {
    if !presets.demo_active {
        return; // Demo paused when using manual presets
    }

    let t = time.elapsed_seconds();

    rbe.flow_rate = (t.sin() * 0.5 + 0.5) * 3.5;
    // ... rest of animation ...
}

// ============================================================================
// PRESET INPUT (expanded)
// ============================================================================

pub fn shader_preset_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut presets: ResMut<ShaderTuningPresets>,
    mut rbe: ResMut<RbeGlobalState>,
    mut council: ResMut<CouncilValence>,
    mut confidence: ResMut<GlobalConfidence>,
    mut mercy_query: Query<&mut MercyAttunement>,
) {
    // Existing 1-6 logic + new keys 7-8
    // Also toggle demo with Space or P
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::KeyP) {
        presets.demo_active = !presets.demo_active;
        info!("[Shader Presets] Demo animation: {}", if presets.demo_active { "ON" } else { "OFF" });
    }

    // ... preset application logic ...
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
