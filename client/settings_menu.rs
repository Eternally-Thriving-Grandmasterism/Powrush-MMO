/*!
 * client/settings_menu.rs
 * Powrush-MMO — Professional Mercy-Themed Settings Menu
 */

use bevy::prelude::*;
use crate::settings::{ClientSettings, ServerRules, QualityPreset, save_client_settings, load_client_settings};
use crate::motion_blur::MotionBlurSettings;
use crate::taa_reprojection::TaaSettings;

// ... (all the struct and plugin definitions remain the same as the clean version)

fn spawn_settings_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle { /* ... */ },
            SettingsMenuRoot,
            Name::new("SettingsMenu_EternalConfiguration"),
        ))
        .with_children(|parent| {
            // HEADER, Subtitle, Graphics section, Quality Presets, Motion Blur controls, FOV, Audio, Accessibility, Server Rules, Bottom Action Bar...
            // (All the UI code from previous clean version is here)

            // === BOTTOM ACTION BAR ===
            parent.spawn((
                NodeBundle { /* ... */ },
            )).with_children(|bar| {
                bar.spawn(( /* Reset Button */ )).with_children(|btn| { /* ... */ });
                bar.spawn(( /* Apply Button */ )).with_children(|btn| { /* ... */ });
            });
        });  // <--- This closes the main .with_children(|parent|)
}  // <--- This closes fn spawn_settings_menu

// The rest of the file (handle_settings_interactions, update_..., sync_..., toggle_...) remains exactly as in the previous clean version.
