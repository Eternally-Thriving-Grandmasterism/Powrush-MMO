/*!
 * Flow Weather — v22.13.0
 *
 * Hidden band + gold-white ribbon. Never a percentage.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::human_presence::SoftPresence;
use crate::living_body::LivingBody;
use crate::living_freshness::LivingFreshness;

const DROP_SECS: f32 = 0.11;
const BEAD_LIFE: f32 = 0.72;

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
    last_drop: f32,
}

impl Default for FlowWeather {
    fn default() -> Self {
        Self {
            band: FlowBand::Rise,
            chain: 0.0,
            last_drop: 0.0,
        }
    }
}

#[derive(Component)]
struct RibbonBead {
    born: f32,
    handle: Handle<StandardMaterial>,
}

#[derive(Resource)]
struct RibbonKit {
    mesh: Handle<Mesh>,
}

pub struct FlowWeatherPlugin;

impl Plugin for FlowWeatherPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FlowWeather>()
            .add_systems(Startup, seed_ribbon)
            .add_systems(Update, (read_band, drop_beads, fade_beads));
    }
}

fn seed_ribbon(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(RibbonKit {
        mesh: meshes.add(Sphere::new(0.06)),
    });
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
    let quiet = harvest.map(|h| h.harvests_this_session == 0 && h.tends_this_session == 0);
    weather.band = if winded || heavy {
        FlowBand::Anxiety
    } else if quiet.unwrap_or(false) && weather.chain < 1.2 {
        FlowBand::Boredom
    } else if weather.chain > 3.5 && !aging {
        FlowBand::Flow
    } else {
        FlowBand::Rise
    };
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
    let glow = match weather.band {
        FlowBand::Flow => LinearRgba::new(1.0, 0.96, 0.62, 1.0),
        FlowBand::Rise => LinearRgba::new(0.72, 0.88, 0.70, 1.0),
        FlowBand::Anxiety => LinearRgba::new(0.35, 0.40, 0.48, 1.0),
        FlowBand::Boredom => LinearRgba::new(0.28, 0.32, 0.30, 1.0),
    };
    let handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.95, 0.78),
        emissive: glow,
        perceptual_roughness: 0.35,
        ..default()
    });
    let scale = if weather.band == FlowBand::Flow {
        1.15
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
        let t = 1.0 - (age / BEAD_LIFE);
        if let Some(mat) = materials.get_mut(&bead.handle) {
            mat.emissive = mat.emissive * t.max(0.05);
        }
    }
}
