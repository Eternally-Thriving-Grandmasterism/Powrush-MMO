fn start_crossfade(
    controller: &mut KiraAmbientController,
    new_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    early_mix: f32,
    source_to_use: Option<&Handle<AudioSource>>,
    audio: Res<AudioManager>,
    time: Res<Time>,
    latency_metrics: Res<AudioLatencyMetrics>,
) {
    if let Some(current) = controller.early_convolution.take() {
        controller.fading_out_convolution = Some(current);
    }
    controller.crossfade_timer = 0.0;

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

fn update_crossfade(
    controller: &mut KiraAmbientController,
    current_ir: &crate::audio::ir_manager::ImpulseResponse,
    current_wetness: f32,
    early_mix: f32,
    time: Res<Time>,
    latency_metrics: Res<AudioLatencyMetrics>,
) {
    controller.crossfade_timer += 1.0 / 60.0;

    let t = (controller.crossfade_timer / controller.crossfade_duration).clamp(0.0, 1.0);

    // fade out / fade in logic...

    if t >= 1.0 {
        if let Some(old) = controller.fading_out_convolution.take() {
            let _ = old;
        }
        controller.crossfade_timer = 0.0;

        latency_metrics.record_crossfade_complete(time.elapsed_secs());
    }
}
