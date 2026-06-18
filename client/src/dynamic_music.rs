/*!
 * Dynamic Music System for Powrush-MMO
 *
 * Improved layer lifecycle and basic crossfading logic.
 *
 * v18.96 — Smoother volume transitions + better deactivation handling.
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

/// Represents one active music layer with oddio handle and smooth volume control
pub struct ActiveLayer {
    pub handle: Option<oddio::Handle<Gain<f32, Stop<Box<dyn oddio::Source<Frame = [f32; 2]> + Send>>>>>,
    pub target_volume: f32,
    pub current_volume: f32,
    pub is_playing: bool,
    pub fade_speed: f32,
}

impl Default for ActiveLayer {
    fn default() -> Self {
        Self {
            handle: None,
            target_volume: 0.0,
            current_volume: 0.0,
            is_playing: false,
            fade_speed: 2.5,
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
        layers.insert(MusicLayerType::BaseHarmony, ActiveLayer { fade_speed: 1.8, ..default() });
        layers.insert(MusicLayerType::AttunementPads, ActiveLayer { fade_speed: 2.2, ..default() });
        layers.insert(MusicLayerType::RhythmicPulse, ActiveLayer { fade_speed: 3.0, ..default() });
        layers.insert(MusicLayerType::BloomResonance, ActiveLayer { fade_speed: 2.5, ..default() });

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

    /// Smoothly update volumes toward targets (basic crossfading behavior)
    pub fn sync_volumes_to_audio(&mut self) {
        for layer in self.layers.values_mut() {
            if let Some(ref mut handle) = layer.handle {
                let diff = layer.target_volume - layer.current_volume;

                if diff.abs() > 0.001 {
                    let step = layer.fade_speed * 0.016;
                    if diff.abs() <= step {
                        layer.current_volume = layer.target_volume;
                    } else {
                        layer.current_volume += diff.signum() * step;
                    }

                    handle.set_gain(layer.current_volume.max(0.0));
                }

                if layer.target_volume < 0.02 && layer.current_volume < 0.02 {
                    layer.is_playing = false;
                }
            }
        }
    }
}

/// System: Smooth volume syncing + crossfading
pub fn sync_music_volumes(
    mut controller: ResMut<DynamicMusicController>,
) {
    controller.sync_volumes_to_audio();
}

/// System: Activate layers when they should play
pub fn activate_music_layers(
    mut controller: ResMut<DynamicMusicController>,
    backend: Res<OddioAudioBackend>,
) {
    for (layer_type, layer) in controller.layers.iter_mut() {
        let should_play = layer.target_volume > 0.04;

        if should_play && !layer.is_playing {
            let frequency = match layer_type {
                MusicLayerType::BaseHarmony => 55.0,
                MusicLayerType::AttunementPads => 110.0,
                MusicLayerType::RhythmicPulse => 220.0,
                MusicLayerType::BloomResonance => 330.0,
            };

            let handle = backend.play_procedural_layer(frequency, layer.target_volume);
            layer.handle = Some(handle);
            layer.is_playing = true;
            layer.current_volume = layer.target_volume;

            info!("🎵 Music layer activated: {:?}", layer_type);
        }
    }
}
