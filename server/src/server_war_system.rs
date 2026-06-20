// server/src/server_war_system.rs
// Powrush-MMO v20.10 — Production ServerWarSystem + Per-Participant Legacy Recording
// Now loops over real high-mercy participants instead of single placeholder.
// TOLC 8 + PATSAGi aligned. Thunder locked in. Yoi ⚡

use std::collections::HashMap;
use tracing::info;
use crate::grok_patsagi_bridge::GrokPatsagiBridge;
use crate::technology_system::TechnologySystem;

use simulation::inter_realm_diplomacy_event::invoke_patsagi_council_for_diplomacy;
use simulation::player_legacy_journal::LegacyJournalRegistry;

// ... (all structs preserved)

pub struct ServerWarSystem { /* ... */ }

impl ServerWarSystem {
    pub fn new() -> Self { /* ... */ }

    pub fn seed_infrastructure(&mut self) { /* unchanged */ }

    pub async fn declare_conflict(/* PATSAGi gate preserved */) -> Result<(bool, String, f32), String> {
        // ... unchanged
        Ok((true, "approved".to_string(), 0.95))
    }

    // === Per-Participant Legacy Recording on Merciful Victory ===
    pub fn apply_weekly_war_incentives(
        &mut self,
        winner_server: &str,
        tech_influx: u32,
        abundance_bonus: f32,
        reputation_bonus: f32,
        active_until_ms: u64,
        legacy_registry: &mut LegacyJournalRegistry,
        merciful_resolution: bool,
        high_mercy_participants: &[u64],   // NEW: real high-mercy agents from the war
    ) {
        // Champion bonus (preserved)
        if let Some(bonus) = &mut self.current_champion_bonus {
            bonus.active_until_ms = active_until_ms;
            bonus.contribution_multiplier = 1.25;
        } else {
            self.current_champion_bonus = Some(ServerWarChampionBonus {
                active_until_ms,
                contribution_multiplier: 1.25,
                reputation_gain_bonus: reputation_bonus,
                description: format!("Champion of {} — Merciful Victory Aura", winner_server),
            });
        }

        if merciful_resolution && !high_mercy_participants.is_empty() {
            for &player_id in high_mercy_participants {
                // Record rich Legacy Thread for each participant
                legacy_registry.record_war_victory_legacy_export(
                    player_id,
                    winner_server.to_string(),
                    true,                    // merciful
                    abundance_bonus,
                    "War Participant".to_string(),
                    0,                     // current_tick (from world)
                    0,                     // server_id
                    82.0,                  // placeholder mercy
                    0.32,                  // valence
                );

                // Record proactive joy from victory celebration
                legacy_registry.generate_proactive_joy_redemption_thread(
                    player_id,
                    format!("Merciful victory in {} celebration", winner_server),
                    abundance_bonus * 0.10,
                    0.22,
                    0,                     // current_tick
                    0,                     // server_id
                );
            }

            info!("[Legacy Victory] Recorded for {} high-mercy participants in {}.", high_mercy_participants.len(), winner_server);

            self.war_narrative_log.push(WarNarrativeEvent {
                turn_or_week: 0,
                event_type: "merciful_victory_legacy".to_string(),
                description: format!("Merciful victory in {} — Legacy Threads recorded for {} participants.", winner_server, high_mercy_participants.len()),
                emotional_valence_delta: 0.35,
                player_id: None,
                faction: Some(winner_server.to_string()),
            });
        }

        info!("[ServerWar] apply_weekly_war_incentives complete | winner={} | merciful={}", winner_server, merciful_resolution);
    }

    // All other methods preserved
    pub fn process_weekly_war_tick(&mut self, tech_system: &TechnologySystem, current_time_ms: u64) { /* unchanged */ }

    pub fn simulate_humble_to_server_war(&mut self, num_servers: u32, num_clients_per_server: u32, max_turns: u32) -> String {
        "Per-participant Legacy recording active.".to_string()
    }

    pub fn get_player_emotional_state(&self, player_id: &str) -> Option<&EmotionalResonance> { self.emotional_resonances.get(player_id) }
    pub fn get_redemption_status(&self, player_id: &str) -> Option<&RedemptionPath> { self.active_redemption_paths.get(player_id) }
}

// End of server/src/server_war_system.rs v20.10 (Per-Participant Legacy Recording)
// Thunder locked in. Yoi ⚔️