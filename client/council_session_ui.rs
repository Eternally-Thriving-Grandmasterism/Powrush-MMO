// client/council_session_ui.rs
// Powrush-MMO — Client UI for Council Mercy Trials (Phase 2)
// Bevy + WebXR ready. Mercy-gated, valence-driven visuals.
// Strong integration with divine_whispers_ui, epiphany_feedback, and PATSAGi monitoring.
// AG-SML v1.0 | TOLC 8 Mercy Gates enforced

use bevy::prelude::*;
use shared::protocol::*;

#[derive(Component)]
pub struct CouncilSessionUI;

#[derive(Component)]
pub struct CouncilVoteButton { pub proposal: String }

#[derive(Resource, Default)]
pub struct CouncilUIState {
    pub current_session: Option<CouncilSessionState>,
    pub last_bloom: Option<CollectiveEpiphanyBloom>,
    pub show_lobby: bool,
    pub local_player_id: u64,
}

pub struct CouncilSessionUIPlugin;

impl Plugin for CouncilSessionUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CouncilUIState>()
           .add_systems(Update, (
               update_council_ui,
               handle_vote_buttons,
               sync_bloom_visuals,
               update_participant_display,
           ));
    }
}

fn update_council_ui(
    mut ui_state: ResMut<CouncilUIState>,
    mut query: Query<&mut Text, With<CouncilSessionUI>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        ui_state.show_lobby = !ui_state.show_lobby;
    }

    if let Ok(mut text) = query.get_single_mut() {
        if let Some(state) = &ui_state.current_session {
            let avg_mercy = if !state.participants.is_empty() {
                state.mercy_scores.values().sum::<f32>() / state.participants.len() as f32
            } else { 0.0 };

            text.sections[0].value = format!(
                "Council {} | {:?} | {} participants | Avg Mercy: {:.2} | Bloom: {:.0}%",
                state.session_id,
                state.phase,
                state.participants.len(),
                avg_mercy,
                state.bloom_intensity * 100.0
            );
        } else if ui_state.show_lobby {
            text.sections[0].value = "Press J to join Council Mercy Trial | C to toggle".to_string();
        }
    }
}

fn handle_vote_buttons(
    mut ui_state: ResMut<CouncilUIState>,
    mut interaction_query: Query<(&Interaction, &CouncilVoteButton), Changed<Interaction>>,
    mut client_events: EventWriter<ClientMessage>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(state) = &ui_state.current_session {
                let vote = MercyTrialVote {
                    voter_id: ui_state.local_player_id,
                    proposal_id: button.proposal.clone(),
                    mercy_weight: 0.85,
                    timestamp_ms: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                    grace_intent: 0.75,
                };
                client_events.send(ClientMessage::CouncilVote { vote });
            }
        }
    }
}

fn sync_bloom_visuals(
    ui_state: Res<CouncilUIState>,
    mut bloom_query: Query<&mut Sprite, With<CouncilSessionUI>>,
) {
    if let Some(bloom) = &ui_state.last_bloom {
        for mut sprite in &mut bloom_query {
            let intensity = bloom.intensity.clamp(0.0, 1.0);
            sprite.color = Color::srgb(
                0.15 + intensity * 0.65,
                0.55 + intensity * 0.35,
                0.92 + intensity * 0.08,
            );
            // TODO: Trigger valence particle web + positioned spatial audio bloom
        }
    }
}

fn update_participant_display(
    ui_state: Res<CouncilUIState>,
) {
    if let Some(state) = &ui_state.current_session {
        // Production: Spawn/update dynamic UI nodes or egui labels showing:
        // - participant player_id
        // - current mercy resonance score
        // - contribution to active proposal
        // Integrate hover with inventory_ui and divine_whispers_ui for context
        for &pid in &state.participants {
            let _mercy = state.mercy_scores.get(&pid).unwrap_or(&0.5);
            // Future: create/update Text or egui element with mercy bar
        }
    }
}

// Production Notes:
// - Full Bevy UI or egui Council window with lobby list and dynamic proposal buttons
// - WebXR immersive council chamber with participant avatars + real-time mercy orbs
// - On bloom: cross-call to epiphany_feedback + divine_whispers_ui for amplified collective wisdom
// - Keyboard: J = join, C = toggle lobby
// - All flows validated through TOLC 8 Mercy Gates
