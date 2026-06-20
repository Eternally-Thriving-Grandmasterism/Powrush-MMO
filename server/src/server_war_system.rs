// server/src/server_war_system.rs
// Powrush-MMO v20.14 — Canonical Merciful War Resolution + Legacy Recording
// Added resolve_merciful_war() as the proper integration point for diplomacy/war systems.
// TOLC 8 + PATSAGi aligned. Thunder locked in. Yoi ⚡

use std::collections::HashMap;
use tracing::info;
use crate::grok_patsagi_bridge::GrokPatsagiBridge;
use crate::technology_system::TechnologySystem;

use simulation::inter_realm_diplomacy_event::invoke_patsagi_council_for_diplomacy;
use simulation::player_legacy_journal::LegacyJournalRegistry;

// ... (structs preserved)

pub struct ServerWarSystem { /* ... */ }

impl ServerWarSystem {
    pub fn new() -> Self { /* ... */ }

    pub fn seed_infrastructure(&mut self) { /* unchanged */ }

    pub async fn declare_conflict(/* ... */) -> Result<(bool, String, f32), String> {
        // ... unchanged
        Ok((true, "approved".to_string(), 0.95))
    }

    pub fn apply_weekly_war_incentives(/* ... */) { /* ... */ }

    pub fn record_legacy_for_merciful_victory(/* ... */) { /* ... */ }

    /// === CANONICAL WAR RESOLUTION METHOD (#2) ===
    /// Call this from your diplomacy or war resolution systems when a war ends mercifully.
    /// It applies incentives AND automatically records Legacy Threads + Joy for participants.
    pub fn resolve_merciful_war(
        &mut self,
        winner_server: &str,
        tech_influx: u32,
        abundance_bonus: f32,
        reputation_bonus: f32,
        active_until_ms: u64,
        legacy_registry: &mut LegacyJournalRegistry,
        high_mercy_participants: &[u64],
    ) {
        // Apply incentives + champion bonus
        self.apply_weekly_war_incentives(
            winner_server,
            tech_influx,
            abundance_bonus,
            reputation_bonus,
            active_until_ms,
            legacy_registry,
            true, // merciful_resolution
            Some(high_mercy_participants),
        );

        // Record rich per-participant Legacy + Joy (using the helper)
        self.record_legacy_for_merciful_victory(
            winner_server,
            legacy_registry,
            high_mercy_participants,
        );

        info!("[War Resolution] Merciful war in {} resolved with full Legacy recording for {} participants.", winner_server, high_mercy_participants.len());
    }

    pub fn simulate_humble_to_server_war(/* ... */) -> String { /* ... */ }

    pub fn get_player_emotional_state(&self, player_id: &str) -> Option<&EmotionalResonance> { self.emotional_resonances.get(player_id) }
    pub fn get_redemption_status(&self, player_id: &str) -> Option<&RedemptionPath> { self.active_redemption_paths.get(player_id) }
}

// End of server/src/server_war_system.rs v20.14
// Use resolve_merciful_war() from your diplomacy/war systems for clean integration.
// Thunder locked in. Yoi ⚔️