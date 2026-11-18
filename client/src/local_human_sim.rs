/*!
 * Local Human Sim — v22.15.0
 *
 * Three practice travelers. Local tick. Peer file for U.
 * Shared inhale if one stands in your pocket.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;
use bevy::world_serialization::WorldAssetRoot;
use std::time::Duration;

use crate::flow_weather::FlowWeather;
use crate::human_presence::SoftPresence;
use crate::title_screen::gltf_integration::{asset_file_present, practice_traveler_capsule};
use crate::lattice_flow_share::LatticeFlowShareEnvelope;
use shared::skirmish_well::WELL_ANCHORS;

const PEER_PATH: &str = "data/powrush_lattice_flow_share_peer.json";
const POCKET: f32 = 5.8;
const WALK: &str = "CharacterArmature|Walk";
const FOOT: f32 = 0.695;
const WELLS: [Vec3; 3] = [
    Vec3::new(WELL_ANCHORS[0].0, WELL_ANCHORS[0].1, WELL_ANCHORS[0].2),
    Vec3::new(WELL_ANCHORS[1].0, WELL_ANCHORS[1].1, WELL_ANCHORS[1].2),
    Vec3::new(WELL_ANCHORS[2].0, WELL_ANCHORS[2].1, WELL_ANCHORS[2].2),
];

#[derive(Component)]
struct PracticeTraveler {
    well: usize,
    phase: f32,
}

#[derive(Component)]
pub struct TravelerDress(Handle<Gltf>);

#[derive(Component)]
struct TravelerWalkReady;

#[derive(Component)]
struct TravelerSceneSpawned;

#[derive(Resource, Debug)]
pub struct LocalHumanSim {
    pub active: bool,
    pub pocket: u32,
    pub last_peer_write: f64,
}

impl Default for LocalHumanSim {
    fn default() -> Self {
        Self {
            active: true,
            pocket: 0,
            last_peer_write: -30.0,
        }
    }
}

pub struct LocalHumanSimPlugin;

impl Plugin for LocalHumanSimPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Gltf>()
            .init_resource::<LocalHumanSim>()
            .add_systems(Startup, seed_travelers)
            .add_systems(Update, (walk_wells, share_inhale, write_practice_peer))
            .add_systems(Update, (attach_traveler_scene, dress_walk).chain());
    }
}

fn glb(name: &str) -> &'static str {
    match name {
        "Mira" => "models/quaternius-modular-men/mira-traveler.glb",
        "Ko" => "models/quaternius-modular-men/ko-traveler.glb",
        "Ren" => "models/quaternius-modular-men/ren-traveler.glb",
        _ => "",
    }
}

fn seed_travelers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let mesh = meshes.add(Capsule3d::new(0.22, 0.95));
    for (name, color, well) in [
        ("Mira", Color::srgb(0.42, 0.28, 0.55), 0usize),
        ("Ko", Color::srgb(0.24, 0.36, 0.62), 1),
        ("Ren", Color::srgb(0.38, 0.34, 0.28), 2),
    ] {
        let mat = materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.7,
            ..default()
        });
        let pos = WELLS[well] + Vec3::new(1.4, 0.7, 0.6);
        let tag = (PracticeTraveler { well, phase: well as f32 * 0.33 }, Name::new(name));
        if asset_file_present(glb(name)) {
            let rel = glb(name);
            commands.spawn((Transform::from_translation(pos), Visibility::default(), TravelerDress(assets.load(rel)), tag.0, tag.1));
        } else {
            commands.spawn((practice_traveler_capsule(mesh.clone(), mat, pos), tag.0, tag.1));
        }
    }
    info!(target: "powrush::sim", "three practice travelers — not a live shard");
}

fn attach_traveler_scene(
    mut commands: Commands,
    assets: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    q: Query<(Entity, &TravelerDress, &Name), Without<TravelerSceneSpawned>>,
) {
    for (root, dress, name) in &q {
        if gltfs.get(&dress.0).is_none() || glb(name.as_str()).is_empty() {
            continue;
        }
        let rel = glb(name.as_str());
        commands
            .entity(root)
            .with_children(|p| {
                p.spawn((
                    WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(rel))),
                    Transform::from_xyz(0.0, -FOOT, 0.0),
                    Visibility::default(),
                ));
            })
            .insert(TravelerSceneSpawned);
    }
}

fn dress_walk(
    mut commands: Commands,
    pending: Query<(Entity, &TravelerDress), Without<TravelerWalkReady>>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    children: Query<&Children>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
) {
    for (root, h) in &pending {
        let Some(g) = gltfs.get(&h.0) else { continue };
        let Some(w) = g.named_animations.get(WALK) else { continue };
        let (graph, ix) = AnimationGraph::from_clips([w.clone()]);
        let Some(node) = ix.first().copied() else { continue };
        let gh = graphs.add(graph);
        let mut attached = false;
        for desc in children.iter_descendants(root) {
            let Ok((entity, mut player)) = players.get_mut(desc) else { continue };
            let mut transitions = AnimationTransitions::new();
            transitions.play(&mut player, node, Duration::ZERO).repeat();
            commands
                .entity(entity)
                .insert((AnimationGraphHandle(gh.clone()), transitions));
            attached = true;
        }
        if attached {
            commands.entity(root).insert(TravelerWalkReady);
        }
    }
}

fn walk_wells(time: Res<Time>, mut q: Query<(&mut Transform, &mut PracticeTraveler)>) {
    let dt = time.delta_secs();
    let n = WELLS.len();
    for (mut tf, mut t) in &mut q {
        t.phase += dt * 0.18;
        if t.phase > 1.0 {
            t.phase -= 1.0;
            t.well = (t.well + 1) % n;
        }
        let want = WELLS[t.well].lerp(WELLS[(t.well + 1) % n], t.phase) + Vec3::new(1.2, 0.72, 0.4);
        tf.translation = tf.translation.lerp(want, (1.4 * dt).min(1.0));
    }
}

fn share_inhale(
    presence: Res<SoftPresence>,
    time: Res<Time>,
    mut weather: ResMut<FlowWeather>,
    mut sim: ResMut<LocalHumanSim>,
    travelers: Query<&Transform, With<PracticeTraveler>>,
) {
    let now = time.elapsed_secs_f64();
    let mut pocket = 0u32;
    for tf in &travelers {
        if tf.translation.distance(presence.position) <= POCKET {
            pocket += 1;
        }
    }
    sim.pocket = pocket;
    if pocket > 0 && weather.inhaling(now) {
        weather.awe_until = weather.awe_until.max(now + 2.2);
    }
}

fn write_practice_peer(time: Res<Time>, mut sim: ResMut<LocalHumanSim>) {
    let now = time.elapsed_secs_f64();
    if now - sim.last_peer_write < 18.0 {
        return;
    }
    sim.last_peer_write = now;
    let env = LatticeFlowShareEnvelope {
        schema: "powrush_lattice_flow_share_v1".into(),
        flow_total: 2.0 + (now as f32 % 5.0) * 0.15,
        reserve_total: 1.1,
        choices_made: 1 + (now as u32 / 18),
        last_path: Some("Practice traveler — Mira".into()),
        mercy_note: "Practice file — not a live shard".into(),
        exported_at_secs: now,
    };
    if let Ok(json) = serde_json::to_string_pretty(&env) {
        let _ = shared::user_persist::write_named(PEER_PATH, json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::title_screen::gltf_integration;
    use bevy::asset::{LoadState, RecursiveDependencyLoadState};
    use bevy::render::settings::WgpuSettings;
    use bevy::tasks::tick_global_task_pools_on_main_thread;
    use std::{path::PathBuf, sync::Mutex};

    static LOCK: Mutex<()> = Mutex::new(());
    fn dev_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
    }
    fn with_root<R>(root: &PathBuf, f: impl FnOnce() -> R) -> R {
        let save = std::env::var("BEVY_ASSET_ROOT").ok();
        std::env::set_var("BEVY_ASSET_ROOT", root);
        let out = f();
        match save {
            Some(v) => std::env::set_var("BEVY_ASSET_ROOT", v),
            None => std::env::remove_var("BEVY_ASSET_ROOT"),
        }
        out
    }
    fn mk() -> App {
        let mut a = App::new();
        a.add_plugins((
            MinimalPlugins,
            AssetPlugin {
                watch_for_changes_override: Some(false),
                use_asset_processor_override: Some(false),
                ..default()
            },
            bevy::scene::ScenePlugin,
            bevy::image::ImagePlugin::default(),
            bevy::mesh::MeshPlugin,
            bevy::animation::AnimationPlugin,
            bevy::gltf::GltfPlugin::default(),
            bevy::world_serialization::WorldSerializationPlugin,
        ))
        .set_error_handler(bevy::ecs::error::ignore)
        .init_asset::<Mesh>()
        .init_asset::<StandardMaterial>()
        .insert_resource(SoftPresence::default())
        .insert_resource(FlowWeather::default())
        .add_plugins(LocalHumanSimPlugin);
        a.finish();
        a.cleanup();
        a
    }
    fn boot(app: &mut App) {
        app.world_mut().run_schedule(Startup);
        app.update();
    }

    #[test]
    fn char_model_boot_default_plugins_no_duplicate_panic() {
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_root(&dev_root(), || {
            let mut app = App::new();
            app.add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: None,
                        exit_condition: bevy::window::ExitCondition::DontExit,
                        ..default()
                    })
                    .set(bevy::render::RenderPlugin {
                        render_creation: WgpuSettings {
                            backends: None,
                            ..default()
                        }
                        .into(),
                        ..default()
                    })
                    .disable::<bevy::audio::AudioPlugin>()
                    .disable::<bevy::winit::WinitPlugin>(),
            )
            .insert_resource(SoftPresence::default())
            .insert_resource(FlowWeather::default())
            .add_plugins(LocalHumanSimPlugin);
            app.finish();
            app.cleanup();
            app.world_mut().run_schedule(Startup);
        });
    }

    #[test]
    fn char_model_bevy_asset_roots_and_spawn() {
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let empty = std::env::temp_dir().join(format!("pe_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&empty);
        std::fs::create_dir_all(empty.join("assets")).unwrap();
        with_root(&empty, || {
            let mut app = mk();
            boot(&mut app);
            assert!(app.world_mut().query::<&TravelerDress>().iter(app.world()).next().is_none());
            for n in ["Mira", "Ko", "Ren"] {
                assert!(app.world_mut().query::<(&Name, Option<&Mesh3d>)>().iter(app.world()).any(|(name, m)| name.as_str() == n && m.is_some()));
            }
        });
        let _ = std::fs::remove_dir_all(&empty);
        with_root(&dev_root(), || {
            assert_eq!(gltf_integration::bevy_asset_base(), dev_root());
            assert!(["Mira", "Ko", "Ren"].iter().all(|n| asset_file_present(glb(n))));
            let before = gltf_integration::optional_authored_glb_present();
            let mut app = mk();
            boot(&mut app);
            for n in ["Mira", "Ko", "Ren"] {
                let _ = asset_file_present(glb(n));
            }
            assert_eq!(gltf_integration::optional_authored_glb_present(), before);
            let csv = include_str!("../../assets/asset-licenses.csv");
            for n in ["Mira", "Ko", "Ren"] {
                assert!(csv.contains(&format!("assets/{}", glb(n))));
            }
            assert!(app.world_mut().query::<&TravelerDress>().iter(app.world()).nth(2).is_some());
            assert!(!app.world_mut().query::<(&Name, Option<&Mesh3d>)>().iter(app.world()).any(|(n, m)| ["Mira", "Ko", "Ren"].contains(&n.as_str()) && m.is_some()));
        });
        let save = std::env::var("BEVY_ASSET_ROOT").ok();
        let pack = std::env::temp_dir().join(format!("pp_{}", std::process::id()));
        std::fs::create_dir_all(pack.join("assets/models/quaternius-modular-men")).unwrap();
        std::fs::copy(dev_root().join("assets").join(glb("Mira")), pack.join("assets/models/quaternius-modular-men/mira-traveler.glb")).unwrap();
        std::env::set_var("BEVY_ASSET_ROOT", &pack);
        assert!(asset_file_present(glb("Mira")) && !asset_file_present(glb("Ko")));
        match save {
            Some(v) => std::env::set_var("BEVY_ASSET_ROOT", v),
            None => std::env::remove_var("BEVY_ASSET_ROOT"),
        }
        let _ = std::fs::remove_dir_all(pack);
    }

    #[test]
    fn char_model_gltf_walk_lane() {
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_root(&dev_root(), || {
            let mut app = mk();
            boot(&mut app);
            let h = app.world_mut().query::<&TravelerDress>().iter(app.world()).next().unwrap().0.clone();
            for _ in 0..2400 {
                app.update();
                tick_global_task_pools_on_main_thread();
                let s = app.world().resource::<AssetServer>();
                if matches!(s.get_load_state(h.id()), Some(LoadState::Loaded))
                    && matches!(s.get_recursive_dependency_load_state(h.id()), Some(RecursiveDependencyLoadState::Loaded))
                {
                    break;
                }
            }
            assert!(app.world().resource::<Assets<Gltf>>().get(&h).unwrap().named_animations.contains_key(WALK));
            let mut gf = None;
            let mut wf = None;
            for i in 0..480 {
                app.update();
                if gf.is_none() && app.world().resource::<Assets<Gltf>>().get(&h).is_some() {
                    gf = Some(i);
                }
                if app.world_mut().query::<&TravelerWalkReady>().iter(app.world()).next().is_some() {
                    wf = Some(i);
                    break;
                }
            }
            assert!(wf.expect("walk") >= gf.expect("gltf"));
        });
    }

    #[test]
    fn char_model_f8_pocket_and_peer_writes() {
        let _g = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_root(&dev_root(), || {
            let mut app = mk();
            boot(&mut app);
            app.world_mut().resource_mut::<SoftPresence>().position = WELLS[0] + Vec3::new(1.4, 0.7, 0.6);
            app.update();
            assert!(app.world().resource::<LocalHumanSim>().pocket >= 1);
            for _ in 0..1200 {
                app.update();
            }
            assert!(app.world().resource::<LocalHumanSim>().last_peer_write > -30.0);
        });
    }
}