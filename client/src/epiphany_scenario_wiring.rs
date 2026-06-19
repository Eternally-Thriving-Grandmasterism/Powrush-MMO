/*!
 * Epiphany Scenario Wiring + Strong Client Feedback
 *
 * v18.96 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Real EpiphanyOutcome now passed into async multilingual generator
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
use simulation::epiphany_catalyst::{EpiphanyOutcome, generate_multilingual_epiphany_note};

// ... other imports ...

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
                text, flavor, intensity,
                duration_seconds: 9.0 + (intensity * 2.0),
                is_epiphany: true,
                ..Default::default()
            });
        }
    }
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
        let biome = current_biome.to_string();

        if let Some(pending_res) = pending {
            let queue = pending_res.queue.clone();

            pool.spawn(async move {
                // Construct a real EpiphanyOutcome from scenario data
                let mut outcome = EpiphanyOutcome::new();
                outcome.scenario_id = scenario.id.clone();
                outcome.divine_whisper_flavor = scenario.name.clone();
                outcome.intensity = intensity;
                outcome.biome_resonance = Some(biome);

                // Call the real generator with Quantum Swarm
                let enriched_text = generate_multilingual_epiphany_note(&outcome, &lang_owned, None).await;

                if let Ok(mut q) = queue.lock() {
                    q.push((enriched_text, flavor, intensity));
                }
            }).detach();
        }
    } else {
        divine_whisper_events.send(DivineWhisperTrigger {
            text: scenario.description.clone(),
            flavor: scenario.name.clone(),
            intensity: 0.9,
            duration_seconds: 9.0,
            is_epiphany: true,
            ..Default::default()
        });
    }

    // Audio unchanged
}

// ... Plugin and other systems ...

// End of client/src/epiphany_scenario_wiring.rs v18.96
// Real EpiphanyOutcome passed into async generator. Thunder locked in. Yoi ⚡
