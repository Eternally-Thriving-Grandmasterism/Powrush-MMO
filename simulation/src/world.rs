/*!
 * Performance-refactored button highlight logic.
 */

use bevy::prelude::*;

// Only runs when CurrentLissajousKnotPreset actually changes
pub fn highlight_active_preset_button(
    current: Res<CurrentLissajousKnotPreset>,
    mut buttons: Query<(&PresetButton, &mut BackgroundColor)>,
) {
    // Bevy will only schedule this system when the resource changes
    // thanks to the implicit change detection on Res<T>
    for (button, mut bg) in &mut buttons {
        let is_active = button.preset == current.preset;
        let target_color: Color = if is_active {
            Color::srgb(0.25, 0.35, 0.55)
        } else {
            Color::srgb(0.15, 0.15, 0.22)
        };

        // Only write if the color actually needs to change (avoids unnecessary GPU updates)
        if bg.0 != target_color {
            *bg = target_color.into();
        }
    }
}
