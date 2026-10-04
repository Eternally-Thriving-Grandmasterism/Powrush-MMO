//! CARD VP-SKY-1 — Sanctuary blue-sky valley backdrop.
//!
//! Council ruling VISUAL-PASS-1 (a): the Sanctuary reads as a lively blue-sky
//! valley, not graphite-warm brown. This plugin adds the far layers behind the
//! existing yard: a gradient sky dome, a wide valley floor, a faceted mountain
//! ring with snow on the tall peaks, and a conifer belt. The ring and the belt
//! are merged into one mesh per material, so the whole backdrop is a handful
//! of draws.
//!
//! Shown only in the Sanctuary (`SoftPlayerRealm.current` None or Some(0)).
//! Per preset ([`backdrop_plan`]): Mobile keeps only the valley floor; Low
//! adds the mountain ring and a 64-tree belt; Medium and up add the dome and
//! the full 200-tree belt. At most 6 draws on every preset. The clear colour
//! carries the sky where the dome is off. Writes no `FogSettings`. No bloom,
//! tonemap, layout or HUD change.
//! Contact: info@Rathor.ai

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

use shared::local_settings::GraphicsPreset;

use crate::living_practice_loop::SoftPlayerRealm;
use crate::local_settings::LocalSettingsState;

/// Sky dome radius (m). Fog is off on the dome, so it never greys out.
pub const DOME_RADIUS: f32 = 900.0;
/// Valley floor width (m), laid just under the yard plate.
pub const GROUND_SIZE: f32 = 1200.0;
pub const GROUND_Y: f32 = -0.02;
/// Mountain ring.
pub const PEAK_COUNT: usize = 30;
pub const PEAK_R_MIN: f32 = 340.0;
pub const PEAK_R_MAX: f32 = 430.0;
pub const PEAK_H_MIN: f32 = 60.0;
pub const PEAK_H_MAX: f32 = 140.0;
/// Peaks taller than this carry a snow cap.
pub const SNOW_CAP_ABOVE: f32 = 100.0;
/// Conifer belt.
pub const CONIFER_COUNT: usize = 200;
/// Low preset belt: the first 64 trees of the same seeded belt.
pub const LOW_CONIFER_COUNT: usize = 64;
/// Draw budget for the whole backdrop on any preset.
pub const MAX_BACKDROP_DRAWS: usize = 6;
const CONIFER_SEED: u32 = 0x1234_5678;
pub const CONIFER_R_MIN: f32 = 30.0;
pub const CONIFER_R_MAX: f32 = 150.0;

/// Sky dome stops (sRGB): zenith, horizon, below-horizon.
pub const SKY_ZENITH: [f32; 3] = [0.16, 0.36, 0.78];
pub const SKY_HORIZON: [f32; 3] = [0.78, 0.86, 0.96];
pub const SKY_BELOW: [f32; 3] = [0.62, 0.70, 0.80];
/// Same valley earth as the Sanctuary plate (climate_plane SANCTUARY_VALLEY_EARTH).
pub const VALLEY_EARTH: [f32; 3] = [0.33, 0.40, 0.20];
pub const RIDGE_ROCK: [f32; 3] = [0.36, 0.36, 0.50];
pub const RIDGE_SNOW: [f32; 3] = [0.95, 0.96, 1.0];
pub const CONIFER_BARK: [f32; 3] = [0.30, 0.22, 0.15];
pub const CONIFER_NEEDLE: [f32; 3] = [0.10, 0.30, 0.16];

/// Which backdrop layer an entity is.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkyBackdropPart {
    Dome,
    Ground,
    Ridge,
    /// Full belt ([`CONIFER_COUNT`] trees), Medium and up.
    BeltFull,
    /// Low belt ([`LOW_CONIFER_COUNT`] trees), Low only.
    BeltLow,
}

/// What the backdrop draws for one realm and preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackdropPlan {
    pub dome: bool,
    pub ground: bool,
    pub peaks: usize,
    pub conifers: usize,
}

impl BackdropPlan {
    pub const NONE: Self = Self { dome: false, ground: false, peaks: 0, conifers: 0 };

    /// Draw calls: dome, ground, ridge rock + snow (one each), belt bark +
    /// needle (one each).
    pub fn draws(&self) -> usize {
        let ridge = if self.peaks > 0 {
            let snow = ring_peaks().iter().take(self.peaks).any(|p| p.snow_capped());
            1 + usize::from(snow)
        } else {
            0
        };
        usize::from(self.dome) + usize::from(self.ground) + ridge + if self.conifers > 0 { 2 } else { 0 }
    }

    pub fn shows(&self, part: SkyBackdropPart) -> bool {
        match part {
            SkyBackdropPart::Dome => self.dome,
            SkyBackdropPart::Ground => self.ground,
            SkyBackdropPart::Ridge => self.peaks > 0,
            SkyBackdropPart::BeltFull => self.conifers == CONIFER_COUNT,
            SkyBackdropPart::BeltLow => self.conifers == LOW_CONIFER_COUNT,
        }
    }
}

/// Per-preset backdrop. Outside the Sanctuary nothing draws.
pub fn backdrop_plan(realm: Option<u8>, preset: GraphicsPreset) -> BackdropPlan {
    if !matches!(realm, None | Some(0)) {
        return BackdropPlan::NONE;
    }
    match preset {
        GraphicsPreset::Mobile => BackdropPlan { dome: false, ground: true, peaks: 0, conifers: 0 },
        GraphicsPreset::Low => BackdropPlan {
            dome: false,
            ground: true,
            peaks: PEAK_COUNT,
            conifers: LOW_CONIFER_COUNT,
        },
        _ => BackdropPlan { dome: true, ground: true, peaks: PEAK_COUNT, conifers: CONIFER_COUNT },
    }
}

/// One mountain in the ring.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Peak {
    pub angle: f32,
    pub radius: f32,
    pub height: f32,
    pub base: f32,
}

impl Peak {
    pub fn snow_capped(&self) -> bool {
        self.height > SNOW_CAP_ABOVE
    }
}

/// Small deterministic xorshift, so the valley is the same every launch.
#[derive(Debug, Clone)]
pub struct Rng(u32);

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self(seed.max(1))
    }
    /// Uniform in 0.0..=1.0.
    pub fn next_f32(&mut self) -> f32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x as f32 / u32::MAX as f32
    }
}

/// True when the backdrop layer should draw for this realm and preset.
pub fn backdrop_visible(realm: Option<u8>, preset: GraphicsPreset, part: SkyBackdropPart) -> bool {
    backdrop_plan(realm, preset).shows(part)
}

/// Dome colour (sRGB) at normalised height `h` in -1.0..=1.0.
pub fn dome_color_at(h: f32) -> [f32; 3] {
    let lerp = |a: [f32; 3], b: [f32; 3], t: f32| {
        let t = t.clamp(0.0, 1.0);
        [
            a[0] + (b[0] - a[0]) * t,
            a[1] + (b[1] - a[1]) * t,
            a[2] + (b[2] - a[2]) * t,
        ]
    };
    if h >= 0.0 {
        lerp(SKY_HORIZON, SKY_ZENITH, h.min(1.0).powf(0.55))
    } else {
        lerp(SKY_HORIZON, SKY_BELOW, -h)
    }
}

/// The 30 ring peaks, deterministic.
pub fn ring_peaks() -> Vec<Peak> {
    (0..PEAK_COUNT)
        .map(|i| {
            let angle = i as f32 / PEAK_COUNT as f32 * TAU + 0.07 * (i % 3) as f32;
            let radius = PEAK_R_MIN + (PEAK_R_MAX - PEAK_R_MIN) * ((i * 7 % 5) as f32 / 4.0);
            let height = PEAK_H_MIN + (PEAK_H_MAX - PEAK_H_MIN) * ((i * 11 % 7) as f32 / 6.0);
            let base = 60.0 + 30.0 * ((i * 5 % 4) as f32 / 3.0);
            Peak { angle, radius, height, base }
        })
        .collect()
}

/// Conifer spots `(position, scale)` in the belt, deterministic.
pub fn conifer_spots(seed: u32, count: usize) -> Vec<(Vec3, f32)> {
    let mut rng = Rng::new(seed);
    (0..count)
        .map(|_| {
            let a = rng.next_f32() * TAU;
            let r = CONIFER_R_MIN + rng.next_f32() * (CONIFER_R_MAX - CONIFER_R_MIN);
            let s = 0.9 + rng.next_f32() * 1.6;
            (Vec3::new(a.cos() * r, 0.0, a.sin() * r), s)
        })
        .collect()
}

fn srgb_to_linear(c: [f32; 3]) -> [f32; 4] {
    let f = |v: f32| v.max(0.0).powf(2.2);
    [f(c[0]), f(c[1]), f(c[2]), 1.0]
}

/// UV sphere with the dome gradient baked into vertex colours.
pub fn gradient_dome(radius: f32) -> Mesh {
    let mut m = Sphere::new(radius).mesh().uv(48, 24);
    let cols: Vec<[f32; 4]> = m
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .and_then(|a| a.as_float3())
        .map(|ps| ps.iter().map(|p| srgb_to_linear(dome_color_at(p[1] / radius))).collect())
        .unwrap_or_default();
    m.insert_attribute(Mesh::ATTRIBUTE_COLOR, cols);
    m
}

/// Low-poly faceted mountain: jittered base ring, mid ring, apex. Flat normals.
pub fn faceted_mountain(rng: &mut Rng, base: f32, height: f32) -> Mesh {
    let n = 9;
    let apex = Vec3::new(
        (rng.next_f32() - 0.5) * base * 0.4,
        height,
        (rng.next_f32() - 0.5) * base * 0.4,
    );
    let ring: Vec<Vec3> = (0..n)
        .map(|i| {
            let a = i as f32 / n as f32 * TAU;
            let r = base * (0.7 + 0.5 * rng.next_f32());
            Vec3::new(a.cos() * r, -2.0, a.sin() * r)
        })
        .collect();
    let mid: Vec<Vec3> = ring
        .iter()
        .map(|p| {
            let q = p.lerp(apex, 0.45 + 0.15 * rng.next_f32());
            q + Vec3::new(
                (rng.next_f32() - 0.5) * base * 0.15,
                0.0,
                (rng.next_f32() - 0.5) * base * 0.15,
            )
        })
        .collect();
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n * 9);
    for i in 0..n {
        let j = (i + 1) % n;
        for p in [ring[i], mid[i], ring[j], ring[j], mid[i], mid[j], mid[i], apex, mid[j]] {
            positions.push(p.into());
        }
    }
    let count = positions.len() as u32;
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32((0..count).collect()));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    mesh
}

/// Merge `parts` into one mesh. All parts must share one attribute layout.
fn merged(parts: impl IntoIterator<Item = Mesh>) -> Option<Mesh> {
    let mut it = parts.into_iter();
    let mut first = it.next()?;
    for m in it {
        first.merge(&m);
    }
    Some(first)
}

/// Mountain ring as (rock, snow) meshes.
pub fn ridge_meshes() -> (Option<Mesh>, Option<Mesh>) {
    let mut rng = Rng::new(0x9E37_79B9);
    let peaks = ring_peaks();
    let rock = merged(peaks.iter().map(|p| {
        faceted_mountain(&mut rng, p.base, p.height)
            .transformed_by(Transform::from_xyz(p.angle.cos() * p.radius, 0.0, p.angle.sin() * p.radius))
    }));
    let snow = merged(peaks.iter().filter(|p| p.snow_capped()).map(|p| {
        let mut cap = Mesh::from(Cone { radius: p.base * 0.22, height: p.height * 0.22 });
        cap.duplicate_vertices();
        cap.compute_flat_normals();
        cap.transformed_by(Transform::from_xyz(
            p.angle.cos() * p.radius,
            p.height * 0.86,
            p.angle.sin() * p.radius,
        ))
    }));
    (rock, snow)
}

/// Conifer belt of `count` trees as (bark, needle) meshes: one trunk and
/// three tiers per tree. Low takes the first 64 trees of the same belt.
pub fn belt_meshes(count: usize) -> (Option<Mesh>, Option<Mesh>) {
    let spots = conifer_spots(CONIFER_SEED, count);
    let bark = merged(spots.iter().map(|(p, s)| {
        Mesh::from(Cylinder::new(0.18, 1.2)).transformed_by(
            Transform::from_translation(*p + Vec3::Y * 0.6 * *s).with_scale(Vec3::splat(*s)),
        )
    }));
    let needle = merged(spots.iter().flat_map(|(p, s)| {
        (0..3).map(move |k| {
            let y = (1.6 + k as f32 * 0.9) * *s;
            let sc = *s * (1.0 - k as f32 * 0.25);
            Mesh::from(Cone { radius: 1.1, height: 2.2 })
                .transformed_by(Transform::from_translation(*p + Vec3::Y * y).with_scale(Vec3::splat(sc)))
        })
    }));
    (bark, needle)
}

pub struct SkyBackdropPlugin;

impl Plugin for SkyBackdropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sky_backdrop)
            .add_systems(Update, show_backdrop_for_realm);
    }
}

fn lit(c: [f32; 3], roughness: f32) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(c[0], c[1], c[2]),
        perceptual_roughness: roughness,
        ..default()
    }
}

fn spawn_sky_backdrop(
    mut commands: Commands,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
) {
    // Headless test apps have no render assets; the backdrop is visual only.
    let (Some(mut meshes), Some(mut materials)) = (meshes, materials) else {
        return;
    };
    // CARD VP-GRADE-1 — the dome neither casts nor receives sun shadows, so
    // the shadowed Sanctuary sun reaches the yard through it.
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(gradient_dome(DOME_RADIUS)),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                fog_enabled: false,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::IDENTITY,
            ..default()
        },
        SkyBackdropPart::Dome,
        Name::new("VP-SKY-1 backdrop"),
        bevy::pbr::NotShadowCaster,
        bevy::pbr::NotShadowReceiver,
    ));
    let mut spawn = |mesh: Mesh, mat: StandardMaterial, part: SkyBackdropPart, t: Transform| {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(mat),
                transform: t,
                ..default()
            },
            part,
            Name::new("VP-SKY-1 backdrop"),
        ));
    };
    spawn(
        Plane3d::default().mesh().size(GROUND_SIZE, GROUND_SIZE).build(),
        lit(VALLEY_EARTH, 0.95),
        SkyBackdropPart::Ground,
        Transform::from_xyz(0.0, GROUND_Y, 0.0),
    );
    let (rock, snow) = ridge_meshes();
    if let Some(m) = rock {
        spawn(m, lit(RIDGE_ROCK, 1.0), SkyBackdropPart::Ridge, Transform::IDENTITY);
    }
    if let Some(m) = snow {
        spawn(m, lit(RIDGE_SNOW, 0.8), SkyBackdropPart::Ridge, Transform::IDENTITY);
    }
    for (count, part) in [
        (CONIFER_COUNT, SkyBackdropPart::BeltFull),
        (LOW_CONIFER_COUNT, SkyBackdropPart::BeltLow),
    ] {
        let (bark, needle) = belt_meshes(count);
        if let Some(m) = bark {
            spawn(m, lit(CONIFER_BARK, 0.9), part, Transform::IDENTITY);
        }
        if let Some(m) = needle {
            spawn(m, lit(CONIFER_NEEDLE, 0.8), part, Transform::IDENTITY);
        }
    }
}

fn show_backdrop_for_realm(
    realm: Option<Res<SoftPlayerRealm>>,
    settings: Option<Res<LocalSettingsState>>,
    mut parts: Query<(&SkyBackdropPart, &mut Visibility)>,
) {
    let current = realm.map(|r| r.current).unwrap_or(None);
    // Missing settings (headless) stay Medium, same fallback as climate_plane.
    let preset = settings
        .as_ref()
        .map(|s| s.inner.graphics_preset)
        .unwrap_or(GraphicsPreset::Medium);
    let plan = backdrop_plan(current, preset);
    for (part, mut vis) in &mut parts {
        let want = if plan.shows(*part) {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if *vis != want {
            *vis = want;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_brownish(c: [f32; 3]) -> bool {
        c[0] >= c[1] && c[1] >= c[2] && (c[0] - c[2]) >= 0.03
    }

    #[test]
    fn dome_is_blue_from_horizon_to_zenith_and_never_brown() {
        let zen = dome_color_at(1.0);
        let hor = dome_color_at(0.0);
        assert!(zen[2] > zen[1] && zen[1] > zen[0]);
        assert!(hor[2] > hor[1] && hor[1] > hor[0]);
        assert!(zen[2] - zen[0] > hor[2] - hor[0], "zenith is the deeper blue");
        for i in -10..=10 {
            let c = dome_color_at(i as f32 / 10.0);
            assert!(!is_brownish(c), "dome at {i} reads brown: {c:?}");
            assert!(c[2] >= 0.75);
        }
    }

    #[test]
    fn ring_has_thirty_peaks_in_band_with_snow_only_on_tall_ones() {
        let peaks = ring_peaks();
        assert_eq!(peaks.len(), PEAK_COUNT);
        for p in &peaks {
            assert!((PEAK_R_MIN..=PEAK_R_MAX).contains(&p.radius));
            assert!((PEAK_H_MIN..=PEAK_H_MAX).contains(&p.height));
            assert_eq!(p.snow_capped(), p.height > SNOW_CAP_ABOVE);
            assert!(p.radius + p.base < DOME_RADIUS);
        }
        let capped = peaks.iter().filter(|p| p.snow_capped()).count();
        assert!(capped > 0 && capped < PEAK_COUNT);
    }

    #[test]
    fn conifer_belt_stays_out_of_the_yard_and_inside_the_valley() {
        let spots = conifer_spots(CONIFER_SEED, CONIFER_COUNT);
        assert_eq!(spots.len(), CONIFER_COUNT);
        for (p, s) in &spots {
            let r = Vec2::new(p.x, p.z).length();
            assert!(r >= CONIFER_R_MIN - 0.01 && r <= CONIFER_R_MAX + 0.01, "r {r}");
            assert!((0.9..=2.5).contains(s));
        }
        assert_eq!(spots, conifer_spots(CONIFER_SEED, CONIFER_COUNT), "deterministic");
        assert_eq!(&spots[..LOW_CONIFER_COUNT], &conifer_spots(CONIFER_SEED, LOW_CONIFER_COUNT)[..]);
    }

    #[test]
    fn ridge_and_belt_merge_to_one_mesh_per_material() {
        let (rock, snow) = ridge_meshes();
        let (bark, needle) = belt_meshes(CONIFER_COUNT);
        let rock = rock.expect("rock");
        let snow = snow.expect("snow");
        let bark = bark.expect("bark");
        let needle = needle.expect("needle");
        let one_tier = Mesh::from(Cone { radius: 1.1, height: 2.2 }).count_vertices();
        assert_eq!(needle.count_vertices(), one_tier * 3 * CONIFER_COUNT);
        let one_trunk = Mesh::from(Cylinder::new(0.18, 1.2)).count_vertices();
        assert_eq!(bark.count_vertices(), one_trunk * CONIFER_COUNT);
        assert!(rock.count_vertices() >= PEAK_COUNT * 81);
        assert!(snow.count_vertices() > 0);
    }

    #[test]
    fn backdrop_only_in_sanctuary() {
        for preset in GraphicsPreset::ALL {
            for realm in [Some(1u8), Some(2), Some(3), Some(4)] {
                assert_eq!(backdrop_plan(realm, preset), BackdropPlan::NONE);
                assert_eq!(backdrop_plan(realm, preset).draws(), 0);
            }
            assert_eq!(backdrop_plan(None, preset), backdrop_plan(Some(0), preset));
        }
    }

    /// Spawn the backdrop in a headless app with render assets, set the
    /// preset, and count what is visible: (draws, conifers, peaks shown).
    fn visible_for(preset: GraphicsPreset) -> (usize, usize, bool) {
        use shared::local_settings::LocalSettings;
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .insert_resource(SoftPlayerRealm { current: Some(0) });
        let mut inner = LocalSettings::default();
        inner.set_graphics_preset(preset);
        app.insert_resource(LocalSettingsState { inner, dirty: false });
        app.add_plugins(SkyBackdropPlugin);
        app.update();
        let one_tier = Mesh::from(Cone { radius: 1.1, height: 2.2 }).count_vertices();
        let world = app.world_mut();
        let mut q = world.query::<(&SkyBackdropPart, &Visibility, &Handle<Mesh>, &Handle<StandardMaterial>)>();
        let rows: Vec<_> = q
            .iter(world)
            .filter(|(_, v, _, _)| **v != Visibility::Hidden)
            .map(|(p, _, m, mat)| (*p, m.clone(), mat.clone()))
            .collect();
        let meshes = world.resource::<Assets<Mesh>>();
        let mats = world.resource::<Assets<StandardMaterial>>();
        let mut conifers = 0;
        let mut ridge = false;
        for (part, mesh, mat) in &rows {
            let c = mats.get(mat).unwrap().base_color.to_srgba();
            let needle = Color::srgb(CONIFER_NEEDLE[0], CONIFER_NEEDLE[1], CONIFER_NEEDLE[2]).to_srgba();
            if matches!(part, SkyBackdropPart::BeltFull | SkyBackdropPart::BeltLow) && c == needle {
                conifers += meshes.get(mesh).unwrap().count_vertices() / (one_tier * 3);
            }
            ridge |= *part == SkyBackdropPart::Ridge;
        }
        (rows.len(), conifers, ridge)
    }

    #[test]
    fn preset_mobile_is_floor_only() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::Mobile);
        assert!(!plan.dome && plan.ground);
        assert_eq!((plan.peaks, plan.conifers, plan.draws()), (0, 0, 1));
        assert_eq!(visible_for(GraphicsPreset::Mobile), (1, 0, false));
    }

    #[test]
    fn preset_low_keeps_ring_drops_dome_and_caps_belt_at_64() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::Low);
        assert!(!plan.dome && plan.ground);
        assert_eq!((plan.peaks, plan.conifers, plan.draws()), (30, 64, 5));
        assert!(plan.conifers <= 64);
        assert_eq!(visible_for(GraphicsPreset::Low), (5, 64, true));
    }

    #[test]
    fn preset_medium_full_valley_in_six_draws() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::Medium);
        assert!(plan.dome && plan.ground);
        assert_eq!((plan.peaks, plan.conifers, plan.draws()), (30, 200, 6));
        assert_eq!(visible_for(GraphicsPreset::Medium), (6, 200, true));
    }

    #[test]
    fn preset_high_full_valley_in_six_draws() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::High);
        assert_eq!((plan.dome, plan.peaks, plan.conifers, plan.draws()), (true, 30, 200, 6));
        assert_eq!(visible_for(GraphicsPreset::High), (6, 200, true));
    }

    #[test]
    fn preset_ultra_full_valley_in_six_draws() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::Ultra);
        assert_eq!((plan.dome, plan.peaks, plan.conifers, plan.draws()), (true, 30, 200, 6));
        assert_eq!(visible_for(GraphicsPreset::Ultra), (6, 200, true));
    }

    #[test]
    fn every_preset_within_draw_budget() {
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        for preset in GraphicsPreset::ALL {
            assert!(backdrop_plan(Some(0), preset).draws() <= MAX_BACKDROP_DRAWS, "{preset:?}");
        }
    }

    #[test]
    fn sky_backdrop_writes_no_fog() {
        let src = include_str!("sky_backdrop.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        assert!(!code.contains("FogSettings {"));
        assert!(!code.contains("Mut<FogSettings>"));
        assert!(!code.contains("mut FogSettings"));
        assert!(!code.contains("FogWriteSet"));
    }

    #[test]
    fn valley_colours_are_lively_not_brown() {
        assert!(VALLEY_EARTH[1] > VALLEY_EARTH[0] && VALLEY_EARTH[1] > VALLEY_EARTH[2]);
        assert!(CONIFER_NEEDLE[1] > CONIFER_NEEDLE[0] && CONIFER_NEEDLE[1] > CONIFER_NEEDLE[2]);
        assert!(RIDGE_ROCK[2] > RIDGE_ROCK[0], "ridge leans blue-violet, not beige");
        assert!(!is_brownish(RIDGE_SNOW));
    }

    /// CARD VP-GRADE-1 — on Medium the dome is shown and carries
    /// `NotShadowCaster` + `NotShadowReceiver`; no other part does.
    #[test]
    fn vp_grade_medium_dome_casts_and_receives_no_shadow() {
        use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
        use shared::local_settings::LocalSettings;
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .insert_resource(SoftPlayerRealm { current: Some(0) });
        let mut inner = LocalSettings::default();
        inner.set_graphics_preset(GraphicsPreset::Medium);
        app.insert_resource(LocalSettingsState { inner, dirty: false });
        app.add_plugins(SkyBackdropPlugin);
        app.update();
        let world = app.world_mut();
        let mut q = world.query::<(
            &SkyBackdropPart,
            &Visibility,
            Option<&NotShadowCaster>,
            Option<&NotShadowReceiver>,
        )>();
        let mut domes = 0;
        for (part, vis, caster, receiver) in q.iter(world) {
            if *part == SkyBackdropPart::Dome {
                domes += 1;
                assert_ne!(*vis, Visibility::Hidden, "Medium shows the dome");
                assert!(caster.is_some(), "dome casts shadows");
                assert!(receiver.is_some(), "dome receives shadows");
            } else {
                assert!(caster.is_none() && receiver.is_none(), "{part:?} tagged");
            }
        }
        assert_eq!(domes, 1);
    }

    #[test]
    fn plugin_is_safe_headless() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(SkyBackdropPlugin);
        app.update();
        app.update();
    }
}
