/*!
 * Environmental Audio - Improved Procedural Reverb Zones
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::settings::audio_mixing::ReverbState;
use crate::settings::environmental_audio::AcousticMaterial;

/// Improved procedural reverb estimation using more rays + material awareness
pub fn update_procedural_reverb(
    mut reverb_state: ResMut<ReverbState>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
    mut last_update: Local<f32>,
) {
    // Throttle for performance (every ~0.2 seconds)
    if time.elapsed_seconds() - *last_update < 0.2 {
        return;
    }
    *last_update = time.elapsed_seconds();

    let Ok(listener_transform) = listener.get_single() else { return; };
    let origin = listener_transform.translation();

    // Define directions (more rays for better estimation)
    let directions = [
        Vec3::X, -Vec3::X, Vec3::Y, -Vec3::Y, Vec3::Z, -Vec3::Z,
        Vec3::new(0.7, 0.0, 0.7).normalize(),
        Vec3::new(-0.7, 0.0, 0.7).normalize(),
        Vec3::new(0.7, 0.0, -0.7).normalize(),
        Vec3::new(-0.7, 0.0, -0.7).normalize(),
        Vec3::new(0.0, 0.7, 0.7).normalize(),
        Vec3::new(0.0, -0.7, 0.7).normalize(),
    ];

    let max_distance = 40.0;
    let mut total_distance = 0.0;
    let mut hit_count = 0;
    let mut absorption_sum = 0.0;
    let mut vertical_hits = 0;

    for dir in directions.iter() {
        if let Some((entity, toi)) = rapier_context.cast_ray(
            origin,
            *dir,
            max_distance,
            true,
            QueryFilter::exclude_sensors(),
        ) {
            total_distance += toi;
            hit_count += 1;

            // Try to read AcousticMaterial from hit entity
            // (In real code this would require a query or component lookup)
            absorption_sum += 0.3; // placeholder until we integrate proper material lookup

            if dir.y.abs() > 0.5 {
                vertical_hits += 1;
            }
        } else {
            total_distance += max_distance;
        }
    }

    if hit_count == 0 {
        // Completely open space
        let target_wetness = 0.15;
        let target_decay = 1.3;
        let target_damping = 0.25;

        reverb_state.wetness = reverb_state.wetness.lerp(target_wetness, 0.12);
        reverb_state.decay_time = reverb_state.decay_time.lerp(target_decay, 0.1);
        reverb_state.damping = reverb_state.damping.lerp(target_damping, 0.1);
        return;
    }

    let avg_distance = total_distance / directions.len() as f32;
    let enclosure = (1.0 - (avg_distance / max_distance)).clamp(0.0, 1.0);

    // More vertical hits = more "room-like"
    let vertical_factor = (vertical_hits as f32 / 4.0).clamp(0.0, 1.0);

    // Estimate reverb parameters
    let target_wetness = (enclosure * 0.75 + vertical_factor * 0.25).clamp(0.1, 0.95);
    let target_decay = (1.2 + enclosure * 3.5).clamp(1.2, 5.5);
    let target_damping = 0.3 + enclosure * 0.4;

    // Smoothly blend into global reverb state
    reverb_state.wetness = reverb_state.wetness.lerp(target_wetness, 0.15);
    reverb_state.decay_time = reverb_state.decay_time.lerp(target_decay, 0.12);
    reverb_state.damping = reverb_state.damping.lerp(target_damping, 0.1);
}
