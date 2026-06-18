/*!
 * Dynamic Music System for Powrush-MMO
 *
 * Phase 3: Full wiring of DynamicMusicController to OddioAudioBackend.
 *
 * v18.95 — Layers now activate and play procedural audio based on Council state.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use crate::oddio_backend::OddioAudioBackend;
use oddio::{Gain, Stop};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MusicLayerType {
    BaseHarmony,
    AttunementPads,
    RhythmicPulse,
    BloomResonance,
}

#[derive(Clone, Debug, Default)]
pub struct MusicState {
    pub council_phase: Option<simulation::council_mercy_trial::CouncilMercyTrialPhase>,
    pub attunement: f32,
    pub intensity: f32,
    pub is_resolving: bool,
}

/// Holds an active oddio layer handle + volume state
pub struct ActiveLayer {
    pub handle: Option<oddio::Handle<Gain<f32, Stop<Box<dyn oddio::Source<Frame = [f32; 2]> + Send>>>>>,
    pub target_volume: f32,
    pub current_volume: f32,
    pub is_playing: bool,
}

impl Default for ActiveLayer {
    fn default() -> Self {
        Self {
            handle: None,
            target_volume: 0.0,
            current_volume: 0.0,
            is_playing: false,
        }
    }
}

#[derive(Resource, Debug)]
pub struct DynamicMusicController {
    pub layers: HashMap<MusicLayerType, ActiveLayer>,
    pub state: MusicState,
}

impl Default for DynamicMusicController {
    fn default() -> Self {
        let mut layers = HashMap::new();
        layers.insert(MusicLayerType::BaseHarmony, ActiveLayer::default());
        layers.insert(MusicLayerType::AttunementPads, ActiveLayer::default());
        layers.insert(MusicLayerType::RhythmicPulse, ActiveLayer::default());
        layers.insert(MusicLayerType::BloomResonance, ActiveLayer::default());

        Self {
            layers,
            state: MusicState::default(),
        }
    }
}

impl DynamicMusicController {
    pub fn apply_state_to_layers(&mut self) {
        if let Some(phase) = self.state.council_phase {
            let att = self.state.attunement.clamp(0.0, 1.0);
            let inten = self.state.intensity.clamp(0.0, 1.0);

            match phase {
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Attunement => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.65);
                    self.set_layer_target(MusicLayerType::AttunementPads, att * 0.85);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, 0.0);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.0);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Voting => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.75);
                    self.set_layer_target(MusicLayerType::AttunementPads, att);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, inten * 0.7);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.0);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Resolution => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.85);
                    self.set_layer_target(MusicLayerType::AttunementPads, 1.0);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, 0.6);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.9);
                }
                _ => {}
            }
        } else {
            // No active council — fade everything down
            for layer in self.layers.values_mut() {
                layer.target_volume = 0.0;
            }
        }
    }

    fn set_layer_target(&mut self, layer_type: MusicLayerType, volume: f32) {
        if let Some(layer) = self.layers.get_mut(&layer_type) {
            layer.target_volume = volume.clamp(0.0, 1.2);
        }
    }

    /// Update volumes on active oddio handles (called every frame)
    pub fn sync_volumes_to_audio(&mut self) {
        for layer in self.layers.values_mut() {
            if let Some(ref mut handle) = layer.handle {
                if (layer.current_volume - layer.target_volume).abs() > 0.005 {
                    layer.current_volume = layer.target_volume;
                    handle.set_gain(layer.current_volume);
                }
            }
        }
    }
}

/// System: Sync layer volumes to oddio every frame
pub fn sync_music_volumes(
    mut controller: ResMut<DynamicMusicController>,
) {
    controller.sync_volumes_to_audio();
}

/// System: Activate layers based on current MusicState (called when state changes)
pub fn activate_music_layers(
    mut controller: ResMut<DynamicMusicController>,
    backend: Res<OddioAudioBackend>,
) {
    for (layer_type, layer) in controller.layers.iter_mut() {
        let should_play = layer.target_volume > 0.05;

        if should_play && !layer.is_playing {
            // Activate the layer with a procedural source
            let frequency = match layer_type {
                MusicLayerType::BaseHarmony => 55.0,      // Low fundamental
                MusicLayerType::AttunementPads => 110.0,  // Octave
                MusicLayerType::RhythmicPulse => 220.0,
                MusicLayerType::BloomResonance => 330.0,
            };

            let handle = backend.play_procedural_layer(frequency, layer.target_volume);
            layer.handle = Some(handle);
            layer.is_playing = true;
            layer.current_volume = layer.target_volume;

            info!("🎵 Activated music layer: {:?} @ {:.2}", layer_type, layer.target_volume);
        }

        if !should_play && layer.is_playing {
            layer.is_playing = false;
            // Note: For full implementation we would stop the source here
            // For now we rely on volume fading to 0
        }
    }
}
