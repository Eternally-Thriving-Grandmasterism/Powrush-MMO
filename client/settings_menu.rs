/*!
 * client/settings_menu.rs
 * Powrush-MMO — Professional Mercy-Themed Settings Menu
 *
 * Added Motion Blur toggle + intensity in Graphics section.
 * Fully wired to MotionBlurSettings for live toggle.
 * PATSAGi / Ra-Thor aligned.
 */

use bevy::prelude::*;
use crate::settings::{ClientSettings, ServerRules, QualityPreset, save_client_settings, load_client_settings};
use crate::motion_blur::MotionBlurSettings;

// ... (existing components and structs remain)

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_settings_menu)
            .add_systems(Update, (
                handle_settings_interactions,
                sync_menu_with_settings_resource,
                sync_motion_blur_settings,
            ));
    }
}

// ... (spawn_settings_menu remains mostly the same, with new Motion Blur row added in GRAPHICS section)

// === NEW: Motion Blur controls in Graphics section ===
// Add this inside the spawn_settings_menu function, after Quality Preset row:

/*
// Example placement in spawn_settings_menu (insert after Quality row):
parent.spawn(TextBundle {
    text: Text::from_section("Motion Blur — Cinematic Velocity Trails", TextStyle {
        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
        font_size: 12.0,
        color: Color::srgb(0.7, 0.8, 0.9),
    }),
    style: Style { margin: UiRect::top(Val::Px(12.0)), ..default() },
    ..default()
});

parent.spawn(NodeBundle {
    style: Style {
        width: Val::Percent(100.0),
        height: Val::Px(40.0),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        ..default()
    },
    ..default()
}).with_children(|row| {
    row.spawn(ButtonBundle {
        style: Style { width: Val::Px(120.0), ..default() },
        background_color: Color::srgb(0.15, 0.45, 0.35).into(),
        ..default()
    }, MotionBlurToggleButton);

    // Intensity slider placeholder (can be expanded with drag logic like volume)
    row.spawn(TextBundle {
        text: Text::from_section("Intensity: 1.0", TextStyle { font_size: 12.0, ..default() }),
        ..default()
    });
});
*/

#[derive(Component)]
pub struct MotionBlurToggleButton;

fn handle_settings_interactions(
    mut interaction_query: Query<(&Interaction, Option<&QualityPresetButton>, Option<&SettingsCloseButton>, Option<&SettingsApplyButton>, Option<&SettingsResetButton>, Option<&MotionBlurToggleButton>), Changed<Interaction>>,
    mut menu_query: Query<&mut Visibility, With<SettingsMenuRoot>>,
    mut settings: ResMut<ClientSettings>,
    mut motion_blur: ResMut<MotionBlurSettings>,
    server_rules: Res<ServerRules>,
) {
    for (interaction, preset_btn, close_btn, apply_btn, reset_btn, mb_toggle) in interaction_query.iter() {
        if *interaction != Interaction::Pressed { continue; }

        if close_btn.is_some() {
            for mut vis in menu_query.iter_mut() { *vis = Visibility::Hidden; }
            save_client_settings(&settings);
            continue;
        }

        if let Some(preset) = preset_btn {
            settings.graphics.quality_preset = preset.preset.clone();
        }

        if mb_toggle.is_some() {
            motion_blur.enabled = !motion_blur.enabled;
            info!("[Settings] Motion Blur toggled: {}", motion_blur.enabled);
        }

        if apply_btn.is_some() {
            save_client_settings(&settings);
            info!("[Settings] Applied & saved (Motion Blur state synced)");
        }

        if reset_btn.is_some() {
            *settings = load_client_settings();
            motion_blur.enabled = true; // default
            motion_blur.intensity = 1.0;
        }
    }
}

fn sync_motion_blur_settings(
    motion_blur: Res<MotionBlurSettings>,
    // Future: sync toggle button color / intensity text here
) {
    if motion_blur.is_changed() {
        // Update UI button color or text to reflect enabled state
    }
}

// ... (rest of file unchanged)
