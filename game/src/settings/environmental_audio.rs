/*!
 * Environmental Audio - Acoustic Material Properties
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio, Priority};

/// Acoustic properties of a surface or zone
#[derive(Component, Clone, Copy, Debug)]
pub struct AcousticMaterial {
    pub absorption: f32,    // 0.0 = reflects everything, 1.0 = absorbs everything
    pub reflection: f32,    // How much sound bounces (affects reverb simulation)
    pub transmission: f32,  // How much sound passes through (for portals/walls)
}

impl Default for AcousticMaterial {
    fn default() -> Self {
        Self {
            absorption: 0.3,
            reflection: 0.7,
            transmission: 0.2,
        }
    }
}

/// Common acoustic materials
impl AcousticMaterial {
    pub const STONE: Self = Self { absorption: 0.1, reflection: 0.9, transmission: 0.05 };
    pub const WOOD: Self = Self { absorption: 0.4, reflection: 0.6, transmission: 0.15 };
    pub const CARPET: Self = Self { absorption: 0.7, reflection: 0.3, transmission: 0.1 };
    pub const METAL: Self = Self { absorption: 0.05, reflection: 0.95, transmission: 0.02 };
    pub const CURTAIN: Self = Self { absorption: 0.8, reflection: 0.2, transmission: 0.3 };
    pub const CONCRETE: Self = Self { absorption: 0.15, reflection: 0.85, transmission: 0.08 };
}

/// Enhanced occlusion that factors in acoustic material of blocking surfaces
pub fn apply_acoustic_occlusion(
    mut dynamic_audio: Query<(&DynamicAudio, &GlobalTransform, &mut AudioSink)>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    rapier_context: Res<RapierContext>,
    mixer: Res<AudioMixer>,
    // In a full implementation, we would query AcousticMaterial on hit colliders
) {
    let Ok(listener_transform) = listener.get_single() else { return; };
    let listener_pos = listener_transform.translation();

    for (audio, transform, mut sink) in dynamic_audio.iter_mut() {
        let source_pos = transform.translation();
        let direction = (listener_pos - source_pos).normalize();
        let distance = source_pos.distance(listener_pos);

        if distance < 2.0 { continue; }

        // Raycast to detect blocking surface
        if let Some((hit_entity, _toi)) = rapier_context.cast_ray(
            source_pos,
            direction,
            distance,
            true,
            QueryFilter::exclude_sensors(),
        ) {
            // Try to get AcousticMaterial from the hit entity
            // For now we use a default material-based calculation
            let material = AcousticMaterial::default(); // TODO: Query actual material from hit_entity

            let transmission = material.transmission.max(0.05);
            let absorption_factor = 1.0 - material.absorption;

            let base_volume = mixer.get_volume_for_category(audio.category);

            // High priority sounds penetrate materials better
            let priority_factor = match audio.priority {
                Priority::Critical => 0.9,
                Priority::High     => 0.7,
                _ => 0.5,
            };

            let occluded_volume = base_volume * transmission * absorption_factor * priority_factor;
            sink.set_volume(occluded_volume.clamp(0.0, base_volume));
        }
    }
}
