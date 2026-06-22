/*!
 * Client-side style event listeners for policy effects.
 */

use bevy::prelude::*;

// System that listens for EpigeneticPolicyEffectApplied events
pub fn handle_epigenetic_policy_effect_applied(
    mut events: EventReader<EpigeneticPolicyEffectApplied>,
    mut query: Query<(&mut crate::epigenetic_modulation::EpigeneticProfile, &Transform)>,
) {
    for event in events.read() {
        // Example client-side reaction:
        // - Trigger visual effect on the agent
        // - Update local UI / notifications
        // - Play sound or particle effect

        info!(
            "[Client] Epigenetic effect from {:?} (strength {:.2}) applied to agent {}",
            event.policy_type, event.strength, event.agent_id
        );

        // Future: Spawn visual effect entity, update UI, etc.
    }
}

// You can add more listeners here, e.g.:
// pub fn handle_interest_zone_replicated(...) { ... }
// pub fn handle_council_bloom_triggered(...) { ... }
