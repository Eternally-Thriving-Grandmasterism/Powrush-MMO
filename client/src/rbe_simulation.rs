/*!
 * RBE Simulation Core for Powrush-MMO
 *
 * Advanced Distribution Logic — Needs-based + Contribution-weighted allocation.
 * Designed with mercy, fairness, and sustainable abundance.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Energy,
    Food,
    Water,
    Materials,
    Knowledge,
    Health,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub amount: f32,
}

#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct AbundancePool {
    pub resources: Vec<Resource>,
    pub total_contribution_score: f32,
}

impl AbundancePool {
    pub fn new() -> Self {
        Self {
            resources: vec![],
            total_contribution_score: 0.0,
        }
    }

    pub fn add_resource(&mut self, resource_type: ResourceType, amount: f32) {
        if let Some(existing) = self.resources.iter_mut().find(|r| r.resource_type == resource_type) {
            existing.amount += amount;
        } else {
            self.resources.push(Resource { resource_type, amount });
        }
    }

    pub fn get_amount(&self, resource_type: ResourceType) -> f32 {
        self.resources
            .iter()
            .find(|r| r.resource_type == resource_type)
            .map(|r| r.amount)
            .unwrap_or(0.0)
    }

    /// === ADVANCED DISTRIBUTION LOGIC ===
    ///
    /// Distributes resources using a mercy-aligned, needs-first + contribution-weighted model.
    ///
    /// Priority:
    /// 1. Basic Needs Floor (everyone gets minimum to survive/thrive)
    /// 2. Contribution-weighted additional allocation
    /// 3. Prevents hoarding while rewarding voluntary participation
    pub fn advanced_distribute(
        &mut self,
        need: ResourceType,
        requested_amount: f32,
        player_contribution: f32,
        total_players: f32,
    ) -> f32 {
        let available = self.get_amount(need);
        if available <= 0.0 {
            return 0.0;
        }

        // 1. Calculate Basic Needs Floor (e.g. 40% of available goes to universal access)
        let basic_needs_floor = available * 0.4;
        let remaining_after_floor = available - basic_needs_floor;

        // 2. Contribution-weighted share of the remaining pool
        let contribution_share = if self.total_contribution_score > 0.0 {
            (player_contribution / self.total_contribution_score) * remaining_after_floor
        } else {
            remaining_after_floor / total_players
        };

        // 3. Final allocation (min of request, available after calculations)
        let allocated = requested_amount.min(basic_needs_floor + contribution_share).min(available);

        // Deduct from pool
        if let Some(resource) = self.resources.iter_mut().find(|r| r.resource_type == need) {
            resource.amount -= allocated;
        }

        allocated
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRBEProfile {
    pub contribution_score: f32,
    pub needs_met: f32,
    pub personal_resources: Vec<Resource>,
}

impl Default for PlayerRBEProfile {
    fn default() -> Self {
        Self {
            contribution_score: 0.0,
            needs_met: 1.0,
            personal_resources: vec![],
        }
    }
}

/// Advanced RBE Simulation Step using the new distribution logic
pub fn rbe_simulation_step(
    mut abundance: ResMut<AbundancePool>,
    mut query: Query<&mut PlayerRBEProfile>,
) {
    let player_count = query.iter().count() as f32;

    for mut profile in query.iter_mut() {
        // Simulate requesting resources based on needs
        let food_allocated = abundance.advanced_distribute(
            ResourceType::Food,
            10.0, // requested
            profile.contribution_score,
            player_count,
        );

        // Update profile based on allocation
        if food_allocated > 5.0 {
            profile.needs_met = (profile.needs_met + 0.1).min(1.0);
        }

        // Gradual contribution growth (voluntary participation)
        profile.contribution_score += 0.02;
        abundance.total_contribution_score += 0.02;
    }

    // Sustainable regeneration
    abundance.add_resource(ResourceType::Energy, 1.0);
    abundance.add_resource(ResourceType::Food, 0.8);
}

pub struct RBESimulationPlugin;

impl Plugin for RBESimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbundancePool>()
            .add_systems(Update, rbe_simulation_step);
    }
}
