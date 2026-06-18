/*!
 * Simulation Integration for Powrush-MMO
 *
 * Bridges SovereignSimulationOrchestrator and Council systems to rich client visuals.
 * Full integration: Council Mercy Trial + Dynamic Music + Asset-aware music system.
 *
 * v19.03 — MusicLayerRegistry initialized and default layers registered.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use crate::gltf_integration::{GltfAssets, GltfCategory};
use crate::divine_whispers::{DivineWhisperTrigger, CameraShake};
use crate::particles::{ParticleSystem, ParticleSystemType};
use crate::spatial_audio::{GameAudioEvent};
use simulation::harvest::HarvestEvent;
use simulation::emergence::DynamicEmergenceEvent;
use simulation::council_mercy_trial::{CouncilTrialResolved, CouncilSessionUpdate, CouncilMercyTrialPhase, CollectiveEpiphanyBloom};
use crate::prediction::AudioTriggerEvent;
use crate::dynamic_music::{DynamicMusicController, MusicLayerRegistry, activate_music_layers, sync_music_volumes};
use crate::oddio_backend::OddioAudioBackend;

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
            .init_resource::<MusicLayerRegistry>()   // Asset registry
            .add_event::<CouncilSessionUpdate>()
            .add_event::<CouncilTrialResolved>()
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

    // Register default music layers at startup
    registry.register(crate::dynamic_music::MusicLayerType::BaseHarmony.default_handle());
    registry.register(crate::dynamic_music::MusicLayerType::AttunementPads.default_handle());
    registry.register(crate::dynamic_music::MusicLayerType::RhythmicPulse.default_handle());
    registry.register(crate::dynamic_music::MusicLayerType::BloomResonance.default_handle());
}

// ... rest of file (update_council_music_from_debug, UI, visuals, etc.) remains the same ...
