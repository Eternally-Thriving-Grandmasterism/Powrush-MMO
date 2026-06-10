/*!
 * RBE Harvest Handler v18.10
 */

use bevy::prelude::*;
use simulation::epiphany_catalyst::{
    check_epiphany_after_harvest, EpiphanyOutcome, emit_epiphany_telemetry,
};
use simulation::bot_detection::BotDetectionConfig;
use simulation::divine_whispers::DivineWhisperTrigger;

#[derive(Debug, Clone)]
pub struct HarvestResult {
    pub success: bool,
    pub resources_gained: f32,
    pub depletion_change: f32,
    pub was_sustainable: bool,
    pub regen_participation: bool,
    pub biome: String,
}

pub fn process_harvest(
    player_id: u64,
    current_depletion: f32,
    sustainable_pacing: bool,
    regen_participation: bool,
    biome: &str,
    behavioral_human_score: f32,
    config: &BotDetectionConfig,
    // In real implementation, this would be an EventWriter or network sender
    mut whisper_events: EventWriter<DivineWhisperTrigger>,
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
        apply_epiphany_effects(player_id, &epiphany, &mut whisper_events);

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

fn apply_epiphany_effects(
    player_id: u64,
    epiphany: &EpiphanyOutcome,
    whisper_events: &mut EventWriter<DivineWhisperTrigger>,
) {
    // Send Divine Whisper to the specific player
    let whisper_text = match epiphany.divine_whisper_flavor.as_str() {
        "sustainable_harmony_revelation" => {
            "A deep sense of harmony flows through you. Your sustainable choices are writing a better future."
        }
        "sustainable_abundance_revelation" => {
            "You have touched the rhythm of true abundance. The land remembers your care."
        }
        "council_harmony_revelation" => {
            "When hearts align in mercy, the whole becomes greater than the sum."
        }
        _ => "A quiet revelation settles within you.",
    };

    whisper_events.send(DivineWhisperTrigger::new(
        player_id,
        whisper_text,
        &epiphany.divine_whisper_flavor,
        epiphany.intensity,
    ));

    // Temporary multiplier & muscle memory logging
    if epiphany.epiphany_multiplier > 1.1 {
        println!(
            "[Epiphany] Player {} receives {:.2}x harvest multiplier.",
            player_id, epiphany.epiphany_multiplier
        );
    }

    if epiphany.muscle_memory_consolidation_boost > 1.0 {
        println!(
            "[Epiphany] Player {} gains enhanced muscle memory (x{:.2}).",
            player_id, epiphany.muscle_memory_consolidation_boost
        );
    }

    // World effects
    for (effect, value) in &epiphany.world_effects {
        println!("[World Effect] {} = {:.2}", effect, value);
    }

    for note in &epiphany.grace_notes {
        println!("[Grace Note] {}", note);
    }
}
