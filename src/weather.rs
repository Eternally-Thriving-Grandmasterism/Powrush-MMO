use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct WorldWeather {
    pub kind: WeatherKind,
    pub timer: Timer,
}

#[derive(Clone, Copy)]
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
        })
        .add_systems(Update, weather_cycle_system);
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
        weather.timer.reset();
    }
}
