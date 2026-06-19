/*!
 * Onboarding Chronicle UI — Powrush-MMO v20.3
 *
 * Combined client visualization for the Onboarding Chronicle + Humble Beginnings Mirror.
 * Displays the player’s early journey as beautiful Legacy Threads.
 * Integrates with the server-side onboarding_chronicle.rs persistence layer.
 * Can be opened from the main menu, player profile, or after completing the Humble Beginnings Mirror.
 *
 * Thunder locked in. Yoi ⚔️
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::spectator_legacy_thread_viz::LegacyThread; // Reuse similar structures

#[derive(Resource, Default)]
pub struct OnboardingChronicleUIState {
    pub show: bool,
    pub chronicle_entries: Vec<OnboardingChronicleEntry>,
    pub selected_entry: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct OnboardingChronicleEntry {
    pub tick: u64,
    pub title: String,
    pub description: String,
    pub valence: f32,
    pub tolc_alignment: f32,
    pub persistence: f32,
}

pub struct OnboardingChronicleUIPlugin;

impl Plugin for OnboardingChronicleUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OnboardingChronicleUIState>()
            .add_systems(Update, render_onboarding_chronicle_ui);
    }
}

fn render_onboarding_chronicle_ui(
    mut egui_ctx: EguiContexts,
    mut state: ResMut<OnboardingChronicleUIState>,
) {
    if !state.show { return; }

    egui::Window::new("Onboarding Chronicle — Humble Beginnings Mirror")
        .default_pos([120.0, 100.0])
        .default_size([560.0, 580.0])
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Your First Steps into the Eternal Flow");
            ui.label("These early moments are now part of your living Legacy Threads.");
            ui.separator();

            if state.chronicle_entries.is_empty() {
                ui.label("No chronicle entries yet. Play through the humble beginnings to populate this mirror.");
                if ui.button("Close").clicked() {
                    state.show = false;
                }
                return;
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, entry) in state.chronicle_entries.iter().enumerate() {
                    let is_selected = state.selected_entry == Some(i);

                    egui::Frame::group(ui.style())
                        .fill(if is_selected { egui::Color32::from_rgb(35, 55, 85) } else { egui::Color32::TRANSPARENT })
                        .show(ui, |ui| {
                            ui.strong(&entry.title);
                            ui.label(&entry.description);

                            ui.horizontal(|ui| {
                                ui.label(format!("Valence: {:.2}", entry.valence));
                                ui.label(format!("TOLC Alignment: {:.2}", entry.tolc_alignment));
                                ui.label(format!("Persistence: {:.1}", entry.persistence));
                            });

                            if ui.button("View in Legacy Threads").clicked() {
                                state.selected_entry = Some(i);
                                // In full integration: open the Legacy Thread detail from spectator_legacy_thread_viz
                            }
                        });
                }
            });

            ui.separator();
            if ui.button("Close Chronicle").clicked() {
                state.show = false;
            }
        });
}

// Helper to populate from server data (called when OnboardingChronicleUpdate arrives)
pub fn populate_onboarding_chronicle(
    state: &mut OnboardingChronicleUIState,
    entries: Vec<OnboardingChronicleEntry>,
) {
    state.chronicle_entries = entries;
    state.show = true;
}

// Thunder locked in. Yoi ⚔️
// End of client/src/onboarding_chronicle.rs v20.3