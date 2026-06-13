/*!
 * Mercy Ascent Trial Phase Management
 * Sacred confrontation with the self. TOLC 8 aligned.
 */

use bevy::prelude::*;
use crate::ascension::{components::*, events::*, resources::*};

pub fn handle_mercy_ascent_attempt_system(
    mut commands: Commands,
    mut events: EventReader<AttemptMercyAscent>,
) {
    for event in events.read() {
        for entity in event.group_members.iter().chain(std::iter::once(&event.initiator)) {
            commands.entity(*entity).insert(InMercyAscentTrial {
                phase: TrialPhase::Reckoning,
                mercy_score: 1.0,
                start_tick: 0, // TODO: set from Time resource
            });
        }
    }
}

pub fn mercy_ascent_phase_manager_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut InMercyAscentTrial, &mut MercyAlignment, &AscensionProgress)>,
    time: Res<Time>,
    server_state: Res<ServerResonanceState>,
) {
    for (entity, mut trial, mut mercy, progress) in query.iter_mut() {
        match trial.phase {
            TrialPhase::Reckoning => {
                // Spawn Echoes based on player history + server mirror_score
                // For now: simple progression gate
                if trial.mercy_score >= 0.68 {
                    trial.phase = TrialPhase::Alignment;
                    mercy.score += 0.08;
                }
            }
            TrialPhase::Alignment => {
                // Dynamic challenges based on AscensionPath
                if trial.mercy_score >= 0.80 {
                    trial.phase = TrialPhase::Bloom;
                }
            }
            TrialPhase::Bloom => {
                let difficulty = 1.0 + (server_state.mirror_score * 1.5);
                // Final boss: must be healed through resonance/mercy, not just defeated
                // Placeholder success condition
                if trial.mercy_score >= 0.92 {
                    commands.entity(entity).remove::<InMercyAscentTrial>();
                    commands.trigger(MercyAscentCompleted {
                        player: entity,
                        success: true,
                    });
                }
            }
        }
    }
}
