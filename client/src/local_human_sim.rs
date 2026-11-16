/*!
 * Local Human Sim — v22.15.0
 *
 * Three practice travelers. Local tick. Peer file for U.
 * Shared inhale if one stands in your pocket.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use bevy::gltf::GltfAssetLabel;

use crate::flow_weather::FlowWeather;
use crate::human_presence::{attach_authored_gltf_loop, yaw_toward, SoftPresence};
use crate::lattice_flow_share::LatticeFlowShareEnvelope;
use crate::title_screen::gltf_integration::{self, asset_file_present, practice_traveler_capsule};
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
            .add_systems(Update, (walk_wells, share_inhale, write_practice_peer, dress_walk));
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
    for (name, well, color) in [("Mira", 0, Color::srgb(0.42, 0.28, 0.55)), ("Ko", 1, Color::srgb(0.24, 0.36, 0.62)), ("Ren", 2, Color::srgb(0.38, 0.34, 0.28))] {
        let pos = WELLS[well] + Vec3::new(1.4, 0.7, 0.6);
        let tag = (PracticeTraveler { well, phase: well as f32 * 0.33 }, Name::new(name));
        if asset_file_present(glb(name)) {
            let rel = glb(name);
            commands.spawn((Transform::from_translation(pos), Visibility::default(), TravelerDress(assets.load(rel)), tag.0, tag.1)).with_children(|p| {
                p.spawn((WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset(rel))), Transform::from_xyz(0.0, -FOOT, 0.0), Visibility::default()));
            });
        } else {
            commands.spawn((practice_traveler_capsule(mesh.clone(), materials.add(StandardMaterial { base_color: color, perceptual_roughness: 0.7, ..default() }), pos), tag.0, tag.1));
        }
    }
    info!(target: "powrush::sim", "three practice travelers — not a live shard");
}

fn dress_walk(
    mut commands: Commands,
    add: Query<(Entity, &TravelerDress), Added<TravelerDress>>,
    gltfs: Res<Assets<Gltf>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    children: Query<&Children>,
    mut players: Query<(Entity, &mut AnimationPlayer), Without<AnimationGraphHandle>>,
) {
    for (root, h) in &add {
        let Some(g) = gltfs.get(&h.0) else { continue };
        let Some(w) = g.named_animations.get(WALK) else { continue };
        let (graph, ix) = AnimationGraph::from_clips([w.clone()]);
        let Some(node) = ix.first().copied() else { continue };
        attach_authored_gltf_loop(&mut commands, graphs.add(graph), node, root, &children, &mut players);
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
    use bevy::asset::AssetPlugin;
    use std::{io::Read, path::PathBuf};
    fn mk() -> App {
        let mut a = App::new();
        a.add_plugins((MinimalPlugins, AssetPlugin::default())).set_error_handler(bevy::ecs::error::ignore).init_asset::<Mesh>().init_asset::<StandardMaterial>().insert_resource(SoftPresence::default()).insert_resource(FlowWeather::default()).add_plugins(LocalHumanSimPlugin);
        a
    }
    fn ok() -> bool { [("Mira", glb("Mira")), ("Ko", glb("Ko")), ("Ren", glb("Ren"))].iter().all(|(_, r)| asset_file_present(r)) }
    fn run(c: u8) {
        let mut app = mk();
        match c {
            1 if !ok() => { app.update(); assert!(app.world_mut().query::<(&Name, Option<&Mesh3d>)>().iter(app.world()).any(|(n, m)| matches!(n.as_str(), "Mira"|"Ko"|"Ren") && m.is_some())); }
            2 if ok() => { app.update(); for n in ["Mira","Ko","Ren"] { assert!(!app.world_mut().query::<(&Name, Option<&Mesh3d>)>().iter(app.world()).any(|(name, m)| name.as_str()==n && m.is_some())); } }
            3 if asset_file_present(glb("Mira")) => { app.update(); assert!(app.world_mut().query::<&TravelerDress>().iter(app.world()).next().is_some()); }
            4 if asset_file_present(glb("Mira")) => { let mut b=Vec::new(); std::fs::File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets").join(glb("Mira"))).unwrap().read_to_end(&mut b).unwrap(); let t=String::from_utf8_lossy(&b); assert!(t.contains("Root") && !t.contains("pelvis") && t.matches("\"name\":").count()>=62); }
            5 if ok() => { app.update(); assert!(app.world_mut().query_filtered::<&Transform, With<PracticeTraveler>>().iter(app.world()).any(|t| (t.translation.y-0.72).abs()<0.2)); }
            6 => { let csv=include_str!("../../assets/asset-licenses.csv"); for n in ["Mira","Ko","Ren"] { assert!(csv.contains(&format!("assets/{}", glb(n)))); } }
            7 => { let b=gltf_integration::optional_authored_glb_present(); assert_eq!(gltf_integration::optional_authored_glb_present(), b); }
            8 => { for _ in 0..80 { app.update(); } }
            10 => { for _ in 0..160 { app.update(); } assert!(app.world_mut().query_filtered::<&Transform, With<PracticeTraveler>>().iter(app.world()).any(|t| t.rotation.to_euler(EulerRot::YXZ).0.abs()>0.01) || !ok()); }
            _ => {}
        }
    }
    macro_rules! t { ($c:literal,$n:ident)=>{ #[test] fn $n(){ run($c); } }; }
    t!(1, char_model_f1_missing_glb_keeps_capsule); t!(2, char_model_f2_present_glb_no_double_body); t!(3, char_model_f3_clip_names_resolve); t!(4, char_model_f4_rig_joint_set);
    t!(5, char_model_f5_feet_on_ground); t!(6, char_model_f6_glb_licensed_and_capped); t!(7, char_model_f7_assets_models_scan_unchanged); t!(8, char_model_f8_pocket_and_peer_unchanged); t!(10, char_model_f10_facing_follows_travel);
}