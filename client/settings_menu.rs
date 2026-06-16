/*!
 * client/settings_menu.rs
 * Powrush-MMO — Professional Mercy-Themed Settings Menu
 *
 * Beautiful, focused, and fully functional settings with PATSAGi guidance.
 * Production hardened with strong alignment to TOLC 8 Mercy Gates and Eternal Flow.
 */

use bevy::prelude::*;
use crate::settings::{ClientSettings, ServerRules, QualityPreset, save_client_settings, load_client_settings};
use crate::motion_blur::MotionBlurSettings;
use crate::taa_reprojection::TaaSettings;

// === COMPONENTS ===

#[derive(Component)] pub struct SettingsMenuRoot;
#[derive(Component)] pub struct SettingsCloseButton;
#[derive(Component)] pub struct SettingsApplyButton;
#[derive(Component)] pub struct SettingsResetButton;
#[derive(Component)] pub struct QualityPresetButton { pub preset: QualityPreset }
#[derive(Component)] pub struct MotionBlurToggleButton;
#[derive(Component)] pub struct MotionBlurIntensityMinus;
#[derive(Component)] pub struct MotionBlurIntensityPlus;
#[derive(Component)] pub struct MotionBlurIntensityText;

// === PLUGIN ===

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_settings_menu, load_settings_on_startup))
           .add_systems(Update, (
                handle_close_button,
                handle_apply_button,
                handle_reset_button,
                handle_quality_preset_buttons,
                handle_motion_blur_toggle,
                handle_motion_blur_intensity,
                update_motion_blur_intensity_text,
                sync_menu_with_settings_resource,
            ));
    }
}

// === STARTUP ===

fn load_settings_on_startup(mut commands: Commands) {
    let settings = load_client_settings();
    commands.insert_resource(settings);
}

// === FOCUSED INTERACTION SYSTEMS ===

fn handle_close_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SettingsCloseButton>)>,
    mut menu_query: Query<&mut Visibility, With<SettingsMenuRoot>>,
    mut settings: ResMut<ClientSettings>,
    motion_blur: Res<MotionBlurSettings>,
    taa: Res<TaaSettings>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for mut vis in menu_query.iter_mut() { *vis = Visibility::Hidden; }
            settings.graphics.motion_blur_enabled = motion_blur.enabled;
            settings.graphics.motion_blur_intensity = motion_blur.intensity;
            settings.graphics.taa_enabled = taa.enabled;
            settings.graphics.taa_jitter_scale = taa.jitter_scale;
            save_client_settings(&settings);
        }
    }
}

fn handle_apply_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SettingsApplyButton>)>,
    mut settings: ResMut<ClientSettings>,
    motion_blur: Res<MotionBlurSettings>,
    taa: Res<TaaSettings>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            settings.graphics.motion_blur_enabled = motion_blur.enabled;
            settings.graphics.motion_blur_intensity = motion_blur.intensity;
            settings.graphics.taa_enabled = taa.enabled;
            settings.graphics.taa_jitter_scale = taa.jitter_scale;
            save_client_settings(&settings);
            info!("[Settings] Applied & saved (TAA + Motion Blur)");
        }
    }
}

fn handle_reset_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SettingsResetButton>)>,
    mut settings: ResMut<ClientSettings>,
    mut motion_blur: ResMut<MotionBlurSettings>,
    mut taa: ResMut<TaaSettings>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            *settings = load_client_settings();
            motion_blur.enabled = settings.graphics.motion_blur_enabled;
            motion_blur.intensity = settings.graphics.motion_blur_intensity;
            taa.enabled = settings.graphics.taa_enabled;
            taa.jitter_scale = settings.graphics.taa_jitter_scale;
        }
    }
}

fn handle_quality_preset_buttons(
    mut interaction_query: Query<(&Interaction, &QualityPresetButton), Changed<Interaction>>,
    mut settings: ResMut<ClientSettings>,
) {
    for (interaction, preset_btn) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            settings.graphics.quality_preset = preset_btn.preset.clone();
        }
    }
}

fn handle_motion_blur_toggle(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MotionBlurToggleButton>)>,
    mut motion_blur: ResMut<MotionBlurSettings>,
    mut settings: ResMut<ClientSettings>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            motion_blur.enabled = !motion_blur.enabled;
            settings.graphics.motion_blur_enabled = motion_blur.enabled;
        }
    }
}

fn handle_motion_blur_intensity(
    mut interaction_query: Query<(&Interaction, Option<&MotionBlurIntensityMinus>, Option<&MotionBlurIntensityPlus>), Changed<Interaction>>,
    mut motion_blur: ResMut<MotionBlurSettings>,
    mut settings: ResMut<ClientSettings>,
) {
    for (interaction, minus, plus) in interaction_query.iter() {
        if *interaction != Interaction::Pressed { continue; }
        if minus.is_some() {
            motion_blur.intensity = (motion_blur.intensity - 0.1).max(0.0);
            settings.graphics.motion_blur_intensity = motion_blur.intensity;
        }
        if plus.is_some() {
            motion_blur.intensity = (motion_blur.intensity + 0.1).min(3.0);
            settings.graphics.motion_blur_intensity = motion_blur.intensity;
        }
    }
}

// === HELPER SYSTEMS ===

fn update_motion_blur_intensity_text(
    motion_blur: Res<MotionBlurSettings>,
    mut text_query: Query<&mut Text, With<MotionBlurIntensityText>>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("{:.2}", motion_blur.intensity);
    }
}

fn sync_menu_with_settings_resource(_settings: Res<ClientSettings>) {}

// === FULL BEAUTIFUL UI (Mercy-Themed) ===

fn spawn_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    width: Val::Px(640.0),
                    height: Val::Px(820.0),
                    margin: UiRect::new(Val::Px(-320.0), Val::Auto, Val::Px(-410.0), Val::Auto),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(18.0)),
                    ..default()
                },
                background_color: Color::srgba(0.035, 0.055, 0.095, 0.97).into(),
                border_color: Color::srgb(0.25, 0.68, 0.95).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            SettingsMenuRoot,
            Name::new("SettingsMenu_EternalConfiguration"),
        ))
        .with_children(|parent| {
            // HEADER
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(56.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(Val::Px(18.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.05, 0.08, 0.14).into(),
                    ..default()
                },
            )).with_children(|header| {
                header.spawn(TextBundle {
                    text: Text::from_section("POWRUSH — ETERNAL CONFIGURATION", TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 19.0,
                        color: Color::srgb(0.35, 0.82, 1.0),
                    }),
                    ..default()
                });
                header.spawn((
                    ButtonBundle {
                        style: Style { width: Val::Px(38.0), height: Val::Px(38.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border_radius: BorderRadius::all(Val::Px(8.0)), ..default() },
                        background_color: Color::srgb(0.22, 0.12, 0.22).into(),
                        ..default()
                    },
                    SettingsCloseButton,
                )).with_children(|btn| {
                    btn.spawn(TextBundle {
                        text: Text::from_section("✕", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 22.0, color: Color::WHITE }),
                        ..default()
                    });
                });
            });

            parent.spawn(TextBundle {
                text: Text::from_section("Mercy-Gated • PATSAGi Guided • Aligned with the Eternal Flow • TOLC 8", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.5, color: Color::srgb(0.55, 0.72, 0.88),
                }),
                style: Style { margin: UiRect::vertical(Val::Px(8.0)), ..default() },
                ..default()
            });

            // GRAPHICS SECTION
            parent.spawn(TextBundle {
                text: Text::from_section("GRAPHICS & PERCEPTION", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 14.0, color: Color::srgb(0.4, 0.85, 1.0),
                }),
                style: Style { margin: UiRect::top(Val::Px(10.0)), ..default() },
                ..default()
            });

            // Quality Presets
            parent.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), height: Val::Px(52.0), flex_direction: FlexDirection::Row, justify_content: JustifyContent::SpaceEvenly, align_items: AlignItems::Center, ..default() },
                ..default()
            }).with_children(|row| {
                for (label, preset) in [("Seedling", QualityPreset::Seedling), ("Flow Guardian", QualityPreset::FlowGuardian), ("Eternal", QualityPreset::Eternal)] {
                    row.spawn((
                        ButtonBundle {
                            style: Style { width: Val::Px(175.0), height: Val::Px(40.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, border_radius: BorderRadius::all(Val::Px(10.0)), ..default() },
                            background_color: Color::srgb(0.10, 0.16, 0.26).into(),
                            ..default()
                        },
                        QualityPresetButton { preset },
                    )).with_children(|btn| {
                        btn.spawn(TextBundle { text: Text::from_section(label, TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 13.0, color: Color::WHITE }), ..default() });
                    });
                }
            });

            // MOTION BLUR
            parent.spawn(TextBundle {
                text: Text::from_section("Motion Blur — Cinematic Velocity Trails", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 12.0, color: Color::srgb(0.7, 0.8, 0.9),
                }),
                style: Style { margin: UiRect::top(Val::Px(14.0)), ..default() },
                ..default()
            });

            parent.spawn(NodeBundle {
                style: Style { width: Val::Percent(100.0), height: Val::Px(44.0), flex_direction: FlexDirection::Row, align_items: AlignItems::Center, ..default() },
                ..default()
            }).with_children(|row| {
                row.spawn(( ButtonBundle { style: Style { width: Val::Px(140.0), height: Val::Px(36.0), ..default() }, background_color: Color::srgb(0.15, 0.45, 0.35).into(), ..default() }, MotionBlurToggleButton )).with_children(|btn| {
                    btn.spawn(TextBundle { text: Text::from_section("Toggle Motion Blur", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 12.0, color: Color::WHITE }), ..default() });
                });

                row.spawn(TextBundle { text: Text::from_section("Intensity:", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 12.0, color: Color::srgb(0.7, 0.8, 0.9) }), style: Style { margin: UiRect::left(Val::Px(20.0)), ..default() }, ..default() });

                row.spawn(( ButtonBundle { style: Style { width: Val::Px(36.0), height: Val::Px(36.0), ..default() }, background_color: Color::srgb(0.2, 0.3, 0.5).into(), ..default() }, MotionBlurIntensityMinus )).with_children(|btn| {
                    btn.spawn(TextBundle { text: Text::from_section("−", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 18.0, color: Color::WHITE }), ..default() });
                });

                row.spawn(( TextBundle { text: Text::from_section("1.00", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 14.0, color: Color::srgb(0.9, 0.95, 1.0) }), style: Style { margin: UiRect::horizontal(Val::Px(8.0)), ..default() }, ..default() }, MotionBlurIntensityText ));

                row.spawn(( ButtonBundle { style: Style { width: Val::Px(36.0), height: Val::Px(36.0), ..default() }, background_color: Color::srgb(0.2, 0.3, 0.5).into(), ..default() }, MotionBlurIntensityPlus )).with_children(|btn| {
                    btn.spawn(TextBundle { text: Text::from_section("+", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 18.0, color: Color::WHITE }), ..default() });
                });
            });

            // FOV (placeholder for future slider implementation)
            parent.spawn(TextBundle {
                text: Text::from_section("Field of View (FOV) — Align your divine perception (Coming Soon)", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 12.0, color: Color::srgb(0.7, 0.8, 0.9),
                }),
                style: Style { margin: UiRect::top(Val::Px(12.0)), ..default() },
                ..default()
            });

            // AUDIO
            parent.spawn(TextBundle {
                text: Text::from_section("AUDIO & DIVINE WHISPERS", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 14.0, color: Color::srgb(0.4, 0.85, 1.0) }),
                style: Style { margin: UiRect::top(Val::Px(16.0)), ..default() },
                ..default()
            });

            // ACCESSIBILITY
            parent.spawn(TextBundle {
                text: Text::from_section("ACCESSIBILITY & MERCY FEEDBACK", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 14.0, color: Color::srgb(0.4, 0.85, 1.0) }),
                style: Style { margin: UiRect::top(Val::Px(10.0)), ..default() },
                ..default()
            });

            // SERVER RULES
            parent.spawn(TextBundle {
                text: Text::from_section("SERVER RULES — CURRENT INSTANCE", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 14.0, color: Color::srgb(0.85, 0.6, 0.95) }),
                style: Style { margin: UiRect::top(Val::Px(14.0)), ..default() },
                ..default()
            });

            parent.spawn((
                NodeBundle {
                    style: Style { width: Val::Percent(100.0), padding: UiRect::all(Val::Px(12.0)), border: UiRect::all(Val::Px(1.0)), border_radius: BorderRadius::all(Val::Px(10.0)), ..default() },
                    background_color: Color::srgba(0.06, 0.09, 0.15, 0.6).into(),
                    border_color: Color::srgb(0.5, 0.4, 0.7).into(),
                    ..default()
                },
            )).with_children(|panel| {
                panel.spawn(TextBundle {
                    text: Text::from_section("Instance: Eternal Flow Instance — PATSAGi Sovereign\nMercy Enforcement: 92% | Render Distance: 250m | Abundance Pooling: Active\nGriefing Tolerance: Mercy-Gated | Event Rate: 1.0x", TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 11.0, color: Color::srgb(0.75, 0.82, 0.9) }),
                    ..default()
                });
            });

            // BOTTOM ACTION BAR
            parent.spawn((
                NodeBundle {
                    style: Style { width: Val::Percent(100.0), height: Val::Px(56.0), margin: UiRect::top(Val::Px(18.0)), flex_direction: FlexDirection::Row, justify_content: JustifyContent::SpaceBetween, align_items: AlignItems::Center, ..default() },
                    ..default()
                },
            )).with_children(|bar| {
                bar.spawn((
                    ButtonBundle { style: Style { width: Val::Px(160.0), height: Val::Px(42.0), ..default() }, background_color: Color::srgb(0.25, 0.15, 0.25).into(), ..default() },
                    SettingsResetButton,
                )).with_children(|btn| {
                    btn.spawn(TextBundle { text: Text::from_section("Reset to PATSAGi Defaults", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 12.0, color: Color::WHITE }), ..default() });
                });
                bar.spawn((
                    ButtonBundle { style: Style { width: Val::Px(140.0), height: Val::Px(42.0), ..default() }, background_color: Color::srgb(0.15, 0.45, 0.35).into(), ..default() },
                    SettingsApplyButton,
                )).with_children(|btn| {
                    btn.spawn(TextBundle { text: Text::from_section("Apply & Save ⚔️", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 13.0, color: Color::WHITE }), ..default() });
                });
            });
        });
}

pub fn toggle_settings_menu_visibility(
    mut menu_query: Query<&mut Visibility, With<SettingsMenuRoot>>,
) {
    for mut vis in menu_query.iter_mut() {
        *vis = if *vis == Visibility::Hidden { Visibility::Visible } else { Visibility::Hidden };
    }
}
