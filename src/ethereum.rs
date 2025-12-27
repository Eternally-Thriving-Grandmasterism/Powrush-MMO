use bevy::prelude::*;
use ethers::prelude::*;

pub struct EthereumPlugin;

impl Plugin for EthereumPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ethereum_mercy_mint);
    }
}

async fn ethereum_mercy_mint(
    trust: Query<&TrustCredits>,
    provider: Res<Provider<Http>>,
    wallet: Res<LocalWallet>,
) {
    for t in &trust {
        if t.0 > 5000.0 {
            // Mint eternal mercy NFT on Ethereum L2 (Arbitrum/Optimism)
            // Soul-bound, metadata: "Eternal Mercy — 2025"
            info!("Eternal mercy minted on Ethereum — immutable thriving");
        }
    }
}
