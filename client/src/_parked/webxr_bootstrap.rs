//! client/src/webxr_bootstrap.rs
//! WebXR Bootstrap + Immersive Powrush Client Entry Point
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use wasm_bindgen::prelude::*;
use web_sys::{window, Window, WebXrSession};
use crate::client_game_loop::ClientGameLoop;
use crate::rbe_client_sync::RbeClientSync;

#[wasm_bindgen]
pub struct PowrushWebXrClient {
    game_loop: ClientGameLoop,
    rbe_sync: RbeClientSync,
    xr_session: Option<WebXrSession>,
}

#[wasm_bindgen]
impl PowrushWebXrClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let game_loop = ClientGameLoop::new();
        let rbe_sync = RbeClientSync::new(game_loop.clone());

        Self {
            game_loop,
            rbe_sync,
            xr_session: None,
        }
    }

    #[wasm_bindgen]
    pub async fn start_webxr(&mut self) -> Result<(), String> {
        let window: Window = window().ok_or("No window")?;
        let navigator = window.navigator();
        let xr = navigator.xr();

        let session = xr.request_session("immersive-vr").await
            .map_err(|e| format!("WebXR session failed: {:?}", e))?;

        self.xr_session = Some(session);
        println!("🌐 Powrush WebXR session started — immersive RBE metaverse active");
        Ok(())
    }

    #[wasm_bindgen]
    pub fn update(&mut self, dt: f32) {
        self.game_loop.update(dt);
    }
}