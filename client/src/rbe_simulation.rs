/*!
 * RBE Simulation Core for Powrush-MMO
 *
 * Player Inventory ↔ RBE Integration
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        Self { resources: vec![], total_contribution_score: 0.0 }
    }

    pub fn add_resource(&mut self, resource_type: ResourceType, amount: f32) {
        if let Some(existing) = self.resources.iter_mut().find(|r| r.resource_type == resource_type) {
            existing.amount += amount;
        } else {
            self.resources.push(Resource { resource_type, amount });
        }
    }

    pub fn get_amount(&self, resource_type: ResourceType) -> f32 {
        self.resources.iter().find(|r| r.resource_type == resource_type).map(|r| r.amount).unwrap_or(0.0)
    }

    pub fn advanced_distribute(
        &mut self,
        need: ResourceType,
        requested_amount: f32,
        player_contribution: f32,
        total_players: f32,
    ) -> f32 {
        let available = self.get_amount(need);
        if available <= 0.0 { return 0.0; }

        let basic_needs_floor = available * 0.4;
        let remaining_after_floor = available - basic_needs_floor;

        let contribution_share = if self.total_contribution_score > 0.0 {
            (player_contribution / self.total_contribution_score) * remaining_after_floor
        } else {
            remaining_after_floor / total_players
        };

        let allocated = requested_amount.min(basic_needs_floor + contribution_share).min(available);

        if let Some(resource) = self.resources.iter_mut().find(|r| r.resource_type == need) {
            resource.amount -= allocated;
        }

        allocated
    }

    /// Allow a player to withdraw resources into their personal inventory
    pub fn withdraw_for_player(
        &mut self,
        resource_type: ResourceType,
        amount: f32,
        player_contribution: f32,
        total_players: f32,
    ) -> f32 {
        self.advanced_distribute(resource_type, amount, player_contribution, total_players)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeedType {
    Hunger,
    Energy,
    Thirst,
    Knowledge,
    Health,
    Shelter,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Needs {
    pub levels: HashMap<NeedType, f32>,
}

impl Default for Needs {
    fn default() -> Self {
        let mut levels = HashMap::new();
        levels.insert(NeedType::Hunger, 0.3);
        levels.insert(NeedType::Energy, 0.25);
        levels.insert(NeedType::Thirst, 0.2);
        levels.insert(NeedType::Knowledge, 0.4);
        levels.insert(NeedType::Health, 0.15);
        levels.insert(NeedType::Shelter, 0.1);
        Self { levels }
    }
}

impl Needs {
    pub fn increase_need(&mut self, need: NeedType, amount: f32) {
        if let Some(level) = self.levels.get_mut(&need) { *level = (*level + amount).min(1.0); }
    }
    pub fn satisfy_need(&mut self, need: NeedType, amount: f32) {
        if let Some(level) = self.levels.get_mut(&need) { *level = (*level - amount).max(0.0); }
    }
    pub fn get_need_level(&self, need: NeedType) -> f32 {
        *self.levels.get(&need).unwrap_or(&0.0)
    }
    pub fn is_critical(&self, need: NeedType) -> bool {
        self.get_need_level(need) > 0.7
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRBEProfile {
    pub contribution_score: f32,
    pub personal_resources: Vec<Resource>,
}

impl Default for PlayerRBEProfile {
    fn default() -> Self {
        Self { contribution_score: 0.0, personal_resources: vec![] }
    }
}

impl PlayerRBEProfile {
    pub fn add_personal_resource(&mut self, resource_type: ResourceType, amount: f32) {
        if let Some(existing) = self.personal_resources.iter_mut().find(|r| r.resource_type == resource_type) {
            existing.amount += amount;
        } else {
            self.personal_resources.push(Resource { resource_type, amount });
        }
    }

    pub fn get_personal_amount(&self, resource_type: ResourceType) -> f32 {
        self.personal_resources.iter().find(|r| r.resource_type == resource_type).map(|r| r.amount).unwrap_or(0.0)
    }

    /// Withdraw resources from the Abundance Pool into personal inventory
    pub fn withdraw_from_pool(
        &mut self,
        abundance: &mut AbundancePool,
        resource_type: ResourceType,
        amount: f32,
        total_players: f32,
    ) -> f32 {
        let allocated = abundance.withdraw_for_player(
            resource_type,
            amount,
            self.contribution_score,
            total_players,
        );

        if allocated > 0.0 {
            self.add_personal_resource(resource_type, allocated);
        }

        allocated
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContributionActionType {
    Gathering,
    Crafting,
    Teaching,
    Building,
    Healing,
    Research,
    CommunitySupport,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PerformedContributionAction {
    pub action_type: ContributionActionType,
    pub intensity: f32,
}

pub fn process_contribution_actions(
    mut query: Query<(&mut PlayerRBEProfile, Option<&PerformedContributionAction>)>,
    mut abundance: ResMut<AbundancePool>,
) {
    for (mut profile, action) in query.iter_mut() {
        if let Some(action) = action {
            let reward = match action.action_type {
                ContributionActionType::Gathering => 0.8 * action.intensity,
                ContributionActionType::Crafting => 1.2 * action.intensity,
                ContributionActionType::Teaching => 1.5 * action.intensity,
                ContributionActionType::Building => 1.3 * action.intensity,
                ContributionActionType::Healing => 1.6 * action.intensity,
                ContributionActionType::Research => 1.4 * action.intensity,
                ContributionActionType::CommunitySupport => 1.0 * action.intensity,
            };

            profile.contribution_score += reward;
            abundance.total_contribution_score += reward;

            if action.action_type == ContributionActionType::Gathering {
                abundance.add_resource(ResourceType::Materials, 2.0 * action.intensity);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceNodeType {
    Tree,
    Crystal,
    Spring,
    HerbPatch,
    Library,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WorldResourceNode {
    pub node_type: ResourceNodeType,
    pub remaining_resources: f32,
    pub regeneration_rate: f32,
}

impl WorldResourceNode {
    pub fn new(node_type: ResourceNodeType) -> Self {
        let (remaining, regen) = match node_type {
            ResourceNodeType::Tree => (100.0, 0.5),
            ResourceNodeType::Crystal => (80.0, 0.3),
            ResourceNodeType::Spring => (150.0, 0.8),
            ResourceNodeType::HerbPatch => (60.0, 0.4),
            ResourceNodeType::Library => (200.0, 0.2),
        };
        Self { node_type, remaining_resources: remaining, regeneration_rate: regen }
    }
}

pub fn regenerate_resource_nodes(mut query: Query<&mut WorldResourceNode>) {
    for mut node in query.iter_mut() {
        if node.remaining_resources < 200.0 {
            node.remaining_resources += node.regeneration_rate;
        }
    }
}

pub fn rbe_simulation_step(
    mut abundance: ResMut<AbundancePool>,
    mut query: Query<(&mut PlayerRBEProfile, &mut Needs)>,
) {
    let player_count = query.iter().count() as f32;

    for (mut profile, mut needs) in query.iter_mut() {
        needs.increase_need(NeedType::Hunger, 0.02);
        needs.increase_need(NeedType::Energy, 0.015);
        needs.increase_need(NeedType::Thirst, 0.018);

        if needs.is_critical(NeedType::Hunger) {
            let allocated = abundance.advanced_distribute(ResourceType::Food, 15.0, profile.contribution_score, player_count);
            if allocated > 0.0 { needs.satisfy_need(NeedType::Hunger, 0.4); }
        }

        if needs.is_critical(NeedType::Energy) {
            let allocated = abundance.advanced_distribute(ResourceType::Energy, 12.0, profile.contribution_score, player_count);
            if allocated > 0.0 { needs.satisfy_need(NeedType::Energy, 0.35); }
        }

        profile.contribution_score += 0.005;
        abundance.total_contribution_score += 0.005;
    }

    abundance.add_resource(ResourceType::Energy, 1.2);
    abundance.add_resource(ResourceType::Food, 1.0);
    abundance.add_resource(ResourceType::Water, 0.8);
}

pub struct RBESimulationPlugin;

impl Plugin for RBESimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbundancePool>()
            .add_systems(Update, (rbe_simulation_step, process_contribution_actions, regenerate_resource_nodes));
    }
}
