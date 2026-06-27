/*!
 * Environmental Audio - Distance-Based Reverb
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, DynamicAudio};
use crate::settings::environmental_audio::ReverbState;

/// Applies distance-based reverb (farther sounds become more reverberant)
pub fn apply_distance_based_reverb(
    reverb_state: Res<ReverbState>,
    mut dynamic_audio: Query<(&DynamicAudio, &GlobalTransform, &mut AudioSink)>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    mixer: Res<AudioMixer>,
) {
    let Ok(listener_transform) = listener.get_single() else { return; };
    let listener_pos = listener_transform.translation();

    for (audio, transform, mut sink) in dynamic_audio.iter_mut() {
        let source_pos = transform.translation();
        let distance = source_pos.distance(listener_pos);

        let base_volume = mixer.get_volume_for_category(audio.category);

        // Direct sound falls off with distance (already handled by attenuation)
        // Reverberant sound stays relatively constant
        let distance_factor = (distance / 40.0).clamp(0.0, 1.0);

        // Increase wet signal with distance
        let reverb_wet = reverb_state.wetness * (0.3 + distance_factor * 0.7);

        // Simulate reverb contribution
        let reverb_volume = base_volume * (1.0 + reverb_wet * 0.5);
        let damped_volume = reverb_volume * (1.0 - reverb_state.damping * distance_factor * 0.3);

        sink.set_volume(damped_volume.clamp(0.0, base_volume * 1.8));
    }
}
