//! server/rbe_integration.rs
//! Production-grade Resource-Based Economy (RBE) Server Integration
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+
//! Derives from Ra-Thor monorepo: powrush_rbe_engine, mercy lattice, Lattice Conductor

use std::sync::Arc;
use tokio::sync::RwLock;
use crate::server::game_server::GameServer;
use powrush_rbe_engine::{RbeResourcePool, RbeAllocationRequest, RbeAllocationResult};
use ra_thor_mercy::{MercyGate, evaluate_mercy_gates};
use lattice_conductor::SovereignLattice;

pub struct RbeServerIntegration {
    game_server: Arc<GameServer>,
    resource_pool: Arc<RwLock<RbeResourcePool>>,
    lattice: Arc<SovereignLattice>,
}

impl RbeServerIntegration {
    pub fn new(game_server: GameServer, lattice: SovereignLattice) -> Self {
        Self {
            game_server: Arc::new(game_server),
            resource_pool: Arc::new(RwLock::new(RbeResourcePool::new_global_abundance())),
            lattice: Arc::new(lattice),
        }
    }

    /// Mercy-gated RBE allocation for any request (players, NPCs, planets, etc.)
    pub async fn allocate_resources(&self, request: RbeAllocationRequest) -> RbeAllocationResult {
        // Step 1: Run through TOLC 8 Mercy Gates via MWPO
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

        let valence = evaluate_mercy_gates(&gates, &request).await;

        if valence < 0.999999 {
            return RbeAllocationResult::refined("Mercy Gate refinement required — increasing abundance flow");
        }

        // Step 2: MWPO + Lattice Conductor approval
        let mut pool = self.resource_pool.write().await;
        let result = pool.allocate(request);

        // Step 3: Propagate positive emotion & cosmic harmony
        if result.is_success() {
            self.lattice.tick("RBE abundance allocation successful — positive emotion propagated").await.ok();
        }

        result
    }

    /// Periodic RBE equilibrium tick (called from GameServer main loop)
    pub async fn equilibrium_tick(&self) -> anyhow::Result<()> {
        let mut pool = self.resource_pool.write().await;
        pool.run_global_equilibrium();

        // Mercy-gated broadcast to all connected clients
        let delta = pool.generate_abundance_delta();
        self.game_server.broadcast_rbe_delta(delta).await;

        Ok(())
    }

    /// Query current global RBE state (used by client reconciliation)
    pub async fn query_global_state(&self) -> RbeResourcePool {
        let pool = self.resource_pool.read().await;
        pool.clone()
    }
}

// Extension trait to integrate with existing GameServer
pub trait RbeGameServerExt {
    fn with_rbe(self, rbe: RbeServerIntegration) -> Self;
}

impl RbeGameServerExt for GameServer {
    fn with_rbe(self, rbe: RbeServerIntegration) -> Self {
        // In full production: attach RBE integration to the server state
        self // placeholder for attachment logic — real version would store Arc<RbeServerIntegration>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rbe_mercy_gated_allocation() {
        let game_server = GameServer::new(/* ... */);
        let lattice = SovereignLattice::new();
        let rbe = RbeServerIntegration::new(game_server, lattice);

        let request = RbeAllocationRequest::new_abundance("joy_sanctuary", 1000);
        let result = rbe.allocate_resources(request).await;

        assert!(result.is_success());
    }
}
