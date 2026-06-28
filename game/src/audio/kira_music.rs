/*!
 * Kira Music - Multi-Band + Convolution with Dynamic IR Swapping
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use kira::effect::convolution::{ConvolutionBuilder, ConvolutionHandle};
use std::collections::HashMap;
use crate::settings::audio_mixing::ReverbState;
use crate::audio::procedural_reverb_estimation::ProceduralReverbEstimate;
use crate::audio::ir_manager::CurrentImpulseResponse;

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
    pub convolution: Option<ConvolutionHandle>,
    pub last_ir_name: String,
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
            convolution: None,
            last_ir_name: "none".to_string(),
        }
    }
}

/// Apply multi-band filtering + dynamic convolution for music
pub fn apply_kira_multi_band_reverb(
    reverb_state: Res<ReverbState>,
    estimate: Res<ProceduralReverbEstimate>,
    current_ir: Res<CurrentImpulseResponse>,
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraMusicController>,
) {
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;
    let intensity = controller.intensity;
    let early_delay = estimate.early_reflection_delay_ms;
    let ir = &current_ir.active;

    let early_mod = (early_delay / 75.0).clamp(0.0, 1.0);

    // Existing multi-band filter modulation
    for (layer, lp_filter) in &controller.low_pass_filters {
        let base = 900.0 + intensity * 10500.0;
        let cutoff = (base * (1.0 - high_damping * 0.75 * (1.0 + early_mod * 0.2))).max(450.0);
        let _ = lp_filter.set_cutoff(cutoff);
    }

    for (layer, hp_filter) in &controller.high_pass_filters {
        let cutoff = 35.0 + low_damping * 140.0 * (1.0 + early_mod * 0.15);
        let _ = hp_filter.set_cutoff(cutoff);
    }

    // === Dynamic Convolution for Music ===
    let ir_changed = controller.last_ir_name != ir.name;
    let has_loaded = ir.loaded_source.is_some();

    if ir_changed && has_loaded {
        if let Some(old_conv) = controller.convolution.take() {
            let _ = old_conv;
        }

        if let Some(loaded) = &ir.loaded_source {
            if let Ok(new_conv) = audio.add_effect(
                ConvolutionBuilder::new()
                    .impulse_response(loaded.clone())
                    .mix((ir.wetness_bias * estimate.wetness * 0.65).clamp(0.0, 0.6))
            ) {
                controller.convolution = Some(new_conv);
                controller.last_ir_name = ir.name.clone();
            }
        }
    } else if let Some(conv) = &controller.convolution {
        let target_wet = (ir.wetness_bias * estimate.wetness * 0.6).clamp(0.0, 0.55);
        let _ = conv.set_mix(target_wet);
    }
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

    // Initialize convolution for music (will be swapped when real IR loads)
    if let Ok(conv) = audio.add_effect(ConvolutionBuilder::new().mix(0.0)) {
        controller.convolution = Some(conv);
    }
}
