//! D2 Local settings resource + runtime apply (v23.2.63)
//!
//! Persist: shared::local_settings → data/powrush_settings.json.
//! Apply hide_slabs → guidance_hidden; look/invert → LocalLookFeel;
//! mute → MasterMuteGain (thin audio hook — pause Mute uses same flag);
//! brightness / text_scale → LocalUiFeel (Title plate contrast stays law).
//! No Online socket toggle. LAN off (default) opens nothing; loopback is 127.0.0.1 only.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::local_settings::LocalSettings;

use crate::lived_hour_bind::LivedHourBind;

/// Bevy resource — same fields as shared LocalSettings.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct LocalSettingsState {
    pub inner: LocalSettings,
    /// Dirty flag so UI clicks persist once.
    pub dirty: bool,
}

impl Default for LocalSettingsState {
    fn default() -> Self {
        Self {
            inner: LocalSettings::load_or_default(),
            dirty: false,
        }
    }
}

impl LocalSettingsState {
    pub fn persist_if_dirty(&mut self) {
        if self.dirty {
            self.inner.persist();
            self.dirty = false;
        }
    }

    pub fn mark_and_persist(&mut self) {
        self.inner.persist();
        self.dirty = false;
    }
}

/// Thin look feel — sensitivity + invert for any mouse-look hook.
#[derive(Resource, Debug, Clone, Copy)]
pub struct LocalLookFeel {
    pub sensitivity: f32,
    pub invert_y: bool,
}

impl Default for LocalLookFeel {
    fn default() -> Self {
        let s = LocalSettings::peace_defaults();
        Self {
            sensitivity: s.look_sensitivity,
            invert_y: s.invert_y,
        }
    }
}

impl LocalLookFeel {
    pub fn from_settings(s: &LocalSettings) -> Self {
        Self {
            sensitivity: s.look_sensitivity,
            invert_y: s.invert_y,
        }
    }

    pub fn apply_delta(&self, dx: f32, dy: f32) -> (f32, f32) {
        let y_sign = if self.invert_y { -1.0 } else { 1.0 };
        (dx * self.sensitivity, dy * self.sensitivity * y_sign)
    }
}

/// Master mute gain for thin audio paths (0 muted, 1 audible).
#[derive(Resource, Debug, Clone, Copy)]
pub struct MasterMuteGain {
    pub gain: f32,
    pub muted: bool,
}

impl Default for MasterMuteGain {
    fn default() -> Self {
        Self {
            gain: 1.0,
            muted: false,
        }
    }
}

/// Thin UI feel — brightness + text scale (does not mutate Title plate colors).
#[derive(Resource, Debug, Clone, Copy)]
pub struct LocalUiFeel {
    pub brightness: f32,
    pub text_scale: f32,
}

impl Default for LocalUiFeel {
    fn default() -> Self {
        let s = LocalSettings::peace_defaults();
        Self {
            brightness: s.brightness,
            text_scale: s.text_scale,
        }
    }
}

impl LocalUiFeel {
    pub fn from_settings(s: &LocalSettings) -> Self {
        Self {
            brightness: s.brightness,
            text_scale: s.text_scale,
        }
    }

    /// Scaled font size for pause/settings rows (base * text_scale).
    pub fn scaled_font(&self, base: f32) -> f32 {
        (base * self.text_scale).clamp(11.0, 22.0)
    }
}

pub struct LocalSettingsPlugin;

impl Plugin for LocalSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalSettingsState>()
            .init_resource::<LocalLookFeel>()
            .init_resource::<MasterMuteGain>()
            .init_resource::<LocalUiFeel>()
            .add_systems(Startup, seed_runtime_from_settings)
            .add_systems(Update, (apply_local_settings_runtime, persist_dirty_settings));
    }
}

fn seed_runtime_from_settings(
    settings: Res<LocalSettingsState>,
    mut look: ResMut<LocalLookFeel>,
    mut mute: ResMut<MasterMuteGain>,
    mut ui: ResMut<LocalUiFeel>,
    mut bind: ResMut<LivedHourBind>,
) {
    *look = LocalLookFeel::from_settings(&settings.inner);
    mute.muted = settings.inner.mute;
    mute.gain = settings.inner.master_gain();
    *ui = LocalUiFeel::from_settings(&settings.inner);
    // Persist default for Hide slabs — H still works in session after this.
    bind.guidance_hidden = settings.inner.hide_slabs;
}

fn apply_local_settings_runtime(
    settings: Res<LocalSettingsState>,
    mut look: ResMut<LocalLookFeel>,
    mut mute: ResMut<MasterMuteGain>,
    mut ui: ResMut<LocalUiFeel>,
    mut bind: ResMut<LivedHourBind>,
) {
    if !settings.is_changed() {
        return;
    }
    *look = LocalLookFeel::from_settings(&settings.inner);
    mute.muted = settings.inner.mute;
    mute.gain = settings.inner.master_gain();
    *ui = LocalUiFeel::from_settings(&settings.inner);
    // Only when settings change (UI) — H session toggles are not overwritten every frame.
    bind.guidance_hidden = settings.inner.hide_slabs;
}

fn persist_dirty_settings(mut settings: ResMut<LocalSettingsState>) {
    settings.persist_if_dirty();
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::local_settings::{
        local_settings_opens_socket, refuse_online_socket_toggle, DEFAULT_LOOK_SENSITIVITY,
        SETTINGS_PATH,
    };

    #[test]
    fn look_feel_respects_invert_and_sensitivity() {
        let feel = LocalLookFeel {
            sensitivity: 2.0,
            invert_y: true,
        };
        let (dx, dy) = feel.apply_delta(1.0, 3.0);
        assert!((dx - 2.0).abs() < f32::EPSILON);
        assert!((dy - (-6.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn defaults_do_not_open_socket() {
        let state = LocalSettingsState {
            inner: LocalSettings::peace_defaults(),
            dirty: false,
        };
        assert!((state.inner.look_sensitivity - DEFAULT_LOOK_SENSITIVITY).abs() < f32::EPSILON);
        assert!(!local_settings_opens_socket(&state.inner));
        assert!(refuse_online_socket_toggle(true));
        assert_eq!(SETTINGS_PATH, "data/powrush_settings.json");
    }

    #[test]
    fn mute_maps_to_zero_gain() {
        let mut s = LocalSettings::default();
        assert!((s.master_gain() - 1.0).abs() < f32::EPSILON);
        s.mute = true;
        assert!((s.master_gain() - 0.0).abs() < f32::EPSILON);
        let mut g = MasterMuteGain::default();
        g.muted = s.mute;
        g.gain = s.master_gain();
        assert!(g.muted);
        assert!((g.gain - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn ui_feel_brightness_and_text_scale() {
        let mut s = LocalSettings::peace_defaults();
        assert!((s.brightness - 1.0).abs() < f32::EPSILON);
        assert!((s.text_scale - 1.0).abs() < f32::EPSILON);
        s.brightness = 1.25;
        s.text_scale = 1.10;
        let ui = LocalUiFeel::from_settings(&s);
        assert!((ui.brightness - 1.25).abs() < 0.01);
        assert!((ui.scaled_font(15.0) - 16.5).abs() < 0.01);
        // Pause mute shares MasterMuteGain with settings.mute
        s.toggle_mute();
        let mut g = MasterMuteGain::default();
        g.muted = s.mute;
        g.gain = s.master_gain();
        assert!(g.muted && g.gain == 0.0);
    }
}
