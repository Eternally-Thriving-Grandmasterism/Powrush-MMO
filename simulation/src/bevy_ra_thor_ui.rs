/*!
 * Bevy UI Components for Ra-Thor Access Level
 *
 * Provides visual representation of a player's earned Ra-Thor access level.
 * This is the first step toward in-game UI for the earned privilege system.
 *
 * Usage:
 *   - Spawn a RaThorAccessLevelUI entity when opening player status / council screen.
 *   - Update it reactively when PlayerSaveData changes.
 */

use bevy::prelude::*;
use crate::bevy_integration::RaThorResource;
use crate::player_persistence::data::PlayerSaveData;
use crate::ra_thor_bridge::{calculate_ra_thor_access_level, RaThorAccessLevel};

/// Marker component for the Ra-Thor Access Level UI element.
#[derive(Component)]
pub struct RaThorAccessLevelUI;

/// Bundle for a simple text-based Ra-Thor access level display.
#[derive(Bundle)]
pub struct RaThorAccessLevelUIBundle {
    pub marker: RaThorAccessLevelUI,
    pub text: Text,
    pub text_font: TextFont,
    pub text_color: TextColor,
    pub node: Node,
}

impl Default for RaThorAccessLevelUIBundle {
    fn default() -> Self {
        Self {
            marker: RaThorAccessLevelUI,
            text: Text::new("Ra-Thor Access: None"),
            text_font: TextFont {
                font_size: 18.0,
                ..default()
            },
            text_color: TextColor(Color::srgb(0.6, 0.6, 0.6)), // Gray for None
            node: Node {
                margin: UiRect::all(Val::Px(8.0)),
                ..default()
            },
        }
    }
}

/// System that updates the Ra-Thor access level UI text and color.
/// Call this in your UI systems when PlayerSaveData changes.
pub fn update_ra_thor_access_level_ui(
    ra_thor: Res<RaThorResource>,
    player_data: Res<PlayerSaveData>,
    mut query: Query<(&mut Text, &mut TextColor), With<RaThorAccessLevelUI>>,
) {
    let access_level = calculate_ra_thor_access_level(&player_data);

    for (mut text, mut color) in query.iter_mut() {
        let (label, new_color) = match access_level {
            RaThorAccessLevel::None => ("Ra-Thor Access: None", Color::srgb(0.5, 0.5, 0.5)),
            RaThorAccessLevel::Lite => ("Ra-Thor Access: Lite", Color::srgb(0.3, 0.6, 1.0)), // Blue
            RaThorAccessLevel::Full => ("Ra-Thor Access: Full", Color::srgb(1.0, 0.85, 0.2)), // Gold
        };

        text.0 = label.to_string();
        color.0 = new_color;
    }
}

/// Helper function to spawn a basic Ra-Thor access level UI element.
/// You can expand this into a full panel with progress toward next tier.
pub fn spawn_ra_thor_access_level_ui(commands: &mut Commands) {
    commands.spawn(RaThorAccessLevelUIBundle::default());
}

/*
 * Future enhancements:
 * - Progress bar toward next access level
 * - Tooltip explaining how to earn higher access
 * - Special effects / animations when access level increases
 * - Integration with Divine Whispers when access is gained
 */
