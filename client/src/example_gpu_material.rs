/*!
 * example_gpu_material.rs
 *
 * Live tuning with import/export for presets.
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
// SERIALIZABLE PRESET
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
// TUNING PRESETS RESOURCE (with import/export)
// ============================================================================

#[derive(Resource, Default)]
pub struct ShaderTuningPresets {
    pub current: usize,
    pub demo_active: bool,
    pub saved_presets: Vec<SavedPreset>,
}

impl ShaderTuningPresets {
    pub fn export_current(
        &self,
        custom_name: Option<String>,
        rbe: &RbeGlobalState,
        council: &CouncilValence,
        confidence: &GlobalConfidence,
        mercy_query: &Query<&MercyAttunement>,
    ) {
        let name = custom_name.unwrap_or_else(|| format!("export_{}", self.saved_presets.len()));

        let mercy_value = mercy_query.iter().next().map(|m| m.value).unwrap_or(0.5);
        let mercy_thrivability = mercy_query.iter().next().map(|m| m.thrivability).unwrap_or(0.6);

        let preset = SavedPreset {
            name: name.clone(),
            rbe_flow: rbe.flow_rate,
            rbe_circulating: rbe.total_circulating,
            player_balance: rbe.player_balance,
            council_valence: council.value,
            council_action: council.active_action,
            council_participants: council.participants,
            confidence: confidence.value,
            mercy_value,
            mercy_thrivability,
        };

        let filename = format!("{}.ron", name);
        if let Ok(ron_str) = ron::to_string(&preset) {
            if std::fs::write(&filename, ron_str).is_ok() {
                info!("[Shader Presets] Exported preset to {}", filename);
            }
        }
    }

    pub fn import_from_file(&mut self, path: &str) {
        if let Ok(content) = std::fs::read_to_string(path) {
            // Try single preset
            if let Ok(single) = ron::from_str::<SavedPreset>(&content) {
                self.saved_presets.push(single);
                info!("[Shader Presets] Imported 1 preset from {}", path);
                return;
            }

            // Try list of presets
            if let Ok(multiple) = ron::from_str::<Vec<SavedPreset>>(&content) {
                let count = multiple.len();
                self.saved_presets.extend(multiple);
                info!("[Shader Presets] Imported {} presets from {}", count, path);
            }
        }
    }

    // ... existing save_current, load_preset, save_to_file, load_from_file ...
}

// ============================================================================
// INPUT (with import/export)
// ============================================================================

pub fn shader_preset_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut presets: ResMut<ShaderTuningPresets>,
    rbe: Res<RbeGlobalState>,
    council: Res<CouncilValence>,
    confidence: Res<GlobalConfidence>,
    mut mercy_query: Query<&mut MercyAttunement>,
) {
    // Existing logic (1-8, S, L, Space/P, Ctrl+S, Ctrl+L) ...

    // Export current state (Ctrl + E)
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyE) {
        presets.export_current(None, &rbe, &council, &confidence, &mercy_query);
    }

    // Import from file (example: press Ctrl + I then type filename in console or hardcode)
    // For simplicity, we provide a method. In a full UI you would have a file dialog.
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyI) {
        // Example: import from a known file
        presets.import_from_file("my_favorite_preset.ron");
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        let mut presets = ShaderTuningPresets::default();
        presets.load_from_file();
        app.insert_resource(presets)
            .add_systems(Update, (demo_animate_gpu_bridges, shader_preset_input));
    }
}
