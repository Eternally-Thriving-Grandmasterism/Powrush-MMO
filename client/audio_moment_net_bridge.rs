/*!
 * client/audio_moment_net_bridge.rs
 * Powrush-MMO — Bridge between realtime audio synthesis and network transport
 *
 * Bevy-native path:
 *   - AudioMomentServerSyncRequest → AudioMomentOutboundQueue
 *   - drain_outbound_to_transport → NativeClientTransportSender (mpsc)
 *   - ServerMessageInbound events → catalog merge
 *
 * WASM path continues via client/main.rs AudioOutbound + try_recv.
 *
 * v21.89.4 | AG-SML v1.0 | TOLC 8 | Permanent PATSAGi
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
use std::sync::mpsc;

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

/// Injected by native host when ClientWsTransport is connected.
/// Holds a clone of the transport outbound sender.
#[derive(Resource)]
pub struct NativeClientTransportSender {
    pub tx: mpsc::Sender<ClientMessage>,
}

impl NativeClientTransportSender {
    pub fn new(tx: mpsc::Sender<ClientMessage>) -> Self {
        Self { tx }
    }

    pub fn send(&self, msg: ClientMessage) -> Result<(), String> {
        self.tx
            .send(msg)
            .map_err(|e| format!("Native transport send failed: {}", e))
    }
}

/// Transport / poll layer pushes inbound ServerMessages into Bevy via this event
#[derive(Event, Clone, Debug)]
pub struct ServerMessageInbound {
    pub message: ServerMessage,
}

#[derive(Resource, Default)]
pub struct AudioNetBridgeStats {
    pub drained: u64,
    pub send_failures: u64,
    pub catalog_merges: u64,
}

pub struct AudioMomentNetBridgePlugin;

impl Plugin for AudioMomentNetBridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioMomentOutboundQueue>()
            .init_resource::<AudioNetBridgeStats>()
            .add_event::<ServerMessageInbound>()
            .add_systems(
                Update,
                (
                    enqueue_server_sync_requests,
                    drain_outbound_to_transport,
                    apply_inbound_server_messages,
                )
                    .chain(),
            );
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

/// Bevy native drain — requires NativeClientTransportSender resource
fn drain_outbound_to_transport(
    mut queue: ResMut<AudioMomentOutboundQueue>,
    sender: Option<Res<NativeClientTransportSender>>,
    mut stats: ResMut<AudioNetBridgeStats>,
) {
    let Some(sender) = sender else {
        return;
    };
    let pending = queue.drain();
    for msg in pending {
        match sender.send(msg) {
            Ok(()) => {
                stats.drained = stats.drained.saturating_add(1);
            }
            Err(e) => {
                stats.send_failures = stats.send_failures.saturating_add(1);
                warn!(target: "powrush::audio", error = %e, "Failed to drain audio outbound");
            }
        }
    }
}

fn apply_inbound_server_messages(
    mut events: EventReader<ServerMessageInbound>,
    mut catalog: ResMut<AudioMomentCatalog>,
    cfg: Res<RealtimeAudioConfig>,
    mut stats: ResMut<AudioNetBridgeStats>,
) {
    for ev in events.read() {
        let before = catalog.moments.len();
        handle_server_audio_message(&ev.message, &mut catalog, &cfg);
        if catalog.moments.len() != before {
            stats.catalog_merges = stats.catalog_merges.saturating_add(1);
        }
    }
}

/// Call from transport poll when a ServerMessage arrives (WASM or non-Bevy)
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

/// Helper: native host wires ClientWsTransport-like channel into Bevy
pub fn inject_native_transport_sender(
    commands: &mut Commands,
    tx: mpsc::Sender<ClientMessage>,
) {
    commands.insert_resource(NativeClientTransportSender::new(tx));
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

// Native host pattern:
//   let (tx, rx) = std::sync::mpsc::channel();
//   // forward rx → ClientWsTransport.send in a thread
//   inject_native_transport_sender(&mut commands, tx);
//   // on try_recv: events.send(ServerMessageInbound { message });
// Thunder locked in. Yoi ⚡
