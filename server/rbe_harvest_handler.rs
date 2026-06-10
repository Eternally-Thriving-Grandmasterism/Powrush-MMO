/*!
 * RBE Harvest Handler v18.10
 */

use bevy::prelude::*;
use simulation::epiphany_catalyst::check_epiphany_after_harvest;
use simulation::bot_detection::BotDetectionConfig;
use simulation::divine_whispers::DivineWhisperTrigger;
use simulation::player_persistence::PlayerSaveData;

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
    mut whisper_events: EventWriter<DivineWhisperTrigger>,
    mut player_save: ResMut<PlayerSaveData>,
) -> HarvestResult {
    let multiplier = player_save.get_current_harvest_multiplier();
    let base_resources = 12.5;
    let final_resources = base_resources * multiplier;

    let mut result = HarvestResult {
        success: true,
        resources_gained: final_resources,
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
        apply_epiphany_effects(player_id, &epiphany, &mut whisper_events, &mut player_save);
    }

    // Clear expired multiplier
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if player_save.temporary_multiplier_expires_at < now {
        player_save.temporary_harvest_multiplier = 1.0;
    }

    result
}

fn apply_epiphany_effects(
    player_id: u64,
    epiphany: &simulation::epiphany_catalyst::EpiphanyOutcome,
    whisper_events: &mut EventWriter<DivineWhisperTrigger>,
    player_save: &mut PlayerSaveData,
) {
    // Apply real temporary gameplay reward
    if epiphany.epiphany_multiplier > 1.0 {
        let duration_seconds: u64 = 600;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        player_save.temporary_harvest_multiplier = epiphany.epiphany_multiplier;
        player_save.temporary_multiplier_expires_at = now + duration_seconds;
    }

    // Send enhanced Divine Whisper for epiphanies
    let whisper_text = match epiphany.divine_whisper_flavor.as_str() {
        "sustainable_harmony_revelation" => "A deep sense of harmony flows through you. Your sustainable choices are writing a better future.",
        "sustainable_abundance_revelation" => "You have touched the rhythm of true abundance. The land remembers your care.",
        "council_harmony_revelation" => "When hearts align in mercy, the whole becomes greater than the sum.",
        _ => "A quiet revelation settles within you.",
    };

    whisper_events.send(DivineWhisperTrigger::from_epiphany(
        player_id,
        whisper_text,
        &epiphany.divine_whisper_flavor,
        epiphany.intensity,
    ));
}
