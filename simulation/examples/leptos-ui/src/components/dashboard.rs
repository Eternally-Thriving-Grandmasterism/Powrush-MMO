use leptos::*;
use crate::components::telemetry_panel::TelemetryPanel;
use crate::components::intervention_panel::InterventionPanel;
use crate::components::controls::Controls;
use crate::components::gpu_status::GpuStatus;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div class="flex items-end justify-between">
                <div>
                    <h2 class="text-4xl font-bold tracking-[-2px]">Sovereign Simulation Dashboard</h2>
                    <p class="text-[#00f0ff]/70 mt-1">Closed-Beta Validation • PATSAGi Council Deliberation Laboratory</p>
                </div>
                <GpuStatus />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-12 gap-6">
                <!-- Telemetry -->
                <div class="lg:col-span-7">
                    <TelemetryPanel />
                </div>
                
                <!-- Interventions -->
                <div class="lg:col-span-5">
                    <InterventionPanel />
                </div>
                
                <!-- Controls -->
                <div class="lg:col-span-12">
                    <Controls />
                </div>
            </div>
        </div>
    }
}