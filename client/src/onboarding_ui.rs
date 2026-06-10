// client/src/onboarding_ui.rs
// Powrush-MMO v18.9 — Captcha UI for Invite Verification

use bevy::prelude::*;
use crate::onboarding::{OnboardingState, OnboardingStep};

#[derive(Component)]
pub struct InviteInputPanel;

#[derive(Component)]
pub struct InviteInputField;

#[derive(Component)]
pub struct InviteStatusText;

#[derive(Component)]
pub struct CaptchaPanel;

#[derive(Component)]
pub struct CaptchaQuestionText;

#[derive(Component)]
pub struct CaptchaInputField;

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
                update_captcha_ui_visibility,
                handle_captcha_input,
            ));
    }
}

fn spawn_onboarding_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ... existing UI code for invite panel ...

    // Captcha Panel (shown after successful invite code)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(520.0),
                height: Val::Px(260.0),
                margin: UiRect::new(Val::Px(-260.0), Val::Auto, Val::Px(-130.0), Val::Auto),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                visibility: Visibility::Hidden,
                ..default()
            },
            CaptchaPanel,
        },
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("VERIFY YOU ARE HUMAN", TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                color: Color::srgb(0.35, 0.82, 1.0),
            }),
            style: Style { margin: UiRect::bottom(Val::Px(12.0)), ..default() },
            ..default()
        });

        parent.spawn((TextBundle {
            text: Text::from_section("", TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 18.0,
                color: Color::WHITE,
            }),
            style: Style { margin: UiRect::bottom(Val::Px(16.0)), ..default() },
            ..default()
        }, CaptchaQuestionText));

        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(48.0),
                    border: UiRect::all(Val::Px(1.5)),
                    border_color: Color::srgb(0.4, 0.7, 0.95).into(),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                background_color: Color::srgb(0.08, 0.1, 0.15).into(),
                ..default()
            },
            CaptchaInputField,
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

fn update_captcha_ui_visibility(
    onboarding: Res<OnboardingState>,
    mut captcha_panel: Query<&mut Visibility, With<CaptchaPanel>>,
) {
    for mut vis in captcha_panel.iter_mut() {
        *vis = if onboarding.step == OnboardingStep::CaptchaVerification {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn handle_captcha_input(
    mut onboarding: ResMut<OnboardingState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut char_events: EventReader<ReceivedCharacter>,
) {
    if onboarding.step != OnboardingStep::CaptchaVerification {
        return;
    }

    for event in char_events.read() {
        if event.char.is_ascii_digit() || event.char == '-' {
            if onboarding.captcha_user_input.len() < 8 {
                onboarding.captcha_user_input.push(event.char);
            }
        }
    }

    if keyboard.just_pressed(KeyCode::Backspace) {
        onboarding.captcha_user_input.pop();
    }

    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::NumpadEnter) {
        // Verification happens in verify_captcha system
    }
}

// Other systems (update_invite_status_text, etc.) remain similar with minor adjustments for captcha step.
