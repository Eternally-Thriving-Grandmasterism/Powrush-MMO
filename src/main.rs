use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;
use crate::emote::EmotePlugin;
use crate::chat::ChatPlugin;
use crate::inventory::InventoryPlugin;
use crate::trading::TradingPlugin;
use crate::auction::AuctionPlugin;
use crate::quests::QuestPlugin;
use crate::leveling::LevelingPlugin;
use crate::guild::GuildPlugin;  // New

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Guilds & Quests Thriving".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(VoicePlugin)
        .add_plugins(EmotePlugin)
        .add_plugins(ChatPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(TradingPlugin)
        .add_plugins(AuctionPlugin)
        .add_plugins(QuestPlugin)
        .add_plugins(LevelingPlugin)
        .add_plugins(GuildPlugin)  // New
        .insert_resource(LatticeStats::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mercy_flow_system,
            trust_multiplier_system,
            lattice_expansion_system,
            spawn_particles_system,
            particle_update_system,
            emote_input_system,
            emote_visual_system,
            emote_audio_system,
            chat_input_system,
            chat_render_system,
            inventory_capacity_system,
            item_decay_system,
            item_generation_system,
            item_interaction_system,
            trade_request_system,
            trade_accept_system,
            auction_bid_system,
            auction_timer_system,
            auction_ui_system,
            quest_progress_system,
            quest_reward_system,
            leveling_system,
            guild_alliance_system,
            guild_quest_bonus_system,
        ))
        .run();
}
