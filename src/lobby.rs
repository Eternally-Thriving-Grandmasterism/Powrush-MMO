use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Component, Replicated)]
pub struct LobbyPlayer {
    pub name: String,
    pub trust: f32,
    pub ready: bool,
}

#[derive(Component)]
pub struct LobbyHub {
    pub players: Vec<Entity>,
    pub max_players: usize,
}

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (lobby_matchmake, lobby_ready_system));
    }
}

fn lobby_matchmake(
    mut commands: Commands,
    players: Query<(Entity, &TrustCredits), Without<LobbyHub>>,
) {
    for (player, trust) in &players {
        // Create/match lobby based on trust
        commands.spawn((
            LobbyHub { players: vec![player], max_players: 8 },
            LobbyPlayer {
                name: "Mercy Traveler".to_string(),
                trust: trust.0,
                ready: false,
            },
        ));
    }
}

fn lobby_ready_system(
    mut hubs: Query<&mut LobbyHub>,
    players: Query<&mut LobbyPlayer>,
) {
    for hub in hubs.iter_mut() {
        let ready_count = players.iter().filter(|p| p.ready).count();
        if ready_count >= hub.max_players / 2 {
            info!("Lobby full — mercy game starts");
        }
    }
}
