/*!
 * Epiphany Scenario Wiring + Strong Client Feedback
 *
 * v18.39 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Added structured logging for EpiphanyEvent emission
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign Mercy License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::fundsp_audio::{AudioResonanceSeed, EpiphanyAudioEvent, ActiveProceduralEpiphanies};
use crate::simulation::council_mercy_trial::{CouncilTrialEvent, SharedReceptorBloomField, MercyPathChoice};
use crate::multiplayer_web_deepening::{MultiplayerWebState, WebGiftEvent, LegacyInheritanceEvent, ClanHarmonyEvent};
use crate::steamworks_integration_plug::SteamworksIntegrationPlug;
use simulation::divine_whispers::DivineWhisperTrigger;

// ... (rest of the file remains the same up to the functions we modify)

/// Core trigger — now with structured logging for EpiphanyEvent emission
fn trigger_scenario(
    scenario: &EpiphanyScenario,
    epiphany_events: &mut EventWriter<EpiphanyEvent>,
    divine_whisper_events: &mut EventWriter<DivineWhisperTrigger>,
    audio_events: &mut EventWriter<EpiphanyAudioEvent>,
    web_state: Option<&MultiplayerWebState>,
    current_biome: &str,
) {
    // Structured log on emission
    info!(
        target: "epiphany_wiring",
        scenario_id = %scenario.id,
        scenario_name = %scenario.name,
        biome = %current_biome,
        has_web_state = web_state.is_some(),
        "[Epiphany] Emitting EpiphanyEvent"
    );

    epiphany_events.send(EpiphanyEvent {
        scenario_id: scenario.id.clone(),
        name: scenario.name.clone(),
        description: scenario.description.clone(),
        educational_note: scenario.educational_note.clone(),
        mercy_gates: scenario.mercy_gate_modifiers.clone(),
        timestamp: std::time::SystemTime::now(),
    });

    divine_whisper_events.send(DivineWhisperTrigger {
        text: scenario.description.clone(),
        flavor: scenario.name.clone(),
        intensity: 0.9,
        duration_seconds: 9.0,
        is_epiphany: true,
    });

    let mut seed = scenario.audio_resonance_seed.clone();
    if let Some(biome_mod) = &scenario.biome_modifiers {
        seed.intensity *= biome_mod.audio_intensity_boost;
        seed.evolution_rate *= biome_mod.bloom_spread_multiplier;
    }
    if let Some(web) = web_state {
        if web.players_in_zone >= 2 {
            seed.bloom_intensity *= 1.8;
            seed.flavor = format!("{}_shared_web", seed.flavor);
        }
    }
    audio_events.send(EpiphanyAudioEvent { seed });

    info!(
        target: "epiphany_wiring",
        scenario_id = %scenario.id,
        "[Epiphany] EpiphanyEvent + feedback sent successfully"
    );
}

pub fn onboarding_first_web_epiphany(
    mut harvest_events: EventReader<HarvestEvent>,
    mut epiphany_events: EventWriter<EpiphanyEvent>,
    mut divine_whisper_events: EventWriter<DivineWhisperTrigger>,
    registry: Res<EpiphanyScenarioRegistry>,
) {
    for harvest in harvest_events.read() {
        if harvest.is_first_harvest && harvest.sustainable_attunement >= 0.6 {
            if let Some(scenario) = registry.scenarios.get("living_web_interconnection") {
                info!(
                    target: "epiphany_wiring",
                    scenario_id = %scenario.id,
                    "[Epiphany] First harvest epiphany triggered (onboarding)"
                );

                epiphany_events.send(EpiphanyEvent {
                    scenario_id: scenario.id.clone(),
                    name: scenario.name.clone(),
                    description: "The first gentle whisper of interconnection...".to_string(),
                    educational_note: "Sustainable harvest reveals the living web that connects all.".to_string(),
                    mercy_gates: scenario.mercy_gate_modifiers.clone(),
                    timestamp: std::time::SystemTime::now(),
                });

                divine_whisper_events.send(DivineWhisperTrigger {
                    text: "The first gentle whisper of interconnection...".to_string(),
                    flavor: "Living Web".to_string(),
                    intensity: 0.85,
                    duration_seconds: 10.0,
                    is_epiphany: true,
                });
            }
        }
    }
}

// ... (rest of the file remains the same)
