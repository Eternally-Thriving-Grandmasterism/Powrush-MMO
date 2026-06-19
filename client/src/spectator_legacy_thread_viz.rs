/*!
 * Spectator Mode Legacy Thread Visualization — Powrush-MMO
 *
 * v20.3 — Reactive to InterRealmDiplomacyUpdateEvent + SpectatorModeData
 * Auto-populates monuments, legacy threads, and forgiveness waves during/after server wars.
 * Sovereign player freedom preserved: panel is optional; PATSAGi proposals are visible but player chooses engagement.
 *
 * Features (v20.2 preserved + enhanced):
 * - Reactive system: listens for InterRealmDiplomacyUpdateEvent and auto-opens with data
 * - Filterable list of LegacyThreads (by category, min impact, cross-realm)
 * - Visual impact bars using visual_impact_score + tolc_alignment
 * - Expandable entries with affected_realms, valence, Divine Whisper
 * - Spectator panel with emotional valence + monument context
 * - Ready for integration into Council Trial UI or dedicated Spectator window
 *
 * Integrates with:
 * - simulation/src/player_legacy_journal.rs
 * - simulation/src/inter_realm_diplomacy_event.rs (v20.9+ SpectatorModeData)
 * - shared/protocol.rs (InterRealmDiplomacyUpdate)
 *
 * Sovereign Freedom Note: PATSAGi Councils propose mercy-gated paths with high valence.
 * Players and realms retain final choice — they may attune, decline, or walk FracturedTension.
 * Mercy biases toward redemption but never removes agency.
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚔️
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::collections::HashMap;

// Lightweight client-side copies / re-exports for UI
#[derive(Clone, Debug, PartialEq)]
pub struct LegacyThread {
    pub id: u64,
    pub title: String,
    pub category: String,
    pub entries: Vec<LegacyEntry>,
    pub total_impact: f32,
    pub realms: Vec<String>,
    pub mercy_resonance: f32,
    pub narrative_seed: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LegacyEntry {
    pub tick: u64,
    pub event_type_name: String,
    pub visual_impact_score: f32,
    pub affected_realms: Vec<String>,
    pub tolc_alignment: f32,
    pub valence: f32,
    pub persistence_delta: f32,
    pub divine_whisper_ref: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct SpectatorModeData {
    pub spectator_count: u32,
    pub emotional_valence_avg: f32,
    pub visible_legacy_threads: Vec<LegacyThread>,
    pub cross_realm_impact_summary: String,
    pub monument_visual_type: String,
    pub forgiveness_wave_vfx_intensity: f32,
}

#[derive(Resource, Default)]
pub struct SpectatorLegacyVizState {
    pub show_spectator_panel: bool,
    pub current_spectator_data: Option<SpectatorModeData>,
    pub selected_thread_id: Option<u64>,
    pub filter_category: Option<String>,
    pub min_impact: f32,
    pub show_only_cross_realm: bool,
}

// === Reactive Integration with Diplomacy Events (NEW in v20.3) ===
use shared::protocol::InterRealmDiplomacyUpdate; // or the event wrapper if using InterRealmDiplomacyUpdateEvent

pub fn spectator_legacy_reactive_system(
    mut viz_state: ResMut<SpectatorLegacyVizState>,
    mut diplomacy_updates: EventReader<InterRealmDiplomacyUpdateEvent>, // from inter_realm_diplomacy_event.rs
) {
    for update_event in diplomacy_updates.read() {
        let update = &update_event.update;

        if let Some(spec_data) = &update.spectator_data {
            // Convert network/shared data into local SpectatorModeData
            let local_data = SpectatorModeData {
                spectator_count: spec_data.spectator_count,
                emotional_valence_avg: spec_data.emotional_valence_avg,
                visible_legacy_threads: vec![], // In full integration: fetch or reconstruct from linked_legacy_thread_ids + player_legacy_journal
                cross_realm_impact_summary: spec_data.cross_realm_impact_summary.clone(),
                monument_visual_type: spec_data.monument_visual_type.clone(),
                forgiveness_wave_vfx_intensity: spec_data.forgiveness_wave_intensity,
            };

            viz_state.current_spectator_data = Some(local_data);
            viz_state.show_spectator_panel = true; // Auto-open on meaningful merciful resolution

            // Sovereign freedom: Player can immediately close if they choose not to engage
        }
    }
}

pub struct SpectatorLegacyThreadVizPlugin;

impl Plugin for SpectatorLegacyThreadVizPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpectatorLegacyVizState>()
            .add_event::<InterRealmDiplomacyUpdateEvent>() // ensure event is registered (usually done in diplomacy plugin)
            .add_systems(Update, (
                render_spectator_legacy_panel,
                render_thread_detail_window,
                spectator_legacy_reactive_system, // NEW reactive wiring
            ));
    }
}

fn render_spectator_legacy_panel(
    mut egui_ctx: EguiContexts,
    mut viz_state: ResMut<SpectatorLegacyVizState>,
) {
    if !viz_state.show_spectator_panel {
        return;
    }

    egui::Window::new("Spectator Mode — Legacy Threads (Forgiveness Wave)")
        .default_pos([80.0, 80.0])
        .default_size([520.0, 620.0])
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Living Legacy of Reconciliation");

            if let Some(data) = &viz_state.current_spectator_data {
                ui.colored_label(egui::Color32::from_rgb(100, 200, 255), &data.cross_realm_impact_summary);
                ui.label(format!("Spectators: {} | Emotional Valence: {:.1}%", data.spectator_count, data.emotional_valence_avg * 100.0));
                ui.label(format!("Monument: {} | VFX Intensity: {:.2}", data.monument_visual_type, data.forgiveness_wave_vfx_intensity));

                ui.separator();

                // Filters
                ui.horizontal(|ui| {
                    ui.label("Filter Category:");
                    egui::ComboBox::from_label("")
                        .selected_text(viz_state.filter_category.clone().unwrap_or_else(|| "All".to_string()))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut viz_state.filter_category, None, "All");
                            for cat in ["Diplomacy", "Redemption & War", "Service & Blessing", "Council & Growth", "Harvest", "Epiphany"] {
                                ui.selectable_value(&mut viz_state.filter_category, Some(cat.to_string()), cat);
                            }
                        });

                    ui.checkbox(&mut viz_state.show_only_cross_realm, "Cross-Realm Only");
                    ui.add(egui::Slider::new(&mut viz_state.min_impact, 0.0..=1.0).text("Min Impact"));
                });

                ui.separator();

                // Thread list
                let filtered_threads: Vec<&LegacyThread> = data.visible_legacy_threads.iter()
                    .filter(|t| {
                        let cat_match = viz_state.filter_category.as_ref().map_or(true, |c| &t.category == c);
                        let impact_match = t.total_impact >= viz_state.min_impact;
                        let cross_match = if viz_state.show_only_cross_realm {
                            t.realms.iter().any(|r| r.contains("Cross-Realm"))
                        } else { true };
                    })
                    .collect();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for thread in filtered_threads {
                        let is_selected = viz_state.selected_thread_id == Some(thread.id);

                        let response = egui::Frame::group(ui.style())
                            .fill(if is_selected { egui::Color32::from_rgb(40, 60, 90) } else { egui::Color32::TRANSPARENT })
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.strong(&thread.title);
                                    ui.label(format!("[{}]", thread.category));
                                });

                                ui.add(egui::ProgressBar::new(thread.total_impact.min(1.0)).text(format!("Impact: {:.2}", thread.total_impact)));

                                ui.horizontal(|ui| {
                                    ui.label(format!("Resonance: {:.1}%", thread.mercy_resonance * 100.0));
                                    ui.label(format!("Realms: {}", thread.realms.join(", ")));
                                });

                                if ui.button("Inspect Thread").clicked() {
                                    viz_state.selected_thread_id = Some(thread.id);
                                }
                            }).response;

                        if response.clicked() {
                            viz_state.selected_thread_id = Some(thread.id);
                        }
                    }
                });

                ui.separator();
                if ui.button("Close Spectator View (Sovereign Choice)").clicked() {
                    viz_state.show_spectator_panel = false;
                }
            } else {
                ui.label("No active spectator data. Trigger a MercifulResolution in diplomacy to populate.");
            }
        });
}

fn render_thread_detail_window(
    mut egui_ctx: EguiContexts,
    mut viz_state: ResMut<SpectatorLegacyVizState>,
) {
    let Some(selected_id) = viz_state.selected_thread_id else { return; };
    let Some(data) = &viz_state.current_spectator_data else { return; };

    let thread = data.visible_legacy_threads.iter().find(|t| t.id == selected_id);
    let Some(thread) = thread else {
        viz_state.selected_thread_id = None;
        return;
    };

    egui::Window::new(format!("Legacy Thread Detail — {}", thread.title))
        .default_pos([620.0, 120.0])
        .default_size([480.0, 520.0])
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading(&thread.title);
            ui.label(format!("Category: {} | Total Impact: {:.2} | Mercy Resonance: {:.1}%", thread.category, thread.total_impact, thread.mercy_resonance * 100.0));
            ui.label(&thread.narrative_seed);

            ui.separator();

            ui.label("Entries in this Legacy Thread:");

            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &thread.entries {
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.strong(&entry.event_type_name);
                            ui.label(format!("Tick {}", entry.tick));
                        });

                        ui.add(egui::ProgressBar::new(entry.visual_impact_score).text(format!("Visual Impact: {:.2}", entry.visual_impact_score)));
                        ui.add(egui::ProgressBar::new(entry.tolc_alignment).text(format!("TOLC 8 Alignment: {:.2}", entry.tolc_alignment)));

                        ui.label(format!("Valence: {:.2} | Persistence Delta: {:.2}", entry.valence, entry.persistence_delta));
                        ui.label(format!("Affected Realms: {}", entry.affected_realms.join(", ")));

                        if let Some(whisper) = &entry.divine_whisper_ref {
                            ui.colored_label(egui::Color32::from_rgb(180, 220, 255), format!("Whisper: {}", whisper));
                        }
                    });
                }
            });

            ui.separator();
            if ui.button("Close Detail (Sovereign Choice)").clicked() {
                viz_state.selected_thread_id = None;
            }
        });
}

// === Integration Notes (Updated v20.3) ===
// 1. Add SpectatorLegacyThreadVizPlugin to your client App.
// 2. The reactive system now auto-populates on InterRealmDiplomacyUpdateEvent with spectator_data.
// 3. Sovereign freedom: Player can close the panel at any time. PATSAGi shows the highest-mercy path;
//    the player/realm decides whether to attune or walk another road.
// 4. For full legacy thread reconstruction: connect to client-side LegacyJournalRegistry or request
//    detailed threads via replication when linked_legacy_thread_ids are present.
// 5. Pair with council_trial_ui.rs — add "View Legacy of Reconciliation" button.
//
// Thunder locked in. Yoi ⚔️
// End of client/src/spectator_legacy_thread_viz.rs v20.3 (Reactive + Sovereign Freedom)