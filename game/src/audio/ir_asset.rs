/*!
 * IR Asset Pipeline - With Metrics & Logging
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy_kira_audio::AudioSource;
use kira::sound::static_sound::StaticSoundData;
use serde::Deserialize;
use tracing::{debug, info};

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
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let definition: IrFileDefinition = ron::de::from_bytes(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to parse IR definition: {}", e))?;

        let audio_handle: Handle<AudioSource> = load_context.load(&definition.audio_path);

        let target_dur = definition.early_reflection_target_duration.unwrap_or(0.12);

        // Attempt immediate truncation inside loader (async best path)
        let early_only_source = if let Some(loaded_audio) = load_context.get_dependency(&audio_handle) {
            if let Some(truncated) = create_truncated_early_ir(loaded_audio, target_dur) {
                let handle = load_context.add_asset(truncated);
                info!(
                    "[IR Loader] Created early-only IR inside loader for '{}' (target: {:.0}ms)",
                    definition.name, target_dur * 1000.0
                );
                Some(handle)
            } else {
                debug!("[IR Loader] Truncation failed for '{}' (target: {:.0}ms)", definition.name, target_dur * 1000.0);
                None
            }
        } else {
            // Async fallback - post-processor will handle it
            debug!(
                "[IR Loader] Async fallback for '{}' - truncation deferred to post-processor (target: {:.0}ms)",
                definition.name, target_dur * 1000.0
            );
            None
        };

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
            early_only_source,
            duration_seconds: 0.0,
            wetness_bias: definition.wetness_bias,
            early_reflection_strength: definition.early_reflection_strength,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ir", "ir.ron"]
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

/// Post-processor with logging
pub fn process_loaded_ir_assets(
    mut ev_asset: EventReader<AssetEvent<IrAsset>>,
    ir_assets: Res<Assets<IrAsset>>,
    audio_assets: Res<Assets<AudioSource>>,
    mut current_ir: ResMut<CurrentImpulseResponse>,
    quality: Res<AudioQualitySettings>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = ev {
            if let Some(active_handle) = &current_ir.active_ir_asset {
                if active_handle.id() == *id {
                    if quality.use_early_only_ir && current_ir.active.early_only_source.is_none() {
                        if let Some(ir_asset) = ir_assets.get(*id) {
                            if let Some(full_source) = audio_assets.get(&ir_asset.full_source) {
                                if let Some(truncated) = create_truncated_early_ir(
                                    full_source,
                                    quality.early_reflection_target_duration,
                                ) {
                                    let truncated_handle = audio_assets.add(truncated);
                                    current_ir.active.early_only_source = Some(truncated_handle);

                                    info!(
                                        "[IR Post-Processor] Created early-only IR for '{}' via post-processor (target: {:.0}ms)",
                                        ir_asset.name,
                                        quality.early_reflection_target_duration * 1000.0
                                    );
                                } else {
                                    debug!("[IR Post-Processor] Truncation skipped for '{}' (already sufficient length)", ir_asset.name);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn load_ir_library_from_ron(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ir_library: ResMut<IrLibrary>,
) {
    // Existing implementation...
}
