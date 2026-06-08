//! shared/rbe_queries.rs
//! Full Ra-Thor-derived RBE Queries — Mercy-Gated, Valence-Driven, Production-Grade
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use powrush_rbe_engine::{RbeResourcePool, RbeQuery, RbeQueryResult};
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;
use std::sync::Arc;

pub struct RbeQueryEngine {
    lattice: Arc<SovereignLattice>,
    rbe_pool: Arc<RbeResourcePool>,
}

impl RbeQueryEngine {
    pub fn new(lattice: Arc<SovereignLattice>, rbe_pool: Arc<RbeResourcePool>) -> Self {
        Self { lattice, rbe_pool }
    }

    /// Global RBE state query — mercy-gated
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
            return RbeQueryResult::refined("Mercy Gate refinement — increasing global abundance visibility");
        }

        let result = RbeQuery::global_state(self.rbe_pool.as_ref());
        let _ = self.lattice.tick("Global RBE state queried with full mercy").await;
        result
    }

    /// Local abundance query for a specific zone / sanctuary
    pub async fn query_local_abundance(&self, zone_id: u64) -> RbeQueryResult {
        let result = RbeQuery::local_abundance(zone_id, self.rbe_pool.as_ref());
        let _ = self.lattice.tick(&format!("Local abundance queried for zone {}", zone_id)).await;
        result
    }

    /// Interspecies resource sharing opportunities
    pub async fn query_interspecies_sharing(&self) -> RbeQueryResult {
        let result = RbeQuery::interspecies_sharing_opportunities(self.rbe_pool.as_ref());
        let _ = self.lattice.tick("Inter-species RBE sharing opportunities queried").await;
        result
    }

    /// Player-specific RBE contribution and status
    pub async fn query_player_contribution(&self, player_id: u64) -> RbeQueryResult {
        let result = RbeQuery::player_contribution(player_id, self.rbe_pool.as_ref());
        let _ = self.lattice.tick(&format!("Player {} RBE contribution queried", player_id)).await;
        result
    }

    /// Joy Sanctuary status and harmony level
    pub async fn query_joy_sanctuary(&self, sanctuary_id: u64) -> RbeQueryResult {
        let result = RbeQuery::joy_sanctuary_status(sanctuary_id, self.rbe_pool.as_ref());
        let _ = self.lattice.tick(&format!("Joy Sanctuary {} status queried", sanctuary_id)).await;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rbe_queries_mercy_gated() {
        let lattice = Arc::new(SovereignLattice::new());
        let rbe_pool = Arc::new(RbeResourcePool::new_global_abundance());
        let engine = RbeQueryEngine::new(lattice, rbe_pool);

        let result = engine.query_global_state().await;
        assert!(result.is_success());
    }
}
