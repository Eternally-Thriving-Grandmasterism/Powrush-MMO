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
            assets.mesh = Some(meshes.add(Sphere::new(0.09).mesh().ico(1).unwrap()));
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

    // Lower third of the view, to either side of centre.
    let (origin, right, up) = cameras
        .iter()
        .next()
        .map(|cam| {
            let fwd = cam.forward().as_vec3();
            (
                cam.translation() + fwd * 4.5 - cam.up().as_vec3() * 1.1,
                cam.right().as_vec3(),
                Vec3::Y,
            )
        })
        .unwrap_or((Vec3::ZERO, Vec3::X, Vec3::Y));

    debug!(target: "powrush::vfx", ?preset, n, ?origin, "tend/flow motes");
    for i in 0..n {
        let t = i as f32 / n.max(1) as f32;
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let pos = origin + right * side * (0.9 + 0.9 * t) + up * (0.15 * t);
        let mote = TendFlowMote {
            age: 0.0,
            life: if flow { VFX_LIFE_FLOW_SECS } else { VFX_LIFE_TEND_SECS },
            rise: if flow { 0.9 } else { 0.6 },
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
