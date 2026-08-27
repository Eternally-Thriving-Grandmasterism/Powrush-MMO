/*!
 * Local Human Sim — v22.15.0
 *
 * Three practice travelers. Local tick. Peer file for U.
 * Shared inhale if one stands in your pocket.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;

use crate::flow_weather::FlowWeather;
use crate::human_presence::SoftPresence;
use crate::lattice_flow_share::LatticeFlowShareEnvelope;
use crate::mercy_harvest_nodes::NODE_ANCHORS;

const PEER_PATH: &str = "data/powrush_lattice_flow_share_peer.json";
const POCKET: f32 = 5.8;

#[derive(Component)]
struct PracticeTraveler {
    name: &'static str,
    race: &'static str,
    well: usize,
    phase: f32,
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
            .add_systems(Update, (walk_wells, share_inhale, write_practice_peer));
    }
}

fn seed_travelers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Capsule3d::new(0.22, 0.95));
    let roster = [
        ("Mira", "Draek", Color::srgb(0.42, 0.28, 0.55), 0usize),
        ("Ko", "Cydruid", Color::srgb(0.22, 0.48, 0.32), 1),
        ("Ren", "Human", Color::srgb(0.38, 0.34, 0.28), 2),
    ];
    for (name, race, color, well) in roster {
        let mat = materials.add(StandardMaterial {
            base_color: color,
            perceptual_roughness: 0.7,
            ..default()
        });
        let pos = NODE_ANCHORS[well].1 + Vec3::new(1.4, 0.7, 0.6);
        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: mat,
                transform: Transform::from_translation(pos),
                ..default()
            },
            PracticeTraveler {
                name,
                race,
                well,
                phase: well as f32 * 1.7,
            },
            Name::new(name),
        ));
    }
    info!(target: "powrush::sim", "three practice travelers — not a live shard");
}

fn walk_wells(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut PracticeTraveler)>,
) {
    let dt = time.delta_seconds();
    let n = NODE_ANCHORS.len();
    for (mut tf, mut t) in &mut q {
        t.phase += dt * 0.18;
        if t.phase > 1.0 {
            t.phase -= 1.0;
            t.well = (t.well + 1) % n;
        }
        let a = NODE_ANCHORS[t.well].1;
        let b = NODE_ANCHORS[(t.well + 1) % n].1;
        let want = a.lerp(b, t.phase) + Vec3::new(1.2, 0.72, 0.4);
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
    let now = time.elapsed_seconds_f64();
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

fn write_practice_peer(
    time: Res<Time>,
    sim: ResMut<LocalHumanSim>,
    travelers: Query<&PracticeTraveler>,
) {
    let now = time.elapsed_seconds_f64();
    if now - sim.last_peer_write < 18.0 {
        return;
    }
    // ResMut needed to update last_peer_write — take mut via Resource
    let _ = travelers;
}

// split so we can mut sim after the early return pattern
fn write_practice_peer_tick(
    time: Res<Time>,
    mut sim: ResMut<LocalHumanSim>,
) {
    let now = time.elapsed_seconds_f64();
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
    let path = PathBuf::from(PEER_PATH);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(&env) {
        let _ = fs::write(path, json);
    }
}
