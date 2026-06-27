/*!
 * Environmental Audio - Occlusion + Layered Footsteps
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio, Priority};

/// Applies real occlusion using Rapier raycasting
pub fn apply_occlusion_with_rapier(
    mut dynamic_audio: Query<(&DynamicAudio, &GlobalTransform, &mut AudioSink)>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    rapier_context: Res<RapierContext>,
    mixer: Res<AudioMixer>,
) {
    let Ok(listener_transform) = listener.get_single() else { return; };
    let listener_pos = listener_transform.translation();

    for (audio, transform, mut sink) in dynamic_audio.iter_mut() {
        let source_pos = transform.translation();
        let direction = listener_pos - source_pos;
        let distance = direction.length();

        if distance < 1.0 {
            continue;
        }

        let ray = Ray::new(source_pos, direction.normalize());
        let max_toi = distance;

        // Cast ray from sound to listener
        if rapier_context.cast_ray(
            ray.origin,
            ray.dir,
            max_toi,
            true,
            QueryFilter::default(),
        ).is_some() {
            // Something is blocking the sound
            let occluded_volume = match audio.priority {
                Priority::Critical => mixer.get_volume_for_category(audio.category) * 0.7,
                Priority::High     => mixer.get_volume_for_category(audio.category) * 0.5,
                _ => mixer.get_volume_for_category(audio.category) * 0.3,
            };
            sink.set_volume(occluded_volume);
        }
    }
}

/// Plays layered footsteps (surface + armor/gear)
pub fn play_layered_footstep(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mixer: Res<AudioMixer>,
    surface: SurfaceType,
    armor_type: ArmorType,
    velocity: f32,
) {
    let volume = (velocity / 8.0).clamp(0.4, 1.0) * mixer.sfx;

    // Surface sound
    let surface_path = match surface {
        SurfaceType::Grass => "audio/footsteps/grass.ogg",
        SurfaceType::Stone => "audio/footsteps/stone.ogg",
        SurfaceType::Water => "audio/footsteps/water.ogg",
        SurfaceType::Snow  => "audio/footsteps/snow.ogg",
        _ => "audio/footsteps/default.ogg",
    };

    commands.spawn((
        AudioBundle {
            source: asset_server.load(surface_path),
            settings: PlaybackSettings::ONCE.with_volume(volume),
        },
        DynamicAudio {
            category: AudioCategory::Sfx,
            priority: Priority::Low,
        },
    ));

    // Armor / gear layer
    let armor_path = match armor_type {
        ArmorType::Light  => "audio/footsteps/armor_light.ogg",
        ArmorType::Medium => "audio/footsteps/armor_medium.ogg",
        ArmorType::Heavy  => "audio/footsteps/armor_heavy.ogg",
        ArmorType::None   => return,
    };

    commands.spawn((
        AudioBundle {
            source: asset_server.load(armor_path),
            settings: PlaybackSettings::ONCE.with_volume(volume * 0.8),
        },
        DynamicAudio {
            category: AudioCategory::Sfx,
            priority: Priority::Low,
        },
    ));
}

#[derive(Clone, Copy)]
pub enum SurfaceType {
    Grass,
    Stone,
    Dirt,
    Water,
    Snow,
    Wood,
}

#[derive(Clone, Copy)]
pub enum ArmorType {
    None,
    Light,
    Medium,
    Heavy,
}
