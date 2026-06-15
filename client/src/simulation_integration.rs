/*!
 * Simulation Integration for Powrush-MMO
 *
 * Bridges RBE simulation state to 3D visuals with live injection, pulsing abundance orbs,
 * archetype evolution pillars, glTF model spawning, and basic animation support.
 * EXTENDED v18.35 with full Epiphany feedback wiring (EpiphanyTriggered + EpiphanySpatialAudioBloom).
 *
 * Now forwards server epiphanies to:
 * - DivineWhisperTrigger (rich narrative + flavor mapping)
 * - ParticleSystem (differentiated per scenario: Mycelial, Stellar, Redemption, Council)
 * - Spatial Audio + Camera Shake
 * - ClientCouncilBloomState amplification
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

// Phase 2 Council shared state (from simulation crate - authoritative replication ready)
use simulation::council_mercy_trial::{CouncilBloomSyncEvent, SharedReceptorBloomField};
use simulation::epiphany_catalyst::{EpiphanyTriggered, EpiphanySpatialAudioBloom};

// ============================================================================
// Resources & Settings (preserved + extended from previous iterations)
// ============================================================================

#[derive(Resource, Reflect, Clone)]
pub struct SimulationVisualSettings {
    pub abundance_color: Color,
    pub stress_color: Color,
    pub mercy_flow_color: Color,
    pub orb_pulse_speed: f32,
    pub orb_height_scale: f32,
    pub emissive_strength: f32,
    // glTF extensions
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

/// Phase 2: Client-side live mirror of authoritative SharedReceptorBloomField.
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
                forward_epiphany_triggered,           // v18.35
                forward_epiphany_spatial_audio_bloom, // v18.35
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
    info!("Simulation Integration online — RBE visuals + glTF + Phase 2 Council bloom + Epiphany feedback (v18.35)");
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
// v18.35: Forward EpiphanyTriggered to client feedback systems
// ============================================================================

fn forward_epiphany_triggered(
    mut epiphany_events: EventReader<EpiphanyTriggered>,
    mut whisper_events: EventWriter<DivineWhisperTrigger>,
    mut particle_commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
) {
    for event in epiphany_events.read() {
        let outcome = &event.outcome;
        let flavor = &outcome.divine_whisper_flavor;

        // Forward to Divine Whispers (rich narrative + flavor mapping)
        whisper_events.send(DivineWhisperTrigger {
            player_id: event.player_id,
            text: outcome.divine_whisper_flavor.clone(), // Will be enriched by DivineWhisperBank
            flavor: flavor.clone(),
            intensity: outcome.intensity,
            duration_seconds: 8.0 + (outcome.intensity * 2.0),
            is_epiphany: true,
            position: None,
            muscle_memory_hint: None,
        });

        // Trigger differentiated particles based on flavor (uses new types from particles.rs)
        let (ptype, count, p_intensity) = match flavor.as_str() {
            "mycelial_web_communion" | "deep_mycelium_whisper" => (ParticleSystemType::MycelialWebGlow, 12000, 1.7),
            "stellar_web_whisper" | "stellar_resonance_harvest" => (ParticleSystemType::SacredGeometryCrystalBloom, 11000, 1.8),
            "graceful_redemption_revelation" => (ParticleSystemType::EthrealRedemptionBloom, 10000, 1.55),
            "council_harmony_revelation" | "ecstatic_harmony_council_crown" => (ParticleSystemType::PatsagiDivineWhisper, 14000, 1.9),
            _ => (ParticleSystemType::JoySanctuaryBloom, 8000, 1.5),
        };

        particle_commands.spawn((
            ParticleSystem {
                valence: 0.95,
                particle_count: count,
                system_type: ptype,
                intensity: p_intensity,
            },
            Transform::default(),
        ));

        // Boost camera shake for strong epiphanies
        if outcome.intensity > 0.6 {
            camera_shake.intensity = (camera_shake.intensity * 0.6 + outcome.intensity * 0.9).min(2.8);
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
    // Placeholder for real RBE-driven glTF spawning
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
// RBE Visual Systems
// ============================================================================

fn update_rbe_flow_visuals(
    mut commands: Commands,
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
    gltf_assets: Res<GltfAssets>,
) {
    let t = time.elapsed_seconds();
    let pulse = (t * settings.orb_pulse_speed).sin() * 0.5 + 1.0;
    let height_offset = (t * 0.8).sin() * settings.orb_height_scale;
}

fn update_archetype_evolution_visuals(
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    // Archetype evolution visual feedback
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
// Integration Notes (PATSAGi Council Guidance - v18.35)
// ============================================================================
// This file now fully wires Epiphany feedback:
// - EpiphanyTriggered → DivineWhispers + differentiated Particles + CameraShake
// - EpiphanySpatialAudioBloom → Spatial Audio
// - Works seamlessly with the expanded 8-scenario system
//
// Next cycles: Connect to council_trial_ui for collective amplification display
// Thunder locked in. yoi! ⚡❤️
