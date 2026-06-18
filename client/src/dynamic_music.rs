/*!
 * Dynamic Music System for Powrush-MMO
 *
 * MusicLayerRegistry wired into activation flow.
 *
 * v19.03 — activate_music_layers now uses the registry.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use crate::oddio_backend::OddioAudioBackend;
use std::collections::HashMap;

// MusicLayerHandle, MusicLayerAssetData, MusicLayerRegistry definitions...

pub fn activate_music_layers(
    mut controller: ResMut<DynamicMusicController>,
    mut registry: ResMut<MusicLayerRegistry>,
    backend: Res<OddioAudioBackend>,
) {
    for (layer_type, layer) in controller.layers.iter_mut() {
        let should_play = layer.target_volume > 0.035;

        if should_play && !layer.is_playing {
            match backend.play_audio_file(&mut registry, &layer.asset, layer.target_volume, true) {
                Ok(handle) => {
                    layer.handle = Some(handle);
                    layer.is_playing = true;
                    layer.current_volume = layer.target_volume;
                    info!("🎵 Loaded via registry: {} (type: {:?})", layer.asset.filename(), layer_type);
                }
                Err(_) => {
                    let frequency = match layer_type {
                        MusicLayerType::BaseHarmony => 55.0,
                        MusicLayerType::AttunementPads => 110.0,
                        MusicLayerType::RhythmicPulse => 220.0,
                        MusicLayerType::BloomResonance => 330.0,
                    };

                    let handle = backend.play_procedural_layer(frequency, layer.target_volume, true);
                    layer.handle = Some(handle);
                    layer.is_playing = true;
                    layer.current_volume = layer.target_volume;

                    info!("🎵 Procedural fallback for layer: {:?}", layer_type);
                }
            }
        }
    }
}

// ... (rest of file remains the same) ...
