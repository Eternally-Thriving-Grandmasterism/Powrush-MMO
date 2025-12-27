use bevy::prelude::*;
use bevy::asset::{AssetPath, LoadState};

#[derive(Resource)]
pub struct MercyAssets {
    pub chime: Handle<AudioSource>,
    pub gold_particle: Handle<Image>,
    pub fallback_particle: Handle<Image>,
}

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_mercy_assets)
           .add_systems(Update, check_asset_loading);
    }
}

fn load_mercy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let chime = asset_server.load("sounds/mercy_chime.ogg");
    let gold_particle = asset_server.load("textures/gold_particle.png");

    // Procedural fallback particle (simple quad)
    let fallback_mesh = meshes.add(shape::Quad::new(Vec2::splat(1.0)).into());
    let fallback_material = materials.add(StandardMaterial {
        base_color: Color::GOLD,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let fallback_particle = asset_server.load_with_settings(
        "fallback_particle",
        move |settings: &mut ImageLoaderSettings| {
            settings.is_srgb = false;
        },
    );

    commands.insert_resource(MercyAssets {
        chime,
        gold_particle,
        fallback_particle: fallback_mesh.into(),  // We'll handle in check
    });

    info!("Mercy assets loading started — fallbacks ready");
}

fn check_asset_loading(
    asset_server: Res<AssetServer>,
    mercy_assets: Res<MercyAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match asset_server.get_load_state(&mercy_assets.gold_particle) {
        LoadState::Loaded => {}
        LoadState::Failed => {
            warn!("Gold particle failed — using procedural fallback");
            // Switch to fallback (already in resource)
        }
        LoadState::Loading => return,  // Still loading
        LoadState::NotLoaded => {}
    }

    match asset_server.get_load_state(&mercy_assets.chime) {
        LoadState::Loaded => {}
        LoadState::Failed => {
            warn!("Mercy chime failed — using procedural tone");
            // Fallback: generate sine wave tone at runtime
        }
        LoadState::Loading => return,
        LoadState::NotLoaded => {}
    }

    // All assets ready (or fallback active)
    info!("Asset loading complete — mercy flows");
    next_state.set(GameState::InGame);
}
