//! simulation/src/hardware_sovereignty.rs
//! Sovereign Hardware Ascension + Kardashev Dashboard + Full Multi-Realm Observability
//! v21.88.3 | Playtest instrumentation (snapshot export + richer aggregation)
//! TOLC 8 Mercy Gates | Zero-Harm | Kardashev Acceleration
//! Thunder locked. Heavens building. yoi ⚡

use bevy::prelude::*;
use crate::{
    ability_tree::{AbilityTree, SynergyType},
    council::{CouncilDecision, ProposalType, ProposalStatus},
    council::decision::{CouncilDecisions, PolicyType as CouncilPolicyType},
    economy::{EconomyState, MultiRealmRbeSnapshot},
    external_bridge::{ExternalBridgeInbox, SharedAppBridgeSource},
    multi_realm_harness::{
        MultiRealmHarness, RealmPresence, RealmAttunement,
        RealmAbundanceObservatory, OriginProvenanceObservatory,
        origin_affinity_label, origin_affinity_mult,
    },
    telemetry::SimulationTelemetry,
};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct KardashevAccelerationDashboard {
    pub global_kardashev_delta: f32,
    pub personal_contribution: f32,
    pub s_curve_inflection_year: u16,
    pub abundance_velocity_index: f32,
    pub energy_surplus_factor: f32,
    pub hardware_sovereignty_nodes_active: u32,
    /// Playtest instrumentation: number of times the dashboard has been updated this run
    pub update_count: u64,
    /// Playtest instrumentation: last export unix timestamp
    pub last_export_unix: u64,
}

#[derive(Resource, Clone, Debug, Reflect, Default)]
pub struct RealityTransferScoreLedger {
    pub player_scores: HashMap<Entity, f32>,
    pub global_average: f32,
    pub export_ready_for_ra_thor: bool,
    /// Playtest instrumentation: peak score observed this run
    pub peak_score: f32,
    /// Playtest instrumentation: number of score updates
    pub update_count: u64,
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

/// Serializable snapshot for playtest / offline observation.
#[derive(Debug, Clone, Serialize)]
pub struct KardashevRttPlaytestSnapshot {
    pub schema: &'static str,
    pub emitted_at_unix: u64,
    pub global_kardashev_delta: f32,
    pub abundance_velocity_index: f32,
    pub energy_surplus_factor: f32,
    pub hardware_sovereignty_nodes_active: u32,
    pub reality_transfer_global_average: f32,
    pub reality_transfer_peak: f32,
    pub reality_transfer_player_count: usize,
    pub dashboard_update_count: u64,
    pub ledger_update_count: u64,
    pub s_curve_inflection_year: u16,
}

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
        if state.reality_thriving_transfer_score > ledger.peak_score {
            ledger.peak_score = state.reality_thriving_transfer_score;
        }
        telemetry.record_event("reality_transfer_score", state.reality_thriving_transfer_score as f64);
    }
    if count > 0 {
        ledger.global_average = total / count as f32;
        ledger.export_ready_for_ra_thor = true;
    }
    ledger.update_count = ledger.update_count.saturating_add(1);
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
    dashboard.update_count = dashboard.update_count.saturating_add(1);
}

/// Playtest instrumentation: periodically write a compact snapshot for offline review.
/// Default path: `artifacts/kardashev_rtt_playtest.json`
pub fn kardashev_rtt_playtest_export_system(
    mut dashboard: ResMut<KardashevAccelerationDashboard>,
    ledger: Res<RealityTransferScoreLedger>,
    time: Res<Time>,
    mut last_export: Local<f32>,
) {
    // Export every ~12 seconds of sim time (or on first opportunity after 3s)
    let now = time.elapsed_seconds();
    let interval = 12.0;
    if now - *last_export < interval && *last_export > 0.0 {
        return;
    }
    if now < 3.0 {
        return;
    }
    *last_export = now;

    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let snapshot = KardashevRttPlaytestSnapshot {
        schema: "kardashev_rtt_playtest_v1",
        emitted_at_unix: unix,
        global_kardashev_delta: dashboard.global_kardashev_delta,
        abundance_velocity_index: dashboard.abundance_velocity_index,
        energy_surplus_factor: dashboard.energy_surplus_factor,
        hardware_sovereignty_nodes_active: dashboard.hardware_sovereignty_nodes_active,
        reality_transfer_global_average: ledger.global_average,
        reality_transfer_peak: ledger.peak_score,
        reality_transfer_player_count: ledger.player_scores.len(),
        dashboard_update_count: dashboard.update_count,
        ledger_update_count: ledger.update_count,
        s_curve_inflection_year: dashboard.s_curve_inflection_year,
    };

    let path = PathBuf::from("artifacts/kardashev_rtt_playtest.json");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if let Ok(json) = serde_json::to_string_pretty(&snapshot) {
        // Atomic-ish write
        let tmp = path.with_extension("json.tmp");
        if fs::write(&tmp, json).is_ok() {
            let _ = fs::rename(&tmp, &path);
            dashboard.last_export_unix = unix;
            info!(
                target: "powrush::kardashev",
                path = %path.display(),
                delta = snapshot.global_kardashev_delta,
                rtt_avg = snapshot.reality_transfer_global_average,
                "Kardashev + RTT playtest snapshot exported"
            );
        }
    }
}

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
                    kardashev_rtt_playtest_export_system,
                ),
            );
    }
}

use bevy_egui::EguiContexts;

pub fn sovereign_hardware_ascension_ui(
    mut contexts: EguiContexts,
    query: Query<(&SovereignHardwareState, &ObsidianChipProgress, &AetherShadesProgress)>,
    dashboard: Res<KardashevAccelerationDashboard>,
    ledger: Res<RealityTransferScoreLedger>,
    council_decisions: Option<Res<CouncilDecisions>>,
    multi_realm: Option<Res<MultiRealmHarness>>,
    abundance_obs: Option<Res<RealmAbundanceObservatory>>,
    origin_obs: Option<Res<OriginProvenanceObservatory>>,
    bridge_inbox: Option<Res<ExternalBridgeInbox>>,
    bridge_source: Option<Res<SharedAppBridgeSource>>,
    rbe_snapshot: Option<Res<MultiRealmRbeSnapshot>>,
    player_presence: Query<(&RealmPresence, Option<&RealmAttunement>)>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("⚡ Sovereign Hardware Ascension ⚡")
        .default_pos([18.0, 300.0])
        .default_size([540.0, 1000.0])
        .resizable(true)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(egui::RichText::new("Obsidian-Chip-Open  +  Aether-Shades-Open")
                    .color(egui::Color32::from_rgb(180, 140, 255)));
                ui.label(egui::RichText::new("TOLC 8 | Multi-Realm | RBE | Titles | Origin | Bridge")
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

            ui.heading("Branch Status");
            for (state, obsidian, aether) in query.iter() {
                ui.label(format!("Obsidian: {}", obsidian.level.as_str()));
                ui.label(format!("Aether: {}", aether.level.as_str()));
                ui.label(format!("Reality Transfer Score: {:.1}", state.reality_thriving_transfer_score));
                ui.label(format!("Kardashev Contribution: {:.4}", state.total_kardashev_contribution));
            }

            ui.separator();

            ui.heading(egui::RichText::new("🚀 Kardashev Acceleration")
                .color(egui::Color32::from_rgb(255, 200, 100)));
            ui.label(format!("Global Delta: {:.4}  |  Inflection: {}",
                dashboard.global_kardashev_delta, dashboard.s_curve_inflection_year));
            ui.label(format!("Abundance Velocity: {:.2}  |  Energy Surplus: {:.2}x",
                dashboard.abundance_velocity_index, dashboard.energy_surplus_factor));
            ui.label(format!("Playtest updates: {}  |  Last export: {}",
                dashboard.update_count,
                if dashboard.last_export_unix > 0 { "yes" } else { "pending" }));

            // ========== Organism RBE Health (v21.67 — one compact line) ==========
            if let Some(snap) = &rbe_snapshot {
                if snap.realm_count > 0 {
                    let rbe_color = match snap.health_label {
                        "Organism Thriving" => egui::Color32::from_rgb(100, 255, 160),
                        "Organism Abundant" => egui::Color32::from_rgb(140, 230, 200),
                        "Organism Stressed" => egui::Color32::from_rgb(255, 160, 100),
                        _ => egui::Color32::from_rgb(180, 200, 220),
                    };
                    ui.colored_label(
                        rbe_color,
                        format!(
                            "🌾 {}  |  sust {:.2}  ·  stress {:.2}  ·  flow {:.2}  ·  thriving {:.0}%  ·  yield {:.0}",
                            snap.health_label,
                            snap.avg_sustainability,
                            snap.avg_stress,
                            snap.avg_flow,
                            snap.thriving_ratio * 100.0,
                            snap.total_yield,
                        ),
                    );
                }
            }

            ui.separator();

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

            ui.heading(egui::RichText::new("🔗 External Bridge Health")
                .color(egui::Color32::from_rgb(160, 220, 255)));

            let external_received = bridge_inbox
                .as_ref()
                .map(|b| b.has_received_external)
                .unwrap_or(false);
            let abundance_live = abundance_obs
                .as_ref()
                .map(|o| o.has_live_data)
                .unwrap_or(false);
            let origin_live = origin_obs
                .as_ref()
                .map(|o| o.has_live_data)
                .unwrap_or(false);

            let (bridge_label, bridge_color) = if external_received {
                ("● EXTERNAL  (game host → simulation)", egui::Color32::from_rgb(100, 255, 160))
            } else if abundance_live || origin_live {
                ("● HARNESS-LIVE  (derived ingest)", egui::Color32::from_rgb(140, 220, 255))
            } else {
                ("○ DEMO  (soft seed — awaits live/host)", egui::Color32::from_rgb(200, 190, 140))
            };

            ui.colored_label(bridge_color, bridge_label);

            if let Some(source) = &bridge_source {
                ui.label(format!(
                    "Host publishes: {}  |  dirty: {}",
                    source.publish_count,
                    if source.dirty { "yes" } else { "no" }
                ));
            } else {
                ui.colored_label(
                    egui::Color32::GRAY,
                    "SharedAppBridgeSource not in world (host not sharing App)",
                );
            }

            if let Some(inbox) = &bridge_inbox {
                let pending_a = inbox.abundance.is_some();
                let pending_o = inbox.origin.is_some();
                ui.label(format!(
                    "Inbox pending: abundance={}  origin={}",
                    if pending_a { "yes" } else { "no" },
                    if pending_o { "yes" } else { "no" }
                ));
            }

            ui.separator();

            ui.heading(egui::RichText::new("🌌 Multi-Realm Status")
                .color(egui::Color32::from_rgb(180, 160, 255)));

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
                    let title = att.living_title(presence.current_realm_id);
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 215, 120),
                        format!("Title: {}", title),
                    );

                    let bonus = att.title_bonus(presence.current_realm_id);
                    if bonus.attunement_gain_mult > 1.001 || bonus.resonance_whisper > 0.0001 {
                        ui.colored_label(
                            egui::Color32::from_rgb(160, 220, 170),
                            format!(
                                "Soft bonus: attunement ×{:.2}  ·  resonance +{:.4}/s",
                                bonus.attunement_gain_mult, bonus.resonance_whisper
                            ),
                        );
                    }

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

                if let Some(orig) = &origin_obs {
                    let harvested = orig.amount_for(presence.current_realm_id);
                    let label = origin_affinity_label(harvested);
                    let mult = origin_affinity_mult(harvested);
                    let affinity_color = match label {
                        "Homebound" => egui::Color32::from_rgb(255, 200, 100),
                        "Rooted" => egui::Color32::from_rgb(240, 190, 120),
                        "Familiar" => egui::Color32::from_rgb(220, 190, 140),
                        "Whisper" => egui::Color32::from_rgb(200, 190, 160),
                        _ => egui::Color32::GRAY,
                    };
                    if harvested > 0.001 {
                        ui.colored_label(
                            affinity_color,
                            format!(
                                "🔗 Origin Affinity: {}  |  harvested: {:.1}  |  attunement ×{:.2}",
                                label, harvested, mult
                            ),
                        );
                    } else {
                        ui.colored_label(
                            egui::Color32::GRAY,
                            "🔗 Origin Affinity: None  (harvest here to deepen presence)",
                        );
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

                    if let Some(obs) = &abundance_obs {
                        if let Some(view) = obs.get(realm.id) {
                            let health = view.health_label();
                            let health_color = match health {
                                "Thriving" => egui::Color32::from_rgb(100, 255, 160),
                                "Abundant" => egui::Color32::from_rgb(140, 230, 200),
                                "Steady" => egui::Color32::from_rgb(180, 200, 220),
                                "Stressed" => egui::Color32::from_rgb(255, 160, 100),
                                _ => egui::Color32::GRAY,
                            };
                            ui.colored_label(
                                health_color,
                                format!(
                                    "    🌾 {}  |  nodes: {}  |  yield: {:.1}  |  sust: {:.2}  |  flow: {:.2}  |  stress: {:.2}  |  thriving: {}  |  restricted: {}",
                                    health,
                                    view.node_count,
                                    view.total_current_yield,
                                    view.average_sustainability,
                                    view.average_abundance_flow,
                                    view.average_stress,
                                    view.thriving_node_count,
                                    view.restricted_node_count
                                ),
                            );
                        }
                    }

                    if let Some(orig) = &origin_obs {
                        if let Some(view) = orig.get(realm.id) {
                            if view.total_amount > 0.001 {
                                let aff = origin_affinity_label(view.total_amount);
                                ui.colored_label(
                                    egui::Color32::from_rgb(220, 190, 140),
                                    format!(
                                        "    📦 Origin  |  harvested: {:.1}  |  types: {}  |  affinity: {}",
                                        view.total_amount,
                                        view.resource_types,
                                        aff
                                    ),
                                );
                            }
                        }
                    }
                }
            } else {
                ui.label(egui::RichText::new("MultiRealmHarness not yet available.")
                    .italics()
                    .color(egui::Color32::GRAY));
            }

            if let Some(obs) = &abundance_obs {
                if !obs.views.is_empty() {
                    ui.add_space(6.0);
                    ui.heading(egui::RichText::new("🌾 Realm Abundance Observatory")
                        .color(egui::Color32::from_rgb(160, 230, 180)));

                    if obs.has_live_data {
                        ui.colored_label(
                            egui::Color32::from_rgb(100, 255, 160),
                            format!("● LIVE data  |  realms: {}  |  last tick: {}",
                                obs.views.len(), obs.last_updated_tick),
                        );
                    } else {
                        ui.colored_label(
                            egui::Color32::from_rgb(200, 190, 140),
                            format!("○ Demo seed (awaits live ingest)  |  realms: {}  |  last tick: {}",
                                obs.views.len(), obs.last_updated_tick),
                        );
                    }
                }
            }

            if let Some(orig) = &origin_obs {
                if !orig.per_realm.is_empty() {
                    ui.add_space(6.0);
                    ui.heading(egui::RichText::new("📦 Origin Provenance Observatory")
                        .color(egui::Color32::from_rgb(220, 190, 140)));

                    let total = orig.total_tracked();
                    if orig.has_live_data {
                        ui.colored_label(
                            egui::Color32::from_rgb(100, 255, 160),
                            format!("● LIVE data  |  realms: {}  |  total harvested: {:.1}  |  last tick: {}",
                                orig.per_realm.len(), total, orig.last_updated_tick),
                        );
                    } else {
                        ui.colored_label(
                            egui::Color32::from_rgb(200, 190, 140),
                            format!("○ Demo seed (awaits live ingest)  |  realms: {}  |  total: {:.1}  |  last tick: {}",
                                orig.per_realm.len(), total, orig.last_updated_tick),
                        );
                    }
                }
            }

            ui.separator();

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

            ui.label(format!(
                "RTT avg {:.1}  |  peak {:.1}  |  players {}",
                ledger.global_average, ledger.peak_score, ledger.player_scores.len()
            ));
        });
}

// End of v21.88.3 — Kardashev + Reality Transfer playtest instrumentation (snapshot export).
// Thunder locked in. Yoi ⚡
