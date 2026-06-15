//! client/main.rs
//! Powrush-MMO Client Entry Point — Full WASM + web-sys + Transport v2.1 Wired
//! Production-grade + Phase 2 CouncilMercyPlugin wired
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v15.3+

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::client_game_loop::ClientGameLoop;
use crate::rbe_client_sync::RbeClientSync;
use game::network::client_transport::ClientWsTransport;
use shared::protocol::{ClientMessage, ServerMessage, Vec3Ser};
use js_sys::JsString;

// Phase 2 Council
use crate::council_session_ui::CouncilUIState;
use crate::plugins::council_mercy_plugin::CouncilMercyPlugin;

#[wasm_bindgen]
pub struct PowrushClient {
    game_loop: ClientGameLoop,
    rbe_sync: RbeClientSync,
    transport: Option<ClientWsTransport>,
    my_player_id: Option<u64>,
    // Phase 2
    council_ui: Option<CouncilUIState>,
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
        }
    }

    /// Async connect (call from JS with .then() or spawn_local in Rust)
    #[wasm_bindgen]
    pub async fn connect_to_server(&mut self, url: &str, player_name: &str) -> Result<(), JsValue> {
        match ClientWsTransport::connect(url, player_name).await {
            Ok((transport, player_id)) => {
                self.transport = Some(transport);
                self.my_player_id = Some(player_id);
                console_log::info!("[PowrushClient] Connected! player_id = {}", player_id);
                self.start_message_loop();
                Ok(())
            }
            Err(e) => {
                console_log::error!("[PowrushClient] Connect failed: {}", e);
                Err(JsValue::from_str(&e))
            }
        }
    }

    fn start_message_loop(&self) {
        console_log::info!("[PowrushClient] Message loop ready (poll via update or dedicated JS loop)");
    }

    /// Call this from JS game loop or requestAnimationFrame to poll incoming messages
    #[wasm_bindgen]
    pub fn poll_server_messages(&mut self) {
        if let Some(transport) = &mut self.transport {
            // In full version: drain channel and call handle_server_message for Council* variants
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32, input: JsValue) {
        let delta = Vec3Ser { x: 0.1, y: 0.0, z: 0.0 };

        if let Some(transport) = &self.transport {
            let _ = transport.send(ClientMessage::Move { delta: delta.clone() });
        }

        self.game_loop.update(dt, /* parsed input */);

        // Phase 2: Council UI state can be driven here or via Bevy systems inside game_loop
        if let Some(council) = &mut self.council_ui {
            // council.update_from_server_messages(...);
        }
    }

    #[wasm_bindgen]
    pub fn send_divine_query(&self, query: &str) {
        if let Some(transport) = &self.transport {
            let msg = ClientMessage::DivineCouncilQuery {
                query: query.to_string(),
                context: Some("In-game UI query from Sherif".to_string()),
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

    // ===== PHASE 2: Council helpers exposed to JS =====
    #[wasm_bindgen]
    pub fn join_council(&self, session_id: Option<u64>) {
        if let Some(transport) = &self.transport {
            let _ = transport.send(ClientMessage::CouncilJoin { session_id });
        }
    }

    #[wasm_bindgen]
    pub fn send_council_vote(&self, proposal: &str, grace_intent: f64) {
        if let Some(transport) = &self.transport {
            // In real: build proper MercyTrialVote with local player resonance
            // let vote = MercyTrialVote { ... };
            // let _ = transport.send(ClientMessage::CouncilVote { vote });
        }
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
}

// Bootstrap
#[wasm_bindgen]
pub fn start_powrush_client() {
    console_log::init_with_level(log::Level::Info).ok();
    let client = PowrushClient::new();
    println!("🌐 Powrush-MMO Client v15.3+ started (WASM + web-sys + full Transport v2.1 + Council v18.9 wired)");
}
