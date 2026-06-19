// simulation/src/council/session.rs
// Basic CouncilSession for Local Council

use crate::council::proposal::CouncilProposal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CouncilSession {
    pub realm_id: u8,
    pub active_proposals: Vec<CouncilProposal>,
    pub last_session_tick: u64,
}

impl CouncilSession {
    pub fn new(realm_id: u8, current_tick: u64) -> Self {
        Self {
            realm_id,
            active_proposals: vec![],
            last_session_tick: current_tick,
        }
    }

    pub fn add_proposal(&mut self, proposal: CouncilProposal) {
        self.active_proposals.push(proposal);
    }
}
