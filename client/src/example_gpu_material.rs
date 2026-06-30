/*!
 * example_gpu_material.rs
 *
 * Live tuning with file-based preset persistence (RON format).
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
use serde::{Deserialize, Serialize};

use crate::gpu_simulation::resources::{RbeGlobalState, CouncilValence, GlobalConfidence, MercyAttunement};

// ============================================================================
// SERIALIZABLE SAVED PRESET
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedPreset {
    pub name: String,
    pub rbe_flow: f32,
    pub rbe_circulating: f32,
    pub player_balance: f32,
    pub council_valence: f32,
    pub council_action: u32,
    pub council_participants: u32,
    pub confidence: f32,
    pub mercy_value: f32,
    pub mercy_thrivability: f32,
}

// ============================================================================
// SHADER TUNING PRESETS (with file persistence)
// ============================================================================

#[derive(Resource, Default)]
pub struct ShaderTuningPresets {
    pub current: usize,
    pub demo_active: bool,
    pub saved_presets: Vec<SavedPreset>,
}

impl ShaderTuningPresets {
    const PRESET_FILE: &'static str = "shader_presets.ron";

    pub fn save_to_file(&self) {
        if let Ok(ron) = ron::to_string(&self.saved_presets) {
            if let Err(e) = std::fs::write(Self::PRESET_FILE, ron) {
                warn!("Failed to save presets: {}", e);
            } else {
                info!("[Shader Presets] Saved {} presets to {}", self.saved_presets.len(), Self::PRESET_FILE);
            }
        }
    }

    pub fn load_from_file(&mut self) {
        if let Ok(content) = std::fs::read_to_string(Self::PRESET_FILE) {
            if let Ok(presets) = ron::from_str::<Vec<SavedPreset>>(&content) {
                self.saved_presets = presets;
                info!("[Shader Presets] Loaded {} presets from file", self.saved_presets.len());
            }
        }
    }

    pub fn save_current(
        &mut self,
        name: String,
        rbe: &RbeGlobalState,
        council: &CouncilValence,
        confidence: &GlobalConfidence,
        mercy_query: &Query<&MercyAttunement>,
    ) {
        // ... same as before ...
    }

    pub fn load_preset(
        &self,
        index: usize,
        rbe: &mut RbeGlobalState,
        council: &mut CouncilValence,
        confidence: &mut GlobalConfidence,
        mercy_query: &mut Query<&mut MercyAttunement>,
    ) {
        // ... same as before ...
    }
}

// ============================================================================
// INPUT SYSTEM (with file save/load)
// ============================================================================

pub fn shader_preset_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut presets: ResMut<ShaderTuningPresets>,
    // ... other resources ...
) {
    // Existing logic for 1-8, S, L, Space/P ...

    // File save (Ctrl + S)
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyS) {
        presets.save_to_file();
    }

    // File load (Ctrl + L)
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyL) {
        presets.load_from_file();
    }

    // Auto-load on first frame if file exists (optional)
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        let mut presets = ShaderTuningPresets::default();
        presets.load_from_file(); // Auto-load saved presets on startup

        app
            .insert_resource(presets)
            .add_systems(Update, (demo_animate_gpu_bridges, shader_preset_input));
    }
}
