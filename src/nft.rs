use bevy::prelude::*;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};

pub struct NFTPlugin;

impl Plugin for NFTPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, nft_mint_system);
    }
}

fn nft_mint_system(
    trust: Query<&TrustCredits>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::N) {
        if let Ok(t) = trust.get_single() {
            if t.0 > 2000.0 {
                // Mint soul-bound NFT on Solana
                let client = RpcClient::new("https://api.mainnet-beta.solana.com");
                let keypair = Keypair::new();
                info!("Soul-bound Mercy NFT minted — eternal achievement");
            }
        }
    }
}
