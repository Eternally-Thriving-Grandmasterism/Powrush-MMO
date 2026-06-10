/*!
 * Divine Whispers - Client Side (with light Spatial Audio integration)
 */

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioTween};
use simulation::divine_whispers::DivineWhisperTrigger;
use std::time::Duration;

use crate::spatial_audio::PlaySpatialSound;

#[derive(Component)]
struct DivineWhisperUI;

#[derive(Component)]
struct WhisperFadeTimer {
    timer: Timer,
}

#[derive(Component)]
struct EpiphanyFlash;

pub struct DivineWhispersPlugin;

impl Plugin for DivineWhispersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DivineWhisperTrigger>()
            .add_systems(Startup, setup_divine_whisper_ui)
            .add_systems(Update, (
                receive_divine_whispers,
                update_whisper_fade,
                update_epiphany_flash,
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
    mut panel_query: Query<(&mut Visibility, &Children, Entity), With<DivineWhisperUI>>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut spatial_events: EventWriter<PlaySpatialSound>,
) {
    for event in events.read() {
        for (mut visibility, children, panel_entity) in panel_query.iter_mut() {
            *visibility = Visibility::Visible;

            let is_epiphany = event.is_epiphany;

            if is_epiphany {
                commands.entity(panel_entity).insert(EpiphanyFlash);

                // Light integration: Trigger spatial sound on epiphany
                spatial_events.send(PlaySpatialSound::new(
                    asset_server.load("sounds/epiphany_impact.ogg"),
                    Vec3::new(0.0, 2.0, -8.0), // Position slightly in front of listener
                ).with_volume(0.9));
            }

            let text_color = if is_epiphany {
                Color::srgb(1.0, 0.95, 0.7)
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

            let duration = if is_epiphany {
                event.duration_seconds.max(8.0)
            } else {
                event.duration_seconds
            };

            commands.entity(panel_entity).insert(WhisperFadeTimer {
                timer: Timer::new(Duration::from_secs_f32(duration), TimerMode::Once),
            });

            play_whisper_sound(&audio, &asset_server, event.intensity, is_epiphany);
            spawn_whisper_particles(&mut commands, event.intensity, event.flavor.clone(), is_epiphany, panel_entity);
        }
    }
}

fn play_whisper_sound(
    audio: &Audio,
    asset_server: &AssetServer,
    intensity: f32,
    is_epiphany: bool,
) {
    let volume = if is_epiphany {
        (0.85 + intensity * 0.15).clamp(0.7, 1.0)
    } else {
        (0.55 + intensity * 0.25).clamp(0.35, 0.85)
    };

    let sound_path = if is_epiphany {
        "sounds/epiphany_whisper.ogg"
    } else {
        "sounds/divine_whisper.ogg"
    };

    audio
        .play(asset_server.load(sound_path))
        .with_volume(volume as f64)
        .fade_in(AudioTween::new(
            Duration::from_millis(if is_epiphany { 250 } else { 180 }),
            bevy_kira_audio::AudioEasing::OutPowi(2),
        ));

    println!(
        "[Audio] Playing {} whisper (intensity: {:.2})",
        if is_epiphany { "EPIPHANY" } else { "normal" },
        intensity
    );
}

fn spawn_whisper_particles(
    commands: &mut Commands,
    intensity: f32,
    flavor: String,
    is_epiphany: bool,
    panel_entity: Entity,
) {
    if is_epiphany {
        println!(
            "[Particles] Strong ethereal epiphany burst (flavor: {}, intensity: {:.2})",
            flavor, intensity
        );

        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(700.0),
                    height: Val::Px(150.0),
                    ..default()
                },
                background_color: Color::srgba(1.0, 0.95, 0.6, 0.18).into(),
                ..default()
            },
            EpiphanyFlash,
        ));
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

fn update_epiphany_flash(
    mut query: Query<Entity, With<EpiphanyFlash>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
