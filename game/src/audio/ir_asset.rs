/*!
 * IR Asset Pipeline with Truncation Logic
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use serde::{Deserialize, Serialize};

use crate::audio::ir_manager::{IrCategory, ImpulseResponse, IrLibrary, CurrentImpulseResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrDefinition {
    pub name: String,
    pub category: String,
    pub asset_path: String,
    pub duration_seconds: f32,
    pub wetness_bias: f32,
    pub early_reflection_strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct IrLibraryDefinition {
    pub version: u32,
    pub impulse_responses: Vec<IrDefinition>,
}

/// Loads IR library definition from RON and populates IrLibrary
pub fn load_ir_library_from_ron(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ir_library: ResMut<IrLibrary>,
) {
    let definition: IrLibraryDefinition = ron::from_str(
        r#"
        (
            version: 1,
            impulse_responses: [
                (name: "small_stone_room", category: "SmallRoom", asset_path: "audio/ir/small_stone_room.wav", duration_seconds: 0.9, wetness_bias: 0.6, early_reflection_strength: 1.4),
                (name: "medium_wood_hall", category: "MediumRoom", asset_path: "audio/ir/medium_wood_hall.wav", duration_seconds: 1.6, wetness_bias: 0.75, early_reflection_strength: 1.1),
                (name: "large_stone_hall", category: "LargeHall", asset_path: "audio/ir/large_stone_hall.wav", duration_seconds: 2.8, wetness_bias: 0.9, early_reflection_strength: 0.8),
                (name: "forest_ambience", category: "Forest", asset_path: "audio/ir/forest_ambience.wav", duration_seconds: 1.4, wetness_bias: 0.55, early_reflection_strength: 1.6),
            ],
        )
        "#,
    ).unwrap_or_else(|_| IrLibraryDefinition { version: 1, impulse_responses: vec![] });

    ir_library.responses.clear();

    for def in definition.impulse_responses {
        let category = match def.category.as_str() {
            "SmallRoom" => IrCategory::SmallRoom,
            "MediumRoom" => IrCategory::MediumRoom,
            "LargeHall" => IrCategory::LargeHall,
            "Forest" => IrCategory::Forest,
            "Cave" => IrCategory::Cave,
            "Cathedral" => IrCategory::Cathedral,
            _ => IrCategory::MediumRoom,
        };

        let ir = ImpulseResponse {
            name: def.name,
            category,
            duration_seconds: def.duration_seconds,
            wetness_bias: def.wetness_bias,
            early_reflection_strength: def.early_reflection_strength,
            asset_path: Some(def.asset_path),
            loaded_source: None,
            early_only_source: None,
        };
        ir_library.responses.entry(category).or_default().push(ir);
    }

    if ir_library.default_ir.is_none() {
        if let Some(list) = ir_library.responses.get(&IrCategory::MediumRoom) {
            ir_library.default_ir = list.first().cloned();
        }
    }
}

/// Ensures the current IR has its full audio loaded, and optionally creates a truncated early-only version.
pub fn ensure_ir_loaded(
    mut current_ir: ResMut<CurrentImpulseResponse>,
    asset_server: Res<AssetServer>,
    quality: Res<crate::settings::audio_quality::AudioQualitySettings>,
) {
    if current_ir.active.loaded_source.is_none() {
        if let Some(path) = &current_ir.active.asset_path {
            let handle: Handle<AudioSource> = asset_server.load(path);
            current_ir.active.loaded_source = Some(handle);
        }
    }

    // Truncation logic: create early-only version if enabled and not yet created
    if quality.use_early_only_ir && current_ir.active.early_only_source.is_none() && current_ir.active.loaded_source.is_some() {
        // In a full implementation we would process the AudioSource samples here
        // For now we reuse the full source (truncation can be added with sample processing)
        current_ir.active.early_only_source = current_ir.active.loaded_source.clone();
    }
}
