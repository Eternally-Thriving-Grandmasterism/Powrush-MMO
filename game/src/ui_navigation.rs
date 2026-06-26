/*!
 * Basic Controller / Gamepad UI Navigation System
 *
 * Provides focusable UI elements and D-pad / analog stick navigation.
 * Designed to be expanded for full menu navigation on Steam Deck.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::input::gamepad::{Gamepad, GamepadAxis, GamepadButton};

/// Marker component for UI elements that can be focused with a controller
#[derive(Component, Clone, Debug, Default)]
pub struct Focusable {
    pub order: i32, // Used for ordering focusable elements
}

/// Resource that tracks the currently focused UI entity
#[derive(Resource, Default)]
pub struct UiFocus {
    pub current: Option<Entity>,
}

/// System that handles gamepad input for UI navigation
pub fn gamepad_ui_navigation(
    gamepads: Res<bevy::input::gamepad::Gamepads>,
    axes: Res<bevy::input::gamepad::GamepadAxis>,
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    mut focus: ResMut<UiFocus>,
    focusables: Query<(Entity, &Focusable, &GlobalTransform)>,
    time: Res<Time>,
) {
    // Simple cooldown to prevent overly fast navigation
    static mut LAST_NAV_TIME: f32 = 0.0;
    let current_time = time.elapsed_seconds();

    unsafe {
        if current_time - LAST_NAV_TIME < 0.2 {
            return;
        }
    }

    for gamepad in gamepads.iter() {
        // D-pad navigation
        let up = buttons.just_pressed(GamepadButton::DPadUp);
        let down = buttons.just_pressed(GamepadButton::DPadDown);
        let left = buttons.just_pressed(GamepadButton::DPadLeft);
        let right = buttons.just_pressed(GamepadButton::DPadRight);

        // Left stick navigation (simple threshold)
        let stick_y = axes.get(GamepadAxis::LeftStickY).unwrap_or(0.0);
        let stick_x = axes.get(GamepadAxis::LeftStickX).unwrap_or(0.0);

        let mut moved = false;

        if up || stick_y > 0.6 {
            move_focus(&mut focus, &focusables, true);
            moved = true;
        } else if down || stick_y < -0.6 {
            move_focus(&mut focus, &focusables, false);
            moved = true;
        }

        if moved {
            unsafe { LAST_NAV_TIME = current_time; }
            break;
        }
    }
}

fn move_focus(
    focus: &mut UiFocus,
    focusables: &Query<(Entity, &Focusable, &GlobalTransform)>,
    up: bool,
) {
    let mut candidates: Vec<(Entity, i32)> = focusables
        .iter()
        .map(|(entity, focusable, _)| (entity, focusable.order))
        .collect();

    if candidates.is_empty() {
        return;
    }

    candidates.sort_by_key(|(_, order)| *order);

    if let Some(current) = focus.current {
        if let Some(pos) = candidates.iter().position(|(e, _)| *e == current) {
            let new_pos = if up {
                if pos == 0 { candidates.len() - 1 } else { pos - 1 }
            } else {
                if pos + 1 >= candidates.len() { 0 } else { pos + 1 }
            };
            focus.current = Some(candidates[new_pos].0);
            return;
        }
    }

    // Default to first element
    focus.current = Some(candidates[0].0);
}

/// Visual feedback for focused UI element (simple example)
pub fn highlight_focused_ui(
    focus: Res<UiFocus>,
    mut query: Query<(&Focusable, &mut BackgroundColor)>,
) {
    for (focusable, mut color) in query.iter_mut() {
        if Some(focusable) == focus.current.as_ref() {
            *color = Color::srgb(0.3, 0.6, 1.0).into(); // Highlight color
        } else {
            *color = Color::srgb(0.2, 0.2, 0.2).into(); // Default
        }
    }
}

/// Plugin to register UI navigation systems
pub struct UiNavigationPlugin;

impl Plugin for UiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFocus>()
            .add_systems(Update, gamepad_ui_navigation)
            .add_systems(Update, highlight_focused_ui);
    }
}
