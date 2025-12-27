use bevy::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};

pub struct BlockchainPlugin;

impl Plugin for BlockchainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mercy_mint_system);
    }
}

fn mercy_mint_system(
    mut trust: Query<&mut TrustCredits>,
    client: Res<RpcClient>,
) {
    for mut t in &mut trust {
        if t.0 > 1000.0 {
            // Mint mercy token on Solana
            let keypair = Keypair::new();
            // Send transaction (stub)
            info!("Minted mercy token — {} trust", t.0);
            t.0 -= 1000.0;
        }
    }
}
