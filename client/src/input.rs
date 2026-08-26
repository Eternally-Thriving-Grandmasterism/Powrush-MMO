//! client/src/input.rs
//! Player input — mainstream muscle memory (v21.97.0)
//!
//! Universal PC cluster (PC Gamer / ARK / Starfield / CoD consensus):
//!   WASD move · Space jump · E interact · Shift sprint · Ctrl/C crouch
//!
//! Soft practice harvest and world interact both ride **E** so one key means
//! “act on the world with care” — never Space (that is jump forever).
//!
//! AG-SML v1.0 | TOLC 8 · Contact: info@Rathor.ai · Yoi ⚡

use bevy::prelude::*;
use bevy::input::gamepad::{GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType};
use crate::prediction::{PredictedPosition, PredictedAbility};
use crate::soft_play_bindings;

#[derive(Resource, Default, Debug)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub ability_slot: Option<u32>,
    /// World interact / use / soft harvest (E or gamepad West/X / South hold patterns).
    pub interact: bool,
    /// Jump (Space or gamepad South / A).
    pub jump: bool,
    /// Sprint held (Shift or gamepad Left Stick press).
    pub sprint: bool,
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
        if lx.abs() > 0.18 {
            movement.x += lx;
        }
        if ly.abs() > 0.18 {
            movement.y += ly;
        }
        break;
    }

    if movement.length_squared() > 1.0 {
        movement = movement.normalize();
    }
    player_input.movement = movement;

    // Ability slots 1–4
    player_input.ability_slot = if keyboard.just_pressed(KeyCode::Digit1)
        || keyboard.just_pressed(KeyCode::Key1)
    {
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

    // Interact: E (mainstream) — not Space
    let mut interact = keyboard.just_pressed(soft_play_bindings::INTERACT);
    for gamepad in gamepads.iter() {
        // West (X / Square) is common secondary interact; South often jump
        if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::West)) {
            interact = true;
            break;
        }
    }
    player_input.interact = interact;

    // Jump: Space (universal muscle memory)
    let mut jump = keyboard.just_pressed(soft_play_bindings::JUMP);
    for gamepad in gamepads.iter() {
        if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            jump = true;
            break;
        }
    }
    player_input.jump = jump;

    // Sprint: Left/Right Shift held
    let mut sprint = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    for gamepad in gamepads.iter() {
        if buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftThumb)) {
            sprint = true;
            break;
        }
    }
    player_input.sprint = sprint;
}

fn apply_input_to_prediction(
    mut query: Query<(&mut PredictedPosition, &mut PredictedAbility), With<Player>>,
    player_input: Res<PlayerInput>,
    time: Res<Time>,
) {
    let speed = if player_input.sprint { 16.0 } else { 10.0 };
    for (mut pos, mut ability) in &mut query {
        let delta = player_input.movement * speed * time.delta_seconds();
        pos.position += delta.extend(0.0);
        pos.velocity = delta.extend(0.0);

        // Soft jump impulse (client prediction placeholder — authoritative later)
        if player_input.jump {
            pos.velocity.y += 6.0;
            pos.position.y += 0.35;
        }

        if let Some(slot) = player_input.ability_slot {
            ability.ability_id = slot;
        }
    }
}

#[derive(Component)]
struct Player;

// Mercy-gated, predicted locally, reconciled with authority.
// Space=jump · E=interact · Shift=sprint — sealed under PATSAGi.
