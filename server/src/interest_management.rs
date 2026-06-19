// server/src/interest_management.rs
// Powrush-MMO v20.7 — InterestManager + Dynamic Bandwidth Scaling + Adaptive Packet Prioritization
// Production-grade spatial interest + load-aware, mercy-gated packet prioritization for large-scale spectator scenarios.
// Uses fields from shared/protocol.rs for intelligent prioritization of Forgiveness Waves, Legacy Threads, Council Blooms, and Monuments.
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor Lattice aligned

use bevy::prelude::*;
use std::collections::HashMap;
use crate::hierarchical_grid::{HierarchicalGrid, HierarchicalGridConfig, InterestPriority, SpatialEntity, InterestUpdateFlag};
use crate::player_account::PlayerAccount;

/// Public facade for all interest/replication queries + adaptive prioritization.
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
        self.grid.query_interested_entities(player_pos, radius_override)
    }

    pub fn get_interested_players(&self, entity: Entity, entity_pos: Vec3) -> Vec<Entity> {
        let candidates = self.grid.query_interested_entities(entity_pos, Some(self.grid.config.interest_radius * 1.5));
        candidates.into_iter().filter(|&e| e != entity).collect()
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

    // === v20.6 Dynamic Bandwidth Scaling (preserved) ===
    pub fn calculate_dynamic_bandwidth_scale(
        &self,
        replication_priority: f32,
        is_critical_for_spectators: bool,
        estimated_spectator_count: u32,
        current_server_load: f32,
    ) -> f32 {
        let mut scale = replication_priority.clamp(0.1, 2.0);
        if is_critical_for_spectators { scale *= 1.6; }
        if estimated_spectator_count > 50 {
            let spectator_factor = (estimated_spectator_count as f32 / 50.0).ln().max(1.0);
            scale *= spectator_factor.min(2.5);
        }
        if current_server_load > 0.75 { scale *= 0.7; }
        else if current_server_load > 0.9 { scale *= 0.5; }
        scale.clamp(0.2, 4.0)
    }

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

    // === v20.7: Adaptive Packet Prioritization ===
    /// Returns an adaptive priority score (higher = send first / higher QoS) for a packet or entity update.
    /// Combines spatial interest, event criticality, spectator load, and mercy alignment.
    /// Used by replication encoders to decide packet ordering under bandwidth pressure.
    pub fn calculate_adaptive_packet_priority(
        &self,
        base_spatial_priority: InterestPriority,
        replication_priority: f32,
        is_critical_for_spectators: bool,
        estimated_spectator_count: u32,
        is_near_player: bool,
        is_council_or_mercy_event: bool,
        current_server_load: f32,
    ) -> f32 {
        let mut priority = match base_spatial_priority {
            InterestPriority::Critical => 1.0,
            InterestPriority::High => 0.85,
            InterestPriority::Medium => 0.6,
            InterestPriority::Low => 0.35,
        };

        // Boost from protocol-level replication priority
        priority = (priority + replication_priority) / 2.0;

        if is_critical_for_spectators { priority *= 1.5; }
        if is_council_or_mercy_event { priority *= 1.35; }
        if is_near_player { priority *= 1.2; }

        // Slight de-prioritization under extreme spectator load (to protect core gameplay)
        if estimated_spectator_count > 200 && current_server_load > 0.8 {
            priority *= 0.85;
        }

        priority.clamp(0.1, 2.0)
    }

    /// Convenience method using the network struct directly
    pub fn calculate_adaptive_priority_from_net(
        &self,
        spectator_data: Option<&crate::shared::protocol::SpectatorModeDataNet>,
        is_critical: bool,
        estimated_spectators: u32,
        is_near_player: bool,
        is_mercy_event: bool,
        server_load: f32,
    ) -> f32 {
        let base = InterestPriority::High; // default for diplomacy events
        let rep_priority = spectator_data.map_or(0.6, |d| d.replication_priority);
        self.calculate_adaptive_packet_priority(base, rep_priority, is_critical, estimated_spectators, is_near_player, is_mercy_event, server_load)
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

// === Integration Notes (v20.7) ===
// Use calculate_adaptive_packet_priority(...) or calculate_adaptive_priority_from_net(...)
// inside your replication loop / TargetedUpdate builder to order packets intelligently.
// Critical mercy / council / spectator events rise to the top even under heavy load.

// Thunder locked in. Adaptive packet prioritization active. Yoi ⚔️