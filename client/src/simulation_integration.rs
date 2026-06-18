/*!
 * Simulation Integration for Powrush-MMO
 *
 * Bridges SovereignSimulationOrchestrator and Council systems to rich client visuals.
 * Phase 2: Oddio backend integration started.
 *
 * v18.95 — OddioAudioBackend initialized.
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
use crate::dynamic_music::{DynamicMusicController, update_music_layer_volumes};
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
            .init_resource::<OddioAudioBackend>()   // New oddio backend
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
                update_music_layer_volumes,
                update_rbe_flow_visuals,
                update_archetype_evolution_visuals,
                rbe_live_injection_system,
                spawn_gltf_for_rbe_entities,
                update_gltf_animations,
            ))
            .register_type::<SimulationVisualSettings>();
    }
}

pub fn setup_simulation_integration(mut commands: Commands) {
    info!("Simulation Integration online — Oddio backend active + Dynamic Music (v18.95)");
}

// New system: Drive MusicState from DebugCouncilTrial
fn update_council_music_from_debug(
    debug_trial: Res<DebugCouncilTrial>,
    mut music: ResMut<DynamicMusicController>,
) {
    if debug_trial.active {
        music.state.council_phase = Some(debug_trial.phase);
        music.state.attunement = debug_trial.attunement;
        music.state.intensity = (debug_trial.attunement * 0.7 + 0.3).clamp(0.0, 1.0);
        music.state.is_resolving = debug_trial.phase == CouncilMercyTrialPhase::Resolution;
    } else {
        music.state.council_phase = None;
        music.state.attunement = 0.0;
        music.state.intensity = 0.0;
        music.state.is_resolving = false;
    }

    music.apply_state_to_layers();
}

// ... (rest of the file remains the same for now)
