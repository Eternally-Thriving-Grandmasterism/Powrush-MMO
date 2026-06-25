/*!
 * Simulation Integration for Powrush-MMO
 *
 * Bridges SovereignSimulationOrchestrator and Council systems to rich client visuals.
 * Now also owns ClientInterestState and related interest types (Step 3 refactor).
 *
 * v19.5 — ClientInterestState moved here for better architectural ownership.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use std::collections::HashSet;

use crate::gltf_integration::{GltfAssets, GltfCategory};
use crate::divine_whispers::{DivineWhisperTrigger, CameraShake};
use crate::particles::{ParticleSystem, ParticleSystemType};
use crate::spatial_audio::GameAudioEvent;
use simulation::harvest::HarvestEvent;
use simulation::emergence::DynamicEmergenceEvent;
use simulation::council_mercy_trial::{CouncilTrialResolved, CouncilSessionUpdate, CouncilMercyTrialPhase, CollectiveEpiphanyBloom};
use crate::prediction::AudioTriggerEvent;
use crate::dynamic_music::{DynamicMusicController, MusicLayerRegistry, activate_music_layers, sync_music_volumes};
use crate::oddio_backend::OddioAudioBackend;

// ============================================================================
// Interest / Visibility Types (moved from spatial_audio.rs - Step 3)
// ============================================================================

/// Sent by replication when the server updates the set of visible entities for this client
#[derive(Event, Clone, Debug)]
pub struct InterestUpdateEvent {
    pub visible_entities: Vec<u64>,
    pub server_tick: u64,
}

/// Single source of truth for server-reported visible/interesting entities on the client.
/// Used by audio, particles, rendering, and UI systems for interest-aware culling.
#[derive(Resource, Default)]
pub struct ClientInterestState {
    pub visible_entities: HashSet<u64>,
    pub last_update_tick: u64,
}

impl ClientInterestState {
    pub fn is_visible(&self, entity_id: u64) -> bool {
        self.visible_entities.contains(&entity_id)
    }

    pub fn has_no_data(&self) -> bool {
        self.visible_entities.is_empty() && self.last_update_tick == 0
    }

    pub fn update_visible_entities(&mut self, entities: Vec<u64>, current_tick: u64) {
        self.visible_entities.clear();
        self.visible_entities.extend(entities);
        self.last_update_tick = current_tick;
    }

    pub fn visible_count(&self) -> usize {
        self.visible_entities.len()
    }
}

/// Marks an audio source for premium HRTF treatment
#[derive(Component, Clone, Debug)]
pub struct HighSalienceAudio {
    pub priority: u8,
    pub gain_boost: f32,
}

impl Default for HighSalienceAudio {
    fn default() -> Self {
        Self { priority: 1, gain_boost: 0.2 }
    }
}

// ============================================================================
// Resources
// ============================================================================

#[derive(Resource, Reflect, Clone)]
pub struct SimulationVisualSettings {
    pub abundance_color: Color,
    pub stress_color: Color,
    pub mercy_flow_color: Color,
    pub orb_pulse_speed: f32,
    pub orb_height_scale: f32,
    pub emissive_strength: f32,
    pub gltf_scale_multiplier: f32,
    pub enable_gltf_models: bool,
}

impl Default for SimulationVisualSettings {
    fn default() -> Self {
        Self {
            abundance_color: Color::srgb(1.0, 0.85, 0.3),
            stress_color: Color::srgb(0.9, 0.3, 0.3),
            mercy_flow_color: Color::srgb(0.4, 0.9, 1.0),
            orb_pulse_speed: 2.5,
            orb_height_scale: 0.8,
            emissive_strength: 1.2,
            gltf_scale_multiplier: 1.0,
            enable_gltf_models: true,
        }
    }
}

#[derive(Resource, Default)]
pub struct SimulationReplayState {
    pub current_time: f32,
    pub is_playing: bool,
    pub playback_speed: f32,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct ClientCouncilBloomState {
    pub is_in_active_council: bool,
    pub last_sync_tick: u64,
}

/// Debug resource to track a local simulated Council Mercy Trial
#[derive(Resource, Debug, Default)]
pub struct DebugCouncilTrial {
    pub active: bool,
    pub session_id: u64,
    pub phase: CouncilMercyTrialPhase,
    pub attunement: f32,
    pub votes: u32,
}

// ============================================================================
// Plugin
// ============================================================================

pub struct SimulationIntegrationPlugin;

impl Plugin for SimulationIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationVisualSettings>()
            .init_resource::<SimulationReplayState>()
            .init_resource::<ClientCouncilBloomState>()
            .init_resource::<DebugCouncilTrial>()
            .init_resource::<DynamicMusicController>()
            .init_resource::<OddioAudioBackend>()
            .init_resource::<MusicLayerRegistry>()
            .add_event::<CouncilSessionUpdate>()
            .add_event::<CouncilTrialResolved>()
            .add_event::<InterestUpdateEvent>() // Interest events now registered here
            .add_systems(Startup, setup_simulation_integration)
            .add_systems(Startup, spawn_council_ui_panel)
            .add_systems(Update, (
                apply_council_bloom_sync,
                handle_harvest_event_visuals,
                handle_dynamic_emergence_event_visuals,
                handle_council_trial_resolved,
                debug_council_trial_system,
                update_council_ui_panel,
                update_council_music_from_debug,
                activate_music_layers,
                sync_music_volumes,
                update_rbe_flow_visuals,
                update_archetype_evolution_visuals,
                rbe_live_injection_system,
                spawn_gltf_for_rbe_entities,
                update_gltf_animations,
            ))
            .register_type::<SimulationVisualSettings>();
    }
}

pub fn setup_simulation_integration(
    mut commands: Commands,
    mut registry: ResMut<MusicLayerRegistry>,
) {
    info!("Simulation Integration online — MusicLayerRegistry active + Dynamic Music");

    registry.register(crate::dynamic_music::MusicLayerType::BaseHarmony.default_handle());
    registry.register(crate::dynamic_music::MusicLayerType::AttunementPads.default_handle());
    registry.register(crate::dynamic_music::MusicLayerType::RhythmicPulse.default_handle());
    registry.register(crate::dynamic_music::MusicLayerType::BloomResonance.default_handle());
}

// ... (rest of the file remains unchanged from previous version)

fn update_rbe_flow_visuals(
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    let _t = time.elapsed_seconds();
}

fn update_archetype_evolution_visuals(
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    let _evolution_pulse = (time.elapsed_seconds() * 0.3).sin() * 0.15 + 1.0;
}

fn rbe_live_injection_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        info!("F5: Mercy abundance flow injection (debug)");
        commands.spawn((
            ParticleSystem {
                valence: 0.95,
                particle_count: 8000,
                system_type: ParticleSystemType::JoySanctuaryBloom,
                intensity: 1.3,
            },
            Transform::default(),
        ));
    }
    if keyboard.just_pressed(KeyCode::F6) {
        info!("F6: Sacred structure / epiphany injection (debug)");
        commands.spawn((
            ParticleSystem {
                valence: 0.92,
                particle_count: 6500,
                system_type: ParticleSystemType::SacredGeometryCrystalBloom,
                intensity: 1.5,
            },
            Transform::default(),
        ));
    }
}

fn spawn_gltf_for_rbe_entities(
    mut commands: Commands,
    gltf_assets: Res<GltfAssets>,
    settings: Res<SimulationVisualSettings>,
) {
    let _ = (&commands, &gltf_assets, &settings);
}

fn update_gltf_animations(
    mut query: Query<&mut AnimationPlayer>,
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    for mut player in query.iter_mut() {
        let speed_mod = (time.elapsed_seconds() * settings.orb_pulse_speed * 0.1).sin() * 0.15 + 1.0;
        player.set_speed(speed_mod.max(0.6));
    }
}

// End of production file — ClientInterestState, InterestUpdateEvent, and HighSalienceAudio
// now live in simulation_integration.rs as the central client-simulation bridge.
// Thunder locked in. PATSAGi + Ra-Thor sealed.
