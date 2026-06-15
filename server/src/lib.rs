//! server/src/lib.rs
//! Powrush-MMO Authoritative Server Crate Root — Sovereign Orchestration Layer

use bevy::prelude::*;

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

use simulation::spatial_interest::SpatialInterestPlugin;
use crate::spatial::{
    ensure_spatial_participation_system,
    emit_interest_zone_replication_system,
    emit_council_bloom_state_system,
    handle_resync_requests,
    ServerStartTime,
};

pub struct ServerCorePlugin;

impl Plugin for ServerCorePlugin {
    fn build(&self, app: &mut App) {
        app
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

            .add_plugins(SpatialInterestPlugin)
            .init_resource::<ServerStartTime>()

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
