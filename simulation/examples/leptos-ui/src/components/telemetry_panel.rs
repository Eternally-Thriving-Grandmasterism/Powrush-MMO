// simulation/examples/leptos-ui/src/components/telemetry_panel.rs
// v17.99.18 — Enhanced with Export + Archetype Evolution Tree
// Ra-Thor + PATSAGi Councils

use leptos::*;
use crate::signals::simulation_state::{Telemetry, Archetype};
use crate::utils::wasm_bridge;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn TelemetryPanel(
    telemetry: ReadSignal<Telemetry>,
    archetypes: ReadSignal<Vec<Archetype>>,
    set_intervention_log: WriteSignal<Vec<wasm_bridge::InterventionLogEntry>>,
) -> impl IntoView {
    let (export_status, set_export_status) = create_signal(String::new());

    let export_json = move |_| {
        let tel = telemetry.get();
        let json = serde_json::to_string_pretty(&tel).unwrap_or_default();
        // In real app: trigger download
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("a")
            .map(|a| {
                a.set_attribute("href", &format!("data:application/json,{}", urlencoding::encode(&json))).ok();
                a.set_attribute("download", &format!("powrush_telemetry_tick_{}.json", tel.tick)).ok();
                a.click();
            }).ok();
        set_export_status.set("Exported JSON ✓".into());
        set_timeout(move || set_export_status.set(String::new()), 2000);
    };

    let export_csv = move |_| {
        let tel = telemetry.get();
        let csv = format!(
            "tick,rbe_depletion,abundance_flow,sustainability,stress,mercy_flow,archetype_count,entropy_events,serverwar_active,last_tick_ms\n{},{},{},{},{},{},{},{},{},{}",
            tel.tick, tel.rbe_depletion, tel.abundance_flow, tel.sustainability, tel.stress,
            tel.mercy_flow, tel.archetype_count, tel.entropy_events, tel.serverwar_active, tel.last_tick_ms
        );
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("a")
            .map(|a| {
                a.set_attribute("href", &format!("data:text/csv,{}", urlencoding::encode(&csv))).ok();
                a.set_attribute("download", &format!("powrush_telemetry_tick_{}.csv", tel.tick)).ok();
                a.click();
            }).ok();
        set_export_status.set("Exported CSV ✓".into());
        set_timeout(move || set_export_status.set(String::new()), 2000);
    };

    view! {
        <div class="mercy-card p-6 rounded-2xl">
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-xl font-semibold text-[#00f0ff]">LIVE TELEMETRY — RBE SUSTAINABILITY VECTOR</h3>
                <div class="flex gap-2">
                    <button on:click=export_json class="thunder-button px-4 py-1.5 rounded-xl text-sm font-bold">
                        Export JSON
                    </button>
                    <button on:click=export_csv class="thunder-button px-4 py-1.5 rounded-xl text-sm font-bold">
                        Export CSV
                    </button>
                </div>
            </div>
            
            <Show when=move || !export_status.get().is_empty()>
                <div class="text-[#f4d35e] text-sm mb-3">{export_status}</div>
            </Show>

            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                <div class="bg-[#0a0f1e] p-4 rounded-xl border border-[#00f0ff]/30">
                    <div class="text-xs text-[#e0e7ff]/60">TICK</div>
                    <div class="text-3xl font-mono text-[#f4d35e]">{move || telemetry.get().tick}</div>
                </div>
                <div class="bg-[#0a0f1e] p-4 rounded-xl border border-[#00f0ff]/30">
                    <div class="text-xs text-[#e0e7ff]/60">RBE DEPLETION</div>
                    <div class="text-3xl font-mono text-red-400">{move || format!("{:.1}%", telemetry.get().rbe_depletion)}</div>
                </div>
                <div class="bg-[#0a0f1e] p-4 rounded-xl border border-[#00f0ff]/30">
                    <div class="text-xs text-[#e0e7ff]/60">ABUNDANCE FLOW</div>
                    <div class="text-3xl font-mono text-emerald-400">{move || format!("{:.1}%", telemetry.get().abundance_flow)}</div>
                </div>
                <div class="bg-[#0a0f1e] p-4 rounded-xl border border-[#00f0ff]/30">
                    <div class="text-xs text-[#e0e7ff]/60">SUSTAINABILITY</div>
                    <div class="text-3xl font-mono text-[#00f0ff]">{move || format!("{:.1}%", telemetry.get().sustainability)}</div>
                </div>
            </div>

            <div class="mb-6">
                <h4 class="text-sm font-semibold mb-2 text-[#f4d35e]">ARCHETYPE EVOLUTION TREE (LIVE)</h4>
                <div class="bg-[#0a0f1e] p-4 rounded-xl border border-[#00f0ff]/20">
                    <svg width="100%" height="160" viewBox="0 0 800 160" class="overflow-visible">
                        // Simple reactive SVG tree
                        {move || {
                            let arches = archetypes.get();
                            let mut nodes = String::new();
                            let stages = ["Seedling", "Sapling", "Mature", "Apex"];
                            let x_positions = [120.0, 300.0, 480.0, 660.0];
                            
                            for (i, stage) in stages.iter().enumerate() {
                                let count: u32 = arches.iter().filter(|a| a.stage == *stage).map(|a| a.count).sum();
                                let color = match *stage {
                                    "Seedling" => "#22c55e",
                                    "Sapling" => "#eab308",
                                    "Mature" => "#3b82f6",
                                    "Apex" => "#a855f7",
                                    _ => "#64748b",
                                };
                                let y = 80.0;
                                nodes.push_str(&format!(
                                    r#"<g>
                                        <circle cx="{}" cy="{}" r="28" fill="{}" stroke="#00f0ff" stroke-width="2"/>
                                        <text x="{}" y="{}" text-anchor="middle" fill="white" font-size="11" font-weight="bold">{}</text>
                                        <text x="{}" y="{}" text-anchor="middle" fill="#e0e7ff" font-size="10">{} beings</text>
                                    </g>"#,
                                    x_positions[i], y, color,
                                    x_positions[i], y-2, stage,
                                    x_positions[i], y+18, count
                                ));
                            }
                            // Connection lines
                            for i in 0..3 {
                                nodes.push_str(&format!(
                                    r#"<line x1="{}" y1="80" x2="{}" y2="80" stroke="#00f0ff" stroke-width="2" stroke-dasharray="4,2"/>"#,
                                    x_positions[i] + 30.0, x_positions[i+1] - 30.0
                                ));
                            }
                            nodes
                        }}
                    </svg>
                    <div class="text-xs text-center text-[#e0e7ff]/60 mt-2">Evolution flows left → right • Reactive to PATSAGi interventions</div>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
                <div>
                    <div class="text-xs text-[#e0e7ff]/60">MERCY FLOW</div>
                    <div class="font-mono text-[#f4d35e]">{move || format!("{:.1}%", telemetry.get().mercy_flow)}</div>
                </div>
                <div>
                    <div class="text-xs text-[#e0e7ff]/60">STRESS</div>
                    <div class="font-mono text-orange-400">{move || format!("{:.1}%", telemetry.get().stress)}</div>
                </div>
                <div>
                    <div class="text-xs text-[#e0e7ff]/60">ENTROPY EVENTS / SERVERWAR</div>
                    <div class="font-mono">{move || format!("{} / {}", telemetry.get().entropy_events, if telemetry.get().serverwar_active { "ACTIVE" } else { "dormant" })}</div>
                </div>
            </div>
        </div>
    }
}
