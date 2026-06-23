/*!
 * Council Systems - Voting, Resolution & Valence Scoring (v19.2.9)
 *
 * Now queries real sustainability data from SovereignWorldState + EconomicLayer.
 */

use bevy::prelude::*;
use crate::game_state::CouncilState;
use crate::council_mercy_trial::CouncilSessionManager;
use crate::world::SovereignWorldState;
use crate::economy::EconomicLayer;

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
    pub valence_score: f32,
}

/// Calculates a valence score (PATSAGi-inspired)
pub fn calculate_valence_score(
    average_attunement: f32,
    participant_count: u8,
    sustainability_impact: f32,
    recent_mercy_resonance: f32,
) -> f32 {
    let base_valence = average_attunement * 0.6;
    let participation_bonus = (participant_count as f32 * 0.03).min(0.15);
    let sustainability_bonus = sustainability_impact.clamp(-0.3, 0.3) * 0.25;
    let mercy_context = (recent_mercy_resonance - 0.5) * 0.2;

    let raw_valence = base_valence + participation_bonus + sustainability_bonus + mercy_context;
    raw_valence.clamp(0.0, 1.0) * 0.95 + 0.05
}

/// Helper: Compute real sustainability impact from world state
fn compute_sustainability_impact(world: &SovereignWorldState) -> f32 {
    if world.resource_nodes.is_empty() {
        return 0.0;
    }

    let total: f32 = world.resource_nodes.values()
        .map(|node| node.sustainability_score)
        .sum();

    let avg = total / world.resource_nodes.len() as f32;

    // Convert average sustainability (0.0-1.0) to impact range (-0.3 to +0.3)
    (avg - 0.5) * 0.6
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

/// Resolves the council using real sustainability data from the world
pub fn council_resolution_system(
    mut next_state: ResMut<NextState<CouncilState>>,
    mut council_manager: ResMut<CouncilSessionManager>,
    mut resolved_events: EventWriter<CouncilResolved>,
    world: Res<SovereignWorldState>,
    _economic_layer: Option<Res<EconomicLayer>>,
) {
    let min_participants = 3;
    let biome = "council_chamber";

    if let Some(bloom) = council_manager.resolve_and_set_bloom_from_real_data(
        0,
        min_participants,
        biome,
    ) {
        let attunement = bloom.collective_attunement_score;
        let mut success = bloom.council_mercy_seal;
        let mut was_tie = false;

        // === Real sustainability impact from world ===
        let sustainability_impact = compute_sustainability_impact(&world);

        let valence = calculate_valence_score(
            attunement,
            bloom.participant_count,
            sustainability_impact,
            0.5, // TODO: replace with real recent mercy resonance tracking
        );

        // Tie-breaking with real valence influence
        let epsilon = 0.02;
        if (attunement - 0.5).abs() < epsilon {
            was_tie = true;
            let participant_bonus = (bloom.participant_count as f32 * 0.015).min(0.08);
            let final_score = attunement + participant_bonus + (valence - 0.5) * 0.1;
            success = final_score >= 0.5;
        }

        resolved_events.send(CouncilResolved {
            final_attunement: attunement,
            success,
            was_tie,
            valence_score: valence,
        });

        info!(
            "Council resolved. Success: {}, Attunement: {:.2}, Valence: {:.2}, Sustainability Impact: {:.2}, Tie: {}",
            success, attunement, valence, sustainability_impact, was_tie
        );

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
