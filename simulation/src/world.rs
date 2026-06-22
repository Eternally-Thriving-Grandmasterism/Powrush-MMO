/*!
 * Button highlight logic for reactive preset UI.
 */

use bevy::prelude::*;

#[derive(Component)]
pub struct PresetButton {
    pub preset: LissajousKnotPreset,
}

// System that highlights the currently active preset button
pub fn highlight_active_preset_button(
    current: Res<CurrentLissajousKnotPreset>,
    mut buttons: Query<(&PresetButton, &mut BackgroundColor)>,
) {
    if current.is_changed() {
        for (button, mut bg) in &mut buttons {
            if button.preset == current.preset {
                *bg = Color::srgb(0.25, 0.35, 0.55).into(); // Highlighted
            } else {
                *bg = Color::srgb(0.15, 0.15, 0.22).into(); // Normal
            }
        }
    }
}

// When spawning buttons, attach the PresetButton component:
// parent.spawn((
//     ButtonBundle { ... },
//     PresetButton { preset },
// ))
// .observe(...);
