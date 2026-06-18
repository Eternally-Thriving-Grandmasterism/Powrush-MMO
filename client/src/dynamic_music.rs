/*!
 * Dynamic Music System for Powrush-MMO
 *
 * Phase 1 of AssetServer integration: Basic asset-aware structure.
 *
 * v19.01 — Introduced MusicLayerHandle and asset-oriented layer management.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use crate::oddio_backend::OddioAudioBackend;
use std::collections::HashMap;

/// Represents a reference to a music layer asset.
/// In later steps this will become a proper Bevy Handle.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MusicLayerHandle {
    pub id: String,
}

impl MusicLayerHandle {
    pub fn new(name: &str) -> Self {
        Self { id: name.to_string() }
    }

    pub fn filename(&self) -> String {
        format!("assets/audio/music_layers/{}.wav", self.id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MusicLayerType {
    BaseHarmony,
    AttunementPads,
    RhythmicPulse,
    BloomResonance,
}

impl MusicLayerType {
    pub fn default_handle(&self) -> MusicLayerHandle {
        let name = match self {
            MusicLayerType::BaseHarmony => "base_harmony",
            MusicLayerType::AttunementPads => "attunement_pads",
            MusicLayerType::RhythmicPulse => "rhythmic_pulse",
            MusicLayerType::BloomResonance => "bloom_resonance",
        };
        MusicLayerHandle::new(name)
    }
}

#[derive(Clone, Debug, Default)]
pub struct MusicState {
    pub council_phase: Option<simulation::council_mercy_trial::CouncilMercyTrialPhase>,
    pub previous_phase: Option<simulation::council_mercy_trial::CouncilMercyTrialPhase>,
    pub attunement: f32,
    pub intensity: f32,
    pub is_resolving: bool,
}

pub struct ActiveLayer {
    pub handle: Option<oddio::Handle<oddio::Gain<f32, oddio::Stop<Box<dyn oddio::Source<Frame = [f32; 2]> + Send>>>>>,
    pub asset: MusicLayerHandle,
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
            asset: MusicLayerHandle::new("base_harmony"),
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
    resolution_swell_timer: f32,
}

impl Default for DynamicMusicController {
    fn default() -> Self {
        let mut layers = HashMap::new();

        layers.insert(
            MusicLayerType::BaseHarmony,
            ActiveLayer {
                asset: MusicLayerType::BaseHarmony.default_handle(),
                fade_speed: 1.6,
                modulation_depth: 0.06,
                modulation_rate: 0.4,
                ..default()
            },
        );
        layers.insert(
            MusicLayerType::AttunementPads,
            ActiveLayer {
                asset: MusicLayerType::AttunementPads.default_handle(),
                fade_speed: 2.0,
                modulation_depth: 0.1,
                modulation_rate: 0.7,
                ..default()
            },
        );
        layers.insert(
            MusicLayerType::RhythmicPulse,
            ActiveLayer {
                asset: MusicLayerType::RhythmicPulse.default_handle(),
                fade_speed: 2.8,
                modulation_depth: 0.12,
                modulation_rate: 1.2,
                ..default()
            },
        );
        layers.insert(
            MusicLayerType::BloomResonance,
            ActiveLayer {
                asset: MusicLayerType::BloomResonance.default_handle(),
                fade_speed: 2.4,
                modulation_depth: 0.18,
                modulation_rate: 0.85,
                ..default()
            },
        );

        Self {
            layers,
            state: MusicState::default(),
            time: 0.0,
            resolution_swell_timer: 0.0,
        }
    }
}

impl DynamicMusicController {
    pub fn apply_state_to_layers(&mut self) {
        let is_major_transition = self.state.previous_phase != self.state.council_phase;
        let entering_resolution = is_major_transition
            && self.state.council_phase == Some(simulation::council_mercy_trial::CouncilMercyTrialPhase::Resolution);

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
                    let swell_strength = 0.15 + (att * 0.25);

                    self.set_layer_target(MusicLayerType::BaseHarmony, 0.92);
                    self.set_layer_target(MusicLayerType::AttunementPads, 1.0 + swell_strength * 0.6);
                    self.set_layer_target(MusicLayerType::RhythmicPulse, 0.75);
                    self.set_layer_target(MusicLayerType::BloomResonance, 1.12 + swell_strength);

                    if let Some(bloom) = self.layers.get_mut(&MusicLayerType::BloomResonance) {
                        bloom.modulation_depth = 0.32 + (att * 0.15);
                        bloom.modulation_rate = 1.15;
                    }
                    if let Some(pads) = self.layers.get_mut(&MusicLayerType::AttunementPads) {
                        pads.modulation_depth = 0.18 + (att * 0.12);
                        pads.modulation_rate = 0.95;
                    }
                }
                _ => {}
            }

            if entering_resolution {
                self.resolution_swell_timer = 4.5;

                if let Some(bloom) = self.layers.get_mut(&MusicLayerType::BloomResonance) {
                    bloom.target_volume = 1.22 + (att * 0.2);
                }
                if let Some(pads) = self.layers.get_mut(&MusicLayerType::AttunementPads) {
                    pads.target_volume = 1.15 + (att * 0.15);
                }

                info!("🌟 Resolution swell triggered | attunement={:.2}", att);
            }
        } else {
            for layer in self.layers.values_mut() {
                layer.target_volume = 0.0;
            }
        }

        if self.resolution_swell_timer > 0.0 {
            self.resolution_swell_timer -= 0.016;
            if self.resolution_swell_timer <= 0.0 {
                if let Some(bloom) = self.layers.get_mut(&MusicLayerType::BloomResonance) {
                    bloom.target_volume = 1.12;
                }
                if let Some(pads) = self.layers.get_mut(&MusicLayerType::AttunementPads) {
                    pads.target_volume = 1.0;
                }
            }
        }

        self.state.previous_phase = self.state.council_phase;
    }

    fn set_layer_target(&mut self, layer_type: MusicLayerType, volume: f32) {
        if let Some(layer) = self.layers.get_mut(&layer_type) {
            layer.target_volume = volume.clamp(0.0, 1.4);
        }
    }

    pub fn sync_volumes_to_audio(&mut self, dt: f32) {
        self.time += dt;

        for layer in self.layers.values_mut() {
            if let Some(ref mut handle) = layer.handle {
                let diff = layer.target_volume - layer.current_volume;

                if diff.abs() > 0.0005 {
                    let t = (layer.fade_speed * dt).min(1.0);
                    layer.current_volume = layer.current_volume * (1.0 - t) + layer.target_volume * t;
                }

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

pub fn activate_music_layers(
    mut controller: ResMut<DynamicMusicController>,
    backend: Res<OddioAudioBackend>,
) {
    for (layer_type, layer) in controller.layers.iter_mut() {
        let should_play = layer.target_volume > 0.035;

        if should_play && !layer.is_playing {
            let filename = layer.asset.filename();

            match backend.play_audio_file(&filename, layer.target_volume, true) {
                Ok(handle) => {
                    layer.handle = Some(handle);
                    layer.is_playing = true;
                    layer.current_volume = layer.target_volume;
                    info!("🎵 Loaded layer asset: {} (type: {:?})", filename, layer_type);
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

pub fn sync_music_volumes(
    time: Res<Time>,
    mut controller: ResMut<DynamicMusicController>,
) {
    controller.sync_volumes_to_audio(time.delta_secs());
}
