//! U4 — Peace yard bed + well sting (client hook)
//!
//! Mixer lives in `shared::peace_audio`. This plugin:
//! - seeds mute from the existing Settings / pause Mute row
//! - loops the quiet bed while `LaunchDoor::InYard`
//! - Depths (HexTravelState → PlaceId::Depths) reuses the same bed quieter
//! - Heartwood (HexTravelState → PlaceId::Heartwood) reuses the same bed as lamp hush
//! - plays a soft one-shot on the first successful E (A1)
//! - plays the well sting on later SoftRbePool harvests / tends (existing Use)
//! - never spawns output when muted or when `audio_output_safe` is false
//!
//! CARD FLESH-HEARTWOOD-DRESS — lamp hush stays `BED_GAIN_HEARTWOOD` (banked).
//! Living-wood fog dress lives in `climate_plane`. This hook does not retune
//! the shared mixer and does not add an asset.
//!
//! CARD FLESH-DEPTHS-DRESS — quieter bed stays `BED_GAIN_DEPTHS` (banked).
//! Wet-stone fog dress lives in `climate_plane`. This hook does not retune
//! the shared mixer and does not add an asset. Do not touch harvest_feel.
//!
//! CARD OPT-AUDIO-COMFORT-LOW — Graphics Low scales yard / Heartwood / Depths
//! bed playback by `BED_GAIN_COMFORT_LOW_SCALE`. Medium and High stay banked.
//! Mute still zeros. No Ultra. Shared mixer unchanged.
//!
//! No second mute. No F-row. No listen. No ALSA/cpal open. Contact: info@Rathor.ai

use bevy::audio::{AudioSink, Volume};
use bevy::prelude::*;

use shared::hex_travel::PlaceId;
use shared::local_settings::GraphicsPreset;
use shared::peace_audio::{audio_output_safe, PeaceVoice, BED_ASSET, STING_ASSET};

use crate::harvest_feel::SoftRbePool;
use crate::hex_travel::HexTravelState;
use crate::local_settings::{LocalMeshLodFeel, LocalSettingsState, MasterMuteGain};
use crate::title_screen::LaunchDoor;

/// CARD OPT-AUDIO-COMFORT-LOW — Comfort Low ceiling on peace bed playback.
/// `0.55` matches Comfort Low gentler intensity. Medium and High return the
/// banked gain (`BED_GAIN_OPEN` / `BED_GAIN_HEARTWOOD` / `BED_GAIN_DEPTHS`).
const BED_GAIN_COMFORT_LOW_SCALE: f32 = 0.55;

/// Playback gain for the yard / Heartwood / Depths bed already on this plugin.
/// Low multiplies the banked hush. Medium and High leave it unchanged.
/// A muted banked gain of 0 stays 0.
fn bed_playback_gain(banked: f32, preset: GraphicsPreset) -> f32 {
    match preset {
        GraphicsPreset::Low => banked * BED_GAIN_COMFORT_LOW_SCALE,
        GraphicsPreset::Medium | GraphicsPreset::High => banked,
    }
}

/// Comfort plate: `LocalMeshLodFeel.preset`, else settings `graphics_preset`.
/// Missing both stays Medium (banked beds). No Ultra till.
fn comfort_preset_from(
    feel: Option<GraphicsPreset>,
    settings: Option<GraphicsPreset>,
) -> GraphicsPreset {
    feel.or(settings).unwrap_or(GraphicsPreset::Medium)
}

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct PeaceAudioState {
    pub voice: PeaceVoice,
}

impl Default for PeaceAudioState {
    fn default() -> Self {
        // Load the same plate the pause Mute row writes. Muted persist must
        // not blast for a frame before the first settings sync.
        let settings = shared::local_settings::LocalSettings::load_or_default();
        Self {
            voice: PeaceVoice::from_settings(&settings, audio_output_safe()),
        }
    }
}

#[derive(Component)]
struct PeaceYardBed;

#[derive(Component)]
struct PeaceWellSting;

#[derive(Component)]
struct PeaceFirstEOneShot;

pub struct PeaceAudioPlugin;

impl Plugin for PeaceAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PeaceAudioState>().add_systems(
            Update,
            (
                sync_voice_from_settings,
                sync_peace_bed,
                sync_first_e_oneshot,
                sync_well_sting,
            )
                .chain(),
        );
    }
}

fn sync_voice_from_settings(
    settings: Res<LocalSettingsState>,
    mute: Res<MasterMuteGain>,
    door: Res<LaunchDoor>,
    travel: Option<Res<HexTravelState>>,
    mut state: ResMut<PeaceAudioState>,
) {
    // Pause Mute · and Settings Mute · share one flag. Gain 0 is also mute.
    let muted = settings.inner.mute || mute.muted || mute.gain <= 0.0;
    state.voice.set_mute(muted);
    state.voice.set_in_yard(*door == LaunchDoor::InYard);
    // Esc→Places→Depths / Heartwood: hush-family gain on the same asset. Mute still zeros.
    // CARD FLESH-HEARTWOOD-DRESS — Heartwood lamp hush is the banked gain.
    // Climate fog does not write this mixer.
    let current = travel.as_ref().map(|t| t.current);
    // CARD FLESH-DEPTHS-DRESS — Depths wet-stone hush is the banked gain.
    // Climate fog does not write this mixer.
    state.voice.set_in_depths(current == Some(PlaceId::Depths));
    state.voice.set_in_heartwood(current == Some(PlaceId::Heartwood));
}

fn sync_peace_bed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<PeaceAudioState>,
    feel: Option<Res<LocalMeshLodFeel>>,
    settings: Option<Res<LocalSettingsState>>,
    beds: Query<(Entity, Option<&AudioSink>), With<PeaceYardBed>>,
) {
    let want = state.voice.should_play_bed();
    // CARD OPT-AUDIO-COMFORT-LOW — cap bed AudioSink volume on Graphics Low only.
    let gain = bed_playback_gain(
        state.voice.bed_gain(),
        comfort_preset_from(
            feel.map(|f| f.preset),
            settings.as_ref().map(|s| s.inner.graphics_preset),
        ),
    );

    if !state.voice.device_ok || !state.voice.in_yard {
        for (entity, sink) in &beds {
            if let Some(sink) = sink {
                sink.set_volume(0.0);
                sink.pause();
            }
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    if beds.is_empty() {
        // Spawn at the current gain (0 when muted) so a muted boot cannot blast.
        commands.spawn((
            AudioBundle {
                source: asset_server.load(BED_ASSET),
                settings: PlaybackSettings::LOOP.with_volume(Volume::new(gain)),
                ..default()
            },
            PeaceYardBed,
        ));
        return;
    }

    for (_, sink) in &beds {
        if let Some(sink) = sink {
            sink.set_volume(gain);
            if want {
                sink.play();
            } else {
                sink.pause();
            }
        }
    }
}

fn sync_first_e_oneshot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<PeaceAudioState>,
    pool: Res<SoftRbePool>,
) {
    // note_well_use once per frame before either emit path.
    state.voice.note_well_use(pool.harvests, pool.tends);
    let gain = state.voice.take_first_e_oneshot();
    if gain <= 0.0 || !state.voice.device_ok {
        return;
    }
    commands.spawn((
        AudioBundle {
            source: asset_server.load(STING_ASSET),
            settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(gain)),
            ..default()
        },
        PeaceFirstEOneShot,
    ));
}

fn sync_well_sting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<PeaceAudioState>,
) {
    let gain = state.voice.take_sting();
    if gain <= 0.0 || !state.voice.device_ok {
        return;
    }
    commands.spawn((
        AudioBundle {
            source: asset_server.load(STING_ASSET),
            settings: PlaybackSettings::DESPAWN.with_volume(Volume::new(gain)),
            ..default()
        },
        PeaceWellSting,
    ));
}

#[cfg(test)]
mod tests {
    use super::{BED_GAIN_COMFORT_LOW_SCALE, bed_playback_gain, comfort_preset_from};
    use super::*;
    use shared::hex_listen::PowrushNet;
    use shared::hex_protocol::default_client_listens;
    use shared::hex_travel::ISOLATION_GAMMA;
    use shared::local_settings::{local_settings_opens_socket, GraphicsPreset, LocalSettings};
    use shared::hex_travel::PlaceId;
    use shared::peace_audio::{
        bed_gain, bed_gain_at, bed_gain_place, peace_audio_opens_socket, should_emit_bed,
        should_emit_sting, sting_gain, title_online_stays_grey, BED_ASSET, BED_GAIN_DEPTHS,
        BED_GAIN_HEARTWOOD, BED_GAIN_OPEN,
    };
    use shared::shard_standing::ShardStanding;
    use shared::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};

    #[test]
    fn mute_silences_bed_and_well_sting() {
        let mut settings = LocalSettings::peace_defaults();
        settings.mute = true;
        let mut voice = PeaceVoice::from_settings(&settings, true);
        voice.set_in_yard(true);
        let mut g = MasterMuteGain::default();
        g.muted = settings.mute;
        g.gain = settings.master_gain();
        assert!(g.muted && g.gain == 0.0);
        assert!((voice.bed_gain() - 0.0).abs() < f32::EPSILON);
        assert!((bed_gain(true) - 0.0).abs() < f32::EPSILON);
        assert!((sting_gain(true) - 0.0).abs() < f32::EPSILON);
        assert!(voice.note_well_use(1, 1));
        assert!((voice.take_first_e_oneshot() - 0.0).abs() < f32::EPSILON);
        assert!((voice.take_sting() - 0.0).abs() < f32::EPSILON);
        assert!(!should_emit_bed(true, true, true));
        assert!(!should_emit_sting(true, true, true));
    }

    #[test]
    fn unmute_does_not_open_a_socket() {
        let mut settings = LocalSettings::peace_defaults();
        settings.mute = false;
        let mut voice = PeaceVoice::from_settings(&settings, true);
        voice.set_in_yard(true);
        voice.set_mute(false);
        assert!(voice.should_play_bed());
        assert!(voice.note_well_use(1, 0));
        assert!(voice.take_first_e_oneshot() > 0.0);
        assert_eq!(voice.take_sting(), 0.0);
        assert!(voice.note_well_use(2, 0));
        assert!(voice.take_sting() > 0.0);
        assert!(!voice.opens_socket());
        assert!(!peace_audio_opens_socket(&voice));
        assert!(!local_settings_opens_socket(&settings));
        assert!(title_online_stays_grey());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(!default_client_listens());
        assert_eq!(ISOLATION_GAMMA, 0.0);
        assert!(!ShardStanding::default().declared_lethal);
    }

    #[test]
    fn plugin_is_a_plugin() {
        let _ = PeaceAudioPlugin;
        let _ = audio_output_safe();
    }

    #[test]
    fn first_e_oneshot_respects_mute_and_fires_once() {
        use shared::peace_audio::FIRST_E_GAIN_OPEN;
        let mut voice = PeaceVoice::new(false, true);
        voice.set_in_yard(true);
        assert!(voice.note_well_use(1, 0));
        assert!((voice.take_first_e_oneshot() - FIRST_E_GAIN_OPEN).abs() < f32::EPSILON);
        assert_eq!(voice.take_first_e_oneshot(), 0.0);
        let mut muted = PeaceVoice::new(true, true);
        muted.set_in_yard(true);
        assert!(muted.note_well_use(1, 0));
        assert_eq!(muted.take_first_e_oneshot(), 0.0);
    }

    #[test]
    fn depths_bed_gain_is_quieter_and_reuses_yard_asset() {
        assert!(BED_GAIN_DEPTHS < BED_GAIN_OPEN);
        assert_eq!(BED_ASSET, "audio/peace_yard_bed.ogg");
        let mut voice = PeaceVoice::new(false, true);
        voice.set_in_yard(true);
        voice.set_in_depths(false);
        let yard = voice.bed_gain();
        voice.set_in_depths(true);
        let depths = voice.bed_gain();
        assert!((yard - BED_GAIN_OPEN).abs() < f32::EPSILON);
        assert!((depths - BED_GAIN_DEPTHS).abs() < f32::EPSILON);
        assert!(depths < yard);
        assert!((bed_gain_at(false, true) - BED_GAIN_DEPTHS).abs() < f32::EPSILON);
        // PlaceId::Depths is the Esc→Places→Depths landing.
        assert_eq!(PlaceId::Depths.display_name(), "Depths");
        assert!(!peace_audio_opens_socket(&voice));
        let _ = audio_output_safe(); // filesystem probe only — no cpal
    }

    #[test]
    fn mute_kills_depths_bed() {
        let mut settings = LocalSettings::peace_defaults();
        settings.mute = true;
        let mut voice = PeaceVoice::from_settings(&settings, true);
        voice.set_in_yard(true);
        voice.set_in_depths(true);
        let mut g = MasterMuteGain::default();
        g.muted = settings.mute;
        g.gain = settings.master_gain();
        assert!(g.muted && g.gain == 0.0);
        assert!((voice.bed_gain() - 0.0).abs() < f32::EPSILON);
        assert!((bed_gain_at(true, true) - 0.0).abs() < f32::EPSILON);
        assert!((bed_gain(true) - 0.0).abs() < f32::EPSILON);
        assert!(!should_emit_bed(true, true, true));
        voice.set_mute(false);
        assert!((voice.bed_gain() - BED_GAIN_DEPTHS).abs() < f32::EPSILON);
        assert!(!voice.opens_socket());
    }

    #[test]
    fn heartwood_bed_gain_is_quieter_and_reuses_yard_asset() {
        assert!(BED_GAIN_HEARTWOOD < BED_GAIN_OPEN);
        assert_eq!(BED_ASSET, "audio/peace_yard_bed.ogg");
        let mut voice = PeaceVoice::new(false, true);
        voice.set_in_yard(true);
        voice.set_in_heartwood(false);
        let yard = voice.bed_gain();
        voice.set_in_heartwood(true);
        let heartwood = voice.bed_gain();
        assert!((yard - BED_GAIN_OPEN).abs() < f32::EPSILON);
        assert!((heartwood - BED_GAIN_HEARTWOOD).abs() < f32::EPSILON);
        assert!(heartwood < yard);
        assert!((bed_gain_place(false, false, true) - BED_GAIN_HEARTWOOD).abs() < f32::EPSILON);
        // Depths path still works.
        voice.set_in_heartwood(false);
        voice.set_in_depths(true);
        assert!((voice.bed_gain() - BED_GAIN_DEPTHS).abs() < f32::EPSILON);
        assert!((bed_gain_at(false, true) - BED_GAIN_DEPTHS).abs() < f32::EPSILON);
        // PlaceId::Heartwood is the Esc→Places→Heartwood landing.
        assert_eq!(PlaceId::Heartwood.display_name(), "Heartwood");
        assert!(!peace_audio_opens_socket(&voice));
        let _ = audio_output_safe(); // filesystem probe only — no cpal
    }

    #[test]
    fn mute_kills_heartwood_bed() {
        let mut settings = LocalSettings::peace_defaults();
        settings.mute = true;
        let mut voice = PeaceVoice::from_settings(&settings, true);
        voice.set_in_yard(true);
        voice.set_in_heartwood(true);
        let mut g = MasterMuteGain::default();
        g.muted = settings.mute;
        g.gain = settings.master_gain();
        assert!(g.muted && g.gain == 0.0);
        assert!((voice.bed_gain() - 0.0).abs() < f32::EPSILON);
        assert!((bed_gain_place(true, false, true) - 0.0).abs() < f32::EPSILON);
        assert!((bed_gain(true) - 0.0).abs() < f32::EPSILON);
        assert!(!should_emit_bed(true, true, true));
        voice.set_mute(false);
        assert!((voice.bed_gain() - BED_GAIN_HEARTWOOD).abs() < f32::EPSILON);
        assert!(!voice.opens_socket());
    }

    /// CARD FLESH-HEARTWOOD-DRESS — lamp hush stays the banked gain on the yard asset.
    /// Climate fog dress does not retune this mixer. No new asset. Online stays grey.
    #[test]
    fn flesh_heartwood_dress_keeps_banked_lamp_hush() {
        assert!((BED_GAIN_HEARTWOOD - 0.04).abs() < f32::EPSILON);
        assert!(BED_GAIN_HEARTWOOD < BED_GAIN_DEPTHS);
        assert!(BED_GAIN_HEARTWOOD < BED_GAIN_OPEN);
        assert!(BED_GAIN_HEARTWOOD > 0.0);
        assert_eq!(BED_ASSET, "audio/peace_yard_bed.ogg");
        let mut voice = PeaceVoice::new(false, true);
        voice.set_in_yard(true);
        voice.set_in_heartwood(true);
        assert!((voice.bed_gain() - BED_GAIN_HEARTWOOD).abs() < f32::EPSILON);
        voice.set_mute(true);
        assert!((voice.bed_gain() - 0.0).abs() < f32::EPSILON);
        voice.set_mute(false);
        voice.set_in_heartwood(false);
        voice.set_in_depths(true);
        assert!((voice.bed_gain() - BED_GAIN_DEPTHS).abs() < f32::EPSILON);
        assert_eq!(PlaceId::Heartwood.display_name(), "Heartwood");
        assert!(title_online_stays_grey());
        assert!(!peace_audio_opens_socket(&voice));
        assert!(!voice.opens_socket());
    }

    /// CARD FLESH-DEPTHS-DRESS — wet-stone hush stays the banked gain on the yard asset.
    /// Climate fog dress does not retune this mixer. No new asset. Online stays grey.
    #[test]
    fn flesh_depths_dress_keeps_banked_wet_stone_hush() {
        assert!((BED_GAIN_DEPTHS - 0.06).abs() < f32::EPSILON);
        assert!(BED_GAIN_DEPTHS < BED_GAIN_OPEN);
        assert!(BED_GAIN_HEARTWOOD < BED_GAIN_DEPTHS);
        assert!(BED_GAIN_DEPTHS > 0.0);
        assert_eq!(BED_ASSET, "audio/peace_yard_bed.ogg");
        let mut voice = PeaceVoice::new(false, true);
        voice.set_in_yard(true);
        voice.set_in_depths(true);
        assert!((voice.bed_gain() - BED_GAIN_DEPTHS).abs() < f32::EPSILON);
        voice.set_mute(true);
        assert!((voice.bed_gain() - 0.0).abs() < f32::EPSILON);
        voice.set_mute(false);
        assert!((voice.bed_gain() - BED_GAIN_DEPTHS).abs() < f32::EPSILON);
        assert_eq!(PlaceId::Depths.display_name(), "Depths");
        assert!(title_online_stays_grey());
        assert!(!peace_audio_opens_socket(&voice));
        assert!(!voice.opens_socket());
    }

    /// CARD OPT-AUDIO-COMFORT-LOW — Low is quieter than Medium for the same place.
    /// Medium and High stay the banked constants. Mute still zeros. No Ultra.
    #[test]
    fn comfort_low_caps_bed_quieter_than_medium_high_stay_banked() {
        assert_eq!(GraphicsPreset::ALL.len(), 3);
        for preset in GraphicsPreset::ALL {
            assert!(!preset.label().contains("Ultra"));
        }
        assert!((BED_GAIN_COMFORT_LOW_SCALE - 0.55).abs() < f32::EPSILON);
        assert!(BED_GAIN_COMFORT_LOW_SCALE > 0.0 && BED_GAIN_COMFORT_LOW_SCALE < 1.0);

        let defaults = LocalSettings::peace_defaults();
        assert_eq!(defaults.graphics_preset, GraphicsPreset::Medium);
        let feel = crate::local_settings::LocalMeshLodFeel::from_settings(&defaults);
        assert_eq!(feel.preset, GraphicsPreset::Medium);
        assert_eq!(
            comfort_preset_from(Some(feel.preset), Some(defaults.graphics_preset)),
            GraphicsPreset::Medium
        );
        assert_eq!(
            comfort_preset_from(Some(GraphicsPreset::Low), Some(GraphicsPreset::High)),
            GraphicsPreset::Low
        );
        assert_eq!(
            comfort_preset_from(None, Some(GraphicsPreset::High)),
            GraphicsPreset::High
        );
        assert_eq!(comfort_preset_from(None, None), GraphicsPreset::Medium);

        let places = [
            (false, false, BED_GAIN_OPEN),
            (true, false, BED_GAIN_DEPTHS),
            (false, true, BED_GAIN_HEARTWOOD),
        ];
        let mut low_gains = [0.0_f32; 3];
        for (i, (in_depths, in_heartwood, banked)) in places.into_iter().enumerate() {
            let mut voice = PeaceVoice::new(false, true);
            voice.set_in_yard(true);
            voice.set_in_depths(in_depths);
            voice.set_in_heartwood(in_heartwood);
            let banked_voice = voice.bed_gain();
            assert!((banked_voice - banked).abs() < f32::EPSILON);

            let low = bed_playback_gain(banked_voice, GraphicsPreset::Low);
            let mid = bed_playback_gain(banked_voice, GraphicsPreset::Medium);
            let high = bed_playback_gain(banked_voice, GraphicsPreset::High);
            assert!(low < mid);
            assert!(low > 0.0);
            assert!((mid - banked).abs() < f32::EPSILON);
            assert!((high - banked).abs() < f32::EPSILON);
            assert!((low - banked * BED_GAIN_COMFORT_LOW_SCALE).abs() < f32::EPSILON);
            low_gains[i] = low;

            voice.set_mute(true);
            assert!(
                (bed_playback_gain(voice.bed_gain(), GraphicsPreset::Low) - 0.0).abs()
                    < f32::EPSILON
            );
            assert!(!voice.opens_socket());
        }
        // Hush family stays ordered on Low: Heartwood < Depths < yard.
        assert!(low_gains[2] < low_gains[1] && low_gains[1] < low_gains[0]);

        let mut low_settings = LocalSettings::peace_defaults();
        low_settings.set_graphics_preset(GraphicsPreset::Low);
        assert_eq!(
            crate::local_settings::LocalMeshLodFeel::from_settings(&low_settings).preset,
            GraphicsPreset::Low
        );
        assert_eq!(BED_ASSET, "audio/peace_yard_bed.ogg");
        assert!(title_online_stays_grey());
        assert!(!peace_audio_opens_socket(&PeaceVoice::new(false, true)));
    }
}
