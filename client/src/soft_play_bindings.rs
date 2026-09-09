/*!
 * Soft-play + core locomotion bindings — v22.1.0
 *
 * I = satchel (GoldenEye watch / Ocarina bag).
 * Contact: info@Rathor.ai · PATSAGi · Yoi ⚡
 */

use bevy::prelude::{ButtonInput, KeyCode};
use shared::local_settings::PeaceKey;

pub const JUMP: KeyCode = KeyCode::Space;
pub const INTERACT: KeyCode = KeyCode::KeyE;
pub const SPRINT_LEFT: KeyCode = KeyCode::ShiftLeft;
pub const SPRINT_RIGHT: KeyCode = KeyCode::ShiftRight;
pub const MOVE_UP: KeyCode = KeyCode::KeyW;
pub const MOVE_DOWN: KeyCode = KeyCode::KeyS;
pub const MOVE_LEFT: KeyCode = KeyCode::KeyA;
pub const MOVE_RIGHT: KeyCode = KeyCode::KeyD;

pub const JOURNEY_ECHO: KeyCode = KeyCode::KeyJ;
pub const FOUNDATION_LATTICE: KeyCode = KeyCode::KeyL;
pub const RESONANCE_CYCLE: KeyCode = KeyCode::KeyG;
pub const MERCY_TRANSPORTERS: KeyCode = KeyCode::KeyT;
pub const PEER_INGEST: KeyCode = KeyCode::KeyU;
pub const MY_MERCY_JOURNEY: KeyCode = KeyCode::KeyM;
pub const REALM_TRAVEL: KeyCode = KeyCode::KeyZ;
pub const INVENTORY: KeyCode = KeyCode::KeyI;
pub const HIDE_GUIDANCE: KeyCode = KeyCode::KeyH;
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

/// Convert the persisted, engine-independent INPUT_CANON key to Bevy's keyboard code.
pub fn peace_key_code(key: PeaceKey) -> KeyCode {
    match key {
        PeaceKey::A => KeyCode::KeyA,
        PeaceKey::B => KeyCode::KeyB,
        PeaceKey::C => KeyCode::KeyC,
        PeaceKey::D => KeyCode::KeyD,
        PeaceKey::E => KeyCode::KeyE,
        PeaceKey::F => KeyCode::KeyF,
        PeaceKey::G => KeyCode::KeyG,
        PeaceKey::H => KeyCode::KeyH,
        PeaceKey::I => KeyCode::KeyI,
        PeaceKey::J => KeyCode::KeyJ,
        PeaceKey::K => KeyCode::KeyK,
        PeaceKey::L => KeyCode::KeyL,
        PeaceKey::M => KeyCode::KeyM,
        PeaceKey::N => KeyCode::KeyN,
        PeaceKey::O => KeyCode::KeyO,
        PeaceKey::P => KeyCode::KeyP,
        PeaceKey::Q => KeyCode::KeyQ,
        PeaceKey::R => KeyCode::KeyR,
        PeaceKey::S => KeyCode::KeyS,
        PeaceKey::T => KeyCode::KeyT,
        PeaceKey::U => KeyCode::KeyU,
        PeaceKey::V => KeyCode::KeyV,
        PeaceKey::W => KeyCode::KeyW,
        PeaceKey::X => KeyCode::KeyX,
        PeaceKey::Y => KeyCode::KeyY,
        PeaceKey::Z => KeyCode::KeyZ,
        PeaceKey::Digit0 => KeyCode::Digit0,
        PeaceKey::Digit1 => KeyCode::Digit1,
        PeaceKey::Digit2 => KeyCode::Digit2,
        PeaceKey::Digit3 => KeyCode::Digit3,
        PeaceKey::Digit4 => KeyCode::Digit4,
        PeaceKey::Digit5 => KeyCode::Digit5,
        PeaceKey::Digit6 => KeyCode::Digit6,
        PeaceKey::Digit7 => KeyCode::Digit7,
        PeaceKey::Digit8 => KeyCode::Digit8,
        PeaceKey::Digit9 => KeyCode::Digit9,
        PeaceKey::ArrowUp => KeyCode::ArrowUp,
        PeaceKey::ArrowDown => KeyCode::ArrowDown,
        PeaceKey::ArrowLeft => KeyCode::ArrowLeft,
        PeaceKey::ArrowRight => KeyCode::ArrowRight,
        PeaceKey::Space => KeyCode::Space,
        PeaceKey::LeftShift => KeyCode::ShiftLeft,
        PeaceKey::RightShift => KeyCode::ShiftRight,
        PeaceKey::LeftControl => KeyCode::ControlLeft,
        PeaceKey::RightControl => KeyCode::ControlRight,
        PeaceKey::LeftAlt => KeyCode::AltLeft,
        PeaceKey::RightAlt => KeyCode::AltRight,
        PeaceKey::Tab => KeyCode::Tab,
        PeaceKey::Enter => KeyCode::Enter,
        PeaceKey::Backspace => KeyCode::Backspace,
    }
}

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
        assert_eq!(MOVE_UP, KeyCode::KeyW);
        assert_eq!(MOVE_DOWN, KeyCode::KeyS);
        assert_eq!(MOVE_LEFT, KeyCode::KeyA);
        assert_eq!(MOVE_RIGHT, KeyCode::KeyD);
        assert_eq!(JUMP, KeyCode::Space);
        assert_eq!(INTERACT, KeyCode::KeyE);
        assert_eq!(INVENTORY, KeyCode::KeyI);
        assert_eq!(HIDE_GUIDANCE, KeyCode::KeyH);
        assert_eq!(ALLOCATE, KeyCode::KeyR);
        assert_eq!(CHART, KeyCode::Tab);
        assert_eq!(SASH, KeyCode::KeyG);
        assert_eq!(LEDGER, KeyCode::KeyL);
        assert_eq!(BUILD_WHEEL, KeyCode::KeyQ);
    }

    #[test]
    fn persisted_peace_keys_resolve_to_existing_input_canon() {
        assert_eq!(peace_key_code(PeaceKey::W), MOVE_UP);
        assert_eq!(peace_key_code(PeaceKey::Space), JUMP);
        assert_eq!(peace_key_code(PeaceKey::LeftShift), SPRINT_LEFT);
        assert_eq!(peace_key_code(PeaceKey::E), INTERACT);
        assert_eq!(peace_key_code(PeaceKey::I), INVENTORY);
        assert_eq!(peace_key_code(PeaceKey::H), HIDE_GUIDANCE);
        assert_eq!(peace_key_code(PeaceKey::R), ALLOCATE);
    }
}
