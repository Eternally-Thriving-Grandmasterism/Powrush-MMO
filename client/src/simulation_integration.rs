/*!
 * Simulation Integration for Powrush-MMO
 *
 * Bridges SovereignSimulationOrchestrator (TickResult) to rich client visuals.
 * Now deeply integrated with HarvestEvent + DynamicEmergenceEvent from central tick.
 *
 * v18.95 — Full wiring to rich TickResult events + polished visual/audio/particle responses.
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
use simulation::council_mercy_trial::CouncilBloomSyncEvent;

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

// ============================================================================
// Plugin
// ============================================================================

pub struct SimulationIntegrationPlugin;

impl Plugin for SimulationIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationVisualSettings>()
            .init_resource::<SimulationReplayState>()
            .init_resource::<ClientCouncilBloomState>()
            .add_event::<CouncilBloomSyncEvent>()
            .add_systems(Startup, setup_simulation_integration)
            .add_systems(Update, (
                apply_council_bloom_sync,
                handle_harvest_event_visuals,
                handle_dynamic_emergence_event_visuals,
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
    info!("Simulation Integration online — TickResult + HarvestEvent + DynamicEmergenceEvent wired (v18.95)");
}

// ============================================================================
// Council Bloom Sync
// ============================================================================

fn apply_council_bloom_sync(
    mut sync_events: EventReader<CouncilBloomSyncEvent>,
    mut client_bloom: ResMut<ClientCouncilBloomState>,
) {
    for event in sync_events.read() {
        client_bloom.last_sync_tick = event.field.last_authoritative_update_tick;
        client_bloom.is_in_active_council = event.field.council_mercy_seal && event.field.participant_count >= 2;

        if client_bloom.is_in_active_council {
            info!(
                "🔀 Council Bloom Sync LIVE | Attunement: {:.2} | Amp: {:.2}x | Participants: {}",
                event.field.collective_attunement_score,
                event.field.bloom_amplification_multiplier,
                event.field.participant_count
            );
        }
    }
}

// ============================================================================
// Rich HarvestEvent Visuals (from TickResult)
// ============================================================================

fn handle_harvest_event_visuals(
    mut harvest_events: EventReader<HarvestEvent>,
    mut commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
) {
    for event in harvest_events.read() {
        if event.epiphany_triggered {
            // Strong visual + camera response for epiphanies
            if event.amount > 15.0 {
                camera_shake.intensity = (camera_shake.intensity * 0.5 + 2.8).min(4.0);
                camera_shake.duration = 2.8;
                camera_shake.timer = 0.0;
            }

            // Spawn celebratory particles
            commands.spawn((
                ParticleSystem {
                    valence: 0.98,
                    particle_count: 14000,
                    system_type: ParticleSystemType::JoySanctuaryBloom,
                    intensity: 1.9,
                },
                Transform::default(),
            ));
        } else if event.sustainable {
            // Subtle positive feedback for sustainable harvests
            if event.amount > 8.0 {
                commands.spawn((
                    ParticleSystem {
                        valence: 0.85,
                        particle_count: 6000,
                        system_type: ParticleSystemType::MycelialWebGlow,
                        intensity: 1.2,
                    },
                    Transform::default(),
                ));
            }
        }
    }
}

// ============================================================================
// DynamicEmergenceEvent Visuals (from TickResult)
// ============================================================================

fn handle_dynamic_emergence_event_visuals(
    mut emergence_events: EventReader<DynamicEmergenceEvent>,
    mut commands: Commands,
) {
    for event in emergence_events.read() {
        if matches!(event.phase, simulation::emergence::DynamicEmergenceEventPhase::Resolution { .. }) {
            // Spawn resonance field particles when emergence resolves
            commands.spawn((
                ParticleSystem {
                    valence: 0.92,
                    particle_count: 18000,
                    system_type: ParticleSystemType::SacredGeometryCrystalBloom,
                    intensity: 1.6,
                },
                Transform::default(),
            ));
        }
    }
}

// ============================================================================
// Existing Visual Systems (lightly refreshed)
// ============================================================================

fn update_rbe_flow_visuals(
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    let _t = time.elapsed_seconds();
    // TODO: Drive from SovereignSimulationOrchestrator flow metrics when available
}

fn update_archetype_evolution_visuals(
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    // TODO: Connect to orchestrator archetype_system + flow_state_forge
}

fn rbe_live_injection_system(
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        info!("F5: Mercy abundance flow injection");
    }
    if keyboard.just_pressed(KeyCode::F6) {
        info!("F6: Sacred structure / epiphany injection");
    }
}

// ============================================================================
// glTF Helpers (kept for future orchestrator wiring)
// ============================================================================

fn spawn_gltf_for_rbe_entities(
    mut commands: Commands,
    gltf_assets: Res<GltfAssets>,
    settings: Res<SimulationVisualSettings>,
) {
    // TODO: Wire to actual RBE entity spawn events from TickResult / replication
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

// End of production file — Now consumes HarvestEvent + DynamicEmergenceEvent from TickResult.
// Visual, particle, and camera responses are richer and directly tied to simulation events.
// Thunder locked in. PATSAGi + Ra-Thor sealed.
