/*!
 * Kira Ambient - Hybrid with Clear Crossfade Logic
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use kira::effect::delay::DelayHandle;
use kira::effect::convolution::{ConvolutionBuilder, ConvolutionHandle};
use crate::settings::audio_mixing::ReverbState;
use crate::audio::procedural_reverb_estimation::{ProceduralReverbEstimate, AudioListener};
use crate::audio::ir_manager::CurrentImpulseResponse;
use crate::settings::audio_quality::AudioQualitySettings;

#[derive(Resource, Default)]
pub struct KiraAmbientController {
    pub low_pass_filter: Option<FilterHandle>,
    pub high_pass_filter: Option<FilterHandle>,
    pub delay: Option<DelayHandle>,

    // Crossfade state
    pub current_convolution: Option<ConvolutionHandle>,
    pub fading_out_convolution: Option<ConvolutionHandle>,
    pub crossfade_timer: f32,
    pub crossfade_duration: f32,
    pub target_ir_name: String,
    pub last_ir_name: String,

    pub ducking: f32,
    pub duck_timer: f32,
}

impl Default for KiraAmbientController {
    fn default() -> Self {
        Self {
            low_pass_filter: None,
            high_pass_filter: None,
            delay: None,
            current_convolution: None,
            fading_out_convolution: None,
            crossfade_timer: 0.0,
            crossfade_duration: 0.25,
            target_ir_name: "none".to_string(),
            last_ir_name: "none".to_string(),
            ducking: 0.0,
            duck_timer: 0.0,
        }
    }
}

pub fn initialize_kira_ambient_filters(
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraAmbientController>,
) {
    if let Ok(lp) = audio.add_filter(FilterBuilder::new().cutoff(2400.0)) {
        controller.low_pass_filter = Some(lp);
    }
    if let Ok(hp) = audio.add_filter(FilterBuilder::new().cutoff(55.0)) {
        controller.high_pass_filter = Some(hp);
    }
    if let Ok(delay) = audio.add_effect(kira::effect::delay::DelayBuilder::new().delay(0.025).feedback(0.15).mix(0.3)) {
        controller.delay = Some(delay);
    }
    if let Ok(conv) = audio.add_effect(ConvolutionBuilder::new().mix(0.0)) {
        controller.current_convolution = Some(conv);
        controller.last_ir_name = "none".to_string();
    }
}

/// Main audio processing with clear crossfade logic
pub fn apply_kira_ambient_multi_band_filtering(
    reverb_state: Res<ReverbState>,
    estimate: Res<ProceduralReverbEstimate>,
    current_ir: Res<CurrentImpulseResponse>,
    quality: Res<AudioQualitySettings>,
    listener: Option<Res<AudioListener>>,
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraAmbientController>,
) {
    // === 1. Basic modulation (filters + delay) ===
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;
    let early_delay = estimate.early_reflection_delay_ms;
    let ir = &current_ir.active;

    let early_mod = (early_delay / 80.0).clamp(0.0, 1.0);

    if let Some(lp) = &controller.low_pass_filter {
        let base = 2100.0 + (1.0 - high_damping) * 8500.0;
        let cutoff = (base * (1.0 - early_mod * 0.15)).max(550.0);
        let _ = lp.set_cutoff(cutoff);
    }

    if let Some(hp) = &controller.high_pass_filter {
        let base = 38.0 + low_damping * 110.0;
        let cutoff = (base * (1.0 + early_mod * 0.25)).max(30.0);
        let _ = hp.set_cutoff(cutoff);
    }

    if let Some(delay) = &controller.delay {
        let target_delay = (early_delay / 1000.0).clamp(0.008, 0.12);
        let _ = delay.set_delay(target_delay);
        let mix = (0.25 + early_mod * 0.35).clamp(0.2, 0.65);
        let _ = delay.set_mix(mix);
    }

    // === 2. Quality & Distance ===
    let listener_pos = listener.as_ref().map(|l| l.position).unwrap_or(Vec3::ZERO);
    let distance = listener_pos.length();
    let quality_multiplier = quality.get_convolution_mix_multiplier(distance);

    // === 3. Crossfade Logic (clear state machine) ===
    let ir_name_changed = controller.last_ir_name != ir.name;
    let has_loaded_source = ir.loaded_source.is_some();
    let is_crossfading = controller.fading_out_convolution.is_some() || controller.crossfade_timer > 0.0;

    if ir_name_changed && has_loaded_source && !is_crossfading {
        // Start new crossfade
        start_crossfade(&mut controller, ir, estimate.wetness, quality_multiplier, audio);
    }

    if is_crossfading {
        update_crossfade(&mut controller, ir, estimate.wetness, quality_multiplier);
    } else if let Some(conv) = &controller.current_convolution {
        // Normal steady-state modulation
        let target_mix = (ir.wetness_bias * estimate.wetness * 0.7 * quality_multiplier).clamp(0.0, 0.65);
        let _ = conv.set_mix(target_mix);
    }
}

/// Starts a smooth crossfade to a new impulse response
fn start_crossfade(
    controller: &mut KiraAmbientController,
    new_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    quality_multiplier: f32,
    audio: Res<AudioManager>,
) {
    // Move current to fading out
    if let Some(current) = controller.current_convolution.take() {
        controller.fading_out_convolution = Some(current);
    }

    controller.crossfade_timer = 0.0;
    controller.target_ir_name = new_ir.name.clone();

    // Create new convolution at mix 0
    if let Some(loaded) = &new_ir.loaded_source {
        let initial_mix = 0.0;
        if let Ok(new_conv) = audio.add_effect(
            ConvolutionBuilder::new()
                .impulse_response(loaded.clone())
                .mix(initial_mix)
        ) {
            controller.current_convolution = Some(new_conv);
        }
    }

    controller.last_ir_name = new_ir.name.clone();
}

/// Updates an ongoing crossfade (called every frame while fading)
fn update_crossfade(
    controller: &mut KiraAmbientController,
    current_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    quality_multiplier: f32,
) {
    controller.crossfade_timer += 1.0 / 60.0; // simple delta approximation

    let t = (controller.crossfade_timer / controller.crossfade_duration).clamp(0.0, 1.0);
    let fade_out_gain = 1.0 - t;
    let fade_in_gain = t;

    // Fade out old
    if let Some(old_conv) = &controller.fading_out_convolution {
        let target = (current_ir.wetness_bias * current_wetness * 0.7 * quality_multiplier * fade_out_gain).clamp(0.0, 0.7);
        let _ = old_conv.set_mix(target);
    }

    // Fade in new
    if let Some(new_conv) = &controller.current_convolution {
        let target = (current_ir.wetness_bias * current_wetness * 0.8 * quality_multiplier * fade_in_gain).clamp(0.0, 0.7);
        let _ = new_conv.set_mix(target);
    }

    // Finish crossfade
    if t >= 1.0 {
        if let Some(old) = controller.fading_out_convolution.take() {
            let _ = old; // TODO: proper effect removal when API available
        }
        controller.crossfade_timer = 0.0;
    }
}
