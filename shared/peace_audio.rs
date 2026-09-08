//! U4 — Peace yard audio bed + well sting
//!
//! Quiet bed while the Peace yard is live. Well sting on the existing well
//! Use (E take / hold-E tend — SoftRbePool harvests / tends). Mute is the
//! existing pause/Settings flag (`LocalSettings.mute` / MasterMute). No
//! second mute, no F-row, no new settings plate.
//!
//! Mute silences bed and sting. Unmute does not open a socket. No ALSA
//! card: boot must not hang (lavapipe) — probe is filesystem only, never
//! cpal/rodio. Title Online stays grey. Isolation gamma stays 0. Do not
//! couple tons, seeds, or declared_lethal across hexes.
//!
//! Contact: info@Rathor.ai. Independent of xAI. No certification / warranty.
//! Ra-Thor does not drive WASD.

use std::path::Path;

use crate::hex_listen::PowrushNet;
use crate::hex_protocol::default_client_listens;
use crate::local_settings::LocalSettings;
use crate::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

/// Bevy asset path (under `assets/`).
pub const BED_ASSET: &str = "audio/peace_yard_bed.ogg";
/// Bevy asset path (under `assets/`).
pub const STING_ASSET: &str = "audio/peace_well_sting.ogg";

/// Quiet yard bed — never a blast.
pub const BED_GAIN_OPEN: f32 = 0.12;
/// Soft well sting — confirmation, not a fanfare.
pub const STING_GAIN_OPEN: f32 = 0.28;

/// Lab override: `POWRUSH_AUDIO=off` never opens an output. `on` trusts the box.
pub const AUDIO_ENV: &str = "POWRUSH_AUDIO";

/// Mixer for the Peace yard bed + well sting. Pure — no socket, no hex climate.
#[derive(Debug, Clone, PartialEq)]
pub struct PeaceVoice {
    pub mute: bool,
    pub device_ok: bool,
    pub in_yard: bool,
    harvests_seen: u32,
    tends_seen: u32,
    sting_armed: bool,
}

impl Default for PeaceVoice {
    fn default() -> Self {
        Self::new(false, false)
    }
}

impl PeaceVoice {
    pub fn new(mute: bool, device_ok: bool) -> Self {
        Self {
            mute,
            device_ok,
            in_yard: false,
            harvests_seen: 0,
            tends_seen: 0,
            sting_armed: false,
        }
    }

    /// Seed from the existing settings plate. Device probe is the caller's job
    /// (`audio_output_safe`) so tests stay deterministic.
    pub fn from_settings(settings: &LocalSettings, device_ok: bool) -> Self {
        Self::new(settings.mute, device_ok)
    }

    pub fn set_mute(&mut self, mute: bool) {
        self.mute = mute;
    }

    pub fn set_in_yard(&mut self, in_yard: bool) {
        self.in_yard = in_yard;
    }

    pub fn bed_gain(&self) -> f32 {
        if self.should_play_bed() {
            BED_GAIN_OPEN
        } else {
            0.0
        }
    }

    pub fn sting_gain_if_used(&self, well_used: bool) -> f32 {
        if self.should_play_sting(well_used) {
            STING_GAIN_OPEN
        } else {
            0.0
        }
    }

    pub fn should_play_bed(&self) -> bool {
        self.device_ok && self.in_yard && !self.mute
    }

    pub fn should_play_sting(&self, well_used: bool) -> bool {
        well_used && self.device_ok && !self.mute
    }

    /// Existing well Use hook: harvests (tap E take) and tends (hold E).
    /// Returns true when a Use was noticed this call.
    pub fn note_well_use(&mut self, harvests: u32, tends: u32) -> bool {
        let used = harvests > self.harvests_seen || tends > self.tends_seen;
        self.harvests_seen = harvests;
        self.tends_seen = tends;
        if used {
            self.sting_armed = true;
        }
        used
    }

    /// Consume a pending sting. 0 when muted, no device, or nothing armed.
    pub fn take_sting(&mut self) -> f32 {
        if !self.sting_armed {
            return 0.0;
        }
        self.sting_armed = false;
        self.sting_gain_if_used(true)
    }

    /// Audio never binds. Unmute is still not a listen.
    pub fn opens_socket(&self) -> bool {
        false
    }
}

/// Master gain after mute (0 muted, else 1). Same law as `LocalSettings::master_gain`.
pub fn voice_gain(mute: bool) -> f32 {
    if mute {
        0.0
    } else {
        1.0
    }
}

pub fn bed_gain(mute: bool) -> f32 {
    BED_GAIN_OPEN * voice_gain(mute)
}

pub fn sting_gain(mute: bool) -> f32 {
    STING_GAIN_OPEN * voice_gain(mute)
}

pub fn should_emit_bed(mute: bool, device_ok: bool, in_yard: bool) -> bool {
    device_ok && in_yard && !mute && bed_gain(mute) > 0.0
}

pub fn should_emit_sting(mute: bool, device_ok: bool, well_used: bool) -> bool {
    well_used && device_ok && !mute && sting_gain(mute) > 0.0
}

/// Fast output probe. Never enumerates cpal/rodio (that hang is the lavapipe fail).
///
/// - `POWRUSH_AUDIO=off` → false
/// - `POWRUSH_AUDIO=on` → true (human box; still not a socket)
/// - Linux: `/proc/asound/cards` must name a numbered card
/// - else `/dev/snd/pcm*`
/// - non-Linux without those paths: assume a device exists
pub fn audio_output_safe() -> bool {
    match std::env::var(AUDIO_ENV) {
        Ok(v) => {
            let t = v.trim().to_ascii_lowercase();
            if matches!(t.as_str(), "off" | "0" | "false" | "none" | "silent") {
                return false;
            }
            if matches!(t.as_str(), "on" | "1" | "true") {
                return true;
            }
        }
        Err(_) => {}
    }
    probe_host_output()
}

fn probe_host_output() -> bool {
    let cards = Path::new("/proc/asound/cards");
    if cards.exists() {
        return match std::fs::read_to_string(cards) {
            Ok(raw) => alsa_cards_named(&raw),
            Err(_) => false,
        };
    }
    let snd = Path::new("/dev/snd");
    if snd.exists() {
        return snd_pcm_present(snd);
    }
    !cfg!(target_os = "linux")
}

fn alsa_cards_named(raw: &str) -> bool {
    let t = raw.trim();
    if t.is_empty() || t.contains("no soundcards") {
        return false;
    }
    t.lines().any(|line| {
        let s = line.trim_start();
        s.chars().next().is_some_and(|c| c.is_ascii_digit())
    })
}

fn snd_pcm_present(dir: &Path) -> bool {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return false;
    };
    rd.filter_map(|e| e.ok())
        .any(|e| e.file_name().to_string_lossy().starts_with("pcm"))
}

/// Peace audio never listens. Title Online stays grey.
pub fn peace_audio_opens_socket(_voice: &PeaceVoice) -> bool {
    false
}

pub fn title_online_stays_grey() -> bool {
    online_row_is_honest_disabled(ONLINE_STUB_LABEL, false)
        && !PowrushNet::Off.title_online_enabled()
        && !default_client_listens()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex_travel::{PlaceId, ISOLATION_GAMMA};
    use crate::local_settings::local_settings_opens_socket;
    use crate::shard_standing::ShardStanding;

    #[test]
    fn mute_silences_bed_and_well_sting() {
        let settings = LocalSettings {
            mute: true,
            ..LocalSettings::peace_defaults()
        };
        let mut voice = PeaceVoice::from_settings(&settings, true);
        voice.set_in_yard(true);
        assert!(settings.mute);
        assert!((settings.master_gain() - 0.0).abs() < f32::EPSILON);
        assert!((voice.bed_gain() - 0.0).abs() < f32::EPSILON);
        assert!((bed_gain(true) - 0.0).abs() < f32::EPSILON);
        assert!((sting_gain(true) - 0.0).abs() < f32::EPSILON);
        assert!(!should_emit_bed(true, true, true));
        assert!(!voice.should_play_bed());
        assert!(voice.note_well_use(1, 0));
        assert!((voice.take_sting() - 0.0).abs() < f32::EPSILON);
        assert!(!should_emit_sting(true, true, true));
        assert!((voice.sting_gain_if_used(true) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn unmute_does_not_open_a_socket() {
        let mut settings = LocalSettings::peace_defaults();
        assert!(!settings.mute);
        settings.toggle_mute();
        assert!(settings.mute);
        settings.toggle_mute();
        assert!(!settings.mute);
        let mut voice = PeaceVoice::from_settings(&settings, true);
        voice.set_in_yard(true);
        voice.set_mute(false);
        assert!(voice.should_play_bed());
        assert!((voice.bed_gain() - BED_GAIN_OPEN).abs() < f32::EPSILON);
        assert!(voice.note_well_use(1, 1));
        assert!((voice.take_sting() - STING_GAIN_OPEN).abs() < f32::EPSILON);
        assert!(!voice.opens_socket());
        assert!(!peace_audio_opens_socket(&voice));
        assert!(!local_settings_opens_socket(&settings));
        assert!(title_online_stays_grey());
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(!default_client_listens());
    }

    #[test]
    fn no_device_stays_silent_even_unmuted() {
        let mut voice = PeaceVoice::new(false, false);
        voice.set_in_yard(true);
        assert!(!voice.should_play_bed());
        assert!((voice.bed_gain() - 0.0).abs() < f32::EPSILON);
        assert!(voice.note_well_use(2, 1));
        assert!((voice.take_sting() - 0.0).abs() < f32::EPSILON);
        assert!(!should_emit_bed(false, false, true));
        assert!(!should_emit_sting(false, false, true));
        assert!(!voice.opens_socket());
    }

    #[test]
    fn title_is_not_the_yard_bed() {
        let mut voice = PeaceVoice::new(false, true);
        assert!(!voice.in_yard);
        assert!(!voice.should_play_bed());
        voice.set_in_yard(true);
        assert!(voice.should_play_bed());
        assert!(!should_emit_bed(false, true, false));
        assert!(should_emit_bed(false, true, true));
    }

    #[test]
    fn well_use_is_existing_harvest_or_tend() {
        let mut voice = PeaceVoice::new(false, true);
        assert!(!voice.note_well_use(0, 0));
        assert!((voice.take_sting() - 0.0).abs() < f32::EPSILON);
        assert!(voice.note_well_use(1, 0));
        assert!((voice.take_sting() - STING_GAIN_OPEN).abs() < f32::EPSILON);
        assert!(!voice.note_well_use(1, 0));
        assert!(voice.note_well_use(1, 1));
        assert!((voice.take_sting() - STING_GAIN_OPEN).abs() < f32::EPSILON);
    }

    #[test]
    fn muted_boot_must_not_blast() {
        // Persisted Mute · on must seed zero gain before any emit.
        let mut s = LocalSettings::peace_defaults();
        s.mute = true;
        let voice = PeaceVoice::from_settings(&s, true);
        assert!((voice.bed_gain() - 0.0).abs() < f32::EPSILON);
        assert!(!should_emit_bed(s.mute, true, true));
        assert!(!should_emit_sting(s.mute, true, true));
    }

    #[test]
    fn alsa_probe_treats_empty_and_none_as_silent() {
        assert!(!alsa_cards_named(""));
        assert!(!alsa_cards_named("--- no soundcards ---\n"));
        assert!(alsa_cards_named(
            " 0 [PCH            ]: HDA-Intel - HDA Intel PCH\n"
        ));
        // Probe itself is a read — must return without hanging.
        let _ = audio_output_safe();
    }

    #[test]
    fn isolation_and_lethal_stay_untouched() {
        assert_eq!(ISOLATION_GAMMA, 0.0);
        let standing = ShardStanding::default();
        assert!(!standing.declared_lethal);
        let mut loud = PeaceVoice::new(false, true);
        loud.set_in_yard(true);
        loud.note_well_use(3, 2);
        let _ = loud.take_sting();
        assert_eq!(ISOLATION_GAMMA, 0.0);
        assert!(!standing.declared_lethal);
        assert_eq!(PlaceId::default(), PlaceId::Sanctuary);
        assert!(!loud.opens_socket());
        assert!(title_online_stays_grey());
    }
}
