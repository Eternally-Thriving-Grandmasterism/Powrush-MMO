/*!
 * IR Management with Truncation Support
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrCategory {
    SmallRoom,
    MediumRoom,
    LargeHall,
    Cave,
    Forest,
    OpenField,
    Cathedral,
    Custom(u32),
}

#[derive(Debug, Clone)]
pub struct ImpulseResponse {
    pub name: String,
    pub category: IrCategory,
    pub duration_seconds: f32,
    pub wetness_bias: f32,
    pub early_reflection_strength: f32,
    pub asset_path: Option<String>,

    pub loaded_source: Option<Handle<AudioSource>>,
    pub early_only_source: Option<Handle<AudioSource>>, // Truncated early reflections only
}

#[derive(Resource, Default, Clone)]
pub struct IrLibrary {
    pub responses: HashMap<IrCategory, Vec<ImpulseResponse>>,
    pub default_ir: Option<ImpulseResponse>,
}

impl IrLibrary {
    pub fn new() -> Self {
        let mut lib = Self {
            responses: HashMap::new(),
            default_ir: None,
        };
        lib.add_example_irs();
        lib
    }

    fn add_example_irs(&mut self) {
        // Example entries (truncated support added)
        self.responses.entry(IrCategory::SmallRoom).or_default().push(ImpulseResponse {
            name: "small_stone_room".to_string(),
            category: IrCategory::SmallRoom,
            duration_seconds: 0.9,
            wetness_bias: 0.6,
            early_reflection_strength: 1.4,
            asset_path: Some("audio/ir/small_stone_room.wav".to_string()),
            loaded_source: None,
            early_only_source: None,
        });

        self.responses.entry(IrCategory::MediumRoom).or_default().push(ImpulseResponse {
            name: "medium_wood_hall".to_string(),
            category: IrCategory::MediumRoom,
            duration_seconds: 1.6,
            wetness_bias: 0.75,
            early_reflection_strength: 1.1,
            asset_path: Some("audio/ir/medium_wood_hall.wav".to_string()),
            loaded_source: None,
            early_only_source: None,
        });

        self.responses.entry(IrCategory::LargeHall).or_default().push(ImpulseResponse {
            name: "large_stone_hall".to_string(),
            category: IrCategory::LargeHall,
            duration_seconds: 2.8,
            wetness_bias: 0.9,
            early_reflection_strength: 0.8,
            asset_path: Some("audio/ir/large_stone_hall.wav".to_string()),
            loaded_source: None,
            early_only_source: None,
        });

        self.responses.entry(IrCategory::Forest).or_default().push(ImpulseResponse {
            name: "forest_ambience".to_string(),
            category: IrCategory::Forest,
            duration_seconds: 1.4,
            wetness_bias: 0.55,
            early_reflection_strength: 1.6,
            asset_path: Some("audio/ir/forest_ambience.wav".to_string()),
            loaded_source: None,
            early_only_source: None,
        });

        self.default_ir = self.responses.get(&IrCategory::MediumRoom)
            .and_then(|v| v.first())
            .cloned();
    }

    pub fn select_best(&self, room_size: f32, wetness: f32, biome_name: &str) -> ImpulseResponse {
        let category = match (room_size, biome_name) {
            (r, "forest" | "woods") if r < 0.6 => IrCategory::Forest,
            (r, _) if r < 0.35 => IrCategory::SmallRoom,
            (r, _) if r < 0.7 => IrCategory::MediumRoom,
            _ => IrCategory::LargeHall,
        };

        if let Some(list) = self.responses.get(&category) {
            if let Some(best) = list.iter().max_by(|a, b| {
                let score_a = (a.wetness_bias - wetness).abs() + (a.early_reflection_strength * 0.5);
                let score_b = (b.wetness_bias - wetness).abs() + (b.early_reflection_strength * 0.5);
                score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
            }) {
                return best.clone();
            }
        }

        self.default_ir.clone().unwrap_or_else(|| ImpulseResponse {
            name: "fallback".to_string(),
            category: IrCategory::MediumRoom,
            duration_seconds: 1.5,
            wetness_bias: 0.7,
            early_reflection_strength: 1.0,
            asset_path: None,
            loaded_source: None,
            early_only_source: None,
        })
    }
}

#[derive(Resource, Clone)]
pub struct CurrentImpulseResponse {
    pub active: ImpulseResponse,
}

impl Default for CurrentImpulseResponse {
    fn default() -> Self {
        Self {
            active: ImpulseResponse {
                name: "default".to_string(),
                category: IrCategory::MediumRoom,
                duration_seconds: 1.5,
                wetness_bias: 0.7,
                early_reflection_strength: 1.0,
                asset_path: None,
                loaded_source: None,
                early_only_source: None,
            },
        }
    }
}
