use bevy::prelude::*;

#[derive(Resource)]
pub struct MercyAssets {
    pub chime: Handle<AudioSource>,
    pub particle: Handle<Image>,
}

pub struct AssetPipelinePlugin;

impl Plugin for AssetPipelinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_mercy_assets);
    }
}

fn load_mercy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let chime: Handle<AudioSource> = asset_server.load("sounds/mercy_chime.ogg");
    let particle: Handle<Image> = asset_server.load("textures/gold_particle.png");

    commands.insert_resource(MercyAssets { chime, particle });
    info!("Mercy assets loaded — hot-reload ready");
}
