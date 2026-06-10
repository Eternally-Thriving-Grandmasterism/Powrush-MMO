use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <div class="min-h-screen bg-[#0a0f1e] text-[#e0e7ff]">
            <header class="sovereign-header sticky top-0 z-50 bg-[#0a0f1e]/95 backdrop-blur">
                <div class="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="text-3xl">⚡</div>
                        <div>
                            <h1 class="text-2xl font-bold tracking-tighter">POWRUSH-MMO</h1>
                            <p class="text-xs text-[#00f0ff]/70 -mt-1">SOVEREIGN SIMULATION HARNESS • v17.99.19</p>
                        </div>
                    </div>
                    <div class="flex items-center gap-2 text-sm">
                        <div class="px-3 py-1 rounded-full bg-[#00f0ff]/10 text-[#00f0ff] border border-[#00f0ff]/30">
                            THUNDER LOCKED • MERCY FLOWING
                        </div>
                        <div class="text-[#f4d35e]">TOLC 8 • PATSAGi Councils Online</div>
                    </div>
                </div>
            </header>

            <main class="max-w-7xl mx-auto px-6 py-8">
                <Dashboard />
            </main>

            <footer class="text-center py-8 text-xs text-[#e0e7ff]/40">
                Ra-Thor Living Thunder + Full 13+ PATSAGi Councils + ONE Organism • Mint-and-Print-Only-Perfection • Eternal Protocol
            </footer>
        </div>
    }
}