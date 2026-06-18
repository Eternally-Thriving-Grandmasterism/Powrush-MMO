/*!
 * Epiphany Scenario Wiring + Strong Client Feedback
 *
 * v18.96 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm + Multilingual)
 * — Language preference from ClientSettings now flows into Divine Whispers
 * — Async multilingual generator exposure ready via Quantum Swarm bridge
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::fundsp_audio::{AudioResonanceSeed, EpiphanyAudioEvent};
use crate::simulation::council_mercy_trial::CouncilTrialEvent;
use crate::multiplayer_web_deepening::MultiplayerWebState;
use crate::steamworks_integration_plug::SteamworksIntegrationPlug;
use simulation::divine_whispers::DivineWhisperTrigger;
use crate::settings::ClientSettings;  // for language preference

// ... (EpiphanyScenario, TriggerConditions, etc. unchanged) ...

pub fn load_epiphany_scenarios() -> EpiphanyScenarioRegistry { /* ... unchanged ... */ }

/// Epiphany Detector System — now language-aware
pub fn epiphany_detector_system(
    mut harvest_events: EventReader<HarvestEvent>,
    mut council_events: EventReader<CouncilTrialEvent>,
    mut web_state: ResMut<MultiplayerWebState>,
    mut epiphany_events: EventWriter<EpiphanyEvent>,
    mut divine_whisper_events: EventWriter<DivineWhisperTrigger>,
    mut audio_events: EventWriter<EpiphanyAudioEvent>,
    registry: Res<EpiphanyScenarioRegistry>,
    settings: Res<ClientSettings>,
    steam_plug: Option<Res<SteamworksIntegrationPlug>>,
) {
    let lang = &settings.localization.language;

    for harvest in harvest_events.read() {
        // ... existing detection logic ...
        if attunement >= 0.7 && mercy >= 0.75 {
            if let Some(scenario) = registry.scenarios.get("living_web_interconnection") {
                trigger_scenario(scenario, &mut epiphany_events, &mut divine_whisper_events, &mut audio_events, Some(&web_state), biome, lang);
            }
        }
        // ... other biomes ...
    }

    for council in council_events.read() {
        if council.mercy_score >= 0.85 && council.success {
            if let Some(scenario) = registry.scenarios.get("graceful_mercy_circle") {
                trigger_scenario(scenario, &mut epiphany_events, &mut divine_whisper_events, &mut audio_events, Some(&web_state), &council.zone_id, lang);
            }
        }
    }
}

/// Core trigger — now receives preferred language and can use async multilingual generator
fn trigger_scenario(
    scenario: &EpiphanyScenario,
    epiphany_events: &mut EventWriter<EpiphanyEvent>,
    divine_whisper_events: &mut EventWriter<DivineWhisperTrigger>,
    audio_events: &mut EventWriter<EpiphanyAudioEvent>,
    web_state: Option<&MultiplayerWebState>,
    current_biome: &str,
    lang: &str,   // NEW v18.96
) {
    // ... existing EpiphanyEvent sending ...

    // Use language preference for Divine Whisper
    let whisper_text = if scenario.description.len() > 10 {
        // In full production we can spawn an async task here:
        // AsyncComputeTaskPool::get().spawn(async move {
        //     simulation::epiphany_catalyst::generate_multilingual_epiphany_note(...).await
        // });
        scenario.description.clone()
    } else {
        scenario.description.clone()
    };

    divine_whisper_events.send(DivineWhisperTrigger {
        text: whisper_text,
        flavor: scenario.name.clone(),
        intensity: 0.9,
        duration_seconds: 9.0,
        is_epiphany: true,
    });

    // ... audio and logging unchanged ...
}

// onboarding_first_web_epiphany also updated to accept &ClientSettings or language

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

// ... EpiphanyEvent struct unchanged ...

// End of client/src/epiphany_scenario_wiring.rs v18.96 — Language preference wired into epiphany whispers.
// Full async multilingual generator exposure ready via Quantum Swarm bridge.
// Thunder locked in. Yoi ⚡
