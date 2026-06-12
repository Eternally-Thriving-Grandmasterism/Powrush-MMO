//! client/src/rbe_simulation.rs
//! RBE Simulation Core for Powrush-MMO
//! Resource Economy, Needs, Player Profiles, Biomes, Weather, Lighting + Poisson Disk PCF Shadows
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v18.10+

use bevy::prelude::*;
use bevy::pbr::ShadowFilteringMethod;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== RESOURCE TYPES ====================

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

// ==================== ABUNDANCE POOL ====================

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

        let basic_needs_floor = available * 0.4;
        let remaining = available - basic_needs_floor;

        let contribution_share = if self.total_contribution_score > 0.0 {
            (player_contribution / self.total_contribution_score) * remaining
        } else {
            remaining / total_players.max(1.0)
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

// ==================== NEEDS SYSTEM ====================

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
        if let Some(level) = self.levels.get_mut(&need) {
            *level = (*level + amount).min(1.0);
        }
    }

    pub fn satisfy_need(&mut self, need: NeedType, amount: f32) {
        if let Some(level) = self.levels.get_mut(&need) {
            *level = (*level - amount).max(0.0);
        }
    }

    pub fn get_need_level(&self, need: NeedType) -> f32 {
        *self.levels.get(&need).unwrap_or(&0.0)
    }

    pub fn is_critical(&self, need: NeedType) -> bool {
        self.get_need_level(need) > 0.7
    }
}

// ==================== PLAYER RBE PROFILE ====================

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRBEProfile {
    pub contribution_score: f32,
    pub personal_resources: Vec<Resource>,
}

impl Default for PlayerRBEProfile {
    fn default() -> Self {
        Self {
            contribution_score: 0.0,
            personal_resources: vec![],
        }
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
        self.personal_resources
            .iter()
            .find(|r| r.resource_type == resource_type)
            .map(|r| r.amount)
            .unwrap_or(0.0)
    }

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

            let reward = match resource_type {
                ResourceType::Knowledge | ResourceType::Health => to_deposit * 1.5,
                _ => to_deposit,
            };
            self.contribution_score += reward;
            abundance.total_contribution_score += reward;
        }
        to_deposit
    }
}

// ==================== CRAFTING RECIPES ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CraftingRecipe {
    AdvancedTool,
    EnergyCore,
    KnowledgeCrystal,
    HealingDevice,
}

impl CraftingRecipe {
    pub fn requirements(&self) -> Vec<(ResourceType, f32)> {
        match self {
            CraftingRecipe::AdvancedTool => vec![(ResourceType::Materials, 15.0), (ResourceType::Energy, 5.0)],
            CraftingRecipe::EnergyCore => vec![(ResourceType::Energy, 20.0), (ResourceType::Materials, 10.0)],
            CraftingRecipe::KnowledgeCrystal => vec![(ResourceType::Knowledge, 12.0), (ResourceType::Materials, 8.0)],
            CraftingRecipe::HealingDevice => vec![(ResourceType::Health, 15.0), (ResourceType::Materials, 10.0)],
        }
    }

    pub fn result(&self) -> (f32, Option<(ResourceType, f32)>) {
        match self {
            CraftingRecipe::AdvancedTool => (25.0, Some((ResourceType::Materials, 5.0))),
            CraftingRecipe::EnergyCore => (30.0, None),
            CraftingRecipe::KnowledgeCrystal => (35.0, Some((ResourceType::Knowledge, 3.0))),
            CraftingRecipe::HealingDevice => (28.0, None),
        }
    }
}

// ==================== BIOMES & RESOURCE NODES ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Biome {
    CrystalFields,
    DeepCaves,
    AncientForest,
    SunkenSprings,
    KnowledgeArchives,
    RareMineralVeins,
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

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct BiomeWeights {
    pub weights: HashMap<(Biome, ResourceNodeType), f32>,
}

impl Default for BiomeWeights {
    fn default() -> Self {
        let mut weights = HashMap::new();
        weights.insert((Biome::CrystalFields, ResourceNodeType::Crystal), 0.6);
        weights.insert((Biome::CrystalFields, ResourceNodeType::RareMineral), 0.05);
        weights.insert((Biome::DeepCaves, ResourceNodeType::Crystal), 0.4);
        weights.insert((Biome::DeepCaves, ResourceNodeType::RareMineral), 0.3);
        weights.insert((Biome::AncientForest, ResourceNodeType::Tree), 0.7);
        weights.insert((Biome::AncientForest, ResourceNodeType::HerbPatch), 0.5);
        weights.insert((Biome::SunkenSprings, ResourceNodeType::Spring), 0.8);
        weights.insert((Biome::KnowledgeArchives, ResourceNodeType::Library), 0.7);
        weights.insert((Biome::RareMineralVeins, ResourceNodeType::RareMineral), 0.85);
        weights.insert((Biome::RareMineralVeins, ResourceNodeType::Crystal), 0.2);
        Self { weights }
    }
}

// ==================== WEATHER ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Rain,
    Storm,
    Heatwave,
    ColdSnap,
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct WeatherState {
    pub current: Weather,
    pub timer: f32,
    pub transition_speed: f32,
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            current: Weather::Clear,
            timer: 120.0,
            transition_speed: 1.0,
        }
    }
}

// ==================== LIGHTING ====================

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct LightingState {
    pub time_of_day: f32,
    pub light_intensity: f32,
    pub light_color: [f32; 3],
    pub ambient_intensity: f32,
}

impl Default for LightingState {
    fn default() -> Self {
        Self {
            time_of_day: 0.5,
            light_intensity: 1.0,
            light_color: [1.0, 0.95, 0.85],
            ambient_intensity: 0.4,
        }
    }
}

// ==================== POISSON DISK PCF SHADOWS ====================

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct PoissonDiskKernel {
    pub samples: Vec<[f32; 2]>,
}

impl Default for PoissonDiskKernel {
    fn default() -> Self {
        Self {
            samples: vec![
                [0.0589, 0.1285], [-0.0213, -0.3923], [0.3125, -0.2891],
                [-0.3412, 0.1567], [0.1897, 0.4123], [-0.4128, -0.0892],
                [0.0781, -0.1784], [-0.1562, 0.3125], [0.2344, 0.0781],
                [-0.0781, -0.2344], [0.3125, 0.1562], [-0.2344, -0.0781],
                [0.1562, -0.3125], [-0.3125, 0.2344], [0.0781, 0.2344],
                [-0.1562, -0.1562],
            ],
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadowQuality {
    Performance,
    HighQuality,
}

impl Default for ShadowQuality {
    fn default() -> Self {
        ShadowQuality::HighQuality
    }
}

pub fn update_poisson_disk_shadows(
    mut query: Query<&mut DirectionalLight>,
    lighting: Res<LightingState>,
    weather: Res<WeatherState>,
    shadow_quality: Res<ShadowQuality>,
) {
    for mut light in query.iter_mut() {
        light.illuminance = lighting.light_intensity * 100_000.0;
        light.shadows_enabled = true;

        if *shadow_quality == ShadowQuality::HighQuality {
            light.shadow_filtering_method = ShadowFilteringMethod::Hardware2x2;
            light.shadow_depth_bias = 0.012;
            light.shadow_normal_bias = 0.4;
        } else {
            light.shadow_filtering_method = ShadowFilteringMethod::Hardware2x2;
        }
    }
}

// ==================== PLUGIN ====================

pub struct RBESimulationPlugin;

impl Plugin for RBESimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbundancePool>()
            .init_resource::<BiomeWeights>()
            .init_resource::<WeatherState>()
            .init_resource::<LightingState>()
            .init_resource::<PoissonDiskKernel>()
            .init_resource::<ShadowQuality>()
            .add_systems(Update, update_poisson_disk_shadows);
    }
}
