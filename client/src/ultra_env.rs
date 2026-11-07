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

    /// CPU-ASSET-PLATEAU-1. CPU asset ids, counts, and ownership for the six
    /// production plugins. The empty fade cache is only the exclusion of
    /// `fade_occluding_trunks`. This is not a leak, GPU, thermal, crash, or
    /// release result.
    #[test]
    fn cpu_asset_plateau_1_counts_and_ids_within_a_visit() {
        use std::collections::BTreeSet;

        use bevy::asset::{AssetApp, AssetId, AssetPlugin};
        use shared::heartwood_lamp::HEARTWOOD_LIP_INSTANCES;
        use shared::heartwood_wards::WARD_POST_CENTERS;
        use shared::hex_travel::PlaceId;
        use shared::local_settings::LocalSettings;

        use crate::heartwood_lip::HeartwoodLipPlugin;
        use crate::heartwood_wards::HeartwoodWardsPlugin;
        use crate::hex_travel::HexTravelState;
        use crate::human_presence::SoftPresence;
        use crate::input::PlayerInput;
        use crate::living_practice_loop::SoftPlayerRealm;
        use crate::painterly_mat::{PainterlyPlugin, PainterlyRampMeshes};
        use crate::painterly_shader::PainterlyMaterial;
        use crate::vegetation::{TrunkFadeCache, VegetationPlugin};
        use crate::water::WaterPlugin;

        /// Walkway, rib, shelf, orb, and bath. One material each.
        const LIP_MESHES: usize = 5;
        /// Shelf, peace orb, and bath, beside [`HEARTWOOD_LIP_INSTANCES`].
        const LIP_EXTRA_ENTITIES: usize = 3;
        /// One shared post mesh and one shared post material.
        const WARD_MESHES: usize = 1;
        /// Tuft, shrub, and canopy. Trunks are [`StandardMaterial`].
        const FOLIAGE_MATERIALS: usize = 3;

        #[derive(Clone, Copy, Debug)]
        struct Cell {
            preset: GraphicsPreset,
            realm: Option<u8>,
            place: PlaceId,
        }

        struct Expect {
            meshes: usize,
            standard: usize,
            painterly: usize,
            image: usize,
            mesh_entities: usize,
            standard_holders: usize,
            painterly_holders: usize,
            ramp: usize,
            env_lights: usize,
        }

        #[derive(PartialEq, Eq, Debug)]
        struct AssetIds {
            mesh: BTreeSet<AssetId<Mesh>>,
            standard: BTreeSet<AssetId<StandardMaterial>>,
            painterly: BTreeSet<AssetId<PainterlyMaterial>>,
            image: BTreeSet<AssetId<Image>>,
        }

        fn matrix() -> [Cell; 45] {
            let presets = [
                GraphicsPreset::Mobile,
                GraphicsPreset::Low,
                GraphicsPreset::Medium,
                GraphicsPreset::High,
                GraphicsPreset::Ultra,
            ];
            let realms = [None, Some(0), Some(1)];
            let places = [PlaceId::Sanctuary, PlaceId::Heartwood, PlaceId::Depths];
            let mut cells = [Cell {
                preset: GraphicsPreset::Mobile,
                realm: None,
                place: PlaceId::Sanctuary,
            }; 45];
            let mut index = 0;
            for preset in presets {
                for realm in realms {
                    for place in places {
                        cells[index] = Cell { preset, realm, place };
                        index += 1;
                    }
                }
            }
            cells
        }

        fn expect(cell: Cell, image_live: bool) -> Expect {
            let shown = matches!(cell.realm, None | Some(0));
            let (draws, instances, trees, plants, veg_meshes, ramp) = if shown {
                match cell.preset {
                    GraphicsPreset::Mobile => (1, 24, 2, 22, 4, 3),
                    GraphicsPreset::Low => (1, 40, 4, 36, 4, 3),
                    GraphicsPreset::Medium => (2, 64, 6, 58, 4, 3),
                    GraphicsPreset::High => (2, 408, 28, 380, 8, 6),
                    GraphicsPreset::Ultra => (3, 736, 48, 688, 8, 6),
                }
            } else {
                (0, 0, 0, 0, 0, 0)
            };
            let heartwood = cell.place == PlaceId::Heartwood;
            let hw_meshes = if heartwood { LIP_MESHES + WARD_MESHES } else { 0 };
            let hw_entities = if heartwood {
                HEARTWOOD_LIP_INSTANCES.len() + LIP_EXTRA_ENTITIES + WARD_POST_CENTERS.len()
            } else {
                0
            };
            let water_and_trunk = usize::from(shown) + usize::from(shown);
            Expect {
                meshes: draws + veg_meshes + hw_meshes,
                standard: water_and_trunk + hw_meshes,
                painterly: if shown { FOLIAGE_MATERIALS } else { 0 },
                image: usize::from(image_live),
                mesh_entities: draws + instances + hw_entities,
                standard_holders: draws + trees + hw_entities,
                painterly_holders: plants,
                ramp,
                env_lights: usize::from(ultra_env_for(cell.preset).is_some()),
            }
        }

        fn pin_oracle() {
            assert_eq!(WARD_POST_CENTERS.len(), 3);
            assert_eq!(HEARTWOOD_LIP_INSTANCES.len(), 7);
            assert_eq!(matrix().len(), 45);
            let row = |preset, realm, place| expect(Cell { preset, realm, place }, true);
            let numbers = |expect: Expect| {
                (
                    expect.meshes,
                    expect.standard,
                    expect.painterly,
                    expect.image,
                    expect.mesh_entities,
                    expect.standard_holders,
                    expect.painterly_holders,
                    expect.ramp,
                )
            };
            assert_eq!(numbers(row(GraphicsPreset::Mobile, None, PlaceId::Sanctuary)), (5, 2, 3, 1, 25, 3, 22, 3));
            assert_eq!(numbers(row(GraphicsPreset::Low, Some(0), PlaceId::Depths)), (5, 2, 3, 1, 41, 5, 36, 3));
            assert_eq!(numbers(row(GraphicsPreset::Medium, None, PlaceId::Sanctuary)), (6, 2, 3, 1, 66, 8, 58, 3));
            assert_eq!(numbers(row(GraphicsPreset::High, Some(0), PlaceId::Depths)), (10, 2, 3, 1, 410, 30, 380, 6));
            assert_eq!(numbers(row(GraphicsPreset::Ultra, None, PlaceId::Sanctuary)), (11, 2, 3, 1, 739, 51, 688, 6));
            assert_eq!(numbers(row(GraphicsPreset::Mobile, Some(0), PlaceId::Heartwood)), (11, 8, 3, 1, 38, 16, 22, 3));
            assert_eq!(numbers(row(GraphicsPreset::Low, None, PlaceId::Heartwood)), (11, 8, 3, 1, 54, 18, 36, 3));
            assert_eq!(numbers(row(GraphicsPreset::Medium, Some(0), PlaceId::Heartwood)), (12, 8, 3, 1, 79, 21, 58, 3));
            assert_eq!(numbers(row(GraphicsPreset::High, None, PlaceId::Heartwood)), (16, 8, 3, 1, 423, 43, 380, 6));
            assert_eq!(numbers(row(GraphicsPreset::Ultra, Some(0), PlaceId::Heartwood)), (17, 8, 3, 1, 752, 64, 688, 6));
            assert_eq!(numbers(row(GraphicsPreset::High, Some(1), PlaceId::Sanctuary)), (0, 0, 0, 1, 0, 0, 0, 0));
            assert_eq!(numbers(row(GraphicsPreset::Ultra, Some(1), PlaceId::Heartwood)), (6, 6, 0, 1, 13, 13, 0, 0));
            assert_eq!(expect(Cell {
                preset: GraphicsPreset::Medium,
                realm: None,
                place: PlaceId::Sanctuary,
            }, false).image, 0);
            assert_eq!(row(GraphicsPreset::High, None, PlaceId::Sanctuary).env_lights, 1);
            assert_eq!(row(GraphicsPreset::Medium, None, PlaceId::Sanctuary).env_lights, 0);
        }

        fn write_cell(app: &mut App, cell: Cell) {
            let mut inner = LocalSettings::peace_defaults();
            inner.set_graphics_preset(cell.preset);
            app.insert_resource(LocalSettingsState { inner, dirty: false });
            app.insert_resource(SoftPlayerRealm { current: cell.realm });
            app.insert_resource(HexTravelState { current: cell.place });
        }

        fn asset_ids<A: bevy::asset::Asset>(app: &App) -> BTreeSet<AssetId<A>> {
            app.world().resource::<Assets<A>>().ids().collect()
        }

        fn snapshot(app: &App) -> AssetIds {
            AssetIds {
                mesh: asset_ids(app),
                standard: asset_ids(app),
                painterly: asset_ids(app),
                image: asset_ids(app),
            }
        }

        fn mesh_holders(app: &mut App) -> (usize, BTreeSet<AssetId<Mesh>>) {
            let world = app.world_mut();
            let mut query = world.query::<&Mesh3d>();
            let mut ids = BTreeSet::new();
            let mut count = 0;
            for mesh in query.iter(world) {
                count += 1;
                ids.insert(mesh.0.id());
            }
            (count, ids)
        }

        fn standard_holders(app: &mut App) -> (usize, BTreeSet<AssetId<StandardMaterial>>) {
            let world = app.world_mut();
            let mut query = world.query::<&MeshMaterial3d<StandardMaterial>>();
            let mut ids = BTreeSet::new();
            let mut count = 0;
            for material in query.iter(world) {
                count += 1;
                ids.insert(material.0.id());
            }
            (count, ids)
        }

        fn painterly_holders(app: &mut App) -> (usize, BTreeSet<AssetId<PainterlyMaterial>>) {
            let world = app.world_mut();
            let mut query = world.query::<&MeshMaterial3d<PainterlyMaterial>>();
            let mut ids = BTreeSet::new();
            let mut count = 0;
            for material in query.iter(world) {
                count += 1;
                ids.insert(material.0.id());
            }
            (count, ids)
        }

        fn ramp_ids(app: &App) -> (usize, BTreeSet<AssetId<Mesh>>) {
            let ramp = &app.world().resource::<PainterlyRampMeshes>().meshes;
            let ids = ramp.iter().map(|(handle, _)| handle.id()).collect();
            (ramp.len(), ids)
        }

        fn env_light_ids(app: &mut App) -> Vec<AssetId<Image>> {
            let world = app.world_mut();
            let mut query = world.query::<&GeneratedEnvironmentMapLight>();
            query.iter(world).map(|light| light.environment_map.id()).collect()
        }

        fn assert_resolved<A: bevy::asset::Asset>(app: &App, ids: &BTreeSet<AssetId<A>>, label: &str) {
            let assets = app.world().resource::<Assets<A>>();
            for id in ids {
                assert!(assets.get(*id).is_some(), "{label} unresolved {id:?}");
            }
        }

        fn check(app: &mut App, cell: Cell, kept: &mut Option<AssetId<Image>>, label: &str) {
            let image_live = kept.is_some() || ultra_env_for(cell.preset).is_some();
            let expect = expect(cell, image_live);
            let assets = snapshot(app);
            let (mesh_entities, mesh_holders) = mesh_holders(app);
            let (standard_entities, standard_holders) = standard_holders(app);
            let (painterly_entities, painterly_holders) = painterly_holders(app);
            let (ramp_len, ramp) = ramp_ids(app);
            let lights = env_light_ids(app);
            let held = app.world().resource::<UltraEnvImage>().0.as_ref().map(|handle| handle.id());
            let fade_empty = app.world().resource::<TrunkFadeCache>().0.is_empty();

            assert_eq!(assets.mesh.len(), expect.meshes, "{label} meshes {cell:?}");
            assert_eq!(assets.standard.len(), expect.standard, "{label} standard materials {cell:?}");
            assert_eq!(assets.painterly.len(), expect.painterly, "{label} painterly materials {cell:?}");
            assert_eq!(assets.image.len(), expect.image, "{label} images {cell:?}");
            assert_eq!(mesh_entities, expect.mesh_entities, "{label} mesh entities {cell:?}");
            assert_eq!(standard_entities, expect.standard_holders, "{label} standard holders {cell:?}");
            assert_eq!(painterly_entities, expect.painterly_holders, "{label} painterly holders {cell:?}");
            assert_eq!(ramp_len, expect.ramp, "{label} ramp {cell:?}");
            assert_eq!(lights.len(), expect.env_lights, "{label} env lights {cell:?}");
            assert!(fade_empty, "{label} fade cache exclusion {cell:?}");

            assert_eq!(mesh_holders, assets.mesh, "{label} mesh ownership {cell:?}");
            assert_eq!(standard_holders, assets.standard, "{label} standard ownership {cell:?}");
            assert_eq!(painterly_holders, assets.painterly, "{label} painterly ownership {cell:?}");
            assert!(ramp.is_subset(&assets.mesh), "{label} ramp mesh {cell:?}");
            assert!(ramp.is_subset(&mesh_holders), "{label} ramp holder {cell:?}");
            assert_resolved(app, &assets.mesh, label);
            assert_resolved(app, &assets.standard, label);
            assert_resolved(app, &assets.painterly, label);
            assert_resolved(app, &ramp, label);

            if image_live {
                let id = assets.image.iter().copied().next().expect("one image");
                assert_eq!(held, Some(id), "{label} ultra image owner {cell:?}");
                assert!(app.world().resource::<Assets<Image>>().get(id).is_some(), "{label} image resolve");
                if let Some(previous) = *kept {
                    assert_eq!(id, previous, "{label} ultra image id changed {cell:?}");
                } else {
                    *kept = Some(id);
                }
                for light in lights {
                    assert_eq!(light, id, "{label} env light image {cell:?}");
                    assert!(app.world().resource::<Assets<Image>>().get(light).is_some());
                }
            } else {
                assert!(held.is_none(), "{label} image retained early {cell:?}");
                assert!(lights.is_empty(), "{label} env light before the image {cell:?}");
            }
        }

        fn visit(app: &mut App, cell: Cell, kept: &mut Option<AssetId<Image>>, label: &str) {
            write_cell(app, cell);
            app.update();
            app.update();
            let after_two = snapshot(app);
            app.update();
            let after_three = snapshot(app);
            assert_eq!(after_two, after_three, "{label} update 2 vs update 3 {cell:?}");
            check(app, cell, kept, &format!("{label} update 3"));
            app.update();
            let after_extra = snapshot(app);
            assert_eq!(after_extra, after_three, "{label} unchanged update vs update 3 {cell:?}");
            check(app, cell, kept, &format!("{label} unchanged"));
        }

        pin_oracle();

        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .init_asset::<Image>()
            .add_plugins(PainterlyPlugin)
            .add_plugins((
                UltraEnvPlugin,
                VegetationPlugin,
                WaterPlugin,
                HeartwoodLipPlugin,
                HeartwoodWardsPlugin,
            ))
            .insert_resource(PlayerInput {
                movement: Vec2::ZERO,
                ability_slot: None,
                interact: false,
                interact_held: false,
                jump: false,
                sprint: false,
                pause_toggle: false,
                sheet_q: false,
                sheet_l: false,
            })
            .insert_resource(SoftPresence {
                position: Vec3::new(0.0, 0.90, 0.0),
                velocity: Vec3::ZERO,
                grounded: true,
                prev_tick: None,
                curr_tick: None,
            });
        app.world_mut().spawn(Camera3d::default());

        let mut kept = None;
        for cell in matrix() {
            visit(&mut app, cell, &mut kept, "warm");
        }
        let image = kept.expect("warm did not retain the ultra image");
        for tour in 1..=20 {
            for cell in matrix() {
                visit(&mut app, cell, &mut kept, &format!("tour {tour}"));
                assert_eq!(kept, Some(image), "tour {tour} ultra image");
            }
        }
    }
}
