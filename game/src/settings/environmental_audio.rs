/*!
 * Environmental Audio - Reverb Zone Blending
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::settings::audio_mixing::ReverbState;

#[derive(Component)]
pub struct ReverbZone {
    pub wetness: f32,
    pub decay_time: f32,
    pub damping: f32,
    pub early_reflections: f32,
    pub influence_radius: f32, // for blending near edges
}

/// Blends multiple overlapping ReverbZones with distance weighting
pub fn blend_reverb_zones(
    mut reverb_state: ResMut<ReverbState>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    zones: Query<(&ReverbZone, &GlobalTransform, Option<&Collider>)>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(listener_transform) = listener.get_single() else { return; };
    let listener_pos = listener_transform.translation();

    let mut total_weight = 0.0;
    let mut blended_wetness = 0.0;
    let mut blended_decay = 0.0;
    let mut blended_damping = 0.0;
    let mut blended_early = 0.0;

    for (zone, zone_transform, collider) in zones.iter() {
        let zone_pos = zone_transform.translation();
        let mut is_inside = false;
        let mut distance = listener_pos.distance(zone_pos);

        if let Some(collider) = collider {
            // Use Rapier to check if listener is inside the collider
            if rapier_context
                .project_point(zone_pos, listener_pos, true, QueryFilter::default())
                .is_some()
            {
                is_inside = true;
                distance = 0.0;
            }
        }

        if is_inside || distance < zone.influence_radius {
            let weight = if is_inside {
                1.0
            } else {
                (zone.influence_radius - distance) / zone.influence_radius
            };

            blended_wetness += zone.wetness * weight;
            blended_decay += zone.decay_time * weight;
            blended_damping += zone.damping * weight;
            blended_early += zone.early_reflections * weight;
            total_weight += weight;
        }
    }

    if total_weight > 0.0 {
        reverb_state.wetness = blended_wetness / total_weight;
        reverb_state.decay_time = blended_decay / total_weight;
        reverb_state.damping = blended_damping / total_weight;
    }
}
