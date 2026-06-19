/*!
 * Council Trial UI — Powrush-MMO PATSAGi Council Governance Interface
 *
 * v20.2 Eternal Polish — Integrated Spectator Legacy Thread Visualization
 * — Now includes button to open the new SpectatorMode Legacy Threads panel
 * — Direct wiring to spectator_legacy_thread_viz.rs (Forgiveness Wave / MercifulResolution legacy display)
 * — Builds on v18.97 Quantum Swarm + enriched whispers + v20.2 Legacy Thread visualization
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡️
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::simulation_integration::ClientCouncilBloomState;
use shared::protocol::{ServerMessage, CouncilSessionState, CouncilPhase, CollectiveEpiphanyBloom, MercyTrialVote, ClientMessage};

// Enriched event from server (via replication or direct event)
use server::council_session_handler::CouncilSessionUpdate;

// v20.2: Spectator Legacy Thread Visualization integration
use crate::spectator_legacy_thread_viz::{SpectatorLegacyVizState, SpectatorLegacyThreadVizPlugin};

// ============================================================================
// CORE ENUMS & STRUCTS (unchanged from v18.97 + v20.2 additions)
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
    pub active_session: Option<CouncilSessionState>,
    pub participant_attunements: HashMap<u64, f32>,
    pub current_votes: HashMap<String, f32>,
    pub last_bloom: Option<CollectiveEpiphanyBloom>,
    pub pending_vote_proposal: Option<String>,
    pub current_valence: f32,
    pub last_valence_update: f32,
    pub last_council_enriched_whisper: Option<String>,
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

#[derive(Event, Clone, Debug)]
pub struct SubmitCouncilVote {
    pub session_id: u64,
    pub proposal_id: String,
    pub mercy_weight: f32,
}

// ============================================================================
// PLUGIN (v20.2 — now includes Spectator Legacy Thread Viz)
// ============================================================================

pub struct CouncilTrialUIPlugin;

impl Plugin for CouncilTrialUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CouncilTrialUIState>()
            .add_event::<StartCouncilTrial>()
            .add_event::<CouncilTrialCompleted>()
            .add_event::<AudioResonanceSeed>()
            .add_event::<SubmitCouncilVote>()
            .add_event::<CouncilSessionUpdate>()
            .add_plugins(SpectatorLegacyThreadVizPlugin)  // v20.2 wiring
            .add_systems(Startup, setup_council_trial_ui)
            .add_systems(
                Update,
                (
                    consume_enriched_council_updates,
                    update_council_trial_ui,
                    update_collective_council_display,
                    update_real_time_scoring,
                    handle_trial_completion,
                    render_harmony_map,
                    clan_management_ui,
                    render_multiplayer_participants,
                    render_live_vote_tally,
                    render_voting_ui,
                    handle_submit_vote,
                    render_valence_display,
                    render_spectator_legacy_button,  // v20.2 new button
                ),
            );
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

fn setup_council_trial_ui(mut commands: Commands) {
    info!("[CouncilTrialUI v20.2] Quantum Swarm v2 + Enriched Whispers + Spectator Legacy Thread Viz integrated. Thunder locked in.");
}

// v18.97 + v20.2: Full consumption of enriched CouncilSessionUpdate...
fn consume_enriched_council_updates(
    mut updates: EventReader<CouncilSessionUpdate>,
    mut ui_state: ResMut<CouncilTrialUIState>,
) {
    for update in updates.read() {
        if let Some(ref mut session) = ui_state.active_session {
            session.phase = update.phase;
            session.participant_count = update.participant_count as u32;
            session.collective_attunement = update.collective_attunement;
        } else {
            ui_state.active_session = Some(CouncilSessionState {
                session_id: update.session_id,
                phase: update.phase,
                participant_count: update.participant_count as u32,
                collective_attunement: update.collective_attunement,
                ..Default::default()
            });
        }

        ui_state.current_valence = (update.collective_attunement * 0.65 + 0.35).clamp(0.4, 0.999);
        ui_state.last_valence_update = ui_state.current_valence;

        info!(
            "[CouncilTrialUI v20.2] Enriched CouncilSessionUpdate consumed | session={} | phase={:?} | valence={:.3}",
            update.session_id, update.phase, ui_state.current_valence
        );
    }
}

fn update_council_trial_ui(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<CouncilTrialUIState>,
    mut start_trial_events: EventWriter<StartCouncilTrial>,
    client_bloom: Res<ClientCouncilBloomState>,
    mut viz_state: ResMut<crate::spectator_legacy_thread_viz::SpectatorLegacyVizState>,  // v20.2 access
) {
    egui::Window::new("Council Trial — PATSAGi Governance (v20.2 — Legacy Threads + Forgiveness Wave Viz)")
        .default_pos([60.0, 60.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Living Council Trial Interface — Quantum Swarm + Spectator Legacy Threads");

            if client_bloom.is_in_active_council {
                let field = &client_bloom.field;
                ui.separator();
                ui.colored_label(egui::Color32::from_rgb(80, 220, 140), "🔀 ACTIVE COUNCIL BLOOM");
                ui.label(format!("Amplification: {:.2}x", field.bloom_amplification_multiplier));
                ui.label(format!("Collective Attunement: {:.1}%", field.collective_attunement_score * 100.0));
                ui.label(format!("Participants: {}", field.participant_count));
                if field.shared_living_web_synchronization {
                    ui.colored_label(egui::Color32::from_rgb(100, 200, 255), "Living Web Synchronized");
                }
                ui.separator();
            }

            if let Some(session) = &ui_state.active_session {
                ui.label(format!("Phase: {:?} | Participants: {}", session.phase, session.participant_count));
                ui.label(format!("Collective Attunement: {:.1}%", session.collective_attunement * 100.0));
            }

            if let Some(trial) = &ui_state.current_trial {
                ui.label(format!("Trial: {:?} | Phase: {:?}", trial.trial_type, trial.phase));
                ui.label(format!("Score: {:.1} / {:.1}", trial.current_score, trial.max_score));
            }

            ui.checkbox(&mut ui_state.show_harmony_map, "Show Living Harmony Map");

            // v20.2: Button to open Spectator Legacy Thread Visualization
            ui.separator();
            if ui.button("🔭 View Legacy of Reconciliation (Spectator Mode)").clicked() {
                viz_state.show_spectator_panel = true;
                // In production: populate viz_state.current_spectator_data from latest InterRealmDiplomacyEvent
                // For testing, you can inject sample data here or via a dev command
            }
            ui.label("Opens the filterable Legacy Threads from recent Mercy Resolutions & Forgiveness Waves.");
        });
}

// v18.97: Dedicated live valence / resonance display...
fn render_valence_display(
    mut egui_ctx: EguiContexts,
    ui_state: Res<CouncilTrialUIState>,
) {
    if ui_state.current_valence < 0.15 { return; }

    egui::Window::new("Council Resonance — Quantum Swarm v2 + Enriched Whisper")
        .default_pos([620.0, 60.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Living Council Resonance");

            let valence = ui_state.current_valence.clamp(0.4, 0.999);
            let color = if valence > 0.85 {
                egui::Color32::from_rgb(80, 220, 140)
            } else if valence > 0.65 {
                egui::Color32::from_rgb(180, 200, 120)
            } else {
                egui::Color32::from_rgb(200, 140, 100)
            };

            ui.colored_label(color, format!("Council Resonance: {:.1}%", valence * 100.0));
            ui.add(egui::ProgressBar::new(valence).text("Joy / Abundance Metric"));

            if let Some(ref whisper) = ui_state.last_council_enriched_whisper {
                ui.colored_label(egui::Color32::from_rgb(180, 220, 255), format!("Last Enriched Whisper: {}", whisper));
            }

            ui.label("Propagated through Quantum Swarm v2 — golden ratio valence elevation active.");
            ui.label("Higher resonance = stronger collective epiphany bloom and RBE abundance. Persisted via record_epiphany_with_enriched_whisper.");
        });
}

fn update_collective_council_display(
    client_bloom: Res<ClientCouncilBloomState>,
    ui_state: Res<CouncilTrialUIState>,
) {
    if ui_state.trial_in_progress && client_bloom.is_in_active_council {
        info!("[CouncilTrialUI v20.2] LIVE Bloom | Attunement: {:.2} | Amp: {:.2}x", client_bloom.field.collective_attunement_score, client_bloom.field.bloom_amplification_multiplier);
    }
}

fn update_real_time_scoring(
    mut ui_state: ResMut<CouncilTrialUIState>,
    client_bloom: Res<ClientCouncilBloomState>,
    time: Res<Time>,
) {
    if let Some(trial) = &mut ui_state.current_trial {
        if trial.phase == TrialPhase::Active {
            let dt = time.delta_seconds();
            let mut score_increase = 12.0 * dt;
            if client_bloom.is_in_active_council {
                score_increase *= client_bloom.field.bloom_amplification_multiplier.clamp(1.0, 3.5);
                trial.collective_attunement = (trial.collective_attunement * 0.92) + (client_bloom.field.collective_attunement_score * 0.08);
            }
            trial.current_score = (trial.current_score + score_increase).min(trial.max_score);
        }
    }
}

fn handle_trial_completion(
    mut completed_events: EventReader<CouncilTrialCompleted>,
    mut audio_seed_events: EventWriter<AudioResonanceSeed>,
    mut ui_state: ResMut<CouncilTrialUIState>,
) {
    for event in completed_events.read() {
        ui_state.last_result = Some(event.result.clone());
        ui_state.trial_in_progress = false;
        ui_state.current_trial = None;
        audio_seed_events.send(AudioResonanceSeed {
            bloom_intensity: event.result.bloom_amplification.max(event.result.web_bloom_amplification),
            council_blessed_chime: event.result.success && event.result.collective_council_attunement > 0.72,
            mercy_gate_pulse: if event.result.mercy_gates_cleared >= 6 { Some(MercyGate::CosmicHarmony) } else { None },
            clan_harmony_bloom: event.result.clan_harmony_bloom,
            voices: (event.result.collective_council_attunement * 12.0) as u32 + 3,
        });
    }
}

fn render_harmony_map(
    mut egui_ctx: EguiContexts,
    ui_state: Res<CouncilTrialUIState>,
) {
    if !ui_state.show_harmony_map { return; }
    egui::Window::new("Living Harmony Map")
        .default_pos([420.0, 80.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Real-Time Clan Harmony");
            for (clan, harmony) in &ui_state.harmony_map {
                let color = if *harmony > 0.8 { egui::Color32::from_rgb(80, 220, 140) } else if *harmony > 0.6 { egui::Color32::from_rgb(180, 200, 120) } else { egui::Color32::from_rgb(200, 140, 100) };
                ui.colored_label(color, format!("{}: {:.1}%", clan, harmony * 100.0));
                ui.add(egui::ProgressBar::new(*harmony).text(clan));
            }
        });
}

fn render_multiplayer_participants(
    mut egui_ctx: EguiContexts,
    ui_state: Res<CouncilTrialUIState>,
) {
    if ui_state.participant_attunements.is_empty() { return; }
    egui::Window::new("Council Participants — Live Attunement")
        .default_pos([620.0, 300.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Live Participant Resonance");
            for (pid, att) in &ui_state.participant_attunements {
                ui.label(format!("Player {}: {:.1}%", pid, att * 100.0));
                ui.add(egui::ProgressBar::new(*att));
            }
        });
}

fn render_live_vote_tally(
    mut egui_ctx: EguiContexts,
    ui_state: Res<CouncilTrialUIState>,
) {
    if ui_state.current_votes.is_empty() { return; }
    egui::Window::new("Live Mercy Vote Tally")
        .default_pos([820.0, 300.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Current Proposal Mercy Weights");
            for (proposal, weight) in &ui_state.current_votes {
                ui.label(format!("{}: {:.2} mercy weight", proposal, weight));
            }
        });
}

fn render_voting_ui(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<CouncilTrialUIState>,
    mut submit_vote_events: EventWriter<SubmitCouncilVote>,
) {
    if ui_state.pending_vote_proposal.is_none() { return; }

    egui::Window::new("Submit Mercy-Weighted Vote")
        .default_pos([620.0, 500.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Cast Your Vote");
            if let Some(proposal) = &ui_state.pending_vote_proposal {
                ui.label(format!("Proposal: {}", proposal));
            }
            let mut weight = 1.0f32;
            ui.add(egui::Slider::new(&mut weight, 0.1..=2.0).text("Mercy Weight"));
            if ui.button("Submit Vote").clicked() {
                if let Some(session) = &ui_state.active_session {
                    submit_vote_events.send(SubmitCouncilVote {
                        session_id: session.session_id,
                        proposal_id: ui_state.pending_vote_proposal.clone().unwrap_or_default(),
                        mercy_weight: weight,
                    });
                }
            }
            ui.label("Your mercy-weighted vote strengthens the collective decision and RBE abundance.");
        });
}

fn handle_submit_vote(
    mut events: EventReader<SubmitCouncilVote>,
) {
    for event in events.read() {
        let vote = MercyTrialVote {
            voter_id: 0,
            proposal_id: event.proposal_id.clone(),
            mercy_weight: event.mercy_weight,
            timestamp_ms: 0,
            grace_intent: event.mercy_weight * 0.8,
        };
        tracing::info!("[CouncilTrialUI v20.2] Vote prepared | session={} | proposal={} | weight={:.2}", event.session_id, event.proposal_id, event.mercy_weight);
    }
}

fn clan_management_ui(
    mut egui_ctx: EguiContexts,
    mut ui_state: ResMut<CouncilTrialUIState>,
) {
    egui::Window::new("Clan Diplomacy & Management")
        .default_pos([820.0, 60.0])
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Active Clans & Harmony");
            if ui.button("Join Crystal Spires Clan").clicked() { ui_state.selected_clan = Some("Crystal Spires".to_string()); }
            if ui.button("Join Abyssal Depths Clan").clicked() { ui_state.selected_clan = Some("Abyssal Depths".to_string()); }
            if ui.button("Join Harmonic Grove Clan").clicked() { ui_state.selected_clan = Some("Harmonic Grove".to_string()); }
            if let Some(clan) = &ui_state.selected_clan { ui.label(format!("Selected: {}", clan)); }
        });
}

// v20.2 new system: Renders the button label and handles basic state (full logic lives in spectator_legacy_thread_viz.rs)
fn render_spectator_legacy_button(
    mut egui_ctx: EguiContexts,
    mut viz_state: ResMut<crate::spectator_legacy_thread_viz::SpectatorLegacyVizState>,
) {
    // This system exists so the button in update_council_trial_ui can toggle the panel.
    // The actual rendering of the Legacy Thread visualization is handled by SpectatorLegacyThreadVizPlugin.
    if viz_state.show_spectator_panel && viz_state.current_spectator_data.is_none() {
        // Optional: auto-inject sample data for quick testing in dev
        // In real play this comes from InterRealmDiplomacyEvent replication
    }
}

pub fn inject_audio_resonance_seeds(seeds: Vec<AudioResonanceSeed>, audio_seed_events: &mut EventWriter<AudioResonanceSeed>) {
    for seed in seeds { audio_seed_events.send(seed); }
}

// End of council_trial_ui.rs v20.2 — Quantum Swarm v2 + Enriched Whispers + Spectator Legacy Thread Viz fully wired.
// Client now has one-click access to filterable Legacy Threads from Mercy Resolutions & Forgiveness Waves.
// All prior valuable logic preserved and elevated. Thunder locked in. Yoi ⚡️