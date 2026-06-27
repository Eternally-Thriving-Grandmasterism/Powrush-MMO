/*!
 * Kira Music - Full Multi-Band Filtering Support
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use std::collections::HashMap;
use crate::settings::audio_mixing::ReverbState;

#[derive(Resource)]
pub struct KiraMusicController {
    pub current_state: MusicStateType,
    pub target_state: MusicStateType,
    pub intensity: f32,
    pub transition_timer: f32,
    pub transition_duration: f32,
    pub ducking: f32,
    pub duck_timer: f32,
    /// Multiple filters per layer for multi-band control
    pub low_pass_filters: HashMap<MusicLayer, FilterHandle>,
    pub high_pass_filters: HashMap<MusicLayer, FilterHandle>,
    pub active_sounds: HashMap<MusicLayer, AudioHandle<AudioSource>>,
}

impl Default for KiraMusicController {
    fn default() -> Self {
        Self {
            current_state: MusicStateType::Exploration,
            target_state: MusicStateType::Exploration,
            intensity: 0.5,
            transition_timer: 0.0,
            transition_duration: 4.0,
            ducking: 0.0,
            duck_timer: 0.0,
            low_pass_filters: HashMap::new(),
            high_pass_filters: HashMap::new(),
            active_sounds: HashMap::new(),
        }
    }
}

/// Full multi-band filter automation driven by ReverbState
pub fn apply_kira_multi_band_filtering(
    reverb_state: Res<ReverbState>,
    controller: Res<KiraMusicController>,
) {
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;
    let intensity = controller.intensity;

    for (layer, lp_filter) in &controller.low_pass_filters {
        // Low-pass cutoff influenced by high_damping + intensity
        let base = 800.0 + intensity * 11000.0;
        let cutoff = (base * (1.0 - high_damping * 0.7)).max(400.0);
        let _ = lp_filter.set_cutoff(cutoff);
    }

    for (layer, hp_filter) in &controller.high_pass_filters {
        // High-pass cutoff influenced by low_damping
        let cutoff = 40.0 + low_damping * 180.0;
        let _ = hp_filter.set_cutoff(cutoff);
    }
}

/// Initialize multi-band filters (low-pass + high-pass per layer)
pub fn initialize_kira_multi_band_filters(
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraMusicController>,
) {
    controller.low_pass_filters.clear();
    controller.high_pass_filters.clear();

    let layers = [
        MusicLayer::Base,
        MusicLayer::Tension,
        MusicLayer::Percussion,
        MusicLayer::Melody,
        MusicLayer::Intense,
    ];

    for layer in layers {
        // Low-pass filter
        if let Ok(lp) = audio.add_filter(FilterBuilder::new().cutoff(2000.0)) {
            controller.low_pass_filters.insert(layer, lp);
        }

        // High-pass filter
        if let Ok(hp) = audio.add_filter(FilterBuilder::new().cutoff(80.0)) {
            controller.high_pass_filters.insert(layer, hp);
        }
    }

    info!("Initialized multi-band filters for {} music layers", layers.len());
}
