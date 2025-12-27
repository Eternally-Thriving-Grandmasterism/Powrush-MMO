use bevy::prelude::*;
use bevy_replicon::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None,
            ..default()
        }))
        .add_plugins(RepliconPlugins)
        .replicate::<MercyPoints>()
        .replicate::<TrustCredits>()
        .add_systems(Update, mercy_flow_server)
        .run();
}

fn mercy_flow_server(
    mut query: Query<&mut TrustCredits>,
) {
    for mut trust in &mut query {
        trust.0 *= 1.01;
    }
}
