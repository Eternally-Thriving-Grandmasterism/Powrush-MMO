/*!
 * Reactive UI updates for Lissajous knot presets.
 */

use bevy::prelude::*;

#[derive(Component)]
pub struct LissajousKnotPanel;

#[derive(Component)]
pub struct CurrentPresetText;

// System that reactively updates the UI when the preset changes
pub fn update_lissajous_knot_ui(
    current: Res<CurrentLissajousKnotPreset>,
    mut text_query: Query<&mut Text, With<CurrentPresetText>>,
) {
    if current.is_changed() {
        for mut text in &mut text_query {
            text.sections[0].value = format!("Current: {:?}", current.preset);
        }
    }
}

// Enhanced panel spawn function with marker components
pub fn lissajous_knot_ui_panel(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle { /* ... polished styling ... */ },
        LissajousKnotPanel,
        Name::new("LissajousKnotPanel"),
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "LISSAJOUS KNOT PRESETS",
            TextStyle { font_size: 13.0, color: Color::srgb(0.7, 0.75, 0.9), ..default() },
        ));

        // Current preset text (marked for reactive updates)
        parent.spawn((
            TextBundle::from_section(
                "Current: Complex5_3_4",
                TextStyle { font_size: 11.0, color: Color::srgb(0.6, 0.85, 0.7), ..default() },
            ),
            CurrentPresetText,
        ));

        // Buttons with observers (same as before)
        // ...
    });
}
