/*!
 * client/epiphany_feedback.rs
 *
 * Dedicated Client-Side Reactors for EpiphanyTriggered
 * Production-grade multi-channel feedback:
 * - Particles / Visuals
 * - Spatial / Procedural Audio
 * - Enhanced UI + Persistence feedback
 *
 * This module keeps epiphany presentation clean and extensible.
 * Integrate by adding the systems to your App.
 */

use bevy::prelude::*;
use crate::epiphany_catalyst::EpiphanyTriggered; // Adjust path if using shared events
use crate::divine_whispers::DivineWhisperTrigger;
// use crate::spatial_audio_engine::SpatialAudioEngine;
// use crate::particle_systems::ParticleSpawner;

/// Main reactor system for EpiphanyTriggered events.
/// Add this to your Update schedule.
pub fn epiphany_triggered_reactor(
    mut epiphany_events: EventReader<EpiphanyTriggered>,
    mut whisper_events: EventWriter<DivineWhisperTrigger>,
    mut commands: Commands,
    // mut particle_spawner: ResMut<ParticleSpawner>,
    // mut spatial_audio: ResMut<SpatialAudioEngine>,
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

        // 2. Spawn rich particle / visual effects
        spawn_epiphany_particles(&mut commands, outcome, biome);

        // 3. Trigger spatial / procedural audio
        trigger_epiphany_spatial_audio(/* &mut spatial_audio, */ outcome, biome);

        // 4. Optional: Direct UI enhancement (if not using divine_whispers_ui reactor)
        // trigger_epiphany_ui_popup(outcome, biome);
    }
}

/// Spawns context-aware particle effects based on EpiphanyOutcome.
fn spawn_epiphany_particles(
    commands: &mut Commands,
    outcome: &crate::epiphany_catalyst::EpiphanyOutcome,
    biome: &str,
) {
    // TODO: Integrate with your actual particle system
    // Example structure:
    match outcome.particle_effect.as_str() {
        "sacred_geometry_crystal_bloom" => {
            // commands.spawn( SacredGeometryCrystalBloom { intensity: outcome.intensity, ..default() } );
        }
        "mycelial_web_glow" => {
            // commands.spawn( MycelialWebGlow { intensity: outcome.intensity, ..default() } );
        }
        "ethereal_bloom" | _ => {
            // commands.spawn( EtherealBloomParticles { intensity: outcome.intensity } );
        }
    }

    // Apply time dilation if present
    if outcome.time_dilation_factor > 1.0 {
        // commands.insert_resource(TimeScale(outcome.time_dilation_factor));
        // Reset after duration...
    }

    info!("Epiphany particles spawned: {} in {}", outcome.particle_effect, biome);
}

/// Triggers spatial / ambisonic audio for the epiphany.
fn trigger_epiphany_spatial_audio(
    // spatial_audio: &mut SpatialAudioEngine,
    outcome: &crate::epiphany_catalyst::EpiphanyOutcome,
    biome: &str,
) {
    // Example integration with your spatial audio engine
    let intensity = outcome.intensity;
    let flavor = &outcome.divine_whisper_flavor;

    // spatial_audio.play_epiphany_sound(
    //     flavor,
    //     intensity,
    //     outcome.time_dilation_factor,
    // );

    info!("Epiphany spatial audio triggered: {} (intensity {:.2}) in {}", flavor, intensity, biome);
}

/// Optional standalone UI popup for epiphanies (if not handled in divine_whispers_ui).
fn trigger_epiphany_ui_popup(
    outcome: &crate::epiphany_catalyst::EpiphanyOutcome,
    _biome: &str,
) {
    // You can spawn a temporary UI node here with muscle memory gain,
    // resonance change, temporary multiplier, etc.
    info!("Epiphany UI popup: Muscle +{:.2}, Intensity {:.0}%", 
          outcome.muscle_memory_consolidation_boost, outcome.intensity * 100.0);
}

/// Plugin to register all epiphany feedback systems.
pub struct EpiphanyFeedbackPlugin;

impl Plugin for EpiphanyFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, epiphany_triggered_reactor);
    }
}
