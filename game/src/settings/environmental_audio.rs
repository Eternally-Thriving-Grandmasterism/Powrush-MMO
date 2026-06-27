/*!
 * Environmental Audio - Unified Spatial Audio System
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio, Priority};
use crate::settings::environmental_audio::ReverbState;

/// Unified system that combines occlusion, distance, and reverb into one coherent pass
pub fn apply_unified_spatial_audio(
    mut dynamic_audio: Query<(&DynamicAudio, &GlobalTransform, &mut AudioSink)>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    rapier_context: Res<RapierContext>,
    mixer: Res<AudioMixer>,
    reverb_state: Res<ReverbState>,
) {
    let Ok(listener_transform) = listener.get_single() else { return; };
    let listener_pos = listener_transform.translation();

    for (audio, transform, mut sink) in dynamic_audio.iter_mut() {
        let source_pos = transform.translation();
        let distance = source_pos.distance(listener_pos);

        if distance < 1.5 {
            // Very close - minimal processing
            let base = mixer.get_volume_for_category(audio.category);
            sink.set_volume(base);
            continue;
        }

        // 1. Base volume from mixer
        let base_volume = mixer.get_volume_for_category(audio.category);

        // 2. Distance attenuation (exponential)
        let distance_attenuation = if distance > 80.0 {
            0.0
        } else {
            let normalized = distance / 80.0;
            (-2.5 * normalized).exp()
        };

        // 3. Occlusion check
        let direction = (listener_pos - source_pos).normalize();
        let occlusion_hit = rapier_context.cast_ray(
            source_pos,
            direction,
            distance,
            true,
            QueryFilter::exclude_sensors(),
        );

        let occlusion_factor = if occlusion_hit.is_some() {
            match audio.priority {
                Priority::Critical => 0.7,
                Priority::High     => 0.5,
                _ => 0.25,
            }
        } else {
            1.0
        };

        // 4. Distance-based reverb contribution
        let distance_reverb = reverb_state.wetness * (0.2 + (distance / 60.0).clamp(0.0, 0.8));
        let reverb_volume = base_volume * (1.0 + distance_reverb * 0.4);

        // 5. Combine everything
        let final_volume = reverb_volume
            * distance_attenuation
            * occlusion_factor
            * (1.0 - reverb_state.damping * 0.25);

        sink.set_volume(final_volume.clamp(0.0, base_volume * 1.6));
    }
}
