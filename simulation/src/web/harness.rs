// simulation/src/web/harness.rs
// Sovereign WebGPU + WASM Harness for Powrush-MMO
// v17.99.20 | TOLC 8 Layer 0 | Mint-and-Print-Only-Perfection
// Exposes stable #[wasm_bindgen] API for Leptos UI and future clients

use wasm_bindgen::prelude::*;
use js_sys::JsValue;
use crate::{run_sovereign_scenario, inject_patsagi_intervention, step_one_tick, get_current_telemetry};

#[wasm_bindgen]
pub async fn run_sovereign_scenario_real(preset: &str, ticks: u32, use_gpu: bool) -> JsValue {
    // TOLC 8 validation happens inside the core functions
    let result = run_sovereign_scenario(preset, ticks, use_gpu);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub async fn inject_patsagi_intervention_real(intervention_json: &str) -> Result<JsValue, JsValue> {
    match inject_patsagi_intervention(intervention_json) {
        Ok(report) => Ok(serde_wasm_bindgen::to_value(&report).unwrap()),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub async fn step_one_tick_real() -> JsValue {
    let telemetry = step_one_tick();
    serde_wasm_bindgen::to_value(&telemetry).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub async fn get_current_telemetry_real() -> JsValue {
    let telemetry = get_current_telemetry();
    serde_wasm_bindgen::to_value(&telemetry).unwrap_or(JsValue::NULL)
}

// Web Worker message types prepared for future offloading
#[wasm_bindgen]
pub struct WorkerMessage {
    pub cmd: String,
    pub payload: JsValue,
}

#[wasm_bindgen]
impl WorkerMessage {
    #[wasm_bindgen(constructor)]
    pub fn new(cmd: String, payload: JsValue) -> Self {
        Self { cmd, payload }
    }
}