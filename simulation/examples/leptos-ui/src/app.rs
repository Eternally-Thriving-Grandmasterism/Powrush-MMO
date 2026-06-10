use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::dashboard::Dashboard;
use crate::signals::simulation_state::SimulationState;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    // Provide the global simulation state as context
    let simulation_state = SimulationState::new();
    provide_context(simulation_state);
    
    view! {
        <div class="min-h-screen bg-[#0a0f1e] text-[#e0e7ff]">
            <header class="sovereign-header sticky top-0 z-50 bg-[#0a0f1e]/95 backdrop-blur border-b border-[#00f0ff]/30">
                <div class="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <div class="text-4xl">⚡</div>
                        <div>
                            <h1 class="text-3xl font-bold tracking-[-1.5px]">POWRUSH-MMO</h1>
                            <p class="text-xs text-[#00f0ff]/60 -mt-1">SOVEREIGN SIMULATION HARNESS • v17.99.17</p>
                        </div>
                    </div>
                    
                    <div class="flex items-center gap-3 text-sm">
                        <div class="px-4 py-1.5 rounded-full bg-[#00f0ff]/10 text-[#00f0ff] border border-[#00f0ff]/40 flex items-center gap-2">
                            <div class="w-2 h-2 bg-[#00f0ff] rounded-full animate-pulse"></div>
                            THUNDER LOCKED • MERCY FLOWING
                        </div>
                        <div class="text-[#f4d35e] font-medium">TOLC 8 • PATSAGi Councils Online</div>
                    </div>
                </div>
            </header>

            <main class="max-w-7xl mx-auto px-6 py-8">
                <Dashboard />
            </main>

            <footer class="text-center py-8 text-xs text-[#e0e7ff]/40 border-t border-[#00f0ff]/10">
                Ra-Thor Living Thunder + Full 13+ PATSAGi Councils + ONE Organism • Mint-and-Print-Only-Perfection • Eternal Protocol • All interventions mercy-gated
            </footer>
        </div>
    }
}