/*!
 * Settings Editor - Polish: Sound + Visual Pop + Haptic
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::settings::editor::SettingsEditor;
use crate::settings::ui::SliderBar;

/// Plays a small UI sound when a setting value changes
pub fn play_value_change_sound(
    editor: Option<Res<SettingsEditor>>,
    mut last_dirty: Local<bool>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Some(editor) = editor {
        if editor.dirty && !*last_dirty {
            // Play a short UI click sound
            let sound = asset_server.load("audio/ui_click.ogg");
            audio.play(sound);
        }
        *last_dirty = editor.dirty;
    } else {
        *last_dirty = false;
    }
}

/// Quick visual "pop" animation on slider bars when value changes
#[derive(Component)]
pub struct SliderPopAnimation {
    pub timer: f32,
}

pub fn animate_slider_bars(
    mut commands: Commands,
    mut query: Query<(Entity, &SliderBar, &mut Transform, Option<&mut SliderPopAnimation>)>,
    time: Res<Time>,
) {
    for (entity, _bar, mut transform, pop) in query.iter_mut() {
        if let Some(mut animation) = pop {
            animation.timer -= time.delta_seconds();
            let scale = 1.0 + (animation.timer * 4.0).sin() * 0.15;
            transform.scale = Vec3::splat(scale.max(1.0));

            if animation.timer <= 0.0 {
                transform.scale = Vec3::ONE;
                commands.entity(entity).remove::<SliderPopAnimation>();
            }
        }
    }
}

/// Trigger a quick pop animation on slider bars when value changes
pub fn trigger_slider_pop(
    mut commands: Commands,
    editor: Option<Res<SettingsEditor>>,
    mut last_dirty: Local<bool>,
    query: Query<(Entity, &SliderBar)>,
) {
    if let Some(editor) = editor {
        if editor.dirty && !*last_dirty {
            for (entity, _bar) in query.iter() {
                commands.entity(entity).insert(SliderPopAnimation { timer: 0.25 });
            }
        }
        *last_dirty = editor.dirty;
    } else {
        *last_dirty = false;
    }
}

/// Basic haptic feedback (vibration) on value change
pub fn trigger_haptic_feedback(
    mut commands: Commands,
    editor: Option<Res<SettingsEditor>>,
    mut last_dirty: Local<bool>,
    gamepads: Res<bevy::input::gamepad::Gamepads>,
) {
    if let Some(editor) = editor {
        if editor.dirty && !*last_dirty {
            for gamepad in gamepads.iter() {
                // Light short vibration
                commands.entity(gamepad).insert(bevy::input::gamepad::GamepadRumbleRequest {
                    duration: std::time::Duration::from_millis(80),
                    intensity: 0.4,
                });
            }
        }
        *last_dirty = editor.dirty;
    } else {
        *last_dirty = false;
    }
}
