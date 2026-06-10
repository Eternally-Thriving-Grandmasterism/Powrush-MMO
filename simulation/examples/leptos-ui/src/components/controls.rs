use leptos::*;
use crate::signals::simulation_state::SimulationState;
use crate::utils::wasm_bridge::execute_run;
use crate::components::scenario_selector::ScenarioSelector;

#[component]
pub fn Controls() -> impl IntoView {
    let state = expect_context::<SimulationState>();
    
    let is_running = state.is_running;
    let is_paused = state.is_paused;
    let use_gpu = state.use_gpu;
    let ticks = create_rw_signal(240u32);
    let preset = state.current_preset;
    
    let on_run = move |_| {
        let p = preset.get();
        let t = ticks.get();
        let gpu = use_gpu.get();
        let s = state.clone();
        spawn_local(async move {
            execute_run(s, p, t, gpu).await;
        });
    };
    
    let on_pause = move |_| {
        state.is_paused.update(|v| *v = !*v);
    };
    
    let on_step = move |_| {
        state.step_one_tick();
    };
    
    let toggle_gpu = move |_| {
        use_gpu.update(|v| *v = !*v);
    };
    
    view! {
        <div class="mercy-card rounded-2xl p-6">
            <div class="flex flex-wrap items-center gap-4">
                <ScenarioSelector current={preset} />
                
                <div class="flex items-center gap-2">
                    <label class="text-xs text-[#00f0ff]/60">TICKS</label>
                    <input 
                        type="number" 
                        class="w-24 bg-[#0a0f1e] border border-[#00f0ff]/30 rounded-lg px-3 py-1.5 text-sm font-mono"
                        prop:value={move || ticks.get()}
                        on:input=move |ev| if let Ok(v) = event_target_value(&ev).parse::<u32>() { ticks.set(v); }
                    />
                </div>

                <div class="flex-1"></div>

                <button 
                    class="thunder-button px-8 py-2.5 rounded-xl text-sm disabled:opacity-60"
                    disabled={move || is_running.get()}
                    on:click=on_run
                >
                    {move || if is_running.get() { "RUNNING..." } else { "RUN SIMULATION" }}
                </button>

                <button 
                    class="px-6 py-2.5 rounded-xl border border-[#00f0ff]/40 text-sm font-medium"
                    on:click=on_pause
                >
                    {move || if is_paused.get() { "RESUME" } else { "PAUSE" }}
                </button>

                <button 
                    class="px-6 py-2.5 rounded-xl border border-[#00f0ff]/40 text-sm font-medium"
                    on:click=on_step
                >
                    STEP 1 TICK
                </button>

                <button 
                    class= {move || format!("px-5 py-2.5 rounded-xl text-sm font-medium border {}", if use_gpu.get() { "border-[#22c55e] text-[#22c55e]" } else { "border-[#00f0ff]/40" }) }
                    on:click=toggle_gpu
                >
                    {move || if use_gpu.get() { "⚡ GPU ON" } else { "CPU MODE" }}
                </button>
            </div>
        </div>
    }
}