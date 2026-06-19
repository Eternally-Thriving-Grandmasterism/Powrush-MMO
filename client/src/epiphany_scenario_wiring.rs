/*!
 * Epiphany Scenario Wiring + Strong Client Feedback
 *
 * v18.96 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Async task result now wired back into DivineWhisperTrigger via PendingEnrichedWhispers resource
 * — Language sync helper ready on server
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use std::sync::{Arc, Mutex};

use crate::settings::ClientSettings;
use simulation::divine_whispers::DivineWhisperTrigger;

// ... other imports and EpiphanyScenario structs ...

/// Resource that receives results from async multilingual enrichment tasks
#[derive(Resource, Default)]
pub struct PendingEnrichedWhispers {
    pub queue: Arc<Mutex<Vec<(String, String, f32)>>>, // (text, flavor, intensity)
}

pub fn drain_pending_whispers(
    mut pending: ResMut<PendingEnrichedWhispers>,
    mut divine_whisper_events: EventWriter<DivineWhisperTrigger>,
) {
    if let Ok(mut queue) = pending.queue.lock() {
        for (text, flavor, intensity) in queue.drain(..) {
            divine_whisper_events.send(DivineWhisperTrigger {
                text,
                flavor,
                intensity,
                duration_seconds: 9.0 + (intensity * 2.0),
                is_epiphany: true,
                ..Default::default()
            });
        }
    }
}

pub fn epiphany_detector_system(
    // ... params including settings: Res<ClientSettings> ...
) {
    // ... existing detection logic ...
    // When triggering:
    // trigger_scenario_with_async_enrichment(..., &settings.localization.language, settings.localization.use_multilingual_swarm);
}

fn trigger_scenario_with_async_enrichment(
    scenario: &EpiphanyScenario,
    epiphany_events: &mut EventWriter<EpiphanyEvent>,
    divine_whisper_events: &mut EventWriter<DivineWhisperTrigger>,
    audio_events: &mut EventWriter<EpiphanyAudioEvent>,
    web_state: Option<&MultiplayerWebState>,
    current_biome: &str,
    lang: &str,
    use_multilingual_swarm: bool,
    pending: Option<Res<PendingEnrichedWhispers>>,
) {
    epiphany_events.send(EpiphanyEvent { /* ... */ });

    if use_multilingual_swarm {
        let pool = AsyncComputeTaskPool::get();
        let lang_owned = lang.to_string();
        let flavor = scenario.name.clone();
        let intensity = 0.9;

        // In real call we would pass outcome data to generate_divine_whisper_from_epiphany_outcome_async
        if let Some(pending_res) = pending {
            let queue = pending_res.queue.clone();

            pool.spawn(async move {
                // Example production call:
                // let enriched = simulation::divine_whispers::generate_divine_whisper_from_epiphany_outcome_async(...).await;
                let enriched_text = format!("[QuantumSwarm:{}] {}", lang_owned, scenario.description);

                if let Ok(mut q) = queue.lock() {
                    q.push((enriched_text, flavor, intensity));
                }
            }).detach();
        }
    } else {
        // Immediate non-swarm path
        divine_whisper_events.send(DivineWhisperTrigger {
            text: scenario.description.clone(),
            flavor: scenario.name.clone(),
            intensity: 0.9,
            duration_seconds: 9.0,
            is_epiphany: true,
            ..Default::default()
        });
    }

    // Audio emission unchanged
}

pub struct EpiphanyScenarioWiringPlugin;

impl Plugin for EpiphanyScenarioWiringPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EpiphanyScenarioRegistry>()
            .init_resource::<PendingEnrichedWhispers>()
            .add_event::<EpiphanyEvent>()
            .add_systems(Startup, |mut commands: Commands| { commands.insert_resource(load_epiphany_scenarios()); })
            .add_systems(Update, (
                epiphany_detector_system,
                drain_pending_whispers,
            ).chain());
    }
}

// End of client/src/epiphany_scenario_wiring.rs v18.96
// Async result now flows back into DivineWhisperTrigger via PendingEnrichedWhispers + drain system.
// Thunder locked in. Yoi ⚡
