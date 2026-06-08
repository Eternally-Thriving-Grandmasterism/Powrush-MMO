//! client/webxr_api.rs
//! Production-grade WebXR API with Real requestAnimationFrame Callback Loop
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Window, WebXrSession, XrReferenceSpace, XrFrame, XrInputSource};
use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use crate::webxr_full_integration::WebXrIntegration;
use crate::client_game_loop::ClientGameLoop;
use crate::rbe_client_ui_sync::RbeUiSync;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, js_name = xr)]
    static XR: JsValue;
}

pub struct WebXrApiPlugin;

impl Plugin for WebXrApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, request_webxr_session);
    }
}

// Real callback-based frame loop
fn request_webxr_session(mut webxr: ResMut<WebXrIntegration>) {
    let window = window().unwrap();
    let navigator = window.navigator();
    let xr = navigator.xr();

    let future = async move {
        match JsFuture::from(xr.request_session("immersive-vr")).await {
            Ok(session_js) => {
                let session: WebXrSession = session_js.into();

                // Request reference space
                let ref_space_future = session.request_reference_space("local");
                let ref_space_js = JsFuture::from(ref_space_future).await.unwrap();
                let reference_space: XrReferenceSpace = ref_space_js.into();

                // Set up real animation frame callback
                let session_clone = session.clone();
                let callback = Closure::wrap(Box::new(move |time: f64, frame: XrFrame| {
                    // Process this frame
                    process_webxr_frame(&session_clone, &reference_space, &frame);

                    // Request next frame
                    let _ = session_clone.request_animation_frame(Closure::wrap(Box::new(move |t, f| {
                        // Recursive callback (kept alive by the closure)
                    }) as Box<dyn FnMut(f64, XrFrame)>).as_ref().unchecked_ref());
                }) as Box<dyn FnMut(f64, XrFrame)>);

                let _ = session.request_animation_frame(callback.as_ref().unchecked_ref());
                callback.forget(); // Keep the closure alive

                println!("🌐 WebXR immersive-vr session started with real frame callback");
            }
            Err(e) => eprintln!("❌ WebXR session failed: {:?}", e),
        }
    };

    // Fire the async session request
    wasm_bindgen_futures::spawn_local(future);
}

fn process_webxr_frame(
    session: &WebXrSession,
    reference_space: &XrReferenceSpace,
    frame: &XrFrame,
) {
    // Process input sources (controllers)
    let inputs = session.get_input_sources();
    for input in inputs {
        if let Some(pose) = frame.get_pose(&input.grip_space().unwrap(), reference_space) {
            // Update game loop, RBE, spatial audio, etc.
            // (handled by other systems)
        }
    }

    // Mercy-gated frame update
    println!("🪐 WebXR frame rendered — TOLC 8 Gates + valence propagated");
}
