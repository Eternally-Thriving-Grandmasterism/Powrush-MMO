// game/anti_cheat.rs
// Powrush-MMO — Server-Side Anti-Cheat & Input Validation
// AG-SML v1.0 License

use crate::game::types::{PlayerInput, PlayerState};
use crate::game::movement::MovementConfig;

#[derive(Debug, Clone)]
pub enum ValidationResult {
    Valid,
    Invalid(String),
}

#[derive(Debug, Clone)]
pub struct AntiCheatConfig {
    pub max_speed_multiplier: f32,
    pub max_position_delta_per_tick: f32,
    pub max_yaw_change_per_tick: f32,
    pub max_pitch_change_per_tick: f32,
    pub max_attacks_per_second: f32,
}

impl Default for AntiCheatConfig {
    fn default() -> Self {
        Self {
            max_speed_multiplier: 1.5,
            max_position_delta_per_tick: 2.0,
            max_yaw_change_per_tick: 120.0,
            max_pitch_change_per_tick: 90.0,
            max_attacks_per_second: 10.0,
        }
    }
}

pub fn validate_input(
    input: &PlayerInput,
    previous_input: Option<&PlayerInput>,
    config: &AntiCheatConfig,
) -> ValidationResult {
    if let Some(prev) = previous_input {
        let yaw_delta = (input.yaw - prev.yaw).abs();
        if yaw_delta > config.max_yaw_change_per_tick {
            return ValidationResult::Invalid(format!("Suspicious yaw change: {:.1}°", yaw_delta));
        }

        let pitch_delta = (input.pitch - prev.pitch).abs();
        if pitch_delta > config.max_pitch_change_per_tick {
            return ValidationResult::Invalid(format!("Suspicious pitch change: {:.1}°", pitch_delta));
        }
    }

    if input.yaw.is_nan() || input.pitch.is_nan() {
        return ValidationResult::Invalid("Invalid rotation values".to_string());
    }

    ValidationResult::Valid
}

pub fn validate_state(
    new_state: &PlayerState,
    old_state: &PlayerState,
    config: &AntiCheatConfig,
    movement_config: &MovementConfig,
) -> ValidationResult {
    let dx = new_state.position.0 - old_state.position.0;
    let dy = new_state.position.1 - old_state.position.1;
    let dz = new_state.position.2 - old_state.position.2;
    let distance = (dx * dx + dy * dy + dz * dz).sqrt();

    let max_allowed = movement_config.sprint_speed * config.max_speed_multiplier;

    if distance > max_allowed {
        return ValidationResult::Invalid(format!("Moved too fast: {:.2} units", distance));
    }

    if dy > 5.0 && !old_state.is_grounded {
        return ValidationResult::Invalid("Suspicious vertical movement".to_string());
    }

    ValidationResult::Valid
}

pub struct AttackRateLimiter {
    last_attack_tick: u64,
    min_ticks_between_attacks: u64,
}

impl AttackRateLimiter {
    pub fn new(attacks_per_second: f32, tick_rate: u64) -> Self {
        let min_ticks = (tick_rate as f32 / attacks_per_second).ceil() as u64;
        Self {
            last_attack_tick: 0,
            min_ticks_between_attacks: min_ticks,
        }
    }

    pub fn can_attack(&mut self, current_tick: u64) -> bool {
        if current_tick >= self.last_attack_tick + self.min_ticks_between_attacks {
            self.last_attack_tick = current_tick;
            true
        } else {
            false
        }
    }
}