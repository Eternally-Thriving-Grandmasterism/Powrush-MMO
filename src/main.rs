use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use console_error_panic_hook::set_once;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        set_once();
        console_log::init_with_level(log::Level::Info).unwrap();
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Powrush-MMO — Mercy Eternal".into(),
                canvas: Some("#bevy-canvas".to_string()),
                fit_canvas_to_parent: true,
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
        .add_plugins(HousingPlugin)
        .add_plugins(WeatherPlugin)
        .add_plugins(NPCPlugin)
        .add_plugins(DialoguePlugin)
        .add_plugins(BlockchainPlugin)
        .add_plugins(RedemptionPlugin)
        .add_plugins(PolkadotPlugin)
        .add_plugins(HUDPlugin)
        .add_plugins(SoundPlugin)
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
            chat_send_system,
            chat_render_system,
            voice_modulation_system,
            proximity_voice_system,
            proximity_chat_filter,
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
            housing_spawn_system,
            housing_bonus_system,
            weather_cycle_system,
            npc_ai_system,
            npc_mercy_system,
            procedural_quest_generation,
            dialogue_interaction_system,
            mercy_mint_system,
            token_redemption_system,
            cross_chain_mercy,
            minimap_hud_system,
            procedural_sound_system,
        ))
        .run();
}            weather_cycle_system,
            npc_ai_system,
            npc_mercy_system,
            procedural_quest_generation,
            dialogue_interaction_system,
            mercy_mint_system,
            token_redemption_system,
            cross_chain_mercy,
            minimap_hud_system,
            procedural_sound_system,
        ))
        .run();
}
