/*!
 * Local Human Sim — v22.15.0
 *
 * Three practice travelers. Local tick. Peer file for U.
 * Shared inhale if one stands in your pocket.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use std::time::Duration;

use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;

#[path = "gltf_integration.rs"]
mod mesh_lod;

use crate::flow_weather::FlowWeather;
use crate::human_presence::SoftPresence;
use crate::lattice_flow_share::LatticeFlowShareEnvelope;
use shared::skirmish_well::WELL_ANCHORS;

const PEER_PATH: &str = "data/powrush_lattice_flow_share_peer.json";
const POCKET: f32 = 5.8;
const WELLS: [Vec3; 3] = [
    Vec3::new(WELL_ANCHORS[0].0, WELL_ANCHORS[0].1, WELL_ANCHORS[0].2),
    Vec3::new(WELL_ANCHORS[1].0, WELL_ANCHORS[1].1, WELL_ANCHORS[1].2),
    Vec3::new(WELL_ANCHORS[2].0, WELL_ANCHORS[2].1, WELL_ANCHORS[2].2),
];
const WALK_CLIP: &str = "CharacterArmature|Walk";
const IDLE_CLIP: &str = "CharacterArmature|Idle";
const FOOT_DROP: f32 = 0.695;

#[derive(Component)]
struct PracticeTraveler {
    well: usize,
    phase: f32,
}

#[derive(Component)]
struct TravelerDress(Handle<Gltf>);

#[derive(Component, Clone)]
struct TravelerAnim {
    graph: Handle<AnimationGraph>,
    walk: AnimationNodeIndex,
}

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
        app.init_resource::<LocalHumanSim>()
            .add_systems(Startup, seed_travelers)
            .add_systems(
                Update,
                (
                    walk_wells,
                    share_inhale,
                    write_practice_peer,
                    traveler_walk_graph,
                ),
            );
    }
}

fn glb_for(name: &str) -> &'static str {
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
    let roster = [
        ("Mira", Color::srgb(0.42, 0.28, 0.55), 0usize),
        ("Ko", Color::srgb(0.24, 0.36, 0.62), 1),
        ("Ren", Color::srgb(0.38, 0.34, 0.28), 2),
    ];
    for (name, color, well) in roster {
        let pos = WELLS[well] + Vec3::new(1.4, 0.7, 0.6);
        let rel = glb_for(name);
        if mesh_lod::asset_file_present(rel) {
            let body: Handle<Gltf> = assets.load(rel);
            commands
                .spawn((
                    Transform::from_translation(pos),
                    Visibility::default(),
                    PracticeTraveler {
                        well,
                        phase: well as f32 * 0.33,
                    },
                    TravelerDress(body),
                    Name::new(name),
                ))
                .with_children(|p| {
                    p.spawn((
                        WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(rel))),
                        Transform::from_xyz(0.0, -FOOT_DROP, 0.0),
                        Visibility::default(),
                    ));
                });
            continue;
        }
        let mat = materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.7,
            ..default()
        });
        commands.spawn((
            (
                Mesh3d(mesh.clone()),
                MeshMaterial3d(mat),
                Transform::from_translation(pos),
            ),
            PracticeTraveler {
                well,
                phase: well as f32 * 0.33,
            },
            Name::new(name),
        ));
    }
    info!(target: "powrush::sim", "three practice travelers — not a live shard");
}

fn traveler_walk_graph(
    mut commands: Commands,
    dressed: Query<(Entity, &TravelerDress), Without<TravelerAnim>>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    children: Query<&Children>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
) {
    for (root, dress) in &dressed {
        let Some(gltf) = gltfs.get(&dress.0) else {
            continue;
        };
        let Some(idle) = gltf.named_animations.get(IDLE_CLIP) else {
            continue;
        };
        let Some(walk) = gltf.named_animations.get(WALK_CLIP) else {
            continue;
        };
        let (graph, nodes) = AnimationGraph::from_clips([idle.clone(), walk.clone()]);
        let Ok([_, walk_ix]) = <[AnimationNodeIndex; 2]>::try_from(nodes) else {
            continue;
        };
        let graph = graphs.add(graph);
        commands.entity(root).insert(TravelerAnim {
            graph: graph.clone(),
            walk: walk_ix,
        });
        for desc in children.iter_descendants(root) {
            let Ok((entity, mut player)) = players.get_mut(desc) else {
                continue;
            };
            let mut transitions = AnimationTransitions::new();
            transitions
                .play(&mut player, walk_ix, Duration::ZERO)
                .repeat();
            commands
                .entity(entity)
                .insert(AnimationGraphHandle(graph.clone()))
                .insert(transitions);
        }
    }
}

fn walk_wells(time: Res<Time>, mut q: Query<(&mut Transform, &mut PracticeTraveler)>) {
    let dt = time.delta_secs();
    let n = WELLS.len();
    for (mut tf, mut t) in &mut q {
        let before = tf.translation;
        t.phase += dt * 0.18;
        if t.phase > 1.0 {
            t.phase -= 1.0;
            t.well = (t.well + 1) % n;
        }
        let want = WELLS[t.well]
            .lerp(WELLS[(t.well + 1) % n], t.phase)
            + Vec3::new(1.2, 0.72, 0.4);
        tf.translation = tf.translation.lerp(want, (1.4 * dt).min(1.0));
        let step = tf.translation - before;
        if step.xz().length_squared() > 1e-6 {
            let yaw = step.x.atan2(step.z);
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
    use bevy::asset::AssetPlugin;
    use std::{io::Read, path::PathBuf};

    fn sim_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .set_error_handler(bevy::ecs::error::ignore)
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .insert_resource(SoftPresence::default())
            .insert_resource(FlowWeather::default())
            .add_plugins(LocalHumanSimPlugin);
        app
    }

    fn dressed() -> bool {
        ["Mira", "Ko", "Ren"].iter().all(|n| mesh_lod::asset_file_present(glb_for(n)))
    }

    fn on_disk(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets").join(rel)
    }

    macro_rules! skip_unless {
        ($ok:expr) => {
            if !$ok {
                return;
            }
        };
    }

    #[test]
    fn char_model_f1_missing_glb_keeps_capsule() {
        skip_unless!(!dressed());
        let mut app = sim_app();
        app.update();
        let mut q = app.world_mut().query::<(&Name, Option<&Mesh3d>)>();
        for (name, mesh) in q.iter(app.world()) {
            if matches!(name.as_str(), "Mira" | "Ko" | "Ren") {
                assert!(mesh.is_some());
            }
        }
    }

    #[test]
    fn char_model_f2_present_glb_no_double_body() {
        skip_unless!(dressed());
        let mut app = sim_app();
        app.update();
        for who in ["Mira", "Ko", "Ren"] {
            let mut root_mesh = 0;
            let mut scene = 0;
            let mut q = app.world_mut().query::<(&Name, Option<&Mesh3d>, Option<&WorldAssetRoot>)>();
            for (n, mesh, scene_root) in q.iter(app.world()) {
                if n.as_str() != who {
                    continue;
                }
                root_mesh += mesh.is_some() as usize;
                scene += scene_root.is_some() as usize;
            }
            assert_eq!(root_mesh, 0);
            assert!(scene >= 1);
        }
    }

    #[test]
    fn char_model_f3_clip_names_resolve() {
        skip_unless!(mesh_lod::asset_file_present(glb_for("Mira")));
        let mut app = sim_app();
        for _ in 0..300 {
            app.update();
            if app.world_mut().query::<&TravelerAnim>().iter(app.world()).next().is_some() {
                return;
            }
        }
        panic!("TravelerAnim missing");
    }

    #[test]
    fn char_model_f4_rig_joint_set() {
        skip_unless!(mesh_lod::asset_file_present(glb_for("Mira")));
        let mut buf = Vec::new();
        std::fs::File::open(on_disk(glb_for("Mira")))
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        let text = String::from_utf8_lossy(&buf);
        assert!(text.contains("Root") && !text.contains("pelvis") && !text.contains("spine_01"));
        assert!(text.matches("\"name\":").count() >= 62);
    }

    #[test]
    fn char_model_f5_feet_on_ground() {
        skip_unless!(dressed());
        let mut app = sim_app();
        app.update();
        let mut q = app.world_mut().query::<(Entity, &Transform, &Children)>();
        let rows: Vec<_> = q
            .iter(app.world())
            .map(|(e, tf, kids)| (*tf, kids.clone(), e))
            .collect();
        for (tf, kids, root) in rows {
            for child in kids.iter() {
                if app.world().get::<WorldAssetRoot>(child).is_none() {
                    continue;
                }
                let foot = tf.translation.y
                    + app
                        .world()
                        .get::<Transform>(child)
                        .map(|c| c.translation.y)
                        .unwrap_or(0.0);
                let _ = root;
                assert!((foot - (tf.translation.y - 0.72)).abs() <= 0.05);
            }
        }
    }

    #[test]
    fn char_model_f6_glb_licensed_and_capped() {
        let csv = include_str!("../../assets/asset-licenses.csv");
        let glbs = [glb_for("Mira"), glb_for("Ko"), glb_for("Ren")];
        let mut sum = 0u64;
        for glb in glbs {
            assert!(csv.contains(&format!("assets/{glb}")));
            if !mesh_lod::asset_file_present(glb) {
                continue;
            }
            let size = std::fs::metadata(on_disk(glb)).unwrap().len();
            sum += size;
            assert!(size <= 1_600_000);
        }
        assert!(sum <= 4_800_000 || sum == 0);
    }

    #[test]
    fn char_model_f8_pocket_and_peer_unchanged() {
        let mut app = sim_app();
        for _ in 0..200 {
            app.update();
        }
        assert!(app.world().resource::<LocalHumanSim>().last_peer_write >= 0.0);
    }

    #[test]
    fn char_model_f10_facing_follows_travel() {
        let mut app = sim_app();
        for _ in 0..240 {
            app.update();
        }
        let dressed = dressed();
        let turned = app
            .world_mut()
            .query_filtered::<&Transform, With<PracticeTraveler>>()
            .iter(app.world())
            .any(|tf| tf.rotation.to_euler(EulerRot::YXZ).0.abs() > 0.01);
        assert!(turned || !dressed);
    }
}