use leptos::*;
use crate::signals::simulation_state::{SimulationState, Telemetry, Archetype};

#[component]
pub fn TelemetryPanel() -> impl IntoView {
    let state = expect_context::<SimulationState>();
    
    let telemetry = state.telemetry;
    let archetypes = state.archetypes;
    let ticks = state.ticks_completed;
    let last_duration = state.last_tick_duration_ms;
    
    view! {
        <div class="mercy-card rounded-2xl p-6">
            <div class="flex items-center justify-between mb-6">
                <h3 class="text-xl font-semibold flex items-center gap-2">
                    <span>⚡</span> Live Telemetry
                </h3>
                <div class="text-xs px-3 py-1 rounded-full bg-[#00f0ff]/10 text-[#00f0ff]">
                    {move || format!("Tick #{}", ticks.get())}
                </div>
            </div>

            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
                <TelemetryMetric label="RBE Depletion" value={move || format!("{:.1}%", telemetry.get().rbe_depletion)} color="#ef4444" />
                <TelemetryMetric label="Abundance Flow" value={move || format!("{:.1}%", telemetry.get().abundance_flow)} color="#22c55e" />
                <TelemetryMetric label="Sustainability" value={move || format!("{:.1}%", telemetry.get().sustainability)} color="#3b82f6" />
                <TelemetryMetric label="Stress Level" value={move || format!("{:.1}%", telemetry.get().stress)} color="#eab308" />
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
                <div class="bg-[#0a0f1e] rounded-xl p-4 border border-[#00f0ff]/20">
                    <div class="text-xs text-[#00f0ff]/60 mb-1">MERCY FLOW</div>
                    <div class="text-3xl font-mono telemetry-value">{move || format!("{:.1}%", telemetry.get().mercy_flow)}</div>
                </div>
                <div class="bg-[#0a0f1e] rounded-xl p-4 border border-[#00f0ff]/20">
                    <div class="text-xs text-[#00f0ff]/60 mb-1">ANOMALIES</div>
                    <div class="text-3xl font-mono telemetry-value">{move || telemetry.get().anomaly_count}</div>
                </div>
                <div class="bg-[#0a0f1e] rounded-xl p-4 border border-[#00f0ff]/20">
                    <div class="text-xs text-[#00f0ff]/60 mb-1">LAST TICK</div>
                    <div class="text-3xl font-mono telemetry-value">{move || format!("{:.1}ms", last_duration.get())}</div>
                </div>
            </div>

            <div>
                <div class="text-sm font-medium mb-3 text-[#00f0ff]/80">ARCHETYPE EVOLUTION</div>
                <div class="space-y-2">
                    {move || archetypes.get().into_iter().map(|a| view! {
                        <div class="flex items-center justify-between bg-[#0a0f1e] px-4 py-2 rounded-lg border border-[#00f0ff]/10">
                            <div class="flex items-center gap-3">
                                <span class="font-medium">{a.name}</span>
                                <span class="text-xs px-2 py-0.5 rounded bg-[#f4d35e]/10 text-[#f4d35e]">Stage {a.evolution_stage}</span>
                            </div>
                            <div class="font-mono text-sm">{a.count} agents</div>
                        </div>
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

#[component]
fn TelemetryMetric(label: &'static str, value: impl Fn() -> String + 'static, color: &'static str) -> impl IntoView {
    view! {
        <div class="bg-[#0a0f1e] rounded-xl p-4 border border-[#00f0ff]/20">
            <div class="text-xs text-[#00f0ff]/60 mb-1">{label}</div>
            <div class="text-3xl font-mono telemetry-value" style={format!("color: {}", color)}>
                {value}
            </div>
        </div>
    }
}