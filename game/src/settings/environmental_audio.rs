/*!
 * Environmental Audio - Portal-Based Audio
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio, Priority};

/// Defines a sound portal (door, window, archway, etc.)
#[derive(Component)]
pub struct Portal {
    pub size: f32,           // Radius or half-width of the opening
    pub muffling: f32,       // How much sound is attenuated when passing through (0.0 = none, 1.0 = heavy)
    pub direction: Vec3,     // Normal of the portal plane
}

/// Applies portal-based audio propagation
pub fn apply_portal_audio(
    mut dynamic_audio: Query<(&DynamicAudio, &GlobalTransform, &mut AudioSink)>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    portals: Query<(&Portal, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mixer: Res<AudioMixer>,
) {
    let Ok(listener_transform) = listener.get_single() else { return; };
    let listener_pos = listener_transform.translation();

    for (audio, source_transform, mut sink) in dynamic_audio.iter_mut() {
        let source_pos = source_transform.translation();
        let direct_distance = source_pos.distance(listener_pos);

        if direct_distance < 3.0 {
            continue; // Too close, no need for portal logic
        }

        // Check for direct line of sight first
        let direct_ray = Ray::new(source_pos, (listener_pos - source_pos).normalize());
        let direct_hit = rapier_context.cast_ray(
            direct_ray.origin,
            direct_ray.dir,
            direct_distance,
            true,
            QueryFilter::exclude_sensors(),
        );

        if direct_hit.is_none() {
            // Direct line of sight exists - no extra occlusion
            continue;
        }

        // Look for portals the sound can travel through
        let mut best_portal_factor = 1.0f32;

        for (portal, portal_transform) in portals.iter() {
            let portal_pos = portal_transform.translation();

            // Simple distance check to portal
            let dist_to_portal = source_pos.distance(portal_pos);
            if dist_to_portal > 60.0 { continue; }

            // Check if sound can reach the portal
            let to_portal = (portal_pos - source_pos).normalize();
            let portal_hit = rapier_context.cast_ray(
                source_pos,
                to_portal,
                dist_to_portal,
                true,
                QueryFilter::exclude_sensors(),
            );

            if portal_hit.is_none() {
                // Sound can reach this portal
                let portal_to_listener = listener_pos.distance(portal_pos);
                let total_distance = dist_to_portal + portal_to_listener;

                // Attenuation through portal
                let portal_attenuation = (1.0 - portal.muffling) * (portal.size / 10.0).clamp(0.3, 1.0);
                let distance_attenuation = (1.0 / (1.0 + total_distance * 0.02)).clamp(0.2, 1.0);

                let portal_factor = portal_attenuation * distance_attenuation;

                if portal_factor > best_portal_factor {
                    best_portal_factor = portal_factor;
                }
            }
        }

        // Apply final volume
        let base_volume = mixer.get_volume_for_category(audio.category);
        let final_volume = base_volume * best_portal_factor;

        // High priority sounds travel better through portals
        let priority_boost = match audio.priority {
            Priority::Critical => 1.3,
            Priority::High     => 1.15,
            _ => 1.0,
        };

        sink.set_volume((final_volume * priority_boost).clamp(0.0, base_volume));
    }
}
