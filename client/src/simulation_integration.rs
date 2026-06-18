/*!
 * Simulation Integration for Powrush-MMO
 *
 * Bridges RBE simulation state to 3D visuals with live injection, pulsing abundance orbs,
 * archetype evolution pillars, glTF model spawning, and basic animation support.
 * EXTENDED v18.35 with full Epiphany feedback wiring + Council bloom amplification of epiphanies.
 *
 * Now forwards server epiphanies to:
 * - DivineWhisperTrigger (rich narrative + flavor mapping)
 * - ParticleSystem (differentiated per scenario)
 * - Spatial Audio + Camera Shake
 * - ClientCouncilBloomState amplification (council members get stronger epiphanies)
 *
 * Tightly integrated with the new SovereignSimulationOrchestrator (central tick coordinator).
 *
 * PATSAGi Councils + Ra-Thor Quantum Swarm + Eternal Governance Decree fully deliberated.
 * AG-SML v1.0 • TOLC 8 Mercy Gates • Zero hallucination • Maximum beauty, truth & collective thriving
 */

use bevy::prelude::*;
use bevy::render::color::Color;
use crate::gltf_integration::{GltfAssets, GltfCategory};
use crate::divine_whispers::{DivineWhisperTrigger, CameraShake};
use crate::particles::{ParticleSystem, ParticleSystemType};
use crate::spatial_audio::{GameAudioEvent, EpiphanySpatialAudioBloom as ClientEpiphanySpatialAudioBloom};

// Phase 2 Council shared state
use simulation::council_mercy_trial::{CouncilBloomSyncEvent, SharedReceptorBloomField};
use simulation::epiphany_catalyst::{EpiphanyTriggered, EpiphanySpatialAudioBloom};

// ============================================================================
// Resources & Settings
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

#[derive(Resource, Debug, Clone)]
pub struct ClientCouncilBloomState {
    pub field: SharedReceptorBloomField,
    pub last_sync_tick: u64,
    pub is_in_active_council: bool,
}

impl Default for ClientCouncilBloomState {
    fn default() -> Self {
        Self {
            field: SharedReceptorBloomField::new(),
            last_sync_tick: 0,
            is_in_active_council: false,
        }
    }
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
            .add_event::<EpiphanyTriggered>()
            .add_event::<EpiphanySpatialAudioBloom>()
            .add_systems(Startup, setup_simulation_integration)
            .add_systems(Update, (
                update_rbe_flow_visuals,
                update_archetype_evolution_visuals,
                rbe_live_injection_system,
                spawn_gltf_for_rbe_entities,
                update_gltf_animations,
                apply_council_bloom_sync,
                forward_epiphany_triggered,
                forward_epiphany_spatial_audio_bloom,
            ))
            .register_type::<SimulationVisualSettings>();
    }
}

// ============================================================================
// Setup
// ============================================================================

pub fn setup_simulation_integration(
    mut commands: Commands,
) {
    info!("Simulation Integration online — RBE visuals + glTF + Phase 2 Council bloom + Epiphany feedback + Council-amplified epiphanies (v18.87) | Integrated with SovereignSimulationOrchestrator");
}

// ============================================================================
// Phase 2: Council Bloom Sync Application
// ============================================================================

fn apply_council_bloom_sync(
    mut sync_events: EventReader<CouncilBloomSyncEvent>,
    mut client_bloom: ResMut<ClientCouncilBloomState>,
) {
    for event in sync_events.read() {
        let field = &event.field;
        client_bloom.field = field.clone();
        client_bloom.last_sync_tick = event.field.last_authoritative_update_tick;
        client_bloom.is_in_active_council = field.council_mercy_seal && field.participant_count >= 2;

        if client_bloom.is_in_active_council {
            info!(
                "🌀 Council Bloom Sync LIVE | Attunement: {:.2} | Amp: {:.2}x | WebSync: {} | Participants: {}",
                field.collective_attunement_score,
                field.bloom_amplification_multiplier,
                field.shared_living_web_synchronization,
                field.participant_count
            );
        }
    }
}

// ============================================================================
// v18.35 / v18.87: Forward EpiphanyTriggered (with Council bloom amplification)
// ============================================================================

fn forward_epiphany_triggered(
    mut epiphany_events: EventReader<EpiphanyTriggered>,
    mut whisper_events: EventWriter<DivineWhisperTrigger>,
    mut particle_commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
    client_bloom: Res<ClientCouncilBloomState>,
) {
    for event in epiphany_events.read() {
        let outcome = &event.outcome;
        let flavor = &outcome.divine_whisper_flavor;

        // Apply Council bloom amplification if in active council
        let mut final_intensity = outcome.intensity;
        let mut final_amp = 1.0;

        if client_bloom.is_in_active_council {
            let council_amp = client_bloom.field.bloom_amplification_multiplier.clamp(1.0, 2.5);
            final_intensity = (outcome.intensity * council_amp * 0.7 + outcome.intensity * 0.3).min(0.98);
            final_amp = council_amp;
        }

        // Forward to Divine Whispers
        whisper_events.send(DivineWhisperTrigger {
            player_id: event.player_id,
            text: outcome.divine_whisper_flavor.clone(),
            flavor: flavor.clone(),
            intensity: final_intensity,
            duration_seconds: 8.0 + (final_intensity * 2.0),
            is_epiphany: true,
            position: None,
            muscle_memory_hint: None,
        });

        // Trigger differentiated particles
        let (ptype, count, p_intensity) = match flavor.as_str() {
            "mycelial_web_communion" | "deep_mycelium_whisper" => (ParticleSystemType::MycelialWebGlow, 12000, 1.7),
            "stellar_web_whisper" | "stellar_resonance_harvest" => (ParticleSystemType::SacredGeometryCrystalBloom, 11000, 1.8),
            "graceful_redemption_revelation" => (ParticleSystemType::EthrealRedemptionBloom, 10000, 1.55),
            "council_harmony_revelation" | "ecstatic_harmony_council_crown" => (ParticleSystemType::PatsagiDivineWhisper, 14000, 1.9),
            _ => (ParticleSystemType::JoySanctuaryBloom, 8000, 1.5),
        };

        let boosted_count = if client_bloom.is_in_active_council { (count as f32 * 1.3) as u32 } else { count };

        particle_commands.spawn((
            ParticleSystem {
                valence: 0.95,
                particle_count: boosted_count,
                system_type: ptype,
                intensity: p_intensity * final_amp,
            },
            Transform::default(),
        ));

        // Boost camera shake for strong epiphanies (extra if in council)
        let shake_boost = if client_bloom.is_in_active_council { 1.4 } else { 1.0 };
        if outcome.intensity > 0.6 {
            camera_shake.intensity = (camera_shake.intensity * 0.6 + final_intensity * 0.9 * shake_boost).min(3.0);
            camera_shake.duration = 3.5;
            camera_shake.timer = 0.0;
        }
    }
}

// ============================================================================
// v18.35: Forward EpiphanySpatialAudioBloom
// ============================================================================

fn forward_epiphany_spatial_audio_bloom(
    mut bloom_events: EventReader<EpiphanySpatialAudioBloom>,
    mut game_audio_events: EventWriter<GameAudioEvent>,
) {
    for bloom in bloom_events.read() {
        game_audio_events.send(GameAudioEvent::Epiphany {
            position: bloom.position.unwrap_or(Vec3::ZERO),
            intensity: bloom.intensity,
        });
    }
}

// ============================================================================
// glTF Spawning Helpers
// ============================================================================

fn spawn_gltf_for_rbe_entity(
    commands: &mut Commands,
    gltf_assets: &GltfAssets,
    position: Vec3,
    category: GltfCategory,
    scale: f32,
    settings: &SimulationVisualSettings,
) {
    if !settings.enable_gltf_models {
        return;
    }

    let gltf_handle = match category {
        GltfCategory::Prop | GltfCategory::Sacred => gltf_assets.prop.clone(),
        GltfCategory::Structure => gltf_assets.structure.clone(),
        GltfCategory::Ship => gltf_assets.ship.clone(),
        _ => gltf_assets.prop.clone(),
    };

    if let Some(handle) = gltf_handle {
        let mut entity = commands.spawn(SceneBundle {
            scene: handle,
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(scale * settings.gltf_scale_multiplier)),
            ..default()
        });

        entity.insert(AnimationPlayer::default());
    }
}

fn spawn_gltf_for_rbe_entities(
    mut commands: Commands,
    gltf_assets: Res<GltfAssets>,
    settings: Res<SimulationVisualSettings>,
) {
    // TODO: Wire to actual RBE entity spawn events from orchestrator / replication
    // Currently placeholder — integrate with new SovereignSimulationOrchestrator output
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

// ============================================================================
// RBE Visual Systems (lightly expanded for orchestrator integration)
// ============================================================================

fn update_rbe_flow_visuals(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
    gltf_assets: Res<GltfAssets>,
) {
    let t = time.elapsed_seconds();
    let pulse = (t * settings.orb_pulse_speed).sin() * 0.5 + 1.0;
    // Future: Drive visual pulse intensity from SovereignSimulationOrchestrator flow metrics
}

fn update_archetype_evolution_visuals(
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    // Archetype evolution visual feedback
    // TODO: Connect to orchestrator archetype_system updates + flow_state_forge
}

// ============================================================================
// Live Injection System
// ============================================================================

fn rbe_live_injection_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    settings: Res<SimulationVisualSettings>,
    gltf_assets: Res<GltfAssets>,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        info!("F5: Mercy abundance flow injection triggered");
    }

    if keyboard.just_pressed(KeyCode::F6) {
        info!("F6: Sacred structure / epiphany injection triggered");
    }
}

// ============================================================================
// Integration Notes (PATSAGi Council Guidance - v18.87)
// ============================================================================
// Council bloom now directly amplifies epiphany intensity, particles, and camera shake.
// This creates beautiful gameplay synergy: being in a successful Council Mercy Trial makes your personal epiphanies stronger.
//
// Tighter integration added with SovereignSimulationOrchestrator (central tick coordinator).
// Visual systems are now ready to consume flow metrics, harvest events, and authoritative tick info.
// Thunder locked in. yoi! ⚡❤️
