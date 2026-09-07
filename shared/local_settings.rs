//! D2 Local settings — persist beside house JSON (v23.2.63)
//!
//! `data/powrush_settings.json` next to `data/powrush_house.json`.
//! Look · Mute · Invert-Y · Hide slabs. Defaults = Peace hour behavior.
//! Online stays grey — no settings toggle binds a socket / POWRUSH_NET=on.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const SETTINGS_PATH: &str = "data/powrush_settings.json";
pub const SETTINGS_SCHEMA: &str = "powrush_settings_v1";

/// Default look sensitivity (Peace hour — no boost, no damp).
pub const DEFAULT_LOOK_SENSITIVITY: f32 = 1.0;
/// Look sensitivity clamp (simple companion — no second HUD).
pub const LOOK_SENS_MIN: f32 = 0.25;
pub const LOOK_SENS_MAX: f32 = 2.0;
pub const LOOK_SENS_STEP: f32 = 0.25;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalSettings {
    pub schema: String,
    /// Mouse look sensitivity multiplier. Default 1.0.
    #[serde(default = "default_look")]
    pub look_sensitivity: f32,
    /// Master mute. Default false (Peace hour hears).
    #[serde(default)]
    pub mute: bool,
    /// Invert look Y. Default false.
    #[serde(default)]
    pub invert_y: bool,
    /// Persist default for guidance/slab hide (H still works in session).
    #[serde(default)]
    pub hide_slabs: bool,
}

fn default_look() -> f32 {
    DEFAULT_LOOK_SENSITIVITY
}

impl Default for LocalSettings {
    fn default() -> Self {
        Self {
            schema: SETTINGS_SCHEMA.into(),
            look_sensitivity: DEFAULT_LOOK_SENSITIVITY,
            mute: false,
            invert_y: false,
            hide_slabs: false,
        }
    }
}

impl LocalSettings {
    pub fn peace_defaults() -> Self {
        Self::default()
    }

    pub fn clamp_look(&mut self) {
        if !self.look_sensitivity.is_finite() {
            self.look_sensitivity = DEFAULT_LOOK_SENSITIVITY;
        }
        self.look_sensitivity = self
            .look_sensitivity
            .clamp(LOOK_SENS_MIN, LOOK_SENS_MAX);
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
        s.clamp_look();
        Ok(s)
    }

    pub fn load_or_default() -> Self {
        let Ok(raw) = fs::read_to_string(SETTINGS_PATH) else {
            return Self::default();
        };
        Self::from_json(&raw).unwrap_or_default()
    }

    pub fn persist(&self) {
        if let Some(parent) = Path::new(SETTINGS_PATH).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = self.to_json() {
            let _ = fs::write(SETTINGS_PATH, json);
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

    #[test]
    fn defaults_match_peace_hour() {
        let s = LocalSettings::peace_defaults();
        assert_eq!(s.schema, SETTINGS_SCHEMA);
        assert!((s.look_sensitivity - DEFAULT_LOOK_SENSITIVITY).abs() < f32::EPSILON);
        assert!(!s.mute);
        assert!(!s.invert_y);
        assert!(!s.hide_slabs);
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
        let raw = s.to_json().unwrap();
        assert!(raw.contains("powrush_settings_v1"));
        assert!(raw.contains("look_sensitivity"));
        let back = LocalSettings::from_json(&raw).unwrap();
        assert_eq!(back, s);
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
}
