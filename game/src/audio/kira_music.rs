/*!
 * Kira Music Crossfade - Ramp time integrated with AdaptiveLayeringSystem
 *
 * Accepts ramp_time from PaletteTransitionEvent so music palette transitions and IR/reverb
 * use the same dynamic duration calculated by context (combat, travel, emotion, tension).
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Powrush-MMO
 */

use bevy::prelude::*;
// TODO: bring in full KiraMusicController, ConvolutionBuilder, AudioManager, Handle, AudioSource, AudioLatencyMetrics when backend complete

pub fn start_music_crossfade(
    controller: &mut KiraMusicController,
    new_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    early_mix: f32,
    source_to_use: Option<&Handle<AudioSource>>,
    audio: Res<AudioManager>,
    time: Res<Time>,
    latency_metrics: Res<AudioLatencyMetrics>,
    ramp_time: f32, // from PaletteTransitionEvent / calculate_dynamic_ramp_time
) {
    if let Some(current) = controller.early_convolution.take() {
        controller.fading_out_convolution = Some(current);
    }
    controller.crossfade_timer = 0.0;
    controller.crossfade_duration = ramp_time.max(0.1); // adaptive ramp applied here

    latency_metrics.record_crossfade_start(time.elapsed_secs());

    if let Some(loaded) = source_to_use {
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

pub fn update_music_crossfade(
    controller: &mut KiraMusicController,
    current_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    early_mix: f32,
    time: Res<Time>,
    latency_metrics: Res<AudioLatencyMetrics>,
) {
    controller.crossfade_timer += 1.0 / 60.0;

    let t = (controller.crossfade_timer / controller.crossfade_duration).clamp(0.0, 1.0);

    // fade logic... (unchanged)

    if t >= 1.0 {
        if let Some(old) = controller.fading_out_convolution.take() {
            let _ = old;
        }
        controller.crossfade_timer = 0.0;

        latency_metrics.record_crossfade_complete(time.elapsed_secs());
    }
}
