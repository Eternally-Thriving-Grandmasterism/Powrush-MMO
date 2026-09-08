//! G0 / G0.5 light gen — hex-seed scatter + climate fog behind UI
//!
//! Enabled when Settings Grove is **light** OR env `POWRUSH_GEN=light` (OR).
//! Default **off**. Same gen path — not a second system. Tiny atlas trees/stones
//! from house⊕hex⊕climate_epoch seed. FogSettings on world Camera3d only — never
//! covers Title / pause / Settings / L / Q. No birds (lavapipe-safe). No new
//! Camera3d, no Avian/Rapier, no combat stats, no sockets.
//! GenShare Method A: offline `data/powrush_genshare.jsonl` seed persist (no port).
//! Contact: info@Rathor.ai

use bevy::pbr::{FogFalloff, FogSettings};
use bevy::prelude::*;

use shared::genshare::{append_genshare, load_genshare, GenShare, GENSHARE_PATH};
use shared::house_name::UNNAMED;
use shared::powrush_gen::{
    climate_epoch, cull_gen_when_plate_open, fog_falloff_from_climate, grove_seed_from,
    light_gen_enabled_with, scatter_from_seed, PowrushGen, ScatterKind,
};

use crate::ledger_bind::LedgerYard;
use crate::lived_hour_bind::LivedHourBind;
use crate::local_settings::LocalSettingsState;
use crate::title_screen::{HouseLabel, LaunchDoor};
use crate::ui_above_world::LivedUiCamera;
use crate::vertical_factory::FactoryYard;

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
    leaf: Handle<StandardMaterial>,
    stone: Handle<StandardMaterial>,
    bush_mat: Handle<StandardMaterial>,
}

pub struct LightGenPlugin;

impl Plugin for LightGenPlugin {
    fn build(&self, app: &mut App) {
        let door = LightGenDoor::default();
        if door.is_light() {
            info!(
                target: "powrush::gen",
                mode = door.mode.as_str(),
                "G0 light gen ON — Settings Grove light and/or POWRUSH_GEN=light"
            );
        } else {
            info!(
                target: "powrush::gen",
                mode = door.mode.as_str(),
                "G0 light gen OFF (Settings Grove · light or POWRUSH_GEN=light)"
            );
        }
        app.insert_resource(door)
            .init_resource::<LightGenSpawned>()
            .add_systems(Startup, prepare_atlas)
            .add_systems(
                Update,
                (
                    sync_light_gen_door,
                    ensure_atlas_when_light,
                    sync_scatter,
                    paint_world_fog_from_climate,
                    cull_scatter_when_plates_open,
                )
                    .chain(),
            );
    }
}

/// Keep door = settings Grove light OR env POWRUSH_GEN=light.
fn sync_light_gen_door(
    settings: Res<LocalSettingsState>,
    mut door: ResMut<LightGenDoor>,
) {
    let want = LightGenDoor::from_settings_grove(Some(settings.inner.grove.as_str())).mode;
    if door.mode != want {
        door.mode = want;
        info!(
            target: "powrush::gen",
            mode = door.mode.as_str(),
            grove = %settings.inner.grove,
            "G0.5 light gen door synced (settings OR env)"
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
) {
    if !door.is_light() || atlas.is_some() {
        return;
    }
    insert_atlas(&mut commands, &mut meshes, &mut materials);
}

fn insert_atlas(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    commands.insert_resource(LightGenAtlas {
        trunk: meshes.add(Cylinder::new(0.18, 2.0)),
        canopy: meshes.add(Sphere::new(0.72)),
        rock: meshes.add(Sphere::new(0.38)),
        bush: meshes.add(Sphere::new(0.32)),
        wood: materials.add(StandardMaterial {
            base_color: Color::srgb(0.26, 0.16, 0.09),
            perceptual_roughness: 0.92,
            ..default()
        }),
        leaf: materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.40, 0.18),
            perceptual_roughness: 0.72,
            ..default()
        }),
        stone: materials.add(StandardMaterial {
            base_color: Color::srgb(0.30, 0.28, 0.26),
            perceptual_roughness: 0.95,
            ..default()
        }),
        bush_mat: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.36, 0.16),
            perceptual_roughness: 0.85,
            ..default()
        }),
    });
}

fn prepare_atlas(
    mut commands: Commands,
    door: Res<LightGenDoor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !door.is_light() {
        return;
    }
    insert_atlas(&mut commands, &mut meshes, &mut materials);
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
    mut spawned: ResMut<LightGenSpawned>,
    existing: Query<Entity, With<LightGenProp>>,
) {
    if !door.is_light() {
        if spawned.seed.is_some() || spawned.pending_seed.is_some() || !existing.is_empty() {
            for e in &existing {
                commands.entity(e).despawn_recursive();
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
        commands.entity(e).despawn_recursive();
    }

    let spots = scatter_from_seed(seed);
    for spot in spots {
        let (mesh, material, scale) = match spot.kind {
            ScatterKind::Trunk => (
                atlas.trunk.clone(),
                atlas.wood.clone(),
                Vec3::new(spot.scale, spot.scale, spot.scale),
            ),
            ScatterKind::Canopy => (
                atlas.canopy.clone(),
                atlas.leaf.clone(),
                Vec3::splat(spot.scale),
            ),
            ScatterKind::Rock => (
                atlas.rock.clone(),
                atlas.stone.clone(),
                Vec3::new(spot.scale * 1.3, spot.scale * 0.55, spot.scale * 1.1),
            ),
            ScatterKind::Bush => (
                atlas.bush.clone(),
                atlas.bush_mat.clone(),
                Vec3::splat(spot.scale),
            ),
        };
        commands.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(spot.x, spot.y, spot.z).with_scale(scale),
                ..default()
            },
            LightGenProp,
            HexScatter,
            Name::new("LightGenProp"),
        ));
    }

    spawned.seed = Some(seed);
    spawned.pending_seed = None;
    info!(
        target: "powrush::gen",
        seed,
        house = house_id,
        hex = hex_id,
        from_disk,
        path = %shared::user_persist::persist_path(GENSHARE_PATH).display(),
        "G0 grove scattered (GenShare L0; no physics, no birds, no socket)"
    );
}

/// Fog on world Camera3d only — never LivedUiCamera / Camera2d.
fn paint_world_fog_from_climate(
    door: Res<LightGenDoor>,
    bind: Res<LivedHourBind>,
    mut fogs: Query<&mut FogSettings, (With<Camera3d>, Without<LivedUiCamera>)>,
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
}
