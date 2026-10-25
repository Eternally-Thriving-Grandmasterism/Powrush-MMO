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
//! Per preset ([`backdrop_plan`]): every Sanctuary tier draws the gradient
//! dome. Mobile keeps the floor plus one silhouette band. Low adds the
//! mountain ring, a 64-tree belt, and two bands. Medium and up keep the full
//! 200-tree belt. High and Ultra draw three or more bands. At most 11 draws
//! on any preset. Writes no `FogSettings`. No bloom, tonemap, layout or HUD
//! change.
//!
//! CARD VP-SKY-2 — the dome runs from the VP-LIGHT-1 golden-hour sun colour
//! at the horizon to a cool blue zenith. In front of the rocky ring, flat
//! unlit silhouette bands step toward the VP-AIR-1 haze blue as they recede.
//! Low uses fewer bands than Medium. The bands cast no shadow and sit past
//! the conifer belt, so the yard, the Use glow, and screen plates stay clear.
//! Contact: info@Rathor.ai

use std::f32::consts::TAU;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

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
/// Dome, floor, ridge rock, ridge snow, belt bark, belt needle, and up to
/// five silhouette bands.
pub const MAX_BACKDROP_DRAWS: usize = 11;
/// CARD VP-SKY-2 — distant silhouette bands. Index 0 is the nearest.
pub const SILHOUETTE_BAND_COUNT: usize = 5;
/// Segments around one band. One mesh, one draw.
const BAND_SEGMENTS: u32 = 80;
const CONIFER_SEED: u32 = 0x1234_5678;
pub const CONIFER_R_MIN: f32 = 30.0;
pub const CONIFER_R_MAX: f32 = 150.0;

/// Sky dome stops (sRGB).
/// CARD VP-SKY-2 — zenith is the cool high. Horizon is the VP-LIGHT-1 sun
/// (`SANCTUARY_SUN` sRGB 1.00, 0.81, 0.55): warm low, red then green then blue.
pub const SKY_ZENITH: [f32; 3] = [0.18, 0.36, 0.74];
pub const SKY_HORIZON: [f32; 3] = [1.00, 0.81, 0.55];
pub const SKY_BELOW: [f32; 3] = [0.62, 0.48, 0.34];
/// CARD VP-SKY-2 — open-yard haze the bands walk toward. Same sRGB as
/// `SANCTUARY_VALLEY_FOG` (VP-AIR-1 / VP-SKY-1). Not a second fog writer.
pub const AERIAL_HAZE_SRGB: [f32; 3] = [0.70, 0.79, 0.90];
/// Band distance (m), nearest first. Past the conifer belt, in front of the
/// rocky ring, so the yard and the well stay in front of every band.
pub const BAND_RADIUS: [f32; 5] = [180.0, 230.0, 275.0, 310.0, 335.0];
/// Ridge height (m), nearest first. Farther crests stand taller so each
/// paler band clears the skyline of the one in front.
pub const BAND_PEAK: [f32; 5] = [24.0, 42.0, 62.0, 78.0, 92.0];
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
    /// Distant flat silhouette. Index 0 is nearest.
    Band(u8),
}

/// What the backdrop draws for one realm and preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackdropPlan {
    pub dome: bool,
    pub ground: bool,
    pub peaks: usize,
    pub conifers: usize,
    /// Nearest [`bands`] silhouettes. High and Ultra are at least 3.
    pub bands: usize,
}

impl BackdropPlan {
    pub const NONE: Self = Self {
        dome: false,
        ground: false,
        peaks: 0,
        conifers: 0,
        bands: 0,
    };

    /// Draw calls: dome, ground, ridge rock + snow (one each), belt bark +
    /// needle (one each), plus one draw per silhouette band.
    pub fn draws(&self) -> usize {
        let ridge = if self.peaks > 0 {
            let snow = ring_peaks()
                .iter()
                .take(self.peaks)
                .any(|p| p.snow_capped());
            1 + usize::from(snow)
        } else {
            0
        };
        usize::from(self.dome)
            + usize::from(self.ground)
            + ridge
            + if self.conifers > 0 { 2 } else { 0 }
            + self.bands
    }

    pub fn shows(&self, part: SkyBackdropPart) -> bool {
        match part {
            SkyBackdropPart::Dome => self.dome,
            SkyBackdropPart::Ground => self.ground,
            SkyBackdropPart::Ridge => self.peaks > 0,
            SkyBackdropPart::BeltFull => self.conifers == CONIFER_COUNT,
            SkyBackdropPart::BeltLow => self.conifers == LOW_CONIFER_COUNT,
            SkyBackdropPart::Band(index) => (index as usize) < self.bands,
        }
    }
}

/// How many silhouette bands a tier draws.
///
/// Mobile is one. Low is two (fewer than the card's three). Medium is the
/// three the card names. High and Ultra add farther, paler bands. Ultra
/// keeps the full set.
pub const fn silhouette_band_count(preset: GraphicsPreset) -> usize {
    match preset {
        GraphicsPreset::Mobile => 1,
        GraphicsPreset::Low => 2,
        GraphicsPreset::Medium => 3,
        GraphicsPreset::High => 4,
        GraphicsPreset::Ultra => SILHOUETTE_BAND_COUNT,
    }
}

/// Per-preset backdrop. Outside the Sanctuary nothing draws.
pub fn backdrop_plan(realm: Option<u8>, preset: GraphicsPreset) -> BackdropPlan {
    if !matches!(realm, None | Some(0)) {
        return BackdropPlan::NONE;
    }
    let bands = silhouette_band_count(preset);
    match preset {
        GraphicsPreset::Mobile => BackdropPlan {
            dome: true,
            ground: true,
            peaks: 0,
            conifers: 0,
            bands,
        },
        GraphicsPreset::Low => BackdropPlan {
            dome: true,
            ground: true,
            peaks: PEAK_COUNT,
            conifers: LOW_CONIFER_COUNT,
            bands,
        },
        _ => BackdropPlan {
            dome: true,
            ground: true,
            peaks: PEAK_COUNT,
            conifers: CONIFER_COUNT,
            bands,
        },
    }
}

/// Flat sRGB for silhouette `index` (0 = nearest).
///
/// Farther bands are lighter, less saturated, and a wider blue-red gap, and
/// they sit closer to [`AERIAL_HAZE_SRGB`]. Blue leads on every band.
pub fn silhouette_band_srgb(index: usize) -> [f32; 3] {
    let last = (SILHOUETTE_BAND_COUNT - 1) as f32;
    let t = (index.min(SILHOUETTE_BAND_COUNT - 1) as f32) / last;
    [0.22 + 0.30 * t, 0.30 + 0.34 * t, 0.44 + 0.36 * t]
}

fn band_ridge_height(index: usize, angle: f32, peak: f32) -> f32 {
    let phase = index as f32 * 1.7;
    let n = (angle * 3.0 + phase).sin() * 0.50
        + (angle * 5.0 + phase * 2.1).sin() * 0.30
        + (angle * 11.0 + phase * 0.6).sin() * 0.20;
    let u = (n * 0.5 + 0.5).clamp(0.0, 1.0);
    let foot = peak * 0.12;
    foot + (peak - foot) * u
}

/// One distant ridgeline: a vertical ring, flat shaded, no texture.
pub fn silhouette_band_mesh(index: usize) -> Mesh {
    let radius = BAND_RADIUS[index];
    let peak = BAND_PEAK[index];
    let n = BAND_SEGMENTS;
    let mut positions = Vec::with_capacity(n as usize * 6);
    for i in 0..n {
        let a0 = i as f32 / n as f32 * TAU;
        let a1 = (i + 1) as f32 / n as f32 * TAU;
        let h0 = band_ridge_height(index, a0, peak);
        let h1 = band_ridge_height(index, a1, peak);
        let p0 = [a0.cos() * radius, -2.0, a0.sin() * radius];
        let p1 = [a1.cos() * radius, -2.0, a1.sin() * radius];
        let t0 = [a0.cos() * radius, h0, a0.sin() * radius];
        let t1 = [a1.cos() * radius, h1, a1.sin() * radius];
        for p in [p0, t0, t1, p0, t1, p1] {
            positions.push(p);
        }
    }
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

/// How far `h` (dome y / radius, 0 at the horizon) has climbed toward the zenith.
///
/// Smoothstep over the lower half of the dome. A yard camera looks only a
/// little above the horizon, so the warm stop has to hold through that band
/// and the cool stop has to arrive before the top of that frame.
fn sky_climb(h: f32) -> f32 {
    let x = (h / 0.50).clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
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
        lerp(SKY_HORIZON, SKY_ZENITH, sky_climb(h.min(1.0)))
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
            Peak {
                angle,
                radius,
                height,
                base,
            }
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
        .map(|ps| {
            ps.iter()
                .map(|p| srgb_to_linear(dome_color_at(p[1] / radius)))
                .collect()
        })
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
        for p in [
            ring[i], mid[i], ring[j], ring[j], mid[i], mid[j], mid[i], apex, mid[j],
        ] {
            positions.push(p.into());
        }
    }
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

/// Merge `parts` into one mesh. All parts must share one attribute layout.
fn merged(parts: impl IntoIterator<Item = Mesh>) -> Option<Mesh> {
    let mut it = parts.into_iter();
    let mut first = it.next()?;
    for m in it {
        first.merge(&m).expect("sky_backdrop mesh merge");
    }
    Some(first)
}

/// Mountain ring as (rock, snow) meshes.
pub fn ridge_meshes() -> (Option<Mesh>, Option<Mesh>) {
    let mut rng = Rng::new(0x9E37_79B9);
    let peaks = ring_peaks();
    let rock = merged(peaks.iter().map(|p| {
        faceted_mountain(&mut rng, p.base, p.height).transformed_by(Transform::from_xyz(
            p.angle.cos() * p.radius,
            0.0,
            p.angle.sin() * p.radius,
        ))
    }));
    let snow = merged(peaks.iter().filter(|p| p.snow_capped()).map(|p| {
        let mut cap = Mesh::from(Cone {
            radius: p.base * 0.22,
            height: p.height * 0.22,
        });
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
            Mesh::from(Cone {
                radius: 1.1,
                height: 2.2,
            })
            .transformed_by(
                Transform::from_translation(*p + Vec3::Y * y).with_scale(Vec3::splat(sc)),
            )
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

/// CARD VP-SKY-2 — flat colour, no sun tint, no distance-fog wash.
/// The depth step is baked (see [`silhouette_band_srgb`]), so Low, which
/// has no aerial fog, still reads the same blue recession as Medium.
fn silhouette_material(c: [f32; 3]) -> StandardMaterial {
    StandardMaterial {
        base_color: Color::srgb(c[0], c[1], c[2]),
        unlit: true,
        fog_enabled: false,
        cull_mode: None,
        ..default()
    }
}

fn spawn_sky_backdrop(
    mut commands: Commands,
    meshes: Option<ResMut<Assets<Mesh>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    settings: Option<Res<LocalSettingsState>>,
    mut ramps: Option<ResMut<crate::painterly_mat::PainterlyRampMeshes>>,
) {
    // Headless test apps have no render assets; the backdrop is visual only.
    let (Some(mut meshes), Some(mut materials)) = (meshes, materials) else {
        return;
    };
    // Missing settings stay Medium, same fallback as climate_plane.
    let preset = settings
        .as_ref()
        .map(|s| s.inner.graphics_preset)
        .unwrap_or(GraphicsPreset::Medium);
    // CARD VP-GRADE-1 — the dome neither casts nor receives sun shadows, so
    // the shadowed Sanctuary sun reaches the yard through it.
    commands.spawn((
        (
            Mesh3d(meshes.add(gradient_dome(DOME_RADIUS))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                fog_enabled: false,
                cull_mode: None,
                ..default()
            })),
            Transform::IDENTITY,
        ),
        SkyBackdropPart::Dome,
        Name::new("VP-SKY-1 backdrop"),
        bevy::light::NotShadowCaster,
        bevy::light::NotShadowReceiver,
    ));
    let mut spawn = |mesh: Mesh,
                     mat: StandardMaterial,
                     part: SkyBackdropPart,
                     t: Transform,
                     painted: Option<crate::painterly_mat::PainterlyMat>| {
        let handle = meshes.add(mesh);
        if let Some(painted) = painted {
            crate::painterly_mat::remember_ramp(&mut ramps, &handle, painted.role);
            commands.spawn((
                (Mesh3d(handle), MeshMaterial3d(materials.add(mat)), t),
                part,
                Name::new("VP-SKY-1 backdrop"),
                painted,
            ));
        } else {
            commands.spawn((
                (Mesh3d(handle), MeshMaterial3d(materials.add(mat)), t),
                part,
                Name::new("VP-SKY-1 backdrop"),
            ));
        }
    };
    // CARD VP-MAT-1 — valley earth hue stays. Bark stays. Foliage, ridge,
    // and snow take the painterly ramp. Bands stay the VP-SKY-2 flat colour.
    spawn(
        Plane3d::default()
            .mesh()
            .size(GROUND_SIZE, GROUND_SIZE)
            .build(),
        lit(VALLEY_EARTH, 0.95),
        SkyBackdropPart::Ground,
        Transform::from_xyz(0.0, GROUND_Y, 0.0),
        None,
    );
    let (rock, snow) = ridge_meshes();
    if let Some(mut m) = rock {
        crate::painterly_mat::paint_height_ramp(
            &mut m,
            &crate::painterly_mat::painterly_shade(
                crate::painterly_mat::PainterlyRole::SkyStone,
                preset,
            ),
        );
        spawn(
            m,
            crate::painterly_mat::painterly_material(
                crate::painterly_mat::PainterlyRole::SkyStone,
                RIDGE_ROCK,
                preset,
            ),
            SkyBackdropPart::Ridge,
            Transform::IDENTITY,
            Some(crate::painterly_mat::PainterlyMat {
                role: crate::painterly_mat::PainterlyRole::SkyStone,
                authored: RIDGE_ROCK,
            }),
        );
    }
    if let Some(mut m) = snow {
        crate::painterly_mat::paint_height_ramp(
            &mut m,
            &crate::painterly_mat::painterly_shade(
                crate::painterly_mat::PainterlyRole::Snow,
                preset,
            ),
        );
        spawn(
            m,
            crate::painterly_mat::painterly_material(
                crate::painterly_mat::PainterlyRole::Snow,
                RIDGE_SNOW,
                preset,
            ),
            SkyBackdropPart::Ridge,
            Transform::IDENTITY,
            Some(crate::painterly_mat::PainterlyMat {
                role: crate::painterly_mat::PainterlyRole::Snow,
                authored: RIDGE_SNOW,
            }),
        );
    }
    for (count, part) in [
        (CONIFER_COUNT, SkyBackdropPart::BeltFull),
        (LOW_CONIFER_COUNT, SkyBackdropPart::BeltLow),
    ] {
        let (bark, needle) = belt_meshes(count);
        if let Some(m) = bark {
            spawn(
                m,
                lit(CONIFER_BARK, 0.9),
                part,
                Transform::IDENTITY,
                None,
            );
        }
        if let Some(mut m) = needle {
            crate::painterly_mat::paint_height_ramp(
                &mut m,
                &crate::painterly_mat::painterly_shade(
                    crate::painterly_mat::PainterlyRole::Foliage,
                    preset,
                ),
            );
            spawn(
                m,
                crate::painterly_mat::painterly_material(
                    crate::painterly_mat::PainterlyRole::Foliage,
                    CONIFER_NEEDLE,
                    preset,
                ),
                part,
                Transform::IDENTITY,
                Some(crate::painterly_mat::PainterlyMat {
                    role: crate::painterly_mat::PainterlyRole::Foliage,
                    authored: CONIFER_NEEDLE,
                }),
            );
        }
    }
    for index in 0..SILHOUETTE_BAND_COUNT {
        let color = silhouette_band_srgb(index);
        commands.spawn((
            (
                Mesh3d(meshes.add(silhouette_band_mesh(index))),
                MeshMaterial3d(materials.add(silhouette_material(color))),
                Transform::IDENTITY,
            ),
            SkyBackdropPart::Band(index as u8),
            Name::new("VP-SKY-2 band"),
            bevy::light::NotShadowCaster,
            bevy::light::NotShadowReceiver,
        ));
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

    fn warmth(c: [f32; 3]) -> f32 {
        c[0] - c[2]
    }

    fn saturation(c: [f32; 3]) -> f32 {
        let mx = c[0].max(c[1]).max(c[2]);
        let mn = c[0].min(c[1]).min(c[2]);
        if mx <= 1.0e-6 {
            0.0
        } else {
            (mx - mn) / mx
        }
    }

    fn luminance(c: [f32; 3]) -> f32 {
        (c[0] + c[1] + c[2]) / 3.0
    }

    fn haze_distance(c: [f32; 3]) -> f32 {
        let d = [
            c[0] - AERIAL_HAZE_SRGB[0],
            c[1] - AERIAL_HAZE_SRGB[1],
            c[2] - AERIAL_HAZE_SRGB[2],
        ];
        (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
    }

    /// CARD VP-SKY-2 — horizon matches the golden-hour sun (warm, red leads).
    /// Zenith is the cool blue. Warmth falls as the sample climbs.
    #[test]
    fn gradient_is_warm_low_and_cool_high() {
        let sun = crate::climate_plane::SANCTUARY_SUN.color.to_srgba();
        let hor = dome_color_at(0.0);
        let zen = dome_color_at(1.0);
        assert!((hor[0] - sun.red).abs() < 1.0e-4, "horizon red {hor:?}");
        assert!((hor[1] - sun.green).abs() < 1.0e-4, "horizon green {hor:?}");
        assert!((hor[2] - sun.blue).abs() < 1.0e-4, "horizon blue {hor:?}");
        assert!(hor[0] > hor[1] && hor[1] > hor[2], "warm low {hor:?}");
        assert!(zen[2] > zen[1] && zen[1] > zen[0], "cool high {zen:?}");
        assert!(warmth(hor) > warmth(zen));
        assert!(zen[2] > hor[2], "zenith carries more blue");
        let mut prev = warmth(hor);
        for i in 1..=8 {
            let c = dome_color_at(i as f32 / 8.0);
            let w = warmth(c);
            assert!(w <= prev + 1.0e-4, "warmth rose at step {i}: {c:?}");
            prev = w;
        }
        assert!(!is_brownish(zen), "zenith {zen:?}");
    }

    /// CARD VP-SKY-2 — each step away is less saturated, a wider blue-red
    /// gap, lighter, and closer to the VP-AIR-1 haze.
    #[test]
    fn bands_get_bluer_and_desaturated_with_depth() {
        assert!(SILHOUETTE_BAND_COUNT >= 3);
        let colors: Vec<_> = (0..SILHOUETTE_BAND_COUNT)
            .map(silhouette_band_srgb)
            .collect();
        for (i, c) in colors.iter().enumerate() {
            assert!(c[2] > c[1] && c[1] > c[0], "band {i} is not blue {c:?}");
        }
        for w in colors.windows(2) {
            let (near, far) = (w[0], w[1]);
            assert!(
                saturation(far) < saturation(near) - 0.01,
                "sat {near:?} -> {far:?}"
            );
            assert!(
                far[2] - far[0] > near[2] - near[0],
                "blue-red {near:?} -> {far:?}"
            );
            assert!(luminance(far) > luminance(near));
            assert!(haze_distance(far) < haze_distance(near));
        }
    }

    /// Low uses fewer than three. Medium is the three the card names.
    /// High and Ultra are three or more, and they do not drop bands.
    #[test]
    fn preset_band_counts() {
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        assert_eq!(silhouette_band_count(GraphicsPreset::Mobile), 1);
        assert_eq!(silhouette_band_count(GraphicsPreset::Low), 2);
        assert_eq!(silhouette_band_count(GraphicsPreset::Medium), 3);
        assert_eq!(silhouette_band_count(GraphicsPreset::High), 4);
        assert_eq!(silhouette_band_count(GraphicsPreset::Ultra), 5);
        assert!(silhouette_band_count(GraphicsPreset::Low) < 3);
        assert!(silhouette_band_count(GraphicsPreset::Medium) >= 3);
        assert!(silhouette_band_count(GraphicsPreset::High) >= 3);
        assert!(silhouette_band_count(GraphicsPreset::Ultra) >= 3);
        assert!(
            silhouette_band_count(GraphicsPreset::Low)
                < silhouette_band_count(GraphicsPreset::Medium)
        );
        assert!(
            silhouette_band_count(GraphicsPreset::Medium)
                <= silhouette_band_count(GraphicsPreset::High)
        );
        for preset in GraphicsPreset::ALL {
            let plan = backdrop_plan(Some(0), preset);
            assert_eq!(plan.bands, silhouette_band_count(preset));
            assert!(plan.dome, "{preset:?} keeps the gradient");
            assert!((1..=SILHOUETTE_BAND_COUNT).contains(&plan.bands));
        }
        assert!(backdrop_visible(
            Some(0),
            GraphicsPreset::Low,
            SkyBackdropPart::Band(0)
        ));
        assert!(backdrop_visible(
            Some(0),
            GraphicsPreset::Low,
            SkyBackdropPart::Band(1)
        ));
        assert!(!backdrop_visible(
            Some(0),
            GraphicsPreset::Low,
            SkyBackdropPart::Band(2)
        ));
        assert!(backdrop_visible(
            Some(0),
            GraphicsPreset::Medium,
            SkyBackdropPart::Band(2)
        ));
        assert!(!backdrop_visible(
            Some(0),
            GraphicsPreset::Medium,
            SkyBackdropPart::Band(3)
        ));
        assert!(backdrop_visible(
            Some(0),
            GraphicsPreset::Ultra,
            SkyBackdropPart::Band(4)
        ));
        assert!(!backdrop_visible(
            Some(1),
            GraphicsPreset::Ultra,
            SkyBackdropPart::Band(0)
        ));
    }

    #[test]
    fn bands_sit_past_the_yard_and_in_front_of_the_peak_ring() {
        assert_eq!(BAND_RADIUS.len(), SILHOUETTE_BAND_COUNT);
        assert_eq!(BAND_PEAK.len(), SILHOUETTE_BAND_COUNT);
        for i in 0..SILHOUETTE_BAND_COUNT {
            assert!(BAND_RADIUS[i] > CONIFER_R_MAX, "band {i} covers the belt");
            assert!(
                BAND_RADIUS[i] < PEAK_R_MIN,
                "band {i} sits inside the rocky ring"
            );
            assert!(BAND_RADIUS[i] + BAND_PEAK[i] < DOME_RADIUS);
            let mesh = silhouette_band_mesh(i);
            assert_eq!(mesh.count_vertices(), BAND_SEGMENTS as usize * 6);
        }
        for w in BAND_RADIUS.windows(2) {
            assert!(w[1] > w[0]);
        }
        for w in BAND_PEAK.windows(2) {
            assert!(w[1] > w[0], "farther crest clears the nearer skyline");
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
            assert!(
                r >= CONIFER_R_MIN - 0.01 && r <= CONIFER_R_MAX + 0.01,
                "r {r}"
            );
            assert!((0.9..=2.5).contains(s));
        }
        assert_eq!(
            spots,
            conifer_spots(CONIFER_SEED, CONIFER_COUNT),
            "deterministic"
        );
        assert_eq!(
            &spots[..LOW_CONIFER_COUNT],
            &conifer_spots(CONIFER_SEED, LOW_CONIFER_COUNT)[..]
        );
    }

    #[test]
    fn ridge_and_belt_merge_to_one_mesh_per_material() {
        let (rock, snow) = ridge_meshes();
        let (bark, needle) = belt_meshes(CONIFER_COUNT);
        let rock = rock.expect("rock");
        let snow = snow.expect("snow");
        let bark = bark.expect("bark");
        let needle = needle.expect("needle");
        let one_tier = Mesh::from(Cone {
            radius: 1.1,
            height: 2.2,
        })
        .count_vertices();
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
            // bevy_render reorganization: Mesh3d no longer `#[require(Visibility)]`.
            // VisibilityPlugin registers that requirement (and the visibility-class hook).
            // SystemParam validation now errors inside the system instead of
            // skipping it (0.18→0.19 guide, PR #23225). VisibilityPlugin's
            // update_skinned_mesh_bounds requires this asset.
            .init_asset::<bevy::mesh::skinning::SkinnedMeshInverseBindposes>()
            .add_plugins(bevy::camera::visibility::VisibilityPlugin)
            .insert_resource(SoftPlayerRealm { current: Some(0) });
        let mut inner = LocalSettings::default();
        inner.set_graphics_preset(preset);
        app.insert_resource(LocalSettingsState {
            inner,
            dirty: false,
        });
        app.add_plugins(SkyBackdropPlugin);
        app.update();
        let one_tier = Mesh::from(Cone {
            radius: 1.1,
            height: 2.2,
        })
        .count_vertices();
        let world = app.world_mut();
        let mut q = world.query::<(
            &SkyBackdropPart,
            &Visibility,
            &Mesh3d,
            &MeshMaterial3d<StandardMaterial>,
        )>();
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
            let material = mats.get(mat).unwrap();
            let c = material.base_color.to_srgba();
            let earth = Color::srgb(VALLEY_EARTH[0], VALLEY_EARTH[1], VALLEY_EARTH[2]).to_srgba();
            let bark = Color::srgb(CONIFER_BARK[0], CONIFER_BARK[1], CONIFER_BARK[2]).to_srgba();
            let snow = Color::srgb(RIDGE_SNOW[0], RIDGE_SNOW[1], RIDGE_SNOW[2]).to_srgba();
            let needle_rgb = crate::painterly_mat::painterly_srgb(
                crate::painterly_mat::PainterlyRole::Foliage,
                CONIFER_NEEDLE,
            );
            let needle = Color::srgb(needle_rgb[0], needle_rgb[1], needle_rgb[2]).to_srgba();
            let rock_rgb = crate::painterly_mat::painterly_srgb(
                crate::painterly_mat::PainterlyRole::SkyStone,
                RIDGE_ROCK,
            );
            let rock = Color::srgb(rock_rgb[0], rock_rgb[1], rock_rgb[2]).to_srgba();
            if *part == SkyBackdropPart::Ground {
                assert_eq!(c, earth, "VP-MAT-1 leaves valley earth");
                assert_eq!(material.perceptual_roughness, 0.95);
            }
            if matches!(part, SkyBackdropPart::BeltFull | SkyBackdropPart::BeltLow) {
                assert!(
                    c == bark || c == needle,
                    "belt material drifted off bark or painterly needle: {c:?}"
                );
            }
            if *part == SkyBackdropPart::Ridge {
                assert!(
                    c == snow || c == rock,
                    "ridge material drifted off snow or painterly rock: {c:?}"
                );
                if c == rock {
                    assert!(rock.blue > rock.red);
                }
            }
            if matches!(part, SkyBackdropPart::BeltFull | SkyBackdropPart::BeltLow) && c == needle {
                conifers += meshes.get(mesh).unwrap().count_vertices() / (one_tier * 3);
            }
            ridge |= *part == SkyBackdropPart::Ridge;
        }
        (rows.len(), conifers, ridge)
    }

    #[test]
    fn preset_mobile_is_gradient_floor_and_one_band() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::Mobile);
        assert!(plan.dome && plan.ground);
        assert_eq!(
            (plan.peaks, plan.conifers, plan.bands, plan.draws()),
            (0, 0, 1, 3)
        );
        assert_eq!(visible_for(GraphicsPreset::Mobile), (3, 0, false));
    }

    #[test]
    fn preset_low_shows_gradient_ring_and_two_bands() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::Low);
        assert!(plan.dome && plan.ground);
        assert_eq!(
            (plan.peaks, plan.conifers, plan.bands, plan.draws()),
            (30, 64, 2, 8)
        );
        assert!(plan.conifers <= 64);
        assert!(plan.bands < 3);
        assert_eq!(visible_for(GraphicsPreset::Low), (8, 64, true));
    }

    #[test]
    fn preset_medium_full_valley_and_three_bands() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::Medium);
        assert!(plan.dome && plan.ground);
        assert_eq!(
            (plan.peaks, plan.conifers, plan.bands, plan.draws()),
            (30, 200, 3, 9)
        );
        assert_eq!(visible_for(GraphicsPreset::Medium), (9, 200, true));
    }

    #[test]
    fn preset_high_full_valley_and_four_bands() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::High);
        assert_eq!(
            (
                plan.dome,
                plan.peaks,
                plan.conifers,
                plan.bands,
                plan.draws()
            ),
            (true, 30, 200, 4, 10)
        );
        assert!(plan.bands >= 3);
        assert_eq!(visible_for(GraphicsPreset::High), (10, 200, true));
    }

    #[test]
    fn preset_ultra_full_valley_and_five_bands() {
        let plan = backdrop_plan(Some(0), GraphicsPreset::Ultra);
        assert_eq!(
            (
                plan.dome,
                plan.peaks,
                plan.conifers,
                plan.bands,
                plan.draws()
            ),
            (true, 30, 200, 5, 11)
        );
        assert!(plan.bands >= 3);
        assert_eq!(visible_for(GraphicsPreset::Ultra), (11, 200, true));
    }

    #[test]
    fn every_preset_within_draw_budget() {
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        for preset in GraphicsPreset::ALL {
            assert!(
                backdrop_plan(Some(0), preset).draws() <= MAX_BACKDROP_DRAWS,
                "{preset:?}"
            );
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
        assert!(
            RIDGE_ROCK[2] > RIDGE_ROCK[0],
            "ridge leans blue-violet, not beige"
        );
        assert!(!is_brownish(RIDGE_SNOW));
    }

    /// CARD VP-GRADE-1 — on Medium the dome is shown and carries
    /// `NotShadowCaster` + `NotShadowReceiver`. CARD VP-SKY-2 — the silhouette
    /// bands do too, so a low sun does not paint the yard with their shadow.
    /// Ground, ridge, and belt stay shadow-capable.
    #[test]
    fn vp_grade_medium_dome_casts_and_receives_no_shadow() {
        use bevy::light::{NotShadowCaster, NotShadowReceiver};
        use shared::local_settings::LocalSettings;
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            // bevy_render reorganization: Mesh3d no longer `#[require(Visibility)]`.
            // VisibilityPlugin registers that requirement (and the visibility-class hook).
            // SystemParam validation now errors inside the system instead of
            // skipping it (0.18→0.19 guide, PR #23225). VisibilityPlugin's
            // update_skinned_mesh_bounds requires this asset.
            .init_asset::<bevy::mesh::skinning::SkinnedMeshInverseBindposes>()
            .add_plugins(bevy::camera::visibility::VisibilityPlugin)
            .insert_resource(SoftPlayerRealm { current: Some(0) });
        let mut inner = LocalSettings::default();
        inner.set_graphics_preset(GraphicsPreset::Medium);
        app.insert_resource(LocalSettingsState {
            inner,
            dirty: false,
        });
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
        let mut bands = 0;
        for (part, vis, caster, receiver) in q.iter(world) {
            match part {
                SkyBackdropPart::Dome => {
                    domes += 1;
                    assert_ne!(*vis, Visibility::Hidden, "Medium shows the dome");
                    assert!(caster.is_some(), "dome carries NotShadowCaster");
                    assert!(receiver.is_some(), "dome carries NotShadowReceiver");
                }
                SkyBackdropPart::Band(_) => {
                    bands += 1;
                    assert!(caster.is_some(), "band carries NotShadowCaster");
                    assert!(receiver.is_some(), "band carries NotShadowReceiver");
                }
                _ => {
                    assert!(caster.is_none() && receiver.is_none(), "{part:?} tagged");
                }
            }
        }
        assert_eq!(domes, 1);
        assert_eq!(bands, SILHOUETTE_BAND_COUNT);
    }

    #[test]
    fn plugin_is_safe_headless() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(SkyBackdropPlugin);
        app.update();
        app.update();
    }

    /// Lavapipe stills for VP-SKY-2. Not part of the headless gate (`--ignored`).
    /// Same yard follow pose, Low then Medium. A readable plate and the warm
    /// well sit in front of the rings so a band that drew over them would cover
    /// those pixels.
    #[test]
    #[ignore = "lavapipe screenshot; DISPLAY and VK_ICD_FILENAMES"]
    fn vp_sky_2_lavapipe_low_and_medium_yard() {
        use shared::local_settings::LocalSettings;

        let mut settings = LocalSettings::default();
        settings.set_graphics_preset(GraphicsPreset::Low);
        let mut app = App::new();
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1280u32, 800u32).into(),
                        title: "vp-sky-2".into(),
                        ..default()
                    }),
                    ..default()
                })
                .disable::<bevy::audio::AudioPlugin>()
                .disable::<bevy::winit::WinitPlugin>(),
        )
        .add_plugins({
            let mut winit = bevy::winit::WinitPlugin::default();
            winit.run_on_any_thread = true;
            winit
        })
        .insert_resource(SoftPlayerRealm { current: Some(0) })
        .insert_resource(LocalSettingsState {
            inner: settings,
            dirty: false,
        })
        .insert_resource(GlobalAmbientLight {
            color: Color::srgb(0.70, 0.80, 1.0),
            brightness: 250.0,
            ..default()
        })
        .insert_resource(ClearColor(crate::climate_plane::sky_for(
            Some(0),
            GraphicsPreset::Low,
        )))
        .insert_resource(SkyProbe::new())
        .add_plugins(SkyBackdropPlugin)
        .add_systems(Update, drive_sky_probe);

        let plate = {
            let mut meshes = app.world_mut().resource_mut::<Assets<Mesh>>();
            meshes.add(Cuboid::new(1.15, 1.45, 0.08))
        };
        let glow = {
            let mut meshes = app.world_mut().resource_mut::<Assets<Mesh>>();
            meshes.add(Sphere::new(0.42))
        };
        let readable = {
            let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
            materials.add(StandardMaterial {
                base_color: Color::srgb(0.93, 0.91, 0.84),
                perceptual_roughness: 0.45,
                ..default()
            })
        };
        let well = {
            let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
            materials.add(StandardMaterial {
                base_color: Color::srgb(0.86, 0.66, 0.29),
                emissive: LinearRgba::new(6.0, 2.4, 0.4, 1.0),
                ..default()
            })
        };
        app.world_mut().spawn((
            Camera3d::default(),
            Camera {
                order: 0,
                ..default()
            },
            crate::climate_plane::WORLD_TONEMAPPING,
            crate::climate_plane::world_color_grading(),
            Transform::from_xyz(0.0, 3.25, 6.4).looking_at(Vec3::new(0.0, 5.2, -28.0), Vec3::Y),
            bevy::render::view::Msaa::Off,
            yard_distance_fog(GraphicsPreset::Low),
        ));
        app.world_mut().spawn((
            crate::climate_plane::SANCTUARY_SUN.bundle(),
            Name::new("SanctuarySun"),
        ));
        app.world_mut().spawn((
            Mesh3d(plate),
            MeshMaterial3d(readable),
            Transform::from_xyz(-2.4, 1.35, 3.4).looking_at(Vec3::new(0.0, 3.25, 6.4), Vec3::Y),
        ));
        app.world_mut().spawn((
            Mesh3d(glow),
            MeshMaterial3d(well),
            Transform::from_xyz(0.35, 0.7, 1.2),
        ));

        let slots = app.world().resource::<SkyProbe>().slots.clone();
        app.run();

        let slots = slots.lock().expect("sky slots");
        let low = slots.low.clone().expect("Low frame");
        let medium = slots.medium.clone().expect("Medium frame");
        let prefix = std::env::var("SHOT_PREFIX").unwrap_or_else(|_| "vp-sky-2".into());
        save_sky_png(&low, &format!("{prefix}-low.png"));
        save_sky_png(&medium, &format!("{prefix}-medium.png"));
    }
}

#[cfg(test)]
fn yard_distance_fog(preset: GraphicsPreset) -> bevy::pbr::DistanceFog {
    let bed = crate::climate_plane::fog_bed_for(Some(0), preset);
    bevy::pbr::DistanceFog {
        color: bed.fog,
        falloff: bevy::pbr::FogFalloff::Linear {
            start: bed.fog_start,
            end: bed.fog_end,
        },
        ..default()
    }
}

#[cfg(test)]
#[derive(Resource)]
struct SkyProbe {
    tx: std::sync::mpsc::Sender<bevy::image::Image>,
    rx: std::sync::Mutex<std::sync::mpsc::Receiver<bevy::image::Image>>,
    frames: u32,
    phase: u8,
    requested: bool,
    slots: std::sync::Arc<std::sync::Mutex<SkySlots>>,
}

#[cfg(test)]
struct SkySlots {
    low: Option<bevy::image::Image>,
    medium: Option<bevy::image::Image>,
}

#[cfg(test)]
impl SkyProbe {
    fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            tx,
            rx: std::sync::Mutex::new(rx),
            frames: 0,
            phase: 0,
            requested: false,
            slots: std::sync::Arc::new(std::sync::Mutex::new(SkySlots {
                low: None,
                medium: None,
            })),
        }
    }
}

#[cfg(test)]
fn drive_sky_probe(
    mut commands: Commands,
    mut probe: ResMut<SkyProbe>,
    mut settings: ResMut<LocalSettingsState>,
    mut clear: ResMut<ClearColor>,
    mut fogs: Query<&mut bevy::pbr::DistanceFog>,
    windows: Query<Entity, With<bevy::window::PrimaryWindow>>,
    mut exit: MessageWriter<AppExit>,
) {
    let warmup = if probe.phase == 0 { 100 } else { 48 };
    probe.frames += 1;
    if probe.frames >= warmup && !probe.requested {
        if let Ok(window) = windows.single() {
            probe.requested = true;
            let tx = probe.tx.clone();
            commands
                .spawn(bevy::render::view::window::screenshot::Screenshot::window(
                    window,
                ))
                .observe(
                    move |trigger: On<
                        bevy::render::view::window::screenshot::ScreenshotCaptured,
                    >| {
                        let _ = tx.send(trigger.event().image.clone());
                    },
                );
        }
    }
    let received = probe.rx.lock().expect("sky inbox").try_recv().ok();
    if let Some(image) = received {
        if sky_shot_is_blank(&image) {
            if probe.frames > 500 {
                panic!("phase {} stayed blank", probe.phase);
            }
            probe.requested = false;
            return;
        }
        let mut slots = probe.slots.lock().expect("sky slots");
        if probe.phase == 0 {
            slots.low = Some(image);
            drop(slots);
            settings.inner.set_graphics_preset(GraphicsPreset::Medium);
            clear.0 = crate::climate_plane::sky_for(Some(0), GraphicsPreset::Medium);
            let fog = yard_distance_fog(GraphicsPreset::Medium);
            for mut have in &mut fogs {
                *have = fog.clone();
            }
            probe.phase = 1;
            probe.frames = 0;
            probe.requested = false;
        } else {
            slots.medium = Some(image);
            drop(slots);
            exit.write(AppExit::Success);
        }
    } else if probe.frames > 500 {
        panic!("screenshot did not arrive in phase {}", probe.phase);
    }
}

#[cfg(test)]
fn sky_shot_is_blank(image: &bevy::image::Image) -> bool {
    let pixels = image.data.as_deref().unwrap_or(&[]);
    if pixels.len() < 64 {
        return true;
    }
    let mut n = 0.0f32;
    let mut mean = 0.0f32;
    let mut moment = 0.0f32;
    for px in pixels.chunks_exact(4) {
        let (r, g, b) = sky_rgb_of(image, px);
        let luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        n += 1.0;
        let delta = luma - mean;
        mean += delta / n;
        moment += delta * (luma - mean);
    }
    n < 10.0 || moment / n < 40.0
}

#[cfg(test)]
fn sky_rgb_of(image: &bevy::image::Image, px: &[u8]) -> (f32, f32, f32) {
    use bevy::render::render_resource::TextureFormat;
    match image.texture_descriptor.format {
        TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
            (px[0] as f32, px[1] as f32, px[2] as f32)
        }
        TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
            (px[2] as f32, px[1] as f32, px[0] as f32)
        }
        other => panic!("unexpected screenshot format {other:?}"),
    }
}

#[cfg(test)]
fn save_sky_png(image: &bevy::image::Image, name: &str) {
    let dir = std::env::var("SHOT_DIR").unwrap_or_else(|_| "/opt/cursor/artifacts".into());
    let path = std::path::Path::new(&dir).join(name);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let dynamic = image.clone().try_into_dynamic().expect("dynamic image");
    dynamic
        .to_rgb8()
        .save(&path)
        .unwrap_or_else(|err| panic!("save {}: {err}", path.display()));
}
