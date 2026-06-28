/*!
 * Kira Ambient - With Latency Recording
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
use crate::audio::latency_metrics::AudioLatencyMetrics;

// ... (KiraAmbientController definition unchanged)

pub fn apply_kira_ambient_multi_band_filtering(
    reverb_state: Res<ReverbState>,
    estimate: Res<ProceduralReverbEstimate>,
    current_ir: Res<CurrentImpulseResponse>,
    quality: Res<AudioQualitySettings>,
    listener: Option<Res<AudioListener>>,
    audio: Res<AudioManager>,
    mut controller: ResMut<KiraAmbientController>,
    time: Res<Time>,
    latency_metrics: Res<AudioLatencyMetrics>,
) {
    // Record that we are applying acoustic changes now
    latency_metrics.record_application(time.elapsed_secs());

    // ... (existing filter, delay, and convolution logic)
    // The rest of the function remains the same
}
