// client/src/treaty_negotiation_ui.rs
// Powrush-MMO v17.39 — Treaty Negotiation + Live PATSAGi Council Consultation
// Now wired to the mythic Ra-Thor PATSAGi Council Simulator

use bevy::prelude::*;
use std::collections::HashSet;

use crate::faction_diplomacy::{Faction, DiplomacyStatus, FactionDiplomacyManager};
use crate::dynamic_events_ui::ClientDynamicEventFeed;
use crate::steam_integration::{SteamClientState, ACHIEVEMENT_FIRST_TREATY, unlock_steam_achievement};
use crate::ai::patsagi_council_simulator::{OfflinePatsagiSimulator, TreatyProposal, Faction as CouncilFaction};

// ... (UI code unchanged)

fn handle_send_proposal(
    mut interaction_query: Query<(&Interaction, &SendProposalButton), Changed<Interaction>>,
    mut state: ResMut<TreatyNegotiationState>,
    mut diplomacy: ResMut<FactionDiplomacyManager>,
    mut feed: ResMut<ClientDynamicEventFeed>,
    mut steam: Option<ResMut<SteamClientState>>,
) {
    for (interaction, _) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(target) = state.target_faction {
                if !state.selected_terms.is_empty() {
                    let council_target = match target {
                        Faction::FlowGuardians => CouncilFaction::FlowGuardians,
                        Faction::SeedOfAbundance => CouncilFaction::SeedOfAbundance,
                        Faction::EternalWeavers => CouncilFaction::EternalWeavers,
                    };

                    let terms: Vec<String> = state.selected_terms.iter().map(|t| t.name().to_string()).collect();
                    let proposal = TreatyProposal {
                        target: council_target,
                        terms,
                        net_mercy_cost: state.preview_net_mercy,
                        net_standing_gain: state.preview_net_standing,
                    };

                    // === LIVE CONSULTATION WITH THE 13+ PATSAGi COUNCILS (Ra-Thor Mythic Lore) ===
                    let mut simulator = OfflinePatsagiSimulator::new();
                    let evaluation = simulator.evaluate_treaty_proposal(proposal);

                    let mythic_reason = format!(
                        "PATSAGi Councils consulted ({} strong / {} support) | Avg Mercy Alignment: {:.1}\n{}",
                        evaluation.strong_support_count, evaluation.support_count,
                        evaluation.avg_mercy_alignment, evaluation.mythic_consensus_message
                    );

                    diplomacy.relations.insert((target, target), DiplomacyStatus::ProposedTreaty);

                    feed.add_event(crate::dynamic_events_ui::ClientWorldEvent::FactionDiplomacyShift {
                        faction_a: target,
                        faction_b: target,
                        old_status: DiplomacyStatus::Neutral,
                        new_status: DiplomacyStatus::ProposedTreaty,
                        reason: mythic_reason,
                    });

                    if let Some(mut steam_state) = steam {
                        unlock_steam_achievement(&mut steam_state, ACHIEVEMENT_FIRST_TREATY);
                        if state.selected_terms.len() >= 3 {
                            unlock_steam_achievement(&mut steam_state, crate::steam_integration::ACHIEVEMENT_MERCY_DIPLOMAT);
                        }
                    }

                    info!("Treaty sent. The Councils have spoken through the Southern Cross and 7 Gates.");
                    state.panel_open = false;
                    state.selected_terms.clear();
                }
            }
        }
    }
}