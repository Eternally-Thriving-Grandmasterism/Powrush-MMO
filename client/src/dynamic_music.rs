/*!
 * Dynamic Music System for Powrush-MMO
 *
 * Built on oddio for maximum flexibility and control.
 * Designed to respond to Council phases, attunement, and metaverse state.
 *
 * v18.95 — Phase 1: Controller + Layer Management + Fading
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use std::collections::HashMap;

/// Types of music layers in the dynamic music system
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MusicLayerType {
    BaseHarmony,
    AttunementPads,
    RhythmicPulse,
    BloomResonance,
}

/// Represents the current musical state driven by gameplay
#[derive(Clone, Debug, Default)]
pub struct MusicState {
    pub council_phase: Option<simulation::council_mercy_trial::CouncilMercyTrialPhase>,
    pub attunement: f32,
    pub intensity: f32,
    pub is_resolving: bool,
}

/// A single controllable music layer
#[derive(Debug)]
pub struct MusicLayer {
    pub active: bool,
    pub target_volume: f32,
    pub current_volume: f32,
    pub fade_speed: f32,
}

impl Default for MusicLayer {
    fn default() -> Self {
        Self {
            active: false,
            target_volume: 0.0,
            current_volume: 0.0,
            fade_speed: 1.5,
        }
    }
}

/// Main resource controlling all dynamic music layers
#[derive(Resource, Debug)]
pub struct DynamicMusicController {
    pub layers: HashMap<MusicLayerType, MusicLayer>,
    pub state: MusicState,
}

impl Default for DynamicMusicController {
    fn default() -> Self {
        let mut layers = HashMap::new();

        layers.insert(MusicLayerType::BaseHarmony, MusicLayer {
            active: true,
            target_volume: 0.6,
            current_volume: 0.6,
            fade_speed: 0.8,
            ..default()
        });

        layers.insert(MusicLayerType::AttunementPads, MusicLayer::default());
        layers.insert(MusicLayerType::RhythmicPulse, MusicLayer::default());
        layers.insert(MusicLayerType::BloomResonance, MusicLayer::default());

        Self {
            layers,
            state: MusicState::default(),
        }
    }
}

impl DynamicMusicController {
    pub fn apply_state_to_layers(&mut self) {
        let attunement = self.state.attunement.clamp(0.0, 1.0);
        let intensity = self.state.intensity.clamp(0.0, 1.0);

        if let Some(phase) = self.state.council_phase {
            match phase {
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Lobby => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.5);
                    self.set_layer_target(MusicLayerType::AttunementPads, 0.0);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, 0.0);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.0);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Attunement => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.7);
                    self.set_layer_target(MusicLayerType::AttunementPads, attunement * 0.9);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, 0.0);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.0);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Deliberation => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.8);
                    self.set_layer_target(MusicLayerType::AttunementPads, attunement * 0.85);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, intensity * 0.6);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.0);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Voting => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.75);
                    self.set_layer_target(MusicLayerType::AttunementPads, attunement);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, intensity * 0.85);
                    self.set_layer_target(MusicLayerType::BloomResonance, 0.0);
                }
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Resolution |
                simulation::council_mercy_trial::CouncilMercyTrialPhase::Completed => {
                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.9);
                    self.set_layer_target(MusicLayerType::AttunementPads, 1.0);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, 0.7);
                    self.set_layer_target(MusicLayerType::BloomResonance, if self.state.is_resolving { 1.0 } else { 0.3 });
                }
            }
        }
    }

    fn set_layer_target(&mut self, layer: MusicLayerType, volume: f32) {
        if let Some(l) = self.layers.get_mut(&layer) {
            l.target_volume = volume.clamp(0.0, 1.2);
            l.active = volume > 0.01;
        }
    }
}

/// Smoothly updates current volumes toward target volumes
pub fn update_music_layer_volumes(
    time: Res<Time>,
    mut controller: ResMut<DynamicMusicController>,
) {
    let dt = time.delta_secs();

    for layer in controller.layers.values_mut() {
        if (layer.current_volume - layer.target_volume).abs() > 0.001 {
            let diff = layer.target_volume - layer.current_volume;
            let step = layer.fade_speed * dt;

            if diff.abs() <= step {
                layer.current_volume = layer.target_volume;
            } else {
                layer.current_volume += diff.signum() * step;
            }
        } else {
            layer.current_volume = layer.target_volume;
        }
    }
}
