/*!
 * IR Management with Global Truncation Cache
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
    /// Global cache of truncated early-only IRs, keyed by IrAsset ID
    pub early_only_cache: HashMap<AssetId<crate::audio::ir_asset::IrAsset>, Handle<AudioSource>>,
}

impl IrLibrary {
    pub fn new() -> Self {
        let mut lib = Self {
            responses: HashMap::new(),
            default_ir: None,
            early_only_cache: HashMap::new(),
        };
        lib.add_example_irs();
        lib
    }

    fn add_example_irs(&mut self) {
        // ... (example data)
    }

    pub fn select_best(&self, room_size: f32, wetness: f32, biome_name: &str) -> ImpulseResponse {
        // ... (selection logic)
    }

    /// Store a truncated version in the global cache
    pub fn cache_early_only(
        &mut self,
        ir_asset_id: AssetId<crate::audio::ir_asset::IrAsset>,
        truncated_handle: Handle<AudioSource>,
    ) {
        self.early_only_cache.insert(ir_asset_id, truncated_handle);
    }

    /// Retrieve a cached truncated version if available
    pub fn get_cached_early_only(
        &self,
        ir_asset_id: AssetId<crate::audio::ir_asset::IrAsset>,
    ) -> Option<Handle<AudioSource>> {
        self.early_only_cache.get(&ir_asset_id).cloned()
    }
}

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
