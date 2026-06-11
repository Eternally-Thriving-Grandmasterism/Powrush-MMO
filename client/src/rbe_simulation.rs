/*!
 * RBE Simulation Core for Powrush-MMO
 *
 * Foundational systems for a true Resource-Based Economy.
 * Designed with mercy, abundance, and voluntary contribution at its core.
 *
 * PATSAGi Council approved architecture.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Core resource types in the RBE system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Energy,
    Food,
    Water,
    Materials,
    Knowledge,
    Health,
    // Extend as needed
}

/// Represents a quantity of a specific resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub amount: f32,
}

/// Global Abundance Pool — the heart of the RBE
#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct AbundancePool {
    pub resources: Vec<Resource>,
    pub total_contribution_score: f32, // Non-coercive contribution tracking
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

    /// Distribute resources based on need + voluntary contribution (core RBE principle)
    pub fn distribute(&mut self, need: ResourceType, amount_requested: f32, contribution_factor: f32) -> f32 {
        let available = self.get_amount(need);
        let allocated = (amount_requested * contribution_factor).min(available);

        if let Some(resource) = self.resources.iter_mut().find(|r| r.resource_type == need) {
            resource.amount -= allocated;
        }

        allocated
    }
}

/// Per-player economy profile in the RBE system
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRBEProfile {
    pub contribution_score: f32,      // Voluntary contribution (not forced labor)
    pub needs_met: f32,               // How well basic needs are currently met (0.0 - 1.0)
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

/// Main RBE Simulation System
pub fn rbe_simulation_step(
    mut abundance: ResMut<AbundancePool>,
    mut query: Query<&mut PlayerRBEProfile>,
) {
    // Example simulation step — can be expanded greatly
    for mut profile in query.iter_mut() {
        // Simple needs satisfaction logic
        if profile.needs_met < 1.0 {
            profile.needs_met = (profile.needs_met + 0.05).min(1.0);
        }

        // Contribution slowly increases over time (voluntary participation)
        profile.contribution_score += 0.01;
    }

    // Global abundance slowly regenerates (sustainable RBE principle)
    abundance.add_resource(ResourceType::Energy, 0.5);
    abundance.add_resource(ResourceType::Food, 0.3);
}

/// Plugin to register RBE systems
pub struct RBESimulationPlugin;

impl Plugin for RBESimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbundancePool>()
            .add_systems(Update, rbe_simulation_step);
    }
}
