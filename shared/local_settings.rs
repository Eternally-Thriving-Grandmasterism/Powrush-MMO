//! D2 Local settings — persist beside house JSON (v23.2.64)
//!
//! `powrush_settings.json` next to house JSON in the OS user-data dir
//! (or `POWRUSH_USER_DIR`). Cwd `data/powrush_settings.json` is adopt-only.
//! Look · Mute · Invert-Y · Hide slabs · Brightness · Text scale · Grove ·
//! Reduced motion · Rumble · Colorblind wells · LAN · Controls (I0).
//! Defaults = Peace hour / Peace desktop (sticks auto-off for mouse Title).
//! Mute on pause plate = same MasterMute flag.
//! Online stays grey — no settings toggle binds a socket / POWRUSH_NET=on.
//! LAN is a separate row (off | loopback). Default **off**. Unknown → off.
//! Loopback may use the existing F8 127.0.0.1 door only — never 0.0.0.0, never Title Online.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

pub const SETTINGS_PATH: &str = "data/powrush_settings.json";
pub const SETTINGS_SCHEMA: &str = "powrush_settings_v1";

/// Default look sensitivity (Peace hour — no boost, no damp).
pub const DEFAULT_LOOK_SENSITIVITY: f32 = 1.0;
/// Look sensitivity clamp (simple companion — no second HUD).
pub const LOOK_SENS_MIN: f32 = 0.25;
pub const LOOK_SENS_MAX: f32 = 2.0;
pub const LOOK_SENS_STEP: f32 = 0.25;

/// Default world/UI brightness multiplier (1.0 = Peace hour).
pub const DEFAULT_BRIGHTNESS: f32 = 1.0;
pub const BRIGHTNESS_MIN: f32 = 0.50;
pub const BRIGHTNESS_MAX: f32 = 1.50;
pub const BRIGHTNESS_STEP: f32 = 0.25;

/// Default UI text scale (1.0 = Peace hour). Title plate contrast law stays absolute.
pub const DEFAULT_TEXT_SCALE: f32 = 1.0;
pub const TEXT_SCALE_MIN: f32 = 0.85;
pub const TEXT_SCALE_MAX: f32 = 1.35;
pub const TEXT_SCALE_STEP: f32 = 0.05;

/// Persisted keyboard keys for INPUT_CANON's remappable Peace actions.
///
/// This intentionally stays independent of Bevy so the shared settings file remains the
/// single bind source. The client converts these values to `KeyCode` at the input boundary.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PeaceKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Space,
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    Tab,
    Enter,
    Backspace,
}

impl PeaceKey {
    /// Short Settings-plate label for a persisted physical key.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::Digit0 => "0",
            Self::Digit1 => "1",
            Self::Digit2 => "2",
            Self::Digit3 => "3",
            Self::Digit4 => "4",
            Self::Digit5 => "5",
            Self::Digit6 => "6",
            Self::Digit7 => "7",
            Self::Digit8 => "8",
            Self::Digit9 => "9",
            Self::ArrowUp => "Up",
            Self::ArrowDown => "Down",
            Self::ArrowLeft => "Left",
            Self::ArrowRight => "Right",
            Self::Space => "Space",
            Self::LeftShift => "Left Shift",
            Self::RightShift => "Right Shift",
            Self::LeftControl => "Left Ctrl",
            Self::RightControl => "Right Ctrl",
            Self::LeftAlt => "Left Alt",
            Self::RightAlt => "Right Alt",
            Self::Tab => "Tab",
            Self::Enter => "Enter",
            Self::Backspace => "Backspace",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalSettings {
    pub schema: String,
    /// Mouse look sensitivity multiplier. Default 1.0.
    #[serde(default = "default_look")]
    pub look_sensitivity: f32,
    /// Master mute. Default false (Peace hour hears). Same flag from pause Mute.
    #[serde(default)]
    pub mute: bool,
    /// Invert look Y. Default false.
    #[serde(default)]
    pub invert_y: bool,
    /// Persist default for guidance/slab hide (H still works in session).
    #[serde(default)]
    pub hide_slabs: bool,
    /// Soft brightness multiplier. Default 1.0. Does not alpha-blend Title plate.
    #[serde(default = "default_brightness")]
    pub brightness: f32,
    /// UI text scale. Default 1.0. Title contrast palette stays opaque law.
    #[serde(default = "default_text_scale")]
    pub text_scale: f32,
    /// G0.5 Grove light-gen: "off" | "light". Default **off**. Unknown → off.
    /// Same path as env `POWRUSH_GEN=light` (OR at the door — not a second gen system).
    #[serde(default = "default_grove")]
    pub grove: String,
    /// INPUT_CANON keyboard remaps. Missing fields preserve the Peace keyboard PASS.
    #[serde(default = "default_key_move_up")]
    pub key_move_up: PeaceKey,
    #[serde(default = "default_key_move_down")]
    pub key_move_down: PeaceKey,
    #[serde(default = "default_key_move_left")]
    pub key_move_left: PeaceKey,
    #[serde(default = "default_key_move_right")]
    pub key_move_right: PeaceKey,
    #[serde(default = "default_key_jump")]
    pub key_jump: PeaceKey,
    #[serde(default = "default_key_sprint")]
    pub key_sprint: PeaceKey,
    #[serde(default = "default_key_use")]
    pub key_use: PeaceKey,
    #[serde(default = "default_key_satchel")]
    pub key_satchel: PeaceKey,
    #[serde(default = "default_key_hide")]
    pub key_hide: PeaceKey,
    #[serde(default = "default_key_allocate")]
    pub key_allocate: PeaceKey,
    /// Accessibility: remove camera punch and force rumble off. Default false.
    #[serde(default)]
    pub reduced_motion: bool,
    /// Accessibility: allow gamepad rumble when reduced motion is off. Default true.
    #[serde(default = "default_true")]
    pub rumble: bool,
    /// B3 colorblind well tokens:
    /// "off" | "deuteranopia" | "protanopia" | "tritanopia" | "shape_only".
    /// Default **off** (B2 word captions alone). Unknown → off.
    /// Non-off adds ring / pip / notch / rest-bar / crack.
    #[serde(default = "default_colorblind_wells")]
    pub colorblind_wells: String,
    /// P3 LAN lab: "off" | "loopback". Default **off**. Unknown → off.
    /// Separate from Title Online / Settings Online stub. Never writes POWRUSH_NET=on.
    /// Loopback unlocks the existing F8 door on 127.0.0.1 only.
    #[serde(default = "default_lan")]
    pub lan: String,
    /// I0 on-screen sticks: "auto" | "on" | "off". Default **auto** (mouse Title stays clean).
    #[serde(default = "default_on_screen_sticks")]
    pub on_screen_sticks: String,
    /// I0 tap-to-use. Default false (tap focuses, then Use).
    #[serde(default)]
    pub tap_to_use: bool,
    /// I0 gamepad South = Use. Default true (INPUT_CANON).
    #[serde(default = "default_true")]
    pub gamepad_south_use: bool,
    /// I0 Nintendo face remap stub: "auto" (unknown → auto). Keeps South = Use.
    #[serde(default = "default_nintendo_face")]
    pub nintendo_face: String,
    /// I0 sprint: "stick" | "trigger" | "key". Default **key** (Peace desktop = Shift).
    #[serde(default = "default_sprint_mode")]
    pub sprint_mode: String,
    /// I0 soft Use cue when in range. Default true.
    #[serde(default = "default_true")]
    pub show_use_prompt: bool,
}

fn default_look() -> f32 {
    DEFAULT_LOOK_SENSITIVITY
}

fn default_brightness() -> f32 {
    DEFAULT_BRIGHTNESS
}

fn default_text_scale() -> f32 {
    DEFAULT_TEXT_SCALE
}

fn default_grove() -> String {
    "off".into()
}

fn default_key_move_up() -> PeaceKey {
    PeaceKey::W
}

fn default_key_move_down() -> PeaceKey {
    PeaceKey::S
}

fn default_key_move_left() -> PeaceKey {
    PeaceKey::A
}

fn default_key_move_right() -> PeaceKey {
    PeaceKey::D
}

fn default_key_jump() -> PeaceKey {
    PeaceKey::Space
}

fn default_key_sprint() -> PeaceKey {
    PeaceKey::LeftShift
}

fn default_key_use() -> PeaceKey {
    PeaceKey::E
}

fn default_key_satchel() -> PeaceKey {
    PeaceKey::I
}

fn default_key_hide() -> PeaceKey {
    PeaceKey::H
}

fn default_key_allocate() -> PeaceKey {
    PeaceKey::R
}

fn default_lan() -> String {
    "off".into()
}

fn default_on_screen_sticks() -> String {
    "auto".into()
}

fn default_true() -> bool {
    true
}

fn default_colorblind_wells() -> String {
    "off".into()
}

fn default_nintendo_face() -> String {
    "auto".into()
}

fn default_sprint_mode() -> String {
    "key".into()
}

impl Default for LocalSettings {
    fn default() -> Self {
        Self {
            schema: SETTINGS_SCHEMA.into(),
            look_sensitivity: DEFAULT_LOOK_SENSITIVITY,
            mute: false,
            invert_y: false,
            hide_slabs: false,
            brightness: DEFAULT_BRIGHTNESS,
            text_scale: DEFAULT_TEXT_SCALE,
            grove: default_grove(),
            key_move_up: default_key_move_up(),
            key_move_down: default_key_move_down(),
            key_move_left: default_key_move_left(),
            key_move_right: default_key_move_right(),
            key_jump: default_key_jump(),
            key_sprint: default_key_sprint(),
            key_use: default_key_use(),
            key_satchel: default_key_satchel(),
            key_hide: default_key_hide(),
            key_allocate: default_key_allocate(),
            reduced_motion: false,
            rumble: true,
            colorblind_wells: default_colorblind_wells(),
            lan: default_lan(),
            on_screen_sticks: default_on_screen_sticks(),
            tap_to_use: false,
            gamepad_south_use: true,
            nintendo_face: default_nintendo_face(),
            sprint_mode: default_sprint_mode(),
            show_use_prompt: true,
        }
    }
}

impl LocalSettings {
    pub fn peace_defaults() -> Self {
        Self::default()
    }

    /// True for a stranger/legacy settings file that has never opted into a remap.
    pub fn peace_keys_are_default(&self) -> bool {
        self.key_move_up == default_key_move_up()
            && self.key_move_down == default_key_move_down()
            && self.key_move_left == default_key_move_left()
            && self.key_move_right == default_key_move_right()
            && self.key_jump == default_key_jump()
            && self.key_sprint == default_key_sprint()
            && self.key_use == default_key_use()
            && self.key_satchel == default_key_satchel()
            && self.key_hide == default_key_hide()
            && self.key_allocate == default_key_allocate()
    }

    pub fn clamp_look(&mut self) {
        if !self.look_sensitivity.is_finite() {
            self.look_sensitivity = DEFAULT_LOOK_SENSITIVITY;
        }
        self.look_sensitivity = self.look_sensitivity.clamp(LOOK_SENS_MIN, LOOK_SENS_MAX);
    }

    pub fn clamp_brightness(&mut self) {
        if !self.brightness.is_finite() {
            self.brightness = DEFAULT_BRIGHTNESS;
        }
        self.brightness = self.brightness.clamp(BRIGHTNESS_MIN, BRIGHTNESS_MAX);
    }

    pub fn clamp_text_scale(&mut self) {
        if !self.text_scale.is_finite() {
            self.text_scale = DEFAULT_TEXT_SCALE;
        }
        self.text_scale = self.text_scale.clamp(TEXT_SCALE_MIN, TEXT_SCALE_MAX);
    }

    pub fn clamp_all(&mut self) {
        self.clamp_look();
        self.clamp_brightness();
        self.clamp_text_scale();
        self.normalize_grove();
        self.normalize_colorblind_wells();
        self.normalize_lan();
        self.normalize_controls();
    }

    /// Clamp grove to "off" | "light". Missing/unknown → off.
    pub fn normalize_grove(&mut self) {
        let t = self.grove.trim().to_ascii_lowercase();
        self.grove = match t.as_str() {
            "light" => "light".into(),
            _ => "off".into(),
        };
    }

    pub fn grove_is_light(&self) -> bool {
        self.grove.eq_ignore_ascii_case("light")
    }

    /// Cycle Grove off ↔ light (Settings plate row).
    pub fn cycle_grove(&mut self) {
        self.normalize_grove();
        if self.grove_is_light() {
            self.grove = "off".into();
        } else {
            self.grove = "light".into();
        }
    }

    pub fn toggle_reduced_motion(&mut self) {
        self.reduced_motion = !self.reduced_motion;
    }

    pub fn toggle_rumble(&mut self) {
        self.rumble = !self.rumble;
    }

    /// Rumble is always suppressed while reduced motion is enabled.
    pub fn rumble_enabled(&self) -> bool {
        self.rumble && !self.reduced_motion
    }

    /// Camera consumers read this scale instead of changing harvest feel.
    pub fn camera_punch_scale(&self) -> f32 {
        if self.reduced_motion {
            0.0
        } else {
            1.0
        }
    }

    /// Clamp colorblind_wells to the five AGENT-pack modes. Missing/unknown → off.
    pub fn normalize_colorblind_wells(&mut self) {
        let t = self.colorblind_wells.trim().to_ascii_lowercase();
        self.colorblind_wells = match t.as_str() {
            "deuteranopia" => "deuteranopia".into(),
            "protanopia" => "protanopia".into(),
            "tritanopia" => "tritanopia".into(),
            "shape_only" => "shape_only".into(),
            _ => "off".into(),
        };
    }

    /// True when climate slab should append ring/pip/notch/rest-bar/crack beside the word.
    pub fn colorblind_wells_shapes(&self) -> bool {
        !self.colorblind_wells.eq_ignore_ascii_case("off")
    }

    /// Settings row face. Unknown already normalized to off.
    pub fn colorblind_wells_label(&self) -> &str {
        match self.colorblind_wells.as_str() {
            "deuteranopia" => "deuteranopia",
            "protanopia" => "protanopia",
            "tritanopia" => "tritanopia",
            "shape_only" => "shape_only",
            _ => "off",
        }
    }

    /// Cycle off → deuteranopia → protanopia → tritanopia → shape_only → off.
    pub fn cycle_colorblind_wells(&mut self) {
        self.normalize_colorblind_wells();
        self.colorblind_wells = match self.colorblind_wells.as_str() {
            "off" => "deuteranopia".into(),
            "deuteranopia" => "protanopia".into(),
            "protanopia" => "tritanopia".into(),
            "tritanopia" => "shape_only".into(),
            _ => "off".into(),
        };
    }

    /// Clamp LAN to "off" | "loopback". Missing/unknown/`on`/public → off.
    pub fn normalize_lan(&mut self) {
        let t = self.lan.trim().to_ascii_lowercase();
        self.lan = match t.as_str() {
            "loopback" => "loopback".into(),
            _ => "off".into(),
        };
    }

    pub fn lan_is_loopback(&self) -> bool {
        self.lan.eq_ignore_ascii_case("loopback")
    }

    /// Settings row face. Unknown already normalized to off.
    pub fn lan_label(&self) -> &'static str {
        if self.lan_is_loopback() {
            "loopback"
        } else {
            "off"
        }
    }

    /// Cycle LAN off ↔ loopback (Settings plate row — not the Online stub).
    pub fn cycle_lan(&mut self) {
        self.normalize_lan();
        if self.lan_is_loopback() {
            self.lan = "off".into();
        } else {
            self.lan = "loopback".into();
        }
    }

    /// Normalize I0 Controls fields (missing/unknown → Peace desktop defaults).
    pub fn normalize_controls(&mut self) {
        self.normalize_on_screen_sticks();
        self.normalize_nintendo_face();
        self.normalize_sprint_mode();
    }

    /// Clamp on_screen_sticks to auto|on|off. Unknown → auto.
    pub fn normalize_on_screen_sticks(&mut self) {
        let t = self.on_screen_sticks.trim().to_ascii_lowercase();
        self.on_screen_sticks = match t.as_str() {
            "on" => "on".into(),
            "off" => "off".into(),
            _ => "auto".into(),
        };
    }

    /// Clamp nintendo_face; unknown → auto (stub — keeps South = Use).
    pub fn normalize_nintendo_face(&mut self) {
        let t = self.nintendo_face.trim().to_ascii_lowercase();
        self.nintendo_face = match t.as_str() {
            "auto" => "auto".into(),
            // Future fixed layouts normalize here; unknown stays auto.
            _ => "auto".into(),
        };
    }

    /// Clamp sprint_mode to stick|trigger|key. Unknown → key.
    pub fn normalize_sprint_mode(&mut self) {
        let t = self.sprint_mode.trim().to_ascii_lowercase();
        self.sprint_mode = match t.as_str() {
            "stick" => "stick".into(),
            "trigger" => "trigger".into(),
            _ => "key".into(),
        };
    }

    pub fn on_screen_sticks_is_on(&self) -> bool {
        self.on_screen_sticks.eq_ignore_ascii_case("on")
    }

    pub fn on_screen_sticks_is_off(&self) -> bool {
        self.on_screen_sticks.eq_ignore_ascii_case("off")
    }

    pub fn on_screen_sticks_is_auto(&self) -> bool {
        !self.on_screen_sticks_is_on() && !self.on_screen_sticks_is_off()
    }

    /// Resolve whether overlay sticks should show given last pointer was Touch.
    pub fn resolve_on_screen_sticks(&self, last_pointer_touch: bool) -> bool {
        match self.on_screen_sticks.trim().to_ascii_lowercase().as_str() {
            "on" => true,
            "off" => false,
            _ => last_pointer_touch, // auto
        }
    }

    /// Cycle on_screen_sticks auto → on → off → auto.
    pub fn cycle_on_screen_sticks(&mut self) {
        self.normalize_on_screen_sticks();
        self.on_screen_sticks = match self.on_screen_sticks.as_str() {
            "auto" => "on".into(),
            "on" => "off".into(),
            _ => "auto".into(),
        };
    }

    pub fn toggle_tap_to_use(&mut self) {
        self.tap_to_use = !self.tap_to_use;
    }

    pub fn toggle_gamepad_south_use(&mut self) {
        self.gamepad_south_use = !self.gamepad_south_use;
    }

    pub fn toggle_show_use_prompt(&mut self) {
        self.show_use_prompt = !self.show_use_prompt;
    }

    /// Cycle sprint_mode key → stick → trigger → key.
    pub fn cycle_sprint_mode(&mut self) {
        self.normalize_sprint_mode();
        self.sprint_mode = match self.sprint_mode.as_str() {
            "key" => "stick".into(),
            "stick" => "trigger".into(),
            _ => "key".into(),
        };
    }

    pub fn sprint_mode_is_stick(&self) -> bool {
        self.sprint_mode.eq_ignore_ascii_case("stick")
    }

    pub fn sprint_mode_is_trigger(&self) -> bool {
        self.sprint_mode.eq_ignore_ascii_case("trigger")
    }

    pub fn sprint_mode_is_key(&self) -> bool {
        !self.sprint_mode_is_stick() && !self.sprint_mode_is_trigger()
    }

    /// Cycle look sensitivity up by one step (wraps at max → min).
    pub fn bump_look(&mut self) {
        self.clamp_look();
        let next = self.look_sensitivity + LOOK_SENS_STEP;
        self.look_sensitivity = if next > LOOK_SENS_MAX + 0.001 {
            LOOK_SENS_MIN
        } else {
            (next * 100.0).round() / 100.0
        };
        self.clamp_look();
    }

    /// Cycle brightness up by one step (wraps at max → min).
    pub fn bump_brightness(&mut self) {
        self.clamp_brightness();
        let next = self.brightness + BRIGHTNESS_STEP;
        self.brightness = if next > BRIGHTNESS_MAX + 0.001 {
            BRIGHTNESS_MIN
        } else {
            (next * 100.0).round() / 100.0
        };
        self.clamp_brightness();
    }

    /// Cycle text scale up by one step (wraps at max → min).
    pub fn bump_text_scale(&mut self) {
        self.clamp_text_scale();
        let next = self.text_scale + TEXT_SCALE_STEP;
        self.text_scale = if next > TEXT_SCALE_MAX + 0.001 {
            TEXT_SCALE_MIN
        } else {
            (next * 100.0).round() / 100.0
        };
        self.clamp_text_scale();
    }

    pub fn toggle_mute(&mut self) {
        self.mute = !self.mute;
    }

    pub fn toggle_invert_y(&mut self) {
        self.invert_y = !self.invert_y;
    }

    pub fn toggle_hide_slabs(&mut self) {
        self.hide_slabs = !self.hide_slabs;
    }

    /// Effective look Y sign: −1 when invert, else +1.
    pub fn look_y_sign(&self) -> f32 {
        if self.invert_y {
            -1.0
        } else {
            1.0
        }
    }

    /// Scale a raw mouse look delta by sensitivity + invert-Y.
    pub fn apply_look_delta(&self, dx: f32, dy: f32) -> (f32, f32) {
        let s = if self.look_sensitivity.is_finite() {
            self.look_sensitivity.clamp(LOOK_SENS_MIN, LOOK_SENS_MAX)
        } else {
            DEFAULT_LOOK_SENSITIVITY
        };
        (dx * s, dy * s * self.look_y_sign())
    }

    /// Master gain after mute (0 when muted, else 1). Thin audio hook.
    pub fn master_gain(&self) -> f32 {
        if self.mute {
            0.0
        } else {
            1.0
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        let mut s: Self = serde_json::from_str(raw)?;
        if s.schema.is_empty() {
            s.schema = SETTINGS_SCHEMA.into();
        }
        s.clamp_all();
        Ok(s)
    }

    pub fn load_or_default() -> Self {
        let Ok(raw) = crate::user_persist::read_named(SETTINGS_PATH) else {
            return Self::default();
        };
        Self::from_json(&raw).unwrap_or_default()
    }

    pub fn persist(&self) {
        if let Ok(json) = self.to_json() {
            let _ = crate::user_persist::write_named(SETTINGS_PATH, json);
        }
    }
}

/// Settings plate must never bind a listen/outbound socket via an Online toggle.
/// Returns `true` when the request is refused (always, for enable attempts).
pub fn refuse_online_socket_toggle(request_enable_online: bool) -> bool {
    // Online stays grey / honest off. No POWRUSH_NET=on from this plate.
    request_enable_online
}

/// Pure proof: local settings never open a socket door.
pub fn local_settings_opens_socket(_settings: &LocalSettings) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn defaults_match_peace_hour() {
        let s = LocalSettings::peace_defaults();
        assert_eq!(s.schema, SETTINGS_SCHEMA);
        assert!((s.look_sensitivity - DEFAULT_LOOK_SENSITIVITY).abs() < f32::EPSILON);
        assert!(!s.mute);
        assert!(!s.invert_y);
        assert!(!s.hide_slabs);
        assert!((s.brightness - DEFAULT_BRIGHTNESS).abs() < f32::EPSILON);
        assert!((s.text_scale - DEFAULT_TEXT_SCALE).abs() < f32::EPSILON);
        assert_eq!(s.grove, "off");
        assert!(!s.grove_is_light());
        assert_eq!(s.key_move_up, PeaceKey::W);
        assert_eq!(s.key_move_down, PeaceKey::S);
        assert_eq!(s.key_move_left, PeaceKey::A);
        assert_eq!(s.key_move_right, PeaceKey::D);
        assert_eq!(s.key_jump, PeaceKey::Space);
        assert_eq!(s.key_sprint, PeaceKey::LeftShift);
        assert_eq!(s.key_use, PeaceKey::E);
        assert_eq!(s.key_satchel, PeaceKey::I);
        assert_eq!(s.key_hide, PeaceKey::H);
        assert_eq!(s.key_allocate, PeaceKey::R);
        assert!(s.peace_keys_are_default());
        assert!(!s.reduced_motion);
        assert!(s.rumble);
        assert!(s.rumble_enabled());
        assert_eq!(s.camera_punch_scale(), 1.0);
        assert_eq!(s.colorblind_wells, "off");
        assert!(!s.colorblind_wells_shapes());
        assert_eq!(s.lan, "off");
        assert!(!s.lan_is_loopback());
        assert_eq!(s.on_screen_sticks, "auto");
        assert!(!s.tap_to_use);
        assert!(s.gamepad_south_use);
        assert_eq!(s.nintendo_face, "auto");
        assert_eq!(s.sprint_mode, "key");
        assert!(s.show_use_prompt);
        assert!(!s.resolve_on_screen_sticks(false));
        assert!(s.resolve_on_screen_sticks(true));
        assert!((s.master_gain() - 1.0).abs() < f32::EPSILON);
        assert!((s.look_y_sign() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn persist_round_trip_json() {
        let mut s = LocalSettings::default();
        s.look_sensitivity = 1.5;
        s.mute = true;
        s.invert_y = true;
        s.hide_slabs = true;
        s.brightness = 1.25;
        s.text_scale = 1.10;
        s.grove = "light".into();
        s.key_move_up = PeaceKey::ArrowUp;
        s.key_jump = PeaceKey::J;
        s.key_sprint = PeaceKey::LeftControl;
        s.key_use = PeaceKey::F;
        s.key_satchel = PeaceKey::B;
        s.key_hide = PeaceKey::V;
        s.key_allocate = PeaceKey::N;
        s.reduced_motion = true;
        s.rumble = false;
        s.colorblind_wells = "shape_only".into();
        s.lan = "loopback".into();
        s.on_screen_sticks = "on".into();
        s.tap_to_use = true;
        s.gamepad_south_use = false;
        s.nintendo_face = "auto".into();
        s.sprint_mode = "stick".into();
        s.show_use_prompt = false;
        let raw = s.to_json().unwrap();
        assert!(raw.contains("powrush_settings_v1"));
        assert!(raw.contains("look_sensitivity"));
        assert!(raw.contains("brightness"));
        assert!(raw.contains("text_scale"));
        assert!(raw.contains("grove") && raw.contains("light"));
        assert!(raw.contains("\"key_move_up\": \"arrow_up\""));
        assert!(raw.contains("\"key_use\": \"f\""));
        assert!(raw.contains("\"key_satchel\": \"b\""));
        assert!(raw.contains("\"reduced_motion\": true"));
        assert!(raw.contains("\"rumble\": false"));
        assert!(raw.contains("\"lan\"") && raw.contains("loopback"));
        assert!(raw.contains("on_screen_sticks"));
        assert!(raw.contains("sprint_mode") && raw.contains("stick"));
        let back = LocalSettings::from_json(&raw).unwrap();
        assert_eq!(back, s);
        assert!(back.grove_is_light());
        assert!(!back.peace_keys_are_default());
        assert_eq!(back.key_move_up, PeaceKey::ArrowUp);
        assert_eq!(back.key_use, PeaceKey::F);
        assert_eq!(back.key_satchel, PeaceKey::B);
        assert_eq!(back.key_hide, PeaceKey::V);
        assert_eq!(back.key_allocate, PeaceKey::N);
        assert!(back.reduced_motion);
        assert!(!back.rumble_enabled());
        assert_eq!(back.camera_punch_scale(), 0.0);
        assert_eq!(back.colorblind_wells, "shape_only");
        assert!(back.colorblind_wells_shapes());
        assert!(back.lan_is_loopback());
        assert!((back.master_gain() - 0.0).abs() < f32::EPSILON);
        let (dx, dy) = back.apply_look_delta(2.0, 4.0);
        assert!((dx - 3.0).abs() < 0.01);
        assert!((dy - (-6.0)).abs() < 0.01);
    }

    #[test]
    fn settings_path_beside_house() {
        assert_eq!(SETTINGS_PATH, "data/powrush_settings.json");
        assert!(SETTINGS_PATH.starts_with("data/powrush_"));
        assert_eq!(
            Path::new(SETTINGS_PATH).parent(),
            Path::new("data/powrush_house.json").parent()
        );
    }

    #[test]
    fn refuse_online_socket_toggle_always() {
        assert!(refuse_online_socket_toggle(true));
        assert!(!refuse_online_socket_toggle(false));
        let s = LocalSettings::default();
        assert!(!local_settings_opens_socket(&s));
        // Mutating local prefs still never opens a socket.
        let mut loud = s.clone();
        loud.mute = false;
        loud.hide_slabs = true;
        assert!(!local_settings_opens_socket(&loud));
    }

    #[test]
    fn bump_look_cycles_within_clamp() {
        let mut s = LocalSettings::default();
        assert!((s.look_sensitivity - 1.0).abs() < f32::EPSILON);
        s.bump_look();
        assert!((s.look_sensitivity - 1.25).abs() < 0.01);
        s.look_sensitivity = LOOK_SENS_MAX;
        s.bump_look();
        assert!((s.look_sensitivity - LOOK_SENS_MIN).abs() < 0.01);
    }

    #[test]
    fn toggles_flip_booleans() {
        let mut s = LocalSettings::default();
        s.toggle_mute();
        s.toggle_invert_y();
        s.toggle_hide_slabs();
        assert!(s.mute && s.invert_y && s.hide_slabs);
        s.toggle_mute();
        assert!(!s.mute);
    }

    #[test]
    fn brightness_and_text_scale_defaults_and_bump() {
        let mut s = LocalSettings::peace_defaults();
        assert!((s.brightness - DEFAULT_BRIGHTNESS).abs() < f32::EPSILON);
        assert!((s.text_scale - DEFAULT_TEXT_SCALE).abs() < f32::EPSILON);
        s.bump_brightness();
        assert!((s.brightness - 1.25).abs() < 0.01);
        s.brightness = BRIGHTNESS_MAX;
        s.bump_brightness();
        assert!((s.brightness - BRIGHTNESS_MIN).abs() < 0.01);
        s.bump_text_scale();
        assert!((s.text_scale - 1.05).abs() < 0.01);
        s.text_scale = TEXT_SCALE_MAX;
        s.bump_text_scale();
        assert!((s.text_scale - TEXT_SCALE_MIN).abs() < 0.01);
        // Missing fields in old JSON → peace defaults
        let legacy = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false}"#;
        let back = LocalSettings::from_json(legacy).unwrap();
        assert!((back.brightness - DEFAULT_BRIGHTNESS).abs() < f32::EPSILON);
        assert!((back.text_scale - DEFAULT_TEXT_SCALE).abs() < f32::EPSILON);
    }

    #[test]
    fn grove_defaults_off_and_normalizes_unknown() {
        let s = LocalSettings::peace_defaults();
        assert_eq!(s.grove, "off");
        assert!(!s.grove_is_light());
        // Missing field in old JSON → off
        let legacy = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false}"#;
        let back = LocalSettings::from_json(legacy).unwrap();
        assert_eq!(back.grove, "off");
        assert!(!back.grove_is_light());
        // Unknown → off
        let junk = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"brightness":1.0,"text_scale":1.0,"grove":"birds"}"#;
        let junked = LocalSettings::from_json(junk).unwrap();
        assert_eq!(junked.grove, "off");
        // light round-trip
        let light = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"brightness":1.0,"text_scale":1.0,"grove":"light"}"#;
        let lit = LocalSettings::from_json(light).unwrap();
        assert_eq!(lit.grove, "light");
        assert!(lit.grove_is_light());
        let mut cyc = LocalSettings::default();
        cyc.cycle_grove();
        assert!(cyc.grove_is_light());
        cyc.cycle_grove();
        assert!(!cyc.grove_is_light());
        assert_eq!(cyc.grove, "off");
    }

    #[test]
    fn motion_and_rumble_defaults_legacy_and_precedence() {
        let mut s = LocalSettings::peace_defaults();
        assert!(!s.reduced_motion);
        assert!(s.rumble_enabled());
        assert_eq!(s.camera_punch_scale(), 1.0);

        s.toggle_reduced_motion();
        assert!(s.reduced_motion);
        assert!(!s.rumble_enabled());
        assert_eq!(s.camera_punch_scale(), 0.0);

        s.toggle_reduced_motion();
        s.toggle_rumble();
        assert!(!s.reduced_motion);
        assert!(!s.rumble);
        assert!(!s.rumble_enabled());
        assert_eq!(s.camera_punch_scale(), 1.0);

        let legacy = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"grove":"light"}"#;
        let back = LocalSettings::from_json(legacy).unwrap();
        assert!(!back.reduced_motion);
        assert!(back.rumble);
        assert!(back.rumble_enabled());
    }

    #[test]
    fn colorblind_wells_defaults_off_normalizes_and_cycles() {
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(s.colorblind_wells, "off");
        assert!(!s.colorblind_wells_shapes());
        assert_eq!(s.colorblind_wells_label(), "off");

        // Missing field in old JSON → off (same persist path as Grove / B1).
        let legacy = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"grove":"off"}"#;
        let back = LocalSettings::from_json(legacy).unwrap();
        assert_eq!(back.colorblind_wells, "off");
        assert!(!back.colorblind_wells_shapes());

        for junk in ["birds", "on", "true", "deuter", "shapes"] {
            let raw = format!(
                r#"{{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"grove":"off","colorblind_wells":"{junk}"}}"#
            );
            let junked = LocalSettings::from_json(&raw).unwrap();
            assert_eq!(junked.colorblind_wells, "off", "unknown {junk} must be off");
        }

        for mode in ["deuteranopia", "protanopia", "tritanopia", "shape_only"] {
            let raw = format!(
                r#"{{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"grove":"off","colorblind_wells":"{mode}"}}"#
            );
            let ok = LocalSettings::from_json(&raw).unwrap();
            assert_eq!(ok.colorblind_wells, mode);
            assert!(ok.colorblind_wells_shapes());
            assert_eq!(ok.colorblind_wells_label(), mode);
        }

        let mut cyc = LocalSettings::default();
        assert_eq!(cyc.colorblind_wells_label(), "off");
        cyc.cycle_colorblind_wells();
        assert_eq!(cyc.colorblind_wells, "deuteranopia");
        cyc.cycle_colorblind_wells();
        assert_eq!(cyc.colorblind_wells, "protanopia");
        cyc.cycle_colorblind_wells();
        assert_eq!(cyc.colorblind_wells, "tritanopia");
        cyc.cycle_colorblind_wells();
        assert_eq!(cyc.colorblind_wells, "shape_only");
        cyc.cycle_colorblind_wells();
        assert_eq!(cyc.colorblind_wells, "off");
        assert!(!cyc.colorblind_wells_shapes());
    }

    #[test]
    fn lan_defaults_off_unknown_and_cycle() {
        let s = LocalSettings::peace_defaults();
        assert_eq!(s.lan, "off");
        assert!(!s.lan_is_loopback());
        assert_eq!(s.lan_label(), "off");
        // Missing field in old JSON → off (same persist path as Grove).
        let legacy = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"grove":"off"}"#;
        let back = LocalSettings::from_json(legacy).unwrap();
        assert_eq!(back.lan, "off");
        assert!(!back.lan_is_loopback());
        // Unknown / "on" / public bind spellings → off. Never Title Online.
        for junk_lan in [
            "on",
            "ON",
            "true",
            "0.0.0.0",
            "localhost",
            "127.0.0.1",
            "birds",
            "public",
        ] {
            let junk = format!(
                r#"{{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"grove":"off","lan":"{}"}}"#,
                junk_lan
            );
            let junked = LocalSettings::from_json(&junk).unwrap();
            assert_eq!(junked.lan, "off", "unknown lan {junk_lan} must be off");
            assert!(!junked.lan_is_loopback());
        }
        let loopback = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"grove":"off","lan":"loopback"}"#;
        let lab = LocalSettings::from_json(loopback).unwrap();
        assert_eq!(lab.lan, "loopback");
        assert!(lab.lan_is_loopback());
        assert_eq!(lab.lan_label(), "loopback");
        // Grove stays independent — LAN does not default Grove on.
        assert!(!lab.grove_is_light());
        let mut cyc = LocalSettings::default();
        cyc.cycle_lan();
        assert!(cyc.lan_is_loopback());
        cyc.cycle_lan();
        assert!(!cyc.lan_is_loopback());
        assert_eq!(cyc.grove, "off");
    }

    #[test]
    fn controls_defaults_and_legacy_json_load() {
        let s = LocalSettings::peace_defaults();
        assert_eq!(s.on_screen_sticks, "auto");
        assert!(!s.tap_to_use);
        assert!(s.gamepad_south_use);
        assert_eq!(s.nintendo_face, "auto");
        assert_eq!(s.sprint_mode, "key");
        assert!(s.show_use_prompt);
        // Legacy JSON without Controls fields still loads (Peace desktop defaults).
        let legacy = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"brightness":1.0,"text_scale":1.0,"grove":"off"}"#;
        let back = LocalSettings::from_json(legacy).unwrap();
        assert_eq!(back.on_screen_sticks, "auto");
        assert!(!back.tap_to_use);
        assert!(back.gamepad_south_use);
        assert_eq!(back.nintendo_face, "auto");
        assert_eq!(back.sprint_mode, "key");
        assert!(back.show_use_prompt);
        assert!(back.peace_keys_are_default());
        // Unknown normalizes
        let junk = r#"{"schema":"powrush_settings_v1","look_sensitivity":1.0,"mute":false,"invert_y":false,"hide_slabs":false,"brightness":1.0,"text_scale":1.0,"grove":"off","on_screen_sticks":"birds","nintendo_face":"pro","sprint_mode":"turbo"}"#;
        let junked = LocalSettings::from_json(junk).unwrap();
        assert_eq!(junked.on_screen_sticks, "auto");
        assert_eq!(junked.nintendo_face, "auto");
        assert_eq!(junked.sprint_mode, "key");
        let mut cyc = LocalSettings::default();
        cyc.cycle_on_screen_sticks();
        assert_eq!(cyc.on_screen_sticks, "on");
        cyc.cycle_on_screen_sticks();
        assert_eq!(cyc.on_screen_sticks, "off");
        cyc.cycle_on_screen_sticks();
        assert_eq!(cyc.on_screen_sticks, "auto");
        cyc.cycle_sprint_mode();
        assert_eq!(cyc.sprint_mode, "stick");
        cyc.cycle_sprint_mode();
        assert_eq!(cyc.sprint_mode, "trigger");
        cyc.cycle_sprint_mode();
        assert_eq!(cyc.sprint_mode, "key");
        assert!(!cyc.resolve_on_screen_sticks(false));
        cyc.on_screen_sticks = "on".into();
        assert!(cyc.resolve_on_screen_sticks(false));
        cyc.on_screen_sticks = "off".into();
        assert!(!cyc.resolve_on_screen_sticks(true));
    }

    #[test]
    fn mute_flag_is_master_mute_for_pause() {
        // Pause Mute · and Settings Mute · share one LocalSettings.mute / master_gain.
        let mut s = LocalSettings::default();
        assert!((s.master_gain() - 1.0).abs() < f32::EPSILON);
        s.toggle_mute();
        assert!(s.mute);
        assert!((s.master_gain() - 0.0).abs() < f32::EPSILON);
    }
}
