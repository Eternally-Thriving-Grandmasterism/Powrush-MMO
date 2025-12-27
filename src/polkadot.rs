use bevy::prelude::*;
use subxt::{OnlineClient, PolkadotConfig};

#[derive(Resource)]
pub struct PolkadotClient(OnlineClient<PolkadotConfig>);

pub struct PolkadotPlugin;

impl Plugin for PolkadotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, connect_polkadot);
    }
}

async fn connect_polkadot(
    mut commands: Commands,
) {
    let client = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io")
        .await
        .unwrap();
    commands.insert_resource(PolkadotClient(client));
    info!("Polkadot connected — cross-chain mercy");
}

fn cross_chain_mercy(
    polka: Res<PolkadotClient>,
    trust: Query<&TrustCredits>,
) {
    if let Ok(t) = trust.get_single() {
        if t.0 > 10000.0 {
            info!("Cross-chain mercy — eternal on Polkadot");
        }
    }
}
