use leptos::*;
use crate::signals::simulation_state::SimulationState;

#[component]
pub fn GpuStatus() -> impl IntoView {
    let state = expect_context::<SimulationState>();
    let use_gpu = state.use_gpu;
    
    view! {
        <div class="flex items-center gap-2 text-xs">
            <div class={move || format!("px-3 py-1 rounded-full border flex items-center gap-1.5 {}", 
                if use_gpu.get() { "border-[#22c55e] text-[#22c55e]" } else { "border-[#00f0ff]/40 text-[#00f0ff]/70" }
            )}>
                <div class={move || format!("w-1.5 h-1.5 rounded-full {}", if use_gpu.get() { "bg-[#22c55e]" } else { "bg-[#00f0ff]/70" })}></div>
                {move || if use_gpu.get() { "WebGPU ACCELERATED" } else { "CPU GOLDEN MASTER" }}
            </div>
            <div class="text-[#00f0ff]/40">Workgroups: 256 | Double-buffered</div>
        </div>
    }
}