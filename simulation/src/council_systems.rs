/*!
 * Council Systems - Voting & Resolution (v19.2.9)
 *
 * Basic voting and resolution logic driven by CouncilState.
 * Integrates with CouncilSessionManager and RBE economy.
 */

use bevy::prelude::*;
use crate::game_state::CouncilState;
use crate::council_mercy_trial::{CouncilSessionManager, SharedReceptorBloomField};

/// Event: A player casts a vote in council
#[derive(Event, Clone, Debug)]
pub struct CastVote {
    pub player_id: u64,
    pub vote_strength: f32, // 0.0 - 1.0
}

/// Event: Council has finished resolving
#[derive(Event, Clone, Debug)]
pub struct CouncilResolved {
    pub final_attunement: f32,
    pub success: bool,
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
        // Accumulate attunement from votes
        council_manager.add_participant_attunement(vote.vote_strength);

        info!("Player {} cast vote with strength {:.2}", vote.player_id, vote.vote_strength);
    }
}

/// System that resolves the council when entering Resolving state
pub fn council_resolution_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<CouncilState>>,
    mut council_manager: ResMut<CouncilSessionManager>,
    mut resolved_events: EventWriter<CouncilResolved>,
) {
    if let Some(bloom) = council_manager.resolve_and_set_bloom_from_real_data(
        0, // current_tick - should come from orchestrator
        3,
        "council_chamber",
    ) {
        let success = bloom.council_mercy_seal;

        resolved_events.send(CouncilResolved {
            final_attunement: bloom.collective_attunement_score,
            success,
        });

        info!("Council resolved. Success: {}, Attunement: {:.2}",
              success, bloom.collective_attunement_score);

        // Return to Inactive after resolution
        next_state.set(CouncilState::Inactive);
    }
}

/// Plugin for Council voting and resolution systems
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
