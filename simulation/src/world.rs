/*!
 * Bevy 0.14 compatible refactor of UI and event systems.
 */

use bevy::prelude::*;
use bevy::input::pointer::Pointer;

// Bevy 0.14 compatible button spawning with observers
pub fn spawn_preset_button(
    parent: &mut ChildBuilder,
    label: &str,
    preset: LissajousKnotPreset,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(180.0),
                    padding: UiRect::all(Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgb(0.15, 0.15, 0.22).into(),
                ..default()
            },
            PresetButton { preset },
        ))
        .observe(
            move |trigger: Trigger<Pointer<Click>>,
                  mut events: EventWriter<SwitchLissajousKnotPreset>| {
                events.send(SwitchLissajousKnotPreset { preset });
            },
        );
}

// The highlight and reactive systems remain compatible with 0.14
// as they use standard Res, Query, and Changed detection.
