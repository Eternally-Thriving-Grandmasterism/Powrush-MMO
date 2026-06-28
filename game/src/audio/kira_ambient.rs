/*!
 * Kira Ambient - Crossfade Timestamp Recording
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

// Controller and other definitions remain the same

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
    latency_metrics.record_application(time.elapsed_secs());

    // ... existing logic ...

    if ir_changed && has_loaded && !is_crossfading {
        latency_metrics.record_crossfade_start(time.elapsed_secs());
        controller.crossfade_duration = quality.crossfade_duration;
        start_crossfade(&mut controller, ir, estimate.wetness, early_mix, source_to_use, audio);
    }

    if is_crossfading {
        update_crossfade(&mut controller, ir, estimate.wetness, early_mix, time.elapsed_secs(), latency_metrics);
    } else if let Some(conv) = &controller.early_convolution {
        // normal modulation
    }
}

fn start_crossfade(
    controller: &mut KiraAmbientController,
    new_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    early_mix: f32,
    source_to_use: Option<&Handle<AudioSource>>,
    audio: Res<AudioManager>,
) {
    // existing start logic
}

fn update_crossfade(
    controller: &mut KiraAmbientController,
    current_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    early_mix: f32,
    current_time: f32,
    latency_metrics: Res<AudioLatencyMetrics>,
) {
    controller.crossfade_timer += 1.0 / 60.0;

    let t = (controller.crossfade_timer / controller.crossfade_duration).clamp(0.0, 1.0);

    // fade logic...

    if t >= 1.0 {
        if let Some(old) = controller.fading_out_convolution.take() {
            let _ = old;
        }
        controller.crossfade_timer = 0.0;

        latency_metrics.record_crossfade_complete(current_time);
    }
}
