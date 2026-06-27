/*!
 * Environmental Audio - Reverb Simulation
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio};

/// Defines a reverb zone (room, hall, cave, etc.)
#[derive(Component)]
pub struct ReverbZone {
    pub wetness: f32,       // 0.0 = dry, 1.0 = very wet
    pub decay_time: f32,    // How long reverb lasts (seconds)
    pub damping: f32,       // High-frequency damping (0.0 = bright, 1.0 = muffled)
}

/// Current global reverb state (updated every frame)
#[derive(Resource, Default)]
pub struct ReverbState {
    pub wetness: f32,
    pub decay_time: f32,
    pub damping: f32,
}

/// Updates the global reverb state based on the zone the listener is in
pub fn update_reverb_state(
    mut reverb_state: ResMut<ReverbState>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    zones: Query<(&ReverbZone, &GlobalTransform, Option<&Collider>)>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(listener_transform) = listener.get_single() else { return; };
    let listener_pos = listener_transform.translation();

    let mut best_wetness = 0.15f32; // Default outdoor/low reverb
    let mut best_decay = 1.2;
    let mut best_damping = 0.3;

    for (zone, zone_transform, collider) in zones.iter() {
        let zone_pos = zone_transform.translation();

        let inside = if let Some(collider) = collider {
            rapier_context
                .project_point(zone_pos, listener_pos, true, QueryFilter::default())
                .is_some()
        } else {
            listener_pos.distance(zone_pos) < 25.0
        };

        if inside {
            if zone.wetness > best_wetness {
                best_wetness = zone.wetness;
                best_decay = zone.decay_time;
                best_damping = zone.damping;
            }
        }
    }

    reverb_state.wetness = best_wetness;
    reverb_state.decay_time = best_decay;
    reverb_state.damping = best_damping;
}

/// Applies reverb wetness to DynamicAudio entities (simulated via volume + low-pass simulation)
pub fn apply_reverb_to_sounds(
    reverb_state: Res<ReverbState>,
    mut dynamic_audio: Query<(&DynamicAudio, &mut AudioSink)>,
    mixer: Res<AudioMixer>,
) {
    for (audio, mut sink) in dynamic_audio.iter_mut() {
        let base_volume = mixer.get_volume_for_category(audio.category);

        // Simple reverb simulation: slightly increase volume + apply damping effect
        // In a real system this would control a reverb send level
        let reverb_volume = base_volume * (1.0 + reverb_state.wetness * 0.3);
        let damped_volume = reverb_volume * (1.0 - reverb_state.damping * 0.2);

        sink.set_volume(damped_volume.clamp(0.0, base_volume * 1.5));
    }
}
