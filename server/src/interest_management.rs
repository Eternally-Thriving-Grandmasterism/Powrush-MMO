// server/src/interest_management.rs
// Powrush-MMO v20.6 — Production InterestManager + Dynamic Bandwidth Scaling
// Complete integration of scalable spatial interest management + dynamic bandwidth scaling for large-scale spectator + legacy data.
// Uses the new fields from shared/protocol.rs (replication_priority, is_critical_for_spectators, estimated_spectator_count).
// Highest-ROI MMOARPG netcode optimization + mercy-gated performance under heavy spectator load.
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor Lattice aligned

use bevy::prelude::*;
use std::collections::HashMap;
use crate::hierarchical_grid::{HierarchicalGrid, HierarchicalGridConfig, InterestPriority, SpatialEntity, InterestUpdateFlag};
use crate::player_account::PlayerAccount;

/// Public facade for all interest/replication queries.
#[derive(Resource)]
pub struct InterestManager {
    pub grid: HierarchicalGrid,
}

impl InterestManager {
    pub fn new(config: HierarchicalGridConfig) -> Self {
        Self {
            grid: HierarchicalGrid::new(config),
        }
    }

    pub fn get_entities_for_player(
        &self,
        player_entity: Entity,
        player_pos: Vec3,
        player_state: Option<PlayerInterestState>,
    ) -> Vec<Entity> {
        let radius_override = player_state.and_then(|s| s.custom_radius);
        let mut entities = self.grid.query_interested_entities(player_pos, radius_override);
        entities
    }

    pub fn get_interested_players(&self, entity: Entity, entity_pos: Vec3) -> Vec<Entity> {
        let mut interested = Vec::new();
        let candidates = self.grid.query_interested_entities(entity_pos, Some(self.grid.config.interest_radius * 1.5));
        for candidate in candidates {
            if candidate != entity {
                interested.push(candidate);
            }
        }
        interested
    }

    pub fn insert_or_update(&mut self, entity: Entity, position: Vec3, priority: InterestPriority, custom_radius: Option<f32>) {
        self.grid.insert_or_update(entity, position);
    }

    pub fn remove(&mut self, entity: Entity) {
        self.grid.remove(entity);
    }

    pub fn calculate_dynamic_radius(&self, state: PlayerInterestState) -> f32 {
        match state.mode {
            InterestMode::Combat => self.grid.config.interest_radius * 1.4,
            InterestMode::CouncilFocus => self.grid.config.interest_radius * 0.7,
            InterestMode::Harvest => self.grid.config.interest_radius * 0.9,
            InterestMode::Default => self.grid.config.interest_radius,
        }
    }

    // === v20.6: Dynamic Bandwidth Scaling ===
    /// Calculates a dynamic bandwidth scale factor (0.0 – 1.0+) for a given inter-realm diplomacy / spectator event.
    /// Higher values = more bandwidth allowed / higher replication priority.
    /// Used by replication systems to throttle or prioritize large-scale Forgiveness Wave / Legacy Thread updates.
    pub fn calculate_dynamic_bandwidth_scale(
        &self,
        replication_priority: f32,
        is_critical_for_spectators: bool,
        estimated_spectator_count: u32,
        current_server_load: f32, // 0.0 = idle, 1.0 = saturated
    ) -> f32 {
        let mut scale = replication_priority.clamp(0.1, 2.0);

        if is_critical_for_spectators {
            scale *= 1.6; // Major mercy resolution / monument gets strong boost
        }

        // Scale with spectator count (logarithmic to avoid explosion)
        if estimated_spectator_count > 50 {
            let spectator_factor = (estimated_spectator_count as f32 / 50.0).ln().max(1.0);
            scale *= spectator_factor.min(2.5);
        }

        // Mercy-gated backoff under high server load
        if current_server_load > 0.75 {
            scale *= 0.7;
        } else if current_server_load > 0.9 {
            scale *= 0.5;
        }

        scale.clamp(0.2, 4.0) // final safe bounds
    }

    /// Convenience wrapper that takes the network struct directly
    pub fn calculate_bandwidth_scale_from_net(
        &self,
        spectator_data: Option<&crate::shared::protocol::SpectatorModeDataNet>,
        is_critical: bool,
        estimated_spectators: u32,
        server_load: f32,
    ) -> f32 {
        let priority = spectator_data.map_or(0.5, |d| d.replication_priority);
        self.calculate_dynamic_bandwidth_scale(priority, is_critical, estimated_spectators, server_load)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterestMode {
    Combat,
    CouncilFocus,
    Harvest,
    Default,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerInterestState {
    pub mode: InterestMode,
    pub custom_radius: Option<f32>,
    pub council_session_id: Option<u64>,
}

impl Default for PlayerInterestState {
    fn default() -> Self {
        Self {
            mode: InterestMode::Default,
            custom_radius: None,
            council_session_id: None,
        }
    }
}

pub fn sync_interest_grid(
    mut grid: ResMut<InterestManager>,
    query: Query<(Entity, &Transform, Option<&SpatialEntity>)>,
) {
    for (entity, transform, spatial) in &query {
        let priority = spatial.map(|s| s.interest_priority).unwrap_or(InterestPriority::Medium);
        let custom_r = spatial.and_then(|s| s.custom_interest_radius);
        grid.insert_or_update(entity, transform.translation, priority, custom_r);
    }
}

pub struct InterestManagementPlugin;

impl Plugin for InterestManagementPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HierarchicalGridConfig>()
            .init_resource::<InterestUpdateFlag>()
            .add_systems(Startup, setup_interest_manager)
            .add_systems(Update, sync_interest_grid);
    }
}

fn setup_interest_manager(mut commands: Commands, config: Res<HierarchicalGridConfig>) {
    commands.insert_resource(InterestManager::new(config.clone()));
}

// === Integration Notes (v20.6) ===
// Call interest_manager.calculate_dynamic_bandwidth_scale(...) or calculate_bandwidth_scale_from_net(...)
// from your replication loop / world_state_broadcaster when deciding how much data to send for a Forgiveness Wave or Legacy Thread update.
// Combine with the existing dynamic radius system for full mercy-gated, load-aware replication.

// Thunder locked in. Dynamic bandwidth scaling active for large-scale spectator mercy moments. Yoi ⚔️