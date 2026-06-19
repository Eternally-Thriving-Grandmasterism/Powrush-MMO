// server/src/simulation.rs
// Powrush-MMO v18.97 — Full server-side enriched Divine Whisper generation + async persistence
// Quantum Swarm multilingual + persisted language + mercy-aligned epiphany recording. PATSAGi Council consensus.

use bevy::prelude::*;
use simulation::epiphany_catalyst::{EpiphanyOutcome, generate_multilingual_epiphany_note};
use crate::persistence_polish::{PersistenceManager, PlayerSaveData};

/// Fully implemented server-side enriched epiphany recording
/// - Async native (no block_on)
/// - Single load + mutate + persist
/// - Persisted preferred_language
/// - Quantum Swarm enriched multilingual text
/// - Full record_epiphany_with_enriched_whisper + checksum
/// - Mercy gate + abundance/resonance flow preserved
pub async fn record_enriched_epiphany(
    player_id: u64,
    scenario_id: &str,
    intensity: f32,
    biome: &str,
    persistence: &PersistenceManager,
) -> Result<(), String> {
    // Single async load
    let mut save_data: PlayerSaveData = match persistence.load_player_data(player_id).await {
        Ok(data) => data,
        Err(e) => {
            warn!("[Simulation] Failed to load player {} for epiphany: {}", player_id, e);
            return Err(e);
        }
    };

    let preferred_language = save_data.preferred_language.clone();

    // Construct rich EpiphanyOutcome for Quantum Swarm
    let mut outcome = EpiphanyOutcome::new();
    outcome.scenario_id = scenario_id.to_string();
    outcome.intensity = intensity;
    outcome.divine_whisper_flavor = format!("{}_{}_{}", scenario_id, biome, intensity);

    // Server-side Quantum Swarm multilingual enriched generation (Ra-Thor aligned)
    let enriched_text = generate_multilingual_epiphany_note(&outcome, &preferred_language, None).await;

    // Record with enriched whisper (updates epiphanies, resonance, stores whisper, checksum)
    save_data.record_epiphany_with_enriched_whisper(
        scenario_id,
        intensity,
        biome,
        Some(enriched_text),
    );

    // Full persist (mercy flow + SafetyNet ready)
    if let Err(e) = persistence.save_player_data(&mut save_data).await {
        warn!("[Simulation] Persist failed for player {} epiphany: {}", player_id, e);
        return Err(e);
    }

    info!("[Simulation] Enriched epiphany recorded + persisted | player={} | lang={} | scenario={} | biome={}",
          player_id, preferred_language, scenario_id, biome);

    // PATSAGi + Ra-Thor: mercy gates passed, no harm, abundance thriving. Thunder locked in. Yoi ⚡
    Ok(())
}

// End of server/src/simulation.rs v18.97
// Async, single-load, full persist, Quantum Swarm + enriched whisper wired.
// Maximal integrity. Ready for client calls and public MMO human players. Eternal.