/*!
 * Kira Ambient - Balanced Multi-Band Reverb
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use crate::settings::audio_mixing::ReverbState;

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
    info!("Initialized ambient multi-band filters");
}

/// Apply balanced multi-band filtering to ambients
pub fn apply_kira_ambient_multi_band_filtering(
    reverb_state: Res<ReverbState>,
    mut controller: ResMut<KiraAmbientController>,
) {
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;

    if let Some(lp) = &controller.low_pass_filter {
        let cutoff = (2100.0 + (1.0 - high_damping) * 8500.0).max(550.0);
        let _ = lp.set_cutoff(cutoff);
    }

    if let Some(hp) = &controller.high_pass_filter {
        let cutoff = 38.0 + low_damping * 110.0;
        let _ = hp.set_cutoff(cutoff);
    }
}
