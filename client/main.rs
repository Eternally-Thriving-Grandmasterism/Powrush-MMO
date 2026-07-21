//! client/main.rs
//! Powrush-MMO Client Entry Point — WASM + web-sys + Transport v2.3
//! v21.89.3 — Full try_recv poll, heartbeat, audio catalog on connect
//! AG-SML v1.0 | TOLC 8 Mercy Gates | ONE Organism | Ra-Thor Sovereign Client

use wasm_bindgen::prelude::*;
use crate::client_game_loop::ClientGameLoop;
use crate::rbe_client_sync::RbeClientSync;
use game::network::client_transport::ClientWsTransport;
use shared::protocol::{ClientMessage, ServerMessage, Vec3Ser, WireAudioMoment};
use std::collections::VecDeque;

use crate::council_session_ui::CouncilUIState;

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
    last_audio_catalog: Vec<WireAudioMoment>,
    last_audio_ack: Option<String>,
    catalog_requested: bool,
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
            catalog_requested: false,
        }
    }

    #[wasm_bindgen]
    pub async fn connect_to_server(&mut self, url: &str, player_name: &str) -> Result<(), JsValue> {
        match ClientWsTransport::connect(url, player_name).await {
            Ok((transport, player_id)) => {
                self.transport = Some(transport);
                if player_id != 0 {
                    self.my_player_id = Some(player_id);
                }
                console_log::info!("[PowrushClient] Connected (handshake pending if id=0)");
                Ok(())
            }
            Err(e) => {
                console_log::error!("[PowrushClient] Connect failed: {}", e);
                Err(JsValue::from_str(&e))
            }
        }
    }

    /// Poll inbound server messages + flush outbound audio + heartbeat
    #[wasm_bindgen]
    pub fn poll_server_messages(&mut self) {
        self.flush_audio_outbound();

        if let Some(transport) = &mut self.transport {
            transport.tick_heartbeat();

            while let Some(msg) = transport.try_recv() {
                // Capture player_id from handshake
                if let ServerMessage::HandshakeResponse {
                    player_id,
                    accepted,
                    ..
                } = &msg
                {
                    if *accepted {
                        self.my_player_id = Some(*player_id);
                        console_log::info!(
                            "[PowrushClient] Handshake accepted player_id={}",
                            player_id
                        );
                        if !self.catalog_requested {
                            self.audio_outbound.push_catalog_request(*player_id);
                            self.catalog_requested = true;
                        }
                    }
                }
                self.route_server_message(msg);
            }
        }

        // Flush again if handshake queued catalog request
        self.flush_audio_outbound();
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
            for msg in pending {
                self.audio_outbound.messages.push_back(msg);
            }
        }
    }

    pub fn route_server_message(&mut self, msg: ServerMessage) {
        match msg {
            ServerMessage::AudioMomentCatalogSnapshot {
                player_id,
                moments,
                next_id,
                last_synced_unix,
            } => {
                console_log::info!(
                    "[PowrushClient] Audio catalog player={} count={} next_id={} synced={}",
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
            ServerMessage::Pong {
                client_time_ms,
                server_time_ms,
            } => {
                console_log::trace!(
                    "[PowrushClient] Pong rtt≈{}ms",
                    server_time_ms.saturating_sub(client_time_ms)
                );
            }
            _ => {}
        }
    }

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
        // Full network poll every frame
        self.poll_server_messages();
        self.game_loop.update(dt);
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
        "🌐 Powrush-MMO Client v21.89.3 started (try_recv + AudioMoments + Council)"
    );
}
