/*!
 * Basic Controller / Gamepad UI Navigation System
 *
 * v2 - Added Button Activation Logic
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::input::gamepad::{GamepadButton, Gamepads};

/// Marker component for UI elements that can be focused with a controller
#[derive(Component, Clone, Debug, Default)]
pub struct Focusable {
    pub order: i32,
}

/// Resource that tracks the currently focused UI entity
#[derive(Resource, Default)]
pub struct UiFocus {
    pub current: Option<Entity>,
}

/// System that handles gamepad input for UI navigation
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
        let up = buttons.just_pressed(GamepadButton::DPadUp);
        let down = buttons.just_pressed(GamepadButton::DPadDown);

        let stick_y = axes.get(bevy::input::gamepad::GamepadAxis::LeftStickY).unwrap_or(0.0);

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

    focus.current = Some(candidates[0].0);
}

/// Visual feedback for focused element
pub fn highlight_focused_ui(
    focus: Res<UiFocus>,
    mut query: Query<(&Focusable, &mut BackgroundColor)>,
) {
    for (focusable, mut color) in query.iter_mut() {
        // Simple highlight - can be improved with a dedicated Focused component later
        if focus.current == Some(focusable) {
            *color = Color::srgb(0.3, 0.6, 1.0).into();
        } else {
            *color = Color::srgb(0.2, 0.2, 0.2).into();
        }
    }
}

/// Activates the currently focused button when South button (A/Cross) is pressed
pub fn activate_focused_button(
    buttons: Res<bevy::input::gamepad::GamepadButton>,
    focus: Res<UiFocus>,
    mut interaction_query: Query<(&Focusable, &mut Interaction)>,
) {
    // South button = A on Xbox, Cross on PlayStation
    if buttons.just_pressed(GamepadButton::South) {
        if let Some(focused_entity) = focus.current {
            if let Ok((_, mut interaction)) = interaction_query.get_mut(focused_entity) {
                *interaction = Interaction::Pressed;
            }
        }
    }

    // Release the button when South is released
    if buttons.just_released(GamepadButton::South) {
        if let Some(focused_entity) = focus.current {
            if let Ok((_, mut interaction)) = interaction_query.get_mut(focused_entity) {
                *interaction = Interaction::None;
            }
        }
    }
}

/// Plugin to register all UI navigation systems
pub struct UiNavigationPlugin;

impl Plugin for UiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFocus>()
            .add_systems(Update, gamepad_ui_navigation)
            .add_systems(Update, highlight_focused_ui)
            .add_systems(Update, activate_focused_button);
    }
}
