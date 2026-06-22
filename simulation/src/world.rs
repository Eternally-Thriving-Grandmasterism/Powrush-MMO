/*!
 * Debug console + UI panel for Lissajous knot preset switching.
 */

use bevy::prelude::*;

// Keyboard debug controls (1-4 keys)
pub fn debug_lissajous_knot_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<SwitchLissajousKnotPreset>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        events.send(SwitchLissajousKnotPreset { preset: LissajousKnotPreset::TrefoilLike });
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        events.send(SwitchLissajousKnotPreset { preset: LissajousKnotPreset::HighWrithe });
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        events.send(SwitchLissajousKnotPreset { preset: LissajousKnotPreset::Symmetric });
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        events.send(SwitchLissajousKnotPreset { preset: LissajousKnotPreset::Complex5_3_4 });
    }
}

// Simple Bevy UI panel for preset switching
pub fn lissajous_knot_ui_panel(
    mut commands: Commands,
    mut events: EventWriter<SwitchLissajousKnotPreset>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Lissajous Knot Presets",
            TextStyle { font_size: 16.0, color: Color::WHITE, ..default() },
        ));

        for (label, preset) in [
            ("1. Trefoil-like", LissajousKnotPreset::TrefoilLike),
            ("2. High Writhe", LissajousKnotPreset::HighWrithe),
            ("3. Symmetric", LissajousKnotPreset::Symmetric),
            ("4. Complex 5:3:4", LissajousKnotPreset::Complex5_3_4),
        ] {
            parent.spawn(ButtonBundle {
                style: Style { margin: UiRect::all(Val::Px(4.0)), ..default() },
                background_color: Color::srgb(0.2, 0.2, 0.3).into(),
                ..default()
            }).with_children(|btn| {
                btn.spawn(TextBundle::from_section(label, TextStyle { font_size: 14.0, ..default() }));
            }).observe(move |_: Trigger<Pointer<Click>>, mut ev: EventWriter<SwitchLissajousKnotPreset>| {
                ev.send(SwitchLissajousKnotPreset { preset });
            });
        }
    });
}
