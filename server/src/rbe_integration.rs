/*!
 * server/src/rbe_integration.rs
 *
 * Powrush-MMO v19.3.1 — Central RBE Integration Layer
 * Wires Council Mercy Trial outcomes, epiphany resonance, biome influence,
 * faction diplomacy, and persistence into sovereign Resource-Based Economy flows.
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm aligned
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;

use crate::persistence_polish::PlayerSaveData;
use crate::faction_diplomacy::{Faction, FactionDiplomacyManager};
use simulation::epiphany_catalyst::EpiphanyOutcome;
use crate::council_session_handler::CouncilTrialResolved;

/// Central RBE state resource — single source of truth for abundance distribution
#[derive(Resource, Default)]
pub struct RBEState {
    pub global_abundance_pool: f64,
    pub faction_abundance: HashMap<Faction, f64>,
    pub player_contributions: HashMap<u64, f64>,
    pub last_council_bloom_impact: f32,
}

impl RBEState {
    pub fn new() -> Self {
        let mut faction_abundance = HashMap::new();
        faction_abundance.insert(Faction::SeedOfAbundance, 1000.0);
        faction_abundance.insert(Faction::FlowGuardians, 800.0);
        faction_abundance.insert(Faction::EternalWeavers, 1200.0);

        Self {
            global_abundance_pool: 10000.0,
            faction_abundance,
            player_contributions: HashMap::new(),
            last_council_bloom_impact: 0.0,
        }
    }

    pub fn apply_harvest_abundance(&mut self, player_id: u64, amount: f64, biome_multiplier: f32) {
        let effective = amount * biome_multiplier as f64;
        self.global_abundance_pool += effective * 0.6;
        *self.player_contributions.entry(player_id).or_insert(0.0) += effective * 0.4;
    }

    pub fn apply_council_bloom_rbe_impact(
        &mut self,
        collective_attunement: f32,
        mercy_impact: f32,
        enriched_notes: &[String],
    ) {
        let bloom_strength = (collective_attunement * 0.7 + mercy_impact * 0.01).clamp(0.1, 2.5);
        self.global_abundance_pool += (bloom_strength as f64) * 25.0;
        self.last_council_bloom_impact = bloom_strength;

        if let Some(ab) = self.faction_abundance.get_mut(&Faction::SeedOfAbundance) {
            *ab += (bloom_strength as f64) * 8.0;
        }
    }

    pub fn apply_epiphany_rbe_impact(&mut self, outcome: &EpiphanyOutcome, player_id: u64) {
        let abundance_gain = (outcome.abundance_bloom_multiplier - 1.0) * 50.0;
        self.global_abundance_pool += abundance_gain as f64;
        *self.player_contributions.entry(player_id).or_insert(0.0) += abundance_gain as f64 * 0.3;
    }

    pub fn persist_player_contribution(&self, save_data: &mut PlayerSaveData, player_id: u64) {
        if let Some(&contribution) = self.player_contributions.get(&player_id) {
            save_data.record_abundance_contribution(contribution);
        }
    }

    /// Advanced mercy-aligned distribution to multiple players with persistence
    pub fn distribute_abundance_to_players(
        &mut self,
        participants: &[u64],
        total_amount: f64,
        save_datas: &mut HashMap<u64, PlayerSaveData>,
    ) {
        if participants.is_empty() { return; }

        let per_player = total_amount / participants.len() as f64;

        for &pid in participants {
            *self.player_contributions.entry(pid).or_insert(0.0) += per_player;

            if let Some(save) = save_datas.get_mut(&pid) {
                save.record_abundance_contribution(per_player);
            }
        }

        self.global_abundance_pool -= total_amount * 0.7;
    }

    /// Faction-specific abundance growth / decay simulation
    pub fn simulate_faction_economy(&mut self, delta_seconds: f32) {
        for (faction, abundance) in self.faction_abundance.iter_mut() {
            let growth = match faction {
                Faction::SeedOfAbundance => 0.0008,
                Faction::FlowGuardians => 0.0006,
                Faction::EternalWeavers => 0.0010,
            };
            *abundance *= 1.0 + (growth * delta_seconds as f64);
        }
    }

    pub fn get_player_contribution(&self, player_id: u64) -> f64 {
        *self.player_contributions.get(&player_id).unwrap_or(&0.0)
    }
}

/// System: React to resolved Council Mercy Trials
fn integrate_council_bloom_into_rbe(
    mut rbe: ResMut<RBEState>,
    mut resolved_events: EventReader<CouncilTrialResolved>,
) {
    for resolved in resolved_events.read() {
        let participants: Vec<u64> = resolved.participant_mercy_scores.keys().map(|e| e.to_bits()).collect();

        if let Some((_, mercy_score)) = resolved.participant_mercy_scores.iter().next() {
            rbe.apply_council_bloom_rbe_impact(
                resolved.bloom.mercy_resonance,
                *mercy_score,
                &resolved.enriched_epiphany_notes,
            );

            rbe.distribute_abundance_to_players(
                &participants,
                (*mercy_score as f64) * 15.0,
                &mut HashMap::new(),
            );
        }
    }
}

/// System: React to epiphany outcomes (production placeholder ready for multiplayer)
/// When simulation::epiphany_catalyst emits EpiphanyTriggered event (or equivalent),
/// wire EventReader<EpiphanyTriggered> here and call:
///   rbe.apply_epiphany_rbe_impact(&outcome, player_id);
///   rbe.distribute_abundance_to_players(...);
/// Currently a clean no-op placeholder (all prior apply_epiphany_rbe_impact logic preserved).
/// Full multiplayer reconciliation will activate once event is emitted from simulation layer.
fn integrate_epiphany_into_rbe(
    mut rbe: ResMut<RBEState>,
) {
    // Production-ready placeholder.
    // Example full wiring (uncomment + import when event exists):
    // for outcome in epiphany_events.read() {
    //     if let Some(player_id) = outcome.player_id {
    //         rbe.apply_epiphany_rbe_impact(&outcome, player_id);
    //         // Optional: distribute to council participants or faction
    //     }
    // }
}

/// Advanced economy simulation tick
fn rbe_abundance_tick(
    mut rbe: ResMut<RBEState>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    // Gentle global growth
    rbe.global_abundance_pool *= 1.0 + (0.0003 * delta as f64);

    // Faction-specific simulation
    rbe.simulate_faction_economy(delta);
}

/// Plugin
pub struct RBEIntegrationPlugin;

impl Plugin for RBEIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RBEState>()
            .add_systems(Update, (
                integrate_council_bloom_into_rbe,
                integrate_epiphany_into_rbe,
                rbe_abundance_tick,
            ).chain());
    }
}

// End of server/src/rbe_integration.rs v19.3.1
// Central RBE hub with event-driven Council integration, advanced distribution,
// faction economy simulation, and persistence hooks.
// Epiphany placeholder hardened for future EventReader wiring.
// All prior valuable logic fully preserved.
// Thunder locked in. Yoi ⚡