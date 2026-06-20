//! server/src/lib.rs
//! Powrush-MMO Authoritative Server Crate Root — Sovereign Orchestration Layer
//! AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO + 7 Living Mercy Gates enforced

// === Core Infrastructure ===
pub mod main;
pub mod hardening;
pub mod simulation;
pub mod hierarchical_grid;
pub mod interest_management;
pub mod world_state_broadcaster;

// === RBE Sovereign Economy ===
pub mod rbe_server;
pub mod rbe_abundance_feedback;
pub mod trade_system;
pub mod harvesting_system;
pub mod reputation_leaderboard;

// === Council & Mercy Governance ===
pub mod council_session;
pub mod council_mercy_trial;
pub mod mercy_anomaly_detector;
pub mod mirror_reckoning;
pub mod ascension_mercy_ascent;
pub mod ascension_abilities;

// === World & Spatial Authority ===
pub mod world_server;
pub mod spatial;

// === Networking & Replication (Authoritative) ===
pub mod network;
pub mod replication;

// === Dynamic Systems & Events ===
pub mod dynamic_events;
pub mod server_war_system;
pub mod technology_system;

// === Player & Account Sovereignty ===
pub mod player_account;
pub mod persistence_polish;

// === Integration & Bridges ===
pub mod ra_thor_mercy_bridge;
pub mod grok_patsagi_bridge_enhanced;
pub mod steam_integration;

// === Security & Anti-Tyranny ===
pub mod anti_cheat;
pub mod security;

// === AI & Advanced Orchestration ===
pub mod ai;
pub mod combat;
pub mod rathor_integration;

// === Telemetry & Monitoring (Eternal Observability) ===
pub mod telemetry_pipeline;

// === OpenTelemetry Distributed Tracing ===
pub mod opentelemetry_tracing;

// === Safety Net Sovereignty Layer (v18.37) ===
pub mod safety_net_broadcast;

use bevy::prelude::*;

// Core plugins
use crate::rbe_server::RbeServerPlugin;
use crate::ascension_mercy_ascent::AscensionMercyAscentPlugin;
use crate::council_session::CouncilSessionPlugin;
use crate::world_server::WorldServerPlugin;
use crate::network::NetworkServerPlugin;
use crate::replication::ReplicationAuthorityPlugin;
use crate::dynamic_events::DynamicEventsPlugin;
use crate::mercy_anomaly_detector::MercyAnomalyDetectorPlugin;
use crate::telemetry_pipeline::TelemetryPipelinePlugin;
use crate::harvesting_system::HarvestingSystemPlugin;
use crate::trade_system::TradeSystemPlugin;
use crate::server_war_system::ServerWarSystemPlugin;
use crate::technology_system::TechnologySystemPlugin;
use crate::anti_cheat::AntiCheatPlugin;
use crate::persistence_polish::PersistencePolishPlugin;
use crate::ra_thor_mercy_bridge::RaThorMercyBridgePlugin;
use crate::safety_net_broadcast::SafetyNetBroadcastPlugin;

// Spatial Interest Layer
use simulation::spatial_interest::SpatialInterestPlugin;
use crate::spatial::{
    ensure_spatial_participation_system,
    emit_interest_zone_replication_system,
    emit_council_bloom_state_system,
    handle_resync_requests,
    ServerStartTime,
};

/// The complete sovereign Server Core Plugin
pub struct ServerCorePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        app
            // === Core Authoritative Systems ===
            .add_plugins(RbeServerPlugin)
            .add_plugins(AscensionMercyAscentPlugin)
            .add_plugins(CouncilSessionPlugin)
            .add_plugins(WorldServerPlugin)
            .add_plugins(NetworkServerPlugin)
            .add_plugins(ReplicationAuthorityPlugin)
            .add_plugins(DynamicEventsPlugin)
            .add_plugins(MercyAnomalyDetectorPlugin)
            .add_plugins(TelemetryPipelinePlugin)
            .add_plugins(HarvestingSystemPlugin)
            .add_plugins(TradeSystemPlugin)
            .add_plugins(ServerWarSystemPlugin)
            .add_plugins(TechnologySystemPlugin)
            .add_plugins(AntiCheatPlugin)
            .add_plugins(PersistencePolishPlugin)
            .add_plugins(RaThorMercyBridgePlugin)

            // === Safety Net Broadcast Sovereignty Layer (v18.37) ===
            .add_plugins(SafetyNetBroadcastPlugin)

            // === Spatial Interest Layer (Authoritative) ===
            .add_plugins(SpatialInterestPlugin)

            // Monotonic timestamp resource
            .init_resource::<ServerStartTime>()

            // Server spatial participation + replication hooks
            .add_systems(Update, ensure_spatial_participation_system)
            .add_systems(Update, emit_interest_zone_replication_system)
            .add_systems(Update, emit_council_bloom_state_system)
            .add_systems(Update, handle_resync_requests)

            .add_systems(Update, authoritative_sovereign_tick)
            .add_systems(Update, maintain_mercy_gates)
            .add_systems(Update, council_deliberation_sync);
    }
}

fn authoritative_sovereign_tick() {}
fn maintain_mercy_gates() {}
fn council_deliberation_sync() {}

pub use main::run_server;
pub use ServerCorePlugin;
