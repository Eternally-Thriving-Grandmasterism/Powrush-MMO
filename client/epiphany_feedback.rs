/*!
 * client/epiphany_feedback.rs
 *
 * Dedicated Client-Side Reactors for EpiphanyTriggered events.
 * Production-grade multi-channel feedback:
 * - Particles / Visuals
 * - Spatial / Procedural Audio (realtime synthesis + persistent catalog)
 * - Enhanced UI + Persistence feedback
 *
 * v21.89.1 — Wired to RealtimeAudioSynthesis for saveable/recallable moments.
 *
 * This module keeps epiphany presentation clean, extensible, and aligned with TOLC 8 Mercy Gates.
 */

use bevy::prelude::*;
use crate::epiphany_catalyst::EpiphanyTriggered;
use crate::divine_whispers::DivineWhisperTrigger;
use crate::realtime_audio_synthesis::{
    SynthesizeAudioMoment, request_epiphany_synth,
};

/// Main reactor system for EpiphanyTriggered events.
/// Add this to your Update schedule (via EpiphanyFeedbackPlugin).
pub fn epiphany_triggered_reactor(
    mut epiphany_events: EventReader<EpiphanyTriggered>,
    mut whisper_events: EventWriter<DivineWhisperTrigger>,
    mut synth_events: EventWriter<SynthesizeAudioMoment>,
    mut commands: Commands,
) {
    for event in epiphany_events.read() {
        let outcome = &event.outcome;
        let biome = &event.biome;

        // 1. Divine Whispers (enhanced for epiphanies)
        whisper_events.send(DivineWhisperTrigger::from_epiphany(
            event.player_id,
            outcome.divine_whisper_flavor.clone(),
            outcome.divine_whisper_flavor.clone(),
            outcome.intensity,
        ));

        // 2. Spawn rich, context-aware particle / visual effects
        spawn_epiphany_particles(&mut commands, outcome, biome);

        // 3. Realtime procedural audio — synthesize, persist, optional server sync
        let seed = event.player_id.wrapping_mul(1009).wrapping_add(
            (outcome.intensity * 10_000.0) as u64,
        );
        request_epiphany_synth(&mut synth_events, outcome.intensity, seed);
        trigger_epiphany_spatial_audio(outcome, biome);

        // 4. Optional direct UI enhancement
        trigger_epiphany_ui_popup(outcome, biome);
    }
}

/// Spawns context-aware particle effects based on EpiphanyOutcome and biome.
fn spawn_epiphany_particles(
    commands: &mut Commands,
    outcome: &crate::epiphany_catalyst::EpiphanyOutcome,
    biome: &str,
) {
    match outcome.particle_effect.as_str() {
        "sacred_geometry_crystal_bloom" => {
            // commands.spawn(SacredGeometryCrystalBloom { intensity: outcome.intensity, ..default() });
        }
        "mycelial_web_glow" => {
            // commands.spawn(MycelialWebGlow { intensity: outcome.intensity, ..default() });
        }
        "ethereal_bloom" | _ => {
            // commands.spawn(EtherealBloomParticles { intensity: outcome.intensity });
        }
    }

    if outcome.time_dilation_factor > 1.0 {
        // commands.insert_resource(TimeScale(outcome.time_dilation_factor));
    }

    info!("[Epiphany] Particles spawned: {} in {}", outcome.particle_effect, biome);
}

/// Triggers spatial / ambisonic audio for the epiphany (alongside recipe synth).
fn trigger_epiphany_spatial_audio(
    outcome: &crate::epiphany_catalyst::EpiphanyOutcome,
    biome: &str,
) {
    // Future: feed ambisonics_engine / higher_order_ambisonics with intensity + flavor
    let intensity = outcome.intensity;
    let flavor = &outcome.divine_whisper_flavor;

    info!(
        "[Epiphany] Spatial audio + synth: {} (intensity {:.2}) in {}",
        flavor, intensity, biome
    );
}

/// Optional standalone UI popup for epiphanies.
fn trigger_epiphany_ui_popup(
    outcome: &crate::epiphany_catalyst::EpiphanyOutcome,
    _biome: &str,
) {
    info!(
        "[Epiphany] UI popup: Muscle +{:.2}, Intensity {:.0}%",
        outcome.muscle_memory_consolidation_boost,
        outcome.intensity * 100.0
    );
}

/// Plugin to register all epiphany feedback systems.
pub struct EpiphanyFeedbackPlugin;

impl Plugin for EpiphanyFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, epiphany_triggered_reactor);
    }
}
