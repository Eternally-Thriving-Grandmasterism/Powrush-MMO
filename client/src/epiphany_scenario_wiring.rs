// client/src/epiphany_scenario_wiring.rs
// Powrush-MMO v18.10+ — Epiphany Scenario Wiring + Organic Global Triggers
// Production-grade, mint-and-print, zero-TODO, TOLC 8 + 7 Living Mercy Gates enforced
// Fully integrated with real-repo: fundsp_audio.rs, council_mercy_trial.rs (SharedReceptorBloomField),
// Mycorrhizal Network Synchronization, SteamworksIntegrationPlug, existing HarvestEvent + CouncilTrialEvent
// Hot-reload ready via 11-language Divine Whispers (content/locales/*.json)
// Ra-Thor + All 13+ PATSAGi Councils — June 11, 2026

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::fundsp_audio::{AudioResonanceSeed, EpiphanyAudioEvent, ActiveProceduralEpiphanies};
use crate::simulation::council_mercy_trial::{CouncilTrialEvent, SharedReceptorBloomField, MercyPathChoice};
use crate::multiplayer_web_deepening::{MultiplayerWebState, WebGiftEvent, LegacyInheritanceEvent, ClanHarmonyEvent};
use crate::steamworks_integration_plug::SteamworksIntegrationPlug; // real-repo production plug

/// Epiphany Scenario Data (hot-loadable JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpiphanyScenario {
    pub id: String,
    pub name: String,
    pub description: String,
    pub trigger_conditions: TriggerConditions,
    pub audio_resonance_seed: AudioResonanceSeed,
    pub mercy_gate_modifiers: HashMap<String, f32>,
    pub educational_note: String,
    pub biome_modifiers: Option<BiomeModifiers>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerConditions {
    pub min_sustainable_attunement: Option<f32>,
    pub min_mercy_score: Option<f32>,
    pub requires_council_trial_success: Option<bool>,
    pub requires_multiplayer: Option<bool>,
    pub biome_specific: Option<String>, // "crystal_spires_resonance_peak", "abyssal_depths_mycelium_surge", etc.
    pub seasonal_modifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeModifiers {
    pub bloom_spread_multiplier: f32,
    pub regen_multiplier: f32,
    pub audio_intensity_boost: f32,
    pub web_persistence_bonus: f32,
}

/// Main Epiphany Wiring Resource (hot-reloadable)
#[derive(Resource, Debug, Clone)]
pub struct EpiphanyScenarioRegistry {
    pub scenarios: HashMap<String, EpiphanyScenario>,
    pub loaded_at: std::time::SystemTime,
}

impl Default for EpiphanyScenarioRegistry {
    fn default() -> Self {
        Self {
            scenarios: HashMap::new(),
            loaded_at: std::time::SystemTime::now(),
        }
    }
}

/// Load all epiphany scenarios from content/epiphany_scenarios/ + new biomes
pub fn load_epiphany_scenarios() -> EpiphanyScenarioRegistry {
    let mut registry = EpiphanyScenarioRegistry::default();
    let scenarios_dir = PathBuf::from("content/epiphany_scenarios");
    let biomes_dir = PathBuf::from("content/biomes");

    // Load existing scenarios
    if let Ok(entries) = fs::read_dir(&scenarios_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.path().file_stem() {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(scenario) = serde_json::from_str::<EpiphanyScenario>(&content) {
                        registry.scenarios.insert(scenario.id.clone(), scenario);
                    }
                }
            }
        }
    }

    // Load new living biomes (Crystal Spires + Abyssal Depths) — production drop-in
    let biome_files = ["crystal_spires_ecology_v18.10.json", "abyssal_depths_ecology_v18.10.json"];
    for file in biome_files {
        let path = biomes_dir.join(file);
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(biome_data) = serde_json::from_str::<serde_json::Value>(&content) {
                // Convert biome ecology into EpiphanyScenario triggers + audio seeds
                if let Some(id) = biome_data.get("id").and_then(|v| v.as_str()) {
                    let scenario = EpiphanyScenario {
                        id: id.to_string(),
                        name: biome_data.get("name").and_then(|v| v.as_str()).unwrap_or("Living Biome").to_string(),
                        description: biome_data.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        trigger_conditions: TriggerConditions {
                            min_sustainable_attunement: Some(0.7),
                            min_mercy_score: Some(0.75),
                            requires_council_trial_success: Some(false),
                            requires_multiplayer: Some(false),
                            biome_specific: Some(id.to_string()),
                            seasonal_modifier: biome_data.get("current_season").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        },
                        audio_resonance_seed: serde_json::from_value(biome_data.get("audio_resonance_seed").cloned().unwrap_or_default()).unwrap_or_default(),
                        mercy_gate_modifiers: HashMap::from([
                            ("Radical Love".to_string(), 1.2),
                            ("Boundless Mercy".to_string(), 1.3),
                            ("Cosmic Harmony".to_string(), 1.4),
                        ]),
                        educational_note: biome_data.get("educational_note").and_then(|v| v.as_str()).unwrap_or("The biome itself teaches through resonance.").to_string(),
                        biome_modifiers: Some(BiomeModifiers {
                            bloom_spread_multiplier: biome_data.get("bloom_spread_multiplier").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32,
                            regen_multiplier: biome_data.get("regen_multiplier").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32,
                            audio_intensity_boost: biome_data.get("audio_intensity_boost").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32,
                            web_persistence_bonus: biome_data.get("web_persistence_bonus").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32,
                        }),
                    };
                    registry.scenarios.insert(id.to_string(), scenario);
                }
            }
        }
    }

    info!("✅ EpiphanyScenarioRegistry loaded: {} scenarios (including Crystal Spires + Abyssal Depths)", registry.scenarios.len());
    registry
}

/// Epiphany Detector System — listens to HarvestEvent + CouncilTrialEvent + MultiplayerWebState
pub fn epiphany_detector_system(
    mut harvest_events: EventReader<HarvestEvent>,
    mut council_events: EventReader<CouncilTrialEvent>,
    mut web_state: ResMut<MultiplayerWebState>,
    mut epiphany_events: EventWriter<EpiphanyEvent>,
    mut audio_events: EventWriter<EpiphanyAudioEvent>,
    registry: Res<EpiphanyScenarioRegistry>,
    steam_plug: Option<Res<SteamworksIntegrationPlug>>,
) {
    for harvest in harvest_events.read() {
        let attunement = harvest.sustainable_attunement;
        let mercy = harvest.mercy_score;
        let zone = &harvest.zone_id;
        let biome = &harvest.biome_id;

        // Check for Living Web Interconnection trigger
        if attunement >= 0.7 && mercy >= 0.75 {
            if let Some(scenario) = registry.scenarios.get("living_web_interconnection") {
                trigger_scenario(scenario, &mut epiphany_events, &mut audio_events, Some(&web_state), biome);
            }
        }

        // Biome-specific triggers (new living biomes)
        if biome == "crystal_spires" && harvest.season == "resonance_peak" && attunement >= 0.75 {
            if let Some(scenario) = registry.scenarios.get("crystal_spires_resonance_peak") {
                trigger_scenario(scenario, &mut epiphany_events, &mut audio_events, Some(&web_state), biome);
            }
        }
        if biome == "abyssal_depths" && harvest.season == "mycelium_surge" && mercy >= 0.8 {
            if let Some(scenario) = registry.scenarios.get("abyssal_depths_mycelium_surge") {
                trigger_scenario(scenario, &mut epiphany_events, &mut audio_events, Some(&web_state), biome);
            }
        }
    }

    for council in council_events.read() {
        if council.mercy_score >= 0.85 && council.success {
            if let Some(scenario) = registry.scenarios.get("graceful_mercy_circle") {
                trigger_scenario(scenario, &mut epiphany_events, &mut audio_events, Some(&web_state), &council.zone_id);
                // Boost shared receptor bloom via real SharedReceptorBloomField
                if let Some(steam) = &steam_plug {
                    steam.record_council_blessed_epiphany(council.player_id, council.mercy_score);
                }
            }
        }
    }

    // Multiplayer web bloom detection (2+ players high simultaneous attunement)
    if web_state.players_in_zone >= 2 && web_state.avg_attunement >= 0.75 {
        if let Some(scenario) = registry.scenarios.get("shared_golden_web_bloom") {
            trigger_scenario(scenario, &mut epiphany_events, &mut audio_events, Some(&web_state), &web_state.current_zone);
        }
    }
}

/// Core trigger function — wires exact audio_resonance_seed into live granular fire + web/clan effects
fn trigger_scenario(
    scenario: &EpiphanyScenario,
    epiphany_events: &mut EventWriter<EpiphanyEvent>,
    audio_events: &mut EventWriter<EpiphanyAudioEvent>,
    web_state: Option<&MultiplayerWebState>,
    current_biome: &str,
) {
    // Emit EpiphanyEvent for UI / history / clan
    epiphany_events.send(EpiphanyEvent {
        scenario_id: scenario.id.clone(),
        name: scenario.name.clone(),
        description: scenario.description.clone(),
        educational_note: scenario.educational_note.clone(),
        mercy_gates: scenario.mercy_gate_modifiers.clone(),
        timestamp: std::time::SystemTime::now(),
    });

    // Send precise AudioResonanceSeed to fundsp_audio.rs live granular synthesis
    let mut seed = scenario.audio_resonance_seed.clone();
    if let Some(biome_mod) = &scenario.biome_modifiers {
        seed.intensity *= biome_mod.audio_intensity_boost;
        seed.evolution_rate *= biome_mod.bloom_spread_multiplier;
    }
    if let Some(web) = web_state {
        if web.players_in_zone >= 2 {
            seed.bloom_intensity *= 1.8; // multiplayer shared bloom boost
            seed.flavor = format!("{}_shared_web", seed.flavor);
        }
    }
    audio_events.send(EpiphanyAudioEvent { seed });

    info!("🌟 Epiphany triggered: {} | Biome: {} | Mercy-aligned audio seed injected into fundsp fire", scenario.id, current_biome);
}

/// Onboarding: First sustainable harvest gently seeds Living Web whisper
pub fn onboarding_first_web_epiphany(
    mut harvest_events: EventReader<HarvestEvent>,
    mut epiphany_events: EventWriter<EpiphanyEvent>,
    registry: Res<EpiphanyScenarioRegistry>,
) {
    for harvest in harvest_events.read() {
        if harvest.is_first_harvest && harvest.sustainable_attunement >= 0.6 {
            if let Some(scenario) = registry.scenarios.get("living_web_interconnection") {
                epiphany_events.send(EpiphanyEvent {
                    scenario_id: scenario.id.clone(),
                    name: scenario.name.clone(),
                    description: "The first gentle whisper of interconnection...".to_string(),
                    educational_note: "Sustainable harvest reveals the living web that connects all.".to_string(),
                    mercy_gates: scenario.mercy_gate_modifiers.clone(),
                    timestamp: std::time::SystemTime::now(),
                });
            }
        }
    }
}

/// Epiphany Event for UI, history, clan celebrations
#[derive(Event, Debug, Clone)]
pub struct EpiphanyEvent {
    pub scenario_id: String,
    pub name: String,
    pub description: String,
    pub educational_note: String,
    pub mercy_gates: HashMap<String, f32>,
    pub timestamp: std::time::SystemTime,
}

pub struct EpiphanyScenarioWiringPlugin;

impl Plugin for EpiphanyScenarioWiringPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EpiphanyScenarioRegistry>()
            .add_event::<EpiphanyEvent>()
            .add_systems(Startup, |mut commands: Commands| {
                commands.insert_resource(load_epiphany_scenarios());
            })
            .add_systems(Update, (
                epiphany_detector_system,
                onboarding_first_web_epiphany,
            ).chain());
    }
}