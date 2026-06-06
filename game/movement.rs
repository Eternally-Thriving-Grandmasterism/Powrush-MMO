// game/movement.rs
// Powrush-MMO — Movement System with Lag Mitigation
// Supports client-side prediction and server reconciliation.
// AG-SML v1.0 License

use crate::game::types::PlayerState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementConfig {
    pub walk_speed: f32,
    pub sprint_speed: f32,
    pub acceleration: f32,
    pub friction: f32,
    pub gravity: f32,
    pub jump_force: f32,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            walk_speed: 5.0,
            sprint_speed: 8.0,
            acceleration: 40.0,
            friction: 10.0,
            gravity: 20.0,
            jump_force: 8.0,
        }
    }
}

pub fn apply_movement(
    mut state: PlayerState,
    input: &crate::game::types::PlayerInput,
    config: &MovementConfig,
    delta_time: f32,
) -> PlayerState {
    let mut wish_dir = (0.0, 0.0, 0.0);

    if input.move_forward { wish_dir.2 += 1.0; }
    if input.move_backward { wish_dir.2 -= 1.0; }
    if input.move_left { wish_dir.0 -= 1.0; }
    if input.move_right { wish_dir.0 += 1.0; }

    let len = (wish_dir.0 * wish_dir.0 + wish_dir.2 * wish_dir.2).sqrt();
    if len > 0.0 {
        wish_dir.0 /= len;
        wish_dir.2 /= len;
    }

    let target_speed = if input.move_forward || input.move_backward || input.move_left || input.move_right {
        if input.move_forward { config.sprint_speed } else { config.walk_speed }
    } else {
        0.0
    };

    let current_speed = state.velocity.0 * wish_dir.0 + state.velocity.2 * wish_dir.2;
    let add_speed = target_speed - current_speed;

    if add_speed > 0.0 {
        let accel = config.acceleration * delta_time;
        let accel_speed = if add_speed < accel { add_speed } else { accel };

        state.velocity.0 += wish_dir.0 * accel_speed;
        state.velocity.2 += wish_dir.2 * accel_speed;
    }

    let speed = (state.velocity.0 * state.velocity.0 + state.velocity.2 * state.velocity.2).sqrt();
    if speed > 0.0 {
        let drop = speed * config.friction * delta_time;
        let new_speed = if speed - drop < 0.0 { 0.0 } else { speed - drop };

        state.velocity.0 *= new_speed / speed;
        state.velocity.2 *= new_speed / speed;
    }

    if !state.is_grounded {
        state.velocity.1 -= config.gravity * delta_time;
    }

    state.position.0 += state.velocity.0 * delta_time;
    state.position.1 += state.velocity.1 * delta_time;
    state.position.2 += state.velocity.2 * delta_time;

    if state.position.1 < 0.0 {
        state.position.1 = 0.0;
        state.velocity.1 = 0.0;
        state.is_grounded = true;
    } else {
        state.is_grounded = false;
    }

    state.yaw = input.yaw;
    state.pitch = input.pitch;

    state
}