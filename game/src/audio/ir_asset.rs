/*!
 * Custom IR Asset Loader with Built-in Truncation
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadContext, io::Reader, AssetReaderError};
use bevy_kira_audio::AudioSource;
use kira::sound::static_sound::StaticSoundData;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::audio::ir_manager::{IrCategory, ImpulseResponse, IrLibrary, CurrentImpulseResponse};

// ... (IrDefinition and IrLibraryDefinition remain the same)

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

/// Custom asset type that holds both full and early-only versions of an impulse response.
#[derive(Asset, TypePath, Debug, Clone)]
pub struct IrAsset {
    pub name: String,
    pub category: IrCategory,
    pub full_source: Handle<AudioSource>,
    pub early_only_source: Option<Handle<AudioSource>>,
    pub duration_seconds: f32,
    pub wetness_bias: f32,
    pub early_reflection_strength: f32,
}

/// Custom loader for .ir files or RON-based IR definitions with built-in truncation.
pub struct IrAssetLoader;

impl AssetLoader for IrAssetLoader {
    type Asset = IrAsset;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        // For simplicity, this example assumes the asset file contains a path to the actual audio
        // In a real implementation you would parse a small RON/JSON header + load the audio dependency.

        // Placeholder implementation - in production this would:
        // 1. Parse header (name, category, target early duration, etc.)
        // 2. Load the audio file as a dependency
        // 3. Create full AudioSource handle
        // 4. Create truncated early-only AudioSource handle
        // 5. Return IrAsset

        // For now we return a minimal valid IrAsset (real logic would go here)
        let full_handle: Handle<AudioSource> = load_context.load("placeholder.wav"); // Replace with real loading

        Ok(IrAsset {
            name: "placeholder".to_string(),
            category: IrCategory::MediumRoom,
            full_source: full_handle.clone(),
            early_only_source: Some(full_handle), // Would be truncated version
            duration_seconds: 1.5,
            wetness_bias: 0.7,
            early_reflection_strength: 1.0,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ir", "ir.ron"]
    }
}

// Keep existing RON library loading for now
pub fn load_ir_library_from_ron(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ir_library: ResMut<IrLibrary>,
) {
    // ... existing implementation ...
}

pub fn create_truncated_early_ir(
    full_source: &AudioSource,
    target_duration: f32,
) -> Option<AudioSource> {
    // Existing truncation helper (kept for compatibility)
    let static_data = match full_source.sound.clone().try_into_static() {
        Ok(data) => data,
        Err(_) => return None,
    };

    let sample_rate = static_data.sample_rate as f32;
    let target_samples = (target_duration * sample_rate) as usize;

    if target_samples == 0 || target_samples >= static_data.frames.len() {
        return None;
    }

    let truncated_frames = static_data.frames[..target_samples].to_vec();

    let truncated_data = StaticSoundData {
        frames: truncated_frames,
        sample_rate: static_data.sample_rate,
        ..static_data
    };

    Some(AudioSource {
        sound: kira::sound::SoundData::Static(truncated_data),
    })
}

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
}
