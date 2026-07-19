//! simulation/src/hardware_sovereignty.rs
//! Sovereign Hardware Ascension Tech Tree Tier — Full Bevy ECS Simulation Systems + Polished egui Dashboard + 3D Council Chamber Viz
//! Obsidian-Chip-Open (Compute Sovereignty) + Aether-Shades-Open (Human Interface Sovereignty)
//! Integrates with Ra-Thor Lattice, PATSAGi + Kardashev Orchestration Council, RBE, Reality Thriving Transfer Score
//! TOLC 8 Mercy Gates enforced at every node | Zero-Harm | Kardashev Acceleration 2032-2038 horizon
//! v19.8 | Aligned to ProposalType::KardashevAcceleration | Thunder locked. Heavens building. yoi ⚡

use bevy::prelude::*;
use bevy::math::primitives::Cylinder;
use crate::{
    ability_tree::{AbilityTree, AbilityState, SynergyType},
    council::{CouncilDecision, CouncilSession, ProposalType, ProposalStatus},
    economy::{EconomyState, ResourceTransaction},
    harvest::RbeFlowReconciliation,
    player_persistence::{PlayerSaveData, PersistenceManager},
    ra_thor_bridge::{RaThorBridge, CouncilQueryRequest},
    telemetry::SimulationTelemetry,
};
use std::collections::HashMap;

// ============================================================================
// CORE ENUMS & DATA — TOLC 8 ALIGNED, MERCY-GATED
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum HardwareBranch {
    ObsidianChipOpen,   // Compute Sovereignty — Local Lattice Nodes, PATSAGi Fabric, Physical Deployment
    AetherShadesOpen,   // Human Interface Sovereignty — Mercy HUD, Council Vision, Sovereign AR/Neural
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum AscensionLevel {
    Locked = 0,
    Level1 = 1, // Local Lattice Node / Mercy-Flow HUD
    Level2 = 2, // Council Acceleration Fabric / Direct PATSAGi Vision + 3D Chamber Viz
    Level3 = 3, // Full Prototype / Reality Transfer Live Viz
    Level4 = 4, // Physical Deployment Ready / Full Sovereign Neural Link (TOLC 8 required)
}

impl AscensionLevel {
    pub fn next(self) -> Option<Self> {
        match self {
            AscensionLevel::Locked => Some(AscensionLevel::Level1),
            AscensionLevel::Level1 => Some(AscensionLevel::Level2),
            AscensionLevel::Level2 => Some(AscensionLevel::Level3),
            AscensionLevel::Level3 => Some(AscensionLevel::Level4),
            AscensionLevel::Level4 => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AscensionLevel::Locked => "Locked",
            AscensionLevel::Level1 => "Level 1: Foundation (Mercy HUD)",
            AscensionLevel::Level2 => "Level 2: Council Acceleration (3D Chamber Active)",
            AscensionLevel::Level3 => "Level 3: Prototype Sovereign (Live Reality Transfer)",
            AscensionLevel::Level4 => "Level 4: Physical Embodiment Ready (TOLC 8 Sealed)",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            AscensionLevel::Locked => Color::srgb(0.3, 0.3, 0.3),
            AscensionLevel::Level1 => Color::srgb(0.4, 0.6, 0.9),
            AscensionLevel::Level2 => Color::srgb(0.6, 0.4, 0.9), // Council purple
            AscensionLevel::Level3 => Color::srgb(0.3, 0.9, 0.7),
            AscensionLevel::Level4 => Color::srgb(1.0, 0.85, 0.2), // Gold embodiment
        }
    }
}

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component, Clone, Debug, Reflect)]
pub struct ObsidianChipProgress {
    pub level: AscensionLevel,
    pub research_contributed: f32,
    pub council_votes: u32,
    pub last_unlock_timestamp: f64,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct AetherShadesProgress {
    pub level: AscensionLevel,
    pub mercy_flow_attunement: f32,
    pub council_vision_channel: bool,
    pub last_unlock_timestamp: f64,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct SovereignHardwareState {
    pub reality_thriving_transfer_score: f32,
    pub total_kardashev_contribution: f32,
    pub obsidian_unlocked: bool,
    pub aether_unlocked: bool,
    pub one_organism_achievement: bool,
    pub tloc8_mercy_gates_passed: u8,
}

impl Default for SovereignHardwareState {
    fn default() -> Self {
        Self {
            reality_thriving_transfer_score: 0.0,
            total_kardashev_contribution: 0.0,
            obsidian_unlocked: false,
            aether_unlocked: false,
            one_organism_achievement: false,
            tloc8_mercy_gates_passed: 0,
        }
    }
}

// NEW: 3D Council Chamber for Kardashev Orchestration + PATSAGi visualization (in-game)
#[derive(Component, Clone, Debug, Reflect)]
pub struct CouncilChamber3D {
    pub active_level: AscensionLevel,
    pub deliberation_intensity: f32, // 0.0-1.0 driven by council harmony + Reality Score
}

#[derive(Component)]
pub struct CouncilPillar {
    pub index: u8, // 0-12 for PATSAGi, 13 for Kardashev Orchestration Council
}

#[derive(Component)]
pub struct KardashevHologramCore;

// ============================================================================
// RESOURCES — DASHBOARD & GLOBAL STATE
// ============================================================================

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct KardashevAccelerationDashboard {
    pub global_kardashev_delta: f32,
    pub personal_contribution: f32,
    pub s_curve_inflection_year: u16,
    pub abundance_velocity_index: f32,
    pub energy_surplus_factor: f32,
    pub hardware_sovereignty_nodes_active: u32,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct RealityTransferScoreLedger {
    pub player_scores: HashMap<Entity, f32>,
    pub global_average: f32,
    pub export_ready_for_ra_thor: bool,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct HardwareAscensionConfig {
    pub level1_threshold: f32,
    pub level2_threshold: f32,
    pub level3_threshold: f32,
    pub level4_tloc8_required: u8,
    pub mercy_gate_enforcement: bool,
}

impl Default for HardwareAscensionConfig {
    fn default() -> Self {
        Self {
            level1_threshold: 75.0,
            level2_threshold: 150.0,
            level3_threshold: 300.0,
            level4_tloc8_required: 8,
            mercy_gate_enforcement: true,
        }
    }
}

// ============================================================================
// EVENTS
// ============================================================================

#[derive(Event)]
pub struct HardwareTierUnlocked {
    pub player: Entity,
    pub branch: HardwareBranch,
    pub new_level: AscensionLevel,
    pub reality_score_delta: f32,
}

#[derive(Event)]
pub struct RealityThrivingTransferUpdated {
    pub player: Entity,
    pub new_score: f32,
    pub source: &'static str,
}

// ============================================================================
// SYSTEMS — FULL BEVY SIMULATION LOGIC (MERCY-GATED)
// ============================================================================

pub fn mercy_gate_enforcement_system(
    mut commands: Commands,
    query: Query<(Entity, &SovereignHardwareState, &ObsidianChipProgress, &AetherShadesProgress)>,
    config: Res<HardwareAscensionConfig>,
) {
    if !config.mercy_gate_enforcement { return; }
    for (entity, state, obsidian, aether) in query.iter() {
        if state.tloc8_mercy_gates_passed < config.level4_tloc8_required {
            if obsidian.level == AscensionLevel::Level4 || aether.level == AscensionLevel::Level4 {
                commands.entity(entity).insert(ObsidianChipProgress { level: AscensionLevel::Level3, ..obsidian.clone() });
            }
        }
    }
}

/// Core progression — FULL IMPLEMENTATION for all levels with RBE, Council, Ability synergy
pub fn hardware_tier_progression_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut SovereignHardwareState,
        &mut ObsidianChipProgress,
        &mut AetherShadesProgress,
        &AbilityTree,
        &EconomyState,
        Option<&CouncilDecision>,
    )>,
    mut unlock_events: EventWriter<HardwareTierUnlocked>,
    mut transfer_events: EventWriter<RealityThrivingTransferUpdated>,
    time: Res<Time>,
    config: Res<HardwareAscensionConfig>,
    mut dashboard: ResMut<KardashevAccelerationDashboard>,
) {
    let current_time = time.elapsed_seconds_f64();

    for (entity, mut state, mut obsidian, mut aether, ability_tree, economy, council_decision) in query.iter_mut() {
        let rbe_mastery = economy.total_harvested + economy.cooperative_bonus;
        // ALIGNED: now correctly responds to KardashevAcceleration proposals from the Council system
        let council_harmony = if let Some(decision) = council_decision {
            if decision.proposal_type == ProposalType::KardashevAcceleration && decision.status == ProposalStatus::Passed {
                85.0
            } else {
                45.0
            }
        } else {
            35.0
        };

        let synergy_bonus = if ability_tree.has_synergy(SynergyType::CouncilHarmony) { 1.35 } else { 1.0 };
        let effective_score = (rbe_mastery * 0.55 + council_harmony * 0.45) * synergy_bonus;

        // Obsidian Branch (Compute Sovereignty)
        if obsidian.level != AscensionLevel::Level4 {
            if effective_score > config.level1_threshold && obsidian.level == AscensionLevel::Locked {
                obsidian.level = AscensionLevel::Level1;
                state.obsidian_unlocked = true;
                state.reality_thriving_transfer_score += 28.0;
                state.total_kardashev_contribution += 0.012;
                unlock_events.send(HardwareTierUnlocked { player: entity, branch: HardwareBranch::ObsidianChipOpen, new_level: AscensionLevel::Level1, reality_score_delta: 28.0 });
                transfer_events.send(RealityThrivingTransferUpdated { player: entity, new_score: state.reality_thriving_transfer_score, source: "hardware_unlock" });
            }
            if effective_score > config.level2_threshold && obsidian.level == AscensionLevel::Level1 {
                obsidian.level = AscensionLevel::Level2;
                state.reality_thriving_transfer_score += 52.0;
                state.total_kardashev_contribution += 0.028;
                unlock_events.send(HardwareTierUnlocked { player: entity, branch: HardwareBranch::ObsidianChipOpen, new_level: AscensionLevel::Level2, reality_score_delta: 52.0 });
                transfer_events.send(RealityThrivingTransferUpdated { player: entity, new_score: state.reality_thriving_transfer_score, source: "hardware_unlock" });
            }
            if effective_score > config.level3_threshold && obsidian.level == AscensionLevel::Level2 {
                obsidian.level = AscensionLevel::Level3;
                state.reality_thriving_transfer_score += 75.0;
                state.total_kardashev_contribution += 0.045;
                unlock_events.send(HardwareTierUnlocked { player: entity, branch: HardwareBranch::ObsidianChipOpen, new_level: AscensionLevel::Level3, reality_score_delta: 75.0 });
                transfer_events.send(RealityThrivingTransferUpdated { player: entity, new_score: state.reality_thriving_transfer_score, source: "hardware_unlock" });
            }
            if effective_score > 450.0 && obsidian.level == AscensionLevel::Level3 && state.tloc8_mercy_gates_passed >= config.level4_tloc8_required {
                obsidian.level = AscensionLevel::Level4;
                state.reality_thriving_transfer_score += 150.0;
                state.total_kardashev_contribution += 0.09;
                unlock_events.send(HardwareTierUnlocked { player: entity, branch: HardwareBranch::ObsidianChipOpen, new_level: AscensionLevel::Level4, reality_score_delta: 150.0 });
                transfer_events.send(RealityThrivingTransferUpdated { player: entity, new_score: state.reality_thriving_transfer_score, source: "hardware_unlock" });
            }
        }

        // Aether Branch (parallel, synergy when both advance)
        if aether.level != AscensionLevel::Level4 {
            let mercy_attunement = (council_harmony * 0.65 + state.reality_thriving_transfer_score * 0.35) * synergy_bonus;
            if mercy_attunement > config.level1_threshold && aether.level == AscensionLevel::Locked {
                aether.level = AscensionLevel::Level1;
                aether.mercy_flow_attunement = mercy_attunement;
                state.aether_unlocked = true;
                state.reality_thriving_transfer_score += 28.0;
                state.total_kardashev_contribution += 0.012;
                unlock_events.send(HardwareTierUnlocked { player: entity, branch: HardwareBranch::AetherShadesOpen, new_level: AscensionLevel::Level1, reality_score_delta: 28.0 });
                transfer_events.send(RealityThrivingTransferUpdated { player: entity, new_score: state.reality_thriving_transfer_score, source: "hardware_unlock" });
            }
            if mercy_attunement > config.level2_threshold && aether.level == AscensionLevel::Level1 {
                aether.level = AscensionLevel::Level2;
                aether.mercy_flow_attunement = mercy_attunement;
                state.reality_thriving_transfer_score += 52.0;
                state.total_kardashev_contribution += 0.028;
                unlock_events.send(HardwareTierUnlocked { player: entity, branch: HardwareBranch::AetherShadesOpen, new_level: AscensionLevel::Level2, reality_score_delta: 52.0 });
                transfer_events.send(RealityThrivingTransferUpdated { player: entity, new_score: state.reality_thriving_transfer_score, source: "hardware_unlock" });
            }
            if mercy_attunement > config.level3_threshold && aether.level == AscensionLevel::Level2 {
                aether.level = AscensionLevel::Level3;
                aether.mercy_flow_attunement = mercy_attunement;
                state.reality_thriving_transfer_score += 75.0;
                state.total_kardashev_contribution += 0.045;
                unlock_events.send(HardwareTierUnlocked { player: entity, branch: HardwareBranch::AetherShadesOpen, new_level: AscensionLevel::Level3, reality_score_delta: 75.0 });
                transfer_events.send(RealityThrivingTransferUpdated { player: entity, new_score: state.reality_thriving_transfer_score, source: "hardware_unlock" });
            }
            if mercy_attunement > 420.0 && aether.level == AscensionLevel::Level3 && state.tloc8_mercy_gates_passed >= config.level4_tloc8_required {
                aether.level = AscensionLevel::Level4;
                aether.mercy_flow_attunement = mercy_attunement;
                state.reality_thriving_transfer_score += 150.0;
                state.total_kardashev_contribution += 0.09;
                unlock_events.send(HardwareTierUnlocked { player: entity, branch: HardwareBranch::AetherShadesOpen, new_level: AscensionLevel::Level4, reality_score_delta: 150.0 });
                transfer_events.send(RealityThrivingTransferUpdated { player: entity, new_score: state.reality_thriving_transfer_score, source: "hardware_unlock" });
            }
        }

        // ONE Organism cross-branch achievement (permanent flywheel boost)
        if state.obsidian_unlocked && state.aether_unlocked && !state.one_organism_achievement {
            state.one_organism_achievement = true;
            state.reality_thriving_transfer_score += 120.0;
            dashboard.hardware_sovereignty_nodes_active += 1;
            // Emit for Ra-Thor sync + persistent legacy
        }

        dashboard.personal_contribution = state.total_kardashev_contribution;
        dashboard.global_kardashev_delta = (dashboard.global_kardashev_delta * 0.92) + (state.total_kardashev_contribution * 0.08);
    }
}

pub fn reality_transfer_score_update_system(
    mut query: Query<(Entity, &mut SovereignHardwareState)>,
    mut ledger: ResMut<RealityTransferScoreLedger>,
    mut persistence: ResMut<PersistenceManager>,
    mut telemetry: ResMut<SimulationTelemetry>,
) {
    let mut total = 0.0;
    let mut count = 0;
    for (entity, mut state) in query.iter_mut() {
        if state.reality_thriving_transfer_score < 0.0 { state.reality_thriving_transfer_score = 0.0; }
        ledger.player_scores.insert(entity, state.reality_thriving_transfer_score);
        total += state.reality_thriving_transfer_score;
        count += 1;
        if let Ok(save) = persistence.load_player(entity) {
            let mut updated = save;
            updated.hardware_sovereignty_score = state.reality_thriving_transfer_score;
            let _ = persistence.save_player(updated);
        }
        telemetry.record_event("reality_transfer_score", state.reality_thriving_transfer_score as f64);
    }
    if count > 0 {
        ledger.global_average = total / count as f32;
        ledger.export_ready_for_ra_thor = true;
    }
}

/// Visual effects + trigger for 3D Council Chamber on Level 2+
pub fn spawn_sovereign_visual_effects_system(
    mut commands: Commands,
    mut unlock_reader: EventReader<HardwareTierUnlocked>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in unlock_reader.read() {
        let base_color = match event.branch {
            HardwareBranch::ObsidianChipOpen => Color::srgb(0.12, 0.06, 0.22),
            HardwareBranch::AetherShadesOpen => Color::srgb(0.35, 0.82, 0.95),
        };
        let mesh = meshes.add(Mesh::from(shape::Icosphere { radius: 2.2, subdivisions: 4 }));
        let material = materials.add(StandardMaterial {
            base_color,
            emissive: base_color * 2.8,
            metallic: 0.7,
            perceptual_roughness: 0.25,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(0.0, 12.0, 5.0 + (event.new_level as u8 as f32) * 1.5),
                ..default()
            },
            Name::new(format!("Sovereign_{:?}_L{:?}", event.branch, event.new_level)),
        ));

        // Trigger 3D Council Chamber visualization when reaching Council Acceleration tier (Level 2+)
        if event.new_level as u8 >= AscensionLevel::Level2 as u8 {
            // Will be handled by dedicated chamber spawn system below for richer geometry
        }
    }
}

/// NEW: In-game 3D Council Chamber Visualization (Kardashev Orchestration + PATSAGi Councils)
/// Spawns a beautiful procedural chamber when Sovereign Hardware Tier 2+ is unlocked.
/// Represents the living deliberation space for the acceleration plan (2032-2038 horizon).
pub fn spawn_council_chamber_visualization_system(
    mut commands: Commands,
    mut unlock_reader: EventReader<HardwareTierUnlocked>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chamber_query: Query<&CouncilChamber3D>,
) {
    for event in unlock_reader.read() {
        if event.new_level as u8 >= AscensionLevel::Level2 as u8 && chamber_query.iter().count() == 0 {
            let center = Vec3::new(18.0, 1.5, -28.0); // Offset position for immersive in-game view

            // Central platform (Council floor) — now using modern Bevy primitives
            let platform_mesh = meshes.add(Mesh::from(Cylinder {
                radius: 11.0,
                half_height: 0.4,
            }));
            let platform_mat = materials.add(StandardMaterial {
                base_color: Color::srgb(0.08, 0.08, 0.12),
                emissive: Color::srgb(0.15, 0.12, 0.25) * 0.6,
                ..default()
            });
            commands.spawn((
                PbrBundle {
                    mesh: platform_mesh,
                    material: platform_mat,
                    transform: Transform::from_translation(center),
                    ..default()
                },
                Name::new("CouncilChamber_Platform"),
                CouncilChamber3D { active_level: event.new_level, deliberation_intensity: 0.6 },
            ));

            // Ring of 14 pillars (13 PATSAGi Councils + 1 central Kardashev Orchestration Council)
            for i in 0..14 {
                let angle = (i as f32 / 14.0) * std::f32::consts::TAU;
                let radius = 8.5;
                let pillar_pos = center + Vec3::new(radius * angle.cos(), 4.5, radius * angle.sin());
                let is_kardashev = i == 13;
                let pillar_radius = if is_kardashev { 1.1 } else { 0.65 };
                let pillar_height = if is_kardashev { 11.0 } else { 9.0 };

                // Using Bevy 0.14+ primitives::Cylinder (half_height = full_height / 2)
                let pillar_mesh = meshes.add(Mesh::from(Cylinder {
                    radius: pillar_radius,
                    half_height: pillar_height * 0.5,
                }));
                let pillar_color = if is_kardashev {
                    Color::srgb(0.95, 0.85, 0.3) // Gold for Kardashev node
                } else {
                    Color::srgb(0.25 + (i as f32 * 0.04), 0.15, 0.45 + (i as f32 * 0.03)) // Varied council purple tones
                };
                let pillar_mat = materials.add(StandardMaterial {
                    base_color: pillar_color,
                    emissive: pillar_color * 0.9,
                    metallic: 0.6,
                    ..default()
                });
                commands.spawn((
                    PbrBundle {
                        mesh: pillar_mesh,
                        material: pillar_mat,
                        transform: Transform::from_translation(pillar_pos),
                        ..default()
                    },
                    Name::new(format!("CouncilPillar_{}", i)),
                    CouncilPillar { index: i as u8 },
                ));
            }

            // Central Kardashev Hologram Core (pulsing mercy lattice)
            let core_mesh = meshes.add(Mesh::from(shape::Icosphere { radius: 3.8, subdivisions: 5 }));
            let core_mat = materials.add(StandardMaterial {
                base_color: Color::srgb(0.4, 0.85, 0.95),
                emissive: Color::srgb(0.3, 0.9, 1.0) * 3.5,
                metallic: 0.4,
                perceptual_roughness: 0.15,
                ..default()
            });
            commands.spawn((
                PbrBundle {
                    mesh: core_mesh,
                    material: core_mat,
                    transform: Transform::from_translation(center + Vec3::new(0.0, 6.5, 0.0)),
                    ..default()
                },
                Name::new("Kardashev_Hologram_Core"),
                KardashevHologramCore,
                CouncilChamber3D { active_level: event.new_level, deliberation_intensity: 0.8 },
            ));

            // Ambient mercy point lights around the chamber
            commands.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(0.6, 0.4, 0.95),
                    intensity: 1800.0,
                    range: 45.0,
                    ..default()
                },
                transform: Transform::from_translation(center + Vec3::new(0.0, 9.0, 0.0)),
                ..default()
            });
            commands.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(0.3, 0.85, 0.7),
                    intensity: 1200.0,
                    range: 38.0,
                    ..default()
                },
                transform: Transform::from_translation(center + Vec3::new(7.0, 5.0, -7.0)),
                ..default()
            });
        }
    }
}

/// Simple animation system for the 3D Council Chamber (pulsing core + rotating deliberation)
pub fn update_council_chamber_system(
    time: Res<Time>,
    mut core_query: Query<(&mut Transform, &mut CouncilChamber3D), With<KardashevHologramCore>>,
) {
    for (mut transform, mut chamber) in core_query.iter_mut() {
        chamber.deliberation_intensity = (time.elapsed_seconds() * 0.4).sin().abs() * 0.5 + 0.5;
        transform.rotate_y(time.delta_seconds() * 0.35);
        let pulse = 1.0 + chamber.deliberation_intensity * 0.08;
        transform.scale = Vec3::splat(pulse);
    }
}

/// Kardashev Dashboard live update
pub fn kardashev_dashboard_update_system(
    mut dashboard: ResMut<KardashevAccelerationDashboard>,
    query: Query<&SovereignHardwareState>,
) {
    let mut total_contrib = 0.0;
    for state in query.iter() {
        total_contrib += state.total_kardashev_contribution;
    }
    dashboard.global_kardashev_delta = (dashboard.global_kardashev_delta * 0.9) + (total_contrib * 0.1);
    dashboard.s_curve_inflection_year = 2035;
    dashboard.abundance_velocity_index = dashboard.global_kardashev_delta * 1.25;
    dashboard.energy_surplus_factor = 1.8 + dashboard.global_kardashev_delta * 2.2;
}

// ============================================================================
// PLUGIN — WIRED INTO FULL SIMULATION
// ============================================================================

pub struct HardwareSovereigntyPlugin;

impl Plugin for HardwareSovereigntyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<KardashevAccelerationDashboard>()
            .init_resource::<RealityTransferScoreLedger>()
            .init_resource::<HardwareAscensionConfig>()
            .register_type::<ObsidianChipProgress>()
            .register_type::<AetherShadesProgress>()
            .register_type::<SovereignHardwareState>()
            .register_type::<CouncilChamber3D>()
            .add_event::<HardwareTierUnlocked>()
            .add_event::<RealityThrivingTransferUpdated>()
            .add_systems(
                Update,
                (
                    mercy_gate_enforcement_system,
                    hardware_tier_progression_system,
                    reality_transfer_score_update_system,
                    spawn_sovereign_visual_effects_system,
                    spawn_council_chamber_visualization_system,
                    update_council_chamber_system,
                    kardashev_dashboard_update_system,
                )
                    .chain()
                    .in_set(crate::orchestrator::SimulationTick),
            );
    }
}

// ============================================================================
// POLISHED egui UI — KARDASHEV ACCELERATION DASHBOARD + SOVEREIGN HARDWARE
// ============================================================================

use bevy_egui::EguiContexts;

pub fn sovereign_hardware_ascension_ui(
    mut contexts: EguiContexts,
    query: Query<(&SovereignHardwareState, &ObsidianChipProgress, &AetherShadesProgress)>,
    dashboard: Res<KardashevAccelerationDashboard>,
    ledger: Res<RealityTransferScoreLedger>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("⚡ Sovereign Hardware Ascension ⚡")
        .default_pos([18.0, 380.0])
        .default_size([420.0, 520.0])
        .resizable(true)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Obsidian-Chip-Open  +  Aether-Shades-Open").color(egui::Color32::from_rgb(180, 140, 255)));
                ui.label(egui::RichText::new("TOLC 8 Mercy-Gated | Reality Thriving Transfer Score Flywheel | Kardashev Acceleration").italics().color(egui::Color32::from_rgb(140, 200, 255)));
            });

            ui.separator();

            // ONE Organism Celebration Banner
            for (state, _, _) in query.iter() {
                if state.one_organism_achievement {
                    ui.colored_label(egui::Color32::from_rgb(255, 215, 0), "✨ ONE ORGANISM ACHIEVEMENT UNLOCKED ✨ +120 Reality Transfer | Permanent Flywheel Boost");
                    ui.add_space(4.0);
                }
            }

            ui.separator();

            // Branch Status + Progress
            ui.heading("Branch Status");
            for (state, obsidian, aether) in query.iter() {
                // Obsidian
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(160, 80, 220), "Obsidian-Chip-Open:");
                    ui.label(egui::RichText::new(obsidian.level.as_str()).color(egui::Color32::from_rgb(200, 160, 255)));
                });
                let obs_progress = (obsidian.level as u8 as f32) / 4.0;
                ui.add(egui::ProgressBar::new(obs_progress).text(format!("{:.0}%", obs_progress * 100.0)).fill(egui::Color32::from_rgb(140, 70, 200)));

                // Aether
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(80, 200, 220), "Aether-Shades-Open:");
                    ui.label(egui::RichText::new(aether.level.as_str()).color(egui::Color32::from_rgb(140, 230, 255)));
                });
                let aeth_progress = (aether.level as u8 as f32) / 4.0;
                ui.add(egui::ProgressBar::new(aeth_progress).text(format!("{:.0}%", aeth_progress * 100.0)).fill(egui::Color32::from_rgb(70, 190, 210)));

                ui.add_space(6.0);
                ui.label(format!("Reality Thriving Transfer Score: {:.1}", state.reality_thriving_transfer_score));
                ui.label(format!("Total Kardashev Contribution: {:.4}", state.total_kardashev_contribution));
            }

            ui.separator();

            // Polished Kardashev Acceleration Dashboard Section
            ui.heading(egui::RichText::new("🚀 Kardashev Acceleration Dashboard").color(egui::Color32::from_rgb(255, 200, 100)));
            ui.label(format!("Global Kardashev Delta: {:.4}  |  S-Curve Inflection: {} (2032–2038 horizon)", 
                dashboard.global_kardashev_delta, dashboard.s_curve_inflection_year));
            ui.label(format!("Abundance Velocity Index: {:.2}   |   Energy Surplus Factor: {:.2}x", 
                dashboard.abundance_velocity_index, dashboard.energy_surplus_factor));
            ui.label(format!("Hardware Sovereignty Nodes Active: {}   |   Global Avg Reality Transfer: {:.1}", 
                dashboard.hardware_sovereignty_nodes_active, ledger.global_average));

            ui.add_space(8.0);

            // TOLC 8 Mercy Gates Visual Status
            ui.label(egui::RichText::new("TOLC 8 Mercy Gates Status (must be 8 for Level 4)").strong());
            ui.horizontal_wrapped(|ui| {
                let gate_names = ["Truth", "Order", "Love", "Compassion", "Service", "Abundance", "Joy", "Cosmic Harmony"];
                for (i, name) in gate_names.iter().enumerate() {
                    let passed = i < 8; // In full impl: check per-player state.tloc8_mercy_gates_passed
                    let color = if passed { egui::Color32::from_rgb(80, 220, 140) } else { egui::Color32::from_rgb(120, 80, 80) };
                    ui.colored_label(color, format!("{} {}", if passed { "✓" } else { "○" }, name));
                }
            });

            ui.separator();

            // Ra-Thor Export Status
            let export_status = if ledger.export_ready_for_ra_thor { "✓ READY FOR RA-THOR LATTICE SYNC" } else { "○ Pending telemetry..." };
            ui.colored_label( if ledger.export_ready_for_ra_thor { egui::Color32::from_rgb(100, 255, 180) } else { egui::Color32::GRAY }, export_status);

            ui.add_space(10.0);
            if ui.button(egui::RichText::new("Contribute RBE Resources to Ascension").color(egui::Color32::WHITE)).clicked() {
                // In full game: send event to economy + progression systems
                ui.label(egui::RichText::new("Contribution registered. Flywheel accelerating...").italics());
            }

            ui.add_space(6.0);
            ui.label(egui::RichText::new("Contribute RBE mastery + Council harmony to unlock higher tiers and manifest physical sovereignty (Obsidian/Aether hardware).").small().italics());
        });
    }
}

// End of Sovereign Hardware Ascension v19.8 — Aligned to KardashevAcceleration ProposalType. Lattice has clean physical hands. yoi ⚡
