//! G0 light gen — hex-seed scatter + climate fog behind UI
//!
//! Env `POWRUSH_GEN=light` (default **off**). Tiny atlas trees/stones from
//! house⊕hex⊕climate_epoch seed. FogSettings on world Camera3d only — never
//! covers Title / pause / Settings / L / Q. No birds (lavapipe-safe). No new
//! Camera3d, no Avian/Rapier, no combat stats, no sockets. Contact: info@Rathor.ai

use bevy::pbr::{FogFalloff, FogSettings};
use bevy::prelude::*;

use shared::house_name::UNNAMED;
use shared::powrush_gen::{
    cull_gen_when_plate_open, fog_falloff_from_climate, grove_seed_from,
    parse_powrush_gen, scatter_from_seed, PowrushGen, ScatterKind,
};

use crate::ledger_bind::LedgerYard;
use crate::lived_hour_bind::LivedHourBind;
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
        Self {
            mode: parse_powrush_gen(),
        }
    }
}

impl LightGenDoor {
    pub fn is_light(self) -> bool {
        self.mode.is_light()
    }
}

#[derive(Resource, Debug, Default)]
struct LightGenSpawned {
    seed: Option<u64>,
}

#[derive(Component)]
struct LightGenProp;

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
                "G0 light gen ON — hex-seed scatter + climate fog (world cam)"
            );
        } else {
            info!(
                target: "powrush::gen",
                mode = door.mode.as_str(),
                "G0 light gen OFF (set POWRUSH_GEN=light to enable)"
            );
        }
        app.insert_resource(door)
            .init_resource::<LightGenSpawned>()
            .add_systems(Startup, prepare_atlas)
            .add_systems(
                Update,
                (
                    sync_scatter,
                    paint_world_fog_from_climate,
                    cull_scatter_when_plates_open,
                )
                    .chain(),
            );
    }
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

fn sync_scatter(
    mut commands: Commands,
    door: Res<LightGenDoor>,
    atlas: Option<Res<LightGenAtlas>>,
    bind: Res<LivedHourBind>,
    house: Res<HouseLabel>,
    mut spawned: ResMut<LightGenSpawned>,
    existing: Query<Entity, With<LightGenProp>>,
) {
    if !door.is_light() {
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
    let seed = grove_seed_from(
        house_id,
        hex_id,
        bind.climate.harmony,
        bind.climate.stress,
    );

    if spawned.seed == Some(seed) {
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
            Name::new("LightGenProp"),
        ));
    }

    spawned.seed = Some(seed);
    info!(
        target: "powrush::gen",
        seed,
        house = house_id,
        hex = hex_id,
        "G0 grove scattered (no physics, no birds)"
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
    mut props: Query<&mut Visibility, With<LightGenProp>>,
) {
    if !door.is_light() {
        return;
    }
    let title_open = matches!(
        *launch,
        LaunchDoor::Title | LaunchDoor::NameHouse | LaunchDoor::HouseDress
    );
    let pause_or_settings = house.settings_open;
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
    use shared::powrush_gen::{cull_gen_when_plate_open, light_gen_enabled_from, grove_seed};

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
}
