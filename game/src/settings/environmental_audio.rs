/*!
 * Environmental Audio - Dynamic Occlusion Zones
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio, Priority};

/// Defines a dynamic occlusion zone (room, building, cave, etc.)
#[derive(Component)]
pub struct OcclusionZone {
    pub muffling: f32,        // 0.0 = no effect, 1.0 = fully muffled
    pub low_pass_strength: f32, // How much high frequencies are cut (simulated)
    pub priority: Priority,
}

/// Applies dynamic occlusion based on zones the listener is inside
pub fn apply_dynamic_occlusion_zones(
    mut dynamic_audio: Query<(&DynamicAudio, &GlobalTransform, &mut AudioSink)>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    zones: Query<(&OcclusionZone, &GlobalTransform, Option<&Collider>)>,
    rapier_context: Res<RapierContext>,
    mixer: Res<AudioMixer>,
) {
    let Ok(listener_transform) = listener.get_single() else { return; };
    let listener_pos = listener_transform.translation();

    // Find strongest active occlusion zone affecting the listener
    let mut strongest_muffling = 0.0f32;
    let mut strongest_priority = Priority::Low;

    for (zone, zone_transform, collider) in zones.iter() {
        let zone_pos = zone_transform.translation();

        let inside = if let Some(collider) = collider {
            // Use Rapier point projection for accurate "inside collider" check
            rapier_context
                .project_point(zone_pos, listener_pos, true, QueryFilter::default())
                .is_some()
        } else {
            // Fallback to simple distance check if no collider
            listener_pos.distance(zone_pos) < 30.0
        };

        if inside {
            if zone.priority as u8 >= strongest_priority as u8 {
                strongest_muffling = zone.muffling.max(strongest_muffling);
                strongest_priority = zone.priority;
            }
        }
    }

    // Apply to all dynamic audio
    for (audio, _transform, mut sink) in dynamic_audio.iter_mut() {
        let base_volume = mixer.get_volume_for_category(audio.category);

        let final_volume = if strongest_muffling > 0.0 {
            match audio.priority {
                Priority::Critical => base_volume,
                Priority::High     => base_volume * (1.0 - strongest_muffling * 0.4),
                _ => base_volume * (1.0 - strongest_muffling),
            }
        } else {
            base_volume
        };

        sink.set_volume(final_volume);
    }
}
