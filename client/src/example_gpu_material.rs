/*!
 * example_gpu_material.rs
 *
 * Live tuning with real-time color pickers for material base colors.
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
use bevy_egui::{egui, EguiContexts};
use serde::{Deserialize, Serialize};

use crate::gpu_simulation::resources::{RbeGlobalState, CouncilValence, GlobalConfidence, MercyAttunement};

// ... (previous code for presets, materials, etc. remains) ...

// ============================================================================
// EGUi TUNING WINDOW (with color pickers)
// ============================================================================

pub fn egui_tuning_window(
    mut contexts: EguiContexts,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut presets: ResMut<ShaderTuningPresets>,
    mut gpu_materials: ResMut<Assets<GpuStateMaterial>>,
    mut halo_materials: ResMut<Assets<ValenceHaloMaterial>>,
    mut mycelial_materials: ResMut<Assets<MycelialWebGlowMaterial>>,
    mut node_materials: ResMut<Assets<ResourceNodeGlowMaterial>>,
    mut burst_materials: ResMut<Assets<EnergyBurstMaterial>>,
    mut field_materials: ResMut<Assets<ResonanceFieldMaterial>>,
    mut wave_materials: ResMut<Assets<ForgivenessWaveMaterial>>,
) {
    let mut egui_context = contexts.ctx_mut();

    let window_open = keyboard.pressed(KeyCode::F2);

    egui::Window::new("Shader Presets & Colors")
        .default_width(320.0)
        .open(&mut window_open)  // Can be toggled with F2
        .show(&mut egui_context, |ui| {
            ui.heading("Presets");

            // Existing preset buttons (1-8 + custom)...

            ui.separator();
            ui.heading("Material Base Colors (Live)");

            // Color pickers for each material type used in the test scene
            if let Some(mat) = gpu_materials.iter_mut().next() {
                let mut color = mat.1.base_color.to_srgba();
                if ui.color_edit_button_srgba(&mut color).changed() {
                    mat.1.base_color = Color::from(color);
                }
                ui.label("GpuStateMaterial");
            }

            if let Some(mat) = halo_materials.iter_mut().next() {
                let mut color = mat.1.base_color.to_srgba();
                if ui.color_edit_button_srgba(&mut color).changed() {
                    mat.1.base_color = Color::from(color);
                }
                ui.label("ValenceHaloMaterial");
            }

            if let Some(mat) = mycelial_materials.iter_mut().next() {
                let mut color = mat.1.base_color.to_srgba();
                if ui.color_edit_button_srgba(&mut color).changed() {
                    mat.1.base_color = Color::from(color);
                }
                ui.label("MycelialWebGlowMaterial");
            }

            if let Some(mat) = node_materials.iter_mut().next() {
                let mut color = mat.1.base_color.to_srgba();
                if ui.color_edit_button_srgba(&mut color).changed() {
                    mat.1.base_color = Color::from(color);
                }
                ui.label("ResourceNodeGlowMaterial");
            }

            ui.separator();
            ui.label("Tip: Use color pickers + presets together for fast iteration.");
        });
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct GpuVisualMaterialsPlugin;

impl Plugin for GpuVisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ShaderTuningPresets>()
            .add_systems(Update, (demo_animate_gpu_bridges, shader_preset_input, egui_tuning_window));
    }
}
