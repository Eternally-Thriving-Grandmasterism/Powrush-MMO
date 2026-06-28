/*!
 * IR Management - Improved with Asset ID tracking
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
    pub early_only_source: Option<Handle<AudioSource>>,
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
        // ... (example data unchanged)
    }

    pub fn select_best(&self, room_size: f32, wetness: f32, biome_name: &str) -> ImpulseResponse {
        // ... (selection logic unchanged)
    }
}

/// Now tracks the active IrAsset handle for reliable matching in post-processors
#[derive(Resource, Clone)]
pub struct CurrentImpulseResponse {
    pub active: ImpulseResponse,
    pub active_ir_asset: Option<Handle<crate::audio::ir_asset::IrAsset>>,
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
            active_ir_asset: None,
        }
    }
}
