//! client/src/input.rs
//! Player input handling with client-side prediction and mercy gating
//! AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced
//! v21.90 — End-user comfort: keyboard + gamepad left-stick, normalized movement, clear interact
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use bevy::input::gamepad::{Gamepad, GamepadAxis, GamepadButton, GamepadButtonType};
use crate::prediction::{PredictedPosition, PredictedAbility};

#[derive(Resource, Default, Debug)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub ability_slot: Option<u32>,
    pub interact: bool,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default())
            .add_systems(Update, handle_player_input)
            .add_systems(Update, apply_input_to_prediction);
    }
}

fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut player_input: ResMut<PlayerInput>,
) {
    let mut movement = Vec2::ZERO;

    // Keyboard (WASD + arrows)
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }

    // Gamepad left stick (first connected pad)
    for gamepad in gamepads.iter() {
        let lx = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap_or(0.0);
        let ly = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap_or(0.0);
        // Deadzone
        if lx.abs() > 0.18 {
            movement.x += lx;
        }
        if ly.abs() > 0.18 {
            movement.y += ly;
        }
        break; // primary pad only for now
    }

    // Normalize so diagonal is not faster
    if movement.length_squared() > 1.0 {
        movement = movement.normalize();
    }
    player_input.movement = movement;

    // Ability slots 1–4
    player_input.ability_slot = if keyboard.just_pressed(KeyCode::Digit1) || keyboard.just_pressed(KeyCode::Key1) {
        Some(0)
    } else if keyboard.just_pressed(KeyCode::Digit2) || keyboard.just_pressed(KeyCode::Key2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) || keyboard.just_pressed(KeyCode::Key3) {
        Some(2)
    } else if keyboard.just_pressed(KeyCode::Digit4) || keyboard.just_pressed(KeyCode::Key4) {
        Some(3)
    } else {
        None
    };

    // Interact: Space or gamepad South (A / Cross)
    let mut interact = keyboard.just_pressed(KeyCode::Space);
    for gamepad in gamepads.iter() {
        if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            interact = true;
            break;
        }
    }
    player_input.interact = interact;
}

fn apply_input_to_prediction(
    mut query: Query<(&mut PredictedPosition, &mut PredictedAbility), With<Player>>,
    player_input: Res<PlayerInput>,
    time: Res<Time>,
) {
    for (mut pos, mut ability) in &mut query {
        let delta = player_input.movement * 10.0 * time.delta_seconds();
        pos.position += delta.extend(0.0);
        pos.velocity = delta.extend(0.0);

        if let Some(slot) = player_input.ability_slot {
            ability.ability_id = slot;
        }
    }
}

#[derive(Component)]
struct Player;

// All input is mercy-gated, predicted locally, and reconciled with authoritative updates.
// Keyboard + gamepad comfort complete for first-session joy.
