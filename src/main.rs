use bevy::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--server".to_string()) {
        ServerApp::new().run();
    } else {
        ClientApp::new().run();
    }
}

struct ServerApp;
impl App for ServerApp {
    fn run(self) {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin { primary_window: None, ..default() }))
            .add_plugins(RepliconPlugins)
            .replicate::<MercyPoints>()
            .add_systems(Update, mercy_flow_server)
            .run();
    }
}

struct ClientApp;
impl App for ClientApp {
    fn run(self) {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(RepliconClientPlugin::default())
            .replicate::<MercyPoints>()
            .add_systems(Startup, connect_to_server)
            .add_systems(Update, client_prediction)
            .run();
    }
}

fn mercy_flow_server(mut query: Query<&mut TrustCredits>) {
    for mut trust in &mut query {
        trust.0 *= 1.01;
    }
}

fn connect_to_server(mut client: ResMut<RepliconClient>) {
    client.connect("ws://localhost:8080").unwrap();
}

fn client_prediction(time: Res<Time>, mut query: Query<&mut TrustCredits>) {
    for mut trust in &mut query {
        trust.0 += time.delta_seconds() * 0.01;
    }
}}
