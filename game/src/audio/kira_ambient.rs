/*!
 * Kira Ambient - Hybrid Early + Late Reverb (Unified Pass)
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
    // Filter + Delay (Late Tail foundation)
    pub low_pass_filter: Option<FilterHandle>,
    pub high_pass_filter: Option<FilterHandle>,
    pub delay: Option<DelayHandle>,

    // Early Reflections (Convolution)
    pub early_convolution: Option<ConvolutionHandle>,
    pub fading_out_convolution: Option<ConvolutionHandle>,
    pub crossfade_timer: f32,
    pub crossfade_duration: f32,
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
            early_convolution: None,
            fading_out_convolution: None,
            crossfade_timer: 0.0,
            crossfade_duration: 0.28,
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
        controller.early_convolution = Some(conv);
        controller.last_ir_name = "none".to_string();
    }
}

pub fn apply_kira_ambient_multi_band_filtering(
    reverb_state: Res<ReverbState>,
    estimate: Res<ProceduralReverbEstimate>,
    current_ir: Res<CurrentImpulseResponse>,
    quality: Res<AudioQualitySettings>,
    listener: Option<Res<AudioListener>>,
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraAmbientController>,
) {
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;
    let early_delay = estimate.early_reflection_delay_ms;
    let ir = &current_ir.active;

    let early_mod = (early_delay / 80.0).clamp(0.0, 1.0);

    // === Late Tail (Filters + Delay) ===
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
        let mix = (0.25 + early_mod * 0.35).clamp(0.2, 0.65) * quality.get_late_mix();
        let _ = delay.set_mix(mix);
    }

    // === Early Reflections (Convolution) ===
    let listener_pos = listener.as_ref().map(|l| l.position).unwrap_or(Vec3::ZERO);
    let distance = listener_pos.length();
    let quality_multiplier = quality.get_convolution_mix_multiplier(distance);
    let early_mix = quality.get_early_mix() * quality_multiplier;

    let ir_changed = controller.last_ir_name != ir.name;
    let has_loaded = ir.loaded_source.is_some();
    let is_crossfading = controller.fading_out_convolution.is_some() || controller.crossfade_timer > 0.0;

    if ir_changed && has_loaded && !is_crossfading {
        controller.crossfade_duration = quality.crossfade_duration;
        start_crossfade(&mut controller, ir, estimate.wetness, early_mix, audio);
    }

    if is_crossfading {
        update_crossfade(&mut controller, ir, estimate.wetness, early_mix);
    } else if let Some(conv) = &controller.early_convolution {
        let target = (ir.wetness_bias * estimate.wetness * 0.75 * early_mix).clamp(0.0, 0.7);
        let _ = conv.set_mix(target);
    }
}

fn start_crossfade(
    controller: &mut KiraAmbientController,
    new_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    early_mix: f32,
    audio: Res<AudioManager>,
) {
    if let Some(current) = controller.early_convolution.take() {
        controller.fading_out_convolution = Some(current);
    }
    controller.crossfade_timer = 0.0;

    if let Some(loaded) = &new_ir.loaded_source {
        if let Ok(new_conv) = audio.add_effect(
            ConvolutionBuilder::new()
                .impulse_response(loaded.clone())
                .mix(0.0)
        ) {
            controller.early_convolution = Some(new_conv);
        }
    }
    controller.last_ir_name = new_ir.name.clone();
}

fn update_crossfade(
    controller: &mut KiraAmbientController,
    current_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    early_mix: f32,
) {
    controller.crossfade_timer += 1.0 / 60.0;

    let t = (controller.crossfade_timer / controller.crossfade_duration).clamp(0.0, 1.0);
    let fade_out = (1.0 - t).sqrt();
    let fade_in = t.sqrt();

    if let Some(old_conv) = &controller.fading_out_convolution {
        let target = (current_ir.wetness_bias * current_wetness * 0.7 * early_mix * fade_out).clamp(0.0, 0.7);
        let _ = old_conv.set_mix(target);
    }

    if let Some(new_conv) = &controller.early_convolution {
        let target = (current_ir.wetness_bias * current_wetness * 0.75 * early_mix * fade_in).clamp(0.0, 0.7);
        let _ = new_conv.set_mix(target);
    }

    if t >= 1.0 {
        if let Some(old) = controller.fading_out_convolution.take() {
            let _ = old;
        }
        controller.crossfade_timer = 0.0;
    }
}
