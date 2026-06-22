/*!
 * ActivePolicy now supports Spatial Targeting.
 */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActivePolicy {
    pub decision_id: u64,
    pub policy_type: PolicyType,
    pub target_faction: Option<u32>,
    pub target_interest_zone: Option<u64>,  // NEW: spatial targeting
    pub strength: f32,
    pub remaining_ticks: u64,
    pub created_tick: u64,
}

// When creating policies, we can now pass target_interest_zone
// Example:
// world.active_policies.push(ActivePolicy {
//     ...
//     target_interest_zone: Some(zone_id),
//     ...
// });
