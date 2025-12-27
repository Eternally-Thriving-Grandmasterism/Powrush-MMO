use bevy::prelude::*;
use bevy_replicon::prelude::*;
use std::collections::HashMap;

#[derive(Component, Replicated)]
pub struct PlayerGuild {
    pub guild_id: u64,
    pub role: GuildRole,
}

#[derive(Clone, Copy, PartialEq, Replicated)]
pub enum GuildRole {
    Member,
    Officer,
    Leader,
}

#[derive(Component)]
pub struct GuildAlliance {
    pub allies: HashMap<u64, f32>,  // ally_guild_id → trust_bonus
}

pub struct GuildPlugin;

impl Plugin for GuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, guild_alliance_system);
    }
}

fn guild_alliance_system(
    mut guilds: Query<&mut GuildAlliance>,
) {
    for mut alliance in guilds.iter_mut() {
        for (_, bonus) in alliance.allies.iter_mut() {
            *bonus *= 1.01;  // Eternal alliance growth
        }
        info!("Guild alliances thriving — mercy multiplies");
    }
}
