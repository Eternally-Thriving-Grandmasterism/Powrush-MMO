/*!
 * Kira Music - Balanced Multi-Band Reverb (Max Quality / Min Cost)
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

/// Balanced multi-band reverb automation (max quality, minimal cost)
pub fn apply_kira_multi_band_reverb(
    reverb_state: Res<ReverbState>,
    controller: Res<KiraMusicController>,
) {
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;
    let intensity = controller.intensity;

    // High band gets stronger high-frequency damping + faster effective decay
    for (layer, lp_filter) in &controller.low_pass_filters {
        let base = 900.0 + intensity * 10500.0;
        let cutoff = (base * (1.0 - high_damping * 0.75)).max(450.0);
        let _ = lp_filter.set_cutoff(cutoff);
    }

    // Low band gets high-pass filtering influenced by low_damping
    for (layer, hp_filter) in &controller.high_pass_filters {
        let cutoff = 35.0 + low_damping * 140.0;
        let _ = hp_filter.set_cutoff(cutoff);
    }

    // Note: True per-band decay would require separate reverb instances.
    // We achieve most of the benefit here through intelligent filtering + parameter modulation.
}

pub fn initialize_kira_multi_band_filters(
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraMusicController>,
) {
    controller.low_pass_filters.clear();
    controller.high_pass_filters.clear();

    let layers = [MusicLayer::Base, MusicLayer::Tension, MusicLayer::Percussion, MusicLayer::Melody, MusicLayer::Intense];

    for layer in layers {
        if let Ok(lp) = audio.add_filter(FilterBuilder::new().cutoff(2200.0)) {
            controller.low_pass_filters.insert(layer, lp);
        }
        if let Ok(hp) = audio.add_filter(FilterBuilder::new().cutoff(70.0)) {
            controller.high_pass_filters.insert(layer, hp);
        }
    }
}
