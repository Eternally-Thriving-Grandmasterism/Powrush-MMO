/*!
 * TickResult now tracks zones with active visual highlighting from policies.
 */

#[derive(Debug, Default, Clone)]
pub struct TickResult {
    // ... existing fields ...
    pub active_policy_count: usize,
    pub active_policy_types: Vec<PolicyType>,
    pub synergies_active: bool,
    pub conflicts_active: bool,

    pub zones_with_visual_highlight: usize,  // NEW: zones currently highlighted by policies
}

// In run_tick, after world updates:
let zones_with_visual_highlight = self.world.interest_zones
    .values()
    .filter(|z| z.visual_highlight > 0.15)
    .count();

let mut tick_result = TickResult {
    // ...
    zones_with_visual_highlight,
    ..Default::default()
};
