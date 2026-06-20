// server/src/server_war_system.rs
// Powrush-MMO v20.15 — Main War Resolution Entry Point
// resolve_war() is now the method diplomacy/war systems should call.
// It automatically handles merciful vs escalated cases + Legacy recording.
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

    pub fn resolve_merciful_war(/* ... */) { /* ... */ }

    /// === MAIN ENTRY POINT FOR DIPLOMACY / WAR SYSTEMS (#1) ===
    /// Call this when a war between servers ends.
    /// It will automatically apply incentives and record Legacy Threads + Joy
    /// only when the resolution was merciful.
    pub fn resolve_war(
        &mut self,
        winner_server: &str,
        tech_influx: u32,
        abundance_bonus: f32,
        reputation_bonus: f32,
        active_until_ms: u64,
        legacy_registry: &mut LegacyJournalRegistry,
        was_merciful: bool,
        high_mercy_participants: &[u64],
    ) {
        if was_merciful {
            self.resolve_merciful_war(
                winner_server,
                tech_influx,
                abundance_bonus,
                reputation_bonus,
                active_until_ms,
                legacy_registry,
                high_mercy_participants,
            );
        } else {
            // Escalated path: still apply incentives but without rich Legacy recording
            self.apply_weekly_war_incentives(
                winner_server,
                tech_influx,
                abundance_bonus,
                reputation_bonus,
                active_until_ms,
                legacy_registry,
                false,
                None,
            );
            info!("[War Resolution] Escalated war in {} resolved (no rich Legacy recording).", winner_server);
        }
    }

    pub fn simulate_humble_to_server_war(/* ... */) -> String { /* ... */ }

    pub fn get_player_emotional_state(&self, player_id: &str) -> Option<&EmotionalResonance> { self.emotional_resonances.get(player_id) }
    pub fn get_redemption_status(&self, player_id: &str) -> Option<&RedemptionPath> { self.active_redemption_paths.get(player_id) }
}

// End of server/src/server_war_system.rs v20.15
// Diplomacy systems should call resolve_war() when a war ends.
// Thunder locked in. Yoi ⚔️