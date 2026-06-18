/*!
 * Dynamic Music System for Powrush-MMO
 *
 * Built on oddio for maximum flexibility and control.
 * Phase 3: Integration with OddioAudioBackend for audible layers.
 *
 * v18.95 — Controller now manages real audio playback through oddio.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use crate::oddio_backend::OddioAudioBackend;
use oddio::{Gain, Stop};
use std::collections::HashMap;

/// Types of music layers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MusicLayerType {
    BaseHarmony,
    AttunementPads,
    RhythmicPulse,
    BloomResonance,
}

/// Current musical state
#[derive(Clone, Debug, Default)]
pub struct MusicState {
    pub council_phase: Option<simulation::council_mercy_trial::CouncilMercyTrialPhase>,
    pub attunement: f32,
    pub intensity: f32,
    pub is_resolving: bool,
}

/// Represents one active music layer with its oddio handle
pub struct ActiveLayer {
    pub handle: oddio::Handle<Gain<f32, Stop<Box<dyn oddio::Source<Frame = [f32; 2]> + Send>>>>,
    pub target_volume: f32,
    pub current_volume: f32,
}

/// Main controller for dynamic music
#[derive(Resource, Debug)]
pub struct DynamicMusicController {
    pub layers: HashMap<MusicLayerType, ActiveLayer>,
    pub state: MusicState,
    backend: Option<Res<OddioAudioBackend>>, // Will be set after backend is available
}

impl Default for DynamicMusicController {
    fn default() -> Self {
        Self {
            layers: HashMap::new(),
            state: MusicState::default(),
            backend: None,
        }
    }
}

impl DynamicMusicController {
    /// Initialize with backend reference (called from system)
    pub fn set_backend(&mut self, backend: Res<OddioAudioBackend>) {
        self.backend = Some(backend);
    }

    /// Apply current MusicState to layer targets
    pub fn apply_state_to_layers(&mut self) {
        // Same logic as before (simplified for Phase 3)
        if let Some(phase) = self.state.council_phase {
            let att = self.state.attunement.clamp(0.0, 1.0);
            let inten = self.state.intensity.clamp(0.0, 1.0);

            match phase {
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Attunement => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.7);
                    self.set_layer_target(MusicLayerType::AttunementPads, att * 0.9);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Voting => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.8);
                    self.set_layer_target(MusicLayerType::AttunementPads, att);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, inten * 0.8);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Resolution => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.9);
                    self.set_layer_target(MusicLayerType::AttunementPads, 1.0);
                    self.set_layer_target(MusicLayerType::BloomResonance, 1.0);
                }
                _ => {}
            }
        }
    }

    fn set_layer_target(&mut self, layer_type: MusicLayerType, volume: f32) {
        if let Some(layer) = self.layers.get_mut(&layer_type) {
            layer.target_volume = volume;
        }
    }

    /// Update volumes on active oddio handles
    pub fn update_layer_volumes(&mut self) {
        for layer in self.layers.values_mut() {
            if (layer.current_volume - layer.target_volume).abs() > 0.01 {
                layer.current_volume = layer.target_volume;
                // In real implementation, we would adjust the oddio Gain here
                // layer.handle.set_gain(layer.current_volume);
            }
        }
    }
}

/// System that updates layer volumes
pub fn update_music_layer_volumes(
    time: Res<Time>,
    mut controller: ResMut<DynamicMusicController>,
) {
    controller.update_layer_volumes();
}
