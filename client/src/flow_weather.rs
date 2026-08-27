/*!
 * Flow Weather — v22.14.0
 *
 * Ribbon + hidden band + solo awe inhale + grove nectar current.
 * No Flow % bar. No Horizon pips (H already hides guidance).
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::human_presence::SoftPresence;
use crate::living_body::LivingBody;
use crate::living_freshness::LivingFreshness;
use crate::living_practice_loop::SoftPlayerRealm;

const DROP_SECS: f32 = 0.11;
const BEAD_LIFE: f32 = 0.72;
const AWE_SECS: f32 = 5.2;
const NECTAR_FROM: Vec3 = Vec3::new(-5.2, 1.15, -1.4);
const NECTAR_TO: Vec3 = Vec3::new(4.6, 1.55, 2.8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBand {
    Rise,
    Flow,
    Boredom,
    Anxiety,
}

#[derive(Resource, Debug)]
pub struct FlowWeather {
    pub band: FlowBand,
    pub chain: f32,
    pub awe_until: f64,
    last_harvests: u32,
}

impl Default for FlowWeather {
    fn default() -> Self {
        Self {
            band: FlowBand::Rise,
            chain: 0.0,
            awe_until: 0.0,
            last_harvests: 0,
        }
    }
}

impl FlowWeather {
    pub fn inhaling(&self, now: f64) -> bool {
        now < self.awe_until
    }
}

#[derive(Component)]
struct RibbonBead {
    born: f32,
    handle: Handle<StandardMaterial>,
    glow: LinearRgba,
}

#[derive(Component)]
struct NectarBead;

#[derive(Resource)]
struct RibbonKit {
    mesh: Handle<Mesh>,
}

pub struct FlowWeatherPlugin;

impl Plugin for FlowWeatherPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlowWeather>()
            .add_systems(Startup, (seed_ribbon, seed_nectar))
            .add_systems(
                Update,
                (
                    read_band,
                    maybe_awe,
                    apply_awe_light,
                    drop_beads,
                    fade_beads,
                    ride_nectar,
                    show_nectar,
                ),
            );
    }
}

fn seed_ribbon(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(RibbonKit {
        mesh: meshes.add(Sphere::new(0.06)),
    });
}

fn seed_nectar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(0.09));
    let mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.85, 0.95, 0.70, 0.55),
        emissive: LinearRgba::new(0.35, 0.55, 0.28, 1.0),
        perceptual_roughness: 0.4,
        ..default()
    });
    let delta = NECTAR_TO - NECTAR_FROM;
    for i in 0..9 {
        let t = i as f32 / 8.0;
        let pos = NECTAR_FROM + delta * t;
        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material: mat.clone(),
                transform: Transform::from_translation(pos),
                visibility: Visibility::Hidden,
                ..default()
            },
            NectarBead,
        ));
    }
}

fn read_band(
    body: Option<Res<LivingBody>>,
    fresh: Option<Res<LivingFreshness>>,
    harvest: Option<Res<FirstHarvestEpiphany>>,
    presence: Res<SoftPresence>,
    time: Res<Time>,
    mut weather: ResMut<FlowWeather>,
) {
    let speed = Vec3::new(presence.velocity.x, 0.0, presence.velocity.z).length();
    let moving = speed > 0.4 || !presence.grounded;
    if moving {
        weather.chain = (weather.chain + time.delta_seconds()).min(12.0);
    } else {
        weather.chain = (weather.chain - time.delta_seconds() * 0.55).max(0.0);
    }
    let winded = body.as_ref().map(|b| b.winded).unwrap_or(false);
    let heavy = body.as_ref().map(|b| b.heavy).unwrap_or(false);
    let aging = fresh.map(|f| f.age > 28.0).unwrap_or(false);
    let quiet = harvest
        .map(|h| h.harvests_this_session == 0 && h.tends_this_session == 0)
        .unwrap_or(false);
    weather.band = if winded || heavy {
        FlowBand::Anxiety
    } else if quiet && weather.chain < 1.2 {
        FlowBand::Boredom
    } else if weather.chain > 3.5 && !aging {
        FlowBand::Flow
    } else {
        FlowBand::Rise
    };
}

fn maybe_awe(
    harvest: Option<Res<FirstHarvestEpiphany>>,
    time: Res<Time>,
    mut weather: ResMut<FlowWeather>,
) {
    let Some(h) = harvest else {
        return;
    };
    let total = h.harvests_this_session + h.tends_this_session;
    if total == weather.last_harvests {
        return;
    }
    weather.last_harvests = total;
    if weather.chain >= 2.4 || weather.band == FlowBand::Flow {
        weather.awe_until = time.elapsed_seconds_f64() + AWE_SECS as f64;
        info!(target: "powrush::flow", "solo world inhale");
    }
}

fn apply_awe_light(
    weather: Res<FlowWeather>,
    time: Res<Time>,
    mut ambient: ResMut<AmbientLight>,
) {
    if weather.inhaling(time.elapsed_seconds_f64()) {
        ambient.brightness = (ambient.brightness + 90.0).min(420.0);
    }
}

fn drop_beads(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    kit: Option<Res<RibbonKit>>,
    presence: Res<SoftPresence>,
    weather: Res<FlowWeather>,
    time: Res<Time>,
    mut last: Local<f32>,
) {
    let Some(kit) = kit else {
        return;
    };
    let speed = Vec3::new(presence.velocity.x, 0.0, presence.velocity.z).length();
    if speed < 0.35 && presence.grounded {
        return;
    }
    let now = time.elapsed_seconds();
    if now - *last < DROP_SECS {
        return;
    }
    *last = now;
    let inhaling = weather.inhaling(time.elapsed_seconds_f64());
    let glow = if inhaling {
        LinearRgba::new(1.15, 1.05, 0.55, 1.0)
    } else {
        match weather.band {
            FlowBand::Flow => LinearRgba::new(1.0, 0.96, 0.62, 1.0),
            FlowBand::Rise => LinearRgba::new(0.72, 0.88, 0.70, 1.0),
            FlowBand::Anxiety => LinearRgba::new(0.35, 0.40, 0.48, 1.0),
            FlowBand::Boredom => LinearRgba::new(0.28, 0.32, 0.30, 1.0),
        }
    };
    let handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.95, 0.78),
        emissive: glow,
        perceptual_roughness: 0.35,
        ..default()
    });
    let scale = if inhaling || weather.band == FlowBand::Flow {
        1.2
    } else {
        0.85
    };
    commands.spawn((
        PbrBundle {
            mesh: kit.mesh.clone(),
            material: handle.clone(),
            transform: Transform::from_translation(presence.position - Vec3::Y * 0.55)
                .with_scale(Vec3::splat(scale)),
            ..default()
        },
        RibbonBead {
            born: now,
            handle,
            glow,
        },
    ));
}

fn fade_beads(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &RibbonBead)>,
) {
    let now = time.elapsed_seconds();
    for (entity, bead) in &q {
        let age = now - bead.born;
        if age > BEAD_LIFE {
            commands.entity(entity).despawn_recursive();
            continue;
        }
        let t = (1.0 - age / BEAD_LIFE).clamp(0.04, 1.0);
        if let Some(mat) = materials.get_mut(&bead.handle) {
            mat.emissive = LinearRgba::new(
                bead.glow.red * t,
                bead.glow.green * t,
                bead.glow.blue * t,
                1.0,
            );
        }
    }
}

fn show_nectar(
    realm: Res<SoftPlayerRealm>,
    mut q: Query<&mut Visibility, With<NectarBead>>,
) {
    let show = matches!(realm.current, Some(0) | Some(2));
    for mut vis in &mut q {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn ride_nectar(
    realm: Res<SoftPlayerRealm>,
    mut presence: ResMut<SoftPresence>,
) {
    if !matches!(realm.current, Some(0) | Some(2)) {
        return;
    }
    let p = presence.position;
    let delta = NECTAR_TO - NECTAR_FROM;
    let len2 = delta.length_squared().max(0.01);
    let t = ((p - NECTAR_FROM).dot(delta) / len2).clamp(0.0, 1.0);
    let nearest = NECTAR_FROM + delta * t;
    if p.distance(nearest) > 1.35 {
        return;
    }
    let dir = delta.normalize();
    presence.velocity.x += dir.x * 1.15;
    presence.velocity.z += dir.z * 1.15;
}
