/*!
 * Council Systems - Voting & Resolution (v19.2.9)
 *
 * Includes tie-breaking logic during resolution.
 */

use bevy::prelude::*;
use crate::game_state::CouncilState;
use crate::council_mercy_trial::CouncilSessionManager;

/// Event: A player casts a vote in council
#[derive(Event, Clone, Debug)]
pub struct CastVote {
    pub player_id: u64,
    pub vote_strength: f32,
}

/// Event: Council has finished resolving
#[derive(Event, Clone, Debug)]
pub struct CouncilResolved {
    pub final_attunement: f32,
    pub success: bool,
    pub was_tie: bool,
}

/// System that processes votes while in Voting state
pub fn council_voting_system(
    mut events: EventReader<CastVote>,
    mut council_manager: ResMut<CouncilSessionManager>,
    current_state: Res<State<CouncilState>>,
) {
    if current_state.get() != &CouncilState::Voting {
        return;
    }

    for vote in events.read() {
        council_manager.add_participant_attunement(vote.vote_strength);
    }
}

/// Resolves the council with basic tie-breaking logic
pub fn council_resolution_system(
    mut next_state: ResMut<NextState<CouncilState>>,
    mut council_manager: ResMut<CouncilSessionManager>,
    mut resolved_events: EventWriter<CouncilResolved>,
) {
    let min_participants = 3;
    let biome = "council_chamber";

    if let Some(bloom) = council_manager.resolve_and_set_bloom_from_real_data(
        0, // TODO: pass real current_tick from orchestrator
        min_participants,
        biome,
    ) {
        let attunement = bloom.collective_attunement_score;
        let mut success = bloom.council_mercy_seal;
        let mut was_tie = false;

        // === Tie-breaking Logic ===
        let epsilon = 0.02;
        if (attunement - 0.5).abs() < epsilon {
            was_tie = true;

            // Tie-breaker: Slight mercy bias + participant count
            let participant_bonus = (bloom.participant_count as f32 * 0.015).min(0.08);
            let final_score = attunement + participant_bonus;

            success = final_score >= 0.5;

            info!(
                "Council tie detected (attunement: {:.2}). Tie-breaker applied → success: {}",
                attunement, success
            );
        }

        resolved_events.send(CouncilResolved {
            final_attunement: attunement,
            success,
            was_tie,
        });

        info!("Council resolved. Success: {}, Attunement: {:.2}, Tie: {}",
              success, attunement, was_tie);

        next_state.set(CouncilState::Inactive);
    }
}

pub struct CouncilSystemsPlugin;

impl Plugin for CouncilSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CastVote>()
            .add_event::<CouncilResolved>()
            .add_systems(Update, council_voting_system.run_if(in_state(CouncilState::Voting)))
            .add_systems(OnEnter(CouncilState::Resolving), council_resolution_system);
    }
}

// Thunder locked in. Yoi ⚡
