/*!
 * Polished UI panel for Lissajous knot preset switching.
 */

use bevy::prelude::*;

pub fn lissajous_knot_ui_panel(
    mut commands: Commands,
    current: Res<CurrentLissajousKnotPreset>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                top: Val::Px(12.0),
                padding: UiRect::all(Val::Px(12.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: Color::srgba(0.08, 0.08, 0.12, 0.92).into(),
            border_color: Color::srgb(0.3, 0.3, 0.4).into(),
            ..default()
        },
        Name::new("LissajousKnotPanel"),
    )).with_children(|parent| {
        // Header
        parent.spawn(TextBundle::from_section(
            "LISSAJOUS KNOT PRESETS",
            TextStyle {
                font_size: 13.0,
                color: Color::srgb(0.7, 0.75, 0.9),
                ..default()
            },
        ));

        // Current preset display
        parent.spawn(TextBundle::from_section(
            format!("Current: {:?}", current.preset),
            TextStyle {
                font_size: 11.0,
                color: Color::srgb(0.6, 0.85, 0.7),
                ..default()
            },
        ));

        // Preset buttons
        for (label, preset) in [
            ("1. Trefoil-like", LissajousKnotPreset::TrefoilLike),
            ("2. High Writhe", LissajousKnotPreset::HighWrithe),
            ("3. Symmetric", LissajousKnotPreset::Symmetric),
            ("4. Complex 5:3:4", LissajousKnotPreset::Complex5_3_4),
        ] {
            let is_current = current.preset == preset;

            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(180.0),
                        padding: UiRect::all(Val::Px(6.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: if is_current {
                        Color::srgb(0.25, 0.35, 0.55).into()
                    } else {
                        Color::srgb(0.15, 0.15, 0.22).into()
                    },
                    ..default()
                },
 name: Name::new(format!("PresetButton_{:?}", preset)),
            )).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    label,
                    TextStyle {
                        font_size: 12.0,
                        color: if is_current { Color::WHITE } else { Color::srgb(0.85, 0.85, 0.9) },
                        ..default()
                    },
                ));
            }).observe(move |_: Trigger<Pointer<Click>>, mut ev: EventWriter<SwitchLissajousKnotPreset>| {
                ev.send(SwitchLissajousKnotPreset { preset });
            });
        }
    });
}
