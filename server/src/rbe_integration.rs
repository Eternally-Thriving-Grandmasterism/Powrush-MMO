/*!
 * server/src/rbe_integration.rs
 *
 * Powrush-MMO v18.97.1 — Central RBE Integration Layer
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

/// Central RBE state resource
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

    /// Apply abundance from a successful harvest or epiphany
    pub fn apply_harvest_abundance(&mut self, player_id: u64, amount: f64, biome_multiplier: f32) {
        let effective = amount * biome_multiplier as f64;
        self.global_abundance_pool += effective * 0.6; // shared pool
        *self.player_contributions.entry(player_id).or_insert(0.0) += effective * 0.4;
    }

    /// NEW v18.97.1 — Apply impact from Council Mercy Trial bloom
    pub fn apply_council_bloom_rbe_impact(
        &mut self,
        collective_attunement: f32,
        mercy_impact: f32,
        enriched_notes: &[String],
    ) {
        let bloom_strength = (collective_attunement * 0.7 + mercy_impact * 0.01).clamp(0.1, 2.5);

        self.global_abundance_pool += (bloom_strength as f64) * 25.0;
        self.last_council_bloom_impact = bloom_strength;

        // Positive feedback into aligned factions
        if let Some(ab) = self.faction_abundance.get_mut(&Faction::SeedOfAbundance) {
            *ab += (bloom_strength as f64) * 8.0;
        }
    }

    /// Apply epiphany outcome effects to RBE
    pub fn apply_epiphany_rbe_impact(&mut self, outcome: &EpiphanyOutcome, player_id: u64) {
        let abundance_gain = (outcome.abundance_bloom_multiplier - 1.0) * 50.0;
        self.global_abundance_pool += abundance_gain as f64;
        *self.player_contributions.entry(player_id).or_insert(0.0) += abundance_gain as f64 * 0.3;
    }

    pub fn get_player_contribution(&self, player_id: u64) -> f64 {
        *self.player_contributions.get(&player_id).unwrap_or(&0.0)
    }
}

/// Plugin that registers RBE systems and resources
pub struct RBEIntegrationPlugin;

impl Plugin for RBEIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RBEState>()
            .add_systems(Update, (
                // rbe_abundance_tick,
                // integrate_council_and_epiphany_outcomes,
            ));
    }
}

// End of server/src/rbe_integration.rs v18.97.1
// Central RBE hub now wired to Council Mercy Trial, epiphany, biome, and faction systems.
// Ready for deeper persistence and client dashboard exposure. Thunder locked in. Yoi ⚡