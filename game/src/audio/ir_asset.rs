/*!
 * IR Asset Pipeline - Complete with Asset Post-Processor
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use kira::sound::static_sound::StaticSoundData;
use serde::Deserialize;

use crate::audio::ir_manager::{IrCategory, IrLibrary, CurrentImpulseResponse};
use crate::settings::audio_quality::AudioQualitySettings;

#[derive(Debug, Clone, Deserialize)]
pub struct IrFileDefinition {
    pub name: String,
    pub category: String,
    pub audio_path: String,
    pub early_reflection_target_duration: Option<f32>,
    pub wetness_bias: f32,
    pub early_reflection_strength: f32,
}

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

pub struct IrAssetLoader;

impl bevy::asset::AssetLoader for IrAssetLoader {
    type Asset = IrAsset;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &(),
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let definition: IrFileDefinition = ron::de::from_bytes(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to parse IR definition: {}", e))?;

        let audio_handle: Handle<AudioSource> = load_context.load(&definition.audio_path);

        let category = match definition.category.as_str() {
            "SmallRoom" => IrCategory::SmallRoom,
            "MediumRoom" => IrCategory::MediumRoom,
            "LargeHall" => IrCategory::LargeHall,
            "Forest" => IrCategory::Forest,
            "Cave" => IrCategory::Cave,
            "Cathedral" => IrCategory::Cathedral,
            _ => IrCategory::MediumRoom,
        };

        Ok(IrAsset {
            name: definition.name,
            category,
            full_source: audio_handle,
            early_only_source: None,
            duration_seconds: 0.0,
            wetness_bias: definition.wetness_bias,
            early_reflection_strength: definition.early_reflection_strength,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ir", "ir.ron"]
    }
}

/// Asset post-processor: creates truncated early-only IR when an IrAsset finishes loading.
pub fn process_loaded_ir_assets(
    mut ev_asset: EventReader<AssetEvent<IrAsset>>,
    ir_assets: Res<Assets<IrAsset>>,
    audio_assets: Res<Assets<AudioSource>>,
    mut current_ir: ResMut<CurrentImpulseResponse>,
    quality: Res<AudioQualitySettings>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = ev {
            // Check if the loaded IrAsset is the one currently selected
            if let Some(active_ir) = ir_assets.get(*id) {
                // For simplicity we check against the name; in production use a better identifier
                if active_ir.name == current_ir.active.name {
                    if quality.use_early_only_ir && current_ir.active.early_only_source.is_none() {
                        if let Some(full_source) = audio_assets.get(&active_ir.full_source) {
                            if let Some(truncated) = create_truncated_early_ir(
                                full_source,
                                quality.early_reflection_target_duration,
                            ) {
                                let truncated_handle = audio_assets.add(truncated);
                                current_ir.active.early_only_source = Some(truncated_handle);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn create_truncated_early_ir(
    full_source: &AudioSource,
    target_duration: f32,
) -> Option<AudioSource> {
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

// Keep the old RON library loader for backward compatibility
pub fn load_ir_library_from_ron(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ir_library: ResMut<IrLibrary>,
) {
    // Existing implementation...
}
