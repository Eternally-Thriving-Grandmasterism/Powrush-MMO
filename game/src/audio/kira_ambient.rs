/*!
 * Kira Ambient - Balanced Multi-Band Reverb with Early Reflection Awareness
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use crate::settings::audio_mixing::ReverbState;
use crate::audio::procedural_reverb_estimation::ProceduralReverbEstimate;

#[derive(Resource, Default)]
pub struct KiraAmbientController {
    pub low_pass_filter: Option<FilterHandle>,
    pub high_pass_filter: Option<FilterHandle>,
    pub ducking: f32,
    pub duck_timer: f32,
}

/// Initialize ambient multi-band filters
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
    info!("Initialized ambient multi-band filters with early reflection support");
}

/// Apply balanced multi-band filtering with early reflection modulation
pub fn apply_kira_ambient_multi_band_filtering(
    reverb_state: Res<ReverbState>,
    estimate: Res<ProceduralReverbEstimate>,
    mut controller: ResMut<KiraAmbientController>,
) {
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;
    let early_delay = estimate.early_reflection_delay_ms;

    // Early reflections influence high-frequency presence
    // Short delay (close reflections) = more open/highs, long delay = more damped
    let early_mod = (early_delay / 80.0).clamp(0.0, 1.0);

    if let Some(lp) = &controller.low_pass_filter {
        let base_cutoff = 2100.0 + (1.0 - high_damping) * 8500.0;
        let cutoff = (base_cutoff * (1.0 - early_mod * 0.15)).max(550.0);
        let _ = lp.set_cutoff(cutoff);
    }

    if let Some(hp) = &controller.high_pass_filter {
        let base_cutoff = 38.0 + low_damping * 110.0;
        // Slightly raise high-pass in very reflective (short delay) spaces
        let cutoff = base_cutoff * (1.0 + early_mod * 0.25);
        let _ = hp.set_cutoff(cutoff.max(30.0));
    }
}
