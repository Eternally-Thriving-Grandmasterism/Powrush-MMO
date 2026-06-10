/*!
 * RBE Harvest Handler v18.10
 *
 * Authoritative harvest logic with full Epiphany system integration.
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
    let mut result = HarvestResult {
        success: true,
        resources_gained: 12.5,
        depletion_change: if sustainable_pacing { -0.08 } else { 0.12 },
        was_sustainable: sustainable_pacing,
        regen_participation,
        biome: biome.to_string(),
    };

    if let Some(epiphany) = check_epiphany_after_harvest(
        current_depletion,
        sustainable_pacing,
        regen_participation,
        biome,
        behavioral_human_score,
    ) {
        apply_epiphany_effects(player_id, &epiphany);

        let _telemetry = emit_epiphany_telemetry(
            &epiphany,
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
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            behavioral_human_score,
        );

        println!(
            "[Epiphany] Player {} triggered {} (intensity {:.2})",
            player_id, epiphany.scenario_id, epiphany.intensity
        );
    }

    result
}

/// Expanded epiphany effects application (Divine Whispers, multipliers, world effects)
fn apply_epiphany_effects(player_id: u64, epiphany: &EpiphanyOutcome) {
    // === 1. Divine Whispers ===
    // Trigger contextually appropriate Divine Whisper based on the epiphany flavor
    trigger_divine_whisper(player_id, &epiphany.divine_whisper_flavor, epiphany.intensity);

    // === 2. Temporary Harvest Multiplier ===
    // In a full implementation, this would be stored in player state with a duration
    // For now we log it clearly
    if epiphany.epiphany_multiplier > 1.1 {
        println!(
            "[Epiphany] Player {} receives {:.2}x harvest multiplier for this epiphany.",
            player_id, epiphany.epiphany_multiplier
        );
        // TODO: Apply temporary modifier to player's harvest multiplier component
    }

    // === 3. Muscle Memory Consolidation ===
    if epiphany.muscle_memory_consolidation_boost > 1.0 {
        println!(
            "[Epiphany] Player {} gains enhanced muscle memory consolidation (x{:.2}).",
            player_id, epiphany.muscle_memory_consolidation_boost
        );
        // TODO: Update player's muscle memory / learning rate
    }

    // === 4. World Effects ===
    for (effect, value) in &epiphany.world_effects {
        match effect.as_str() {
            "collective_abundance_bloom" => {
                println!(
                    "[World Effect] Player {} triggered collective abundance bloom (x{:.2})",
                    player_id, value
                );
                // TODO: Broadcast to nearby players or Council session
            }
            _ => {
                println!(
                    "[World Effect] Player {} triggered unknown effect: {} = {:.2}",
                    player_id, effect, value
                );
            }
        }
    }

    // === 5. Grace Notes ===
    for note in &epiphany.grace_notes {
        println!("[Grace Note] {}", note);
    }

    // === 6. Particle / Visual Effects (client-side) ===
    // The particle_effect field should be sent to the client for visual feedback
    if !epiphany.particle_effect.is_empty() && epiphany.particle_effect != "default" {
        println!(
            "[Visual] Player {} should spawn particle effect: {}",
            player_id, epiphany.particle_effect
        );
        // TODO: Send network event to client to spawn particles
    }
}

/// Placeholder for triggering Divine Whispers from server
/// In full implementation this would send a network event to the client
fn trigger_divine_whisper(player_id: u64, flavor: &str, intensity: f32) {
    println!(
        "[Divine Whisper] Player {} receives whisper '{}' (intensity {:.2})",
        player_id, flavor, intensity
    );
    // TODO: Send DivineWhisperEvent or equivalent network message to client
}
