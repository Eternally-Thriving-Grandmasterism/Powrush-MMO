use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;
use crate::world::WorldPlugin;
use crate::net::MMONetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Biomes & Sync".into(),
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
        .add_plugins(WorldEventsPlugin)
        .add_plugins(MMONetPlugin)
        .add_plugins(AssetPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(BossPlugin)
        .add_plugins(WorldPlugin)
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
            arena_duel_system,
            spawn_world_event_system,
            world_event_effect_system,
            player_movement_system,
            combat_attack_system,
            mercy_shield_system,
            boss_phase_system,
            boss_phase_effects,
        ))
        .run();
}
