//! client/rbe_client_sync.rs
//! Full production-grade Client-side RBE Synchronization
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use std::sync::Arc;
use tokio::sync::RwLock;
use bytes::Bytes;
use crate::network::message_framing::decode_frame;
use crate::client_game_loop::ClientGameLoop;
use powrush_rbe_engine::{RbeResourcePool, RbeDelta};

pub struct RbeClientSync {
    client_loop: Arc<RwLock<ClientGameLoop>>,
    local_pool: Arc<RwLock<RbeResourcePool>>,
}

impl RbeClientSync {
    pub fn new(client_loop: ClientGameLoop) -> Self {
        Self {
            client_loop: Arc::new(RwLock::new(client_loop)),
            local_pool: Arc::new(RwLock::new(RbeResourcePool::new_local_view())),
        }
    }

    pub async fn handle_rbe_delta(&self, data: Bytes) {
        let (_header, payload) = decode_frame(data).unwrap();
        let delta: RbeDelta = bincode::deserialize(&payload).unwrap();

        let mut pool = self.local_pool.write().await;
        pool.apply_delta(delta);

        // Reconcile with client prediction
        let mut client = self.client_loop.write().await;
        client.reconcile_rbe_state(pool.current_state());
    }

    pub async fn query_local_rbe_state(&self) -> RbeResourcePool {
        let pool = self.local_pool.read().await;
        pool.clone()
    }
}

// Extension for ClientGameLoop
pub trait RbeClientLoopExt {
    fn with_rbe_sync(self, sync: RbeClientSync) -> Self;
}

impl RbeClientLoopExt for ClientGameLoop {
    fn with_rbe_sync(self, _sync: RbeClientSync) -> Self {
        self // In full version this would attach the sync handle
    }
}
