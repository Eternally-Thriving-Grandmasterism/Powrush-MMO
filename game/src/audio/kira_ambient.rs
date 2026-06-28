/*!
 * Kira Ambient - Hybrid with Smooth IR Crossfading
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

    // Convolution crossfade system
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
            crossfade_duration: 0.25, // 250ms crossfade
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
    // Initial convolution
    if let Ok(conv) = audio.add_effect(ConvolutionBuilder::new().mix(0.0)) {
        controller.current_convolution = Some(conv);
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

    // Filter + Delay modulation (unchanged)
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

    // === Smooth Crossfading Convolution ===
    let listener_pos = listener.as_ref().map(|l| l.position).unwrap_or(Vec3::ZERO);
    let distance = listener_pos.length();
    let quality_multiplier = quality.get_convolution_mix_multiplier(distance);

    let ir_name = &ir.name;
    let ir_changed = controller.last_ir_name != *ir_name;
    let has_loaded = ir.loaded_source.is_some();

    // Start crossfade if IR changed and we have a loaded source
    if ir_changed && has_loaded && controller.fading_out_convolution.is_none() {
        // Move current to fading out
        if let Some(current) = controller.current_convolution.take() {
            controller.fading_out_convolution = Some(current);
        }
        controller.crossfade_timer = 0.0;
        controller.target_ir_name = ir_name.clone();

        // Create new convolution immediately
        if let Some(loaded) = &ir.loaded_source {
            let base_mix = (ir.wetness_bias * estimate.wetness * 0.8 * quality_multiplier).clamp(0.0, 0.7);
            if let Ok(new_conv) = audio.add_effect(
                ConvolutionBuilder::new()
                    .impulse_response(loaded.clone())
                    .mix(0.0) // start at 0, will fade in
            ) {
                controller.current_convolution = Some(new_conv);
            }
        }
        controller.last_ir_name = ir_name.clone();
    }

    // Handle ongoing crossfade
    if controller.fading_out_convolution.is_some() || controller.crossfade_timer > 0.0 {
        controller.crossfade_timer += 1.0 / 60.0; // assume ~60fps for simplicity

        let t = (controller.crossfade_timer / controller.crossfade_duration).clamp(0.0, 1.0);
        let fade_out = 1.0 - t;
        let fade_in = t;

        if let Some(old_conv) = &controller.fading_out_convolution {
            let _ = old_conv.set_mix(fade_out * quality_multiplier);
        }

        if let Some(new_conv) = &controller.current_convolution {
            let target = (ir.wetness_bias * estimate.wetness * 0.8 * quality_multiplier).clamp(0.0, 0.7);
            let _ = new_conv.set_mix(fade_in * target);
        }

        // Finish crossfade
        if t >= 1.0 {
            if let Some(old) = controller.fading_out_convolution.take() {
                // In full implementation we would properly remove the effect
                let _ = old;
            }
            controller.crossfade_timer = 0.0;
        }
    } else if let Some(conv) = &controller.current_convolution {
        // Normal modulation when not crossfading
        let target = (ir.wetness_bias * estimate.wetness * 0.7 * quality_multiplier).clamp(0.0, 0.65);
        let _ = conv.set_mix(target);
    }
}
