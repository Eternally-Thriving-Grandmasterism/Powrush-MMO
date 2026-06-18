/*!
 * Council Session Handler (Server Authoritative)
 *
 * Manages the full lifecycle of Council Mercy Trials.
 * This is the authoritative source of truth for trial state and phase progression.
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
            .add_systems(Update, (
                handle_council_trial_events,
                advance_trial_phases,
                broadcast_council_updates,
            ));
    }
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
                state.phase_duration = 45.0; // Lobby duration

                trials.sessions.insert(session_id, state);

                info!(
                    "Council Mercy Trial started | session={} | host={:?} | participants={}",
                    session_id, host, participants.len()
                );
            }

            CouncilTrialEvent::AdvancePhase => {
                // Server can force phase advancement (e.g. from admin or timer)
                // Full logic will be expanded in next iteration
            }

            CouncilTrialEvent::CastVote { participant, vote } => {
                // Find session this participant belongs to and record vote
                for state in trials.sessions.values_mut() {
                    if state.participants.contains(participant) {
                        state.votes.insert(*participant, *vote);
                        info!("Vote recorded | session={} | participant={:?}", state.session_id, participant);
                        break;
                    }
                }
            }

            CouncilTrialEvent::ResolveTrial => {
                // Trigger resolution logic (calculate bloom, distribute RBE, etc.)
            }

            CouncilTrialEvent::CancelTrial => {
                // Cleanup session
            }
        }
    }
}

/// System that automatically advances phases based on timers
fn advance_trial_phases(
    mut trials: ResMut<ActiveCouncilTrials>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();

    for state in trials.sessions.values_mut() {
        let elapsed = (now - state.current_phase_start) as f32;

        if elapsed >= state.phase_duration {
            // Advance to next logical phase
            state.phase = match state.phase {
                CouncilMercyTrialPhase::Lobby => CouncilMercyTrialPhase::Attunement,
                CouncilMercyTrialPhase::Attunement => CouncilMercyTrialPhase::Deliberation,
                CouncilMercyTrialPhase::Deliberation => CouncilMercyTrialPhase::Voting,
                CouncilMercyTrialPhase::Voting => CouncilMercyTrialPhase::Resolution,
                CouncilMercyTrialPhase::Resolution => CouncilMercyTrialPhase::Completed,
                CouncilMercyTrialPhase::Completed => CouncilMercyTrialPhase::Completed,
            };

            state.current_phase_start = now;
            state.phase_duration = match state.phase {
                CouncilMercyTrialPhase::Attunement => 60.0,
                CouncilMercyTrialPhase::Deliberation => 90.0,
                CouncilMercyTrialPhase::Voting => 30.0,
                CouncilMercyTrialPhase::Resolution => 20.0,
                _ => 30.0,
            };

            info!("Council trial advanced | session={} | new_phase={:?}", state.session_id, state.phase);
        }
    }
}

/// Broadcasts state updates to connected clients (placeholder for networking layer)
fn broadcast_council_updates(
    trials: Res<ActiveCouncilTrials>,
    // mut net: EventWriter<NetworkBroadcast>, // TODO: wire to actual networking
) {
    for state in trials.sessions.values() {
        if state.phase != CouncilMercyTrialPhase::Completed {
            // TODO: Send CouncilSessionUpdate to relevant clients
            // net.send(...);
        }
    }
}

// End of initial skeleton — Phase machine + event handling in place.
// Next: Full resolution logic + RBE integration + client replication.
