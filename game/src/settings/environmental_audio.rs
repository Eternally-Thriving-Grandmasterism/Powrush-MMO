/*!
 * Environmental Audio - Weather-Driven + Layered Ambients
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::audio_mixing::{AudioMixer, AudioCategory, DynamicAudio, Priority};

/// Simple weather state that can drive audio
#[derive(Resource)]
pub struct WeatherState {
    pub rain_intensity: f32,   // 0.0 - 1.0
    pub wind_intensity: f32,   // 0.0 - 1.0
    pub thunder_probability: f32,
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            rain_intensity: 0.0,
            wind_intensity: 0.4,
            thunder_probability: 0.0,
        }
    }
}

/// Layered ambient audio system that reacts to weather
pub fn update_weather_ambients(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    weather: Res<WeatherState>,
    mixer: Res<AudioMixer>,
    mut rain_entity: Local<Option<Entity>>,
    mut wind_entity: Local<Option<Entity>>,
) {
    // Rain layer
    let rain_volume = mixer.ambient * weather.rain_intensity * 0.9;

    if weather.rain_intensity > 0.05 {
        if rain_entity.is_none() {
            let rain = asset_server.load("audio/ambient/rain.ogg");
            let entity = commands.spawn((
                AudioBundle {
                    source: rain,
                    settings: PlaybackSettings::LOOP.with_volume(rain_volume),
                },
                DynamicAudio {
                    category: AudioCategory::Ambient,
                    priority: Priority::Low,
                },
            )).id();
            *rain_entity = Some(entity);
        }
    } else if let Some(entity) = *rain_entity {
        commands.entity(entity).despawn();
        *rain_entity = None;
    }

    // Wind layer (intensity driven by weather + existing BiomeWind)
    let wind_volume = mixer.ambient * weather.wind_intensity * 0.7;

    if wind_entity.is_none() && weather.wind_intensity > 0.1 {
        let wind = asset_server.load("audio/ambient/wind.ogg");
        let entity = commands.spawn((
            AudioBundle {
                source: wind,
                settings: PlaybackSettings::LOOP.with_volume(wind_volume),
            },
            DynamicAudio {
                category: AudioCategory::Ambient,
                priority: Priority::Low,
            },
        )).id();
        *wind_entity = Some(entity);
    }
}

/// Example: Thunder system (can be expanded with distance-based panning/volume)
pub fn trigger_thunder(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    weather: Res<WeatherState>,
    mixer: Res<AudioMixer>,
) {
    if weather.thunder_probability > 0.8 {
        // Simple random thunder chance
        if rand::random::<f32>() < 0.02 {
            let thunder = asset_server.load("audio/ambient/thunder.ogg");
            commands.spawn((
                AudioBundle {
                    source: thunder,
                    settings: PlaybackSettings::ONCE.with_volume(mixer.ambient * 0.85),
                },
                DynamicAudio {
                    category: AudioCategory::Ambient,
                    priority: Priority::Normal,
                },
            ));
        }
    }
}
