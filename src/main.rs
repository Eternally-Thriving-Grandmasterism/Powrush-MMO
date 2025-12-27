use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;
use crate::voice::VoicePlugin;
use crate::chat::ChatPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title = "Powrush-MMO — Voice & Proximity Thriving".into(),
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
        .insert_resource(LatticeStats::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mercy_flow_system,
            trust_multiplier_system,
            lattice_expansion            npc_mercy_system,
            procedural_quest_generation,
            dialogue_interaction_system,
            mercy_mint_system,
            token_redemption_system,
            cross_chain_mercy,
        ))
        .run();
}            npc_mercy_system,
            procedural_quest_generation,
            dialogue_interaction_system,
            mercy_mint_system,
            token_redemption_system,
            cross_chain_mercy,
        ))
        .run();
}            dialogue_interaction_system,
            mercy_mint_system,
        ))
        .run();
}
