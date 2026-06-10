use leptos::*;

#[component]
pub fn ScenarioSelector(current: RwSignal<String>) -> impl IntoView {
    let presets = vec![
        "Balanced RBE Emergence",
        "High Mercy Flow",
        "Archetype Explosion",
        "ServerWar Stress Test",
        "Abundance Cascade",
        "Zero Entropy Genesis",
    ];
    
    view! {
        <div class="flex items-center gap-2">
            <label class="text-xs text-[#00f0ff]/60">SCENARIO</label>
            <select 
                class="bg-[#0a0f1e] border border-[#00f0ff]/30 rounded-lg px-4 py-2 text-sm font-medium min-w-[220px]"
                on:change=move |ev| current.set(event_target_value(&ev))
            >
                {presets.into_iter().map(|p| {
                    let selected = current.get() == p;
                    view! { <option value={p.clone()} selected={selected}>{p}</option> }
                }).collect_view()}
            </select>
        </div>
    }
}