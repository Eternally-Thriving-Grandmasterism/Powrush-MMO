// client/src/onboarding_ui.rs
// Powrush-MMO v18.9 — Visual Polish for Invite Validation States

use bevy::prelude::*;
use crate::onboarding::{OnboardingState, OnboardingStep};

#[derive(Component)]
pub struct InviteInputPanel;

#[derive(Component)]
pub struct InviteInputField;

#[derive(Component)]
pub struct InviteStatusText;

#[derive(Resource, Default)]
pub struct InviteInputState {
    pub current_text: String,
    pub is_focused: bool,
}

pub struct OnboardingUIPlugin;

impl Plugin for OnboardingUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InviteInputState>()
            .add_systems(Startup, spawn_onboarding_ui)
            .add_systems(Update, (
                update_onboarding_step_panels,
                update_invite_ui_visibility,
                handle_invite_text_input,
                handle_invite_submission,
                update_invite_input_display,
                update_invite_status_text,
                update_input_field_border_color,
            ));
    }
}

fn spawn_onboarding_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ... existing spawning code ...

    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(520.0),
                height: Val::Px(280.0),
                margin: UiRect::new(Val::Px(-260.0), Val::Auto, Val::Px(-140.0), Val::Auto),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                visibility: Visibility::Hidden,
                ..default()
            },
            InviteInputPanel,
        },
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("ENTER INVITE CODE", TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 22.0,
                color: Color::srgb(0.35, 0.82, 1.0),
            }),
            style: Style { margin: UiRect::bottom(Val::Px(16.0)), ..default() },
            ..default()
        });

        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(52.0),
                    border: UiRect::all(Val::Px(1.5)),
                    border_color: Color::srgb(0.4, 0.7, 0.95).into(),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                background_color: Color::srgb(0.08, 0.1, 0.15).into(),
                ..default()
            },
            InviteInputField,
        ));

        parent.spawn((TextBundle {
            text: Text::from_section("", TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 14.0,
                color: Color::srgb(0.85, 0.85, 0.9),
            }),
            style: Style { margin: UiRect::top(Val::Px(12.0)), ..default() },
            ..default()
        }, InviteStatusText));
    });
}

fn update_invite_ui_visibility(
    onboarding: Res<OnboardingState>,
    mut invite_panel: Query<&mut Visibility, With<InviteInputPanel>>,
) {
    for mut vis in invite_panel.iter_mut() {
        *vis = if onboarding.step == OnboardingStep::InviteValidation {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn handle_invite_text_input(
    mut input_state: ResMut<InviteInputState>,
    mut keyboard_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    mut char_events: EventReader<ReceivedCharacter>,
) {
    if !input_state.is_focused { return; }

    for event in keyboard_events.read() {
        if event.state.is_pressed() && event.key_code == KeyCode::Backspace {
            input_state.current_text.pop();
        }
    }

    for event in char_events.read() {
        if (event.char.is_alphanumeric() || event.char == '-' || event.char == '_') && input_state.current_text.len() < 32 {
            input_state.current_text.push(event.char);
        }
    }
}

fn handle_invite_submission(
    mut input_state: ResMut<InviteInputState>,
    mut onboarding: ResMut<OnboardingState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if onboarding.step != OnboardingStep::InviteValidation { return; }

    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::NumpadEnter) {
        if !input_state.current_text.is_empty() {
            onboarding.invite_code = Some(input_state.current_text.clone());
            input_state.current_text.clear();
        }
    }
}

fn update_invite_input_display(
    input_state: Res<InviteInputState>,
    mut query: Query<&mut Text, With<InviteInputField>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = if input_state.current_text.is_empty() {
            "Type invite code...".to_string()
        } else {
            input_state.current_text.clone()
        };
    }
}

// Polished status text with color coding
fn update_invite_status_text(
    onboarding: Res<OnboardingState>,
    mut status_query: Query<&mut Text, With<InviteStatusText>>,
) {
    for mut text in status_query.iter_mut() {
        if let Some(error) = &onboarding.invite_error {
            text.sections[0].value = error.clone();

            if error.contains("wait") {
                // Rate limited state (orange/warning)
                text.sections[0].style.color = Color::srgb(0.95, 0.75, 0.4);
            } else {
                // Regular error (red)
                text.sections[0].style.color = Color::srgb(0.95, 0.55, 0.55);
            }
        } else if onboarding.invite_validated {
            text.sections[0].value = "Invite accepted! Welcome to the beta.".to_string();
            text.sections[0].style.color = Color::srgb(0.5, 0.95, 0.65);
        } else {
            text.sections[0].value = "Enter a valid invite code and press Enter".to_string();
            text.sections[0].style.color = Color::srgb(0.8, 0.85, 0.95);
        }
    }
}

// Visual feedback on input field border
fn update_input_field_border_color(
    onboarding: Res<OnboardingState>,
    mut field_query: Query<&mut NodeBundle, With<InviteInputField>>,
) {
    for mut node in field_query.iter_mut() {
        if onboarding.invite_error.is_some() {
            if onboarding.invite_error.as_ref().unwrap().contains("wait") {
                // Rate limit warning
                node.border_color = Color::srgb(0.95, 0.7, 0.3).into();
            } else {
                node.border_color = Color::srgb(0.95, 0.5, 0.5).into();
            }
        } else if onboarding.invite_validated {
            node.border_color = Color::srgb(0.4, 0.85, 0.5).into();
        } else {
            node.border_color = Color::srgb(0.4, 0.7, 0.95).into();
        }
    }
}
