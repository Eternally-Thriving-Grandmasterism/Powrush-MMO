/*!
 * Environmental Audio - Weather-Aware Footsteps
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio, Priority};
use crate::settings::environmental_audio::{SurfaceType, MaterialModifier, ArmorType, WeatherState};

/// Weather-aware layered footsteps (rain affects surface sounds)
pub fn play_weather_aware_footstep(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mixer: Res<AudioMixer>,
    weather: Res<WeatherState>,
    surface: SurfaceType,
    armor: ArmorType,
    velocity: f32,
) {
    let base_volume = (velocity / 8.0).clamp(0.4, 1.0) * mixer.sfx;
    let rain_factor = weather.rain_intensity;

    // Determine effective material based on weather
    let effective_material = if rain_factor > 0.4 {
        match surface {
            SurfaceType::Stone | SurfaceType::Dirt | SurfaceType::Wood => MaterialModifier::Wet,
            SurfaceType::Grass => MaterialModifier::Muddy,
            _ => MaterialModifier::Wet,
        }
    } else {
        MaterialModifier::Dry
    };

    // Play surface sound (weather-modified)
    let surface_path = match (surface, effective_material) {
        (SurfaceType::Stone, MaterialModifier::Wet) => "audio/footsteps/stone_wet.ogg",
        (SurfaceType::Dirt, MaterialModifier::Muddy) => "audio/footsteps/dirt_muddy.ogg",
        (SurfaceType::Grass, MaterialModifier::Muddy) => "audio/footsteps/grass_wet.ogg",
        _ => match surface {
            SurfaceType::Grass => "audio/footsteps/grass.ogg",
            SurfaceType::Stone => "audio/footsteps/stone.ogg",
            SurfaceType::Water => "audio/footsteps/water.ogg",
            SurfaceType::Snow  => "audio/footsteps/snow.ogg",
            _ => "audio/footsteps/default.ogg",
        },
    };

    commands.spawn((
        AudioBundle {
            source: asset_server.load(surface_path),
            settings: PlaybackSettings::ONCE.with_volume(base_volume),
        },
        DynamicAudio {
            category: AudioCategory::Sfx,
            priority: Priority::Low,
        },
    ));

    // Armor layer (unchanged by weather for now)
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
                settings: PlaybackSettings::ONCE.with_volume(base_volume * 0.75),
            },
            DynamicAudio {
                category: AudioCategory::Sfx,
                priority: Priority::Low,
            },
        ));
    }

    // Extra splash layer when raining heavily
    if rain_factor > 0.6 && (surface == SurfaceType::Stone || surface == SurfaceType::Dirt || surface == SurfaceType::Wood) {
        let splash = asset_server.load("audio/footsteps/splash.ogg");
        commands.spawn((
            AudioBundle {
                source: splash,
                settings: PlaybackSettings::ONCE.with_volume(base_volume * rain_factor * 0.6),
            },
            DynamicAudio {
                category: AudioCategory::Sfx,
                priority: Priority::Low,
            },
        ));
    }
}
