/*!
 * RBE Simulation Core for Powrush-MMO
 *
 * Rare Mineral Crafting Recipes
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

    pub fn withdraw_from_pool(
        &mut self,
        abundance: &mut AbundancePool,
        resource_type: ResourceType,
        amount: f32,
        total_players: f32,
    ) -> f32 {
        let allocated = abundance.withdraw_for_player(resource_type, amount, self.contribution_score, total_players);
        if allocated > 0.0 {
            self.add_personal_resource(resource_type, allocated);
        }
        allocated
    }

    pub fn deposit_to_pool(
        &mut self,
        abundance: &mut AbundancePool,
        resource_type: ResourceType,
        amount: f32,
    ) -> f32 {
        let available = self.get_personal_amount(resource_type);
        let to_deposit = amount.min(available);

        if to_deposit > 0.0 {
            if let Some(existing) = self.personal_resources.iter_mut().find(|r| r.resource_type == resource_type) {
                existing.amount -= to_deposit;
                if existing.amount <= 0.0 {
                    self.personal_resources.retain(|r| r.resource_type != resource_type);
                }
            }

            abundance.add_resource(resource_type, to_deposit);

            let contribution_reward = match resource_type {
                ResourceType::Knowledge | ResourceType::Health => to_deposit * 1.5,
                _ => to_deposit,
            };

            self.contribution_score += contribution_reward;
            abundance.total_contribution_score += contribution_reward;
        }

        to_deposit
    }

    /// Craft an item using rare minerals and other resources
    pub fn craft_recipe(
        &mut self,
        recipe: CraftingRecipe,
    ) -> bool {
        let requirements = recipe.requirements();

        // Check if player has all required resources
        for (resource_type, amount_needed) in &requirements {
            if self.get_personal_amount(*resource_type) < *amount_needed {
                return false; // Not enough resources
            }
        }

        // Consume resources
        for (resource_type, amount_needed) in &requirements {
            if let Some(existing) = self.personal_resources.iter_mut().find(|r| r.resource_type == *resource_type) {
                existing.amount -= amount_needed;
                if existing.amount <= 0.0 {
                    self.personal_resources.retain(|r| r.resource_type != *resource_type);
                }
            }
        }

        // Apply result (for now we just reward contribution + optionally add a result resource)
        let (contribution_reward, result_resource) = recipe.result();

        self.contribution_score += contribution_reward;

        if let Some((result_type, result_amount)) = result_resource {
            self.add_personal_resource(result_type, result_amount);
        }

        true
    }
}

/// Crafting recipes that can use Rare Minerals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CraftingRecipe {
    AdvancedTool,        // Requires Rare Mineral + Materials
    EnergyCore,          // Requires Rare Mineral + Energy
    KnowledgeCrystal,    // Requires Rare Mineral + Knowledge
    HealingDevice,       // Requires Rare Mineral + Health
}

impl CraftingRecipe {
    pub fn requirements(&self) -> Vec<(ResourceType, f32)> {
        match self {
            CraftingRecipe::AdvancedTool => {
                vec![(ResourceType::Materials, 15.0), (ResourceType::Energy, 5.0)]
            }
            CraftingRecipe::EnergyCore => {
                vec![(ResourceType::Energy, 20.0), (ResourceType::Materials, 10.0)]
            }
            CraftingRecipe::KnowledgeCrystal => {
                vec![(ResourceType::Knowledge, 12.0), (ResourceType::Materials, 8.0)]
            }
            CraftingRecipe::HealingDevice => {
                vec![(ResourceType::Health, 15.0), (ResourceType::Materials, 10.0)]
            }
        }
    }

    pub fn result(&self) -> (f32, Option<(ResourceType, f32)>) {
        match self {
            CraftingRecipe::AdvancedTool => (25.0, Some((ResourceType::Materials, 5.0))), // High contribution + leftover
            CraftingRecipe::EnergyCore => (30.0, None),
            CraftingRecipe::KnowledgeCrystal => (35.0, Some((ResourceType::Knowledge, 3.0))),
            CraftingRecipe::HealingDevice => (28.0, None),
        }
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
    RareMineral,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WorldResourceNode {
    pub node_type: ResourceNodeType,
    pub remaining_resources: f32,
    pub regeneration_rate: f32,
    pub max_resources: f32,
}

impl WorldResourceNode {
    pub fn new(node_type: ResourceNodeType) -> Self {
        let (remaining, regen, max_res) = match node_type {
            ResourceNodeType::Tree => (100.0, 0.5, 120.0),
            ResourceNodeType::Crystal => (80.0, 0.3, 100.0),
            ResourceNodeType::Spring => (150.0, 0.8, 200.0),
            ResourceNodeType::HerbPatch => (60.0, 0.4, 80.0),
            ResourceNodeType::Library => (200.0, 0.2, 250.0),
            ResourceNodeType::RareMineral => (40.0, 0.1, 50.0),
        };
        Self { node_type, remaining_resources: remaining, regeneration_rate: regen, max_resources: max_res }
    }
}

pub fn regenerate_resource_nodes(mut query: Query<&mut WorldResourceNode>) {
    for mut node in query.iter_mut() {
        if node.remaining_resources < node.max_resources {
            node.remaining_resources = (node.remaining_resources + node.regeneration_rate)
                .min(node.max_resources);
        }
    }
}

#[derive(Event)]
pub struct GatherFromNodeEvent {
    pub node_entity: Entity,
    pub gather_amount: f32,
}

pub fn handle_gather_from_node(
    mut commands: Commands,
    mut abundance: ResMut<AbundancePool>,
    mut events: EventReader<GatherFromNodeEvent>,
    mut node_query: Query<(Entity, &mut WorldResourceNode)>,
    mut profile_query: Query<&mut PlayerRBEProfile>,
) {
    for event in events.read() {
        if let Ok((entity, mut node)) = node_query.get_mut(event.node_entity) {
            if node.remaining_resources >= event.gather_amount {
                node.remaining_resources -= event.gather_amount;

                match node.node_type {
                    ResourceNodeType::Tree => {
                        abundance.add_resource(ResourceType::Food, event.gather_amount * 0.6);
                        abundance.add_resource(ResourceType::Materials, event.gather_amount * 0.4);
                    }
                    ResourceNodeType::Crystal => {
                        abundance.add_resource(ResourceType::Energy, event.gather_amount * 0.7);
                        abundance.add_resource(ResourceType::Knowledge, event.gather_amount * 0.3);
                    }
                    ResourceNodeType::Spring => {
                        abundance.add_resource(ResourceType::Water, event.gather_amount);
                    }
                    ResourceNodeType::HerbPatch => {
                        abundance.add_resource(ResourceType::Health, event.gather_amount * 0.8);
                        abundance.add_resource(ResourceType::Food, event.gather_amount * 0.2);
                    }
                    ResourceNodeType::Library => {
                        abundance.add_resource(ResourceType::Knowledge, event.gather_amount);
                    }
                    ResourceNodeType::RareMineral => {
                        abundance.add_resource(ResourceType::Materials, event.gather_amount * 0.8);
                        abundance.add_resource(ResourceType::Energy, event.gather_amount * 0.2);
                    }
                }

                let contribution_reward = if node.node_type == ResourceNodeType::RareMineral {
                    2.0
                } else {
                    0.8
                };

                for mut profile in profile_query.iter_mut() {
                    profile.contribution_score += contribution_reward;
                    abundance.total_contribution_score += contribution_reward;
                }

                if node.remaining_resources <= 0.0 {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

#[derive(Event)]
pub struct ResourceDepositedEvent {
    pub resource_type: ResourceType,
    pub amount: f32,
    pub contribution_gained: f32,
}

pub fn deposit_visual_feedback(
    mut commands: Commands,
    mut deposit_events: EventReader<ResourceDepositedEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in deposit_events.read() {
        commands.spawn((
            TextBundle {
                text: Text::from_section(
                    format!("+{:.1} {:?} | +{:.1} Contribution", event.amount, event.resource_type, event.contribution_gained),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 18.0,
                        color: Color::srgb(0.4, 0.9, 0.6),
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(45.0),
                    top: Val::Percent(35.0),
                    ..default()
                },
                ..default()
            },
            DepositVisualEffect {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

#[derive(Component)]
pub struct DepositVisualEffect {
    pub timer: Timer,
}

pub fn cleanup_deposit_effects(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DepositVisualEffect, &mut Text)>,
    time: Res<Time>,
) {
    for (entity, mut effect, mut text) in query.iter_mut() {
        effect.timer.tick(time.delta());
        let alpha = 1.0 - effect.timer.percent();
        text.sections[0].style.color.set_a(alpha);

        if effect.timer.finished() {
            commands.entity(entity).despawn();
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
            .add_event::<GatherFromNodeEvent>()
            .add_event::<ResourceDepositedEvent>()
            .add_systems(Update, (
                rbe_simulation_step,
                process_contribution_actions,
                regenerate_resource_nodes,
                handle_gather_from_node,
                deposit_visual_feedback,
                cleanup_deposit_effects,
            ));
    }
}
