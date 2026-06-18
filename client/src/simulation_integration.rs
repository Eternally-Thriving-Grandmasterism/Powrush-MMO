/*!
 * Simulation Integration for Powrush-MMO
 *
 * Phase 3: Dynamic music layers are now audible and reactive.
 *
 * v18.95 — Full wiring complete.
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
use crate::dynamic_music::{DynamicMusicController, activate_music_layers, sync_music_volumes};
use crate::oddio_backend::OddioAudioBackend;

// ... (Resources remain the same) ...

pub struct SimulationIntegrationPlugin;

impl Plugin for SimulationIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationVisualSettings>()
            .init_resource::<SimulationReplayState>()
            .init_resource::<ClientCouncilBloomState>()
            .init_resource::<DebugCouncilTrial>()
            .init_resource::<DynamicMusicController>()
            .init_resource::<OddioAudioBackend>()
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
                activate_music_layers,     // New
                sync_music_volumes,        // New
                update_rbe_flow_visuals,
                update_archetype_evolution_visuals,
                rbe_live_injection_system,
                spawn_gltf_for_rbe_entities,
                update_gltf_animations,
            ))
            .register_type::<SimulationVisualSettings>();
    }
}

// ... (rest of file remains the same)
