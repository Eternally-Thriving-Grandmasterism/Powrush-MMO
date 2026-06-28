/*!
 * Kira Ambient - Hybrid with True IR Swapping on Load
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use kira::effect::delay::DelayHandle;
use kira::effect::convolution::{ConvolutionBuilder, ConvolutionHandle};
use crate::settings::audio_mixing::ReverbState;
use crate::audio::procedural_reverb_estimation::ProceduralReverbEstimate;
use crate::audio::ir_manager::CurrentImpulseResponse;

#[derive(Resource, Default)]
pub struct KiraAmbientController {
    pub low_pass_filter: Option<FilterHandle>,
    pub high_pass_filter: Option<FilterHandle>,
    pub delay: Option<DelayHandle>,
    pub convolution: Option<ConvolutionHandle>,
    pub last_ir_name: String, // Track to detect changes
    pub ducking: f32,
    pub duck_timer: f32,
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
    // Initial convolution (will be replaced when real IR loads)
    if let Ok(conv) = audio.add_effect(ConvolutionBuilder::new().mix(0.0)) {
        controller.convolution = Some(conv);
        controller.last_ir_name = "none".to_string();
    }
}

/// Detects loaded IR changes and recreates the convolution effect with the real impulse response.
pub fn apply_kira_ambient_multi_band_filtering(
    reverb_state: Res<ReverbState>,
    estimate: Res<ProceduralReverbEstimate>,
    current_ir: Res<CurrentImpulseResponse>,
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraAmbientController>,
) {
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;
    let early_delay = estimate.early_reflection_delay_ms;
    let ir = &current_ir.active;

    let early_mod = (early_delay / 80.0).clamp(0.0, 1.0);

    // === Filter + Delay modulation (unchanged) ===
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

    // === True IR Swapping ===
    let ir_changed = controller.last_ir_name != ir.name;
    let has_loaded_source = ir.loaded_source.is_some();

    if ir_changed && has_loaded_source {
        // Remove old convolution if it exists
        if let Some(old_conv) = controller.convolution.take() {
            // Note: In full Kira usage we would remove the effect here if API allows
            let _ = old_conv; // placeholder
        }

        // Create new convolution with the loaded impulse response
        if let Some(loaded_handle) = &ir.loaded_source {
            if let Ok(new_conv) = audio.add_effect(
                ConvolutionBuilder::new()
                    .impulse_response(loaded_handle.clone()) // Use loaded AudioSource
                    .mix((ir.wetness_bias * estimate.wetness * 0.8).clamp(0.0, 0.7))
            ) {
                controller.convolution = Some(new_conv);
                controller.last_ir_name = ir.name.clone();
            }
        }
    } else if let Some(conv) = &controller.convolution {
        // Normal modulation when IR hasn't changed
        let target_wet = (ir.wetness_bias * estimate.wetness * 0.7).clamp(0.0, 0.65);
        let _ = conv.set_mix(target_wet);
    }
}
