/*!
 * Council Session Handler (Server Authoritative)
 *
 * Manages the full lifecycle of Council Mercy Trials, including resolution and bloom generation.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use shared::council_mercy_trial::*;
use std::collections::HashMap;

/// Resource that holds all active council trial sessions on the server
#[derive(Resource, Default)]
pub struct ActiveCouncilTrials {
    pub sessions: HashMap<u64, CouncilSessionState>,
    pub next_session_id: u64,
}

/// Plugin that registers council trial systems
pub struct CouncilSessionPlugin;

impl Plugin for CouncilSessionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveCouncilTrials>()
            .add_event::<CouncilTrialEvent>()
            .add_event::<CouncilTrialResolved>()
            .add_systems(Update, (
                handle_council_trial_events,
                advance_trial_phases,
                resolve_completed_trials,
                broadcast_council_updates,
            ));
    }
}

/// Event emitted when a Council Mercy Trial successfully resolves
#[derive(Event, Clone, Debug)]
pub struct CouncilTrialResolved {
    pub session_id: u64,
    pub bloom: CollectiveEpiphanyBloom,
}

/// Main system that processes CouncilTrialEvent commands
fn handle_council_trial_events(
    mut events: EventReader<CouncilTrialEvent>,
    mut trials: ResMut<ActiveCouncilTrials>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();

    for event in events.read() {
        match event {
            CouncilTrialEvent::StartTrial { host, participants } => {
                let session_id = trials.next_session_id;
                trials.next_session_id += 1;

                let mut state = CouncilSessionState::default();
                state.session_id = session_id;
                state.host = Some(*host);
                state.participants = participants.clone();
                state.phase = CouncilMercyTrialPhase::Lobby;
                state.start_time = now;
                state.current_phase_start = now;
                state.phase_duration = 45.0;

                trials.sessions.insert(session_id, state);

                info!(
                    "Council Mercy Trial started | session={} | participants={}",
                    session_id, participants.len()
                );
            }

            CouncilTrialEvent::CastVote { participant, vote } => {
                for state in trials.sessions.values_mut() {
                    if state.participants.contains(participant) {
                        state.votes.insert(*participant, *vote);
                        break;
                    }
                }
            }

            CouncilTrialEvent::ResolveTrial => {
                for state in trials.sessions.values_mut() {
                    if state.phase == CouncilMercyTrialPhase::Voting {
                        state.phase = CouncilMercyTrialPhase::Resolution;
                        state.current_phase_start = now;
                        state.phase_duration = 15.0;
                    }
                }
            }

            _ => {}
        }
    }
}

/// Automatically advances phases based on timers
fn advance_trial_phases(
    mut trials: ResMut<ActiveCouncilTrials>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();

    for state in trials.sessions.values_mut() {
        let elapsed = (now - state.current_phase_start) as f32;

        if elapsed >= state.phase_duration {
            let next_phase = match state.phase {
                CouncilMercyTrialPhase::Lobby => CouncilMercyTrialPhase::Attunement,
                CouncilMercyTrialPhase::Attunement => CouncilMercyTrialPhase::Deliberation,
                CouncilMercyTrialPhase::Deliberation => CouncilMercyTrialPhase::Voting,
                CouncilMercyTrialPhase::Voting => CouncilMercyTrialPhase::Resolution,
                CouncilMercyTrialPhase::Resolution => CouncilMercyTrialPhase::Completed,
                CouncilMercyTrialPhase::Completed => CouncilMercyTrialPhase::Completed,
            };

            state.phase = next_phase;
            state.current_phase_start = now;

            state.phase_duration = match next_phase {
                CouncilMercyTrialPhase::Attunement => 60.0,
                CouncilMercyTrialPhase::Deliberation => 90.0,
                CouncilMercyTrialPhase::Voting => 30.0,
                CouncilMercyTrialPhase::Resolution => 15.0,
                _ => 30.0,
            };

            info!("Council trial phase advanced | session={} | phase={:?}", state.session_id, next_phase);
        }
    }
}

/// Resolves trials that have reached the Completed phase and generates the final bloom
fn resolve_completed_trials(
    mut trials: ResMut<ActiveCouncilTrials>,
    mut resolved_events: EventWriter<CouncilTrialResolved>,
) {
    let mut to_remove = Vec::new();

    for (session_id, state) in trials.sessions.iter_mut() {
        if state.phase == CouncilMercyTrialPhase::Completed {
            let bloom = calculate_collective_bloom(state);

            resolved_events.send(CouncilTrialResolved {
                session_id: *session_id,
                bloom: bloom.clone(),
            });

            info!(
                "Council Mercy Trial RESOLVED | session={} | intensity={:.2} | rbe_amp={:.2}x",
                session_id, bloom.intensity, bloom.rbe_amplification
            );

            to_remove.push(*session_id);
        }
    }

    for id in to_remove {
        trials.sessions.remove(&id);
    }
}

/// Core resolution logic: calculates the final CollectiveEpiphanyBloom
fn calculate_collective_bloom(state: &CouncilSessionState) -> CollectiveEpiphanyBloom {
    let participant_count = state.participants.len() as f32;
    if participant_count == 0.0 {
        return CollectiveEpiphanyBloom {
            session_id: state.session_id,
            intensity: 0.3,
            mercy_resonance: 0.5,
            bloom_amplification: 1.0,
            participant_contributions: vec![],
            rbe_amplification: 1.0,
            created_at: 0.0,
        };
    }

    let mut full_mercy = 0;
    let mut balanced = 0;
    let mut cautious = 0;

    for vote in state.votes.values() {
        match vote {
            MercyTrialVote::FullMercy => full_mercy += 1,
            MercyTrialVote::BalancedMercy => balanced += 1,
            MercyTrialVote::CautiousMercy => cautious += 1,
        }
    }

    let base_intensity = (state.collective_attunement * 0.6 + (participant_count / 8.0).min(1.0) * 0.4).clamp(0.4, 0.95);

    let vote_influence = (full_mercy as f32 * 1.15 + balanced as f32 * 0.95 + cautious as f32 * 0.75)
        / participant_count.max(1.0);

    let final_intensity = (base_intensity * 0.7 + vote_influence * 0.3).clamp(0.5, 0.98);

    let rbe_amp = (1.0 + (final_intensity - 0.5) * 1.8 + state.collective_attunement * 0.6).clamp(1.0, 3.5);

    CollectiveEpiphanyBloom {
        session_id: state.session_id,
        intensity: final_intensity,
        mercy_resonance: state.collective_attunement,
        bloom_amplification: state.bloom_amplification,
        participant_contributions: vec![],
        rbe_amplification: rbe_amp,
        created_at: state.current_phase_start,
    }
}

fn broadcast_council_updates(
    trials: Res<ActiveCouncilTrials>,
) {
    for state in trials.sessions.values() {
        if state.phase != CouncilMercyTrialPhase::Completed {
            // TODO: Send CouncilSessionUpdate to clients
        }
    }
}

// End of resolution logic implementation.
// Next: Client-side reaction to CouncilTrialResolved + RBE integration.
