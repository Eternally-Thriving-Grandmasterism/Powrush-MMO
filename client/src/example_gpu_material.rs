/*!
 * example_gpu_material.rs
 *
 * Live tuning with full save/load preset support.
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
// SAVED PRESET DATA
// ============================================================================

#[derive(Clone, Debug)]
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
// SHADER TUNING PRESETS RESOURCE (with save/load)
// ============================================================================

#[derive(Resource, Default)]
pub struct ShaderTuningPresets {
    pub current: usize,
    pub demo_active: bool,
    pub saved_presets: Vec<SavedPreset>,
}

impl ShaderTuningPresets {
    pub fn save_current(
        &mut self,
        name: String,
        rbe: &RbeGlobalState,
        council: &CouncilValence,
        confidence: &GlobalConfidence,
        mercy_query: &Query<&MercyAttunement>,
    ) {
        let mercy_value = mercy_query.iter().next().map(|m| m.value).unwrap_or(0.5);
        let mercy_thrivability = mercy_query.iter().next().map(|m| m.thrivability).unwrap_or(0.6);

        let preset = SavedPreset {
            name,
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

        self.saved_presets.push(preset);
        info!("[Shader Presets] Saved new preset. Total: {}", self.saved_presets.len());
    }

    pub fn load_preset(
        &self,
        index: usize,
        rbe: &mut RbeGlobalState,
        council: &mut CouncilValence,
        confidence: &mut GlobalConfidence,
        mercy_query: &mut Query<&mut MercyAttunement>,
    ) {
        if let Some(preset) = self.saved_presets.get(index) {
            rbe.flow_rate = preset.rbe_flow;
            rbe.total_circulating = preset.rbe_circulating;
            rbe.player_balance = preset.player_balance;

            council.value = preset.council_valence;
            council.active_action = preset.council_action;
            council.participants = preset.council_participants;

            confidence.value = preset.confidence;

            for mut attunement in mercy_query.iter_mut() {
                attunement.value = preset.mercy_value;
                attunement.thrivability = preset.mercy_thrivability;
            }

            info!("[Shader Presets] Loaded: {}", preset.name);
        }
    }
}

// ============================================================================
// KEYBOARD INPUT (expanded with save/load)
// ============================================================================

pub fn shader_preset_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut presets: ResMut<ShaderTuningPresets>,
    rbe: Res<RbeGlobalState>,
    council: Res<CouncilValence>,
    confidence: Res<GlobalConfidence>,
    mut mercy_query: Query<&mut MercyAttunement>,
    mut rbe_mut: ResMut<RbeGlobalState>,
    mut council_mut: ResMut<CouncilValence>,
    mut confidence_mut: ResMut<GlobalConfidence>,
) {
    // Existing 1-8 preset logic...

    // Save current state as custom preset (Key S)
    if keyboard.just_pressed(KeyCode::KeyS) {
        let name = format!("Custom_{}", presets.saved_presets.len() + 1);
        presets.save_current(name, &rbe, &council, &confidence, &mercy_query);
    }

    // Load last saved preset (Key L)
    if keyboard.just_pressed(KeyCode::KeyL) && !presets.saved_presets.is_empty() {
        let last_index = presets.saved_presets.len() - 1;
        presets.load_preset(last_index, &mut rbe_mut, &mut council_mut, &mut confidence_mut, &mut mercy_query);
    }

    // Toggle demo
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::KeyP) {
        presets.demo_active = !presets.demo_active;
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
