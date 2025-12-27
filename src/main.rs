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
use crate::guild::GuildPlugin;
use crate::arena::ArenaPlugin;
use crate::events::EventsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Events Thriving".into(),
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
        .add_plugins(GuildPlugin)
        .add_plugins(ArenaPlugin)
        .add_plugins(EventsPlugin)
        .insert_resource(LatticeStats::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mercy_flow_system,
            trust_multiplier_system,
            lattice_expansion_system,
            spawn_particles_system,
            particle_update_system,
            emote_input_system,
            emote_visual
