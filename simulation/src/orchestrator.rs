/*!
 * TickResult with enhanced policy visibility (synergies, conflicts, types).
 */

use crate::council::decision::PolicyType;

#[derive(Debug, Default, Clone)]
pub struct TickResult {
    // ... existing fields ...
    pub active_policy_count: usize,
    pub active_policy_types: Vec<PolicyType>,   // Unique active policy types this tick
    pub synergies_active: bool,
    pub conflicts_active: bool,
}

// In run_tick, after processing council effects:
let active_policy_types: Vec<PolicyType> = self.world.active_policies
    .iter()
    .map(|p| p.policy_type)
    .collect::<std::collections::HashSet<_>>()
    .into_iter()
    .collect();

// For synergies/conflicts, we can set flags if any policies exist (simple heuristic)
// For more precision, we could track during application, but this gives good visibility.
let synergies_active = active_policy_types.len() >= 2;
let conflicts_active = active_policy_types.len() >= 2;

let mut tick_result = TickResult {
    // ...
    active_policy_count: self.world.active_policies.len(),
    active_policy_types,
    synergies_active,
    conflicts_active,
    ..Default::default()
};
