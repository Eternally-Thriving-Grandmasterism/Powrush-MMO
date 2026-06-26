/*!
 * Basic Controller / Gamepad UI Navigation System
 *
 * v4 - Added horizontal navigation support
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

/// Handles D-pad and analog stick navigation (both vertical and horizontal)
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
        // Vertical input
        let up = buttons.just_pressed(GamepadButton::DPadUp);
        let down = buttons.just_pressed(GamepadButton::DPadDown);
        let stick_y = axes.get(bevy::input::gamepad::GamepadAxis::LeftStickY).unwrap_or(0.0);

        // Horizontal input
        let left = buttons.just_pressed(GamepadButton::DPadLeft);
        let right = buttons.just_pressed(GamepadButton::DPadRight);
        let stick_x = axes.get(bevy::input::gamepad::GamepadAxis::LeftStickX).unwrap_or(0.0);

        let mut moved = false;

        // Vertical movement
        if up || stick_y > 0.6 {
            move_focus(&mut focus, &focusables, true);
            moved = true;
        } else if down || stick_y < -0.6 {
            move_focus(&mut focus, &focusables, false);
            moved = true;
        }

        // Horizontal movement (treats left as previous, right as next in order)
        if left || stick_x < -0.6 {
            move_focus(&mut focus, &focusables, true);
            moved = true;
        } else if right || stick_x > 0.6 {
            move_focus(&mut focus, &focusables, false);
            moved = true;
        }

        if moved {
            unsafe { LAST_NAV_TIME = current_time; }
            break;
        }
    }
}

/// Moves focus forward or backward in the ordered list of focusable elements
fn move_focus(
    focus: &mut UiFocus,
    focusables: &Query<(Entity, &Focusable, &GlobalTransform)>,
    move_previous: bool,
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
            let new_index = if move_previous {
                if current_index == 0 { candidates.len() - 1 } else { current_index - 1 }
            } else {
                if current_index + 1 >= candidates.len() { 0 } else { current_index + 1 }
            };
            focus.current = Some(candidates[new_index].0);
            return;
        }
    }

    // Default to first element
    focus.current = Some(candidates[0].0);
}

/// Visual feedback for focused element
pub fn highlight_focused_ui(
    focus: Res<UiFocus>,
    mut query: Query<(&Focusable, &mut BackgroundColor)>,
) {
    for (focusable, mut color) in query.iter_mut() {
        if focus.current == Some(focusable) {
            *color = Color::srgb(0.3, 0.6, 1.0).into();
        } else {
            *color = Color::srgb(0.2, 0.2, 0.2).into();
        }
    }
}

/// Activates the focused button when South button is pressed
pub fn activate_focused_button(
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    focus: Res<UiFocus>,
    mut interaction_query: Query<(&Focusable, &mut Interaction)>,
) {
    let Some(focused_entity) = focus.current else { return; };
    let Ok((_, mut interaction)) = interaction_query.get_mut(focused_entity) else { return; };

    if buttons.just_pressed(GamepadButton::South) {
        *interaction = Interaction::Pressed;
    }

    if buttons.just_released(GamepadButton::South) {
        *interaction = Interaction::None;
    }
}

/// Plugin registering all UI navigation systems
pub struct UiNavigationPlugin;

impl Plugin for UiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFocus>()
            .add_systems(Update, gamepad_ui_navigation)
            .add_systems(Update, highlight_focused_ui)
            .add_systems(Update, activate_focused_button);
    }
}
