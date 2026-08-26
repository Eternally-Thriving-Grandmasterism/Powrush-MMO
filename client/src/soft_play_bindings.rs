/*!
 * Soft-play + core locomotion bindings — v21.99.3
 *
 * M / Z are primary. F2 / F3 remain silent aliases so old muscle memory is not punished.
 *
 * Contact: info@Rathor.ai · PATSAGi · Yoi ⚡
 */

use bevy::prelude::{ButtonInput, KeyCode};

pub const JUMP: KeyCode = KeyCode::Space;
pub const INTERACT: KeyCode = KeyCode::KeyE;
pub const SPRINT_LEFT: KeyCode = KeyCode::ShiftLeft;
pub const SPRINT_RIGHT: KeyCode = KeyCode::ShiftRight;

pub const JOURNEY_ECHO: KeyCode = KeyCode::KeyJ;
pub const FOUNDATION_LATTICE: KeyCode = KeyCode::KeyL;
pub const RESONANCE_CYCLE: KeyCode = KeyCode::KeyG;
pub const MERCY_TRANSPORTERS: KeyCode = KeyCode::KeyT;
pub const PEER_INGEST: KeyCode = KeyCode::KeyU;
pub const MY_MERCY_JOURNEY: KeyCode = KeyCode::KeyM;
pub const REALM_TRAVEL: KeyCode = KeyCode::KeyZ;
pub const FORCE_CLOUD_FLUSH: KeyCode = KeyCode::KeyT;

/// Legacy F-row aliases — heard, never taught.
pub const MY_MERCY_JOURNEY_ALIAS: KeyCode = KeyCode::F2;
pub const REALM_TRAVEL_ALIAS: KeyCode = KeyCode::F3;

pub fn mercy_journey_just_pressed(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.just_pressed(MY_MERCY_JOURNEY) || keyboard.just_pressed(MY_MERCY_JOURNEY_ALIAS)
}

pub fn realm_travel_just_pressed(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.just_pressed(REALM_TRAVEL) || keyboard.just_pressed(REALM_TRAVEL_ALIAS)
}

pub fn soft_play_legend() -> &'static str {
    "WASD move · Space jump · E interact · Shift sprint · P practice · R allocate · J journey · L lattice · G resonance · T transporters · U peer · M mercy · Z realm · Shift+T cloud"
}

pub fn locomotion_legend() -> &'static str {
    "WASD move · Space jump · E interact · Shift sprint"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_keys_are_mainstream() {
        assert_eq!(JUMP, KeyCode::Space);
        assert_eq!(INTERACT, KeyCode::KeyE);
    }

    #[test]
    fn f_row_is_alias_only() {
        assert_eq!(MY_MERCY_JOURNEY, KeyCode::KeyM);
        assert_eq!(REALM_TRAVEL, KeyCode::KeyZ);
        assert_eq!(MY_MERCY_JOURNEY_ALIAS, KeyCode::F2);
        assert_eq!(REALM_TRAVEL_ALIAS, KeyCode::F3);
    }
}
