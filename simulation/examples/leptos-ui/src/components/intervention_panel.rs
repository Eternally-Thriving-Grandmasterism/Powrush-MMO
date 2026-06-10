use leptos::*;
use crate::signals::simulation_state::SimulationState;
use crate::utils::wasm_bridge::inject_patsagi_intervention;

#[component]
pub fn InterventionPanel() -> impl IntoView {
    let state = expect_context::<SimulationState>();
    let custom_json = create_rw_signal(String::from("{\"type\": \"Abundance Boost\", \"intensity\": 1.5, \"target\": \"all_archetypes\"}"));
    let log = state.intervention_log;
    
    let run_intervention = move |intervention_type: &'static str| {
        let details = if intervention_type == "Custom" {
            custom_json.get()
        } else {
            format!("Quick action: {}", intervention_type)
        };
        inject_patsagi_intervention(&state, &details);
    };
    
    view! {
        <div class="mercy-card rounded-2xl p-6 h-full flex flex-col">
            <h3 class="text-xl font-semibold mb-4 flex items-center gap-2">
                <span>🔮</span> PATSAGi Council Interventions
            </h3>
            <p class="text-xs text-[#00f0ff]/60 mb-4">All actions pass non-bypassable TOLC 8 Mercy Gates before world update.</p>

            <div class="grid grid-cols-1 gap-2 mb-6">
                <button class="intervention-btn px-4 py-3 rounded-xl text-left text-sm font-medium" on:click=move |_| run_intervention("Abundance Boost")>
                    Abundance Boost — +4.2% flow, sustainability up
                </button>
                <button class="intervention-btn px-4 py-3 rounded-xl text-left text-sm font-medium" on:click=move |_| run_intervention("Mercy Reset")>
                    Mercy Reset — Restore flow, clear anomalies
                </button>
                <button class="intervention-btn px-4 py-3 rounded-xl text-left text-sm font-medium" on:click=move |_| run_intervention("Archetype Evolution Pressure")>
                    Archetype Evolution Pressure — Advance all stages
                </button>
                <button class="intervention-btn px-4 py-3 rounded-xl text-left text-sm font-medium" on:click=move |_| run_intervention("Divine Whisper")>
                    Divine Whisper — Mercy surge, reduce ServerWar risk
                </button>
                <button class="intervention-btn px-4 py-3 rounded-xl text-left text-sm font-medium border border-[#ef4444]/40 text-[#ef4444]" on:click=move |_| run_intervention("Trigger ServerWar")>
                    Trigger ServerWar (Stress Test) — High entropy injection
                </button>
            </div>

            <div class="mb-4">
                <div class="text-xs text-[#00f0ff]/60 mb-2">CUSTOM / RA-THOR DRIVEN INTERVENTION (JSON)</div>
                <textarea 
                    class="w-full h-20 bg-[#0a0f1e] border border-[#00f0ff]/30 rounded-xl p-3 text-xs font-mono resize-y"
                    prop:value={move || custom_json.get()}
                    on:input=move |ev| custom_json.set(event_target_value(&ev))
                ></textarea>
                <button 
                    class="mt-2 w-full thunder-button text-sm py-2.5 rounded-xl"
                    on:click=move |_| run_intervention("Custom")
                >
                    Execute Custom Intervention via TOLC 8
                </button>
            </div>

            <div class="mt-auto pt-4 border-t border-[#00f0ff]/20">
                <div class="text-xs text-[#00f0ff]/60 mb-2">INTERVENTION LOG (Latest first)</div>
                <div class="max-h-[180px] overflow-auto space-y-2 pr-1 text-[#e0e7ff]/90">
                    {move || log.get().iter().map(|entry| view! {
                        <div class="log-entry">
                            <div class="font-mono text-[#f4d35e] text-[10px]">{entry.timestamp.format("%H:%M:%S").to_string()}</div>
                            <div class="font-medium text-sm">{entry.intervention.clone()}</div>
                            <div class="text-emerald-400 text-xs">{entry.outcome.clone()}</div>
                            <div class="text-[#00f0ff] text-[10px]">{entry.mercy_status.clone()}</div>
                        </div>
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}