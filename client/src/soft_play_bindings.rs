/*!
 * Soft-play + core locomotion bindings — v22.1.0
 *
 * I = satchel (GoldenEye watch / Ocarina bag).
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
pub const INVENTORY: KeyCode = KeyCode::KeyI;
pub const ALLOCATE: KeyCode = KeyCode::KeyR;
/// Charter Chart. No-op in Peace (Slice 0).
pub const CHART: KeyCode = KeyCode::Tab;
/// Charter sash. Same physical key as resonance; gated by HourSacred.
pub const SASH: KeyCode = KeyCode::KeyG;
/// Ledger. Same physical key as foundation lattice; gated by HourSacred.
pub const LEDGER: KeyCode = KeyCode::KeyL;
/// Build wheel. Dies in Peace.
pub const BUILD_WHEEL: KeyCode = KeyCode::KeyQ;
pub const FORCE_CLOUD_FLUSH: KeyCode = KeyCode::KeyT;

pub const MY_MERCY_JOURNEY_ALIAS: KeyCode = KeyCode::F2;
pub const REALM_TRAVEL_ALIAS: KeyCode = KeyCode::F3;

pub fn mercy_journey_just_pressed(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.just_pressed(MY_MERCY_JOURNEY) || keyboard.just_pressed(MY_MERCY_JOURNEY_ALIAS)
}

pub fn realm_travel_just_pressed(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.just_pressed(REALM_TRAVEL) || keyboard.just_pressed(REALM_TRAVEL_ALIAS)
}

pub fn soft_play_legend() -> &'static str {
    "WASD move · Space jump · E interact · Shift sprint · I satchel · P practice · R allocate · J journey · M mercy · Z realm"
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
        assert_eq!(INVENTORY, KeyCode::KeyI);
        assert_eq!(ALLOCATE, KeyCode::KeyR);
        assert_eq!(CHART, KeyCode::Tab);
        assert_eq!(SASH, KeyCode::KeyG);
        assert_eq!(LEDGER, KeyCode::KeyL);
        assert_eq!(BUILD_WHEEL, KeyCode::KeyQ);
    }
}
