use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct EmoteEvent {
    pub player_id: u64,
    pub emote_type: EmoteType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EmoteType {
    Dance,
    Wave,
    Mercy,
}

pub struct EmotePlugin;

impl Plugin for EmotePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EmoteEvent>()
           .add_systems(Update, (emote_input_system, emote_visual_system));
    }
}

fn emote_input_system(
    keyboard: Res<Input<KeyCode>>,
    mut events: EventWriter<EmoteEvent>,
) {
    if keyboard.just_pressed(KeyCode::D) {
        events.send(EmoteEvent { player_id: 1, emote_type: EmoteType::Dance });
    } else if keyboard.just_pressed(KeyCode::W) {
        events.send(EmoteEvent { player_id: 1, emote_type: EmoteType::Wave });
    } else if keyboard.just_pressed(KeyCode::M) {
        events.send(EmoteEvent { player_id: 1, emote_type: EmoteType::Mercy });
    }
}

fn emote_visual_system(
    mut commands: Commands,
    events: EventReader<EmoteEvent>,
) {
    let mut rng = rand::thread_rng();
    for event in events.read() {
        match event.emote_type {
            EmoteType::Dance => {
                for _ in 0..30 {
                    let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
                    let speed = rng.gen_range(100.0..200.0);
                    commands.spawn((
                        MercyParticle {
                            lifetime: Timer::from_seconds(1.0, TimerMode::Once),
                            velocity: Vec3::new(angle.cos() * speed, angle.sin() * speed, 0.0),
                            radius: 10.0,
                            color: Color::GOLD,
                        },
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::GOLD,
                                custom_size: Some(Vec2::splat(12.0)),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                }
            }
            EmoteType::Wave => {
                info!("Wave emote — cyan ripple");
            }
            EmoteType::Mercy => {
                info!("Mercy emote — lattice bloom");
            }
        }
    }
}
