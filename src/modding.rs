use bevy::prelude::*;
use bevy::asset::AssetServer;

pub struct ModdingPlugin;

impl Plugin for ModdingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_mod_assets);
    }
}

fn load_mod_assets(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    // Hot-reload mod folder
    let _custom_quest = asset_server.load("mods/custom_quest.ron");
    let _custom_item = asset_server.load("mods/custom_item.ron");
    info!("Modding support active — community mercy loaded");
}
