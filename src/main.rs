use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_replicon::prelude::*;

mod assets;
mod chat;
mod combat;
mod housing;
mod npc;
mod quest;
mod ui;
mod voice;
mod weather;
mod world;

use assets::AssetPlugin;
use chat::ChatPlugin;
use combat::CombatPlugin;
use housing::HousingPlugin;
use npc::NPCPlugin;
use quest::QuestPlugin;
use ui::UIPlugin;
use voice::VoicePlugin;
use weather::WeatherPlugin;
use world::WorldPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        console_error_panic_hook::set_once();
        console_log::init_with_level(log::Level::Info).unwrap();
    }

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Powrush-MMO — Eternal Mercy".into(),
                    fit_canvas_to_parent: true,
                    canvas: Some("#bevy-canvas".to_string()),
                    ..default()
                }),
                ..default()
            }),
            AudioPlugin,
            RepliconPlugins,
            AssetPlugin,
            WorldPlugin,
            QuestPlugin,
            HousingPlugin,
            WeatherPlugin,
            CombatPlugin,
            NPCPlugin,
            VoicePlugin,
            ChatPlugin,
            UIPlugin,
        ))
        .insert_resource(LatticeStats::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            mercy_flow_system,
            trust_multiplier_system,
            lattice_expansion_system,
            player_movement_system,
            weather_cycle_system,
            quest_progress_system,
            quest_reward_system,
            npc_ai_system,
            voice_modulation_system,
            chat_input_system,
            chat_send_system,
            chat_render_system,
            ui_update_system,
        ))
        .run();
}

#[derive(Resource, Default)]
pub struct LatticeStats {
    pub nodes: u32,
    pub connections: u32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    info!("Powrush-MMO — Mercy universe spawned");
}

fn mercy_flow_system(time: Res<Time>, mut lattice: ResMut<LatticeStats>) {
    lattice.connections += (time.delta_seconds() * 10.0) as u32;
}

fn trust_multiplier_system(mut query: Query<&mut TrustCredits>) {
    for mut t in &mut query {
        t.0 *= 1.01;
    }
}

#[derive(Component)]
pub struct TrustCredits(pub f32);

fn lattice_expansion_system(mut lattice: ResMut<LatticeStats>, time: Res<Time>) {
    if rand::thread_rng().gen_bool(0.1 * time.delta_seconds() as f64) {
        lattice.nodes += 1;
    }
}

fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut dir = Vec3::ZERO;
    if keyboard.pressed(KeyCode::W) { dir.z -= 1.0; }
    if keyboard.pressed(KeyCode::S) { dir.z += 1.0; }
    if keyboard.pressed(KeyCode::A) { dir.x -= 1.0; }
    if keyboard.pressed(KeyCode::D) { dir.x += 1.0; }

    for mut trans in &mut query {
        trans.translation += dir.normalize_or_zero() * 5.0 * time.delta_seconds();
    }
}
fn mercy_flow_system(time: Res<Time>, mut lattice: ResMut<LatticeStats>) {
    lattice.connections += (time.delta_seconds() * 10.0) as u32;
}

fn trust_multiplier_system(mut query: Query<&mut TrustCredits>) {
    for mut t in &mut query {
        t.0 *= 1.01;
    }
}

#[derive(Component)]
pub struct TrustCredits(pub f32);

fn lattice_expansion_system(mut lattice: ResMut<LatticeStats>, time: Res<Time>) {
    if rand::thread_rng().gen_bool(0.1 * time.delta_seconds() as f64) {
        lattice.nodes += 1;
    }
}

fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut dir = Vec3::ZERO;
    if keyboard.pressed(KeyCode::W) { dir.z -= 1.0; }
    if keyboard.pressed(KeyCode::S) { dir.z += 1.0; }
    if keyboard.pressed(KeyCode::A) { dir.x -= 1.0; }
    if keyboard.pressed(KeyCode::D) { dir.x += 1.0; }

    for mut trans in &mut query {
        trans.translation += dir.normalize_or_zero() * 5.0 * time.delta_seconds();
    }
}
