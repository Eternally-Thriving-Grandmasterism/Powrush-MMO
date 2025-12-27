use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct WorldWeather {
    pub kind: WeatherKind,
    pub timer: Timer,
    pub intensity: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WeatherKind {
    Clear,
    MercyRain,
    LatticeStorm,
    GoldenSun,
}

pub struct WeatherPlugin;

impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldWeather {
            kind: WeatherKind::Clear,
            timer: Timer::from_seconds(300.0, TimerMode::Once),
            intensity: 0.0,
        })
        .add_systems(Update, (weather_cycle_system, weather_effects_system));
    }
}

fn weather_cycle_system(
    mut weather: ResMut<WorldWeather>,
    time: Res<Time>,
) {
    weather.timer.tick(time.delta());
    if weather.timer.finished() {
        let mut rng = rand::thread_rng();
        weather.kind = match rng.gen_range(0..4) {
            0 => WeatherKind::Clear,
            1 => WeatherKind::MercyRain,
            2 => WeatherKind::LatticeStorm,
            _ => WeatherKind::GoldenSun,
        };
        weather.intensity = 1.0;
        weather.timer.reset();
        info!("Weather changed — {:?}", weather.kind);
    } else {
        weather.intensity = 1.0 - weather.timer.percent();
    }
}

fn weather_effects_system(
    weather: Res<WorldWeather>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    match weather.kind {
        WeatherKind::MercyRain => {
            // Rain particles + sound
            if weather.intensity > 0.5 {
                audio.play(asset_server.load("sounds/rain.ogg"));
            }
        }
        WeatherKind::LatticeStorm => {
            // Lightning flash + lattice glow
            commands.spawn(PointLightBundle {
                point_light: PointLight {
                    intensity: 10000.0 * weather.intensity,
                    color: Color::CYAN,
                    ..default()
                },
                ..default()
            });
        }
        WeatherKind::GoldenSun => {
            // Warm glow
            commands.spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 50000.0 * weather.intensity,
                    color: Color::rgb(1.0, 0.9, 0.7),
                    ..default()
                },
                ..default()
            });
        }
        WeatherKind::Clear => {}
    }
}
