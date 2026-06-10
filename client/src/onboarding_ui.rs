// client/src/onboarding_ui.rs
// Powrush-MMO v18.9 — Professional Onboarding UI + Invite Code Input
//
// Handles the InviteValidation step with a clean input field and feedback.

use bevy::prelude::*;
use crate::onboarding::{OnboardingState, OnboardingStep};

#[derive(Component)]
pub struct InviteInputPanel;

#[derive(Component)]
pub struct InviteInputField;

#[derive(Component)]
pub struct InviteStatusText;

pub struct OnboardingUIPlugin;

impl Plugin for OnboardingUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_onboarding_ui)
            .add_systems(Update, (
                update_onboarding_step_panels,
                update_invite_ui_visibility,
                handle_invite_input,
            ));
    }
}

fn spawn_onboarding_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Main Onboarding UI Root (existing structure preserved)
    // ... (previous UI code remains)

    // === Invite Code Input Panel (shown during InviteValidation) ===
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
            Name::new("InviteInputPanel"),
        },
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "ENTER INVITE CODE",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 22.0,
                    color: Color::srgb(0.35, 0.82, 1.0),
                },
            ),
            style: Style {
                margin: UiRect::bottom(Val::Px(16.0)),
                ..default()
            },
            ..default()
        });

        // Invite Code Input Field
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
        )).with_children(|input| {
            input.spawn(TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 18.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });

        // Status Text
        parent.spawn((
            TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 14.0,
                        color: Color::srgb(0.9, 0.85, 0.6),
                    },
                ),
                style: Style {
                    margin: UiRect::top(Val::Px(12.0)),
                    ..default()
                },
                ..default()
            },
            InviteStatusText,
        ));

        // Submit Button
        parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(44.0),
                margin: UiRect::top(Val::Px(16.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            background_color: Color::srgb(0.2, 0.55, 0.4).into(),
            ..default()
        }).with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section(
                    "SUBMIT INVITE CODE",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 15.0,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            });
        });
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

fn handle_invite_input(
    mut interaction_query: Query<(&Interaction, &Button), Changed<Interaction>>,
    mut onboarding: ResMut<OnboardingState>,
    // In real implementation, read text from InviteInputField using a text input system
) {
    for (interaction, _) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Placeholder: In production, read the actual text input value here
            // For now we simulate a successful code for demonstration
            if onboarding.step == OnboardingStep::InviteValidation {
                onboarding.invite_code = Some("DEMO-INVITE-2026".to_string());
                // The actual validation happens in onboarding.rs handle_invite_validation
            }
        }
    }
}

// Additional systems for real text input can be added using bevy_ui or a text input plugin.
// The structure above provides a clean, professional foundation.
