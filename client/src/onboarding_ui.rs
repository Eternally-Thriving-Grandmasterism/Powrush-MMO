/*!
 * Onboarding UI — Powrush-MMO Educational Panels + RBE Integration + Prior Council Reflection + War Context
 *
 * v21.90 — Restored full invite UI systems (were previously empty stubs).
 * Public path remains frictionless; beta path is now fully interactive.
 * TOLC 8 + PATSAGi + Mercy Gates aligned.
 *
 * AG-SML v1.0 Sovereign License | Contact: info@Rathor.ai
 * Thunder locked in. Yoi ⚡
 */

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

#[derive(Component)]
pub struct RBEducationPanel;

#[derive(Component)]
pub struct RBEducationTitle;

#[derive(Component)]
pub struct RBEducationBody;

#[derive(Component)]
pub struct PriorCouncilReflectionPanel;

#[derive(Component)]
pub struct WarContextPanel;

#[derive(Resource, Default)]
pub struct InviteInputState {
    pub current_text: String,
    pub is_focused: bool,
}

#[derive(Resource, Default)]
pub struct CurrentRealmTension {
    pub realm_id: u8,
    pub tension_score: f32,
    pub in_active_war: bool,
    pub linked_legacy_thread: Option<u64>,
    pub war_summary: String,
}

pub struct OnboardingUIPlugin;

impl Plugin for OnboardingUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InviteInputState>()
            .init_resource::<CurrentRealmTension>()
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
                update_rbe_education_panel,
                update_prior_council_reflection_panel,
                update_war_context_panel,
            ));
    }
}

fn spawn_onboarding_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Invite Panel
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(35.0),
                width: Val::Px(480.0),
                height: Val::Px(320.0),
                margin: UiRect::new(Val::Px(-240.0), Val::Auto, Val::Auto, Val::Auto),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(18.0)),
                ..default()
            },
            visibility: Visibility::Hidden,
            background_color: Color::srgba(0.06, 0.08, 0.12, 0.96).into(),
            border_color: Color::srgb(0.4, 0.7, 0.95).into(),
            ..default()
        },
        InviteInputPanel,
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
                    height: Val::Px(48.0),
                    border: UiRect::all(Val::Px(1.5)),
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                background_color: Color::srgb(0.08, 0.1, 0.15).into(),
                border_color: Color::srgb(0.4, 0.7, 0.95).into(),
                ..default()
            },
            InviteInputField,
        )).with_children(|field| {
            field.spawn(TextBundle {
                text: Text::from_section("", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                    font_size: 18.0,
                    color: Color::srgb(0.9, 0.93, 1.0),
                }),
                ..default()
            });
        });

        parent.spawn((TextBundle {
            text: Text::from_section("Type code · Enter to submit", TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 14.0,
                color: Color::srgb(0.85, 0.85, 0.9),
            }),
            style: Style { margin: UiRect::top(Val::Px(12.0)), ..default() },
            ..default()
        }, InviteStatusText));
    });

    // Captcha Panel
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
                ..default()
            },
            visibility: Visibility::Hidden,
            background_color: Color::srgba(0.06, 0.08, 0.12, 0.96).into(),
            border_color: Color::srgb(0.4, 0.7, 0.95).into(),
            ..default()
        },
        CaptchaPanel,
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
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                },
                background_color: Color::srgb(0.08, 0.1, 0.15).into(),
                border_color: Color::srgb(0.4, 0.7, 0.95).into(),
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

    // RBE Education Panel
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(4.0),
                top: Val::Percent(18.0),
                width: Val::Px(380.0),
                min_height: Val::Px(220.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            },
            visibility: Visibility::Hidden,
            background_color: Color::srgba(0.06, 0.08, 0.12, 0.96).into(),
            border_color: Color::srgb(0.5, 0.75, 1.0).into(),
            ..default()
        },
        RBEducationPanel,
    )).with_children(|parent| {
        parent.spawn((TextBundle {
            text: Text::from_section("", TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 18.0,
                color: Color::srgb(0.6, 0.85, 1.0),
            }),
            style: Style { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
            ..default()
        }, RBEducationTitle));

        parent.spawn((TextBundle {
            text: Text::from_section("", TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 15.0,
                color: Color::srgb(0.92, 0.94, 0.98),
            }),
            style: Style { max_width: Val::Px(340.0), ..default() },
            ..default()
        }, RBEducationBody));
    });

    // Prior Council Reflection Panel
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(4.0),
                top: Val::Percent(42.0),
                width: Val::Px(380.0),
                min_height: Val::Px(80.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(14.0)),
                ..default()
            },
            visibility: Visibility::Hidden,
            background_color: Color::srgba(0.08, 0.06, 0.12, 0.95).into(),
            border_color: Color::srgb(0.85, 0.7, 1.0).into(),
            ..default()
        },
        PriorCouncilReflectionPanel,
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("", TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                color: Color::srgb(0.95, 0.85, 1.0),
            }),
            style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
            ..default()
        });

        parent.spawn(TextBundle {
            text: Text::from_section("", TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 14.0,
                color: Color::srgb(0.92, 0.94, 0.98),
            }),
            style: Style { max_width: Val::Px(340.0), ..default() },
            ..default()
        });
    });

    // War Context Panel
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(4.0),
                top: Val::Percent(58.0),
                width: Val::Px(380.0),
                min_height: Val::Px(110.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(14.0)),
                ..default()
            },
            visibility: Visibility::Hidden,
            background_color: Color::srgba(0.12, 0.08, 0.06, 0.95).into(),
            border_color: Color::srgb(0.95, 0.6, 0.4).into(),
            ..default()
        },
        WarContextPanel,
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section("REALM STATUS — LIVE", TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 15.0,
                color: Color::srgb(0.95, 0.7, 0.5),
            }),
            style: Style { margin: UiRect::bottom(Val::Px(6.0)), ..default() },
            ..default()
        });

        parent.spawn(TextBundle {
            text: Text::from_section("", TextStyle {
                font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                font_size: 13.0,
                color: Color::srgb(0.92, 0.9, 0.88),
            }),
            style: Style { max_width: Val::Px(360.0), ..default() },
            ..default()
        });
    });
}

fn update_war_context_panel(
    tension: Res<CurrentRealmTension>,
    mut panel_query: Query<&mut Visibility, With<WarContextPanel>>,
) {
    let show = tension.tension_score > 0.55 || tension.in_active_war;
    for mut vis in panel_query.iter_mut() {
        *vis = if show { Visibility::Visible } else { Visibility::Hidden };
    }
}

fn update_prior_council_reflection_panel(
    onboarding: Res<OnboardingState>,
    mut panel_query: Query<&mut Visibility, With<PriorCouncilReflectionPanel>>,
) {
    let has_prior = onboarding.prior_council_blooms > 0;
    for mut vis in panel_query.iter_mut() {
        *vis = if has_prior { Visibility::Visible } else { Visibility::Hidden };
    }
}

fn update_rbe_education_panel(
    onboarding: Res<OnboardingState>,
    mut panel_query: Query<&mut Visibility, With<RBEducationPanel>>,
    mut title_query: Query<&mut Text, With<RBEducationTitle>>,
    mut body_query: Query<&mut Text, With<RBEducationBody>>,
) {
    let show_panel = matches!(
        onboarding.step,
        OnboardingStep::RBEPrimer
            | OnboardingStep::MercyContribution
            | OnboardingStep::SovereignStart
            | OnboardingStep::FirstCouncilBloom
    );

    for mut vis in panel_query.iter_mut() {
        *vis = if show_panel { Visibility::Visible } else { Visibility::Hidden };
    }

    if !show_panel {
        return;
    }

    let (title, body) = match onboarding.step {
        OnboardingStep::RBEPrimer => (
            "The Lattice",
            "Everything is interconnected. Every choice ripples through the living web. What you nurture, nurtures all.",
        ),
        OnboardingStep::MercyContribution => (
            "Mercy as Multiplier",
            "Mercy is the true currency of the eternal Lattice. It multiplies when given. Every act of presence strengthens the whole web.",
        ),
        OnboardingStep::SovereignStart => (
            "Earned Abundance & Sovereignty",
            "Abundance without extraction is grown, not given. The Lattice reveals deeper gifts to those who hold them with mercy.",
        ),
        OnboardingStep::FirstCouncilBloom => (
            "Council as Living Governance",
            "When many align in mercy, something greater awakens. The Council is you, remembering how to move as one living body.",
        ),
        _ => ("", ""),
    };

    for mut title_text in title_query.iter_mut() {
        if let Some(s) = title_text.sections.get_mut(0) {
            s.value = title.to_string();
        }
    }
    for mut body_text in body_query.iter_mut() {
        if let Some(s) = body_text.sections.get_mut(0) {
            s.value = body.to_string();
        }
    }
}

fn update_captcha_ui_visibility(
    onboarding: Res<OnboardingState>,
    mut captcha_panel: Query<&mut Visibility, With<CaptchaPanel>>,
    mut question_q: Query<&mut Text, With<CaptchaQuestionText>>,
) {
    let show = onboarding.step == OnboardingStep::CaptchaVerification;
    for mut vis in captcha_panel.iter_mut() {
        *vis = if show { Visibility::Visible } else { Visibility::Hidden };
    }
    if show {
        if let Some(q) = &onboarding.captcha_question {
            for mut text in question_q.iter_mut() {
                if let Some(s) = text.sections.get_mut(0) {
                    s.value = q.clone();
                }
            }
        }
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
}

// ─── Restored invite UI systems (were empty stubs) ───────────────────────────

fn update_onboarding_step_panels(_onboarding: Res<OnboardingState>) {
    // Reserved for future step-specific ambient UI transitions.
}

fn update_invite_ui_visibility(
    onboarding: Res<OnboardingState>,
    mut panel: Query<&mut Visibility, With<InviteInputPanel>>,
) {
    let show = onboarding.step == OnboardingStep::InviteValidation;
    for mut vis in panel.iter_mut() {
        *vis = if show { Visibility::Visible } else { Visibility::Hidden };
    }
}

fn handle_invite_text_input(
    mut onboarding: ResMut<OnboardingState>,
    mut invite_input: ResMut<InviteInputState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut char_events: EventReader<ReceivedCharacter>,
) {
    if onboarding.step != OnboardingStep::InviteValidation {
        return;
    }

    for event in char_events.read() {
        let c = event.char;
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
            if invite_input.current_text.len() < 32 {
                invite_input.current_text.push(c.to_ascii_uppercase());
            }
        }
    }

    if keyboard.just_pressed(KeyCode::Backspace) {
        invite_input.current_text.pop();
    }

    // Mirror into onboarding state for validation systems
    if !invite_input.current_text.is_empty() {
        onboarding.invite_code = Some(invite_input.current_text.clone());
    }
}

fn handle_invite_submission(
    mut onboarding: ResMut<OnboardingState>,
    invite_input: Res<InviteInputState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if onboarding.step != OnboardingStep::InviteValidation {
        return;
    }
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::NumpadEnter) {
        if !invite_input.current_text.is_empty() {
            onboarding.invite_code = Some(invite_input.current_text.clone());
            // process_invite_validation in onboarding.rs will consume it next frame
        }
    }
}

fn update_invite_input_display(
    invite_input: Res<InviteInputState>,
    mut field_query: Query<&Children, With<InviteInputField>>,
    mut text_query: Query<&mut Text>,
) {
    for children in field_query.iter() {
        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                if let Some(s) = text.sections.get_mut(0) {
                    s.value = if invite_input.current_text.is_empty() {
                        "_".to_string()
                    } else {
                        invite_input.current_text.clone()
                    };
                }
            }
        }
    }
}

fn update_invite_status_text(
    onboarding: Res<OnboardingState>,
    mut status_query: Query<&mut Text, With<InviteStatusText>>,
) {
    for mut text in status_query.iter_mut() {
        if let Some(s) = text.sections.get_mut(0) {
            if let Some(err) = &onboarding.invite_error {
                s.value = err.clone();
                s.style.color = Color::srgb(1.0, 0.55, 0.45);
            } else if onboarding.invite_validated {
                s.value = "Invite accepted · welcome home".to_string();
                s.style.color = Color::srgb(0.55, 0.95, 0.7);
            } else {
                s.value = "Type code · Enter to submit".to_string();
                s.style.color = Color::srgb(0.85, 0.85, 0.9);
            }
        }
    }
}

// End of client/src/onboarding_ui.rs v21.90 — invite path fully interactive again.
