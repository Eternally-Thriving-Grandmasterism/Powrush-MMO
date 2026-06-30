/*!
 * example_gpu_material.rs
 *
 * Added PolygonMode selection to egui tuning window.
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

// ... (previous code) ...

// ============================================================================
// EGUi TUNING WINDOW WITH POLYGON MODE
// ============================================================================

pub fn egui_tuning_window(
    mut contexts: EguiContexts,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut presets: ResMut<ShaderTuningPresets>,
    mut halo_materials: ResMut<Assets<ValenceHaloMaterial>>,
    mut mycelial_materials: ResMut<Assets<MycelialWebGlowMaterial>>,
    mut node_materials: ResMut<Assets<ResourceNodeGlowMaterial>>,
    mut burst_materials: ResMut<Assets<EnergyBurstMaterial>>,
) {
    let mut egui_context = contexts.ctx_mut();

    egui::Window::new("Shader Presets & Render State")
        .default_width(400.0)
        .show(&mut egui_context, |ui| {
            ui.heading("Render State (Live)");

            macro_rules! render_state_controls {
                ($materials:expr, $label:expr) => {
                    if let Some((_, mat)) = $materials.iter_mut().next() {
                        let mut srgba = mat.base_color.to_srgba();
                        let mut state = mat.render_state;

                        ui.horizontal(|ui| {
                            ui.color_edit_button_srgba(&mut srgba);
                            ui.add(egui::Slider::new(&mut srgba.a, 0.0..=1.0).text("Alpha"));
                        });

                        // Blend Mode
                        ui.horizontal(|ui| {
                            ui.label("Blend:");
                            egui::ComboBox::from_label("")
                                .selected_text(format!("{:?}", state.blend_mode))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut state.blend_mode, AlphaBlendMode::Alpha, "Alpha");
                                    ui.selectable_value(&mut state.blend_mode, AlphaBlendMode::Additive, "Additive");
                                });
                        });

                        // Depth Write
                        ui.checkbox(&mut state.depth_write, "Depth Write");

                        // Cull Mode
                        ui.horizontal(|ui| {
                            ui.label("Cull:");
                            egui::ComboBox::from_label("")
                                .selected_text(format!("{:?}", state.cull_mode))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut state.cull_mode, None, "None");
                                    ui.selectable_value(&mut state.cull_mode, Some(Face::Front), "Front");
                                    ui.selectable_value(&mut state.cull_mode, Some(Face::Back), "Back");
                                });
                        });

                        // NEW: Polygon Mode
                        ui.horizontal(|ui| {
                            ui.label("Polygon:");
                            egui::ComboBox::from_label("")
                                .selected_text(format!("{:?}", state.polygon_mode))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut state.polygon_mode, PolygonMode::Fill, "Fill");
                                    ui.selectable_value(&mut state.polygon_mode, PolygonMode::Line, "Line (Wireframe)");
                                    ui.selectable_value(&mut state.polygon_mode, PolygonMode::Point, "Point");
                                });
                        });

                        mat.base_color = Color::from(srgba);
                        mat.render_state = state;
                        ui.label($label);
                    }
                };
            }

            render_state_controls!(burst_materials, "EnergyBurstMaterial");
            render_state_controls!(halo_materials, "ValenceHaloMaterial");
            render_state_controls!(mycelial_materials, "MycelialWebGlowMaterial");
            render_state_controls!(node_materials, "ResourceNodeGlowMaterial");

            ui.separator();
            ui.label("Polygon Mode: Fill = normal, Line = wireframe, Point = point cloud");
        });
}

// ... (rest of file) ...
