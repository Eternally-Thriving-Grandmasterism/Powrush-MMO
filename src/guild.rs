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
        app.add_systems(Update, (
            guild_alliance_system,
            guild_quest_bonus_system,  // Quests boosted by guild
        ));
    }
}

fn guild_alliance_system(
    mut guilds: Query<&mut GuildAlliance>,
) {
    for mut alliance in &mut guilds {
        for (_, bonus) in alliance.allies.iter_mut() {
            *bonus *= 1.01;
        }
    }
}

fn guild_quest_bonus_system(
    player_guild: Query<&PlayerGuild>,
    mut quests: Query<&mut Quest>,
) {
    for guild in &player_guild {
        for mut quest in &mut quests {
            if guild.role == GuildRole::Leader {
                quest.goal *= 0.9;  // Guild leader bonus
            }
        }
    }
}
