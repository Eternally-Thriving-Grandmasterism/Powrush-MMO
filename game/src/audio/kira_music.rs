/*!
 * Kira Music - Multi-Band + Refactored Smooth Crossfading
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use kira::effect::convolution::{ConvolutionBuilder, ConvolutionHandle};
use std::collections::HashMap;
use crate::settings::audio_mixing::ReverbState;
use crate::audio::procedural_reverb_estimation::{ProceduralReverbEstimate, AudioListener};
use crate::audio::ir_manager::CurrentImpulseResponse;
use crate::settings::audio_quality::AudioQualitySettings;

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

    // Crossfade state (refactored pattern)
    pub current_convolution: Option<ConvolutionHandle>,
    pub fading_out_convolution: Option<ConvolutionHandle>,
    pub crossfade_timer: f32,
    pub crossfade_duration: f32,
    pub target_ir_name: String,
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
            current_convolution: None,
            fading_out_convolution: None,
            crossfade_timer: 0.0,
            crossfade_duration: 0.3,
            target_ir_name: "none".to_string(),
            last_ir_name: "none".to_string(),
        }
    }
}

pub fn apply_kira_multi_band_reverb(
    reverb_state: Res<ReverbState>,
    estimate: Res<ProceduralReverbEstimate>,
    current_ir: Res<CurrentImpulseResponse>,
    quality: Res<AudioQualitySettings>,
    listener: Option<Res<AudioListener>>,
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraMusicController>,
) {
    // === 1. Basic modulation ===
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;
    let intensity = controller.intensity;
    let early_delay = estimate.early_reflection_delay_ms;
    let ir = &current_ir.active;

    let early_mod = (early_delay / 75.0).clamp(0.0, 1.0);

    for (layer, lp_filter) in &controller.low_pass_filters {
        let base = 900.0 + intensity * 10500.0;
        let cutoff = (base * (1.0 - high_damping * 0.75 * (1.0 + early_mod * 0.2))).max(450.0);
        let _ = lp_filter.set_cutoff(cutoff);
    }

    for (layer, hp_filter) in &controller.high_pass_filters {
        let cutoff = 35.0 + low_damping * 140.0 * (1.0 + early_mod * 0.15);
        let _ = hp_filter.set_cutoff(cutoff);
    }

    // === 2. Quality + Distance ===
    let listener_pos = listener.as_ref().map(|l| l.position).unwrap_or(Vec3::ZERO);
    let distance = listener_pos.length();
    let quality_multiplier = quality.get_convolution_mix_multiplier(distance);

    // === 3. Clear Crossfade Logic ===
    let ir_name_changed = controller.last_ir_name != ir.name;
    let has_loaded = ir.loaded_source.is_some();
    let is_crossfading = controller.fading_out_convolution.is_some() || controller.crossfade_timer > 0.0;

    if ir_name_changed && has_loaded && !is_crossfading {
        start_music_crossfade(&mut controller, ir, estimate.wetness, quality_multiplier, audio);
    }

    if is_crossfading {
        update_music_crossfade(&mut controller, ir, estimate.wetness, quality_multiplier);
    } else if let Some(conv) = &controller.current_convolution {
        let target = (ir.wetness_bias * estimate.wetness * 0.6 * quality_multiplier).clamp(0.0, 0.55);
        let _ = conv.set_mix(target);
    }
}

fn start_music_crossfade(
    controller: &mut KiraMusicController,
    new_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    quality_multiplier: f32,
    audio: Res<AudioManager>,
) {
    if let Some(current) = controller.current_convolution.take() {
        controller.fading_out_convolution = Some(current);
    }

    controller.crossfade_timer = 0.0;
    controller.target_ir_name = new_ir.name.clone();

    if let Some(loaded) = &new_ir.loaded_source {
        if let Ok(new_conv) = audio.add_effect(
            ConvolutionBuilder::new()
                .impulse_response(loaded.clone())
                .mix(0.0)
        ) {
            controller.current_convolution = Some(new_conv);
        }
    }

    controller.last_ir_name = new_ir.name.clone();
}

fn update_music_crossfade(
    controller: &mut KiraMusicController,
    current_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    quality_multiplier: f32,
) {
    controller.crossfade_timer += 1.0 / 60.0;

    let t = (controller.crossfade_timer / controller.crossfade_duration).clamp(0.0, 1.0);
    let fade_out = 1.0 - t;
    let fade_in = t;

    if let Some(old_conv) = &controller.fading_out_convolution {
        let target = (current_ir.wetness_bias * current_wetness * 0.55 * quality_multiplier * fade_out).clamp(0.0, 0.55);
        let _ = old_conv.set_mix(target);
    }

    if let Some(new_conv) = &controller.current_convolution {
        let target = (current_ir.wetness_bias * current_wetness * 0.65 * quality_multiplier * fade_in).clamp(0.0, 0.6);
        let _ = new_conv.set_mix(target);
    }

    if t >= 1.0 {
        if let Some(old) = controller.fading_out_convolution.take() {
            let _ = old;
        }
        controller.crossfade_timer = 0.0;
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

    if let Ok(conv) = audio.add_effect(ConvolutionBuilder::new().mix(0.0)) {
        controller.current_convolution = Some(conv);
    }
}
