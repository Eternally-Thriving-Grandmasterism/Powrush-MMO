/*!
 * Kira Ambient - Balanced Multi-Band Reverb with Early Reflection + Pre-Delay
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use kira::effect::delay::{DelayBuilder, DelayHandle};
use crate::settings::audio_mixing::ReverbState;
use crate::audio::procedural_reverb_estimation::ProceduralReverbEstimate;

#[derive(Resource, Default)]
pub struct KiraAmbientController {
    pub low_pass_filter: Option<FilterHandle>,
    pub high_pass_filter: Option<FilterHandle>,
    pub delay: Option<DelayHandle>,
    pub ducking: f32,
    pub duck_timer: f32,
}

/// Initialize ambient multi-band filters + pre-delay
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
    if let Ok(delay) = audio.add_effect(DelayBuilder::new().delay(0.025).feedback(0.15).mix(0.35)) {
        controller.delay = Some(delay);
    }
    info!("Initialized ambient multi-band filters + early reflection pre-delay");
}

/// Apply balanced multi-band filtering with early reflection pre-delay
pub fn apply_kira_ambient_multi_band_filtering(
    reverb_state: Res<ReverbState>,
    estimate: Res<ProceduralReverbEstimate>,
    mut controller: ResMut<KiraAmbientController>,
) {
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;
    let early_delay = estimate.early_reflection_delay_ms;

    let early_mod = (early_delay / 80.0).clamp(0.0, 1.0);

    // Modulate filters based on early reflections
    if let Some(lp) = &controller.low_pass_filter {
        let base_cutoff = 2100.0 + (1.0 - high_damping) * 8500.0;
        let cutoff = (base_cutoff * (1.0 - early_mod * 0.15)).max(550.0);
        let _ = lp.set_cutoff(cutoff);
    }

    if let Some(hp) = &controller.high_pass_filter {
        let base_cutoff = 38.0 + low_damping * 110.0;
        let cutoff = (base_cutoff * (1.0 + early_mod * 0.25)).max(30.0);
        let _ = hp.set_cutoff(cutoff);
    }

    // Drive pre-delay from early reflection estimation
    if let Some(delay) = &controller.delay {
        // Map early reflection delay (ms) to actual delay time (seconds)
        let target_delay = (early_delay / 1000.0).clamp(0.008, 0.12);
        let _ = delay.set_delay(target_delay);
        // Slightly increase mix in more reverberant spaces
        let mix = (0.25 + early_mod * 0.35).clamp(0.2, 0.65);
        let _ = delay.set_mix(mix);
    }
}
