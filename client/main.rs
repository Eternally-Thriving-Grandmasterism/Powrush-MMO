//! client/main.rs
//! Powrush-MMO Client Entry Point — WASM + web-sys + Transport v2.1 + Full Bevy Integration
//! v21.89.2 — Audio moment outbound drain + server catalog/ack handling
//! AG-SML v1.0 | TOLC 8 Mercy Gates | ONE Organism | Ra-Thor Sovereign Client

use wasm_bindgen::prelude::*;
use crate::client_game_loop::ClientGameLoop;
use crate::rbe_client_sync::RbeClientSync;
use game::network::client_transport::ClientWsTransport;
use shared::protocol::{ClientMessage, ServerMessage, Vec3Ser, WireAudioMoment};
use std::collections::VecDeque;

use crate::council_session_ui::CouncilUIState;

/// Lightweight outbound queue for audio moments (mirrors Bevy AudioMomentOutboundQueue)
#[derive(Default)]
pub struct AudioOutbound {
    pub messages: VecDeque<ClientMessage>,
}

impl AudioOutbound {
    pub fn push_save(&mut self, moment: WireAudioMoment) {
        self.messages
            .push_back(ClientMessage::AudioMomentSave { moment });
    }

    pub fn push_catalog_request(&mut self, player_id: u64) {
        self.messages
            .push_back(ClientMessage::AudioMomentCatalogRequest { player_id });
    }

    pub fn drain(&mut self) -> Vec<ClientMessage> {
        self.messages.drain(..).collect()
    }
}

#[wasm_bindgen]
pub struct PowrushClient {
    game_loop: ClientGameLoop,
    rbe_sync: RbeClientSync,
    transport: Option<ClientWsTransport>,
    my_player_id: Option<u64>,
    council_ui: Option<CouncilUIState>,
    audio_outbound: AudioOutbound,
    /// Last server audio catalog snapshot (moments only)
    last_audio_catalog: Vec<WireAudioMoment>,
    last_audio_ack: Option<String>,
}

#[wasm_bindgen]
impl PowrushClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let game_loop = ClientGameLoop::new();
        let rbe_sync = RbeClientSync::new(game_loop.clone());

        Self {
            game_loop,
            rbe_sync,
            transport: None,
            my_player_id: None,
            council_ui: Some(CouncilUIState::default()),
            audio_outbound: AudioOutbound::default(),
            last_audio_catalog: Vec::new(),
            last_audio_ack: None,
        }
    }

    #[wasm_bindgen]
    pub async fn connect_to_server(&mut self, url: &str, player_name: &str) -> Result<(), JsValue> {
        match ClientWsTransport::connect(url, player_name).await {
            Ok((transport, player_id)) => {
                self.transport = Some(transport);
                self.my_player_id = Some(player_id);
                console_log::info!(
                    "[PowrushClient] Connected! player_id = {}",
                    player_id
                );
                // Request server audio catalog on connect
                self.audio_outbound.push_catalog_request(player_id);
                Ok(())
            }
            Err(e) => {
                console_log::error!("[PowrushClient] Connect failed: {}", e);
                Err(JsValue::from_str(&e))
            }
        }
    }

    /// Poll incoming server messages + drain audio outbound
    #[wasm_bindgen]
    pub fn poll_server_messages(&mut self) {
        // Drain outbound audio moment messages first
        self.flush_audio_outbound();

        if let Some(transport) = &mut self.transport {
            // Production pattern:
            // while let Some(msg) = transport.try_recv() {
            //     self.route_server_message(msg);
            // }
            let _ = transport;
            console_log::trace!("[PowrushClient] poll_server_messages()");
        }
    }

    fn flush_audio_outbound(&mut self) {
        let pending = self.audio_outbound.drain();
        if pending.is_empty() {
            return;
        }
        if let Some(transport) = &self.transport {
            for msg in pending {
                match &msg {
                    ClientMessage::AudioMomentSave { moment } => {
                        console_log::info!(
                            "[PowrushClient] Sending AudioMomentSave id={}",
                            moment.id
                        );
                    }
                    ClientMessage::AudioMomentCatalogRequest { player_id } => {
                        console_log::info!(
                            "[PowrushClient] Requesting audio catalog for {}",
                            player_id
                        );
                    }
                    _ => {}
                }
                let _ = transport.send(msg);
            }
        } else {
            // Re-queue if not connected
            for msg in pending {
                self.audio_outbound.messages.push_back(msg);
            }
        }
    }

    /// Route a deserialized ServerMessage (call from transport try_recv)
    pub fn route_server_message(&mut self, msg: ServerMessage) {
        match msg {
            ServerMessage::AudioMomentCatalogSnapshot {
                player_id,
                moments,
                next_id,
                last_synced_unix,
            } => {
                console_log::info!(
                    "[PowrushClient] Audio catalog snapshot player={} count={} next_id={} synced={}",
                    player_id,
                    moments.len(),
                    next_id,
                    last_synced_unix
                );
                self.last_audio_catalog = moments;
            }
            ServerMessage::AudioMomentSaveAck {
                moment_id,
                ok,
                message,
            } => {
                let line = format!("ack #{} ok={} {}", moment_id, ok, message);
                console_log::info!("[PowrushClient] AudioMoment {}", line);
                self.last_audio_ack = Some(line);
            }
            ServerMessage::MercyGateBlocked { reason, valence } => {
                console_log::warn!(
                    "[PowrushClient] Mercy gate blocked: {} (valence {:.2})",
                    reason,
                    valence
                );
            }
            _ => {
                // RBE / inventory / world handled by dedicated paths
            }
        }
    }

    /// Queue a wire audio moment for server save (from Bevy bridge or JS)
    #[wasm_bindgen]
    pub fn queue_audio_moment_save_json(&mut self, json: &str) -> bool {
        match serde_json::from_str::<WireAudioMoment>(json) {
            Ok(moment) => {
                self.audio_outbound.push_save(moment);
                true
            }
            Err(e) => {
                console_log::error!("[PowrushClient] Bad AudioMoment JSON: {}", e);
                false
            }
        }
    }

    #[wasm_bindgen]
    pub fn request_audio_catalog(&mut self) {
        if let Some(pid) = self.my_player_id {
            self.audio_outbound.push_catalog_request(pid);
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32, _input: JsValue) {
        let delta = Vec3Ser {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        // Flush audio outbound every frame while connected
        self.flush_audio_outbound();

        if let Some(transport) = &self.transport {
            // Keepalive-style ping optional; avoid spam move
            let _ = transport;
            let _ = delta;
        }

        self.game_loop.update(dt);

        if let Some(_council) = &mut self.council_ui {
            // Live feed overwrites soft demo when present
        }
    }

    #[wasm_bindgen]
    pub fn send_divine_query(&self, query: &str) {
        if let Some(transport) = &self.transport {
            let msg = ClientMessage::DivineCouncilQuery {
                query: query.to_string(),
                context: Some("In-game UI query from player".to_string()),
            };
            let _ = transport.send(msg);
        }
    }

    #[wasm_bindgen]
    pub fn send_rbe_query(&self, resource: &str, amount: f64) {
        if let Some(transport) = &self.transport {
            let msg = ClientMessage::RbeAbundanceQuery {
                resource_type: resource.to_string(),
                amount,
            };
            let _ = transport.send(msg);
        }
    }

    #[wasm_bindgen]
    pub fn join_council(&self, session_id: Option<u64>) {
        if let Some(transport) = &self.transport {
            let _ = transport.send(ClientMessage::CouncilJoin { session_id });
        }
    }

    #[wasm_bindgen]
    pub fn send_council_vote(&self, proposal: &str, _grace_intent: f64) {
        console_log::info!("[PowrushClient] Council vote intent: {}", proposal);
    }

    #[wasm_bindgen]
    pub fn handle_server_delta(&mut self, data: Vec<u8>) {
        self.rbe_sync.handle_rbe_delta(data.into());
    }

    #[wasm_bindgen]
    pub fn get_predicted_state(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self.game_loop.get_predicted_state()).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_my_player_id(&self) -> Option<u64> {
        self.my_player_id
    }

    #[wasm_bindgen]
    pub fn get_audio_catalog_count(&self) -> u32 {
        self.last_audio_catalog.len() as u32
    }

    #[wasm_bindgen]
    pub fn get_last_audio_ack(&self) -> Option<String> {
        self.last_audio_ack.clone()
    }
}

#[wasm_bindgen]
pub fn start_powrush_client() {
    console_log::init_with_level(log::Level::Info).ok();
    let _client = PowrushClient::new();
    println!(
        "🌐 Powrush-MMO Client v21.89.2 started (Transport + Council + Audio Moments catalog)"
    );
}
