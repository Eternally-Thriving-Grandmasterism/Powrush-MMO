/*!
 * example_gpu_material.rs
 *
 * Live tuning with color pickers + alpha sliders.
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

// ... (imports and other code remain) ...

// ============================================================================
// EGUi TUNING WINDOW (with color pickers + alpha sliders)
// ============================================================================

pub fn egui_tuning_window(
    mut contexts: EguiContexts,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut presets: ResMut<ShaderTuningPresets>,
    mut gpu_materials: ResMut<Assets<GpuStateMaterial>>,
    mut halo_materials: ResMut<Assets<ValenceHaloMaterial>>,
    mut mycelial_materials: ResMut<Assets<MycelialWebGlowMaterial>>,
    mut node_materials: ResMut<Assets<ResourceNodeGlowMaterial>>,
) {
    let mut egui_context = contexts.ctx_mut();

    egui::Window::new("Shader Presets & Colors")
        .default_width(340.0)
        .show(&mut egui_context, |ui| {
            ui.heading("Material Base Colors + Alpha (Live)");

            // Helper macro for color + alpha control
            macro_rules! color_and_alpha {
                ($materials:expr, $label:expr) => {
                    if let Some((_, mat)) = $materials.iter_mut().next() {
                        let mut srgba = mat.base_color.to_srgba();

                        let mut changed = false;

                        ui.horizontal(|ui| {
                            if ui.color_edit_button_srgba(&mut srgba).changed() {
                                changed = true;
                            }
                            if ui.add(egui::Slider::new(&mut srgba.a, 0.0..=1.0).text("Alpha")).changed() {
                                changed = true;
                            }
                        });

                        if changed {
                            mat.base_color = Color::from(srgba);
                        }
                        ui.label($label);
                    }
                };
            }

            color_and_alpha!(gpu_materials, "GpuStateMaterial");
            color_and_alpha!(halo_materials, "ValenceHaloMaterial");
            color_and_alpha!(mycelial_materials, "MycelialWebGlowMaterial");
            color_and_alpha!(node_materials, "ResourceNodeGlowMaterial");

            ui.separator();
            ui.label("Tip: Alpha sliders are great for tuning transparency effects in the shaders.");
        });
}

// ... (rest of the file remains the same) ...
