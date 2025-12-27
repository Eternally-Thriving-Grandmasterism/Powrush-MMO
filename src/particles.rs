use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct MercyParticle {
    lifetime: Timer,
    velocity: Vec3,
}

fn spawn_mercy_burst(
    mut commands: Commands,
    events: EventReader<TrustGrowthEvent>,
) {
    let mut rng = rand::thread_rng();
    for _ in events.read() {
        for _ in 0..20 {
            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let speed = rng.gen_range(50.0..150.0);
            commands.spawn((
                MercyParticle {
                    lifetime: Timer::from_seconds(1.5, TimerMode::Once),
                    velocity: Vec3::new(angle.cos() * speed, angle.sin() * speed, 0.0),
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::CYAN,
                        custom_size: Some(Vec2::splat(10.0)),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}
