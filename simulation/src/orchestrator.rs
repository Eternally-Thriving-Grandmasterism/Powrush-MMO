/*!
 * TickResult now includes active policy count for observability of Persistent Policy Modifiers.
 */

// ... existing imports ...

#[derive(Debug, Default, Clone)]
pub struct TickResult {
    // ... existing fields ...
    pub resolved_council_proposals: Vec<CouncilProposal>,
    pub applied_council_decisions: Vec<CouncilDecision>,
    pub dynamic_mercy_threshold: Option<f32>,
    pub last_base_weight: Option<f32>,
    pub last_archetype_weight: Option<f32>,
    pub last_delta_weight: Option<f32>,

    pub active_policy_count: usize,  // NEW: number of active persistent policies
}

// In run_tick, after processing:
let active_policy_count = self.world.active_policies.len();

let mut tick_result = TickResult {
    // ...
    active_policy_count,
    ..Default::default()
};
