/*!
 * server/src/audio_moment_net_handler.rs
 * Powrush-MMO — Map protocol ClientMessage audio variants → ServerAudioMomentStore
 *
 * Call `route_client_audio_message` from the authoritative network ingress.
 * Emits ServerMessage responses for the transport to send back.
 *
 * v21.89.1 | AG-SML v1.0 | TOLC 8 | Permanent PATSAGi
 * Contact: info@Rathor.ai
 */

use crate::audio_moment_catalog::{
    AudioMoment, AudioMomentFlavor, AudioMomentSource, AudioSynthesisRecipe,
    ServerAudioMomentStore, WaveformKind,
};
use shared::protocol::{
    ClientMessage, ServerMessage, WireAudioMoment, WireAudioMomentFlavor,
    WireAudioMomentSource, WireWaveformKind,
};

/// Route a client audio-related message into the store; returns optional replies.
pub fn route_client_audio_message(
    msg: &ClientMessage,
    store: &mut ServerAudioMomentStore,
) -> Vec<ServerMessage> {
    match msg {
        ClientMessage::AudioMomentSave { moment } => {
            if !moment.mercy_seal {
                return vec![ServerMessage::AudioMomentSaveAck {
                    moment_id: moment.id,
                    ok: false,
                    message: "Rejected: mercy_seal required".into(),
                }];
            }
            let internal = from_wire(moment);
            let player_id = internal.owner_player_id;
            let id = store.upsert_moment(player_id, internal);
            vec![ServerMessage::AudioMomentSaveAck {
                moment_id: id,
                ok: true,
                message: "Saved".into(),
            }]
        }
        ClientMessage::AudioMomentCatalogRequest { player_id } => {
            let catalog = store.get_catalog(*player_id);
            let moments: Vec<WireAudioMoment> =
                catalog.moments.values().map(to_wire).collect();
            vec![ServerMessage::AudioMomentCatalogSnapshot {
                player_id: *player_id,
                moments,
                next_id: catalog.next_id,
                last_synced_unix: catalog.last_synced_unix,
            }]
        }
        ClientMessage::AudioMomentSetFavorite {
            moment_id,
            favorite,
        } => {
            // Best-effort: scan all loaded players (production: pass player_id)
            for (pid, cat) in store.by_player.iter_mut() {
                if let Some(m) = cat.moments.get_mut(moment_id) {
                    m.favorite = *favorite;
                    let pid = *pid;
                    drop(cat);
                    store.save_player(pid);
                    return vec![ServerMessage::AudioMomentSaveAck {
                        moment_id: *moment_id,
                        ok: true,
                        message: format!("Favorite={}", favorite),
                    }];
                }
            }
            vec![ServerMessage::AudioMomentSaveAck {
                moment_id: *moment_id,
                ok: false,
                message: "Moment not found".into(),
            }]
        }
        _ => vec![],
    }
}

fn from_wire(w: &WireAudioMoment) -> AudioMoment {
    AudioMoment {
        id: w.id,
        owner_player_id: w.owner_player_id,
        title: w.title.clone(),
        flavor: match w.flavor {
            WireAudioMomentFlavor::DivineWhisper => AudioMomentFlavor::DivineWhisper,
            WireAudioMomentFlavor::CouncilBloom => AudioMomentFlavor::CouncilBloom,
            WireAudioMomentFlavor::EpiphanyChime => AudioMomentFlavor::EpiphanyChime,
            WireAudioMomentFlavor::MercyResonance => AudioMomentFlavor::MercyResonance,
            WireAudioMomentFlavor::AmbientPad => AudioMomentFlavor::AmbientPad,
            WireAudioMomentFlavor::TransitionStinger => AudioMomentFlavor::TransitionStinger,
            WireAudioMomentFlavor::Custom => AudioMomentFlavor::Custom,
        },
        source: match w.source {
            WireAudioMomentSource::RealtimeSynthesis => AudioMomentSource::RealtimeSynthesis,
            WireAudioMomentSource::PremadeAsset => AudioMomentSource::PremadeAsset,
            WireAudioMomentSource::RecipeRecall => AudioMomentSource::RecipeRecall,
            WireAudioMomentSource::ExternalImport => AudioMomentSource::ExternalImport,
        },
        created_at_unix: w.created_at_unix,
        context: w.context.clone(),
        recipe: AudioSynthesisRecipe {
            waveform: match w.recipe.waveform {
                WireWaveformKind::Sine => WaveformKind::Sine,
                WireWaveformKind::Triangle => WaveformKind::Triangle,
                WireWaveformKind::SoftSquare => WaveformKind::SoftSquare,
                WireWaveformKind::NoiseBurst => WaveformKind::NoiseBurst,
                WireWaveformKind::HarmonicStack => WaveformKind::HarmonicStack,
            },
            frequency_hz: w.recipe.frequency_hz,
            duration_secs: w.recipe.duration_secs,
            sample_rate: w.recipe.sample_rate,
            amplitude: w.recipe.amplitude,
            attack: w.recipe.attack,
            decay: w.recipe.decay,
            sustain: w.recipe.sustain,
            release: w.recipe.release,
            partial_hz: w.recipe.partial_hz,
            partial_amp: w.recipe.partial_amp,
            brightness: w.recipe.brightness,
            valence: w.recipe.valence,
            seed: w.recipe.seed,
        },
        rendered_path: w.rendered_path.clone(),
        favorite: w.favorite,
        play_count: w.play_count,
        mercy_seal: w.mercy_seal,
    }
}

fn to_wire(m: &AudioMoment) -> WireAudioMoment {
    use shared::protocol::WireAudioSynthesisRecipe;
    WireAudioMoment {
        id: m.id,
        owner_player_id: m.owner_player_id,
        title: m.title.clone(),
        flavor: match m.flavor {
            AudioMomentFlavor::DivineWhisper => WireAudioMomentFlavor::DivineWhisper,
            AudioMomentFlavor::CouncilBloom => WireAudioMomentFlavor::CouncilBloom,
            AudioMomentFlavor::EpiphanyChime => WireAudioMomentFlavor::EpiphanyChime,
            AudioMomentFlavor::MercyResonance => WireAudioMomentFlavor::MercyResonance,
            AudioMomentFlavor::AmbientPad => WireAudioMomentFlavor::AmbientPad,
            AudioMomentFlavor::TransitionStinger => WireAudioMomentFlavor::TransitionStinger,
            AudioMomentFlavor::Custom => WireAudioMomentFlavor::Custom,
        },
        source: match m.source {
            AudioMomentSource::RealtimeSynthesis => WireAudioMomentSource::RealtimeSynthesis,
            AudioMomentSource::PremadeAsset => WireAudioMomentSource::PremadeAsset,
            AudioMomentSource::RecipeRecall => WireAudioMomentSource::RecipeRecall,
            AudioMomentSource::ExternalImport => WireAudioMomentSource::ExternalImport,
        },
        created_at_unix: m.created_at_unix,
        context: m.context.clone(),
        recipe: WireAudioSynthesisRecipe {
            waveform: match m.recipe.waveform {
                WaveformKind::Sine => WireWaveformKind::Sine,
                WaveformKind::Triangle => WireWaveformKind::Triangle,
                WaveformKind::SoftSquare => WireWaveformKind::SoftSquare,
                WaveformKind::NoiseBurst => WireWaveformKind::NoiseBurst,
                WaveformKind::HarmonicStack => WireWaveformKind::HarmonicStack,
            },
            frequency_hz: m.recipe.frequency_hz,
            duration_secs: m.recipe.duration_secs,
            sample_rate: m.recipe.sample_rate,
            amplitude: m.recipe.amplitude,
            attack: m.recipe.attack,
            decay: m.recipe.decay,
            sustain: m.recipe.sustain,
            release: m.recipe.release,
            partial_hz: m.recipe.partial_hz,
            partial_amp: m.recipe.partial_amp,
            brightness: m.recipe.brightness,
            valence: m.recipe.valence,
            seed: m.recipe.seed,
        },
        rendered_path: m.rendered_path.clone(),
        favorite: m.favorite,
        play_count: m.play_count,
        mercy_seal: m.mercy_seal,
    }
}

// Wire into network ingress:
//   for reply in route_client_audio_message(&msg, &mut store) { send(reply); }
// Thunder locked in. Yoi ⚡
