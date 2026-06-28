/*!
 * Kira Music - With Latency Recording
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
use crate::audio::latency_metrics::AudioLatencyMetrics;

// ... (KiraMusicController definition unchanged)

pub fn apply_kira_multi_band_reverb(
    reverb_state: Res<ReverbState>,
    estimate: Res<ProceduralReverbEstimate>,
    current_ir: Res<CurrentImpulseResponse>,
    quality: Res<AudioQualitySettings>,
    listener: Option<Res<AudioListener>>,
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraMusicController>,
    time: Res<Time>,
    latency_metrics: Res<AudioLatencyMetrics>,
) {
    // Record application timestamp for latency tracking
    latency_metrics.record_application(time.elapsed_secs());

    // ... (existing music reverb logic remains the same)
}
