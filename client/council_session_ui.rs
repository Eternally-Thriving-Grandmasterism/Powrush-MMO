// client/council_session_ui.rs
// Powrush-MMO — Client UI for Council Mercy Trials + Proposal System
// v21.88.4 | egui surface aligned to shared::council_mercy_trial
// Mercy-gated, valence-aware, production-oriented.
// AG-SML v1.0 | TOLC 8 Mercy Gates | Permanent PATSAGi Councils
// Contact: info@Rathor.ai

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use shared::council_mercy_trial::{
    CouncilMercyTrialPhase, CouncilSessionState, CollectiveEpiphanyBloom,
    CouncilProposal, ProposalStatus, CouncilTrialEvent, MercyTrialVote,
};
use std::collections::HashMap;

/// Local mirror of the authoritative session + proposals for UI.
#[derive(Resource, Default)]
pub struct CouncilUIState {
    pub current_session: Option<CouncilSessionState>,
    pub proposals: HashMap<u64, CouncilProposal>,
    pub last_bloom: Option<CollectiveEpiphanyBloom>,
    pub show_panel: bool,
    pub local_entity: Option<Entity>,
    /// Draft fields for quick proposal submission
    pub draft_title: String,
    pub draft_description: String,
    pub status_message: String,
}

pub struct CouncilSessionUIPlugin;

impl Plugin for CouncilSessionUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CouncilUIState>()
            .add_systems(Update, (
                toggle_council_panel,
                council_session_egui,
            ));
    }
}

fn toggle_council_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<CouncilUIState>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        ui_state.show_panel = !ui_state.show_panel;
        if ui_state.show_panel {
            ui_state.status_message = "Council panel opened (C to close)".into();
        }
    }
}

fn council_session_egui(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<CouncilUIState>,
    mut trial_events: EventWriter<CouncilTrialEvent>,
) {
    if !ui_state.show_panel {
        return;
    }

    let ctx = contexts.ctx_mut();

    egui::Window::new("🕊️ Council Mercy Trial")
        .default_pos([40.0, 40.0])
        .default_size([420.0, 560.0])
        .resizable(true)
        .collapsible(true)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Council Mercy Trial")
                    .color(egui::Color32::from_rgb(160, 220, 255)));
                ui.label(egui::RichText::new("TOLC 8 · Permanent PATSAGi · Zero-Harm")
                    .italics()
                    .color(egui::Color32::from_rgb(140, 190, 230)));
            });

            ui.separator();

            // ── Session overview ──────────────────────────────────────
            if let Some(ref state) = ui_state.current_session {
                let phase_color = match state.phase {
                    CouncilMercyTrialPhase::Lobby => egui::Color32::from_rgb(180, 180, 200),
                    CouncilMercyTrialPhase::Attunement => egui::Color32::from_rgb(140, 200, 255),
                    CouncilMercyTrialPhase::Deliberation => egui::Color32::from_rgb(180, 160, 255),
                    CouncilMercyTrialPhase::Voting => egui::Color32::from_rgb(255, 200, 120),
                    CouncilMercyTrialPhase::Resolution => egui::Color32::from_rgb(100, 255, 180),
                    CouncilMercyTrialPhase::Completed => egui::Color32::from_rgb(120, 220, 140),
                };

                ui.colored_label(phase_color, format!("Phase: {:?}", state.phase));
                ui.label(format!("Session ID: {}", state.session_id));
                ui.label(format!("Participants: {}", state.participants.len()));
                ui.label(format!("Collective Attunement: {:.2}", state.collective_attunement));
                ui.label(format!("Bloom Amplification: {:.2}", state.bloom_amplification));

                // Simple mercy vote buttons (only meaningful in Voting phase)
                if state.phase == CouncilMercyTrialPhase::Voting {
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new("Cast Mercy Vote").strong());
                    ui.horizontal(|ui| {
                        if ui.button("Full Mercy").clicked() {
                            if let Some(local) = ui_state.local_entity {
                                trial_events.send(CouncilTrialEvent::CastVote {
                                    participant: local,
                                    vote: MercyTrialVote::FullMercy,
                                });
                                ui_state.status_message = "Voted: Full Mercy".into();
                            } else {
                                ui_state.status_message = "Local entity not set — cannot vote".into();
                            }
                        }
                        if ui.button("Balanced").clicked() {
                            if let Some(local) = ui_state.local_entity {
                                trial_events.send(CouncilTrialEvent::CastVote {
                                    participant: local,
                                    vote: MercyTrialVote::BalancedMercy,
                                });
                                ui_state.status_message = "Voted: Balanced Mercy".into();
                            }
                        }
                        if ui.button("Cautious").clicked() {
                            if let Some(local) = ui_state.local_entity {
                                trial_events.send(CouncilTrialEvent::CastVote {
                                    participant: local,
                                    vote: MercyTrialVote::CautiousMercy,
                                });
                                ui_state.status_message = "Voted: Cautious Mercy".into();
                            }
                        }
                    });
                }
            } else {
                ui.colored_label(
                    egui::Color32::GRAY,
                    "No active Council session. Join or wait for host.",
                );
            }

            ui.separator();

            // ── Proposals ─────────────────────────────────────────────
            ui.heading(egui::RichText::new("📜 Proposals")
                .color(egui::Color32::from_rgb(200, 180, 255)));

            if ui_state.proposals.is_empty() {
                ui.label(egui::RichText::new("No proposals yet.").italics().color(egui::Color32::GRAY));
            } else {
                let mut sorted: Vec<_> = ui_state.proposals.values().cloned().collect();
                sorted.sort_by_key(|p| p.id);

                for proposal in sorted.iter().rev().take(8) {
                    let status_color = match proposal.status {
                        ProposalStatus::Submitted => egui::Color32::from_rgb(180, 180, 200),
                        ProposalStatus::UnderDeliberation => egui::Color32::from_rgb(160, 200, 255),
                        ProposalStatus::Voting => egui::Color32::from_rgb(255, 210, 120),
                        ProposalStatus::Passed => egui::Color32::from_rgb(100, 255, 160),
                        ProposalStatus::Rejected => egui::Color32::from_rgb(255, 140, 120),
                        ProposalStatus::Withdrawn => egui::Color32::GRAY,
                    };

                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.colored_label(status_color, format!("#{} [{:?}]", proposal.id, proposal.status));
                            if let Some(sid) = proposal.linked_session_id {
                                ui.label(format!("(session {})", sid));
                            }
                        });
                        ui.label(egui::RichText::new(&proposal.title).strong());
                        if !proposal.description.is_empty() {
                            ui.label(egui::RichText::new(&proposal.description).small());
                        }
                        ui.label(format!("For: {}  |  Against: {}", proposal.votes_for, proposal.votes_against));

                        // Vote buttons only while open
                        if matches!(
                            proposal.status,
                            ProposalStatus::Submitted
                                | ProposalStatus::UnderDeliberation
                                | ProposalStatus::Voting
                        ) {
                            ui.horizontal(|ui| {
                                if ui.small_button("For").clicked() {
                                    if let Some(local) = ui_state.local_entity {
                                        trial_events.send(CouncilTrialEvent::CastProposalVote {
                                            proposal_id: proposal.id,
                                            voter: local,
                                            is_for: true,
                                        });
                                        ui_state.status_message = format!("Voted FOR proposal #{}", proposal.id);
                                    }
                                }
                                if ui.small_button("Against").clicked() {
                                    if let Some(local) = ui_state.local_entity {
                                        trial_events.send(CouncilTrialEvent::CastProposalVote {
                                            proposal_id: proposal.id,
                                            voter: local,
                                            is_for: false,
                                        });
                                        ui_state.status_message = format!("Voted AGAINST proposal #{}", proposal.id);
                                    }
                                }
                            });
                        }
                    });
                }
            }

            ui.separator();

            // ── Submit new proposal ───────────────────────────────────
            ui.heading(egui::RichText::new("✦ Submit Proposal")
                .color(egui::Color32::from_rgb(180, 220, 160)));

            ui.horizontal(|ui| {
                ui.label("Title:");
                ui.text_edit_singleline(&mut ui_state.draft_title);
            });
            ui.label("Description:");
            ui.text_edit_multiline(&mut ui_state.draft_description);

            if ui.button("Submit to Council").clicked() {
                let title = ui_state.draft_title.trim().to_string();
                let description = ui_state.draft_description.trim().to_string();
                if title.is_empty() {
                    ui_state.status_message = "Title required".into();
                } else if let Some(local) = ui_state.local_entity {
                    trial_events.send(CouncilTrialEvent::SubmitProposal {
                        proposer: local,
                        title: title.clone(),
                        description,
                    });
                    ui_state.status_message = format!("Proposal submitted: {}", title);
                    ui_state.draft_title.clear();
                    ui_state.draft_description.clear();
                } else {
                    ui_state.status_message = "Local entity not set — cannot submit".into();
                }
            }

            ui.separator();

            // ── Last bloom ────────────────────────────────────────────
            if let Some(ref bloom) = ui_state.last_bloom {
                ui.heading(egui::RichText::new("✨ Last Collective Bloom")
                    .color(egui::Color32::from_rgb(255, 220, 140)));
                ui.label(format!("Session: {}", bloom.session_id));
                ui.label(format!("Intensity: {:.2}  |  Mercy Resonance: {:.2}", bloom.intensity, bloom.mercy_resonance));
                ui.label(format!("RBE Amplification: {:.2}x", bloom.rbe_amplification));
            }

            if !ui_state.status_message.is_empty() {
                ui.add_space(4.0);
                ui.colored_label(
                    egui::Color32::from_rgb(160, 230, 180),
                    &ui_state.status_message,
                );
            }

            ui.add_space(6.0);
            ui.label(egui::RichText::new("Hotkey: C  —  toggle panel")
                .small()
                .color(egui::Color32::GRAY));
        });
}

// Production notes:
// - Wire CouncilUIState.current_session / proposals from network or server mirror systems.
// - Set local_entity once the local player Entity is known.
// - On CouncilTrialResolved / bloom events, populate last_bloom for the visual summary.
// - All proposal & vote events route through the authoritative server handler.
// Thunder locked in. Permanent PATSAGi. Yoi ⚡
