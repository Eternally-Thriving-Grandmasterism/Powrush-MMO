//! client/src/input.rs
//! Player input — INPUT_CANON one Use verb (I0 / v23.2.64)
//!
//! Layers: Pointer · Move · Use · Menu · Sheets · Look.
//! Keyboard+mouse Peace: WASD · Shift sprint · Space jump · **E = Use**.
//! Gamepad: **South = Use** (when `gamepad_south_use`); Jump = LB / LeftTrigger
//! (not South); Sprint per `sprint_mode`; Start = Pause (wired in title_screen).
//! Device changes *how you point*, never *what Use means*.
//!
//! No second Use verb. No combat face. Soft GPU first-class.
//! Contact: info@Rathor.ai · Yoi ⚡

use bevy::input::gamepad::{GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::touch::TouchInput;
use bevy::input::InputSystem;
use bevy::prelude::*;

use crate::local_settings::LocalSettingsState;
use crate::soft_play_bindings;
use shared::local_settings::LocalSettings;

/// Default left-stick deadzone (Peace desktop).
pub const STICK_DEADZONE: f32 = 0.18;

#[derive(Resource, Default, Debug)]
pub struct PlayerInput {
    pub movement: Vec2,
    pub ability_slot: Option<u32>,
    /// World Use / interact / soft harvest (E or gamepad South when enabled).
    pub interact: bool,
    /// Jump (Space; gamepad LeftTrigger/LeftBumper when South is Use).
    pub jump: bool,
    /// Sprint held (Shift; gamepad per sprint_mode).
    pub sprint: bool,
    /// Pad Start/Options edge — title_screen treats like Esc pause toggle.
    pub pause_toggle: bool,
    /// Pad West edge — sheet Q path (house / fabricator face).
    pub sheet_q: bool,
    /// Pad North edge — sheet L path (ledger).
    pub sheet_l: bool,
}

/// Last pointing device seen — drives `on_screen_sticks=auto`.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LastPointerKind {
    #[default]
    Mouse,
    Touch,
    Pad,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputMapSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default())
            .insert_resource(LastPointerKind::default())
            .configure_sets(Update, InputMapSet)
            .add_systems(PreUpdate, remap_peace_keyboard.after(InputSystem))
            .add_systems(
                Update,
                (track_last_pointer_kind, handle_player_input)
                    .chain()
                    .in_set(InputMapSet),
            );
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct KeyState {
    pressed: bool,
    just_pressed: bool,
    just_released: bool,
}

fn key_state(keyboard: &ButtonInput<KeyCode>, key: KeyCode) -> KeyState {
    KeyState {
        pressed: keyboard.pressed(key),
        just_pressed: keyboard.just_pressed(key),
        just_released: keyboard.just_released(key),
    }
}

fn either_key_state(keyboard: &ButtonInput<KeyCode>, left: KeyCode, right: KeyCode) -> KeyState {
    let left = key_state(keyboard, left);
    let right = key_state(keyboard, right);
    KeyState {
        pressed: left.pressed || right.pressed,
        just_pressed: left.just_pressed || right.just_pressed,
        just_released: left.just_released || right.just_released,
    }
}

fn write_key_state(keyboard: &mut ButtonInput<KeyCode>, key: KeyCode, state: KeyState) {
    keyboard.reset(key);
    if state.pressed {
        keyboard.press(key);
        if !state.just_pressed {
            keyboard.clear_just_pressed(key);
        }
    } else if state.just_released {
        // Recreate the release edge after reset so held-E/release consumers keep working.
        keyboard.press(key);
        keyboard.clear_just_pressed(key);
        keyboard.release(key);
    }
}

/// Translate persisted physical Peace keys onto the existing INPUT_CANON action keys.
///
/// Existing systems continue to consume one E Use, I satchel, H hide, R allocate, and the
/// canonical locomotion keys. A completely default/legacy settings file takes the exact old
/// keyboard path. Custom physical keys are consumed here so they do not also fire unrelated
/// fixed bindings.
fn remap_peace_keyboard(
    settings: Res<LocalSettingsState>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    apply_peace_keyboard(&settings.inner, &mut keyboard);
}

fn apply_peace_keyboard(cfg: &LocalSettings, keyboard: &mut ButtonInput<KeyCode>) {
    if cfg.peace_keys_are_default() {
        return;
    }

    let sprint_source = soft_play_bindings::peace_key_code(cfg.key_sprint);
    let sprint_state = if cfg.key_sprint == shared::local_settings::PeaceKey::LeftShift {
        // An unrelated remap must not remove the existing either-Shift default.
        either_key_state(
            &keyboard,
            soft_play_bindings::SPRINT_LEFT,
            soft_play_bindings::SPRINT_RIGHT,
        )
    } else {
        key_state(&keyboard, sprint_source)
    };

    let bindings = [
        (
            soft_play_bindings::MOVE_UP,
            soft_play_bindings::peace_key_code(cfg.key_move_up),
            key_state(
                &keyboard,
                soft_play_bindings::peace_key_code(cfg.key_move_up),
            ),
        ),
        (
            soft_play_bindings::MOVE_DOWN,
            soft_play_bindings::peace_key_code(cfg.key_move_down),
            key_state(
                &keyboard,
                soft_play_bindings::peace_key_code(cfg.key_move_down),
            ),
        ),
        (
            soft_play_bindings::MOVE_LEFT,
            soft_play_bindings::peace_key_code(cfg.key_move_left),
            key_state(
                &keyboard,
                soft_play_bindings::peace_key_code(cfg.key_move_left),
            ),
        ),
        (
            soft_play_bindings::MOVE_RIGHT,
            soft_play_bindings::peace_key_code(cfg.key_move_right),
            key_state(
                &keyboard,
                soft_play_bindings::peace_key_code(cfg.key_move_right),
            ),
        ),
        (
            soft_play_bindings::JUMP,
            soft_play_bindings::peace_key_code(cfg.key_jump),
            key_state(&keyboard, soft_play_bindings::peace_key_code(cfg.key_jump)),
        ),
        (soft_play_bindings::SPRINT_LEFT, sprint_source, sprint_state),
        (
            soft_play_bindings::INTERACT,
            soft_play_bindings::peace_key_code(cfg.key_use),
            key_state(&keyboard, soft_play_bindings::peace_key_code(cfg.key_use)),
        ),
        (
            soft_play_bindings::INVENTORY,
            soft_play_bindings::peace_key_code(cfg.key_satchel),
            key_state(
                &keyboard,
                soft_play_bindings::peace_key_code(cfg.key_satchel),
            ),
        ),
        (
            soft_play_bindings::HIDE_GUIDANCE,
            soft_play_bindings::peace_key_code(cfg.key_hide),
            key_state(&keyboard, soft_play_bindings::peace_key_code(cfg.key_hide)),
        ),
        (
            soft_play_bindings::ALLOCATE,
            soft_play_bindings::peace_key_code(cfg.key_allocate),
            key_state(
                &keyboard,
                soft_play_bindings::peace_key_code(cfg.key_allocate),
            ),
        ),
    ];

    // Clear every destination and every custom source before rebuilding canonical state.
    for (canonical, source, _) in bindings {
        keyboard.reset(canonical);
        keyboard.reset(source);
    }
    keyboard.reset(soft_play_bindings::SPRINT_RIGHT);

    for (canonical, _, state) in bindings {
        write_key_state(&mut keyboard, canonical, state);
    }
}

fn track_last_pointer_kind(
    mut last: ResMut<LastPointerKind>,
    mut mouse: EventReader<MouseButtonInput>,
    mut touch: EventReader<TouchInput>,
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
) {
    if touch.read().next().is_some() {
        *last = LastPointerKind::Touch;
        return;
    }
    if mouse.read().next().is_some() {
        *last = LastPointerKind::Mouse;
        return;
    }
    // Any connected pad activity → Pad (hot-plug: first connected is P1 elsewhere).
    for gamepad in gamepads.iter() {
        let active_btn = [
            GamepadButtonType::South,
            GamepadButtonType::East,
            GamepadButtonType::West,
            GamepadButtonType::North,
            GamepadButtonType::Start,
            GamepadButtonType::Select,
            GamepadButtonType::LeftTrigger,
            GamepadButtonType::RightTrigger,
            GamepadButtonType::LeftThumb,
            GamepadButtonType::RightThumb,
        ]
        .iter()
        .any(|t| {
            buttons.pressed(GamepadButton::new(gamepad, *t))
                || buttons.just_pressed(GamepadButton::new(gamepad, *t))
        });
        if active_btn {
            *last = LastPointerKind::Pad;
            return;
        }
        let lx = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap_or(0.0);
        let ly = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap_or(0.0);
        if lx.abs() > STICK_DEADZONE || ly.abs() > STICK_DEADZONE {
            *last = LastPointerKind::Pad;
            return;
        }
        break; // Player 1 = first connected
    }
}

fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<ButtonInput<GamepadButton>>,
    settings: Res<LocalSettingsState>,
    mut player_input: ResMut<PlayerInput>,
) {
    let cfg = &settings.inner;
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

    // Gamepad left stick — Player 1 = first connected (hot-plug safe).
    let mut pad = None;
    for gamepad in gamepads.iter() {
        pad = Some(gamepad);
        let lx = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap_or(0.0);
        let ly = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap_or(0.0);
        if lx.abs() > STICK_DEADZONE {
            movement.x += lx;
        }
        if ly.abs() > STICK_DEADZONE {
            movement.y += ly;
        }
        break;
    }

    if movement.length_squared() > 1.0 {
        movement = movement.normalize();
    }
    player_input.movement = movement;

    // Ability slots 1–4
    player_input.ability_slot = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(3)
    } else {
        None
    };

    let kb_use = keyboard.just_pressed(soft_play_bindings::INTERACT);
    let pad_use = pad
        .map(|g| pad_use_just_pressed(&buttons, g, cfg.gamepad_south_use))
        .unwrap_or(false);
    // One Use edge — keyboard E and South alias must not double-fire same frame.
    player_input.interact = use_edge(kb_use, pad_use);

    let kb_jump = keyboard.just_pressed(soft_play_bindings::JUMP);
    let pad_jump = pad
        .map(|g| pad_jump_just_pressed(&buttons, g, cfg.gamepad_south_use))
        .unwrap_or(false);
    player_input.jump = kb_jump || pad_jump;

    let kb_sprint = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let pad_sprint = pad
        .map(|g| pad_sprint_pressed(&buttons, g, cfg.sprint_mode.as_str()))
        .unwrap_or(false);
    player_input.sprint = kb_sprint || pad_sprint;

    player_input.pause_toggle = pad
        .map(|g| buttons.just_pressed(GamepadButton::new(g, GamepadButtonType::Start)))
        .unwrap_or(false);
    player_input.sheet_q = pad
        .map(|g| buttons.just_pressed(GamepadButton::new(g, GamepadButtonType::West)))
        .unwrap_or(false);
    player_input.sheet_l = pad
        .map(|g| buttons.just_pressed(GamepadButton::new(g, GamepadButtonType::North)))
        .unwrap_or(false);
}

/// Collapse keyboard+pad Use edges into one verb fire.
pub fn use_edge(keyboard_use: bool, pad_use: bool) -> bool {
    keyboard_use || pad_use
}

/// Pure: pad Use when South maps to Use.
pub fn pad_use_just_pressed(
    buttons: &ButtonInput<GamepadButton>,
    gamepad: Gamepad,
    gamepad_south_use: bool,
) -> bool {
    if !gamepad_south_use {
        return false;
    }
    buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South))
}

/// Pure helper for unit tests — South maps to Use iff enabled.
pub fn south_is_use(gamepad_south_use: bool) -> bool {
    gamepad_south_use
}

/// Jump on pad: when South is Use, jump is LeftTrigger or LeftBumper (LB).
/// When South is not Use, South remains jump (legacy escape hatch).
pub fn pad_jump_just_pressed(
    buttons: &ButtonInput<GamepadButton>,
    gamepad: Gamepad,
    gamepad_south_use: bool,
) -> bool {
    if gamepad_south_use {
        // Bevy 0.14: LeftTrigger = LB (bumper); LeftTrigger2 = LT (analog).
        buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger))
    } else {
        buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South))
    }
}

/// Sprint hold from pad per sprint_mode (stick / trigger / key).
/// `key` = keyboard Shift only (no pad sprint).
/// `stick` = LeftThumb (stick click).
/// `trigger` = LeftTrigger2 (LT analog digital).
pub fn pad_sprint_pressed(
    buttons: &ButtonInput<GamepadButton>,
    gamepad: Gamepad,
    sprint_mode: &str,
) -> bool {
    match sprint_mode.trim().to_ascii_lowercase().as_str() {
        "stick" => buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftThumb)),
        "trigger" => buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger2)),
        _ => false, // key = Shift only
    }
}

/// Resolve overlay visibility from settings + last pointer (pure).
pub fn overlay_sticks_visible(settings: &LocalSettings, last: LastPointerKind) -> bool {
    settings.resolve_on_screen_sticks(last == LastPointerKind::Touch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::local_settings::PeaceKey;

    #[test]
    fn use_edge_dedupes_same_frame_alias() {
        assert!(!use_edge(false, false));
        assert!(use_edge(true, false));
        assert!(use_edge(false, true));
        // Both true → still one Use (bool OR, not two events).
        assert!(use_edge(true, true));
    }

    #[test]
    fn south_maps_to_use_by_default() {
        assert!(south_is_use(true));
        assert!(!south_is_use(false));
        let s = LocalSettings::peace_defaults();
        assert!(s.gamepad_south_use);
        assert!(south_is_use(s.gamepad_south_use));
    }

    #[test]
    fn overlay_hidden_for_mouse_auto() {
        let s = LocalSettings::peace_defaults();
        assert_eq!(s.on_screen_sticks, "auto");
        assert!(!overlay_sticks_visible(&s, LastPointerKind::Mouse));
        assert!(!overlay_sticks_visible(&s, LastPointerKind::Pad));
        assert!(overlay_sticks_visible(&s, LastPointerKind::Touch));
        let mut on = s.clone();
        on.on_screen_sticks = "on".into();
        assert!(overlay_sticks_visible(&on, LastPointerKind::Mouse));
        let mut off = s.clone();
        off.on_screen_sticks = "off".into();
        assert!(!overlay_sticks_visible(&off, LastPointerKind::Touch));
    }

    #[test]
    fn sprint_mode_key_is_peace_default() {
        let s = LocalSettings::peace_defaults();
        assert!(s.sprint_mode_is_key());
        assert!(!s.sprint_mode_is_stick());
        assert!(!s.sprint_mode_is_trigger());
    }

    #[test]
    fn default_keyboard_path_is_untouched() {
        let settings = LocalSettings::peace_defaults();
        let mut keyboard = ButtonInput::default();
        keyboard.press(KeyCode::KeyE);
        keyboard.press(KeyCode::ShiftRight);

        apply_peace_keyboard(&settings, &mut keyboard);

        assert!(keyboard.just_pressed(KeyCode::KeyE));
        assert!(keyboard.pressed(KeyCode::ShiftRight));
        assert!(!keyboard.pressed(KeyCode::ShiftLeft));
    }

    #[test]
    fn custom_keys_feed_existing_input_canon_and_consume_sources() {
        let mut settings = LocalSettings::peace_defaults();
        settings.key_move_up = PeaceKey::ArrowUp;
        settings.key_jump = PeaceKey::J;
        settings.key_sprint = PeaceKey::LeftControl;
        settings.key_use = PeaceKey::F;
        settings.key_satchel = PeaceKey::B;
        settings.key_hide = PeaceKey::V;
        settings.key_allocate = PeaceKey::N;
        let mut keyboard = ButtonInput::default();
        for key in [
            KeyCode::ArrowUp,
            KeyCode::KeyJ,
            KeyCode::ControlLeft,
            KeyCode::KeyF,
            KeyCode::KeyB,
            KeyCode::KeyV,
            KeyCode::KeyN,
        ] {
            keyboard.press(key);
        }
        // Old defaults are physical presses too, but a custom map replaces them.
        keyboard.press(KeyCode::KeyE);
        keyboard.press(KeyCode::Space);

        apply_peace_keyboard(&settings, &mut keyboard);

        for canonical in [
            KeyCode::KeyW,
            KeyCode::Space,
            KeyCode::ShiftLeft,
            KeyCode::KeyE,
            KeyCode::KeyI,
            KeyCode::KeyH,
            KeyCode::KeyR,
        ] {
            assert!(
                keyboard.pressed(canonical),
                "{canonical:?} was not translated"
            );
            assert!(
                keyboard.just_pressed(canonical),
                "{canonical:?} lost its press edge"
            );
        }
        for source in [
            KeyCode::ArrowUp,
            KeyCode::KeyJ,
            KeyCode::ControlLeft,
            KeyCode::KeyF,
            KeyCode::KeyB,
            KeyCode::KeyV,
            KeyCode::KeyN,
        ] {
            assert!(!keyboard.pressed(source), "{source:?} was not consumed");
        }
    }

    #[test]
    fn remapped_use_preserves_release_edge_for_hold_consumers() {
        let mut settings = LocalSettings::peace_defaults();
        settings.key_use = PeaceKey::F;
        let mut keyboard = ButtonInput::default();
        keyboard.press(KeyCode::KeyF);
        keyboard.clear();
        keyboard.release(KeyCode::KeyF);

        apply_peace_keyboard(&settings, &mut keyboard);

        assert!(!keyboard.pressed(KeyCode::KeyE));
        assert!(keyboard.just_released(KeyCode::KeyE));
        assert!(!keyboard.just_released(KeyCode::KeyF));
    }
}
