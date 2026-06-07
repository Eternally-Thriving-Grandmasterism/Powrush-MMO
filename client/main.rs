//! client/main.rs
//! Powrush-MMO Client Entry Point — WebXR Ready
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use wasm_bindgen::prelude::*;
use crate::client_game_loop::ClientGameLoop;
use crate::rbe_client_sync::RbeClientSync;

#[wasm_bindgen]
pub struct PowrushClient {
    game_loop: ClientGameLoop,
    rbe_sync: RbeClientSync,
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
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32, input: JsValue) {
        // Convert JS input to ClientInput and update prediction
        self.game_loop.update(dt, /* parsed input */);
    }

    #[wasm_bindgen]
    pub fn handle_server_delta(&mut self, data: Vec<u8>) {
        self.rbe_sync.handle_rbe_delta(data.into());
    }

    #[wasm_bindgen]
    pub fn get_predicted_state(&self) -> JsValue {
        // Return predicted state for rendering (position, rotation, RBE view)
        serde_wasm_bindgen::to_value(self.game_loop.get_predicted_state()).unwrap()
    }
}

// WebXR bootstrap (called from index.html or Tauri)
#[wasm_bindgen]
pub fn start_powrush_client() {
    console_log::init_with_level(log::Level::Info).ok();
    let client = PowrushClient::new();
    // WebXR session setup would go here in full version
    println!("🌐 Powrush-MMO Client v14.6.0+ started (WebXR ready)");
}
