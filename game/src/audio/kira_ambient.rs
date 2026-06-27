/*!
 * Kira Ambient Track - Multi-Band Filtering
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use kira::effect::filter::{FilterBuilder, FilterHandle};
use std::collections::HashMap;
use crate::settings::audio_mixing::ReverbState;

#[derive(Resource, Default)]
pub struct KiraAmbientController {
    pub low_pass_filter: Option<FilterHandle>,
    pub high_pass_filter: Option<FilterHandle>,
    pub ducking: f32,
    pub duck_timer: f32,
}

/// Initialize dedicated ambient multi-band filters
pub fn initialize_kira_ambient_filters(
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraAmbientController>,
) {
    // Low-pass filter for ambients
    if let Ok(lp) = audio.add_filter(FilterBuilder::new().cutoff(2500.0)) {
        controller.low_pass_filter = Some(lp);
    }

    // High-pass filter for ambients
    if let Ok(hp) = audio.add_filter(FilterBuilder::new().cutoff(60.0)) {
        controller.high_pass_filter = Some(hp);
    }

    info!("Initialized dedicated ambient multi-band filters");
}

/// Apply multi-band filtering to ambients based on ReverbState
pub fn apply_kira_ambient_multi_band_filtering(
    reverb_state: Res<ReverbState>,
    mut controller: ResMut<KiraAmbientController>,
) {
    let low_damping = reverb_state.low_damping;
    let high_damping = reverb_state.high_damping;

    if let Some(lp) = &controller.low_pass_filter {
        let cutoff = (2200.0 + (1.0 - high_damping) * 9000.0).max(500.0);
        let _ = lp.set_cutoff(cutoff);
    }

    if let Some(hp) = &controller.high_pass_filter {
        let cutoff = 40.0 + low_damping * 120.0;
        let _ = hp.set_cutoff(cutoff);
    }
}

/// Helper to trigger ducking on ambient track
pub fn duck_ambient_track(
    mut controller: ResMut<KiraAmbientController>,
    duck_amount: f32,
    duration: f32,
) {
    controller.ducking = duck_amount;
    controller.duck_timer = duration;
}
