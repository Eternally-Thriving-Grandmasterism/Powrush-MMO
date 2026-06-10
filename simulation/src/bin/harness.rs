#![allow(clippy::too_many_arguments)]

//! Sovereign Simulation Harness Binary
//! End-to-end runnable example for closed-beta validation and RBE policy testing.
//!
//! Usage:
//!   cargo run --features gpu -p powrush-simulation --bin harness -- --preset LongTermRbeStability --gpu --ticks 500
//!   cargo run -p powrush-simulation --bin harness -- --preset AbundanceSurgeWithEvolution --ticks 200

use powrush_simulation::{
    archetype::SovereignArchetypeSystem,
    economy::EconomicLayer,
    harvest::HarvestingSystem,
    mercy::MercyGate,
    orchestrator::SovereignOrchestrator,
    scenario::{ScenarioConfig, ScenarioPreset},
    telemetry::TelemetryCollector,
    world::SovereignWorldState,
};
use std::env;
use std::time::Instant;

fn main() {
    println!("\n⚡ SOVEREIGN SIMULATION HARNESS — END-TO-END CLOSED-BETA VALIDATION RUN ⚡");
    println!("Ra-Thor Living Thunder + Full PATSAGi Councils — Thunder locked. Mercy flowing.\n");

    // === Parse simple CLI args ===
    let args: Vec<String> = env::args().collect();
    let preset_name = args.iter().position(|a| a == "--preset").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "LongTermRbeStability".to_string());
    let use_gpu = args.iter().any(|a| a == "--gpu");
    let tick_count: u64 = args.iter().position(|a| a == "--ticks").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(300);

    println!("Preset: {}", preset_name);
    println!("GPU acceleration: {}", if use_gpu { "ENABLED (wgpu + WGSL patsagi_economic)" } else { "DISABLED (CPU golden master)" });
    println!("Target ticks: {}\n", tick_count);

    // === Initialize deterministic scenario ===
    let preset = match preset_name.as_str() {
        "LongTermRbeStability" => ScenarioPreset::LongTermRbeStability,
        "HighGriefStressTest" => ScenarioPreset::HighGriefStressTest,
        "ArchetypeEvolutionUnderAbundance" => ScenarioPreset::ArchetypeEvolutionUnderAbundance,
        "ServerWarSimulation" => ScenarioPreset::ServerWarSimulation,
        "MinimalViableRbeTest" => ScenarioPreset::MinimalViableRbeTest,
        "AbundanceSurgeWithEvolution" => ScenarioPreset::AbundanceSurgeWithEvolution,
        _ => {
            eprintln!("Unknown preset '{}'. Falling back to LongTermRbeStability.", preset_name);
            ScenarioPreset::LongTermRbeStability
        }
    };

    let config: ScenarioConfig = preset.to_config();
    println!("Scenario initialized with {} resource nodes, {} factions, {} archetype templates.\n", 
        config.resource_templates.len(), config.faction_templates.len(), config.archetype_templates.len());

    // === Build Sovereign World ===
    let mut world = SovereignWorldState::from_scenario_config(&config);
    println!("World state created deterministically (seed: {}).\n", world.global_seed);

    // === Initialize Systems ===
    let mercy_gate = MercyGate::new_tolc8();
    let mut archetype_system = SovereignArchetypeSystem::from_templates(&config.archetype_templates);
    let mut economic_layer = EconomicLayer::new();
    economic_layer.cpu_precision_mode = !use_gpu; // false = try GPU when feature enabled
    let harvesting_system = HarvestingSystem::new();
    let mut telemetry = TelemetryCollector::new();

    // === Create Orchestrator ===
    let mut orchestrator = SovereignOrchestrator::new(
        world,
        archetype_system,
        economic_layer,
        harvesting_system,
        mercy_gate,
        telemetry,
        config.time_acceleration,
    );

    println!("Orchestrator ready. Starting deterministic simulation...\n");

    let start = Instant::now();

    // === Run the simulation ===
    match orchestrator.run_for_duration(tick_count) {
        Ok(_) => {
            let elapsed = start.elapsed();
            println!("\nSimulation completed successfully in {:.2?}.", elapsed);
            println!("Final sim time: {:.1} days (accelerated).\n", orchestrator.world().sim_time.days());

            // === Generate rich council-ready report ===
            let report = orchestrator.telemetry().generate_final_report();
            println!("{}", report);

            println!("\n⚡ THUNDER LOCKED. MERCY FLOWING. ALL VERSIONS PRESERVED AND ELEVATED. ⚡");
            println!("This run is sovereign, deterministic, and ready for closed-beta validation + PATSAGi policy testing.\n");
        }
        Err(e) => {
            eprintln!("Simulation failed with mercy violation or error: {:?}", e);
            std::process::exit(1);
        }
    }
}
