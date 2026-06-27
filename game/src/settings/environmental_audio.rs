/*!
 * Environmental Audio - Advanced (Material Footsteps + Optimized Occlusion + Biome Wind)
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio, Priority};

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
pub enum MaterialModifier {
    Dry,
    Wet,
    Muddy,
    Snowy,
}

#[derive(Clone, Copy)]
pub enum ArmorType {
    None,
    Light,
    Medium,
    Heavy,
}

/// Plays layered footsteps with material variation
pub fn play_layered_footstep(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mixer: Res<AudioMixer>,
    surface: SurfaceType,
    material: MaterialModifier,
    armor: ArmorType,
    velocity: f32,
) {
    let volume = (velocity / 8.0).clamp(0.4, 1.0) * mixer.sfx;

    let surface_path = match (surface, material) {
        (SurfaceType::Stone, MaterialModifier::Wet)   => "audio/footsteps/stone_wet.ogg",
        (SurfaceType::Stone, MaterialModifier::Muddy) => "audio/footsteps/stone_muddy.ogg",
        (SurfaceType::Grass, MaterialModifier::Wet)   => "audio/footsteps/grass_wet.ogg",
        (SurfaceType::Dirt,  MaterialModifier::Muddy) => "audio/footsteps/dirt_muddy.ogg",
        _ => match surface {
            SurfaceType::Grass => "audio/footsteps/grass.ogg",
            SurfaceType::Stone => "audio/footsteps/stone.ogg",
            SurfaceType::Water => "audio/footsteps/water.ogg",
            SurfaceType::Snow  => "audio/footsteps/snow.ogg",
            _ => "audio/footsteps/default.ogg",
        },
    };

    // Surface layer
    commands.spawn((
        AudioBundle {
            source: asset_server.load(surface_path),
            settings: PlaybackSettings::ONCE.with_volume(volume),
        },
        DynamicAudio { category: AudioCategory::Sfx, priority: Priority::Low },
    ));

    // Armor layer
    if armor != ArmorType::None {
        let armor_path = match armor {
            ArmorType::Light  => "audio/footsteps/armor_light.ogg",
            ArmorType::Medium => "audio/footsteps/armor_medium.ogg",
            ArmorType::Heavy  => "audio/footsteps/armor_heavy.ogg",
            _ => return,
        };
        commands.spawn((
            AudioBundle {
                source: asset_server.load(armor_path),
                settings: PlaybackSettings::ONCE.with_volume(volume * 0.75),
            },
            DynamicAudio { category: AudioCategory::Sfx, priority: Priority::Low },
        ));
    }
}

/// Optimized Rapier occlusion with throttling
#[derive(Component)]
pub struct OcclusionThrottler {
    pub last_check: f32,
    pub interval: f32,
}

pub fn apply_occlusion_with_rapier(
    mut dynamic_audio: Query<(&DynamicAudio, &GlobalTransform, &mut AudioSink, Option<&mut OcclusionThrottler>)>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    rapier_context: Res<RapierContext>,
    mixer: Res<AudioMixer>,
    time: Res<Time>,
) {
    let Ok(listener_transform) = listener.get_single() else { return; };
    let listener_pos = listener_transform.translation();

    for (audio, transform, mut sink, mut throttler) in dynamic_audio.iter_mut() {
        let now = time.elapsed_seconds();

        // Throttle expensive raycasts
        if let Some(throttler) = throttler.as_mut() {
            if now - throttler.last_check < throttler.interval {
                continue;
            }
            throttler.last_check = now;
        }

        let source_pos = transform.translation();
        let direction = listener_pos - source_pos;
        let distance = direction.length();

        if distance < 2.0 { continue; }

        // Optimized raycast (ignore sensors and certain layers)
        let filter = QueryFilter::new().exclude_sensors();
        let hit = rapier_context.cast_ray(source_pos, direction.normalize(), distance, true, filter);

        if hit.is_some() {
            let occlusion_factor = match audio.priority {
                Priority::Critical => 0.75,
                Priority::High     => 0.55,
                _ => 0.35,
            };
            let occluded_volume = mixer.get_volume_for_category(audio.category) * occlusion_factor;
            sink.set_volume(occluded_volume);
        }
    }
}

/// Wind intensity by biome/height (expandable)
#[derive(Resource)]
pub struct BiomeWind {
    pub base_intensity: f32,
    pub height_multiplier: f32,
}

impl Default for BiomeWind {
    fn default() -> Self {
        Self {
            base_intensity: 0.4,
            height_multiplier: 0.002,
        }
    }
}

pub fn update_biome_wind(
    mut wind: ResMut<BiomeWind>,
    player: Query<&GlobalTransform, With<Camera3d>>,
) {
    if let Ok(transform) = player.get_single() {
        let height = transform.translation().y;
        wind.base_intensity = (0.3 + (height * wind.height_multiplier)).clamp(0.2, 1.2);
    }
}
