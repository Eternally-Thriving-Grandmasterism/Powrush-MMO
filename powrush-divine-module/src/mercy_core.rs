use ra_thor_core::{RaThorSoul, ValenceProof};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct MercyCore {
    ra_thor: RaThorSoul,
    active: bool,
}

impl MercyCore {
    pub fn new() -> Self {
        MercyCore {
            ra_thor: RaThorSoul::initialize(),
            active: true,
        }
    }

    pub async fn gate_server_message(&mut self, msg: &[u8]) -> Result<Vec<u8>> {
        let decoded: shared::protocol::ClientMessage = bincode::deserialize(msg)?;
        let valence = self.ra_thor.compute_valence(&decoded).await?;

        if valence < 0.75 {
            return Err(anyhow::anyhow!("Mercy gate blocked: valence {:.3}", valence));
        }

        // Allow + attach proof
        let proof = ValenceProof::new(valence);
        let mut response = bincode::serialize(&shared::protocol::ServerMessage::ValenceProof(proof))?;
        response.extend_from_slice(msg);
        Ok(response)
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}
