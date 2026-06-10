/*!
 * Divine Whispers - Client Side UI + Sound + Particles
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
                    width: Val::Px(620.0),
                    height: Val::Px(110.0),
                    margin: UiRect::new(Val::Px(-310.0), Val::Auto, Val::Auto, Val::Auto),
                    padding: UiRect::all(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(16.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    visibility: Visibility::Hidden,
                    ..default()
                },
                background_color: Color::srgba(0.06, 0.08, 0.12, 0.92).into(),
                border_color: Color::srgb(0.4, 0.7, 0.95).into(),
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
                            font_size: 18.0,
                            color: Color::srgb(0.95, 0.97, 1.0),
                        },
                    ),
                    style: Style {
                        max_width: Val::Px(580.0),
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
                            font_size: 13.0,
                            color: Color::srgb(0.6, 0.75, 0.95),
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(8.0)),
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
    mut query: Query<(&mut Visibility, &Children), With<DivineWhisperUI>>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in events.read() {
        for (mut visibility, children) in query.iter_mut() {
            *visibility = Visibility::Visible;

            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if text.sections.len() > 0 {
                        if text.sections[0].value.len() < 5 {
                            text.sections[0].value = event.text.clone();
                        } else {
                            text.sections[0].value = format!("— {}", event.flavor);
                        }
                    }
                }
            }

            // Spawn fade timer
            commands.entity(child).insert(WhisperFadeTimer {
                timer: Timer::new(
                    Duration::from_secs_f32(event.duration_seconds),
                    TimerMode::Once,
                ),
            });

            // === Sound + Particles ===
            play_whisper_sound(&asset_server, event.intensity);
            spawn_whisper_particles(&mut commands, event.intensity, event.flavor.clone());
        }
    }
}

fn play_whisper_sound(asset_server: &AssetServer, intensity: f32) {
    // Placeholder for audio playback
    // In production: use AudioBundle or bevy_kira_audio
    println!("[Audio] Playing subtle Divine Whisper sound (intensity: {:.2})", intensity);

    // Example (if using bevy_audio):
    // commands.spawn(AudioBundle {
    //     source: asset_server.load("sounds/divine_whisper.ogg"),
    //     settings: PlaybackSettings::DESPAWN,
    // });
}

fn spawn_whisper_particles(commands: &mut Commands, intensity: f32, flavor: String) {
    // Spawn subtle ethereal particles around the whisper panel
    // This is a simplified version. In production use a proper particle plugin.

    println!(
        "[Particles] Spawning ethereal particles for whisper (flavor: {}, intensity: {:.2})",
        flavor, intensity
    );

    // TODO: Spawn actual particle entities or use bevy_particle_systems
    // Example direction:
    // - Soft glowing orbs rising slowly
    // - Color based on flavor (blue for harmony, green for abundance, etc.)
    // - Lifetime and count scaled by intensity
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
