/*!
 * Local Human Sim — v22.15.0
 *
 * Three practice travelers. Local tick. Peer file for U.
 * Shared inhale if one stands in your pocket.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;
use bevy::world_serialization::WorldAsset;
use std::time::Duration;

use crate::flow_weather::FlowWeather;
use crate::human_presence::{yaw_toward, SoftPresence};
use crate::title_screen::gltf_integration::{self, asset_file_present, practice_traveler_capsule};
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
    world_assets: Option<Res<Assets<WorldAsset>>>,
) {
    let mesh = meshes.add(Capsule3d::new(0.22, 0.95));
    let roster = [
        ("Mira", Color::srgb(0.42, 0.28, 0.55), 0usize),
        ("Ko", Color::srgb(0.24, 0.36, 0.62), 1),
        ("Ren", Color::srgb(0.38, 0.34, 0.28), 2),
    ];
    for (name, color, well) in roster {
        let mat = materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.7,
            ..default()
        });
        let pos = WELLS[well] + Vec3::new(1.4, 0.7, 0.6);
        let tag = (PracticeTraveler { well, phase: well as f32 * 0.33 }, Name::new(name));
        if asset_file_present(glb(name)) && world_assets.is_some() {
            let rel = glb(name);
            commands.spawn((
                Transform::from_translation(pos),
                Visibility::default(),
                TravelerDress(assets.load(rel)),
                tag.0,
                tag.1,
            ));
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
        if gltfs.get(&dress.0).is_none() {
            continue;
        }
        let rel = glb(name.as_str());
        if rel.is_empty() {
            continue;
        }
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
                .insert(AnimationGraphHandle(gh.clone()))
                .insert(transitions);
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
        let a = WELLS[t.well];
        let b = WELLS[(t.well + 1) % n];
        let want = a.lerp(b, t.phase) + Vec3::new(1.2, 0.72, 0.4);
        let from = tf.translation;
        tf.translation = from.lerp(want, (1.4 * dt).min(1.0));
        if let Some(yaw) = yaw_toward(from, want) {
            tf.rotation = tf.rotation.slerp(Quat::from_rotation_y(yaw), (5.0 * dt).min(1.0));
        }
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
    use bevy::asset::{LoadState, RecursiveDependencyLoadState};
    use bevy::tasks::tick_global_task_pools_on_main_thread;
    use std::{io::Read, path::PathBuf, sync::Mutex};

    static BEVY_ASSET_ROOT_TEST_LOCK: Mutex<()> = Mutex::new(());
    fn dev_asset_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
    }
    fn with_dev_asset_root<R>(f: impl FnOnce() -> R) -> R {
        let save = std::env::var("BEVY_ASSET_ROOT").ok();
        std::env::set_var("BEVY_ASSET_ROOT", dev_asset_root());
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
        .init_asset::<StandardMaterial>()
        .insert_resource(SoftPresence::default())
        .insert_resource(FlowWeather::default())
        .add_plugins(LocalHumanSimPlugin);
        a.finish();
        a.cleanup();
        a
    }
    fn ok() -> bool {
        [("Mira", glb("Mira")), ("Ko", glb("Ko")), ("Ren", glb("Ren"))]
            .iter()
            .all(|(_, r)| asset_file_present(r))
    }
    fn pump_until_gltf(app: &mut App, h: &Handle<Gltf>, frames: u32) -> bool {
        for i in 0..frames {
            app.update();
            tick_global_task_pools_on_main_thread();
            let server = app.world().resource::<AssetServer>();
            match server.get_load_state(h.id()) {
                Some(LoadState::Loaded) => match server.get_recursive_dependency_load_state(h.id()) {
                    Some(RecursiveDependencyLoadState::Loaded) => return true,
                    Some(RecursiveDependencyLoadState::Failed(err)) => {
                        panic!("gltf deps failed: {err:?}")
                    }
                    _ => {}
                },
                Some(LoadState::Failed(err)) => panic!("gltf load failed: {err:?}"),
                _ => {}
            }
        }
        false
    }
    fn run(c: u8) {
        let _env = BEVY_ASSET_ROOT_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        match c {
            1 => {
                let save = std::env::var("BEVY_ASSET_ROOT").ok();
                let root = std::env::temp_dir().join(format!("powrush_f1_{}", std::process::id()));
                let _ = std::fs::remove_dir_all(&root);
                std::fs::create_dir_all(root.join("assets")).unwrap();
                std::env::set_var("BEVY_ASSET_ROOT", &root);
                let mut app = mk();
                app.update();
                for n in ["Mira", "Ko", "Ren"] {
                    assert!(app.world_mut().query::<(&Name, Option<&Mesh3d>)>().iter(app.world()).any(|(name, m)| name.as_str() == n && m.is_some()));
                }
                assert!(app.world_mut().query::<&TravelerDress>().iter(app.world()).next().is_none());
                match save {
                    Some(v) => std::env::set_var("BEVY_ASSET_ROOT", v),
                    None => std::env::remove_var("BEVY_ASSET_ROOT"),
                }
                let _ = std::fs::remove_dir_all(root);
            }
            2 => with_dev_asset_root(|| {
                assert!(ok());
                let mut app = mk();
                app.update();
                for n in ["Mira", "Ko", "Ren"] {
                    assert!(!app.world_mut().query::<(&Name, Option<&Mesh3d>)>().iter(app.world()).any(|(name, m)| name.as_str() == n && m.is_some()));
                }
            }),
            3 => with_dev_asset_root(|| {
                assert!(ok());
                let mut app = mk();
                app.update();
                let h = app
                    .world_mut()
                    .query::<&TravelerDress>()
                    .iter(app.world())
                    .next()
                    .map(|d| d.0.clone())
                    .expect("TravelerDress");
                assert!(
                    pump_until_gltf(&mut app, &h, 2400),
                    "F3: glTF never loaded"
                );
                let g = app.world().resource::<Assets<Gltf>>().get(&h);
                assert!(g.is_some_and(|g| g.named_animations.contains_key(WALK)));
                for _ in 0..480 {
                    app.update();
                    if app
                        .world_mut()
                        .query::<&TravelerWalkReady>()
                        .iter(app.world())
                        .next()
                        .is_some()
                    {
                        return;
                    }
                }
                panic!("F3: Walk never attached");
            }),
            4 => with_dev_asset_root(|| {
                assert!(ok());
                let mut b = Vec::new();
                std::fs::File::open(gltf_integration::bevy_asset_base().join("assets").join(glb("Mira")))
                    .unwrap()
                    .read_to_end(&mut b)
                    .unwrap();
                let t = String::from_utf8_lossy(&b);
                assert!(t.contains("Root") && !t.contains("pelvis") && t.matches("\"name\":").count() >= 62);
            }),
            5 => with_dev_asset_root(|| {
                assert!(ok());
                let mut app = mk();
                app.update();
                let gy = WELLS[0].y + 0.72;
                assert!(app
                    .world_mut()
                    .query_filtered::<&Transform, With<PracticeTraveler>>()
                    .iter(app.world())
                    .any(|t| (t.translation.y - gy).abs() < 0.05));
            }),
            6 => {
                let csv = include_str!("../../assets/asset-licenses.csv");
                for n in ["Mira", "Ko", "Ren"] {
                    assert!(csv.contains(&format!("assets/{}", glb(n))));
                }
            }
            7 => with_dev_asset_root(|| {
                let before = gltf_integration::optional_authored_glb_present();
                let _ = asset_file_present(glb("Mira"));
                let _ = asset_file_present(glb("Ko"));
                let _ = asset_file_present(glb("Ren"));
                assert_eq!(gltf_integration::optional_authored_glb_present(), before);
            }),
            8 => with_dev_asset_root(|| {
                assert_eq!(POCKET, 5.8);
                assert_eq!(PEER_PATH, "data/powrush_lattice_flow_share_peer.json");
                let mut app = mk();
                assert_eq!(app.world().resource::<LocalHumanSim>().last_peer_write, -30.0);
                app.world_mut().resource_mut::<SoftPresence>().position = WELLS[0] + Vec3::new(1.4, 0.7, 0.6);
                app.update();
                assert!(app.world().resource::<LocalHumanSim>().pocket >= 1);
                for _ in 0..1200 {
                    app.update();
                }
                assert!(app.world().resource::<LocalHumanSim>().last_peer_write > -30.0);
            }),
            10 => with_dev_asset_root(|| {
                assert!(ok());
                let mut app = mk();
                for _ in 0..160 {
                    app.update();
                }
                assert!(app
                    .world_mut()
                    .query_filtered::<&Transform, With<PracticeTraveler>>()
                    .iter(app.world())
                    .any(|t| t.rotation.to_euler(EulerRot::YXZ).0.abs() > 0.01));
            }),
            _ => {}
        }
    }
    macro_rules! t { ($c:literal,$n:ident)=>{ #[test] fn $n(){ run($c); } }; }
    t!(1, char_model_f1_missing_glb_keeps_capsule); t!(2, char_model_f2_present_glb_no_double_body); t!(3, char_model_f3_clip_names_resolve); t!(4, char_model_f4_rig_joint_set);
    t!(5, char_model_f5_feet_on_ground); t!(6, char_model_f6_glb_licensed_and_capped); t!(7, char_model_f7_assets_models_scan_unchanged); t!(8, char_model_f8_pocket_and_peer_unchanged); t!(10, char_model_f10_facing_follows_travel);

    #[test]
    fn char_model_boot_default_plugins_no_duplicate_panic() {
        let _env = BEVY_ASSET_ROOT_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_dev_asset_root(|| {
            let mut app = App::new();
            app.add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: None,
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
    fn char_model_asset_dev_layout_matches_bevy_base() {
        let _env = BEVY_ASSET_ROOT_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_dev_asset_root(|| {
            assert_eq!(gltf_integration::bevy_asset_base(), dev_asset_root());
            assert!(ok());
        });
    }

    #[test]
    fn char_model_asset_packaged_layout_under_exe_root() {
        let _env = BEVY_ASSET_ROOT_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let save = std::env::var("BEVY_ASSET_ROOT").ok();
        let pack = std::env::temp_dir().join(format!("powrush_pack_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&pack);
        let assets = pack.join("assets").join("models").join("quaternius-modular-men");
        std::fs::create_dir_all(&assets).unwrap();
        std::fs::copy(
            dev_asset_root().join("assets").join(glb("Mira")),
            assets.join("mira-traveler.glb"),
        )
        .unwrap();
        std::env::set_var("BEVY_ASSET_ROOT", &pack);
        assert!(asset_file_present(glb("Mira")));
        assert!(!asset_file_present(glb("Ko")));
        match save {
            Some(v) => std::env::set_var("BEVY_ASSET_ROOT", v),
            None => std::env::remove_var("BEVY_ASSET_ROOT"),
        }
        let _ = std::fs::remove_dir_all(pack);
    }

    #[test]
    fn char_model_dress_walk_late_gltf_load() {
        let _env = BEVY_ASSET_ROOT_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_dev_asset_root(|| {
            assert!(ok());
            let mut app = mk();
            app.update();
            let h = app
                .world_mut()
                .query::<&TravelerDress>()
                .iter(app.world())
                .next()
                .map(|d| d.0.clone())
                .expect("TravelerDress");
            let mut gltf_frame = None;
            let mut walk_frame = None;
            for i in 0..2400 {
                app.update();
                if gltf_frame.is_none() && app.world().resource::<Assets<Gltf>>().get(&h).is_some() {
                    gltf_frame = Some(i);
                }
                if walk_frame.is_none()
                    && app
                        .world_mut()
                        .query::<&TravelerWalkReady>()
                        .iter(app.world())
                        .next()
                        .is_some()
                {
                    walk_frame = Some(i);
                    break;
                }
            }
            let gltf_frame = gltf_frame.expect("late load: glTF never arrived");
            let walk_frame = walk_frame.expect("late load: Walk never attached");
            assert!(
                walk_frame >= gltf_frame,
                "Walk should attach on or after glTF is resident (gltf@{gltf_frame}, walk@{walk_frame})"
            );
        });
    }
}
