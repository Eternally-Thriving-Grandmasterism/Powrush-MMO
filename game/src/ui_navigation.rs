/*!
 * Basic Controller / Gamepad UI Navigation System
 *
 * v3 - Refactored activation logic for clarity
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::input::gamepad::{GamepadButton, Gamepads};

/// Marker component for UI elements that can receive gamepad focus
#[derive(Component, Clone, Debug, Default)]
pub struct Focusable {
    pub order: i32,
}

/// Resource tracking the currently focused UI entity
#[derive(Resource, Default)]
pub struct UiFocus {
    pub current: Option<Entity>,
}

/// Handles D-pad and left stick navigation between focusable UI elements
pub fn gamepad_ui_navigation(
    gamepads: Res<Gamepads>,
    axes: Res<bevy::input::gamepad::GamepadAxis>,
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    mut focus: ResMut<UiFocus>,
    focusables: Query<(Entity, &Focusable, &GlobalTransform)>,
    time: Res<Time>,
) {
    static mut LAST_NAV_TIME: f32 = 0.0;
    let current_time = time.elapsed_seconds();

    unsafe {
        if current_time - LAST_NAV_TIME < 0.2 {
            return;
        }
    }

    for _gamepad in gamepads.iter() {
        let up_pressed = buttons.just_pressed(GamepadButton::DPadUp);
        let down_pressed = buttons.just_pressed(GamepadButton::DPadDown);
        let stick_y = axes.get(bevy::input::gamepad::GamepadAxis::LeftStickY).unwrap_or(0.0);

        let mut moved = false;

        if up_pressed || stick_y > 0.6 {
            move_focus(&mut focus, &focusables, true);
            moved = true;
        } else if down_pressed || stick_y < -0.6 {
            move_focus(&mut focus, &focusables, false);
            moved = true;
        }

        if moved {
            unsafe { LAST_NAV_TIME = current_time; }
            break;
        }
    }
}

/// Moves focus to the next or previous focusable element
fn move_focus(
    focus: &mut UiFocus,
    focusables: &Query<(Entity, &Focusable, &GlobalTransform)>,
    move_up: bool,
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
        if let Some(current_index) = candidates.iter().position(|(e, _)| *e == current) {
            let new_index = if move_up {
                if current_index == 0 { candidates.len() - 1 } else { current_index - 1 }
            } else {
                if current_index + 1 >= candidates.len() { 0 } else { current_index + 1 }
            };
            focus.current = Some(candidates[new_index].0);
            return;
        }
    }

    // Default to the first focusable element
    focus.current = Some(candidates[0].0);
}

/// Provides basic visual feedback for the currently focused element
pub fn highlight_focused_ui(
    focus: Res<UiFocus>,
    mut query: Query<(&Focusable, &mut BackgroundColor)>,
) {
    for (focusable, mut color) in query.iter_mut() {
        if focus.current == Some(focusable) {
            *color = Color::srgb(0.3, 0.6, 1.0).into(); // Highlighted
        } else {
            *color = Color::srgb(0.2, 0.2, 0.2).into(); // Default
        }
    }
}

/// Activates the currently focused button when the South gamepad button is pressed.
/// This simulates a mouse click by setting Interaction::Pressed.
pub fn activate_focused_button(
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    focus: Res<UiFocus>,
    mut interaction_query: Query<(&Focusable, &mut Interaction)>,
) {
    // Only act if we have a currently focused entity
    let Some(focused_entity) = focus.current else {
        return;
    };

    // Check if the focused entity has an Interaction component (i.e., it's a button)
    let Ok((_, mut interaction)) = interaction_query.get_mut(focused_entity) else {
        return;
    };

    // South button = A (Xbox) / Cross (PlayStation)
    if buttons.just_pressed(GamepadButton::South) {
        *interaction = Interaction::Pressed;
    }

    if buttons.just_released(GamepadButton::South) {
        *interaction = Interaction::None;
    }
}

/// Plugin that registers all UI navigation systems
pub struct UiNavigationPlugin;

impl Plugin for UiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFocus>()
            .add_systems(Update, gamepad_ui_navigation)
            .add_systems(Update, highlight_focused_ui)
            .add_systems(Update, activate_focused_button);
    }
}
