use bevy::prelude::*;
use bevy_replicon::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RepliconClientPlugin::default())
        .replicate::<MercyPoints>()
        .replicate::<TrustCredits>()
        .add_systems(Startup, connect_to_server)
        .add_systems(Update, client_prediction)
        .run();
}

fn connect_to_server(mut client: ResMut<RepliconClient>) {
    client.connect("ws://localhost:8080").unwrap();
}

fn client_prediction(
    time: Res<Time>,
    mut query: Query<&mut TrustCredits>,
) {
    for mut trust in &mut query {
        trust.0 += time.delta_seconds() * 0.01;  // Predict
    }
}
