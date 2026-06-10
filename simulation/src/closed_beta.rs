/*!
 * Powrush-MMO v18.9 — Closed Beta Infrastructure Foundation
 *
 * Professional, sovereign-friendly foundation for running high-quality closed betas.
 *
 * Features:
 * - Invite-only / access control
 * - Telemetry export (JSON)
 * - Player progress summary export
 * - Easy integration with existing persistence and telemetry
 *
 * All systems are mercy-aligned and can be fully disabled for sovereign self-host.
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// === Closed Beta Configuration ===

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct ClosedBetaConfig {
    pub enabled: bool,
    pub require_invite: bool,
    pub allow_self_host: bool,
    pub max_participants: Option<u32>,
    pub beta_version: String,
}

impl Default for ClosedBetaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            require_invite: true,
            allow_self_host: true,
            max_participants: Some(500),
            beta_version: "v0.9-closed-beta".to_string(),
        }
    }
}

// === Invite / Access Control ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteCode {
    pub code: String,
    pub uses_remaining: u32,
    pub created_by: Option<String>,
    pub expires_at: Option<u64>,
}

#[derive(Resource, Default, Debug)]
pub struct InviteManager {
    pub active_invites: HashMap<String, InviteCode>,
}

impl InviteManager {
    pub fn validate_invite(&self, code: &str) -> bool {
        if let Some(invite) = self.active_invites.get(code) {
            return invite.uses_remaining > 0;
        }
        false
    }

    pub fn consume_invite(&mut self, code: &str) -> bool {
        if let Some(invite) = self.active_invites.get_mut(code) {
            if invite.uses_remaining > 0 {
                invite.uses_remaining -= 1;
                return true;
            }
        }
        false
    }

    pub fn add_invite(&mut self, code: String, uses: u32) {
        self.active_invites.insert(
            code,
            InviteCode {
                code: code.clone(),
                uses_remaining: uses,
                created_by: None,
                expires_at: None,
            },
        );
    }
}

// === Telemetry & Progress Export ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProgressSummary {
    pub player_id: u64,
    pub total_epiphanies: u32,
    pub highest_muscle_memory: f32,
    pub average_mercy_alignment: f32,
    pub council_sessions_participated: u32,
    pub favorite_biome: Option<String>,
    pub total_playtime_minutes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosedBetaExport {
    pub beta_version: String,
    pub exported_at: u64,
    pub total_players: u32,
    pub total_epiphanies_recorded: u64,
    pub player_summaries: Vec<PlayerProgressSummary>,
    pub telemetry_events: Vec<serde_json::Value>, // Flexible for different event types
}

/// Generates a JSON-ready export of closed beta data.
/// Can be called periodically or at the end of a beta session.
pub fn generate_closed_beta_export(
    beta_config: &ClosedBetaConfig,
    player_summaries: Vec<PlayerProgressSummary>,
    telemetry_events: Vec<serde_json::Value>,
) -> ClosedBetaExport {
    ClosedBetaExport {
        beta_version: beta_config.beta_version.clone(),
        exported_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        total_players: player_summaries.len() as u32,
        total_epiphanies_recorded: telemetry_events.len() as u64,
        player_summaries,
        telemetry_events,
    }
}

// === Integration Notes ===

/*
How to wire this into your game:

1. In App startup:
   app.insert_resource(ClosedBetaConfig::default());
   app.insert_resource(InviteManager::default());

2. During player login / onboarding:
   if closed_beta_config.require_invite {
       // validate invite code before allowing full access
   }

3. When exporting data for analysis:
   let export = generate_closed_beta_export(&config, summaries, events);
   // Save to file or send to your analysis dashboard

4. For sovereign self-host:
   Set ClosedBetaConfig.enabled = false to disable all restrictions.
*/
