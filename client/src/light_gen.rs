//! G0 / G0.5 light gen — hex-seed scatter + climate fog behind UI
//!
//! Enabled when Settings Grove is **light** OR env `POWRUSH_GEN=light` (OR).
//! Default **off**. Same gen path — not a second system. Tiny atlas trees/stones
//! from house⊕hex⊕climate_epoch seed. FogSettings on world Camera3d only — never
//! covers Title / pause / Settings / L / Q. No birds (lavapipe-safe). No new
//! Camera3d, no Avian/Rapier, no combat stats, no sockets.
//! GenShare Method A: offline `data/powrush_genshare.jsonl` seed persist (no port).
//!
//! CARD FLESH-LIGHT-GEN — the existing gen logs may name the Place
//! (`HexTravelState::chip_name`). Absent travel keeps each bare line.
//! No light HUD. No Ultra. reduced_motion / Low stay outside this line.
//! Peak memory, cited: walked · tended · week was the bill · yard remembered.
//! Contact: info@Rathor.ai

use bevy::pbr::{FogFalloff, DistanceFog};
use bevy::prelude::*;

use shared::genshare::{append_genshare, load_genshare, GenShare, GENSHARE_PATH};
use shared::house_name::UNNAMED;
use shared::powrush_gen::{
    climate_epoch, cull_gen_when_plate_open, fog_falloff_from_climate, grove_seed_from,
    light_gen_enabled_with, scatter_from_seed, PowrushGen, ScatterKind,
};

use crate::climate_plane::{configure_fog_write_sets, FogWriteSet};
use crate::ledger_bind::LedgerYard;
use crate::lived_hour_bind::LivedHourBind;
use crate::local_settings::LocalSettingsState;
use crate::title_screen::{HouseLabel, LaunchDoor};
use crate::ui_above_world::LivedUiCamera;
use crate::vertical_factory::FactoryYard;

/// Existing startup line when gen is on. Absent travel keeps this byte for byte.
const GEN_ON_LINE: &str = "G0 light gen ON — Settings Grove light and/or POWRUSH_GEN=light";
/// Existing startup line when gen is off. Absent travel keeps this byte for byte.
const GEN_OFF_LINE: &str = "G0 light gen OFF (Settings Grove · light or POWRUSH_GEN=light)";
/// Existing door-sync line. Absent travel keeps this byte for byte.
const GEN_SYNC_LINE: &str = "G0.5 light gen door synced (settings OR env)";
/// Existing scatter line. Absent travel keeps this byte for byte.
const GEN_SCATTER_LINE: &str = "G0 grove scattered (GenShare L0; no physics, no birds, no socket)";

/// CARD FLESH-LIGHT-GEN — `{place} · {bare}` when a chip is present.
/// `None` returns `bare`. One string on the existing log. No second HUD.
fn light_gen_line(bare: &str, place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {bare}"),
        None => bare.to_string(),
    }
}

fn stood_chip(travel: Option<&crate::hex_travel::HexTravelState>) -> Option<&'static str> {
    travel.map(|state| state.chip_name())
}

/// Runtime gen door — parsed once at plugin build / resource init.
#[derive(Resource, Debug, Clone, Copy)]
pub struct LightGenDoor {
    pub mode: PowrushGen,
}

impl Default for LightGenDoor {
    fn default() -> Self {
        // Startup: OR settings grove with env (settings file may already say light).
        let grove = shared::local_settings::LocalSettings::load_or_default().grove;
        Self::from_settings_grove(Some(grove.as_str()))
    }
}

impl LightGenDoor {
    pub fn is_light(self) -> bool {
        self.mode.is_light()
    }

    pub fn from_settings_grove(settings_grove: Option<&str>) -> Self {
        let mode = if light_gen_enabled_with(settings_grove) {
            PowrushGen::Light
        } else {
            PowrushGen::Off
        };
        Self { mode }
    }
}

#[derive(Resource, Debug, Default)]
struct LightGenSpawned {
    seed: Option<u64>,
    /// Seed written to JSONL but mesh rebuild deferred while plates open.
    pending_seed: Option<u64>,
}

/// G0 light-gen prop marker (atlas trees/stones).
#[derive(Component)]
struct LightGenProp;

/// Alias marker for GenShare rebuild docs (`HexScatter` = same props as LightGenProp).
#[derive(Component)]
struct HexScatter;

#[derive(Resource)]
struct LightGenAtlas {
    trunk: Handle<Mesh>,
    canopy: Handle<Mesh>,
    rock: Handle<Mesh>,
    bush: Handle<Mesh>,
    wood: Handle<StandardMaterial>,
    leaf: Handle<crate::painterly_shader::PainterlyMaterial>,
    stone: Handle<StandardMaterial>,
    bush_mat: Handle<crate::painterly_shader::PainterlyMaterial>,
}

pub struct LightGenPlugin;

impl Plugin for LightGenPlugin {
    fn build(&self, app: &mut App) {
        let door = LightGenDoor::default();
        let place = stood_chip(
            app.world()
                .get_resource::<crate::hex_travel::HexTravelState>(),
        );
        let bare = if door.is_light() {
            GEN_ON_LINE
        } else {
            GEN_OFF_LINE
        };
        let line = light_gen_line(bare, place);
        info!(
            target: "powrush::gen",
            mode = door.mode.as_str(),
            "{line}"
        );
        configure_fog_write_sets(app);
        app.insert_resource(door)
            .init_resource::<LightGenSpawned>()
            .add_systems(Startup, prepare_atlas)
            .add_systems(
                Update,
                (
                    sync_light_gen_door,
                    ensure_atlas_when_light,
                    sync_scatter,
                    paint_world_fog_from_climate.in_set(FogWriteSet::LightDoor),
                    cull_scatter_when_plates_open,
                )
                    .chain(),
            );
    }
}

/// Keep door = settings Grove light OR env POWRUSH_GEN=light.
fn sync_light_gen_door(
    settings: Res<LocalSettingsState>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut door: ResMut<LightGenDoor>,
) {
    let want = LightGenDoor::from_settings_grove(Some(settings.inner.grove.as_str())).mode;
    if door.mode != want {
        door.mode = want;
        let line = light_gen_line(GEN_SYNC_LINE, stood_chip(travel.as_deref()));
        info!(
            target: "powrush::gen",
            mode = door.mode.as_str(),
            grove = %settings.inner.grove,
            "{line}"
        );
    }
}

/// Lazy atlas when Grove turns light after startup (Settings toggle).
fn ensure_atlas_when_light(
    mut commands: Commands,
    door: Res<LightGenDoor>,
    atlas: Option<Res<LightGenAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut painted: ResMut<Assets<crate::painterly_shader::PainterlyMaterial>>,
    settings: Option<Res<crate::local_settings::LocalSettingsState>>,
    mut ramps: Option<ResMut<crate::painterly_mat::PainterlyRampMeshes>>,
) {
    if !door.is_light() || atlas.is_some() {
        return;
    }
    insert_atlas(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut painted,
        settings.as_deref(),
        &mut ramps,
    );
}

fn insert_atlas(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    painted: &mut Assets<crate::painterly_shader::PainterlyMaterial>,
    settings: Option<&crate::local_settings::LocalSettingsState>,
    ramps: &mut Option<ResMut<crate::painterly_mat::PainterlyRampMeshes>>,
) {
    let preset = settings
        .map(|s| s.inner.graphics_preset)
        .unwrap_or(shared::local_settings::GraphicsPreset::Medium);
    // CARD VP-MAT-1 — wood and stone hues stay. Leaf and bush take the
    // valley green and the cheap height ramp.
    const GEN_LEAF: [f32; 3] = [0.14, 0.40, 0.18];
    const GEN_BUSH: [f32; 3] = [0.18, 0.36, 0.16];
    // CARD PROC-KIT-2 — in-place organic jitter on a shared mesh. Fixed-seeded
    // per kind (`tag`) and keyed on each vertex's position rounded to 1e-4, so
    // seam and pole twins move together and the mesh stays closed. Only
    // positions scale, by 1 + a·h with h in [−1, 1]: foliage a = 0.04, rocks
    // a = 0.06. Same vertices, same indices. Normals are rebuilt after.
    let jitter = |mesh: &mut Mesh, tag: u32, a: f32| {
        if let Some(bevy::mesh::VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            for p in positions.iter_mut() {
                let mut h = tag.wrapping_mul(0x9E37_79B9);
                for c in *p {
                    let key = (c * 10_000.0).round() as i32 as u32;
                    h = (h ^ key).wrapping_mul(0x85EB_CA6B);
                    h ^= h >> 13;
                }
                h = h.wrapping_mul(0x2C1B_3C6D);
                h ^= h >> 12;
                let unit = (h & 0xFFFF) as f32 / 65535.0 * 2.0 - 1.0;
                let s = 1.0 + a * unit;
                p[0] *= s;
                p[1] *= s;
                p[2] *= s;
            }
        }
        mesh.compute_smooth_normals();
    };
    let mut canopy = Mesh::from(Sphere::new(0.72));
    let mut bush = Mesh::from(Sphere::new(0.32));
    let mut rock = Mesh::from(Sphere::new(0.38));
    jitter(&mut canopy, 3, 0.04);
    jitter(&mut bush, 4, 0.04);
    jitter(&mut rock, 5, 0.06);
    let foliage = crate::painterly_mat::painterly_shade(
        crate::painterly_mat::PainterlyRole::Foliage,
        preset,
    );
    crate::painterly_mat::paint_height_ramp(&mut canopy, &foliage);
    crate::painterly_mat::paint_height_ramp(&mut bush, &foliage);
    let canopy = meshes.add(canopy);
    let bush = meshes.add(bush);
    crate::painterly_mat::remember_ramp(
        ramps,
        &canopy,
        crate::painterly_mat::PainterlyRole::Foliage,
    );
    crate::painterly_mat::remember_ramp(
        ramps,
        &bush,
        crate::painterly_mat::PainterlyRole::Foliage,
    );
    commands.insert_resource(LightGenAtlas {
        trunk: meshes.add(Cylinder::new(0.18, 2.0)),
        canopy,
        rock: meshes.add(rock),
        bush,
        wood: materials.add(StandardMaterial {
            base_color: Color::srgb(0.26, 0.16, 0.09),
            perceptual_roughness: 0.92,
            ..default()
        }),
        leaf: painted.add(crate::painterly_mat::painterly_material(
            crate::painterly_mat::PainterlyRole::Foliage,
            GEN_LEAF,
            preset,
        )),
        stone: materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.28, 0.26),
            perceptual_roughness: 0.95,
            ..default()
        }),
        bush_mat: painted.add(crate::painterly_mat::painterly_material(
            crate::painterly_mat::PainterlyRole::Foliage,
            GEN_BUSH,
            preset,
        )),
    });
}

fn prepare_atlas(
    mut commands: Commands,
    door: Res<LightGenDoor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut painted: ResMut<Assets<crate::painterly_shader::PainterlyMaterial>>,
    settings: Option<Res<crate::local_settings::LocalSettingsState>>,
    mut ramps: Option<ResMut<crate::painterly_mat::PainterlyRampMeshes>>,
) {
    if !door.is_light() {
        return;
    }
    insert_atlas(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut painted,
        settings.as_deref(),
        &mut ramps,
    );
}

fn sync_scatter(
    mut commands: Commands,
    door: Res<LightGenDoor>,
    atlas: Option<Res<LightGenAtlas>>,
    bind: Res<LivedHourBind>,
    house: Res<HouseLabel>,
    launch: Res<LaunchDoor>,
    ledger: Res<LedgerYard>,
    factory: Res<FactoryYard>,
    places: Option<Res<crate::hex_travel::PlacesPlate>>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut spawned: ResMut<LightGenSpawned>,
    existing: Query<Entity, With<LightGenProp>>,
) {
    if !door.is_light() {
        if spawned.seed.is_some() || spawned.pending_seed.is_some() || !existing.is_empty() {
            for e in &existing {
                commands.entity(e).despawn();
            }
            spawned.seed = None;
            spawned.pending_seed = None;
        }
        return;
    }
    let Some(atlas) = atlas else {
        return;
    };

    let house_id = {
        let n = house.house.display_name();
        if n.trim().is_empty() {
            UNNAMED
        } else {
            n
        }
    };
    let hex_id = if bind.climate.hex_id.trim().is_empty() {
        "local-hex"
    } else {
        bind.climate.hex_id.as_str()
    };
    let dress = house
        .house
        .dress_line_for_plate()
        .unwrap_or_default();

    // GenShare A: prefer persisted seed for house⊕hex; else compute + append JSONL.
    let (seed, from_disk) = if let Some(row) = load_genshare(hex_id, Some(house_id)) {
        (row.seed_u64, true)
    } else if let Some(row) = load_genshare(hex_id, None) {
        (row.seed_u64, true)
    } else {
        let local = grove_seed_from(
            house_id,
            hex_id,
            bind.climate.harmony,
            bind.climate.stress,
        );
        let envelope = GenShare::build(
            house_id,
            hex_id,
            climate_epoch(bind.climate.harmony, bind.climate.stress),
            local,
            bind.climate.harmony,
            bind.climate.stress,
            &dress,
        );
        // Write immediately — never open a plate-bury path for file I/O.
        let _ = append_genshare(&envelope);
        (local, false)
    };
    if spawned.seed == Some(seed) {
        spawned.pending_seed = None;
        return;
    }

    // Defer mesh hitch while plates are open (Title / pause / Settings / L / Q).
    // File already written above when missing; rebuild waits for a safe frame.
    let title_open = matches!(
        *launch,
        LaunchDoor::Title | LaunchDoor::NameHouse | LaunchDoor::HouseDress
    );
    let pause_or_settings = house.settings_open
        || places
            .as_ref()
            .map(|p| crate::hex_travel::places_culls_sticks(p))
            .unwrap_or(false);
    let ledger_open = ledger.sash_open;
    let q_open = factory.factory.founded && !factory.factory.tutorial_complete();
    let plates_open = cull_gen_when_plate_open(title_open, pause_or_settings, ledger_open, q_open);
    if plates_open && spawned.seed.is_some() {
        spawned.pending_seed = Some(seed);
        return;
    }

    for e in &existing {
        commands.entity(e).despawn();
    }

    // CARD PROC-KIT-2 — yaw-only variety, hashed from the spawn's own x/z.
    let yaw_at = |x: f32, z: f32| -> f32 {
        let mut h = ((x * 10_000.0).round() as i32 as u32).wrapping_mul(0x27D4_EB2F)
            ^ ((z * 10_000.0).round() as i32 as u32).wrapping_mul(0x1656_67B1);
        h ^= h >> 15;
        h = h.wrapping_mul(0x2C1B_3C6D);
        h ^= h >> 12;
        (h & 0xFFFF) as f32 / 65535.0 * std::f32::consts::TAU
    };
    let spots = scatter_from_seed(seed);
    for spot in spots {
        let scale = match spot.kind {
            ScatterKind::Trunk => Vec3::new(spot.scale, spot.scale, spot.scale),
            ScatterKind::Rock => Vec3::new(spot.scale * 1.3, spot.scale * 0.55, spot.scale * 1.1),
            ScatterKind::Canopy | ScatterKind::Bush => Vec3::splat(spot.scale),
        };
        let transform = Transform::from_xyz(spot.x, spot.y, spot.z)
            .with_scale(scale)
            .with_rotation(Quat::from_rotation_y(yaw_at(spot.x, spot.z)));
        let mut entity = match spot.kind {
            ScatterKind::Trunk => commands.spawn((
                (
                    Mesh3d(atlas.trunk.clone()),
                    MeshMaterial3d(atlas.wood.clone()),
                    transform,
                ),
                LightGenProp,
                HexScatter,
                Name::new("LightGenProp"),
            )),
            ScatterKind::Rock => commands.spawn((
                (
                    Mesh3d(atlas.rock.clone()),
                    MeshMaterial3d(atlas.stone.clone()),
                    transform,
                ),
                LightGenProp,
                HexScatter,
                Name::new("LightGenProp"),
            )),
            ScatterKind::Canopy => commands.spawn((
                (
                    Mesh3d(atlas.canopy.clone()),
                    MeshMaterial3d(atlas.leaf.clone()),
                    transform,
                ),
                LightGenProp,
                HexScatter,
                Name::new("LightGenProp"),
            )),
            ScatterKind::Bush => commands.spawn((
                (
                    Mesh3d(atlas.bush.clone()),
                    MeshMaterial3d(atlas.bush_mat.clone()),
                    transform,
                ),
                LightGenProp,
                HexScatter,
                Name::new("LightGenProp"),
            )),
        };
        // CARD VP-MAT-1 — canopy and bush only. Wood and stone stay earth.
        let authored = match spot.kind {
            ScatterKind::Canopy => Some([0.14, 0.40, 0.18]),
            ScatterKind::Bush => Some([0.18, 0.36, 0.16]),
            ScatterKind::Trunk | ScatterKind::Rock => None,
        };
        if let Some(authored) = authored {
            entity.insert(crate::painterly_mat::PainterlyMat {
                role: crate::painterly_mat::PainterlyRole::Foliage,
                authored,
            });
        }
    }

    spawned.seed = Some(seed);
    spawned.pending_seed = None;
    let line = light_gen_line(GEN_SCATTER_LINE, stood_chip(travel.as_deref()));
    info!(
        target: "powrush::gen",
        seed,
        house = house_id,
        hex = hex_id,
        from_disk,
        path = %shared::user_persist::persist_path(GENSHARE_PATH).display(),
        "{line}"
    );
}

/// Fog on world Camera3d only — never LivedUiCamera / Camera2d.
fn paint_world_fog_from_climate(
    door: Res<LightGenDoor>,
    bind: Res<LivedHourBind>,
    mut fogs: Query<&mut DistanceFog, (With<Camera3d>, Without<LivedUiCamera>)>,
) {
    if !door.is_light() {
        return;
    }
    let (start, end) = fog_falloff_from_climate(
        bind.climate.stress,
        bind.climate.harmony,
        bind.climate.regen,
    );
    let t = bind.climate.stress.clamp(0.0, 1.0);
    let color = Color::srgba(
        0.26 + (1.0 - t) * 0.14,
        0.46 - t * 0.16,
        0.40 - t * 0.10,
        1.0,
    );
    for mut fog in &mut fogs {
        fog.color = color;
        fog.falloff = FogFalloff::Linear { start, end };
    }
}

fn cull_scatter_when_plates_open(
    door: Res<LightGenDoor>,
    launch: Res<LaunchDoor>,
    house: Res<HouseLabel>,
    ledger: Res<LedgerYard>,
    factory: Res<FactoryYard>,
    places: Option<Res<crate::hex_travel::PlacesPlate>>,
    mut props: Query<&mut Visibility, With<LightGenProp>>,
) {
    if !door.is_light() {
        return;
    }
    let title_open = matches!(
        *launch,
        LaunchDoor::Title | LaunchDoor::NameHouse | LaunchDoor::HouseDress
    );
    let pause_or_settings = house.settings_open
        || places
            .as_ref()
            .map(|p| crate::hex_travel::places_culls_sticks(p))
            .unwrap_or(false);
    let ledger_open = ledger.sash_open;
    // Q plate: charter / fabricator tutorial active (Frontier slab content).
    let q_open = factory.factory.founded && !factory.factory.tutorial_complete();
    let cull = cull_gen_when_plate_open(title_open, pause_or_settings, ledger_open, q_open);
    let vis = if cull {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
    for mut v in &mut props {
        if *v != vis {
            *v = vis;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mesh_positions(app: &App, handle: &Handle<Mesh>) -> Vec<[f32; 3]> {
        let mesh = app.world().resource::<Assets<Mesh>>().get(handle).expect("mesh");
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|v| v.as_float3())
            .expect("positions")
            .to_vec()
    }

    fn primitive_positions(mesh: Mesh) -> Vec<[f32; 3]> {
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|v| v.as_float3())
            .expect("positions")
            .to_vec()
    }

    /// CARD PROC-KIT-2 — jitter is in place: same count, within 1 ± a of the
    /// primitive, twins stay twins, and something actually moved.
    fn assert_jittered(fresh: &[[f32; 3]], got: &[[f32; 3]], a: f32) {
        assert_eq!(fresh.len(), got.len());
        let len = |p: &[f32; 3]| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        let mut moved = false;
        for (i, (f, g)) in fresh.iter().zip(got).enumerate() {
            if len(f) > 1e-6 {
                let r = len(g) / len(f);
                assert!(r >= 1.0 - a - 1e-5 && r <= 1.0 + a + 1e-5, "vertex {i} ratio {r}");
            }
            moved |= f != g;
            for (j, f2) in fresh.iter().enumerate().skip(i + 1) {
                if f == f2 {
                    let g2 = got[j];
                    let d = (g[0] - g2[0]).abs() + (g[1] - g2[1]).abs() + (g[2] - g2[2]).abs();
                    assert!(d < 1e-6, "twins {i}/{j} split by {d}");
                }
            }
        }
        assert!(moved, "jitter left the primitive unchanged");
    }

    fn build_atlas_for_test(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut painted: ResMut<Assets<crate::painterly_shader::PainterlyMaterial>>,
        mut ramps: Option<ResMut<crate::painterly_mat::PainterlyRampMeshes>>,
    ) {
        insert_atlas(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut painted,
            None,
            &mut ramps,
        );
    }

    fn atlas_build() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .init_asset::<crate::painterly_shader::PainterlyMaterial>()
            .add_systems(Startup, build_atlas_for_test);
        app.finish();
        app.cleanup();
        app.update();
        app
    }

    /// CARD PROC-KIT-2 — two atlas builds give the same jittered canopy, bush
    /// and rock; the trunk stays the plain cylinder.
    #[test]
    fn proc_kit_2_atlas_jitter_is_deterministic_and_trunk_untouched() {
        let first = atlas_build();
        let second = atlas_build();
        let a1 = first.world().resource::<LightGenAtlas>();
        let a2 = second.world().resource::<LightGenAtlas>();
        for (h1, h2, fresh, a) in [
            (&a1.canopy, &a2.canopy, Mesh::from(Sphere::new(0.72)), 0.04),
            (&a1.bush, &a2.bush, Mesh::from(Sphere::new(0.32)), 0.04),
            (&a1.rock, &a2.rock, Mesh::from(Sphere::new(0.38)), 0.06),
        ] {
            let got = mesh_positions(&first, h1);
            assert_eq!(got, mesh_positions(&second, h2));
            assert_jittered(&primitive_positions(fresh), &got, a);
        }
        assert_eq!(
            mesh_positions(&first, &a1.trunk),
            primitive_positions(Mesh::from(Cylinder::new(0.18, 2.0)))
        );
    }
    use shared::powrush_gen::{
        cull_gen_when_plate_open, grove_seed, light_gen_enabled_from, light_gen_enabled_or,
    };

    #[test]
    fn door_defaults_off_without_env_light() {
        // Resource default reads process env; unit path must not require light.
        assert!(!light_gen_enabled_from(None));
        assert!(!light_gen_enabled_from(Some("off")));
        let off = LightGenDoor {
            mode: PowrushGen::Off,
        };
        assert!(!off.is_light());
        let on = LightGenDoor {
            mode: PowrushGen::Light,
        };
        assert!(on.is_light());
    }

    #[test]
    fn seed_stability_matches_shared() {
        assert_eq!(grove_seed(10, 20, 1), grove_seed(10, 20, 1));
        assert_ne!(grove_seed(10, 20, 1), grove_seed(10, 20, 2));
    }

    #[test]
    fn cull_helper_title_pause_l_q() {
        assert!(cull_gen_when_plate_open(true, false, false, false));
        assert!(cull_gen_when_plate_open(false, true, false, false));
        assert!(cull_gen_when_plate_open(false, false, true, false));
        assert!(cull_gen_when_plate_open(false, false, false, true));
        assert!(!cull_gen_when_plate_open(false, false, false, false));
    }

    #[test]
    fn light_gen_enabled_matches_env_parser() {
        assert!(light_gen_enabled_from(Some("light")));
        assert!(!light_gen_enabled_from(None));
        assert!(!light_gen_enabled_from(Some("off")));
    }

    #[test]
    fn door_from_settings_grove_ors_with_env_half() {
        // Pure OR (no process env): settings light alone enables.
        assert!(light_gen_enabled_or(Some("light"), Some("off")));
        assert!(!light_gen_enabled_or(Some("off"), Some("off")));
        let on = LightGenDoor {
            mode: PowrushGen::Light,
        };
        assert!(on.is_light());
        let off = LightGenDoor::from_settings_grove(Some("off"));
        // May still be light if process env is set — only assert Off when env off.
        if !light_gen_enabled_from(std::env::var("POWRUSH_GEN").ok().as_deref()) {
            assert!(!off.is_light());
        }
    }

    /// CARD FLESH-LIGHT-GEN — Place prefixes the gen logs; no travel keeps them exact.
    #[test]
    fn flesh_light_gen_line_names_chip_or_keeps_bare_line() {
        use shared::hex_travel::PlaceId;

        let bares = [GEN_ON_LINE, GEN_OFF_LINE, GEN_SYNC_LINE, GEN_SCATTER_LINE];
        for bare in bares {
            assert_eq!(light_gen_line(bare, None), bare);
        }
        assert_eq!(
            light_gen_line(GEN_ON_LINE, None),
            "G0 light gen ON — Settings Grove light and/or POWRUSH_GEN=light"
        );
        assert_eq!(
            light_gen_line(GEN_OFF_LINE, None),
            "G0 light gen OFF (Settings Grove · light or POWRUSH_GEN=light)"
        );
        assert_eq!(
            light_gen_line(GEN_SYNC_LINE, None),
            "G0.5 light gen door synced (settings OR env)"
        );
        assert_eq!(
            light_gen_line(GEN_SCATTER_LINE, None),
            "G0 grove scattered (GenShare L0; no physics, no birds, no socket)"
        );

        let cases = [
            (PlaceId::Sanctuary, "Sanctuary Prime"),
            (PlaceId::Heartwood, "Heartwood"),
            (PlaceId::Depths, "Depths"),
        ];
        for (id, name) in cases {
            assert_eq!(id.chip_name(), name);
            let travel = crate::hex_travel::HexTravelState { current: id };
            assert_eq!(stood_chip(Some(&travel)), Some(name));
            for bare in bares {
                assert_eq!(
                    light_gen_line(bare, stood_chip(Some(&travel))),
                    format!("{name} · {bare}")
                );
            }
        }

        for sample in [
            light_gen_line(GEN_SCATTER_LINE, Some(PlaceId::Heartwood.chip_name())),
            light_gen_line(GEN_ON_LINE, None),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("ultra"), "{sample}");
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("xp"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }
}
