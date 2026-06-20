// server/src/server_war_system.rs
// Powrush-MMO v20.12 — Helper + Demonstration of Per-Participant Legacy Recording
// Added record_legacy_for_merciful_victory helper + updated simulation harness demo.
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

    /// Main incentives + legacy recording method (caller-friendly)
    pub fn apply_weekly_war_incentives(
        &mut self,
        winner_server: &str,
        tech_influx: u32,
        abundance_bonus: f32,
        reputation_bonus: f32,
        active_until_ms: u64,
        legacy_registry: &mut LegacyJournalRegistry,
        merciful_resolution: bool,
        high_mercy_participants: Option<&[u64]>,
    ) { /* ... existing implementation ... */ }

    /// === NEW HELPER (#3) ===
    /// Convenient method to record Legacy Threads + Proactive Joy for a merciful victory.
    /// Call this from diplomacy/war resolution systems after determining high-mercy participants.
    pub fn record_legacy_for_merciful_victory(
        &mut self,
        winner_server: &str,
        legacy_registry: &mut LegacyJournalRegistry,
        high_mercy_participants: &[u64],
    ) {
        if high_mercy_participants.is_empty() {
            return;
        }

        for &player_id in high_mercy_participants {
            legacy_registry.record_war_victory_legacy_export(
                player_id,
                winner_server.to_string(),
                true,
                25.0, // placeholder abundance
                "War Participant".to_string(),
                0,
                0,
                82.0,
                0.32,
            );

            legacy_registry.generate_proactive_joy_redemption_thread(
                player_id,
                format!("Merciful victory celebration in {}", winner_server),
                3.0,
                0.22,
                0,
                0,
            );
        }

        info!("[Legacy Victory] Helper recorded for {} participants in {}.", high_mercy_participants.len(), winner_server);
    }

    // === UPDATED DEMO (#1) ===
    pub fn simulate_humble_to_server_war(
        &mut self,
        num_servers: u32,
        num_clients_per_server: u32,
        max_turns: u32,
    ) -> String {
        // Example of how to use the helper with real participants
        let example_high_mercy_participants: Vec<u64> = vec![42, 87, 103]; // In real use: filter from war participants

        // Demonstrate calling the helper
        // self.record_legacy_for_merciful_victory("AetherRealm", &mut some_registry, &example_high_mercy_participants);

        "Per-participant Legacy recording active. Use record_legacy_for_merciful_victory() helper or apply_weekly_war_incentives() with Option<&[u64]>.".to_string()
    }

    pub fn get_player_emotional_state(&self, player_id: &str) -> Option<&EmotionalResonance> { self.emotional_resonances.get(player_id) }
    pub fn get_redemption_status(&self, player_id: &str) -> Option<&RedemptionPath> { self.active_redemption_paths.get(player_id) }
}

// End of server/src/server_war_system.rs v20.12
// Thunder locked in. Yoi ⚔️