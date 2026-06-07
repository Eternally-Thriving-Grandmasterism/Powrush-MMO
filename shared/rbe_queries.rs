//! shared/rbe_queries.rs
//! Full Ra-Thor-derived RBE Queries + Mercy-Gated Access
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use powrush_rbe_engine::{RbeResourcePool, RbeQuery, RbeQueryResult};
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;

pub struct RbeQueryEngine {
    lattice: SovereignLattice,
}

impl RbeQueryEngine {
    pub fn new(lattice: SovereignLattice) -> Self {
        Self { lattice }
    }

    /// Mercy-gated global RBE state query
    pub async fn query_global_state(&self) -> RbeQueryResult {
        let gates = [
            MercyGate::Truth,
            MercyGate::Order,
            MercyGate::Love,
            MercyGate::Compassion,
            MercyGate::Service,
            MercyGate::Abundance,
            MercyGate::Joy,
            MercyGate::CosmicHarmony,
        ];

        let valence = evaluate_mercy_gates(&gates, "global_rbe_query").await;
        if valence < 0.999999 {
            return RbeQueryResult::refined("Mercy refinement — increasing abundance visibility");
        }

        let result = RbeQuery::global_state();
        self.lattice.tick("RBE global query served with full mercy").await.ok();
        result
    }

    /// Query resources for a specific joy sanctuary or player zone
    pub async fn query_local_abundance(&self, zone_id: u64) -> RbeQueryResult {
        let result = RbeQuery::local_abundance(zone_id);
        self.lattice.tick(&format!("RBE local abundance query for zone {}", zone_id)).await.ok();
        result
    }

    /// Query inter-species resource sharing opportunities
    pub async fn query_interspecies_sharing(&self) -> RbeQueryResult {
        let result = RbeQuery::interspecies_sharing_opportunities();
        self.lattice.tick("Inter-species RBE sharing query served").await.ok();
        result
    }
}
