//! CARD VP-WATER-1 — water, falls, and reflections.
//!
//! Water is a bath: one flat surface. On Sanctuary it sits on the first
//! terrace shelf, outside the 56×56 yard. The Heartwood pond keeps its
//! cylinder and its bath-return; this module only supplies the shared
//! material. Mobile and Low draw the flat bath alone. Medium adds one
//! static sheet at the back shelf edge and stays at [`CHEAP_VERTEX_CAP`].
//! High and Ultra turn on Bevy's `ScreenSpaceReflections` (see
//! [`crate::local_settings::ssr_for`]). That path is not the hand-rolled
//! node in `ssr_render_node.rs`.
//!
//! Every preset shades the bath with a dielectric fresnel and a sky tint.
//! Mobile, Low, and Medium stay on the forward pass. High and Ultra mark
//! the water material deferred so the built-in SSR pass can read it.
//! `DefaultOpaqueRendererMethod` stays forward, so other materials keep
//! the VP-MAT-1 forward shade. Falls and spray are static meshes on the
//! back shelf edge. They add no lights. This module adds no volumetric
//! fog; Ultra shafts stay the existing fog on the world camera.
//!
//! The mesh is built when the preset or realm changes. A steady frame
//! does not rebuild it.
//!
//! Contact: info@Rathor.ai

use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::asset::RenderAssetUsages;
use bevy::material::{AlphaMode, OpaqueRendererMethod};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use shared::local_settings::GraphicsPreset;

use crate::living_practice_loop::SoftPlayerRealm;
use crate::local_settings::{ssr_enabled, LocalSettingsState};
use crate::sky_backdrop::SKY_ZENITH;
use crate::terrain::{COURT_RADIUS, PLAY_FLOOR_Y, SHELF_Y};

/// Pond body, the Heartwood bath colour (sRGB).
pub const BATH_SRGB: [f32; 3] = [0.06, 0.24, 0.28];
/// Pond alpha on the forward tiers. High and Ultra are opaque so SSR can read them.
pub const BATH_ALPHA: f32 = 0.78;
/// Dielectric fresnel at a facing view. Water IOR ~1.33.
pub const WATER_F0: f32 = 0.02;
/// Sky tint mixed in by the fresnel. Sanctuary zenith.
pub const SKY_TINT: [f32; 3] = SKY_ZENITH;
/// View cosine baked into the albedo. Grazing enough for the sky to read.
const BAKED_VIEW: f32 = 0.22;

/// Inner rim of the moat. Outside the court disk and the 56×56 yard.
pub const BATH_INNER: f32 = 48.0;
/// Outer rim. Inside the second terrace (`SHELF_OUTER[1]` is 70).
pub const BATH_OUTER: f32 = 66.0;
/// Sits on the first dropped shelf, a hair above the tread.
pub const BATH_Y: f32 = SHELF_Y[1] + 0.05;
/// Top of a fall, just outside the court rim (the first shelf edge).
pub const FALL_TOP_R: f32 = COURT_RADIUS + 0.55;
/// Bottom of a fall. Still outside the court, tucked in so the face
/// the yard sees points up toward the sun.
pub const FALL_BOT_R: f32 = COURT_RADIUS + 0.2;
const FALL_HALF_WIDTH: f32 = 1.15;
/// Back arc, centred on −Z (behind the yard from the boot camera at +Z).
const FALL_CENTER: f32 = -FRAC_PI_2;
const FALL_SPAN: f32 = 0.8;

/// Mobile, Low, and Medium stay at or under the Medium mesh.
pub const CHEAP_VERTEX_CAP: u32 = water_plan(GraphicsPreset::Medium).vertices();

/// Preset → bath detail, falls, spray, and whether SSR is on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaterPlan {
    /// Rim segments of the flat bath. At least 3.
    pub segments: u32,
    /// Static sheets on the back shelf edge. Zero on Mobile and Low.
    pub falls: u32,
    /// Flat spray cards. Ultra only.
    pub spray: u32,
    /// Bevy `ScreenSpaceReflections` on the world camera.
    pub ssr: bool,
    /// Mobile, Low, and Medium.
    pub cheap: bool,
    /// Always false. The mesh is rebuilt only when the preset or realm changes.
    pub rebuild_each_frame: bool,
    /// This card does not add shafts. Ultra fog stays in `climate_plane`.
    pub volumetric_shafts: bool,
    /// Falls and spray add none.
    pub per_entity_lights: u32,
}

impl WaterPlan {
    pub const fn flat_bath(self) -> bool {
        self.falls == 0 && self.spray == 0
    }

    /// Depth and deferred prepasses travel with SSR, and only then.
    pub const fn prepass(self) -> bool {
        self.ssr
    }

    pub const fn vertices(self) -> u32 {
        let bath = self.segments * 6;
        let falls = self.falls * 6;
        let spray = self.spray * 6;
        bath + falls + spray
    }

    pub const fn draws(self) -> u32 {
        let falls = if self.falls > 0 { 1 } else { 0 };
        let spray = if self.spray > 0 { 1 } else { 0 };
        1 + falls + spray
    }
}

/// Density for one graphics preset.
///
/// | Preset | Segments | Falls | Spray | SSR | Vertices |
/// |---|---|---|---|---|---|
/// | Mobile | 8 | 0 | 0 | no | 48 |
/// | Low | 12 | 0 | 0 | no | 72 |
/// | Medium | 16 | 1 | 0 | no | 102 |
/// | High | 24 | 3 | 0 | yes | 162 |
/// | Ultra | 32 | 4 | 2 | yes | 228 |
pub const fn water_plan(preset: GraphicsPreset) -> WaterPlan {
    let ssr = ssr_enabled(preset);
    match preset {
        GraphicsPreset::Mobile => WaterPlan {
            segments: 8,
            falls: 0,
            spray: 0,
            ssr,
            cheap: true,
            rebuild_each_frame: false,
            volumetric_shafts: false,
            per_entity_lights: 0,
        },
        GraphicsPreset::Low => WaterPlan {
            segments: 12,
            falls: 0,
            spray: 0,
            ssr,
            cheap: true,
            rebuild_each_frame: false,
            volumetric_shafts: false,
            per_entity_lights: 0,
        },
        GraphicsPreset::Medium => WaterPlan {
            segments: 16,
            falls: 1,
            spray: 0,
            ssr,
            cheap: true,
            rebuild_each_frame: false,
            volumetric_shafts: false,
            per_entity_lights: 0,
        },
        GraphicsPreset::High => WaterPlan {
            segments: 24,
            falls: 3,
            spray: 0,
            ssr,
            cheap: false,
            rebuild_each_frame: false,
            volumetric_shafts: false,
            per_entity_lights: 0,
        },
        GraphicsPreset::Ultra => WaterPlan {
            segments: 32,
            falls: 4,
            spray: 2,
            ssr,
            cheap: false,
            rebuild_each_frame: false,
            volumetric_shafts: false,
            per_entity_lights: 0,
        },
    }
}

/// Sanctuary only (`None` or `Some(0)`), same gate as the sky backdrop.
pub const fn water_shown(realm: Option<u8>) -> bool {
    matches!(realm, None | Some(0))
}

/// Schlick fresnel. `n_dot_v` is the cosine of the view angle against the bath.
pub fn water_fresnel(n_dot_v: f32) -> f32 {
    let cos = n_dot_v.clamp(0.0, 1.0);
    let m = 1.0 - cos;
    let m2 = m * m;
    WATER_F0 + (1.0 - WATER_F0) * (m2 * m2 * m)
}

/// Bath colour pulled toward [`SKY_TINT`] by [`water_fresnel`].
pub fn water_reflect_srgb(n_dot_v: f32) -> [f32; 3] {
    mix(BATH_SRGB, SKY_TINT, water_fresnel(n_dot_v))
}

/// Shading for one preset. The fresnel and the sky tint are on every tier.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WaterShade {
    pub rgb: [f32; 3],
    pub sky: [f32; 3],
    pub alpha: f32,
    pub roughness: f32,
    pub reflectance: f32,
    pub blend: bool,
    pub deferred: bool,
}

pub fn water_shade(preset: GraphicsPreset) -> WaterShade {
    let ssr = ssr_enabled(preset);
    WaterShade {
        rgb: water_reflect_srgb(BAKED_VIEW),
        sky: SKY_TINT,
        alpha: BATH_ALPHA,
        // Inside Bevy's SSR window on High/Ultra (fully on ~0.12–0.55).
        // Above that window on the forward tiers.
        roughness: if ssr { 0.34 } else { 0.62 },
        reflectance: WATER_F0,
        blend: !ssr,
        deferred: ssr,
    }
}

/// Shared bath material. Heartwood's pond and the Sanctuary moat both use it.
pub fn bath_material(preset: GraphicsPreset) -> StandardMaterial {
    let shade = water_shade(preset);
    let base_color = if shade.blend {
        Color::srgba(shade.rgb[0], shade.rgb[1], shade.rgb[2], shade.alpha)
    } else {
        Color::srgb(shade.rgb[0], shade.rgb[1], shade.rgb[2])
    };
    StandardMaterial {
        base_color,
        perceptual_roughness: shade.roughness,
        metallic: 0.0,
        reflectance: shade.reflectance,
        specular_tint: Color::srgb(shade.sky[0], shade.sky[1], shade.sky[2]),
        alpha_mode: if shade.blend {
            AlphaMode::Blend
        } else {
            AlphaMode::Opaque
        },
        opaque_render_method: if shade.deferred {
            OpaqueRendererMethod::Deferred
        } else {
            OpaqueRendererMethod::Forward
        },
        cull_mode: None,
        ..default()
    }
}

/// True when the moat disk covers `(x, z)`. The yard square is outside it.
pub fn bath_covers(x: f32, z: f32) -> bool {
    let r = (x * x + z * z).sqrt();
    r >= BATH_INNER && r <= BATH_OUTER
}

/// Flat annulus on the dropped shelf. Every vertex shares [`BATH_Y`].
pub fn bath_positions(plan: WaterPlan) -> Vec<[f32; 3]> {
    let n = plan.segments.max(3);
    let mut positions = Vec::with_capacity((n * 6) as usize);
    for i in 0..n {
        let i0 = ring(BATH_INNER, BATH_Y, i, n);
        let i1 = ring(BATH_INNER, BATH_Y, i + 1, n);
        let o0 = ring(BATH_OUTER, BATH_Y, i, n);
        let o1 = ring(BATH_OUTER, BATH_Y, i + 1, n);
        push_tri(&mut positions, i0, o1, o0);
        push_tri(&mut positions, i0, i1, o1);
    }
    positions
}

/// Short sheets on the back shelf edge. The bottom tucks in so the face
/// toward the yard also points up.
pub fn fall_positions(plan: WaterPlan) -> Vec<[f32; 3]> {
    let mut positions = Vec::with_capacity((plan.falls * 6) as usize);
    for index in 0..plan.falls {
        let angle = fall_angle(index, plan.falls);
        let (t0, t1, b0, b1) = fall_corners(angle);
        // Inward and up: the yard looks at the sheet, and the sun from +Z lights it.
        push_tri(&mut positions, t0, b0, t1);
        push_tri(&mut positions, b0, b1, t1);
    }
    positions
}

/// Flat cards just above the moat, on the same back arc. Ultra only.
pub fn spray_positions(plan: WaterPlan) -> Vec<[f32; 3]> {
    let mut positions = Vec::with_capacity((plan.spray * 6) as usize);
    let y = BATH_Y + 0.55;
    for index in 0..plan.spray {
        let angle = fall_angle(index, plan.falls.max(plan.spray).max(1));
        let a0 = angle - 0.08;
        let a1 = angle + 0.08;
        let inner = BATH_INNER + 1.0;
        let outer = BATH_INNER + 7.0;
        let i0 = polar(inner, y, a0);
        let i1 = polar(inner, y, a1);
        let o0 = polar(outer, y, a0);
        let o1 = polar(outer, y, a1);
        push_tri(&mut positions, i0, o1, o0);
        push_tri(&mut positions, i0, i1, o1);
    }
    positions
}

pub fn water_positions(plan: WaterPlan) -> Vec<[f32; 3]> {
    let mut positions = bath_positions(plan);
    positions.extend(fall_positions(plan));
    positions.extend(spray_positions(plan));
    positions
}

fn mesh_from(positions: Vec<[f32; 3]>) -> Mesh {
    let count = positions.len() as u32;
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32((0..count).collect()));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    mesh
}

fn ring(radius: f32, y: f32, index: u32, segments: u32) -> [f32; 3] {
    let angle = index as f32 / segments as f32 * TAU;
    polar(radius, y, angle)
}

fn polar(radius: f32, y: f32, angle: f32) -> [f32; 3] {
    [angle.cos() * radius, y, angle.sin() * radius]
}

fn fall_angle(index: u32, count: u32) -> f32 {
    if count <= 1 {
        return FALL_CENTER;
    }
    let t = index as f32 / (count - 1) as f32;
    FALL_CENTER - FALL_SPAN * 0.5 + FALL_SPAN * t
}

fn fall_corners(angle: f32) -> ([f32; 3], [f32; 3], [f32; 3], [f32; 3]) {
    let dir = Vec2::new(angle.cos(), angle.sin());
    let tangent = Vec2::new(-dir.y, dir.x);
    let corner = |radius: f32, y: f32, side: f32| {
        let p = dir * radius + tangent * side;
        [p.x, y, p.y]
    };
    (
        corner(FALL_TOP_R, PLAY_FLOOR_Y, -FALL_HALF_WIDTH),
        corner(FALL_TOP_R, PLAY_FLOOR_Y, FALL_HALF_WIDTH),
        corner(FALL_BOT_R, BATH_Y, -FALL_HALF_WIDTH),
        corner(FALL_BOT_R, BATH_Y, FALL_HALF_WIDTH),
    )
}

fn push_tri(positions: &mut Vec<[f32; 3]>, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    positions.push(a);
    positions.push(b);
    positions.push(c);
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// Marker so a preset change can retint the Heartwood pond without rebuilding it.
#[derive(Component, Clone, Copy, Debug)]
pub struct BathMark;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum WaterPart {
    Bath,
    Fall,
    Spray,
}

#[derive(Resource, Default)]
struct WaterStamp {
    key: Option<(GraphicsPreset, Option<u8>)>,
}

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WaterStamp>()
            .add_systems(Update, sync_sanctuary_water);
    }
}

fn sync_sanctuary_water(
    mut commands: Commands,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    settings: Option<Res<LocalSettingsState>>,
    realm: Option<Res<SoftPlayerRealm>>,
    mut stamp: ResMut<WaterStamp>,
    ponds: Query<&MeshMaterial3d<StandardMaterial>, (With<BathMark>, Without<WaterPart>)>,
    existing: Query<Entity, With<WaterPart>>,
) {
    let (Some(mut meshes), Some(mut materials)) = (meshes, materials) else {
        return;
    };
    let preset = settings
        .as_ref()
        .map(|state| state.inner.graphics_preset)
        .unwrap_or(GraphicsPreset::Medium);
    let realm = realm.as_ref().map(|realm| realm.current).unwrap_or(None);
    let key = (preset, realm);
    if stamp.key == Some(key) {
        return;
    }

    let mat = bath_material(preset);
    for handle in &ponds {
        if let Some(mut existing) = materials.get_mut(&handle.0) {
            *existing = mat.clone();
        }
    }

    let stale: Vec<Entity> = existing.iter().collect();
    for entity in stale {
        commands.entity(entity).despawn();
    }
    if water_shown(realm) {
        spawn_water(&mut commands, &mut meshes, &mut materials, preset);
    }
    stamp.key = Some(key);
}

fn spawn_water(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    preset: GraphicsPreset,
) {
    let plan = water_plan(preset);
    let material = materials.add(bath_material(preset));
    let bath = meshes.add(mesh_from(bath_positions(plan)));
    commands.spawn((
        (
            Mesh3d(bath),
            MeshMaterial3d(material.clone()),
            Transform::IDENTITY,
        ),
        WaterPart::Bath,
        BathMark,
        Name::new("VP-WATER-1 bath"),
        bevy::light::NotShadowCaster,
    ));
    let falls = fall_positions(plan);
    if !falls.is_empty() {
        commands.spawn((
            (
                Mesh3d(meshes.add(mesh_from(falls))),
                MeshMaterial3d(material.clone()),
                Transform::IDENTITY,
            ),
            WaterPart::Fall,
            Name::new("VP-WATER-1 fall"),
            bevy::light::NotShadowCaster,
        ));
    }
    let spray = spray_positions(plan);
    if !spray.is_empty() {
        commands.spawn((
            (
                Mesh3d(meshes.add(mesh_from(spray))),
                MeshMaterial3d(material),
                Transform::IDENTITY,
            ),
            WaterPart::Spray,
            Name::new("VP-WATER-1 spray"),
            bevy::light::NotShadowCaster,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use shared::heartwood_lamp::{
        heartwood_bath_return, heartwood_lip_is_valid, lip_instance_outside_water,
        HeartwoodLipKind, HEARTWOOD_BATH_RETURN, HEARTWOOD_LIP_INSTANCES, WATER_POND_CENTER,
    };
    use shared::hex_travel::PlaceId;

    use crate::local_settings::ssr_for;
    use crate::painterly_mat::{painterly_srgb, PainterlyRole};
    use crate::terrain::{play_footprint, PLAY_HALF};

    fn positions(mesh: &Mesh) -> Vec<[f32; 3]> {
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|values| values.as_float3())
            .unwrap()
            .to_vec()
    }

    fn normals(mesh: &Mesh) -> Vec<[f32; 3]> {
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            .and_then(|values| values.as_float3())
            .unwrap()
            .to_vec()
    }

    #[test]
    fn mobile_and_low_are_a_flat_bath() {
        for preset in [GraphicsPreset::Mobile, GraphicsPreset::Low] {
            let plan = water_plan(preset);
            assert!(plan.flat_bath(), "{preset:?}");
            assert!(plan.cheap, "{preset:?}");
            assert!(!plan.ssr && !plan.prepass(), "{preset:?}");
            assert!(!plan.rebuild_each_frame, "{preset:?}");
            assert_eq!(plan.per_entity_lights, 0);
            assert!(!plan.volumetric_shafts);
            assert!(plan.vertices() <= CHEAP_VERTEX_CAP, "{preset:?}");
            let bath = bath_positions(plan);
            assert!(fall_positions(plan).is_empty());
            assert!(spray_positions(plan).is_empty());
            assert_eq!(bath.len(), plan.vertices() as usize);
            let y = bath[0][1];
            assert!((y - BATH_Y).abs() < 1.0e-4);
            for p in &bath {
                assert!((p[1] - y).abs() < 1.0e-5, "bath is not flat: {p:?}");
            }
            let mesh = mesh_from(bath);
            assert_eq!(mesh.count_vertices(), plan.vertices() as usize);
        }
        assert!(
            water_plan(GraphicsPreset::Mobile).segments < water_plan(GraphicsPreset::Low).segments
        );
        let medium = water_plan(GraphicsPreset::Medium);
        assert!(medium.cheap);
        assert!(!medium.ssr && !medium.prepass());
        assert_eq!(medium.falls, 1);
        assert_eq!(medium.spray, 0);
        assert_eq!(medium.vertices(), CHEAP_VERTEX_CAP);
        assert!(medium.vertices() > water_plan(GraphicsPreset::Low).vertices());
    }

    #[test]
    fn ssr_only_on_high_and_ultra() {
        for preset in GraphicsPreset::ALL {
            let plan = water_plan(preset);
            let enabled = ssr_enabled(preset);
            assert_eq!(plan.ssr, enabled, "{preset:?}");
            assert_eq!(plan.prepass(), enabled, "{preset:?}");
            assert_eq!(ssr_for(preset).is_some(), enabled, "{preset:?}");
            let shade = water_shade(preset);
            assert_eq!(shade.deferred, enabled, "{preset:?}");
            assert_eq!(shade.blend, !enabled, "{preset:?}");
            assert_eq!(shade.reflectance, WATER_F0, "{preset:?}");
            assert_eq!(shade.sky, SKY_TINT, "{preset:?}");
            if enabled {
                assert!((shade.roughness - 0.34).abs() < 1.0e-5);
                assert!(shade.roughness > 0.12 && shade.roughness < 0.55);
            } else {
                assert!(shade.roughness > 0.6, "{preset:?}");
            }
        }
        assert!(ssr_for(GraphicsPreset::Mobile).is_none());
        assert!(ssr_for(GraphicsPreset::Low).is_none());
        assert!(ssr_for(GraphicsPreset::Medium).is_none());
        let high = ssr_for(GraphicsPreset::High).expect("High SSR");
        let ultra = ssr_for(GraphicsPreset::Ultra).expect("Ultra SSR");
        assert!(high.linear_steps < ultra.linear_steps);
        assert!(high.linear_steps > 0 && ultra.linear_steps > 0);
        let high_plan = water_plan(GraphicsPreset::High);
        let ultra_plan = water_plan(GraphicsPreset::Ultra);
        assert!(!high_plan.cheap && !ultra_plan.cheap);
        assert!(high_plan.vertices() > CHEAP_VERTEX_CAP);
        assert!(ultra_plan.vertices() > high_plan.vertices());
        assert_eq!(ultra_plan.spray, 2);
        assert_eq!(high_plan.spray, 0);
        assert!(high_plan.falls > 0 && ultra_plan.falls > high_plan.falls);
    }

    #[test]
    fn sanctuary_water_stays_off_the_56_court() {
        assert_eq!(PLAY_HALF * 2.0, 56.0);
        let yard_corner = Vec2::new(PLAY_HALF, PLAY_HALF).length();
        assert!(
            BATH_INNER > yard_corner,
            "moat {BATH_INNER} cuts the yard corner {yard_corner}"
        );
        assert!(BATH_INNER > COURT_RADIUS);
        assert!(FALL_TOP_R > FALL_BOT_R && FALL_BOT_R > COURT_RADIUS);
        assert!(BATH_Y < PLAY_FLOOR_Y);
        let xs = [-PLAY_HALF, -20.0, -8.0, 0.0, 8.0, 14.0, PLAY_HALF];
        let zs = [-PLAY_HALF, -14.0, 0.0, 14.0, PLAY_HALF];
        for x in xs {
            for z in zs {
                assert!(play_footprint(x, z), "{x},{z}");
                assert!(!bath_covers(x, z), "bath covers the court at {x},{z}");
            }
        }
        for preset in GraphicsPreset::ALL {
            let positions = water_positions(water_plan(preset));
            assert_eq!(
                positions.len(),
                water_plan(preset).vertices() as usize,
                "{preset:?}"
            );
            for p in &positions {
                assert!(
                    !play_footprint(p[0], p[2]),
                    "{preset:?} water under the court: {p:?}"
                );
                let r = (p[0] * p[0] + p[2] * p[2]).sqrt();
                assert!(r > yard_corner, "{preset:?} water at {p:?} r {r}");
            }
            for p in fall_positions(water_plan(preset))
                .into_iter()
                .chain(spray_positions(water_plan(preset)))
            {
                assert!(
                    p[2] < -PLAY_HALF,
                    "{preset:?} fall or spray is not behind the yard: {p:?}"
                );
            }
        }
        // The shelf the player stands on is still the old floor.
        assert_eq!(crate::terrain::terrace_height(0.0, 0.0), PLAY_FLOOR_Y);
        assert_eq!(
            painterly_srgb(PainterlyRole::Ground, [0.33, 0.40, 0.20]),
            [0.33, 0.40, 0.20]
        );
    }

    #[test]
    fn fresnel_pulls_the_bath_toward_the_sky() {
        let facing = water_fresnel(1.0);
        let grazing = water_fresnel(0.0);
        assert!(facing < 0.05, "{facing}");
        assert!(grazing > 0.95, "{grazing}");
        assert!(water_fresnel(0.2) > water_fresnel(0.8));
        let face = water_reflect_srgb(1.0);
        let graze = water_reflect_srgb(0.0);
        for channel in 0..3 {
            assert!(
                (face[channel] - BATH_SRGB[channel]).abs() < 0.03,
                "facing left the bath: {face:?}"
            );
        }
        assert!(graze[2] > face[2], "grazing blue {graze:?} vs {face:?}");
        assert!(graze[2] > graze[0] && graze[2] > graze[1]);
        for preset in GraphicsPreset::ALL {
            let mat = bath_material(preset);
            assert!((mat.reflectance - WATER_F0).abs() < 1.0e-5);
            assert_eq!(mat.metallic, 0.0);
            let tint = mat.specular_tint.to_srgba();
            assert!(tint.blue > tint.red && tint.blue > tint.green);
            let rgb = mat.base_color.to_srgba();
            assert!(rgb.blue > rgb.red);
            if ssr_enabled(preset) {
                assert_eq!(mat.alpha_mode, AlphaMode::Opaque);
                assert_eq!(mat.opaque_render_method, OpaqueRendererMethod::Deferred);
            } else {
                assert_eq!(mat.alpha_mode, AlphaMode::Blend);
                assert!((rgb.alpha - BATH_ALPHA).abs() < 1.0e-5);
                assert_eq!(mat.opaque_render_method, OpaqueRendererMethod::Forward);
            }
        }
    }

    #[test]
    fn heartwood_ribs_stay_outside_the_pond_and_the_bath_return_holds() {
        assert!(heartwood_lip_is_valid());
        let ribs: Vec<_> = HEARTWOOD_LIP_INSTANCES
            .iter()
            .filter(|instance| instance.kind == HeartwoodLipKind::HangingRib)
            .collect();
        assert_eq!(ribs.len(), 5);
        for rib in ribs {
            assert!(lip_instance_outside_water(rib));
        }
        assert_eq!(
            heartwood_bath_return(
                PlaceId::Heartwood,
                WATER_POND_CENTER[0],
                WATER_POND_CENTER[1]
            ),
            Some(HEARTWOOD_BATH_RETURN)
        );
        assert_eq!(
            heartwood_bath_return(PlaceId::Sanctuary, 0.0, 0.0),
            None
        );
    }

    #[test]
    fn bath_faces_up_and_falls_face_the_yard() {
        let plan = water_plan(GraphicsPreset::Ultra);
        let bath = mesh_from(bath_positions(plan));
        for (index, normal) in normals(&bath).iter().enumerate() {
            assert!(
                normal[1] > 0.9,
                "bath normal {normal:?} at {:?}",
                positions(&bath)[index]
            );
        }
        let falls = mesh_from(fall_positions(plan));
        let pos = positions(&falls);
        let nrm = normals(&falls);
        assert_eq!(pos.len(), nrm.len());
        for (index, p) in pos.iter().enumerate() {
            let n = nrm[index];
            let inward = n[0] * p[0] + n[2] * p[2];
            assert!(inward < 0.0, "fall faces outward at {p:?} {n:?}");
            assert!(n[1] > 0.0, "fall normal points down at {p:?} {n:?}");
        }
    }

    #[test]
    fn spawn_tracks_preset_and_does_not_rebuild_a_steady_frame() {
        use bevy::asset::AssetPlugin;
        use shared::local_settings::LocalSettings;

        fn boot(preset: GraphicsPreset, realm: Option<u8>) -> App {
            let mut app = App::new();
            app.add_plugins((MinimalPlugins, AssetPlugin::default()))
                .init_asset::<Mesh>()
                .init_asset::<StandardMaterial>()
                .add_plugins(WaterPlugin)
                .insert_resource(SoftPlayerRealm { current: realm });
            let mut inner = LocalSettings::default();
            inner.set_graphics_preset(preset);
            app.insert_resource(LocalSettingsState {
                inner,
                dirty: false,
            });
            app
        }

        fn rows(app: &mut App) -> Vec<(WaterPart, bevy::asset::AssetId<Mesh>)> {
            let world = app.world_mut();
            let mut query = world.query::<(&WaterPart, &Mesh3d)>();
            let mut rows: Vec<_> = query
                .iter(world)
                .map(|(part, mesh)| (*part, mesh.0.id()))
                .collect();
            rows.sort_by_key(|row| match row.0 {
                WaterPart::Bath => 0,
                WaterPart::Fall => 1,
                WaterPart::Spray => 2,
            });
            rows
        }

        let mut app = boot(GraphicsPreset::Low, Some(0));
        app.update();
        let low = rows(&mut app);
        assert_eq!(low.len(), 1);
        assert_eq!(low[0].0, WaterPart::Bath);
        let mut lights = app.world_mut().query::<&PointLight>();
        assert_eq!(lights.iter(app.world()).count(), 0);
        app.update();
        assert_eq!(rows(&mut app), low, "steady Low frame rebuilt the bath");

        app.world_mut()
            .resource_mut::<LocalSettingsState>()
            .inner
            .set_graphics_preset(GraphicsPreset::Ultra);
        app.update();
        let ultra = rows(&mut app);
        assert_eq!(ultra.len(), water_plan(GraphicsPreset::Ultra).draws() as usize);
        assert_ne!(ultra[0].1, low[0].1);
        app.update();
        assert_eq!(rows(&mut app), ultra, "steady Ultra frame rebuilt water");

        app.world_mut()
            .resource_mut::<SoftPlayerRealm>()
            .current = Some(1);
        app.update();
        assert!(rows(&mut app).is_empty(), "Heartwood kept the Sanctuary moat");
        app.update();
        assert!(rows(&mut app).is_empty());
    }
}
