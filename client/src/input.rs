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

use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::touch::TouchInput;
use bevy::input::{ButtonState, InputSystems};
use bevy::prelude::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

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
    /// Just-pressed edge. The 0.120 s feel-move buffer still folds into this flag.
    pub interact: bool,
    /// Same Use sources, held this frame: canonical E (after the remap path)
    /// or the first gamepad's Use button. Edit mode clears this with `interact`.
    pub interact_held: bool,
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

/// PreUpdate chain that copies physical Peace keys onto the canon action keys.
/// Edit mode's key guard runs after this set.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PeaceRemapSet;

/// Physical keyboard state retained before custom source keys are consumed.
#[derive(Resource, Default)]
struct PhysicalKeyboard {
    pressed: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInput::default())
            .insert_resource(LastPointerKind::default())
            .init_resource::<PhysicalKeyboard>()
            .configure_sets(Update, InputMapSet)
            .configure_sets(PreUpdate, PeaceRemapSet.after(InputSystems))
            .add_systems(
                PreUpdate,
                (capture_physical_keyboard, remap_peace_keyboard)
                    .chain()
                    .in_set(PeaceRemapSet),
            )
            .add_systems(
                Update,
                (track_last_pointer_kind, handle_player_input)
                    .chain()
                    .in_set(InputMapSet),
            )
            // No resource → this system does not run. No `--script` stays on the device path.
            // Idempotent: WindowPlugin already registers this message in the full client.
            .add_message::<bevy::window::WindowCloseRequested>()
            .add_systems(
                Update,
                apply_script_timeline
                    .after(handle_player_input)
                    .in_set(InputMapSet)
                    .run_if(resource_exists::<ScriptTimeline>),
            );
    }
}

fn capture_physical_keyboard(
    mut events: MessageReader<KeyboardInput>,
    mut physical: ResMut<PhysicalKeyboard>,
) {
    physical.just_pressed.clear();
    physical.just_released.clear();
    for event in events.read() {
        match event.state {
            ButtonState::Pressed => {
                if physical.pressed.insert(event.key_code) {
                    physical.just_pressed.insert(event.key_code);
                }
            }
            ButtonState::Released => {
                if physical.pressed.remove(&event.key_code) {
                    physical.just_released.insert(event.key_code);
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct KeyState {
    pressed: bool,
    just_pressed: bool,
    just_released: bool,
}

fn key_state(keyboard: &PhysicalKeyboard, key: KeyCode) -> KeyState {
    KeyState {
        pressed: keyboard.pressed.contains(&key),
        just_pressed: keyboard.just_pressed.contains(&key),
        just_released: keyboard.just_released.contains(&key),
    }
}

fn either_key_state(keyboard: &PhysicalKeyboard, left: KeyCode, right: KeyCode) -> KeyState {
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
    physical: Res<PhysicalKeyboard>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
) {
    apply_peace_keyboard(&settings.inner, &physical, &mut keyboard);
}

fn apply_peace_keyboard(
    cfg: &LocalSettings,
    physical: &PhysicalKeyboard,
    keyboard: &mut ButtonInput<KeyCode>,
) {
    if cfg.peace_keys_are_default() {
        return;
    }

    let sprint_source = soft_play_bindings::peace_key_code(cfg.key_sprint);
    let sprint_state = if cfg.key_sprint == shared::local_settings::PeaceKey::LeftShift {
        // An unrelated remap must not remove the existing either-Shift default.
        either_key_state(
            physical,
            soft_play_bindings::SPRINT_LEFT,
            soft_play_bindings::SPRINT_RIGHT,
        )
    } else {
        key_state(physical, sprint_source)
    };

    let bindings = [
        (
            soft_play_bindings::MOVE_UP,
            soft_play_bindings::peace_key_code(cfg.key_move_up),
            key_state(
                physical,
                soft_play_bindings::peace_key_code(cfg.key_move_up),
            ),
        ),
        (
            soft_play_bindings::MOVE_DOWN,
            soft_play_bindings::peace_key_code(cfg.key_move_down),
            key_state(
                physical,
                soft_play_bindings::peace_key_code(cfg.key_move_down),
            ),
        ),
        (
            soft_play_bindings::MOVE_LEFT,
            soft_play_bindings::peace_key_code(cfg.key_move_left),
            key_state(
                physical,
                soft_play_bindings::peace_key_code(cfg.key_move_left),
            ),
        ),
        (
            soft_play_bindings::MOVE_RIGHT,
            soft_play_bindings::peace_key_code(cfg.key_move_right),
            key_state(
                physical,
                soft_play_bindings::peace_key_code(cfg.key_move_right),
            ),
        ),
        (
            soft_play_bindings::JUMP,
            soft_play_bindings::peace_key_code(cfg.key_jump),
            key_state(physical, soft_play_bindings::peace_key_code(cfg.key_jump)),
        ),
        (soft_play_bindings::SPRINT_LEFT, sprint_source, sprint_state),
        (
            soft_play_bindings::INTERACT,
            soft_play_bindings::peace_key_code(cfg.key_use),
            key_state(physical, soft_play_bindings::peace_key_code(cfg.key_use)),
        ),
        (
            soft_play_bindings::INVENTORY,
            soft_play_bindings::peace_key_code(cfg.key_satchel),
            key_state(
                physical,
                soft_play_bindings::peace_key_code(cfg.key_satchel),
            ),
        ),
        (
            soft_play_bindings::HIDE_GUIDANCE,
            soft_play_bindings::peace_key_code(cfg.key_hide),
            key_state(physical, soft_play_bindings::peace_key_code(cfg.key_hide)),
        ),
        (
            soft_play_bindings::ALLOCATE,
            soft_play_bindings::peace_key_code(cfg.key_allocate),
            key_state(
                physical,
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
        write_key_state(keyboard, canonical, state);
    }
}

fn track_last_pointer_kind(
    mut last: ResMut<LastPointerKind>,
    mut mouse: MessageReader<MouseButtonInput>,
    mut touch: MessageReader<TouchInput>,
    gamepads: Query<&Gamepad>,
) {
    if touch.read().next().is_some() {
        *last = LastPointerKind::Touch;
        return;
    }
    if mouse.read().next().is_some() {
        *last = LastPointerKind::Mouse;
        return;
    }
    // Any connected pad activity → Pad (hot-plug: first query hit is P1).
    for gamepad in &gamepads {
        let active_btn = [
            GamepadButton::South,
            GamepadButton::East,
            GamepadButton::West,
            GamepadButton::North,
            GamepadButton::Start,
            GamepadButton::Select,
            GamepadButton::LeftTrigger,
            GamepadButton::RightTrigger,
            GamepadButton::LeftThumb,
            GamepadButton::RightThumb,
        ]
        .iter()
        .any(|button| gamepad.pressed(*button) || gamepad.just_pressed(*button));
        if active_btn {
            *last = LastPointerKind::Pad;
            return;
        }
        let lx = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
        let ly = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);
        if lx.abs() > STICK_DEADZONE || ly.abs() > STICK_DEADZONE {
            *last = LastPointerKind::Pad;
            return;
        }
        break; // Player 1 = first connected
    }
}

pub(crate) fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    settings: Res<LocalSettingsState>,
    edit: Option<Res<crate::hud_edit_mode::HudEditMode>>,
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
    for gamepad in &gamepads {
        pad = Some(gamepad);
        let lx = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
        let ly = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);
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
        .map(|g| pad_use_just_pressed(g, cfg.gamepad_south_use))
        .unwrap_or(false);
    let kb_held = keyboard.pressed(soft_play_bindings::INTERACT);
    let pad_held = pad
        .map(|g| pad_use_held(g, cfg.gamepad_south_use))
        .unwrap_or(false);
    // One Use edge — keyboard E and South alias must not double-fire same frame.
    // Q17: edit mode kills Use. WASD, jump, and sprint above stay live.
    // The held level uses that same gate and the same two sources.
    let editing = edit.as_ref().is_some_and(|mode| mode.active);
    player_input.interact = if editing {
        false
    } else {
        use_edge(kb_use, pad_use)
    };
    player_input.interact_held = if editing { false } else { kb_held || pad_held };

    let kb_jump = keyboard.just_pressed(soft_play_bindings::JUMP);
    let pad_jump = pad
        .map(|g| pad_jump_just_pressed(g, cfg.gamepad_south_use))
        .unwrap_or(false);
    player_input.jump = kb_jump || pad_jump;

    let kb_sprint = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let pad_sprint = pad
        .map(|g| pad_sprint_pressed(g, cfg.sprint_mode.as_str()))
        .unwrap_or(false);
    player_input.sprint = kb_sprint || pad_sprint;

    player_input.pause_toggle = if editing {
        false
    } else {
        pad.map(|g| g.just_pressed(GamepadButton::Start))
            .unwrap_or(false)
    };
    player_input.sheet_q = if editing {
        false
    } else {
        pad.map(|g| g.just_pressed(GamepadButton::West))
            .unwrap_or(false)
    };
    player_input.sheet_l = pad
        .map(|g| g.just_pressed(GamepadButton::North))
        .unwrap_or(false);
}

/// Collapse keyboard+pad Use edges into one verb fire.
pub fn use_edge(keyboard_use: bool, pad_use: bool) -> bool {
    keyboard_use || pad_use
}

/// Pure: pad Use when South maps to Use.
pub fn pad_use_just_pressed(gamepad: &Gamepad, gamepad_south_use: bool) -> bool {
    if !gamepad_south_use {
        return false;
    }
    gamepad.just_pressed(GamepadButton::South)
}

/// Pure: pad Use held when South maps to Use. Same button as [`pad_use_just_pressed`].
fn pad_use_held(gamepad: &Gamepad, gamepad_south_use: bool) -> bool {
    if !gamepad_south_use {
        return false;
    }
    gamepad.pressed(GamepadButton::South)
}

/// Pure helper for unit tests — South maps to Use iff enabled.
pub fn south_is_use(gamepad_south_use: bool) -> bool {
    gamepad_south_use
}

/// Jump on pad: when South is Use, jump is LeftTrigger or LeftBumper (LB).
/// When South is not Use, South remains jump (legacy escape hatch).
pub fn pad_jump_just_pressed(gamepad: &Gamepad, gamepad_south_use: bool) -> bool {
    if gamepad_south_use {
        // Bevy 0.15: LeftTrigger is still the bumper; LeftTrigger2 is the analog trigger.
        gamepad.just_pressed(GamepadButton::LeftTrigger)
    } else {
        gamepad.just_pressed(GamepadButton::South)
    }
}

/// Sprint hold from pad per sprint_mode (stick / trigger / key).
/// `key` = keyboard Shift only (no pad sprint).
/// `stick` = LeftThumb (stick click).
/// `trigger` = LeftTrigger2 (LT analog digital).
pub fn pad_sprint_pressed(gamepad: &Gamepad, sprint_mode: &str) -> bool {
    match sprint_mode.trim().to_ascii_lowercase().as_str() {
        "stick" => gamepad.pressed(GamepadButton::LeftThumb),
        "trigger" => gamepad.pressed(GamepadButton::LeftTrigger2),
        _ => false, // key = Shift only
    }
}

/// Resolve overlay visibility from settings + last pointer (pure).
pub fn overlay_sticks_visible(settings: &LocalSettings, last: LastPointerKind) -> bool {
    settings.resolve_on_screen_sticks(last == LastPointerKind::Touch)
}

/// One row of a `--script` timeline.
///
/// `seconds` is [`Time::elapsed_secs_f64`]. From that time on, `move_x` /
/// `move_y` are [`PlayerInput::movement`] and `use_held` is the Use level.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScriptSample {
    pub seconds: f64,
    pub move_x: f32,
    pub move_y: f32,
    pub use_held: bool,
}

/// `--script <path>` timeline. `main` inserts this only when that argument is present.
///
/// Plain text. Blank lines and `#` comments are skipped. Every other line is
/// `seconds move_x move_y use_held`, split on whitespace, commas, or both.
/// `use_held` is the token `0` or `1`. Each line holds until the next one.
/// Before the first timestamp the three columns are 0.
///
/// A missing file or a bad line logs one warning and leaves device input.
///
/// Once [`Time::elapsed_secs_f64`] passes the last row's timestamp, one
/// [`bevy::window::WindowCloseRequested`] is written for the primary window.
/// Further frames do not write another.
#[derive(Resource, Debug)]
pub struct ScriptTimeline {
    path: PathBuf,
    phase: ScriptPhase,
    /// Previous frame's `use_held` column. The Use edge is the 0→1 step.
    prev_held: bool,
    /// Latched after the one close past the last row.
    close_sent: bool,
}

#[derive(Debug)]
enum ScriptPhase {
    Pending,
    Live(Vec<ScriptSample>),
    Fallback,
}

impl ScriptTimeline {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            phase: ScriptPhase::Pending,
            prev_held: false,
            close_sent: false,
        }
    }

    /// `Ok(true)` once a timeline is live. `Ok(false)` after the one warning.
    fn ensure_loaded(&mut self) -> bool {
        match self.phase {
            ScriptPhase::Live(_) => true,
            ScriptPhase::Fallback => false,
            ScriptPhase::Pending => {
                let path = self.path.clone();
                match read_script_file(&path) {
                    Ok(samples) => {
                        self.phase = ScriptPhase::Live(samples);
                        true
                    }
                    Err(err) => {
                        warn!("--script {}: {err}", path.display());
                        self.phase = ScriptPhase::Fallback;
                        false
                    }
                }
            }
        }
    }
}

fn read_script_file(path: &Path) -> Result<Vec<ScriptSample>, String> {
    let text = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
    parse_script_timeline(&text)
}

/// Parse a timeline. Bad lines are `Err`; this does not panic.
pub fn parse_script_timeline(text: &str) -> Result<Vec<ScriptSample>, String> {
    let mut samples = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = trimmed
            .split(|c: char| c.is_whitespace() || c == ',')
            .filter(|part| !part.is_empty())
            .collect();
        if fields.len() != 4 {
            return Err(format!(
                "line {line_no}: expected seconds, move_x, move_y, use_held (0/1)"
            ));
        }
        let seconds = parse_finite_f64(fields[0])
            .ok_or_else(|| format!("line {line_no}: seconds is not a finite number"))?;
        let move_x = parse_finite_f32(fields[1])
            .ok_or_else(|| format!("line {line_no}: move_x is not a finite number"))?;
        let move_y = parse_finite_f32(fields[2])
            .ok_or_else(|| format!("line {line_no}: move_y is not a finite number"))?;
        let use_held = match fields[3] {
            "0" => false,
            "1" => true,
            _ => return Err(format!("line {line_no}: use_held must be 0 or 1")),
        };
        samples.push(ScriptSample {
            seconds,
            move_x,
            move_y,
            use_held,
        });
    }
    Ok(samples)
}

fn parse_finite_f64(text: &str) -> Option<f64> {
    let value: f64 = text.parse().ok()?;
    value.is_finite().then_some(value)
}

fn parse_finite_f32(text: &str) -> Option<f32> {
    let value: f32 = text.parse().ok()?;
    value.is_finite().then_some(value)
}

/// Greatest timestamp. That row is the end of the timeline.
fn last_row_secs(samples: &[ScriptSample]) -> Option<f64> {
    samples.iter().map(|sample| sample.seconds).reduce(f64::max)
}

/// Latest line with `seconds <= now`. Equal timestamps: the later line wins.
fn sample_at(samples: &[ScriptSample], now: f64) -> Option<&ScriptSample> {
    let mut best: Option<&ScriptSample> = None;
    for sample in samples {
        if sample.seconds <= now {
            let replace = best.is_none_or(|prev| sample.seconds >= prev.seconds);
            if replace {
                best = Some(sample);
            }
        }
    }
    best
}

/// Device reads run first. This writes movement and Use for the loaded timeline.
///
/// `interact` is the 0→1 rise of `use_held`, one frame per rise.
/// Edit mode clears `interact` and `interact_held` the same way device Use does.
///
/// When the clock passes the last row, write one [`bevy::window::WindowCloseRequested`]
/// for the [`bevy::window::PrimaryWindow`] entity — the same message winit sends
/// on the close button — then latch. No `AppExit` here. A missing primary window
/// waits; it does not latch.
pub(crate) fn apply_script_timeline(
    time: Res<Time>,
    edit: Option<Res<crate::hud_edit_mode::HudEditMode>>,
    mut script: ResMut<ScriptTimeline>,
    mut player_input: ResMut<PlayerInput>,
    primary: Query<Entity, With<bevy::window::PrimaryWindow>>,
    mut close: MessageWriter<bevy::window::WindowCloseRequested>,
) {
    if !script.ensure_loaded() {
        return;
    }
    let now = time.elapsed_secs_f64();
    let (move_x, move_y, held, past_last) = {
        let ScriptPhase::Live(samples) = &script.phase else {
            return;
        };
        let past_last = !script.close_sent && last_row_secs(samples).is_some_and(|end| now > end);
        let (move_x, move_y, held) = match sample_at(samples, now) {
            Some(sample) => (sample.move_x, sample.move_y, sample.use_held),
            None => (0.0, 0.0, false),
        };
        (move_x, move_y, held, past_last)
    };
    let edge = !script.prev_held && held;
    script.prev_held = held;
    player_input.movement = Vec2::new(move_x, move_y);
    let editing = edit.as_ref().is_some_and(|mode| mode.active);
    player_input.interact = if editing { false } else { edge };
    player_input.interact_held = if editing { false } else { held };
    if past_last {
        if let Ok(window) = primary.single() {
            close.write(bevy::window::WindowCloseRequested { window });
            script.close_sent = true;
        }
    }
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

        apply_peace_keyboard(&settings, &PhysicalKeyboard::default(), &mut keyboard);

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
        let remapped_sources = [
            KeyCode::ArrowUp,
            KeyCode::KeyJ,
            KeyCode::ControlLeft,
            KeyCode::KeyF,
            KeyCode::KeyB,
            KeyCode::KeyV,
            KeyCode::KeyN,
        ];
        let physical = PhysicalKeyboard {
            pressed: remapped_sources.into_iter().collect(),
            just_pressed: remapped_sources.into_iter().collect(),
            just_released: HashSet::new(),
        };

        apply_peace_keyboard(&settings, &physical, &mut keyboard);

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
        let physical = PhysicalKeyboard {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: [KeyCode::KeyF].into_iter().collect(),
        };

        apply_peace_keyboard(&settings, &physical, &mut keyboard);

        assert!(!keyboard.pressed(KeyCode::KeyE));
        assert!(keyboard.just_released(KeyCode::KeyE));
        assert!(!keyboard.just_released(KeyCode::KeyF));
    }

    #[test]
    fn remapped_hold_survives_after_source_is_consumed() {
        let mut settings = LocalSettings::peace_defaults();
        settings.key_move_up = PeaceKey::ArrowUp;
        let mut physical = PhysicalKeyboard {
            pressed: [KeyCode::ArrowUp].into_iter().collect(),
            just_pressed: [KeyCode::ArrowUp].into_iter().collect(),
            just_released: HashSet::new(),
        };
        let mut keyboard = ButtonInput::default();
        keyboard.press(KeyCode::ArrowUp);

        apply_peace_keyboard(&settings, &physical, &mut keyboard);
        assert!(keyboard.pressed(KeyCode::KeyW));
        assert!(keyboard.just_pressed(KeyCode::KeyW));
        assert!(!keyboard.pressed(KeyCode::ArrowUp));

        physical.just_pressed.clear();
        keyboard.clear();
        apply_peace_keyboard(&settings, &physical, &mut keyboard);
        assert!(keyboard.pressed(KeyCode::KeyW));
        assert!(!keyboard.just_pressed(KeyCode::KeyW));
        assert!(!keyboard.pressed(KeyCode::ArrowUp));
    }

    fn input_app() -> App {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .insert_resource(LocalSettingsState {
                inner: LocalSettings::peace_defaults(),
                dirty: false,
            })
            .insert_resource(PlayerInput::default())
            .init_resource::<ButtonInput<KeyCode>>()
            .add_systems(Update, handle_player_input);
        app
    }

    /// CARD SCRIPT-HOLD-TEND-1 — E fills the held level beside the just-pressed edge.
    /// Q17 edit mode clears both.
    #[test]
    fn interact_held_tracks_use_key_and_edit_mode_clears_it() {
        let mut app = input_app();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.update();
        {
            let input = app.world().resource::<PlayerInput>();
            assert!(input.interact, "press frame is the Use edge");
            assert!(input.interact_held, "press frame is also held");
        }

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
        app.update();
        {
            let input = app.world().resource::<PlayerInput>();
            assert!(!input.interact, "held frame drops the edge");
            assert!(input.interact_held, "key still down");
        }

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyE);
        app.update();
        {
            let input = app.world().resource::<PlayerInput>();
            assert!(!input.interact);
            assert!(!input.interact_held);
        }

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        let mut edit = crate::hud_edit_mode::HudEditMode::default();
        edit.active = true;
        app.insert_resource(edit);
        app.update();
        let input = app.world().resource::<PlayerInput>();
        assert!(!input.interact, "edit mode kills the Use edge");
        assert!(!input.interact_held, "edit mode kills the held level");
    }

    /// Remap writes the physical Use key onto canonical E before this system reads it.
    #[test]
    fn remapped_use_hold_sets_interact_held() {
        let mut settings = LocalSettings::peace_defaults();
        settings.key_use = PeaceKey::F;
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .insert_resource(LocalSettingsState {
                inner: settings,
                dirty: false,
            })
            .insert_resource(PlayerInput::default())
            .init_resource::<ButtonInput<KeyCode>>()
            .insert_resource(PhysicalKeyboard {
                pressed: [KeyCode::KeyF].into_iter().collect(),
                just_pressed: [KeyCode::KeyF].into_iter().collect(),
                just_released: HashSet::new(),
            })
            .add_systems(Update, (remap_peace_keyboard, handle_player_input).chain());

        app.update();
        {
            let input = app.world().resource::<PlayerInput>();
            assert!(input.interact);
            assert!(input.interact_held);
        }
        assert!(app
            .world()
            .resource::<ButtonInput<KeyCode>>()
            .pressed(KeyCode::KeyE));

        app.world_mut()
            .resource_mut::<PhysicalKeyboard>()
            .just_pressed
            .clear();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
        app.update();
        let input = app.world().resource::<PlayerInput>();
        assert!(!input.interact, "remap hold is not a new edge");
        assert!(input.interact_held, "remapped Use stays held");
    }

    /// First connected pad's existing South Use button. No new pad source.
    #[test]
    fn first_gamepad_use_hold_sets_interact_held() {
        let mut app = input_app();
        let mut pad = Gamepad::default();
        pad.digital_mut().press(GamepadButton::South);
        app.world_mut().spawn(pad);
        app.update();
        {
            let input = app.world().resource::<PlayerInput>();
            assert!(input.interact);
            assert!(input.interact_held);
        }

        {
            let mut pads = app.world_mut().query::<&mut Gamepad>();
            pads.single_mut(app.world_mut())
                .unwrap()
                .digital_mut()
                .clear();
        }
        app.update();
        {
            let input = app.world().resource::<PlayerInput>();
            assert!(!input.interact, "pad hold drops the edge");
            assert!(input.interact_held, "South still down");
        }

        app.world_mut()
            .resource_mut::<LocalSettingsState>()
            .inner
            .gamepad_south_use = false;
        app.update();
        let input = app.world().resource::<PlayerInput>();
        assert!(!input.interact);
        assert!(
            !input.interact_held,
            "South is not Use when gamepad_south_use is off"
        );
    }

    #[test]
    fn script_timeline_parse_reads_inline_steps() {
        let text = "\
# east, then a tie at 1.5
\n\
0, 1, 0, 0\n\
\n\
1.5  -0.25\t0.5, 1\n\
1.5, 0.5, 0, 0\n";
        let samples = parse_script_timeline(text).expect("inline timeline");
        assert_eq!(
            samples,
            vec![
                ScriptSample {
                    seconds: 0.0,
                    move_x: 1.0,
                    move_y: 0.0,
                    use_held: false,
                },
                ScriptSample {
                    seconds: 1.5,
                    move_x: -0.25,
                    move_y: 0.5,
                    use_held: true,
                },
                ScriptSample {
                    seconds: 1.5,
                    move_x: 0.5,
                    move_y: 0.0,
                    use_held: false,
                },
            ]
        );
        assert!(sample_at(&samples, -0.1).is_none());
        assert_eq!(sample_at(&samples, 1.0).unwrap().move_x, 1.0);
        let tied = sample_at(&samples, 1.5).unwrap();
        assert_eq!(tied.move_x, 0.5);
        assert!(!tied.use_held);
        assert!(parse_script_timeline("").unwrap().is_empty());
        assert!(parse_script_timeline("# only\n\n").unwrap().is_empty());
    }

    #[test]
    fn script_timeline_parse_bad_line_is_err() {
        for bad in [
            "0 0 0",
            "0 0 0 2",
            "0 0 0 1 extra",
            "nope",
            "0 0 NaN 0",
            "0 0 0 1.0",
        ] {
            let text = format!("0 0 0 0\n{bad}\n");
            let parsed = parse_script_timeline(&text);
            assert!(parsed.is_err(), "{bad} should be Err, got {parsed:?}");
        }
        let err = parse_script_timeline("0 0 0 0\nbad").unwrap_err();
        assert!(err.contains("line 2"), "{err}");
    }

    fn write_timeline(name: &str, body: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("powrush-script-hour-drive-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(name);
        std::fs::write(&path, body).unwrap();
        path
    }

    fn widen_clock(app: &mut App) {
        app.world_mut()
            .resource_mut::<Time<bevy::time::Virtual>>()
            .set_max_delta(std::time::Duration::from_secs(2));
    }

    fn set_dt(app: &mut App, secs: f64) {
        *app.world_mut()
            .resource_mut::<bevy::time::TimeUpdateStrategy>() =
            bevy::time::TimeUpdateStrategy::ManualDuration(std::time::Duration::from_secs_f64(
                secs,
            ));
    }

    fn device_app(script: Option<PathBuf>) -> App {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                std::time::Duration::ZERO,
            ))
            .insert_resource(LocalSettingsState {
                inner: LocalSettings::peace_defaults(),
                dirty: false,
            })
            .add_message::<KeyboardInput>()
            .add_message::<MouseButtonInput>()
            .add_message::<TouchInput>()
            .init_resource::<ButtonInput<KeyCode>>()
            .add_plugins(InputPlugin);
        if let Some(path) = script {
            app.insert_resource(ScriptTimeline::from_path(path));
        }
        widen_clock(&mut app);
        app
    }

    fn predicted_elapsed(dts: &[f64]) -> Vec<f64> {
        let mut t = 0.0;
        dts.iter()
            .enumerate()
            .map(|(i, dt)| {
                if i > 0 {
                    t += *dt;
                }
                t
            })
            .collect()
    }

    #[derive(Clone, Copy, Debug)]
    struct UseSnap {
        elapsed: f64,
        interact: bool,
        interact_held: bool,
    }

    fn use_snap(app: &App) -> UseSnap {
        let input = app.world().resource::<PlayerInput>();
        UseSnap {
            elapsed: app.world().resource::<Time>().elapsed_secs_f64(),
            interact: input.interact,
            interact_held: input.interact_held,
        }
    }

    /// Keyboard Use for one frame. `edge` is the just-pressed frame.
    fn set_use_key(app: &mut App, held: bool, edge: bool) {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        if held && edge {
            keys.release(KeyCode::KeyE);
            keys.clear();
            keys.press(KeyCode::KeyE);
        } else if held {
            keys.press(KeyCode::KeyE);
            keys.clear();
        } else {
            keys.release(KeyCode::KeyE);
            keys.clear();
        }
    }

    fn run_script_use(body: &str, name: &str, dts: &[f64]) -> Vec<UseSnap> {
        let path = write_timeline(name, body);
        let mut app = device_app(Some(path));
        let mut snaps = Vec::new();
        for (i, dt) in dts.iter().enumerate() {
            set_dt(&mut app, *dt);
            app.update();
            let snap = use_snap(&app);
            let want = predicted_elapsed(dts)[i];
            assert!(
                (snap.elapsed - want).abs() < 1e-6,
                "script clock {snap:?} wanted {want}"
            );
            assert_eq!(
                app.world().resource::<PlayerInput>().movement,
                Vec2::ZERO,
                "use rows in this timeline do not move"
            );
            snaps.push(snap);
        }
        snaps
    }

    fn run_keyboard_use(body: &str, dts: &[f64]) -> Vec<UseSnap> {
        let samples = parse_script_timeline(body).unwrap();
        let mut app = device_app(None);
        let mut prev = false;
        let mut snaps = Vec::new();
        let elapsed = predicted_elapsed(dts);
        for (i, dt) in dts.iter().enumerate() {
            let held = sample_at(&samples, elapsed[i]).is_some_and(|sample| sample.use_held);
            set_dt(&mut app, *dt);
            set_use_key(&mut app, held, held && !prev);
            prev = held;
            app.update();
            let snap = use_snap(&app);
            assert!(
                (snap.elapsed - elapsed[i]).abs() < 1e-6,
                "keyboard clock {snap:?} wanted {}",
                elapsed[i]
            );
            snaps.push(snap);
        }
        snaps
    }

    fn assert_script_matches_keyboard(body: &str, name: &str, dts: &[f64]) {
        let scripted = run_script_use(body, name, dts);
        let keyboard = run_keyboard_use(body, dts);
        assert_eq!(scripted.len(), keyboard.len());
        for (index, (scripted, keyboard)) in scripted.iter().zip(keyboard.iter()).enumerate() {
            assert!(
                (scripted.elapsed - keyboard.elapsed).abs() < 1e-6,
                "frame {index} clock"
            );
            assert_eq!(
                scripted.interact, keyboard.interact,
                "frame {index} interact at t={}",
                scripted.elapsed
            );
            assert_eq!(
                scripted.interact_held, keyboard.interact_held,
                "frame {index} interact_held at t={}",
                scripted.elapsed
            );
        }
    }

    /// CARD SCRIPT-HOLD-TEND-1's harness is private in `first_harvest_epiphany`.
    /// The keyboard path there maps this Use sequence to take+tend `(1, 1)` when
    /// the hold lasts `TEND_HOLD` (0.42) or more, and to take `(1, 0)` when the
    /// release lands sooner. This locks the same `PlayerInput` sequence.
    #[test]
    fn script_timeline_use_matches_keyboard_hold_and_release() {
        let tend = crate::first_harvest_epiphany::TEND_HOLD;
        assert!((tend - 0.42).abs() < 1e-9);

        let hold = "\
0 0 0 0
1.0 0 0 1
";
        let hold_dts = [0.0, 1.0, 0.5];
        assert_script_matches_keyboard(hold, "hold.txt", &hold_dts);
        let hold_snaps = run_script_use(hold, "hold-read.txt", &hold_dts);
        let edges: Vec<_> = hold_snaps.iter().filter(|snap| snap.interact).collect();
        assert_eq!(edges.len(), 1, "one rise, one interact edge");
        assert!((edges[0].elapsed - 1.0).abs() < 1e-6);
        let span = hold_snaps.last().unwrap().elapsed - edges[0].elapsed;
        assert!(span >= tend, "hold span {span} must reach TEND_HOLD");
        assert!(hold_snaps.last().unwrap().interact_held);
        assert!(!hold_snaps.last().unwrap().interact);

        let short = "\
0 0 0 0
1.0 0 0 1
1.2 0 0 0
";
        let short_dts = [0.0, 1.0, 0.2];
        assert_script_matches_keyboard(short, "short.txt", &short_dts);
        let short_snaps = run_script_use(short, "short-read.txt", &short_dts);
        let short_edges: Vec<_> = short_snaps.iter().filter(|snap| snap.interact).collect();
        assert_eq!(short_edges.len(), 1, "short hold still has one press edge");
        let short_span = short_snaps.last().unwrap().elapsed - short_edges[0].elapsed;
        assert!(
            short_span < tend,
            "short span {short_span} stays under TEND_HOLD"
        );
        assert!(!short_snaps.last().unwrap().interact_held);
        assert!(!short_snaps.last().unwrap().interact);

        let again = "\
0 0 0 0
1 0 0 1
2 0 0 0
3 0 0 1
3.5 0 0 1
";
        let again_dts = [0.0, 1.0, 1.0, 1.0, 0.5];
        assert_script_matches_keyboard(again, "again.txt", &again_dts);
        let again_snaps = run_script_use(again, "again-read.txt", &again_dts);
        let again_edges: Vec<_> = again_snaps
            .iter()
            .filter(|snap| snap.interact)
            .map(|snap| snap.elapsed)
            .collect();
        assert_eq!(again_edges.len(), 2, "each 0→1 rise is one edge");
        assert!((again_edges[0] - 1.0).abs() < 1e-6);
        assert!((again_edges[1] - 3.0).abs() < 1e-6);
    }

    #[test]
    fn script_timeline_walk_overrides_movement() {
        let path = write_timeline("walk.txt", "# east then north\n1.0, 1, 0, 0\n2.0 0, 1, 0\n");
        let mut app = device_app(Some(path));
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::KeyW);
            keys.press(KeyCode::Space);
            keys.press(KeyCode::ShiftLeft);
        }
        let steps = [
            (0.0, 0.0, Vec2::ZERO),
            (1.0, 1.0, Vec2::new(1.0, 0.0)),
            (1.0, 2.0, Vec2::new(0.0, 1.0)),
        ];
        for (dt, want_t, want_move) in steps {
            set_dt(&mut app, dt);
            app.update();
            let now = app.world().resource::<Time>().elapsed_secs_f64();
            let input = app.world().resource::<PlayerInput>();
            assert!((now - want_t).abs() < 1e-6, "t={now}");
            assert_eq!(input.movement, want_move, "script replaces WASD at t={now}");
            assert!(input.jump, "script leaves jump");
            assert!(input.sprint, "script leaves sprint");
            assert!(!input.interact);
            assert!(!input.interact_held);
        }
    }

    /// Past the last row: one `WindowCloseRequested` for the primary window.
    /// On that row, and before it, the count stays 0. Further steps stay at 1.
    #[test]
    fn script_timeline_end_sends_one_window_close() {
        let path = write_timeline("end-close.txt", "0 0 0 0\n1.0 1 0 0\n2.0 0 1 0\n");
        let mut app = device_app(Some(path));
        let window = app.world_mut().spawn(bevy::window::PrimaryWindow).id();
        let mut cursor =
            bevy::ecs::message::MessageCursor::<bevy::window::WindowCloseRequested>::default();
        let mut total = 0usize;
        let mut saw_before = false;
        let mut saw_on_row = false;
        let mut saw_past = false;
        let end = 2.0;
        for dt in [0.0, 1.0, 1.0, 0.25, 0.25, 0.5, 1.0] {
            set_dt(&mut app, dt);
            app.update();
            let now = app.world().resource::<Time>().elapsed_secs_f64();
            let frames: Vec<_> = {
                let messages = app
                    .world()
                    .resource::<bevy::ecs::message::Messages<bevy::window::WindowCloseRequested>>();
                cursor.read(messages).map(|msg| msg.window).collect()
            };
            if now < end - 1e-6 {
                assert!(
                    frames.is_empty(),
                    "no close before the end at t={now}, got {frames:?}"
                );
                saw_before = true;
            } else if (now - end).abs() < 1e-6 {
                assert!(
                    frames.is_empty(),
                    "on the last row is not past it (t={now})"
                );
                assert_eq!(
                    app.world().resource::<PlayerInput>().movement,
                    Vec2::new(0.0, 1.0),
                    "the last row still drives movement"
                );
                saw_on_row = true;
            } else {
                assert!(now > end, "t={now}");
                saw_past = true;
            }
            total += frames.len();
            for entity in &frames {
                assert_eq!(*entity, window, "close targets the primary window");
            }
        }
        assert!(saw_before, "stepped frames before the last row");
        assert!(saw_on_row, "landed on the last row");
        assert!(saw_past, "kept stepping past the end");
        assert_eq!(total, 1, "one close, then the latch holds");
        assert!(app.world().resource::<ScriptTimeline>().close_sent);
    }

    #[test]
    fn script_timeline_use_edit_gate_matches_keyboard() {
        let body = "0, 1, 0, 1\n";
        let mut scripted = device_app(Some(write_timeline("edit.txt", body)));
        let mut keyboard = device_app(None);
        let mut edit = crate::hud_edit_mode::HudEditMode::default();
        edit.active = true;
        scripted.insert_resource(edit.clone());
        keyboard.insert_resource(edit);
        set_dt(&mut scripted, 0.0);
        set_dt(&mut keyboard, 0.0);
        set_use_key(&mut keyboard, true, true);
        scripted.update();
        keyboard.update();
        for app in [&scripted, &keyboard] {
            let input = app.world().resource::<PlayerInput>();
            assert!(!input.interact, "edit mode kills the Use edge");
            assert!(!input.interact_held, "edit mode kills the held level");
        }
        assert_eq!(
            scripted.world().resource::<PlayerInput>().movement,
            Vec2::new(1.0, 0.0),
            "edit mode does not clear scripted move"
        );

        scripted
            .world_mut()
            .resource_mut::<crate::hud_edit_mode::HudEditMode>()
            .active = false;
        keyboard
            .world_mut()
            .resource_mut::<crate::hud_edit_mode::HudEditMode>()
            .active = false;
        set_dt(&mut scripted, 0.0);
        set_dt(&mut keyboard, 0.0);
        set_use_key(&mut keyboard, true, false);
        scripted.update();
        keyboard.update();
        for app in [&scripted, &keyboard] {
            let input = app.world().resource::<PlayerInput>();
            assert!(
                !input.interact,
                "the rise already happened; leaving edit does not re-fire it"
            );
            assert!(input.interact_held);
        }

        let mut open = device_app(Some(write_timeline("edit-open.txt", body)));
        set_dt(&mut open, 0.0);
        open.update();
        let input = open.world().resource::<PlayerInput>();
        assert!(input.interact, "without edit, the rise is the Use edge");
        assert!(input.interact_held);
        assert_eq!(input.movement, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn script_timeline_absent_leaves_device_input() {
        let mut app = device_app(None);
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::KeyW);
            keys.press(KeyCode::KeyE);
        }
        set_dt(&mut app, 0.0);
        app.update();
        let input = app.world().resource::<PlayerInput>();
        assert_eq!(input.movement, Vec2::new(0.0, 1.0));
        assert!(input.interact);
        assert!(input.interact_held);
        assert!(app.world().get_resource::<ScriptTimeline>().is_none());
    }

    #[test]
    fn script_timeline_bad_file_warns_once_and_keeps_human_input() {
        assert_fallback_keeps_wasd(PathBuf::from(
            "/tmp/powrush-script-hour-drive-missing-no-such-file",
        ));
        let bad = write_timeline("bad.txt", "0 0 0 0\nnot-a-row\n");
        assert_fallback_keeps_wasd(bad);
    }

    fn assert_fallback_keeps_wasd(path: PathBuf) {
        let count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let counter = ScriptWarnCounter(std::sync::Arc::clone(&count));
        let mut app = device_app(Some(path));
        // The log subscriber is thread-local. The multi-thread executor would
        // emit the warning on a worker, where this subscriber is not installed.
        app.edit_schedule(Update, |schedule| {
            schedule.set_executor(bevy::ecs::schedule::SingleThreadedExecutor::new());
        });
        {
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::KeyW);
        }
        tracing::subscriber::with_default(counter, || {
            set_dt(&mut app, 0.0);
            app.update();
            set_dt(&mut app, 0.0);
            app.update();
        });
        assert_eq!(
            count.load(std::sync::atomic::Ordering::Relaxed),
            1,
            "a bad or missing script logs one warning"
        );
        let input = app.world().resource::<PlayerInput>();
        assert_eq!(input.movement, Vec2::new(0.0, 1.0));
        assert!(!input.interact);
        assert!(matches!(
            app.world().resource::<ScriptTimeline>().phase,
            ScriptPhase::Fallback
        ));
    }

    struct ScriptWarnCounter(std::sync::Arc<std::sync::atomic::AtomicUsize>);

    impl tracing::Subscriber for ScriptWarnCounter {
        fn enabled(&self, metadata: &tracing::Metadata<'_>) -> bool {
            *metadata.level() == tracing::Level::WARN && metadata.target().ends_with("::input")
        }

        fn new_span(&self, _span: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::span::Id::from_u64(1)
        }

        fn record(&self, _span: &tracing::span::Id, _values: &tracing::span::Record<'_>) {}

        fn record_follows_from(&self, _span: &tracing::span::Id, _follows: &tracing::span::Id) {}

        fn event(&self, event: &tracing::Event<'_>) {
            let meta = event.metadata();
            if *meta.level() == tracing::Level::WARN && meta.target().ends_with("::input") {
                self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }

        fn enter(&self, _span: &tracing::span::Id) {}

        fn exit(&self, _span: &tracing::span::Id) {}
    }
}
