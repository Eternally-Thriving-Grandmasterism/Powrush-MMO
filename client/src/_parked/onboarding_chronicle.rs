/*!
 * Onboarding Chronicle UI — Powrush-MMO v20.4
 *
 * Combined client visualization for the Onboarding Chronicle + Humble Beginnings Mirror.
 * Now includes "Current Realm Conflict Legacy" section for players joining during active inter-realm wars/tensions.
 * Mercy-gated narrative context so late joiners feel grounded rather than blindsided.
 * Integrates with diplomacy events and spectator legacy threads.
 * Sovereign freedom preserved: players can choose to attune to war legacy or focus on their personal humble beginnings.
 *
 * Thunder locked in. Yoi ⚔️
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::spectator_legacy_thread_viz::LegacyThread;

#[derive(Resource, Default)]
pub struct OnboardingChronicleUIState {
    pub show: bool,
    pub chronicle_entries: Vec<OnboardingChronicleEntry>,
    pub selected_entry: Option<usize>,
    // v20.4: War context for late joiners
    pub current_war_legacy: Option<WarConflictLegacy>,
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

// v20.4: War context data (populated from InterRealmDiplomacyUpdateEvent)
#[derive(Clone, Debug, Default)]
pub struct WarConflictLegacy {
    pub realm_a: u8,
    pub realm_b: u8,
    pub summary: String,
    pub redemption_score: f32,
    pub forgiveness_wave_active: bool,
    pub linked_legacy_thread_ids: Vec<u64>,
    pub mercy_context: String,
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

    egui::Window::new("Onboarding Chronicle — Humble Beginnings Mirror + Realm Legacy")
        .default_pos([120.0, 100.0])
        .default_size([580.0, 620.0])
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Your First Steps into the Eternal Flow");
            ui.label("These early moments are now part of your living Legacy Threads.");
            ui.separator();

            // v20.4: Current Realm Conflict Legacy section (for late joiners during wars)
            if let Some(war) = &state.current_war_legacy {
                ui.colored_label(egui::Color32::from_rgb(255, 180, 100), "⚠️ CURRENT REALM CONFLICT LEGACY");
                ui.label(format!("Realms {} ↔ {} | Redemption Potential: {:.1}%", war.realm_a, war.realm_b, war.redemption_score * 100.0));
                ui.label(&war.summary);
                if war.forgiveness_wave_active {
                    ui.colored_label(egui::Color32::from_rgb(100, 200, 255), "Forgiveness Wave active — Mercy path available");
                }
                ui.label(&war.mercy_context);

                ui.horizontal(|ui| {
                    if ui.button("Attune to War Legacy (Open Spectator Threads)").clicked() {
                        // In full integration: open spectator_legacy_thread_viz with linked threads
                        // This is a sovereign choice
                    }
                    if ui.button("Focus on My Humble Beginnings (Sovereign Choice)").clicked() {
                        // Player chooses to prioritize personal onboarding chronicle
                        state.current_war_legacy = None; // Optional: hide war context temporarily
                    }
                });

                ui.separator();
            }

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
                            }
                        });
                }
            });

            ui.separator();
            if ui.button("Close Chronicle (Sovereign Choice)").clicked() {
                state.show = false;
            }
        });
}

// Helper to populate from server data (humble beginnings)
pub fn populate_onboarding_chronicle(
    state: &mut OnboardingChronicleUIState,
    entries: Vec<OnboardingChronicleEntry>,
) {
    state.chronicle_entries = entries;
    state.show = true;
}

// v20.4: Populate war conflict legacy for late joiners (called from diplomacy reactive system or onboarding update)
pub fn populate_war_conflict_legacy(
    state: &mut OnboardingChronicleUIState,
    war: WarConflictLegacy,
) {
    state.current_war_legacy = Some(war);
    state.show = true; // Auto-open chronicle with war context for late joiners
}

// Thunder locked in. Yoi ⚔️
// End of client/src/onboarding_chronicle.rs v20.4 (War Context + Sovereign Freedom)