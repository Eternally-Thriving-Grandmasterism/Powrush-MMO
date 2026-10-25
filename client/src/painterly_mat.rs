//! CARD VP-MAT-1 — painterly materials and palette.
//!
//! Non-ground materials move toward the TARGET_BOARD palette: lively valley
//! colour, sky blues, golden-hour warms. Each Place keeps the one accent it
//! already has. Ground and earth hues are returned unchanged, so the open
//! earth-tone question (muted earth vs no brown/beige) stays open.
//!
//! The colour ramp is vertex colour on the existing `StandardMaterial`
//! shader (`VERTEX_COLORS` multiplies `base_color`). No extra pass, no
//! texture, no new asset. Mobile, Low, and Medium use two stops (cool
//! shadow, then the base). High and Ultra add a golden-hour tip and a warm
//! specular tint. Roughness stays soft on every tier. Fog, the sun, and the
//! sky dome stay with VP-AIR-1, VP-LIGHT-1, and VP-SKY-2.
//!
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::local_settings::GraphicsPreset;

/// What a material is allowed to do under VP-MAT-1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PainterlyRole {
    /// Yard, valley, and Place ground. Hue is not retuned.
    Ground,
    /// Bark, wood, and other existing earth hues. Hue is not retuned.
    Earth,
    /// Foliage. Lively valley green.
    Foliage,
    /// Distant rock. Sky blue.
    SkyStone,
    /// Snow. Authored hue stays. The ramp only cools the shade.
    Snow,
}

/// Lively valley green foliage walks toward. Green leads both other channels.
const VALLEY_GREEN: [f32; 3] = [0.22, 0.55, 0.24];
/// Sky blue the dull ridge walks toward. Blue leads.
const SKY_BLUE_ROCK: [f32; 3] = [0.28, 0.44, 0.72];
/// How far a retuned role moves toward its target. Ground and earth stay at 0.
const PALETTE_MIX: f32 = 0.78;

/// Shading plan for one role and preset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PainterlyShade {
    pub roughness: f32,
    pub reflectance: f32,
    /// Vertex-colour multiplier at the foot. sRGB.
    pub shadow: [f32; 3],
    /// Vertex-colour multiplier at the body. sRGB.
    pub mid: [f32; 3],
    /// Vertex-colour multiplier at the crown. Used when [`stops`] is 3.
    pub tip: [f32; 3],
    /// 1 = flat (ground / earth). 2 = simple ramp. 3 = golden tip.
    pub stops: u8,
    pub specular_tint: Color,
}

/// Authored sRGB after the palette step. Ground and earth come back identical.
pub fn painterly_srgb(role: PainterlyRole, authored: [f32; 3]) -> [f32; 3] {
    match role {
        PainterlyRole::Ground | PainterlyRole::Earth | PainterlyRole::Snow => authored,
        PainterlyRole::Foliage => mix(authored, VALLEY_GREEN, PALETTE_MIX),
        PainterlyRole::SkyStone => mix(authored, SKY_BLUE_ROCK, PALETTE_MIX),
    }
}

/// Soft roughness on every tier. Low and Medium (and Mobile) share the
/// simpler two-stop path. High and Ultra share the golden tip.
pub fn painterly_shade(role: PainterlyRole, preset: GraphicsPreset) -> PainterlyShade {
    let simple = matches!(
        preset,
        GraphicsPreset::Mobile | GraphicsPreset::Low | GraphicsPreset::Medium
    );
    let flat = PainterlyShade {
        roughness: 0.95,
        reflectance: 0.5,
        shadow: [1.0, 1.0, 1.0],
        mid: [1.0, 1.0, 1.0],
        tip: [1.0, 1.0, 1.0],
        stops: 1,
        specular_tint: Color::WHITE,
    };
    match role {
        PainterlyRole::Ground | PainterlyRole::Earth => flat,
        PainterlyRole::Foliage => shade_for(
            simple,
            [0.72, 0.86, 1.0],
            [1.0, 0.93, 0.74],
        ),
        PainterlyRole::SkyStone => shade_for(
            simple,
            [0.64, 0.76, 1.0],
            [1.0, 0.90, 0.72],
        ),
        PainterlyRole::Snow => shade_for(
            simple,
            [0.80, 0.88, 1.0],
            [1.0, 0.97, 0.92],
        ),
    }
}

fn shade_for(simple: bool, shadow: [f32; 3], tip: [f32; 3]) -> PainterlyShade {
    if simple {
        PainterlyShade {
            roughness: 0.97,
            reflectance: 0.12,
            shadow,
            mid: [1.0, 1.0, 1.0],
            tip,
            stops: 2,
            specular_tint: Color::WHITE,
        }
    } else {
        PainterlyShade {
            // Still soft. A little tighter so the warm specular can read.
            roughness: 0.90,
            reflectance: 0.42,
            shadow,
            mid: [1.0, 1.0, 1.0],
            tip,
            stops: 3,
            // Same sRGB family as the VP-LIGHT-1 sun (1.00, 0.81, 0.55).
            specular_tint: Color::srgb(1.0, 0.81, 0.55),
        }
    }
}

/// sRGB vertex multiplier at height `t` in 0..=1.
pub fn ramp_srgb(shade: &PainterlyShade, t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    match shade.stops {
        0 | 1 => [1.0, 1.0, 1.0],
        2 => mix(shade.shadow, shade.mid, t),
        _ => {
            if t < 0.55 {
                mix(shade.shadow, shade.mid, t / 0.55)
            } else {
                mix(shade.mid, shade.tip, (t - 0.55) / 0.45)
            }
        }
    }
}

/// Standard material for one role. Metallic stays 0. No texture.
pub fn painterly_material(
    role: PainterlyRole,
    authored: [f32; 3],
    preset: GraphicsPreset,
) -> StandardMaterial {
    let shade = painterly_shade(role, preset);
    let c = painterly_srgb(role, authored);
    StandardMaterial {
        base_color: Color::srgb(c[0], c[1], c[2]),
        perceptual_roughness: shade.roughness,
        reflectance: shade.reflectance,
        specular_tint: shade.specular_tint,
        metallic: 0.0,
        ..default()
    }
}

/// Marker so a preset change can retune the shared material.
#[derive(Component, Clone, Copy, Debug)]
pub struct PainterlyMat {
    pub role: PainterlyRole,
    pub authored: [f32; 3],
}

/// Meshes whose vertex colours follow [`painterly_shade`].
#[derive(Resource, Default)]
pub struct PainterlyRampMeshes {
    pub meshes: Vec<(Handle<Mesh>, PainterlyRole)>,
}

pub struct PainterlyPlugin;

impl Plugin for PainterlyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PainterlyRampMeshes>()
            .add_systems(Update, sync_painterly_preset);
    }
}

fn sync_painterly_preset(
    settings: Option<Res<crate::local_settings::LocalSettingsState>>,
    mut materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    ramps: Option<Res<PainterlyRampMeshes>>,
    painted: Query<(&PainterlyMat, &MeshMaterial3d<StandardMaterial>)>,
    mut applied: Local<Option<GraphicsPreset>>,
) {
    let preset = settings
        .as_ref()
        .map(|s| s.inner.graphics_preset)
        .unwrap_or(GraphicsPreset::Medium);
    if *applied == Some(preset) {
        return;
    }
    if let Some(materials) = materials.as_mut() {
        for (src, handle) in &painted {
            if let Some(mut mat) = materials.get_mut(handle) {
                let shade = painterly_shade(src.role, preset);
                let c = painterly_srgb(src.role, src.authored);
                mat.base_color = Color::srgb(c[0], c[1], c[2]);
                mat.perceptual_roughness = shade.roughness;
                mat.reflectance = shade.reflectance;
                mat.specular_tint = shade.specular_tint;
                mat.metallic = 0.0;
            }
        }
    }
    if let (Some(meshes), Some(ramps)) = (meshes.as_mut(), ramps.as_ref()) {
        for (handle, role) in &ramps.meshes {
            if let Some(mut mesh) = meshes.get_mut(handle) {
                paint_height_ramp(&mut mesh, &painterly_shade(*role, preset));
            }
        }
    }
    *applied = Some(preset);
}

/// Bake the height ramp into vertex colour. Positions are mesh-local.
/// A flat shade (ground / earth) writes white, so the authored hue holds.
pub fn paint_height_ramp(mesh: &mut Mesh, shade: &PainterlyShade) {
    let Some(values) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
        return;
    };
    let Some(positions) = values.as_float3() else {
        return;
    };
    let positions: Vec<[f32; 3]> = positions.to_vec();
    if positions.is_empty() {
        return;
    }
    let mut y_min = f32::MAX;
    let mut y_max = f32::MIN;
    for p in &positions {
        y_min = y_min.min(p[1]);
        y_max = y_max.max(p[1]);
    }
    let span = (y_max - y_min).max(1.0e-4);
    let cols: Vec<[f32; 4]> = positions
        .iter()
        .map(|p| {
            let t = ((p[1] - y_min) / span).clamp(0.0, 1.0);
            srgb_to_linear(ramp_srgb(shade, t))
        })
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, cols);
}

pub fn remember_ramp(
    ramps: &mut Option<ResMut<PainterlyRampMeshes>>,
    handle: &Handle<Mesh>,
    role: PainterlyRole,
) {
    if let Some(ramps) = ramps.as_mut() {
        ramps.meshes.push((handle.clone(), role));
    }
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

/// Same approx the sky dome uses, so the ramp sits in that vertex space.
fn srgb_to_linear(c: [f32; 3]) -> [f32; 4] {
    let f = |v: f32| v.max(0.0).powf(2.2);
    [f(c[0]), f(c[1]), f(c[2]), 1.0]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ground and earth literals already in the client. VP-MAT-1 must
    /// return each one unchanged.
    const EARTH_HUES: [[f32; 3]; 10] = [
        [0.33, 0.40, 0.20], // Sanctuary valley earth
        [0.20, 0.15, 0.07], // Heartwood ground
        [0.12, 0.12, 0.15], // Threshold ground
        [0.04, 0.08, 0.09], // Depths ground
        [0.30, 0.22, 0.15], // conifer bark
        [0.28, 0.18, 0.10], // yard trunk
        [0.26, 0.16, 0.09], // light-gen wood
        [0.42, 0.30, 0.18], // deer
        [0.32, 0.30, 0.28], // ecology stone
        [0.30, 0.28, 0.26], // light-gen stone
    ];

    #[test]
    fn ground_and_earth_hues_pass_through() {
        for c in EARTH_HUES {
            assert_eq!(painterly_srgb(PainterlyRole::Ground, c), c);
            assert_eq!(painterly_srgb(PainterlyRole::Earth, c), c);
            let shade = painterly_shade(PainterlyRole::Ground, GraphicsPreset::Ultra);
            assert_eq!(shade.stops, 1);
            assert_eq!(ramp_srgb(&shade, 0.0), [1.0, 1.0, 1.0]);
            assert_eq!(ramp_srgb(&shade, 1.0), [1.0, 1.0, 1.0]);
        }
        assert_eq!(
            painterly_srgb(PainterlyRole::Snow, [0.95, 0.96, 1.0]),
            [0.95, 0.96, 1.0]
        );
    }

    #[test]
    fn foliage_is_livelier_and_still_green() {
        let authored = [0.10, 0.30, 0.16];
        let g = painterly_srgb(PainterlyRole::Foliage, authored);
        assert!(g[1] > authored[1], "valley green lifts");
        assert!(g[1] > g[0] + 0.08 && g[1] > g[2] + 0.08);
        assert!(g[0] < 0.40 && g[2] < 0.40);
    }

    #[test]
    fn ridge_walks_toward_sky_blue() {
        let authored = [0.36, 0.36, 0.50];
        let rock = painterly_srgb(PainterlyRole::SkyStone, authored);
        assert!(rock[2] > authored[2]);
        assert!(rock[2] > rock[0] + 0.2);
        assert!(rock[2] > rock[1]);
        let brown = rock[0] >= rock[1] && rock[1] >= rock[2] && rock[0] - rock[2] >= 0.03;
        assert!(!brown, "ridge stays off the earth family");
    }

    #[test]
    fn low_and_medium_use_the_simpler_ramp() {
        for preset in [
            GraphicsPreset::Mobile,
            GraphicsPreset::Low,
            GraphicsPreset::Medium,
        ] {
            let shade = painterly_shade(PainterlyRole::Foliage, preset);
            assert_eq!(shade.stops, 2, "{preset:?}");
            assert!(shade.roughness >= 0.94);
            assert!(shade.reflectance < 0.2);
            assert_eq!(shade.specular_tint, Color::WHITE);
            assert_eq!(ramp_srgb(&shade, 0.0), shade.shadow);
            assert_eq!(ramp_srgb(&shade, 1.0), [1.0, 1.0, 1.0]);
        }
        assert_eq!(
            painterly_shade(PainterlyRole::Foliage, GraphicsPreset::Mobile),
            painterly_shade(PainterlyRole::Foliage, GraphicsPreset::Low)
        );
        assert_eq!(
            painterly_shade(PainterlyRole::SkyStone, GraphicsPreset::Low),
            painterly_shade(PainterlyRole::SkyStone, GraphicsPreset::Medium)
        );
    }

    #[test]
    fn high_and_ultra_add_a_golden_tip() {
        let simple = painterly_shade(PainterlyRole::Foliage, GraphicsPreset::Medium);
        for preset in [GraphicsPreset::High, GraphicsPreset::Ultra] {
            let shade = painterly_shade(PainterlyRole::Foliage, preset);
            assert_eq!(shade.stops, 3, "{preset:?}");
            assert!(shade.roughness < simple.roughness);
            assert!(shade.roughness >= 0.85);
            assert_eq!(ramp_srgb(&shade, 1.0), shade.tip);
            assert!(shade.tip[0] > shade.tip[2], "tip is warm");
            let tint = shade.specular_tint.to_srgba();
            assert!(tint.red > tint.green && tint.green > tint.blue);
        }
        assert_eq!(
            painterly_shade(PainterlyRole::Foliage, GraphicsPreset::High),
            painterly_shade(PainterlyRole::Foliage, GraphicsPreset::Ultra)
        );
        assert_eq!(
            painterly_shade(PainterlyRole::SkyStone, GraphicsPreset::High),
            painterly_shade(PainterlyRole::SkyStone, GraphicsPreset::Ultra)
        );
    }

    #[test]
    fn material_stays_soft_dielectric() {
        let mat = painterly_material(
            PainterlyRole::Foliage,
            [0.10, 0.30, 0.16],
            GraphicsPreset::Low,
        );
        assert_eq!(mat.metallic, 0.0);
        assert!(mat.perceptual_roughness >= 0.94);
        assert!(!mat.unlit);
        assert!(mat.base_color_texture.is_none());
    }

    #[test]
    fn height_ramp_paints_one_colour_per_vertex() {
        let mut mesh = Mesh::from(Cone {
            radius: 1.0,
            height: 2.0,
        });
        let before = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .len();
        paint_height_ramp(
            &mut mesh,
            &painterly_shade(PainterlyRole::Foliage, GraphicsPreset::Low),
        );
        let colors = mesh.attribute(Mesh::ATTRIBUTE_COLOR).expect("ramp");
        assert_eq!(colors.len(), before);
        let mut flat = Mesh::from(Cone {
            radius: 1.0,
            height: 2.0,
        });
        paint_height_ramp(
            &mut flat,
            &painterly_shade(PainterlyRole::Ground, GraphicsPreset::High),
        );
        let cols = flat.attribute(Mesh::ATTRIBUTE_COLOR).unwrap();
        let bevy::mesh::VertexAttributeValues::Float32x4(values) = cols else {
            panic!("colour is float4");
        };
        for c in values {
            assert_eq!(*c, [1.0, 1.0, 1.0, 1.0]);
        }
    }
}
