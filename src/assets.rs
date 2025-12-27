use bevy::prelude::*;

pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Procedural fallback — real assets if present
    let _chime = asset_server.load("sounds/chime.ogg");
    // Add more as needed
}
