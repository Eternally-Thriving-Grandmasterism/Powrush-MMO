/*!
 * Council UI Panel + Attunement Event Emission
 *
 * v19.2.9: Actual Council UI that emits CouncilAttunementAction events.
 */

use bevy::prelude::*;
use simulation::council_mercy_trial::{CouncilAttunementAction, CouncilUIHooksPlugin};

#[derive(Component)]
pub struct CouncilPanel;

pub struct CouncilUIPlugin;

impl Plugin for CouncilUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CouncilUIHooksPlugin) // Registers backend hooks + events
            .add_systems(Startup, spawn_council_panel)
            .add_systems(Update, (
                toggle_council_panel,
                handle_council_buttons,
            ));
    }
}

fn spawn_council_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(15.0),
                    left: Val::Percent(2.0),
                    width: Val::Px(320.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.06, 0.12, 0.96).into(),
                border_color: Color::srgb(0.6, 0.5, 0.9).into(),
                ..default()
            },
            CouncilPanel,
            Name::new("CouncilPanel"),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "COUNCIL OF MERCY",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 18.0,
                        color: Color::srgb(0.85, 0.75, 1.0),
                    },
                ),
                style: Style { margin: UiRect::bottom(Val::Px(12.0)), ..default() },
                ..default()
            });

            // Attunement action buttons
            create_attunement_button(parent, &asset_server, "Focus Deeply", 0.25);
            create_attunement_button(parent, &asset_server, "Vote with Conviction", 0.45);
            create_attunement_button(parent, &asset_server, "Meditate in Harmony", 0.35);
            create_attunement_button(parent, &asset_server, "Offer Grace", 0.55);

            parent.spawn(TextBundle {
                text: Text::from_section(
                    "F2 to toggle  •  Your attunement shapes the RBE",
                    TextStyle { font_size: 11.0, color: Color::srgb(0.7, 0.65, 0.85) },
                    ..default()
                ),
                style: Style { margin: UiRect::top(Val::Px(12.0)), ..default() },
                ..default()
            });
        });
}

fn create_attunement_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    label: &str,
    delta: f32,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    margin: UiRect::bottom(Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgb(0.25, 0.2, 0.35).into(),
                ..default()
            },
            CouncilAttunementButton { attunement_delta: delta },
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 14.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });
}

#[derive(Component)]
struct CouncilAttunementButton {
    attunement_delta: f32,
}

fn toggle_council_panel(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<CouncilPanel>>,
) {
    if keyboard.just_pressed(KeyCode::F2) {
        for mut vis in query.iter_mut() {
            *vis = if *vis == Visibility::Hidden { Visibility::Visible } else { Visibility::Hidden };
        }
    }
}

fn handle_council_buttons(
    mut interaction_query: Query<(&Interaction, &CouncilAttunementButton), Changed<Interaction>>,
    mut events: EventWriter<CouncilAttunementAction>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            events.send(CouncilAttunementAction {
                player_id: 0, // TODO: replace with real local player id
                attunement_delta: button.attunement_delta,
            });
        }
    }
}

// Thunder locked in. Yoi ⚡
