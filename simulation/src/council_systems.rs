/*!
 * Council Systems - Voting, Resolution, Valence & Mercy Resonance (v19.2.9)
 */

use bevy::prelude::*;
use crate::game_state::CouncilState;
use crate::council_mercy_trial::CouncilSessionManager;
use crate::world::SovereignWorldState;

#[derive(Event, Clone, Debug)]
pub struct CastVote {
    pub player_id: u64,
    pub vote_strength: f32,
}

#[derive(Event, Clone, Debug)]
pub struct CouncilResolved {
    pub final_attunement: f32,
    pub success: bool,
    pub was_tie: bool,
    pub valence_score: f32,
}

/// Tracks recent mercy resonance
#[derive(Resource, Default)]
pub struct RecentMercyResonance {
    pub value: f32,
}

/// Stores the last council valence score for UI display
#[derive(Resource, Default)]
pub struct LastCouncilValence {
    pub value: f32,
}

pub fn calculate_valence_score(
    average_attunement: f32,
    participant_count: u8,
    sustainability_impact: f32,
    recent_mercy_resonance: f32,
) -> f32 {
    let base_valence = average_attunement * 0.55;
    let participation_bonus = (participant_count as f32 * 0.03).min(0.15);
    let sustainability_bonus = sustainability_impact.clamp(-0.3, 0.3) * 0.25;
    let mercy_context = (recent_mercy_resonance - 0.5) * 0.25;

    let raw_valence = base_valence + participation_bonus + sustainability_bonus + mercy_context;
    raw_valence.clamp(0.0, 1.0) * 0.92 + 0.08
}

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

/// Updates resonance and stores last valence after resolution
pub fn update_recent_mercy_resonance(
    mut events: EventReader<CouncilResolved>,
    mut resonance: ResMut<RecentMercyResonance>,
    mut last_valence: ResMut<LastCouncilValence>,
) {
    for event in events.read() {
        let target = if event.success { 0.82 } else { 0.28 };
        resonance.value = resonance.value * 0.65 + target * 0.35;
        resonance.value = resonance.value.clamp(0.0, 1.0);

        last_valence.value = event.valence_score;
    }
}

fn compute_sustainability_impact(world: &SovereignWorldState) -> f32 {
    if world.resource_nodes.is_empty() {
        return 0.0;
    }
    let total: f32 = world.resource_nodes.values()
        .map(|node| node.sustainability_score)
        .sum();
    let avg = total / world.resource_nodes.len() as f32;
    (avg - 0.5) * 0.6
}

pub fn council_resolution_system(
    mut next_state: ResMut<NextState<CouncilState>>,
    mut council_manager: ResMut<CouncilSessionManager>,
    mut resolved_events: EventWriter<CouncilResolved>,
    world: Res<SovereignWorldState>,
    resonance: Res<RecentMercyResonance>,
) {
    let min_participants = 3;
    let biome = "council_chamber";

    if let Some(bloom) = council_manager.resolve_and_set_bloom_from_real_data(0, min_participants, biome) {
        let attunement = bloom.collective_attunement_score;
        let mut success = bloom.council_mercy_seal;
        let mut was_tie = false;

        let sustainability_impact = compute_sustainability_impact(&world);
        let valence = calculate_valence_score(attunement, bloom.participant_count, sustainability_impact, resonance.value);

        let epsilon = 0.02;
        if (attunement - 0.5).abs() < epsilon {
            was_tie = true;
            let participant_bonus = (bloom.participant_count as f32 * 0.015).min(0.08);
            let final_score = attunement + participant_bonus + (valence - 0.5) * 0.12;
            success = final_score >= 0.5;
        }

        resolved_events.send(CouncilResolved {
            final_attunement: attunement,
            success,
            was_tie,
            valence_score: valence,
        });

        info!("Council resolved. Success: {}, Attunement: {:.2}, Valence: {:.2}, Resonance: {:.2}, Tie: {}",
              success, attunement, valence, resonance.value, was_tie);

        next_state.set(CouncilState::Inactive);
    }
}

pub struct CouncilSystemsPlugin;

impl Plugin for CouncilSystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RecentMercyResonance>()
            .init_resource::<LastCouncilValence>()
            .add_event::<CastVote>()
            .add_event::<CouncilResolved>()
            .add_systems(Update, (
                council_voting_system.run_if(in_state(CouncilState::Voting)),
                update_recent_mercy_resonance,
            ))
            .add_systems(OnEnter(CouncilState::Resolving), council_resolution_system);
    }
}

// Thunder locked in. Yoi ⚡
