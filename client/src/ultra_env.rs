//! CARD VP-ULTRA-1 / GFX-IBL-1 — High + Ultra environment-map lighting, generated in code.
//!
//! No texture binary is committed: a small golden-hour sky cubemap is built
//! procedurally ([`ultra_env_texel`]) and filtered at runtime by Bevy's
//! built-in [`GeneratedEnvironmentMapLight`]. It is inserted on the world
//! [`Camera3d`] on [`GraphicsPreset::High`] and [`GraphicsPreset::Ultra`] and
//! removed on every other preset, so Mobile / Low / Medium never see it. The lived UI is a
//! `Camera2d` and is never touched. Grade, bloom and lamp emissive are not
//! touched; the light is a soft fill scaled by [`ULTRA_ENV_INTENSITY`].

use bevy::asset::RenderAssetUsages;
use bevy::light::GeneratedEnvironmentMapLight;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension,
};
use shared::local_settings::GraphicsPreset;

use crate::local_settings::LocalSettingsState;

/// Edge of each generated cubemap face (power of two, as Bevy requires).
pub const ULTRA_ENV_FACE: u32 = 32;
/// Env-map fill in cd/m². Soft: the sun and ambient stay the key light.
pub const ULTRA_ENV_INTENSITY: f32 = 350.0;
/// CARD GFX-IBL-1 — softer High fill, below Ultra; tunable on the real-PC pass.
pub const HIGH_ENV_INTENSITY: f32 = 200.0;
/// Golden-hour palette (sRGB) for the generated sky.
pub const ULTRA_ENV_ZENITH: [f32; 3] = [0.46, 0.56, 0.74];
pub const ULTRA_ENV_HORIZON: [f32; 3] = [0.92, 0.76, 0.52];
pub const ULTRA_ENV_GROUND: [f32; 3] = [0.34, 0.30, 0.18];

/// Env-map intensity for a preset: [`Some`] on High and Ultra only.
pub const fn ultra_env_for(preset: GraphicsPreset) -> Option<f32> {
    match preset {
        GraphicsPreset::High => Some(HIGH_ENV_INTENSITY),
        GraphicsPreset::Ultra => Some(ULTRA_ENV_INTENSITY),
        _ => None,
    }
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

/// Direction (unnormalised) for texel (u, v) in [-1, 1] on cube face `face`
/// (+X, -X, +Y, -Y, +Z, -Z — wgpu cube order).
fn face_dir(face: usize, u: f32, v: f32) -> Vec3 {
    match face {
        0 => Vec3::new(1.0, -v, -u),
        1 => Vec3::new(-1.0, -v, u),
        2 => Vec3::new(u, 1.0, v),
        3 => Vec3::new(u, -1.0, -v),
        4 => Vec3::new(u, -v, 1.0),
        _ => Vec3::new(-u, -v, -1.0),
    }
}

/// Pure sky colour for a cube texel: ground below the horizon, warm horizon
/// band, cooler zenith above.
pub fn ultra_env_texel(face: usize, u: f32, v: f32) -> [f32; 3] {
    let y = face_dir(face, u, v).normalize().y;
    if y < 0.0 {
        mix(ULTRA_ENV_HORIZON, ULTRA_ENV_GROUND, (-y * 4.0).min(1.0))
    } else {
        mix(ULTRA_ENV_HORIZON, ULTRA_ENV_ZENITH, y.powf(0.6))
    }
}

/// Builds the procedural cubemap image (in memory only).
pub fn ultra_env_cubemap() -> Image {
    let n = ULTRA_ENV_FACE;
    let mut data = Vec::with_capacity((n * n * 6 * 4) as usize);
    for face in 0..6 {
        for y in 0..n {
            for x in 0..n {
                let u = (x as f32 + 0.5) / n as f32 * 2.0 - 1.0;
                let v = (y as f32 + 0.5) / n as f32 * 2.0 - 1.0;
                let c = ultra_env_texel(face, u, v);
                data.extend(c.iter().map(|ch| (ch.clamp(0.0, 1.0) * 255.0).round() as u8));
                data.push(255);
            }
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: n,
            height: n,
            depth_or_array_layers: 6,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
    image
}

#[derive(Resource, Default)]
struct UltraEnvImage(Option<Handle<Image>>);

pub struct UltraEnvPlugin;

impl Plugin for UltraEnvPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UltraEnvImage>()
            .add_systems(Update, sync_ultra_env);
    }
}

fn sync_ultra_env(
    mut commands: Commands,
    settings: Option<Res<LocalSettingsState>>,
    images: Option<ResMut<Assets<Image>>>,
    mut held: ResMut<UltraEnvImage>,
    cameras: Query<(Entity, Option<&GeneratedEnvironmentMapLight>), With<Camera3d>>,
) {
    let preset = settings
        .map(|s| s.inner.graphics_preset)
        .unwrap_or(GraphicsPreset::Medium);
    match ultra_env_for(preset) {
        Some(intensity) => {
            let handle = match (&held.0, images) {
                (Some(h), _) => h.clone(),
                (None, Some(mut images)) => {
                    let h = images.add(ultra_env_cubemap());
                    held.0 = Some(h.clone());
                    h
                }
                (None, None) => Handle::default(),
            };
            // Re-insert only when missing or stale (intensity / handle), never per frame.
            for (e, has) in &cameras {
                if has.is_none_or(|c| c.intensity != intensity || c.environment_map != handle) {
                    commands.entity(e).insert(GeneratedEnvironmentMapLight {
                        environment_map: handle.clone(),
                        intensity,
                        ..default()
                    });
                }
            }
        }
        None => {
            for (e, has) in &cameras {
                if has.is_some() {
                    commands.entity(e).remove::<GeneratedEnvironmentMapLight>();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_map_is_high_and_ultra_only() {
        for p in [GraphicsPreset::Mobile, GraphicsPreset::Low, GraphicsPreset::Medium] {
            assert_eq!(ultra_env_for(p), None, "{p:?}");
        }
        assert_eq!(ultra_env_for(GraphicsPreset::High), Some(HIGH_ENV_INTENSITY));
        assert_eq!(ultra_env_for(GraphicsPreset::Ultra), Some(ULTRA_ENV_INTENSITY));
        assert!(0.0 < HIGH_ENV_INTENSITY && HIGH_ENV_INTENSITY < ULTRA_ENV_INTENSITY);
        assert!(ULTRA_ENV_FACE.is_power_of_two());
    }

    #[test]
    fn generated_sky_is_warm_horizon_cool_zenith_dark_ground() {
        let zen = ultra_env_texel(2, 0.0, 0.0);
        let gnd = ultra_env_texel(3, 0.0, 0.0);
        let hor = ultra_env_texel(4, 0.0, 0.0);
        assert!(zen[2] > zen[0], "zenith cooler");
        assert!(hor[0] > hor[2], "horizon warm");
        assert!(gnd.iter().sum::<f32>() < hor.iter().sum::<f32>(), "ground darker");
        let img = ultra_env_cubemap();
        assert_eq!(img.texture_descriptor.size.depth_or_array_layers, 6);
    }

    #[test]
    fn camera_gets_env_only_on_high_ultra_and_ui_never() {
        let mut app = App::new();
        let mut s = LocalSettingsState::default();
        s.inner.graphics_preset = GraphicsPreset::High;
        app.insert_resource(s).add_plugins(UltraEnvPlugin);
        let cam = app.world_mut().spawn(Camera3d::default()).id();
        let ui = app.world_mut().spawn(Camera2d).id();
        for (p, want) in [
            (GraphicsPreset::High, true),
            (GraphicsPreset::Ultra, true),
            (GraphicsPreset::Medium, false),
            (GraphicsPreset::Ultra, true),
            (GraphicsPreset::Low, false),
        ] {
            app.world_mut().resource_mut::<LocalSettingsState>().inner.graphics_preset = p;
            app.update();
            assert_eq!(app.world().entity(cam).contains::<GeneratedEnvironmentMapLight>(), want, "{p:?}");
            assert!(!app.world().entity(ui).contains::<GeneratedEnvironmentMapLight>());
        }
    }

    /// CARD GFX-IBL-1 — one camera follows High→Ultra→High→Medium at 200, 350,
    /// 200, then removed; an idle frame never re-inserts; UI never gets it.
    #[test]
    fn env_intensity_follows_preset_switch_without_idle_reinsert() {
        let mut app = App::new();
        app.insert_resource(LocalSettingsState::default()).add_plugins(UltraEnvPlugin);
        let cam = app.world_mut().spawn(Camera3d::default()).id();
        let ui = app.world_mut().spawn(Camera2d).id();
        for (p, want) in [
            (GraphicsPreset::High, Some(200.0)),
            (GraphicsPreset::Ultra, Some(350.0)),
            (GraphicsPreset::High, Some(200.0)),
            (GraphicsPreset::Medium, None),
        ] {
            app.world_mut().resource_mut::<LocalSettingsState>().inner.graphics_preset = p;
            app.update();
            let got = app.world().entity(cam).get::<GeneratedEnvironmentMapLight>().map(|l| l.intensity);
            assert_eq!(got, want, "{p:?}");
            assert!(!app.world().entity(ui).contains::<GeneratedEnvironmentMapLight>());
            if want.is_some() {
                let before = app.world().entity(cam).get_change_ticks::<GeneratedEnvironmentMapLight>().unwrap();
                app.update();
                let after = app.world().entity(cam).get_change_ticks::<GeneratedEnvironmentMapLight>().unwrap();
                assert_eq!((after.added, after.changed), (before.added, before.changed), "idle {p:?}");
            }
        }
    }

    /// CARD VP-ULTRA-1 — Ultra is a real superset of High on every tier knob,
    /// and Mobile / Low / Medium values are pinned unchanged.
    #[test]
    fn ultra_is_high_superset_and_lower_tiers_unchanged() {
        use crate::climate_plane::{aerial_haze_for, ssao_for, ultra_volumetric_for};
        use crate::local_settings::{anti_alias_for, ssr_for, AntiAliasKind};
        use crate::vegetation::vegetation_plan;
        use bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel as Q;
        let (h, u) = (GraphicsPreset::High, GraphicsPreset::Ultra);

        let rank = |q: Q| match q {
            Q::Low => 0,
            Q::Medium => 1,
            Q::High => 2,
            Q::Ultra => 3,
            _ => 4,
        };
        let (sh, su) = (ssao_for(h).unwrap(), ssao_for(u).unwrap());
        assert!(rank(su.quality_level) > rank(sh.quality_level));
        assert_eq!(rank(sh.quality_level), 2, "High SSAO unchanged");

        let (rh, ru) = (ssr_for(h).unwrap(), ssr_for(u).unwrap());
        assert!(ru.linear_steps >= rh.linear_steps && ru.bisection_steps >= rh.bisection_steps);

        let (ah, au) = (aerial_haze_for(h).unwrap(), aerial_haze_for(u).unwrap());
        assert!(au.fog_end >= ah.fog_end && au.fog_start >= ah.fog_start);
        assert_eq!((ah.fog_start, ah.fog_end), (24.0, 480.0), "High haze unchanged");

        let (vh, vu) = (vegetation_plan(h), vegetation_plan(u));
        assert!(vu.tufts >= vh.tufts && vu.shrubs >= vh.shrubs && vu.trees >= vh.trees);
        assert!(vu.reach >= vh.reach && vu.fine_segments >= vh.fine_segments);

        assert!(ultra_volumetric_for(u).is_some() && ultra_volumetric_for(h).is_none());
        assert_eq!((ultra_env_for(h), ultra_env_for(u)), (Some(HIGH_ENV_INTENSITY), Some(ULTRA_ENV_INTENSITY)));
        assert_eq!(anti_alias_for(h), AntiAliasKind::Smaa, "High AA unchanged");
        assert_eq!(anti_alias_for(u), AntiAliasKind::Taa, "one AA path on Ultra");

        // High pinned byte-for-byte to main (VP-AO-1 / VP-WATER-1 values).
        assert_eq!(sh, bevy::pbr::ScreenSpaceAmbientOcclusion {
            quality_level: Q::High,
            radius: 0.62,
            constant_object_thickness: 0.25,
        });
        assert_eq!((rh.linear_steps, rh.bisection_steps, rh.use_secant, rh.thickness), (6, 2, false, 0.25));
        assert_eq!((vh.tufts, vh.shrubs, vh.trees, vh.reach), (280, 72, 28, 48.0));

        // Lower tiers pinned.
        for (p, aa, haze, tufts) in [
            (GraphicsPreset::Mobile, AntiAliasKind::Fxaa, None, 16),
            (GraphicsPreset::Low, AntiAliasKind::Fxaa, None, 24),
            (GraphicsPreset::Medium, AntiAliasKind::Smaa, Some((28.0, 600.0)), 40),
        ] {
            assert_eq!(anti_alias_for(p), aa, "{p:?}");
            assert!(ssao_for(p).is_none() && ssr_for(p).is_none(), "{p:?}");
            assert_eq!(aerial_haze_for(p).map(|a| (a.fog_start, a.fog_end)), haze, "{p:?}");
            assert_eq!(vegetation_plan(p).tufts, tufts, "{p:?}");
            assert!(ultra_volumetric_for(p).is_none() && ultra_env_for(p).is_none(), "{p:?}");
        }
    }
}
