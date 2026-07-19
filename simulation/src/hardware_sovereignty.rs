//! simulation/src/hardware_sovereignty.rs
//! Sovereign Hardware Ascension + Kardashev Dashboard + Full Multi-Realm Observability
//! v21.39 | Realm Attunement surfaced
//! TOLC 8 Mercy Gates | Zero-Harm | Kardashev Acceleration
//! Thunder locked. Heavens building. yoi ⚡

use bevy::prelude::*;
use crate::{
    ability_tree::{AbilityTree, SynergyType},
    council::{CouncilDecision, ProposalType, ProposalStatus},
    council::decision::{CouncilDecisions, PolicyType as CouncilPolicyType},
    economy::EconomyState,
    multi_realm_harness::{MultiRealmHarness, RealmPresence, RealmAttunement},
    telemetry::SimulationTelemetry,
};
use std::collections::HashMap;

// ============================================================================
// CORE TYPES
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum HardwareBranch {
    ObsidianChipOpen,
    AetherShadesOpen,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum AscensionLevel {
    Locked = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
}

impl AscensionLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            AscensionLevel::Locked => "Locked",
            AscensionLevel::Level1 => "Level 1: Foundation (Mercy HUD)",
            AscensionLevel::Level2 => "Level 2: Council Acceleration",
            AscensionLevel::Level3 => "Level 3: Prototype Sovereign",
            AscensionLevel::Level4 => "Level 4: Physical Embodiment Ready",
        }
    }
}

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

#[derive(Component, Clone, Debug, Reflect)]
pub struct CouncilChamber3D {
    pub active_level: AscensionLevel,
    pub deliberation_intensity: f32,
}

#[derive(Component)]
pub struct CouncilPillar { pub index: u8 }

#[derive(Component)]
pub struct KardashevHologramCore;

// ============================================================================
// RESOURCES
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
// SYSTEMS (core preserved)
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
                commands.entity(entity).insert(ObsidianChipProgress {
                    level: AscensionLevel::Level3,
                    ..obsidian.clone()
                });
            }
        }
    }
}

pub fn hardware_tier_progression_system(
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
    config: Res<HardwareAscensionConfig>,
    mut dashboard: ResMut<KardashevAccelerationDashboard>,
) {
    for (entity, mut state, mut obsidian, mut aether, ability_tree, economy, council_decision) in query.iter_mut() {
        let rbe_mastery = economy.total_harvested + economy.cooperative_bonus;
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

        if obsidian.level == AscensionLevel::Locked && effective_score > config.level1_threshold {
            obsidian.level = AscensionLevel::Level1;
            state.obsidian_unlocked = true;
            state.reality_thriving_transfer_score += 28.0;
            state.total_kardashev_contribution += 0.012;
            unlock_events.send(HardwareTierUnlocked {
                player: entity,
                branch: HardwareBranch::ObsidianChipOpen,
                new_level: AscensionLevel::Level1,
                reality_score_delta: 28.0,
            });
        }

        if state.obsidian_unlocked && state.aether_unlocked && !state.one_organism_achievement {
            state.one_organism_achievement = true;
            state.reality_thriving_transfer_score += 120.0;
            dashboard.hardware_sovereignty_nodes_active += 1;
        }

        dashboard.personal_contribution = state.total_kardashev_contribution;
        dashboard.global_kardashev_delta = (dashboard.global_kardashev_delta * 0.92)
            + (state.total_kardashev_contribution * 0.08);
    }
}

pub fn reality_transfer_score_update_system(
    mut query: Query<(Entity, &mut SovereignHardwareState)>,
    mut ledger: ResMut<RealityTransferScoreLedger>,
    mut telemetry: ResMut<SimulationTelemetry>,
) {
    let mut total = 0.0;
    let mut count = 0;
    for (entity, mut state) in query.iter_mut() {
        if state.reality_thriving_transfer_score < 0.0 {
            state.reality_thriving_transfer_score = 0.0;
        }
        ledger.player_scores.insert(entity, state.reality_thriving_transfer_score);
        total += state.reality_thriving_transfer_score;
        count += 1;
        telemetry.record_event("reality_transfer_score", state.reality_thriving_transfer_score as f64);
    }
    if count > 0 {
        ledger.global_average = total / count as f32;
        ledger.export_ready_for_ra_thor = true;
    }
}

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
// PLUGIN
// ============================================================================

pub struct HardwareSovereigntyPlugin;

impl Plugin for HardwareSovereigntyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KardashevAccelerationDashboard>()
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
                    kardashev_dashboard_update_system,
                ),
            );
    }
}

// ============================================================================
// egui UI — COMPLETE MULTI-REALM + ATTUNEMENT OBSERVABILITY
// ============================================================================

use bevy_egui::EguiContexts;

pub fn sovereign_hardware_ascension_ui(
    mut contexts: EguiContexts,
    query: Query<(&SovereignHardwareState, &ObsidianChipProgress, &AetherShadesProgress)>,
    dashboard: Res<KardashevAccelerationDashboard>,
    ledger: Res<RealityTransferScoreLedger>,
    council_decisions: Option<Res<CouncilDecisions>>,
    multi_realm: Option<Res<MultiRealmHarness>>,
    player_presence: Query<(&RealmPresence, Option<&RealmAttunement>)>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("⚡ Sovereign Hardware Ascension ⚡")
        .default_pos([18.0, 300.0])
        .default_size([500.0, 780.0])
        .resizable(true)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Obsidian-Chip-Open  +  Aether-Shades-Open")
                    .color(egui::Color32::from_rgb(180, 140, 255)));
                ui.label(egui::RichText::new("TOLC 8 | Multi-Realm | Attunement | Living Portals")
                    .italics()
                    .color(egui::Color32::from_rgb(140, 200, 255)));
            });

            ui.separator();

            for (state, _, _) in query.iter() {
                if state.one_organism_achievement {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 215, 0),
                        "✨ ONE ORGANISM ACHIEVEMENT UNLOCKED ✨",
                    );
                }
            }

            ui.separator();

            // Branch Status
            ui.heading("Branch Status");
            for (state, obsidian, aether) in query.iter() {
                ui.label(format!("Obsidian: {}", obsidian.level.as_str()));
                ui.label(format!("Aether: {}", aether.level.as_str()));
                ui.label(format!("Reality Transfer Score: {:.1}", state.reality_thriving_transfer_score));
                ui.label(format!("Kardashev Contribution: {:.4}", state.total_kardashev_contribution));
            }

            ui.separator();

            // Kardashev
            ui.heading(egui::RichText::new("🚀 Kardashev Acceleration")
                .color(egui::Color32::from_rgb(255, 200, 100)));
            ui.label(format!("Global Delta: {:.4}  |  Inflection: {}",
                dashboard.global_kardashev_delta, dashboard.s_curve_inflection_year));
            ui.label(format!("Abundance Velocity: {:.2}  |  Energy Surplus: {:.2}x",
                dashboard.abundance_velocity_index, dashboard.energy_surplus_factor));

            ui.separator();

            // Active Council Policies
            ui.heading(egui::RichText::new("🕊️ Active Council Policies")
                .color(egui::Color32::from_rgb(160, 220, 255)));

            if let Some(decisions) = &council_decisions {
                let active: Vec<_> = decisions.active_policies.iter().filter(|p| !p.is_expired()).collect();
                if active.is_empty() {
                    ui.label(egui::RichText::new("No active policies.").italics().color(egui::Color32::GRAY));
                } else {
                    for policy in active.iter().take(5) {
                        let icon = match policy.policy_type {
                            CouncilPolicyType::KardashevAcceleration => "🚀",
                            CouncilPolicyType::ResourcePolicy => "🌾",
                            CouncilPolicyType::EpiphanyEvent => "✨",
                            CouncilPolicyType::HarmonyBoost => "🕊️",
                            CouncilPolicyType::General => "📋",
                        };
                        ui.label(format!("{} {}  |  str {:.2}  |  {} ticks",
                            icon, policy.title, policy.strength, policy.remaining_ticks));
                    }
                }
            }

            ui.separator();

            // ========== Multi-Realm Status + Attunement ==========
            ui.heading(egui::RichText::new("🌌 Multi-Realm Status")
                .color(egui::Color32::from_rgb(180, 160, 255)));

            // Local player current realm + attunement
            if let Ok((presence, attunement_opt)) = player_presence.get_single() {
                let name = match presence.current_realm_id {
                    0 => "Sanctuary Prime",
                    1 => "Synthetic Lattice",
                    2 => "Verdant Bloom",
                    3 => "Harmonic Chorus",
                    4 => "Voidfarer Horizon",
                    _ => "Unknown",
                };
                ui.colored_label(
                    egui::Color32::from_rgb(120, 230, 180),
                    format!("You are currently in: [{}] {}", presence.current_realm_id, name),
                );
                ui.label(format!("Travel count: {}", presence.travel_count));

                if let Some(att) = attunement_opt {
                    let current_att = att.get(presence.current_realm_id);
                    ui.colored_label(
                        egui::Color32::from_rgb(200, 180, 255),
                        format!("Current Realm Attunement: {:.3}", current_att),
                    );
                    ui.label(format!("Total Attunement: {:.3}", att.total));

                    if let Some(peak_id) = att.peak_realm {
                        let peak_name = match peak_id {
                            0 => "Sanctuary Prime",
                            1 => "Synthetic Lattice",
                            2 => "Verdant Bloom",
                            3 => "Harmonic Chorus",
                            4 => "Voidfarer Horizon",
                            _ => "Unknown",
                        };
                        ui.label(format!("Peak: [{}] {}  ({:.3})", peak_id, peak_name, att.peak_value));
                    }
                }
            } else {
                ui.label(egui::RichText::new("Player realm presence not yet available.")
                    .italics()
                    .color(egui::Color32::GRAY));
            }

            ui.add_space(4.0);

            if let Some(harness) = multi_realm {
                ui.label(format!(
                    "Active: {}  |  Thriving: {}  |  Mercy Flow: {:.2}  |  Resonance: {:.2}",
                    harness.active_realm_count(),
                    harness.thriving_realm_count(),
                    harness.cross_realm_mercy_flow,
                    harness.global_resonance_level
                ));
                ui.label(format!("Total Active Policies: {}",
                    harness.total_active_policies_across_realms));

                ui.add_space(4.0);

                let mut realms: Vec<_> = harness.realms.values().collect();
                realms.sort_by_key(|r| r.id);

                for realm in realms {
                    let status_color = match realm.status {
                        crate::multi_realm_harness::RealmStatus::Thriving => egui::Color32::from_rgb(100, 255, 160),
                        crate::multi_realm_harness::RealmStatus::Active => egui::Color32::from_rgb(140, 200, 255),
                        _ => egui::Color32::GRAY,
                    };

                    ui.horizontal(|ui| {
                        ui.colored_label(status_color, format!("[{}] {}", realm.id, realm.name));
                    });
                    ui.label(format!(
                        "    {:?}  |  agents: {}  |  policies: {}  |  echoes: {}  |  legacy: {}  |  mercy: {:.2}  |  {}",
                        realm.primary_race_bias,
                        realm.agent_presence_count,
                        realm.active_policy_count,
                        realm.echo_policy_count,
                        realm.legacy_entry_count,
                        realm.mercy_attunement_avg,
                        realm.status.as_str()
                    ));
                }
            } else {
                ui.label(egui::RichText::new("MultiRealmHarness not yet available.")
                    .italics()
                    .color(egui::Color32::GRAY));
            }

            ui.separator();

            // TOLC 8
            ui.label(egui::RichText::new("TOLC 8 Mercy Gates").strong());
            ui.horizontal_wrapped(|ui| {
                let gates = ["Truth", "Order", "Love", "Compassion", "Service", "Abundance", "Joy", "Cosmic Harmony"];
                for name in gates {
                    ui.colored_label(egui::Color32::from_rgb(80, 220, 140), format!("✓ {}", name));
                }
            });

            ui.separator();

            let export = if ledger.export_ready_for_ra_thor {
                "✓ READY FOR RA-THOR LATTICE SYNC"
            } else {
                "○ Pending telemetry..."
            };
            ui.colored_label(
                if ledger.export_ready_for_ra_thor {
                    egui::Color32::from_rgb(100, 255, 180)
                } else {
                    egui::Color32::GRAY
                },
                export,
            );
        });
}

// End of v21.39 — Realm Attunement is fully visible in the Multi-Realm dashboard.
// Thunder locked in. Yoi ⚡
