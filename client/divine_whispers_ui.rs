//! client/divine_whispers_ui.rs
//! Elegant, mercy-themed UI for displaying Divine Whispers received from the local Ra-Thor server bridge.
//! Non-intrusive, fades beautifully, integrates with existing Bevy UI / inventory systems.
//! AG-SML | One Lattice

use bevy::prelude::*;
use powrush_divine_module::DivineWhisper;
use std::time::Duration;

/// Component attached to the Divine Whisper UI entity
#[derive(Component)]
pub struct DivineWhisperUI {
    pub lifetime: Timer,
}

/// Resource holding the latest whisper (or queue if we want history later)
#[derive(Resource, Default)]
pub struct CurrentDivineWhisper {
    pub whisper: Option<DivineWhisper>,
}

/// Plugin to add Divine Whispers UI
pub struct DivineWhispersUIPlugin;

impl Plugin for DivineWhispersUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentDivineWhisper>()
            .add_systems(Startup, spawn_divine_whisper_ui)
            .add_systems(Update, (
                update_divine_whisper_display,
                fade_out_whisper,
            ));
    }
}

/// Spawn the UI container (call once at startup)
fn spawn_divine_whisper_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    bottom: Val::Px(80.0),
                    width: Val::Px(420.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.06, 0.12, 0.85).into(), // deep mercy purple
                border_radius: BorderRadius::all(Val::Px(12.0)),
                ..default()
            },
            DivineWhisperUI {
                lifetime: Timer::new(Duration::from_secs(8), TimerMode::Once),
            },
            Name::new("DivineWhisperPanel"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "The Lattice is silent...",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"), // or your preferred font
                            font_size: 18.0,
                            color: Color::srgb(0.95, 0.92, 1.0), // soft luminous white
                        },
                    ),
                    style: Style {
                        max_width: Val::Px(380.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("DivineWhisperText"),
            ));
        });
}

/// Call this from your harvest / event systems when a new DivineWhisper arrives from server
pub fn show_divine_whisper(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
) {
    current.whisper = Some(whisper.clone());

    for (mut text, mut ui) in query.iter_mut() {
        text.sections[0].value = whisper.message.clone();
        // Reset lifetime
        ui.lifetime = Timer::new(Duration::from_secs(8), TimerMode::Once);
        ui.lifetime.reset();
    }
}

fn update_divine_whisper_display(
    current: Res<CurrentDivineWhisper>,
    mut query: Query<&mut Text, With<DivineWhisperUI>>,
) {
    if let Some(whisper) = &current.whisper {
        for mut text in query.iter_mut() {
            if text.sections[0].value != whisper.message {
                text.sections[0].value = whisper.message.clone();
            }
        }
    }
}

fn fade_out_whisper(
    time: Res<Time>,
    mut query: Query<(&mut DivineWhisperUI, &mut Visibility)>,
    mut current: ResMut<CurrentDivineWhisper>,
) {
    for (mut ui, mut visibility) in query.iter_mut() {
        ui.lifetime.tick(time.delta());

        if ui.lifetime.finished() {
            *visibility = Visibility::Hidden;
            current.whisper = None;
        } else if ui.lifetime.percent() > 0.75 {
            // Gentle fade near end
            *visibility = Visibility::Visible; // could lerp alpha here with bevy 0.14+
        }
    }
}

/// Example: call this from bevy_harvest_integration.rs or rbe_ui_feedback.rs
/// when server sends a DivineWhisper after harvest
pub fn receive_divine_whisper_from_server(
    whisper: DivineWhisper,
    current: &mut CurrentDivineWhisper,
    ui_query: &mut Query<(&mut Text, &mut DivineWhisperUI)>,
) {
    show_divine_whisper(whisper, current, ui_query);
}