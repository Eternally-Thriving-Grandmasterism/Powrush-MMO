/*!
 * Spatial Grid UI Navigation System (Advanced)
 *
 * Supports true spatial navigation based on element positions.
 * Much better for grid-style UIs (inventory, skills, button grids).
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use bevy::input::gamepad::{GamepadButton, Gamepads};

#[derive(Component, Clone, Debug, Default)]
pub struct Focusable {
    pub order: i32,
}

#[derive(Resource, Default)]
pub struct UiFocus {
    pub current: Option<Entity>,
}

/// Direction for spatial navigation
#[derive(Clone, Copy)]
pub enum NavDirection {
    Up,
    Down,
    Left,
    Right,
}

impl NavDirection {
    fn as_vec2(self) -> Vec2 {
        match self {
            NavDirection::Up => Vec2::new(0.0, 1.0),
            NavDirection::Down => Vec2::new(0.0, -1.0),
            NavDirection::Left => Vec2::new(-1.0, 0.0),
            NavDirection::Right => Vec2::new(1.0, 0.0),
        }
    }
}

/// Main navigation system with spatial awareness
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
        if current_time - LAST_NAV_TIME < 0.15 {
            return;
        }
    }

    for _gamepad in gamepads.iter() {
        let mut direction: Option<NavDirection> = None;

        // D-pad
        if buttons.just_pressed(GamepadButton::DPadUp) { direction = Some(NavDirection::Up); }
        else if buttons.just_pressed(GamepadButton::DPadDown) { direction = Some(NavDirection::Down); }
        else if buttons.just_pressed(GamepadButton::DPadLeft) { direction = Some(NavDirection::Left); }
        else if buttons.just_pressed(GamepadButton::DPadRight) { direction = Some(NavDirection::Right); }

        // Left stick (with threshold)
        let stick_x = axes.get(bevy::input::gamepad::GamepadAxis::LeftStickX).unwrap_or(0.0);
        let stick_y = axes.get(bevy::input::gamepad::GamepadAxis::LeftStickY).unwrap_or(0.0);

        if direction.is_none() {
            if stick_y > 0.7 { direction = Some(NavDirection::Up); }
            else if stick_y < -0.7 { direction = Some(NavDirection::Down); }
            else if stick_x < -0.7 { direction = Some(NavDirection::Left); }
            else if stick_x > 0.7 { direction = Some(NavDirection::Right); }
        }

        if let Some(dir) = direction {
            if let Some(current) = focus.current {
                if let Some(best) = find_best_focusable_in_direction(current, dir, &focusables) {
                    focus.current = Some(best);
                    unsafe { LAST_NAV_TIME = current_time; }
                }
            } else {
                // No current focus - pick first element
                if let Some((first, _, _)) = focusables.iter().next() {
                    focus.current = Some(first);
                }
            }
        }
    }
}

/// Finds the best focusable element in a given direction from the current one
fn find_best_focusable_in_direction(
    current: Entity,
    direction: NavDirection,
    focusables: &Query<(Entity, &Focusable, &GlobalTransform)>,
) -> Option<Entity> {
    let current_pos = focusables
        .get(current)
        .map(|(_, _, transform)| transform.translation().truncate())
        .ok()?;

    let dir_vec = direction.as_vec2();

    let mut best_entity = None;
    let mut best_score = f32::NEG_INFINITY;

    for (entity, _, transform) in focusables.iter() {
        if entity == current {
            continue;
        }

        let pos = transform.translation().truncate();
        let to_target = pos - current_pos;

        // Only consider elements in the general direction we're moving
        let dot = to_target.dot(dir_vec);
        if dot <= 0.0 {
            continue;
        }

        // Score based on alignment + distance
        let distance = to_target.length();
        let alignment = dot / distance.max(1.0); // How well aligned with direction

        // Prefer elements that are well aligned and not too far
        let score = alignment * 100.0 - distance * 0.5;

        if score > best_score {
            best_score = score;
            best_entity = Some(entity);
        }
    }

    best_entity
}

/// Visual feedback
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

/// Button activation (South button)
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

pub struct UiNavigationPlugin;

impl Plugin for UiNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiFocus>()
            .add_systems(Update, gamepad_ui_navigation)
            .add_systems(Update, highlight_focused_ui)
            .add_systems(Update, activate_focused_button);
    }
}
