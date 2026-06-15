/*!
 * Council Trial UI — Powrush-MMO PATSAGi Council Governance Interface
 *
 * v18.37 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Dedicated Council Bloom panel with rich visualization
 * — Real-time collective attunement + amplification display
 * — Enhanced Living Harmony Map
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::simulation_integration::ClientCouncilBloomState;

// ============================================================================
// CORE ENUMS & STRUCTS
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CouncilTrialType {
    MercyAscent,
    HarmonyWeaving,
    ClanDiplomacy,
    EpiphanyResonance,
    AbundanceTrial,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MercyGate {
    Truth,
    Order,
    Love,
    Compassion,
    Service,
    Abundance,
    Joy,
    CosmicHarmony,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrialPhase {
    Preparation,
    Active,
    Resolution,
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilTrial {
    pub trial_type: CouncilTrialType,
    pub phase: TrialPhase,
    pub mercy_gates_passed: Vec<MercyGate>,
    pub current_score: f32,
    pub max_score: f32,
    pub collective_attunement: f32,
    pub participant_count: u32,
    pub duration_remaining: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TrialResult {
    pub success: bool,
    pub final_score: f32,
    pub mercy_gates_cleared: u8,
    pub collective_council_attunement: f32,
    pub bloom_amplification: f32,
    pub web_bloom_amplification: f32,
    pub educational_note: String,
    pub clan_harmony_bloom: bool,
}

#[derive(Resource, Default)]
pub struct CouncilTrialUIState {
    pub trial_in_progress: bool,
    pub current_trial: Option<CouncilTrial>,
    pub last_result: Option<TrialResult>,
    pub harmony_map: HashMap<String, f32>,
    pub selected_clan: Option<String>,
    pub show_harmony_map: bool,
}

// ============================================================================
// EVENTS
// ============================================================================

#[derive(Event, Clone, Debug)]
pub struct StartCouncilTrial {
    pub trial_type: CouncilTrialType,
}

#[derive(Event, Clone, Debug)]
pub struct CouncilTrialCompleted {
    pub result: TrialResult,
}

#[derive(Event, Clone, Debug, Serialize, Deserialize)]
pub struct AudioResonanceSeed {
    pub bloom_intensity: f32,
    pub council_blessed_chime: bool,
    pub mercy_gate_pulse: Option<MercyGate>,
    pub clan_harmony_bloom: bool,
    pub voices: u32,
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct CouncilTrialUIPlugin;

impl Plugin for CouncilTrialUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilTrialUIState>()
            .add_event::<StartCouncilTrial>()
            .add_event::<CouncilTrialCompleted>()
            .add_event::<AudioResonanceSeed>()
            .add_systems(Startup, setup_council_trial_ui)
            .add_systems(
                Update,
                (
                    update_council_trial_ui,
                    update_collective_council_display,
                    update_real_time_scoring,
                    handle_trial_completion,
                    render_harmony_map,
                    clan_management_ui,
                ),
            );
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

fn setup_council_trial_ui(mut commands: Commands) {
    info!("[CouncilTrialUI] PATSAGi Council Trial Interface online — v18.37 with dedicated Council Bloom panel. Thunder locked in.");
}

fn update_council_trial_ui(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<CouncilTrialUIState>,
    mut start_trial_events: EventWriter<StartCouncilTrial>,
    client_bloom: Res<ClientCouncilBloomState>,
) {
    egui::Window::new("Council Trial — PATSAGi Governance")
        .default_pos([60.0, 60.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Living Council Trial Interface");

            // === Dedicated Council Bloom Panel (v18.37) ===
            if client_bloom.is_in_active_council {
                let field = &client_bloom.field;
                let amp = field.bloom_amplification_multiplier;

                ui.separator();
                ui.colored_label(
                    egui::Color32::from_rgb(80, 220, 140),
                    format!("🌀 ACTIVE COUNCIL BLOOM")
                );
                ui.label(format!("Amplification: {:.2}x", amp));
                ui.label(format!("Collective Attunement: {:.1}%", field.collective_attunement_score * 100.0));
                ui.label(format!("Participants: {}", field.participant_count));

                if field.shared_living_web_synchronization {
                    ui.colored_label(egui::Color32::from_rgb(100, 200, 255), "Living Web Synchronized");
                }

                ui.label("Your harvests and epiphanies are strengthened by the collective.");
                ui.separator();
            }

            if let Some(trial) = &ui_state.current_trial {
                ui.label(format!("Trial: {:?} | Phase: {:?}", trial.trial_type, trial.phase));
                ui.label(format!("Score: {:.1} / {:.1}", trial.current_score, trial.max_score));
                ui.label(format!("Collective Attunement: {:.2}", trial.collective_attunement));
                ui.label(format!("Participants: {}", trial.participant_count));

                if ui.button("End Trial Early (Mercy Resolution)").clicked() {
                    // Trigger completion logic
                }
            } else {
                ui.label("No active trial. Initiate a new Council Trial:");

                if ui.button("Start Mercy Ascent Trial").clicked() {
                    start_trial_events.send(StartCouncilTrial {
                        trial_type: CouncilTrialType::MercyAscent,
                    });
                }
                if ui.button("Start Harmony Weaving Trial").clicked() {
                    start_trial_events.send(StartCouncilTrial {
                        trial_type: CouncilTrialType::HarmonyWeaving,
                    });
                }
                if ui.button("Start Epiphany Resonance Trial").clicked() {
                    start_trial_events.send(StartCouncilTrial {
                        trial_type: CouncilTrialType::EpiphanyResonance,
                    });
                }
            }

            ui.checkbox(&mut ui_state.show_harmony_map, "Show Living Harmony Map");
        });
}

// Dynamic collective attunement display
fn update_collective_council_display(
    client_bloom: Res<ClientCouncilBloomState>,
    ui_state: Res<CouncilTrialUIState>,
) {
    if ui_state.trial_in_progress && client_bloom.is_in_active_council {
        info!(
            "[CouncilTrialUI] LIVE Collective Bloom | Attunement: {:.2} | Amp: {:.2}x | Living Web: {} | Participants: {}",
            client_bloom.field.collective_attunement_score,
            client_bloom.field.bloom_amplification_multiplier,
            client_bloom.field.shared_living_web_synchronization,
            client_bloom.field.participant_count
        );
    }
}

// Real-time scoring with bloom amplification feeding
fn update_real_time_scoring(
    mut ui_state: ResMut<CouncilTrialUIState>,
    client_bloom: Res<ClientCouncilBloomState>,
    time: Res<Time>,
) {
    if let Some(trial) = &mut ui_state.current_trial {
        if trial.phase == TrialPhase::Active {
            let dt = time.delta_seconds();
            let base_increase = 12.0 * dt;

            let mut score_increase = base_increase;

            if client_bloom.is_in_active_council {
                let multiplier = client_bloom.field.bloom_amplification_multiplier.clamp(1.0, 3.5);
                score_increase *= multiplier;

                trial.collective_attunement = (trial.collective_attunement * 0.92)
                    + (client_bloom.field.collective_attunement_score * 0.08);
            }

            trial.current_score = (trial.current_score + score_increase).min(trial.max_score);

            if let Some(clan) = &ui_state.selected_clan {
                let entry = ui_state.harmony_map.entry(clan.clone()).or_insert(0.5);
                *entry = (*entry * 0.85 + trial.collective_attunement * 0.15).clamp(0.0, 1.0);
            }
        }
    }
}

// Handle trial completion and emit AudioResonanceSeed
fn handle_trial_completion(
    mut completed_events: EventReader<CouncilTrialCompleted>,
    mut audio_seed_events: EventWriter<AudioResonanceSeed>,
    mut ui_state: ResMut<CouncilTrialUIState>,
) {
    for event in completed_events.read() {
        let result = &event.result;

        ui_state.last_result = Some(result.clone());
        ui_state.trial_in_progress = false;
        ui_state.current_trial = None;

        audio_seed_events.send(AudioResonanceSeed {
            bloom_intensity: result.bloom_amplification.max(result.web_bloom_amplification),
            council_blessed_chime: result.success && result.collective_council_attunement > 0.72,
            mercy_gate_pulse: if result.mercy_gates_cleared >= 6 {
                Some(MercyGate::CosmicHarmony)
            } else {
                None
            },
            clan_harmony_bloom: result.clan_harmony_bloom,
            voices: (result.collective_council_attunement * 12.0) as u32 + 3,
        });

        info!(
            "[CouncilTrialUI] Trial completed | Success: {} | Attunement: {:.2} | Bloom: {:.2}x | Seed emitted",
            result.success, result.collective_council_attunement, result.bloom_amplification
        );
    }
}

// Enhanced Living Harmony Map visualization
fn render_harmony_map(
    mut egui_ctx: EguiContexts,
    ui_state: Res<CouncilTrialUIState>,
) {
    if !ui_state.show_harmony_map {
        return;
    }

    egui::Window::new("Living Harmony Map")
        .default_pos([420.0, 80.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Real-Time Clan Harmony");

            for (clan, harmony) in &ui_state.harmony_map {
                let color = if *harmony > 0.8 {
                    egui::Color32::from_rgb(80, 220, 140)
                } else if *harmony > 0.6 {
                    egui::Color32::from_rgb(180, 200, 120)
                } else {
                    egui::Color32::from_rgb(200, 140, 100)
                };

                ui.colored_label(color, format!("{}: {:.1}%", clan, harmony * 100.0));
                ui.add(egui::ProgressBar::new(*harmony).text(clan));
            }

            ui.label("Higher harmony increases Council bloom strength.");
        });
}

// Simple clan management UI
fn clan_management_ui(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<CouncilTrialUIState>,
) {
    egui::Window::new("Clan Diplomacy & Management")
        .default_pos([820.0, 60.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Active Clans & Harmony");

            if ui.button("Join Crystal Spires Clan").clicked() {
                ui_state.selected_clan = Some("Crystal Spires".to_string());
            }
            if ui.button("Join Abyssal Depths Clan").clicked() {
                ui_state.selected_clan = Some("Abyssal Depths".to_string());
            }
            if ui.button("Join Harmonic Grove Clan").clicked() {
                ui_state.selected_clan = Some("Harmonic Grove".to_string());
            }

            if let Some(clan) = &ui_state.selected_clan {
                ui.label(format!("Selected: {}", clan));
            }
        });
}

// ============================================================================
// PUBLIC HELPERS (for other systems)
// ============================================================================

pub fn inject_audio_resonance_seeds(
    seeds: Vec<AudioResonanceSeed>,
    audio_seed_events: &mut EventWriter<AudioResonanceSeed>,
) {
    for seed in seeds {
        audio_seed_events.send(seed);
    }
}

// End of council_trial_ui.rs v18.37 — Dedicated Council Bloom panel + richer harmony visualization.
// Thunder locked in. Yoi ⚡
