/*!
 * client/audio_moment_net_bridge.rs
 * Powrush-MMO — Bridge between realtime audio synthesis and network transport
 *
 * - Converts AudioMomentServerSyncRequest → outbound ClientMessage::AudioMomentSave
 * - Queues messages for the WASM/native transport to drain
 * - Merges ServerMessage::AudioMomentCatalogSnapshot into local catalog
 *
 * Transport remains ownership of client/main.rs / ClientWsTransport.
 * This bridge is Bevy-side and transport-agnostic.
 *
 * v21.89.1 | AG-SML v1.0 | TOLC 8 | Permanent PATSAGi
 * Contact: info@Rathor.ai
 */

use bevy::prelude::*;
use crate::realtime_audio_synthesis::{
    AudioMoment, AudioMomentCatalog, AudioMomentServerSyncRequest,
    AudioMomentFlavor, AudioMomentSource, AudioSynthesisRecipe, WaveformKind,
    RealtimeAudioConfig,
};
use shared::protocol::{
    ClientMessage, ServerMessage, WireAudioMoment, WireAudioSynthesisRecipe,
    WireAudioMomentFlavor, WireAudioMomentSource, WireWaveformKind,
};
use std::collections::VecDeque;
use std::fs;

/// Outbound queue drained by the transport layer each tick
#[derive(Resource, Default)]
pub struct AudioMomentOutboundQueue {
    pub messages: VecDeque<ClientMessage>,
}

impl AudioMomentOutboundQueue {
    pub fn push_save(&mut self, moment: &AudioMoment) {
        self.messages
            .push_back(ClientMessage::AudioMomentSave {
                moment: to_wire(moment),
            });
    }

    pub fn push_catalog_request(&mut self, player_id: u64) {
        self.messages
            .push_back(ClientMessage::AudioMomentCatalogRequest { player_id });
    }

    pub fn drain(&mut self) -> Vec<ClientMessage> {
        self.messages.drain(..).collect()
    }
}

pub struct AudioMomentNetBridgePlugin;

impl Plugin for AudioMomentNetBridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioMomentOutboundQueue>()
            .add_systems(Update, (
                enqueue_server_sync_requests,
                // Catalog merge is invoked when transport pushes ServerMessage into an event
            ));
    }
}

fn enqueue_server_sync_requests(
    mut events: EventReader<AudioMomentServerSyncRequest>,
    mut queue: ResMut<AudioMomentOutboundQueue>,
) {
    for ev in events.read() {
        if !ev.moment.mercy_seal {
            continue;
        }
        queue.push_save(&ev.moment);
        info!(
            target: "powrush::audio",
            moment_id = ev.moment.id,
            "Audio moment queued for server catalog sync"
        );
    }
}

/// Call from transport poll when a ServerMessage arrives
pub fn handle_server_audio_message(
    msg: &ServerMessage,
    catalog: &mut AudioMomentCatalog,
    cfg: &RealtimeAudioConfig,
) {
    match msg {
        ServerMessage::AudioMomentCatalogSnapshot {
            player_id,
            moments,
            next_id,
            last_synced_unix,
        } => {
            catalog.owner_player_id = *player_id;
            catalog.next_id = (*next_id).max(catalog.next_id);
            catalog.last_synced_unix = *last_synced_unix;
            for w in moments {
                let m = from_wire(w);
                catalog.moments.insert(m.id, m);
            }
            // Persist merged catalog locally
            let path = cfg.local_root.join("catalog.json");
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(catalog) {
                let tmp = path.with_extension("json.tmp");
                if fs::write(&tmp, json).is_ok() {
                    let _ = fs::rename(&tmp, &path);
                }
            }
            info!(
                target: "powrush::audio",
                player_id,
                count = moments.len(),
                "Merged server audio moment catalog into local"
            );
        }
        ServerMessage::AudioMomentSaveAck {
            moment_id,
            ok,
            message,
        } => {
            info!(
                target: "powrush::audio",
                moment_id,
                ok,
                %message,
                "Server audio moment save ack"
            );
        }
        _ => {}
    }
}

fn to_wire(m: &AudioMoment) -> WireAudioMoment {
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

// Transport integration (client/main.rs pattern):
//   for msg in audio_outbound.drain() { transport.send(msg); }
//   on ServerMessage → handle_server_audio_message(&msg, &mut catalog, &cfg);
// Thunder locked in. Yoi ⚡
