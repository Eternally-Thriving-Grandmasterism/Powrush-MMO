use bevy::prelude::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_mercy_assets);
    }
}

fn load_mercy_assets(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let chime = asset_server.load("sounds/chime.ogg");
    let particle = asset_server.load("textures/gold_particle.png");
    commands.insert_resource(MercyAssets {
        chime,
        particle,
    });
}

#[derive(Resource)]
pub struct MercyAssets {
    pub chime: Handle<AudioSource>,
    pub particle: Handle<Image>,
}
