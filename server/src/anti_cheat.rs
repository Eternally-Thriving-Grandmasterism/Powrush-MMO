/*!
 * Powrush-MMO Server — Anti-Cheat Systems
 */

use bevy::prelude::*;
use simulation::bot_detection::{BotDetectionConfig, ServerRateLimiter, BotSuspicion, validate_harvest_action, validate_epiphany_trigger};
use simulation::epiphany_catalyst::{EpiphanyContext, EpiphanyTelemetryEvent};

pub struct ServerAntiCheatPlugin;

impl Plugin for ServerAntiCheatPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ServerRateLimiter>()
            .init_resource::<BotSuspicion>()
            .add_systems(Update, (
                process_incoming_telemetry,
                enforce_rate_limits,
            ));
    }
}

fn process_incoming_telemetry(
    mut suspicion: ResMut<BotSuspicion>,
    config: Res<BotDetectionConfig>,
    time: Res<Time>,
    // In real implementation: receive telemetry events from clients
) {
    if !config.enabled || !config.telemetry_anomaly_enabled {
        return;
    }

    // Placeholder: In production this would receive EpiphanyTelemetryEvent from clients
    // and call suspicion.add_anomaly_score(...)
}

fn enforce_rate_limits(
    mut rate_limiter: ResMut<ServerRateLimiter>,
    config: Res<BotDetectionConfig>,
    // Listen for harvest/epiphany action requests from clients
) {
    if !config.enabled || !config.server_rate_limiting_enabled {
        return;
    }

    // Example: When a harvest action arrives from client
    // if !rate_limiter.check_harvest(player_id, current_time_ms) {
    //     // Reject the action
    // }
}

// Example authoritative validation function (to be called from rbe_harvest_handler)
pub fn server_validate_harvest(
    player_id: u64,
    depletion: f32,
    sustainable_pacing: bool,
    biome: &str,
    rate_limiter: &mut ServerRateLimiter,
    current_ms: u64,
) -> bool {
    if !validate_harvest_action(depletion, sustainable_pacing, biome) {
        return false;
    }

    if !rate_limiter.check_harvest(player_id, current_ms) {
        return false;
    }

    true
}
