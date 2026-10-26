//! CARD VP-VFX-1 — capped visual feedback for the existing Tend and Flow answers.
//!
//! Reads [`WorldAnswer`] only (fired by `resolve_tend` and the R→1 Flow allocate).
//! Never writes it, so what Tend / Flow do — gameplay, timing, cooldowns,
//! networking — is unchanged. No lighting, bloom or grade is touched.
//!
//! Each fire spawns a short burst of small unlit-emissive motes in the lower
//! third of the view (never over screen centre, where the Use glow and plates
//! read). Motes rise, shrink and despawn. Live motes are hard-capped per
//! [`GraphicsPreset`] by the named `VFX_CAP_*` constants; Mobile is off,
//! Low is a minimal variant, High == Ultra (Ultra adds nothing).

use bevy::prelude::*;
use shared::local_settings::GraphicsPreset;

use crate::local_settings::LocalSettingsState;
use crate::mercy_harvest_nodes::MercyHarvestNode;
use crate::world_answer::{AnswerKind, WorldAnswer};

/// Mobile: off.
pub const VFX_CAP_MOBILE: usize = 0;
/// Low: minimal variant.
pub const VFX_CAP_LOW: usize = 6;
pub const VFX_CAP_MEDIUM: usize = 14;
pub const VFX_CAP_HIGH: usize = 24;
/// Ultra only equals High — no Ultra-only stack.
pub const VFX_CAP_ULTRA: usize = VFX_CAP_HIGH;
/// Most motes one Tend / Flow fire may add (further clamped by the cap).
pub const VFX_BURST_TEND: usize = 6;
pub const VFX_BURST_FLOW: usize = 10;
pub const VFX_LIFE_TEND_SECS: f32 = 1.6;
pub const VFX_LIFE_FLOW_SECS: f32 = 2.2;

/// Mote mesh radius (world units).
pub const VFX_MOTE_RADIUS: f32 = 0.09;
/// Harvest-node (Use glow) sphere radius at scale 1 (`mercy_harvest_nodes`).
pub const GLOW_BASE_RADIUS: f32 = 0.48;
/// Clearance kept around every glow, as a multiple of its drawn radius.
pub const GLOW_CLEARANCE_FACTOR: f32 = 1.6;
/// Rise speeds (world units / s).
pub const VFX_RISE_TEND: f32 = 0.6;
pub const VFX_RISE_FLOW: f32 = 0.9;

/// A Use-glow source: world centre and clearance radius (already padded).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlowClear {
    pub center: Vec3,
    pub radius: f32,
}

/// Clearance sphere for a harvest-node glow drawn at `scale`.
pub fn glow_clear(center: Vec3, scale: f32) -> GlowClear {
    GlowClear {
        center,
        radius: GLOW_BASE_RADIUS * scale.max(0.0) * GLOW_CLEARANCE_FACTOR,
    }
}

/// True when a mote at `p`, seen from `eye`, keeps clear of the glow on screen:
/// the angular gap between the mote and the glow centre exceeds both angular
/// radii (mote + padded glow). Screen-space, so a near mote can never sit
/// over a far glow. Glows behind the eye are ignored.
pub fn mote_clear_of(eye: Vec3, p: Vec3, glow: GlowClear) -> bool {
    let to_m = p - eye;
    let to_g = glow.center - eye;
    let (dm, dg) = (to_m.length(), to_g.length());
    if dg <= glow.radius || dm <= f32::EPSILON {
        return dg > glow.radius && dm > f32::EPSILON; // eye inside a glow: nothing is clear
    }
    let gap = to_m.angle_between(to_g);
    let ang_g = (glow.radius / dg).clamp(-1.0, 1.0).asin();
    let ang_m = (VFX_MOTE_RADIUS / dm).clamp(-1.0, 1.0).asin();
    gap > ang_g + ang_m
}

/// Whole rising path (start → start + up·rise·life) stays clear of every glow.
pub fn mote_path_clear(eye: Vec3, start: Vec3, rise_total: f32, glows: &[GlowClear]) -> bool {
    const STEPS: usize = 32;
    (0..=STEPS).all(|k| {
        let p = start + Vec3::Y * rise_total * (k as f32 / STEPS as f32);
        glows.iter().all(|g| mote_clear_of(eye, p, *g))
    })
}

/// Pure placement: up to `n` mote starts in the lower view band, beside centre,
/// each with a whole path clear of every glow. Blocked slots are skipped
/// (shifted to the next candidate); never more than `n`.
pub fn plan_mote_starts(
    eye: Vec3,
    forward: Vec3,
    right: Vec3,
    cam_up: Vec3,
    n: usize,
    rise_total: f32,
    glows: &[GlowClear],
) -> Vec<Vec3> {
    let mut out = Vec::with_capacity(n);
    if n == 0 {
        return out;
    }
    // Candidate slots: preferred band first, then lower / wider / nearer fallbacks.
    const DEPTHS: [f32; 2] = [4.5, 3.4];
    const DROPS: [f32; 3] = [1.1, 1.45, 0.8];
    const LATERAL: [f32; 8] = [0.9, 1.15, 1.4, 1.65, 1.9, 2.2, 2.5, 2.8];
    'outer: for depth in DEPTHS {
        for drop in DROPS {
            let origin = eye + forward * depth - cam_up * drop;
            for (j, lat) in LATERAL.iter().enumerate() {
                for side in [-1.0f32, 1.0] {
                    let p = origin + right * side * *lat + Vec3::Y * (0.02 * j as f32);
                    if mote_path_clear(eye, p, rise_total, glows)
                        && out.iter().all(|q: &Vec3| q.distance(p) > 2.0 * VFX_MOTE_RADIUS)
                    {
                        out.push(p);
                        if out.len() == n {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    out
}

/// Hard ceiling on live motes for a preset.
pub const fn vfx_cap(preset: GraphicsPreset) -> usize {
    match preset {
        GraphicsPreset::Mobile => VFX_CAP_MOBILE,
        GraphicsPreset::Low => VFX_CAP_LOW,
        GraphicsPreset::Medium => VFX_CAP_MEDIUM,
        GraphicsPreset::High => VFX_CAP_HIGH,
        GraphicsPreset::Ultra => VFX_CAP_ULTRA,
    }
}

/// Burst size for one answer fire. Only Tend and Flow draw VFX.
pub const fn vfx_burst(kind: AnswerKind) -> usize {
    match kind {
        AnswerKind::Tend => VFX_BURST_TEND,
        AnswerKind::Flow => VFX_BURST_FLOW,
        _ => 0,
    }
}

/// How many motes may spawn now without exceeding the cap.
pub const fn vfx_spawn_budget(preset: GraphicsPreset, kind: AnswerKind, live: usize) -> usize {
    let room = vfx_cap(preset).saturating_sub(live);
    let burst = vfx_burst(kind);
    if burst < room { burst } else { room }
}

#[derive(Component, Debug)]
pub struct TendFlowMote {
    age: f32,
    life: f32,
    rise: f32,
    base_scale: f32,
}

#[derive(Resource, Default)]
struct MoteAssets {
    mesh: Option<Handle<Mesh>>,
    tend: Option<Handle<StandardMaterial>>,
    flow: Option<Handle<StandardMaterial>>,
}

pub struct TendFlowVfxPlugin;

impl Plugin for TendFlowVfxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MoteAssets>()
            .add_systems(Update, (spawn_tend_flow_motes, age_tend_flow_motes).chain());
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_tend_flow_motes(
    mut commands: Commands,
    answer: Option<Res<WorldAnswer>>,
    settings: Option<Res<LocalSettingsState>>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
    live: Query<(), With<TendFlowMote>>,
    glows: Query<&GlobalTransform, With<MercyHarvestNode>>,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut assets: ResMut<MoteAssets>,
    mut seen_until: Local<f64>,
) {
    let Some(answer) = answer else { return };
    // A new fire always moves `until`; read-only edge detect.
    if answer.until == *seen_until {
        return;
    }
    *seen_until = answer.until;
    let preset = settings
        .map(|s| s.inner.graphics_preset)
        .unwrap_or(GraphicsPreset::Medium);
    let n = vfx_spawn_budget(preset, answer.kind, live.iter().count());
    if n == 0 {
        return;
    }
    let flow = answer.kind == AnswerKind::Flow;

    if let (Some(mut meshes), Some(mut materials)) = (meshes, materials) {
        if assets.mesh.is_none() {
            assets.mesh = Some(meshes.add(Sphere::new(VFX_MOTE_RADIUS).mesh().ico(1).unwrap()));
            assets.tend = Some(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.82, 0.45),
                emissive: LinearRgba::rgb(2.2, 1.5, 0.55),
                unlit: true,
                ..default()
            }));
            assets.flow = Some(materials.add(StandardMaterial {
                base_color: Color::srgb(0.7, 0.92, 0.95),
                emissive: LinearRgba::rgb(0.9, 1.6, 1.7),
                unlit: true,
                ..default()
            }));
        }
    }

    let rise = if flow { VFX_RISE_FLOW } else { VFX_RISE_TEND };
    let life = if flow { VFX_LIFE_FLOW_SECS } else { VFX_LIFE_TEND_SECS };
    let clears: Vec<GlowClear> = glows
        .iter()
        .map(|g| {
            let (scale, _, t) = g.to_scale_rotation_translation();
            glow_clear(t, scale.max_element())
        })
        .collect();
    let cam = cameras.iter().next().copied().unwrap_or_default();
    let starts = plan_mote_starts(
        cam.translation(),
        cam.forward().as_vec3(),
        cam.right().as_vec3(),
        cam.up().as_vec3(),
        n,
        rise * life,
        &clears,
    );
    debug!(target: "powrush::vfx", ?preset, n, placed = starts.len(), "tend/flow motes");
    let count = starts.len();
    for (i, pos) in starts.into_iter().enumerate() {
        let t = i as f32 / count.max(1) as f32;
        let mote = TendFlowMote {
            age: 0.0,
            life,
            rise,
            base_scale: 1.0 - 0.3 * t,
        };
        let mut e = commands.spawn((mote, Transform::from_translation(pos), Visibility::default()));
        if let Some(mesh) = assets.mesh.clone() {
            e.insert(Mesh3d(mesh));
        }
        let mat = if flow { assets.flow.clone() } else { assets.tend.clone() };
        if let Some(mat) = mat {
            e.insert(MeshMaterial3d(mat));
        }
    }
}

fn age_tend_flow_motes(
    mut commands: Commands,
    time: Res<Time>,
    mut motes: Query<(Entity, &mut TendFlowMote, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (e, mut m, mut tf) in &mut motes {
        m.age += dt;
        if m.age >= m.life {
            commands.entity(e).despawn();
            continue;
        }
        let k = 1.0 - m.age / m.life;
        tf.translation.y += m.rise * dt;
        tf.scale = Vec3::splat(m.base_scale * k);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [GraphicsPreset; 5] = [
        GraphicsPreset::Mobile,
        GraphicsPreset::Low,
        GraphicsPreset::Medium,
        GraphicsPreset::High,
        GraphicsPreset::Ultra,
    ];

    #[test]
    fn caps_are_ordered_and_ultra_does_not_stack() {
        assert_eq!(vfx_cap(GraphicsPreset::Mobile), 0);
        assert!(vfx_cap(GraphicsPreset::Low) <= 6, "Low stays minimal");
        assert!(vfx_cap(GraphicsPreset::Low) < vfx_cap(GraphicsPreset::Medium));
        assert!(vfx_cap(GraphicsPreset::Medium) <= vfx_cap(GraphicsPreset::High));
        assert_eq!(vfx_cap(GraphicsPreset::High), vfx_cap(GraphicsPreset::Ultra));
    }

    #[test]
    fn only_tend_and_flow_draw() {
        for kind in [AnswerKind::Idle, AnswerKind::Take, AnswerKind::Reserve] {
            assert_eq!(vfx_spawn_budget(GraphicsPreset::Ultra, kind, 0), 0);
        }
        assert!(vfx_spawn_budget(GraphicsPreset::Medium, AnswerKind::Tend, 0) > 0);
        assert!(vfx_spawn_budget(GraphicsPreset::Medium, AnswerKind::Flow, 0) > 0);
    }

    fn app_with(preset: GraphicsPreset) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        let mut s = LocalSettingsState::default();
        s.inner.graphics_preset = preset;
        app.insert_resource(s)
            .init_resource::<WorldAnswer>()
            .add_plugins(TendFlowVfxPlugin);
        app
    }

    fn live(app: &mut App) -> usize {
        app.world_mut()
            .query::<&TendFlowMote>()
            .iter(app.world())
            .count()
    }

    #[test]
    fn cap_holds_under_repeated_triggers() {
        for preset in ALL {
            let mut app = app_with(preset);
            let cap = vfx_cap(preset);
            let mut peak = 0;
            for i in 0..200 {
                let kind = if i % 2 == 0 { AnswerKind::Tend } else { AnswerKind::Flow };
                app.world_mut()
                    .resource_mut::<WorldAnswer>()
                    .fire(kind, 1.0 + i as f64, "t");
                app.update();
                let n = live(&mut app);
                assert!(n <= cap, "{preset:?}: {n} live > cap {cap}");
                peak = peak.max(n);
            }
            assert_eq!(peak, cap, "{preset:?} should reach exactly its cap");
            if preset == GraphicsPreset::Mobile {
                assert_eq!(peak, 0, "Mobile is off");
            }
        }
    }

    #[test]
    fn motes_never_land_inside_a_glow_clearance() {
        let eye = Vec3::new(0.0, 3.0, 8.0);
        let mut seed = 0x9e3779b9u32;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            (seed as f32 / u32::MAX as f32) * 2.0 - 1.0
        };
        let mut placed_total = 0;
        for case in 0..400 {
            let look = Vec3::new(rnd() * 2.0, -0.4 + rnd() * 0.3, -1.0).normalize();
            let tf = Transform::from_translation(eye).looking_to(look, Vec3::Y);
            let (fwd, right, up) = (tf.forward().as_vec3(), tf.right().as_vec3(), tf.up().as_vec3());
            // Glows scattered in the lower view, including right where the band sits.
            let glows: Vec<GlowClear> = (0..1 + case % 4)
                .map(|_| {
                    let c = eye + fwd * (3.0 + 6.0 * rnd().abs()) + right * rnd() * 3.0 - up * (0.6 + rnd().abs());
                    glow_clear(c, 0.9 + 0.4 * rnd().abs())
                })
                .collect();
            for (rise, n) in [(VFX_RISE_TEND * VFX_LIFE_TEND_SECS, VFX_BURST_TEND), (VFX_RISE_FLOW * VFX_LIFE_FLOW_SECS, VFX_BURST_FLOW)] {
                let starts = plan_mote_starts(eye, fwd, right, up, n, rise, &glows);
                assert!(starts.len() <= n);
                placed_total += starts.len();
                for p in starts {
                    for k in 0..=32 {
                        let q = p + Vec3::Y * rise * k as f32 / 32.0;
                        for g in &glows {
                            assert!(q.distance(g.center) > g.radius, "inside world clearance");
                            assert!(mote_clear_of(eye, q, *g), "overlaps glow on screen");
                        }
                    }
                }
            }
        }
        assert!(placed_total > 0, "placement still draws motes");
    }

    #[test]
    fn mote_over_a_glow_is_rejected() {
        let eye = Vec3::ZERO;
        let g = glow_clear(Vec3::new(0.0, 0.0, -8.0), 1.0);
        assert!(!mote_clear_of(eye, Vec3::new(0.0, 0.0, -4.0), g));
        assert!(mote_clear_of(eye, Vec3::new(3.0, 0.0, -4.0), g));
    }

    #[test]
    fn vfx_never_writes_the_answer() {
        let mut app = app_with(GraphicsPreset::High);
        app.world_mut()
            .resource_mut::<WorldAnswer>()
            .fire(AnswerKind::Flow, 5.0, "flow");
        app.update();
        let a = app.world().resource::<WorldAnswer>();
        assert_eq!(a.kind, AnswerKind::Flow);
        assert_eq!(a.until, 5.0 + 3.4);
        assert_eq!(a.warmth, 0.85);
    }
}
