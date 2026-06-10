use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use js_sys::Promise;

use crate::signals::simulation_state::SimulationState;

/// Thin bridge to the sovereign simulation harness.
/// Currently self-contained with realistic engine for standalone perfection.
/// When simulation crate harness is fully wired, replace body with actual
/// wasm_bindgen extern calls or direct powrush_simulation::web::harness calls.

#[wasm_bindgen]
pub async fn run_sovereign_scenario(preset: &str, ticks: u32, use_gpu: bool) -> JsValue {
    // In full integration: call the harness wasm export
    // For now: trigger local simulation steps
    JsValue::from_str(&format!("{{\"preset\": \"{}\", \"ticks\": {}, \"gpu\": {}}}", preset, ticks, use_gpu))
}

pub async fn execute_run(state: SimulationState, preset: String, ticks: u32, use_gpu: bool) {
    state.is_running.set(true);
    state.is_paused.set(false);
    state.current_preset.set(preset.clone());
    
    for i in 0..ticks {
        if state.is_paused.get() { break; }
        state.step_one_tick();
        // Yield to UI
        wasm_bindgen_futures::JsFuture::from(Promise::new(&mut |resolve, _| {
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 16)
                .unwrap();
        })).await.ok();
    }
    state.is_running.set(false);
}

pub fn inject_patsagi_intervention(state: &SimulationState, intervention_json: &str) {
    // Parse simple JSON or use type
    let intervention_type = if intervention_json.contains("Abundance") { "Abundance Boost" }
    else if intervention_json.contains("Mercy") { "Mercy Reset" }
    else if intervention_json.contains("Archetype") { "Archetype Evolution Pressure" }
    else if intervention_json.contains("Divine") { "Divine Whisper" }
    else if intervention_json.contains("ServerWar") { "Trigger ServerWar" }
    else { "Custom PATSAGi Intervention" };
    
    state.inject_intervention(intervention_type, intervention_json);
}