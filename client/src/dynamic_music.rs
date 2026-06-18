/*!
 * Dynamic Music System for Powrush-MMO
 *
 * Phase A: Smoother musical crossfading + basic modulation.
 *
 * v18.96 — Improved transitions and subtle movement in layers.
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

pub struct ActiveLayer {
    pub handle: Option<oddio::Handle<Gain<f32, Stop<Box<dyn oddio::Source<Frame = [f32; 2]> + Send>>>>>,
    pub target_volume: f32,
    pub current_volume: f32,
    pub is_playing: bool,
    pub fade_speed: f32,
    pub modulation_depth: f32,
    pub modulation_rate: f32,
}

impl Default for ActiveLayer {
    fn default() -> Self {
        Self {
            handle: None,
            target_volume: 0.0,
            current_volume: 0.0,
            is_playing: false,
            fade_speed: 2.2,
            modulation_depth: 0.08,
            modulation_rate: 0.6,
        }
    }
}

#[derive(Resource, Debug)]
pub struct DynamicMusicController {
    pub layers: HashMap<MusicLayerType, ActiveLayer>,
    pub state: MusicState,
    time: f32,
}

impl Default for DynamicMusicController {
    fn default() -> Self {
        let mut layers = HashMap::new();

        layers.insert(MusicLayerType::BaseHarmony, ActiveLayer {
            fade_speed: 1.6,
            modulation_depth: 0.06,
            modulation_rate: 0.4,
            ..default()
        });
        layers.insert(MusicLayerType::AttunementPads, ActiveLayer {
            fade_speed: 2.0,
            modulation_depth: 0.1,
            modulation_rate: 0.7,
            ..default()
        });
        layers.insert(MusicLayerType::RhythmicPulse, ActiveLayer {
            fade_speed: 2.8,
            modulation_depth: 0.12,
            modulation_rate: 1.2,
            ..default()
        });
        layers.insert(MusicLayerType::BloomResonance, ActiveLayer {
            fade_speed: 2.4,
            modulation_depth: 0.15,
            modulation_rate: 0.9,
            ..default()
        });

        Self {
            layers,
            state: MusicState::default(),
            time: 0.0,
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
                    self.set_layer_target(MusicLayerType::AttunementPads, att * 0.88);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, 0.0);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.0);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Voting => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.78);
                    self.set_layer_target(MusicLayerType::AttunementPads, att);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, inten * 0.75);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.0);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Resolution => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.88);
                    self.set_layer_target(MusicLayerType::AttunementPads, 1.0);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, 0.65);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.92);
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

    /// Smooth musical crossfading + subtle modulation
    pub fn sync_volumes_to_audio(&mut self, dt: f32) {
        self.time += dt;

        for layer in self.layers.values_mut() {
            if let Some(ref mut handle) = layer.handle {
                // Smooth eased transition toward target
                let diff = layer.target_volume - layer.current_volume;
                if diff.abs() > 0.0005 {
                    let t = (layer.fade_speed * dt).min(1.0);
                    layer.current_volume = layer.current_volume * (1.0 - t) + layer.target_volume * t;
                }

                // Subtle modulation (makes the layer feel alive)
                let modulation = if layer.modulation_depth > 0.0 {
                    (self.time * layer.modulation_rate).sin() as f32 * layer.modulation_depth
                } else {
                    0.0
                };

                let final_gain = (layer.current_volume + modulation).max(0.0);
                handle.set_gain(final_gain);

                if layer.target_volume < 0.015 && layer.current_volume < 0.02 {
                    layer.is_playing = false;
                }
            }
        }
    }
}

/// System: Advanced volume syncing with modulation
pub fn sync_music_volumes(
    time: Res<Time>,
    mut controller: ResMut<DynamicMusicController>,
) {
    controller.sync_volumes_to_audio(time.delta_secs());
}

/// System: Activate layers
pub fn activate_music_layers(
    mut controller: ResMut<DynamicMusicController>,
    backend: Res<OddioAudioBackend>,
) {
    for (layer_type, layer) in controller.layers.iter_mut() {
        let should_play = layer.target_volume > 0.035;

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

            info!("🎵 Activated: {:?} (modulation: {:.2})", layer_type, layer.modulation_depth);
        }
    }
}
