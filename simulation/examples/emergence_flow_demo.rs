/*!
 * Emergence + Ra-Thor Council Flow Demo (End-to-End)
 *
 * This example demonstrates the complete Phase 1 + Phase 2 flow:
 * - PlayerSaveData (persistence)
 * - EmergenceSeed generation from epiphany/harvest activity
 * - RaThorBridge council query
 * - DynamicEmergenceEvent creation and effect application
 *
 * Run with:
 *   cargo run --example emergence_flow_demo -p simulation
 *
 * This is plain Rust (no Bevy App required) so it works in the current
 * simulation crate. The Bevy Plugin version can be wired later in client/server.
 */

use simulation::player_persistence::data::PlayerSaveData;
use simulation::emergence::{
    EmergenceSeed, EmergenceSource, DynamicEmergenceEvent, DynamicEmergenceEventPhase,
    EmergenceEffect, CouncilGuidance,
};
use simulation::ra_thor_bridge::RaThorBridge;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("\n=== Powrush-MMO Emergence + Ra-Thor Council Flow Demo ===\n");

    // === 1. Setup Player Persistence (existing system) ===
    let mut player = PlayerSaveData::new(42);
    player.record_epiphany("first_contact", 0.85, "Crystal Spires");
    player.record_harvest(true);
    player.record_harvest(true);
    println!("Player created with {} epiphanies and muscle memory {:.2}",
             player.total_epiphanies, player.muscle_memory_level);

    // === 2. Create EmergenceOrchestrator + RaThorBridge ===
    let bridge = RaThorBridge::new(true);
    println!("RaThorBridge initialized (simulation mode: {})\n", bridge.simulation_mode);

    // === 3. Simulate an Epiphany-triggered EmergenceSeed ===
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let seed = EmergenceSeed {
        source: EmergenceSource::Epiphany,
        location: None,
        valence_delta: 0.35,
        intensity: 0.82,
        biome: "Crystal Spires".to_string(),
        group_size: 1,
        timestamp: now,
    };

    println!("EmergenceSeed created from Epiphany (intensity: {:.2})", seed.intensity);

    // === 4. Query Ra-Thor / PATSAGi Council via Bridge ===
    let council_guidance = bridge.query_council_guidance(&seed, 0.78, 0.91);

    match &council_guidance {
        Some(guidance) => {
            println!("Council responded!");
            println!("  Flavor: {}", guidance.flavor);
            println!("  Suggested intensity: {:.2}", guidance.suggested_intensity);
            println!("  Mercy note: {}\n", guidance.mercy_note);
        }
        None => println!("No council guidance this time (mercy or conditions not met)\n"),
    }

    // === 5. Build DynamicEmergenceEvent ===
    let mut event = DynamicEmergenceEvent {
        id: 1,
        phase: if council_guidance.is_some() {
            DynamicEmergenceEventPhase::CouncilReview {
                guidance: Some("ra_thor_abundance_guidance".to_string()),
            }
        } else {
            DynamicEmergenceEventPhase::Proposal
        },
        seed: seed.clone(),
        proposed_effects: vec![],
        mercy_score: 0.89,
        created_at: now,
    };

    // Add effects based on source + council input
    if matches!(seed.source, EmergenceSource::Epiphany) {
        event.proposed_effects.push(EmergenceEffect::EpiphanyTrigger {
            scenario_id: "emergent_reflection".to_string(),
            intensity_boost: 0.25,
        });
        event.proposed_effects.push(EmergenceEffect::MuscleMemoryConsolidation { boost: 0.12 });
    }

    println!("DynamicEmergenceEvent created with {} proposed effects", event.proposed_effects.len());
    println!("  Phase: {:?}\n", event.phase);

    // === 6. Apply Effects to Player Persistence (end-to-end) ===
    for effect in &event.proposed_effects {
        match effect {
            EmergenceEffect::MuscleMemoryConsolidation { boost } => {
                player.muscle_memory_level = (player.muscle_memory_level + boost).min(5.0);
                println!("Applied MuscleMemoryConsolidation +{:.2} -> {:.2}", boost, player.muscle_memory_level);
            }
            EmergenceEffect::EpiphanyTrigger { intensity_boost, .. } => {
                println!("Epiphany amplification triggered (+{:.2} intensity)", intensity_boost);
            }
            _ => {}
        }
    }

    player.dirty = true;

    println!("\n=== Final Player State ===");
    println!("Muscle Memory Level: {:.2}", player.muscle_memory_level);
    println!("Total Epiphanies: {}", player.total_epiphanies);
    println!("Resonance Score: {:.2}", player.resonance_score);

    println!("\n=== Demo Complete - Full Emergence + Council Flow Successful ===\n");
}
