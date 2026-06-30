/*!
 * example_gpu_material.rs
 *
 * Added live blend mode selection in egui window.
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

// ... (rest of imports and code) ...

// ============================================================================
// EGUi TUNING WINDOW (with blend mode selection)
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
) {
    let mut egui_context = contexts.ctx_mut();

    egui::Window::new("Shader Presets & Colors")
        .default_width(360.0)
        .show(&mut egui_context, |ui| {
            ui.heading("Material Base Colors + Alpha + Blend Mode");

            // Helper for color + alpha + blend mode
            macro_rules! material_controls {
                ($materials:expr, $label:expr, $has_blend:expr) => {
                    if let Some((_, mat)) = $materials.iter_mut().next() {
                        let mut srgba = mat.base_color.to_srgba();

                        ui.horizontal(|ui| {
                            ui.color_edit_button_srgba(&mut srgba);
                            ui.add(egui::Slider::new(&mut srgba.a, 0.0..=1.0).text("Alpha"));
                        });

                        if $has_blend {
                            ui.horizontal(|ui| {
                                ui.label("Blend:");
                                egui::ComboBox::from_label("")
                                    .selected_text(format!("{:?}", mat.blend_mode))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut mat.blend_mode, AlphaBlendMode::Alpha, "Alpha");
                                        ui.selectable_value(&mut mat.blend_mode, AlphaBlendMode::Additive, "Additive");
                                    });
                            });
                        }

                        mat.base_color = Color::from(srgba);
                        ui.label($label);
                    }
                };
            }

            material_controls!(gpu_materials, "GpuStateMaterial", false);
            material_controls!(halo_materials, "ValenceHaloMaterial", true);
            material_controls!(mycelial_materials, "MycelialWebGlowMaterial", true);
            material_controls!(node_materials, "ResourceNodeGlowMaterial", true);
            material_controls!(burst_materials, "EnergyBurstMaterial", true);

            ui.separator();
            ui.label("Note: Changing blend mode requires pipeline re-specialization (works for specialized materials).");
        });
}

// ... (rest of file) ...
