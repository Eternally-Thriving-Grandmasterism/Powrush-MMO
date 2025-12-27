use bevy::prelude::*;
use bevy_replicon::prelude::*;
use std::collections::HashMap;

#[derive(Component, Replicated)]
pub struct LobbyPlayer {
    pub name: String,
    pub trust: f32,
    pub ready: bool,
    pub guild_id: Option<u64>,
}

#[derive(Component)]
pub struct LobbyHub {
    pub players: Vec<Entity>,
    pub max_players: usize,
    pub alliances: HashMap<u64, u64>,  // Guild alliances
}

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            lobby_matchmake,
            lobby_ready_system,
            guild_alliance_system,
            lobby_ui_visuals,
        ));
    }
}

fn lobby_matchmake(
    mut commands: Commands,
    players: Query<(Entity, &TrustCredits, &MemberOf), Without<LobbyHub>>,
) {
    for (player, trust, member_of) in &players {
        // Match by trust + guild
        let mut best_hub = None;
        // Find hub with closest trust + guild match
        // Simplified: spawn new if none
        let hub = commands.spawn((
            LobbyHub {
                players: vec![player],
                max_players: 8,
                alliances: HashMap::new(),
            },
        )).id();

        commands.entity(player).insert(LobbyPlayer {
            name: "Mercy Traveler".to_string(),
            trust: trust.0,
            ready: false,
            guild_id: Some(member_of.0 .0),
        });
    }
}

fn guild_alliance_system(
    mut hubs: Query<&mut LobbyHub>,
) {
    for mut hub in &mut hubs {
        // Auto-alliance same guilds
        let guild_counts = hub.players.iter().fold(HashMap::new(), |mut acc, p| {
            if let Ok(player) = p.get::<LobbyPlayer>() {
                *acc.entry(player.guild_id.unwrap_or(0)).or_insert(0) += 1;
            }
            acc
        });
        // Bonus trust if >3 same guild
        if let Some((&guild, &count)) = guild_counts.iter().max_by_key(|&(_, c)| c) {
            if count >= 3 {
                hub.alliances.insert(guild, count as u64);
                info!("Guild alliance formed — +{} trust", count * 10);
            }
        }
    }
}

fn lobby_ready_system(
    mut hubs: Query<&mut LobbyHub>,
    players: Query<&LobbyPlayer>,
) {
    for hub in &mut hubs {
        let ready = players.iter().filter(|p| p.ready).count();
        if ready >= hub.max_players / 2 {
            info!("Lobby ready — mercy game begins");
        }
    }
}

fn lobby_ui_visuals(
    mut commands: Commands,
    hubs: Query<&LobbyHub>,
) {
    for hub in &hubs {
        // Procedural lobby UI
        commands.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.05, 0.1, 0.2, 0.8)),
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Mercy Lobby — Players Ready",
                TextStyle { font_size: 40.0, color: Color::CYAN, ..default() },
            ));
        });
    }
}
