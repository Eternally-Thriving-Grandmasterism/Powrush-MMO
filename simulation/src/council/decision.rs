/*!
 * CouncilDecision with mercy-aligned policy duration/intensity scaling.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::AgentId;

// ... existing structs ...

impl CouncilDecision {
    // ... existing methods ...

    /// Creates an ActivePolicy with duration and strength scaled by final_mercy_alignment_score.
    pub fn create_active_policy(
        &self,
        policy_type: PolicyType,
        base_strength: f32,
        base_duration: u64,
    ) -> ActivePolicy {
        let mas = self.final_mercy_alignment_score.max(0.4);

        // Higher MAS = longer duration and stronger intensity
        let duration_multiplier = 0.7 + (mas * 0.8);           // 0.7x to 1.5x
        let strength_multiplier = 0.85 + (mas * 0.6);          // 0.85x to 1.45x

        ActivePolicy {
            decision_id: self.proposal_id,
            policy_type,
            target_faction: None,
            strength: base_strength * strength_multiplier,
            remaining_ticks: ((base_duration as f32) * duration_multiplier) as u64,
            created_tick: self.passed_tick,
        }
    }
}

// Update the places where we create policies to use this method
// Example in apply_council_decision_effects:
// world.active_policies.push(decision.create_active_policy(PolicyType::AbundanceBoost, 0.15 * mag, 120));
