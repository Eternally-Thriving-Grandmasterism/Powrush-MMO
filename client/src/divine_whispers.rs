/*!
 * Divine Whispers - Client Side (Enhanced for Epiphanies)
 */

use bevy::prelude::*;
use simulation::divine_whispers::DivineWhisperTrigger;
use std::time::Duration;

#[derive(Component)]
struct DivineWhisperUI;

#[derive(Component)]
struct WhisperFadeTimer {
    timer: Timer,
}

pub struct DivineWhispersPlugin;

impl Plugin for DivineWhispersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DivineWhisperTrigger>()
            .add_systems(Startup, setup_divine_whisper_ui)
            .add_systems(Update, (
                receive_divine_whispers,
                update_whisper_fade,
            ));
    }
}

fn setup_divine_whisper_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(18.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(680.0),
                    height: Val::Px(130.0),
                    margin: UiRect::new(Val::Px(-340.0), Val::Auto, Val::Auto, Val::Auto),
                    padding: UiRect::all(Val::Px(24.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(18.0)),
                    flex_direction: FlexDirection::Column,
                    visibility: Visibility::Hidden,
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.11, 0.96).into(),
                border_color: Color::srgb(0.5, 0.75, 1.0).into(),
                ..default()
            },
            DivineWhisperUI,
            Name::new("DivineWhisperPanel"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 20.0,
                            color: Color::srgb(0.95, 0.97, 1.0),
                        },
                    ),
                    style: Style {
                        max_width: Val::Px(620.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("WhisperText"),
            ));

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Italic.ttf"),
                            font_size: 14.0,
                            color: Color::srgb(0.65, 0.8, 0.95),
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                },
                Name::new("WhisperFlavor"),
            ));
        });
}

fn receive_divine_whispers(
    mut events: EventReader<DivineWhisperTrigger>,
    mut panel_query: Query<(&mut Visibility, &Children), With<DivineWhisperUI>>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        for (mut visibility, children) in panel_query.iter_mut() {
            *visibility = Visibility::Visible;

            let is_epiphany = event.is_epiphany;
            let text_color = if is_epiphany {
                Color::srgb(1.0, 0.95, 0.7) // Golden for epiphanies
            } else {
                Color::srgb(0.95, 0.97, 1.0)
            };

            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if text.sections.len() > 0 {
                        text.sections[0].value = if is_epiphany {
                            format!("⚡ {}", event.text)
                        } else {
                            event.text.clone()
                        };
                        text.sections[0].style.color = text_color;
                        text.sections[0].style.font_size = if is_epiphany { 22.0 } else { 20.0 };
                    }
                }
            }

            // Longer duration + special styling for epiphanies
            let duration = if is_epiphany {
                event.duration_seconds.max(8.0)
            } else {
                event.duration_seconds
            };

            commands.entity(child).insert(WhisperFadeTimer {
                timer: Timer::new(Duration::from_secs_f32(duration), TimerMode::Once),
            });

            // Enhanced sound + particles for epiphanies
            play_whisper_sound(&asset_server, event.intensity, is_epiphany);
            spawn_whisper_particles(commands, event.intensity, event.flavor.clone(), is_epiphany);
        }
    }
}

fn play_whisper_sound(
    asset_server: &AssetServer,
    intensity: f32,
    is_epiphany: bool,
) {
    let volume = if is_epiphany {
        (0.75 + intensity * 0.25).clamp(0.5, 1.0)
    } else {
        (0.5 + intensity * 0.3).clamp(0.3, 0.9)
    };

    println!(
        "[Audio] Playing {} whisper (intensity: {:.2}, volume: {:.2})",
        if is_epiphany { "EPIPHANY" } else { "normal" },
        intensity,
        volume
    );
}

fn spawn_whisper_particles(
    commands: &mut Commands,
    intensity: f32,
    flavor: String,
    is_epiphany: bool,
) {
    if is_epiphany {
        println!(
            "[Particles] Strong ethereal burst for epiphany (flavor: {}, intensity: {:.2})",
            flavor, intensity
        );
        // TODO: Spawn richer particle effect for epiphanies
    } else {
        println!("[Particles] Subtle particles (flavor: {})", flavor);
    }
}

fn update_whisper_fade(
    mut query: Query<(Entity, &mut WhisperFadeTimer, &mut Visibility)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut fade, mut visibility) in query.iter_mut() {
        fade.timer.tick(time.delta());

        if fade.timer.finished() {
            *visibility = Visibility::Hidden;
            commands.entity(entity).remove::<WhisperFadeTimer>();
        }
    }
}
