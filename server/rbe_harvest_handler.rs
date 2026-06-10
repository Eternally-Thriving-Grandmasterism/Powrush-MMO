/*!
 * RBE Harvest Handler v18.10
 *
 * Authoritative harvest logic with full Epiphany system integration.
 * After every successful harvest, we check for epiphanies using the centralized catalyst.
 */

use bevy::prelude::*;
use simulation::epiphany_catalyst::{
    check_epiphany_after_harvest, EpiphanyOutcome, emit_epiphany_telemetry,
};
use simulation::bot_detection::BotDetectionConfig;

/// Main harvest result after processing
#[derive(Debug, Clone)]
pub struct HarvestResult {
    pub success: bool,
    pub resources_gained: f32,
    pub depletion_change: f32,
    pub was_sustainable: bool,
    pub regen_participation: bool,
    pub biome: String,
}

/// Called when a player completes a harvest action (authoritative server side)
pub fn process_harvest(
    player_id: u64,
    current_depletion: f32,
    sustainable_pacing: bool,
    regen_participation: bool,
    biome: &str,
    behavioral_human_score: f32,
    config: &BotDetectionConfig,
) -> HarvestResult {
    // === Core harvest logic (simplified for integration clarity) ===
    let mut result = HarvestResult {
        success: true,
        resources_gained: 12.5, // example
        depletion_change: if sustainable_pacing { -0.08 } else { 0.12 },
        was_sustainable: sustainable_pacing,
        regen_participation,
        biome: biome.to_string(),
    };

    // === Epiphany Check (Highest Impact Integration) ===
    if let Some(epiphany) = check_epiphany_after_harvest(
        current_depletion,
        sustainable_pacing,
        regen_participation,
        biome,
        behavioral_human_score,
    ) {
        apply_epiphany_effects(player_id, &epiphany);

        // Emit rich telemetry including behavioral context
        let _telemetry = emit_epiphany_telemetry(
            &epiphany,
            // We reconstruct a minimal context here for telemetry
            &simulation::epiphany_catalyst::EpiphanyContext {
                depletion: current_depletion,
                sustainable_pacing,
                regen_participation,
                biome: biome.to_string(),
                participant_count: 1,
                collective_attunement: 0.0,
                duration_ticks: 0,
            },
            Some(player_id),
            // current timestamp in ms
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            behavioral_human_score,
        );

        // Optional: log for debugging
        println!(
            "[Epiphany] Player {} triggered {} (intensity {:.2})",
            player_id, epiphany.scenario_id, epiphany.intensity
        );
    }

    result
}

/// Apply the effects of a triggered epiphany
fn apply_epiphany_effects(player_id: u64, epiphany: &EpiphanyOutcome) {
    // TODO: Integrate with actual player state, inventory, particles, Divine Whispers, etc.
    // Example effects:
    // - Apply epiphany.epiphany_multiplier to future harvests for a duration
    // - Trigger Divine Whispers with epiphany.divine_whisper_flavor
    // - Spawn particles based on epiphany.particle_effect
    // - Increase muscle memory via epiphany.muscle_memory_consolidation_boost
    // - Apply world_effects (e.g. collective_abundance_bloom)

    // For now we log the key effects
    println!(
        "[Epiphany Effects] Player {} | Multiplier: {:.2}x | Muscle Memory Boost: {:.2}x | Whisper: {}",
        player_id,
        epiphany.epiphany_multiplier,
        epiphany.muscle_memory_consolidation_boost,
        epiphany.divine_whisper_flavor
    );
}
