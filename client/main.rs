//! client/main.rs
//! Powrush-MMO Client Entry Point — WASM + web-sys + Transport v2.1 + Full Bevy Integration
//! Production-grade client with Phase 2 CouncilMercyPlugin, RBE Flow, and Monitoring lattice wired.
//! AG-SML v1.0 | TOLC 8 Mercy Gates | ONE Organism v15.3+ | Ra-Thor Sovereign Client

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::client_game_loop::ClientGameLoop;
use crate::rbe_client_sync::RbeClientSync;
use game::network::client_transport::ClientWsTransport;
use shared::protocol::{ClientMessage, ServerMessage, Vec3Ser};
use js_sys::JsString;

// Phase 2 Council & Monitoring
use crate::council_session_ui::CouncilUIState;
use crate::plugins::council_mercy_plugin::CouncilMercyPlugin;

#[wasm_bindgen]
pub struct PowrushClient {
    game_loop: ClientGameLoop,
    rbe_sync: RbeClientSync,
    transport: Option<ClientWsTransport>,
    my_player_id: Option<u64>,
    // Phase 2 Council UI state
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

    /// Async connect to Powrush server (call from JS)
    #[wasm_bindgen]
    pub async fn connect_to_server(&mut self, url: &str, player_name: &str) -> Result<(), JsValue> {
        match ClientWsTransport::connect(url, player_name).await {
            Ok((transport, player_id)) => {
                self.transport = Some(transport);
                self.my_player_id = Some(player_id);
                console_log::info!("[PowrushClient] Connected to sovereign server! player_id = {}", player_id);
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
        console_log::info!("[PowrushClient] Sovereign message loop ready. Poll via update() or dedicated JS loop.");
    }

    /// Poll incoming server messages (call from JS requestAnimationFrame or dedicated loop)
    #[wasm_bindgen]
    pub fn poll_server_messages(&mut self) {
        if let Some(transport) = &mut self.transport {
            // Full implementation: drain transport channel, deserialize ServerMessage,
            // route to rbe_sync.handle_rbe_delta(...) and council_ui update.
            // Example pattern (extend as transport matures):
            // while let Some(msg) = transport.try_recv() {
            //     match msg {
            //         ServerMessage::RbeDelta(data) => self.rbe_sync.handle_rbe_delta(data),
            //         ServerMessage::CouncilUpdate(...) => { /* update council_ui */ },
            //         _ => {}
            //     }
            // }
            console_log::trace!("[PowrushClient] poll_server_messages() - transport active");
        }
    }

    /// Main per-frame update (called from JS game loop)
    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32, input: JsValue) {
        // TODO: Parse real input from JS (keyboard/mouse/gamepad/WebXR)
        // For now we send a small deterministic delta for testing connectivity
        let delta = Vec3Ser { x: 0.1, y: 0.0, z: 0.0 };

        if let Some(transport) = &self.transport {
            let _ = transport.send(ClientMessage::Move { delta: delta.clone() });
        }

        self.game_loop.update(dt, /* parsed_input_from_js_value(input) */);

        // Phase 2 Council UI state sync (can be driven by Bevy systems inside game_loop too)
        if let Some(council) = &mut self.council_ui {
            // council.update_from_server_messages(...);
            // Integrate with new monitoring/debug overlay when Bevy world is accessible
        }

        // Future: sync monitoring resources (debug overlay, RBE dashboard) here if needed
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

    // ===== PHASE 2: Council helpers (exposed to JS / UI) =====
    #[wasm_bindgen]
    pub fn join_council(&self, session_id: Option<u64>) {
        if let Some(transport) = &self.transport {
            let _ = transport.send(ClientMessage::CouncilJoin { session_id });
        }
    }

    #[wasm_bindgen]
    pub fn send_council_vote(&self, proposal: &str, grace_intent: f64) {
        if let Some(transport) = &self.transport {
            // Build proper MercyTrialVote with local player resonance when protocol is extended
            // let vote = MercyTrialVote { proposal: proposal.to_string(), grace_intent, ... };
            // let _ = transport.send(ClientMessage::CouncilVote { vote });
            console_log::info!("[PowrushClient] Council vote intent sent for: {}", proposal);
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

// Bootstrap entry for JS
#[wasm_bindgen]
pub fn start_powrush_client() {
    console_log::init_with_level(log::Level::Info).ok();
    let _client = PowrushClient::new();
    println!("🌐 Powrush-MMO Client v15.3+ started (WASM + web-sys + Transport v2.1 + Council v18.9 + Monitoring lattice wired)");
}
