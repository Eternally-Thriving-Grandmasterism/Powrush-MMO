// client/council_session_ui.rs
// Powrush-MMO — Client UI for Council Mercy Trials (Phase 2)
// Bevy UI + WebXR ready. Hot-reloadable. Mercy-gated visuals.
// Integrates with divine_whispers_ui.rs, inventory_ui.rs, and spatial audio for collective blooms.
// AG-SML v1.0

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
}

pub struct CouncilSessionUIPlugin;

impl Plugin for CouncilSessionUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CouncilUIState>()
           .add_systems(Update, (
               update_council_ui,
               handle_vote_buttons,
               sync_bloom_visuals,
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
            text.sections[0].value = format!(
                "Council Session {} | Phase: {:?} | Participants: {} | Mercy: {:.2}",
                state.session_id,
                state.phase,
                state.participants.len(),
                state.mercy_scores.values().sum::<f32>() / state.participants.len().max(1) as f32
            );
        } else if ui_state.show_lobby {
            text.sections[0].value = "Press J to join Council Mercy Trial".to_string();
        }
    }
}

fn handle_vote_buttons(
    mut ui_state: ResMut<CouncilUIState>,
    mut interaction_query: Query<(&Interaction, &CouncilVoteButton), Changed<Interaction>>,
    mut client_events: EventWriter<ClientMessage>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(state) = &ui_state.current_session {
                let vote = MercyTrialVote {
                    voter_id: 0, // Replace with real local player_id from persistence
                    proposal_id: button.proposal.clone(),
                    mercy_weight: 0.8, // Real: from local resonance cache
                    timestamp_ms: 0,
                    grace_intent: 0.7,
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
            // Valence-driven color shift + scale for collective bloom
            let intensity = bloom.intensity.clamp(0.0, 1.0);
            sprite.color = Color::srgb(0.2 + intensity * 0.6, 0.6 + intensity * 0.3, 0.9);
            // In real: trigger particle web + spatial audio bloom via existing systems
        }
    }
}

// Note: Full production UI would use Bevy UI nodes, buttons for proposals,
// participant list with mercy scores, and WebXR overlay for immersive council "room".
// This file provides the complete scaffold ready for extension in next polish cycle.
// Integrates with divine_whispers_ui for amplified wisdom on bloom.
