#![cfg(test)]
//! ENTITY-CONTRACT-1a. C1 names any Mesh3d with no policy. C2: walk-throughs are only `KNOWN_GAPS` rows, so dropping a row fails C1. C3: spawn (0, 0.90, 0) misses shown solids. C6: tells are NoCollide.

use bevy::prelude::*;
use shared::hex_travel::PlaceId;
use crate::{
    climate_plane::{ClimatePlanePlugin, ClimateStone},
    combat_core::{ClearedTell, CombatCorePlugin, LungeTell, Predator, RiseTell},
    depths_landing::DepthsLandingPlugin,
    droid::{Droid, DroidPlugin, DroidTell},
    grazer::{Grazer, GrazerPlugin},
    heartwood_lip::HeartwoodLipPlugin,
    heartwood_wards::HeartwoodWardsPlugin,
    hex_travel::HexTravelState,
    human_presence::{HumanPresence, HumanPresencePlugin, SoftPresence},
    living_ecology::{EcologyProp, LivingEcologyPlugin, PropKind, YARD_ROCK_R, YARD_TRUNK_R},
    living_practice_loop::SoftPlayerRealm,
    local_human_sim::{LocalHumanSim, LocalHumanSimPlugin},
    mercy_harvest_nodes::{MercyHarvestNode, MercyHarvestNodesPlugin},
    sky_backdrop::{SkyBackdropPart, SkyBackdropPlugin},
    stone_ground::StoneGroundTile,
    vegetation::{CamTrunk, VegetationPlugin, TRUNK_RADIUS},
    water::{BathMark, WaterPlugin},
};

/// Lived trunk radius. Push-out adds it to each solid; `human_presence` keeps that const private.
const BODY_R: f32 = 0.30;
const MESH3D: &str = concat!("Mesh3d", "(");
/// Owning card per walk-through. The row is that mesh's only declaration.
const KNOWN_GAPS: &[(&str, &str)] = &[
    ("deer", "CREATURE-BODY-1"), ("crystal", "PROP-COLLIDE-1"), ("traveler", "CREATURE-BODY-1"),
    ("predator", "CREATURE-BODY-1"), ("droid", "CREATURE-BODY-1"), ("grazer", "CREATURE-BODY-1"),
    ("ward", "PROP-COLLIDE-1"), ("walkway", "PROP-COLLIDE-1"), ("rib", "PROP-COLLIDE-1"), ("shelf", "PROP-COLLIDE-1"),
];

#[derive(Clone, Copy, PartialEq, Debug)]
enum Policy { Solid(f32), NoCollide, Walkable, Trigger, Gap }

fn gap(key: &str) -> Result<Policy, String> {
    if KNOWN_GAPS.iter().any(|(k, card)| *k == key && !card.is_empty()) { Ok(Policy::Gap) }
    else { Err(key.to_string()) }
}
fn ancestor_name(world: &World, mut id: Entity) -> Option<String> {
    for _ in 0..8 {
        if let Some(name) = world.get::<Name>(id) { return Some(name.as_str().to_string()); }
        id = world.get::<ChildOf>(id)?.parent();
    }
    None
}
fn shown(world: &World, mut id: Entity) -> bool {
    for _ in 0..8 {
        if world.get::<Visibility>(id).copied() == Some(Visibility::Hidden) { return false; }
        if world.get::<Visibility>(id).copied() == Some(Visibility::Visible) { return true; }
        let Some(child) = world.get::<ChildOf>(id) else { return true };
        id = child.parent();
    }
    true
}
fn by_name(name: &str) -> Option<Result<Policy, String>> {
    const PREFIX: &[(&str, &str)] = &[
        ("WardPost", "ward"), ("HeartwoodWalkway", "walkway"),
        ("HeartwoodHangingRib", "rib"), ("ThresholdShelf", "shelf"),
    ];
    if let Some((_, key)) = PREFIX.iter().find(|(p, _)| name.starts_with(p)) { return Some(gap(key)); }
    if matches!(name, "Mira" | "Ko" | "Ren") { return Some(gap("traveler")); }
    let nocollide = name.starts_with("VP-") || name.starts_with("Person")
        || matches!(name, "HumanPresence" | "HeartwoodBath" | "LightGenProp");
    if nocollide { return Some(Ok(Policy::NoCollide)); }
    if matches!(name, "ClimateGround" | "DepthsLanding") { return Some(Ok(Policy::Walkable)); }
    if matches!(name, "ThresholdPeaceNode" | "DepthsPeaceNode") { return Some(Ok(Policy::Trigger)); }
    None
}
fn policy(world: &World, id: Entity) -> Result<Policy, String> {
    if let Some(prop) = world.get::<EcologyProp>(id) {
        return match prop.kind {
            PropKind::Tree if world.get::<CamTrunk>(id).is_some() => Ok(Policy::Solid(YARD_TRUNK_R)),
            PropKind::Tree | PropKind::Mycelium => Ok(Policy::NoCollide),
            PropKind::Stone => Ok(Policy::Solid(YARD_ROCK_R)),
            PropKind::Deer => gap("deer"),
            PropKind::Crystal => gap("crystal"),
        };
    }
    let has = |id: Entity| {
        world.get::<ClimateStone>(id).is_some() || world.get::<LungeTell>(id).is_some()
            || world.get::<RiseTell>(id).is_some() || world.get::<DroidTell>(id).is_some()
            || world.get::<ClearedTell>(id).is_some() || world.get::<BathMark>(id).is_some()
            || world.get::<SkyBackdropPart>(id).is_some() || world.get::<HumanPresence>(id).is_some()
    };
    if has(id) { return Ok(Policy::NoCollide); }
    if world.get::<Predator>(id).is_some() { return gap("predator"); }
    if world.get::<Droid>(id).is_some() { return gap("droid"); }
    if world.get::<Grazer>(id).is_some() { return gap("grazer"); }
    if world.get::<MercyHarvestNode>(id).is_some() { return Ok(Policy::Trigger); }
    if world.get::<StoneGroundTile>(id).is_some() { return Ok(Policy::Walkable); }
    if world.get::<CamTrunk>(id).is_some() {
        let scale = world.get::<Transform>(id).map(|tf| tf.scale.x).unwrap_or(1.0);
        return Ok(Policy::Solid(TRUNK_RADIUS * scale));
    }
    let name = ancestor_name(world, id);
    name.as_deref().and_then(by_name).unwrap_or_else(|| Err(name.unwrap_or_else(|| format!("{id:?}"))))
}
fn mesh_ids(app: &mut App) -> Vec<Entity> {
    let mut query = app.world_mut().query_filtered::<Entity, With<Mesh3d>>();
    query.iter(app.world()).collect()
}
fn assert_declared(app: &mut App) {
    let mut bad: Vec<String> = mesh_ids(app).into_iter().filter_map(|id| policy(app.world(), id).err()).collect();
    bad.sort();
    bad.dedup();
    assert!(bad.is_empty(), "C1 undeclared Mesh3d: {}", bad.join(", "));
}
fn assert_spawn_clear(app: &mut App) {
    let spawn = SoftPresence::default().position.xz();
    let mut solids = 0u32;
    for id in mesh_ids(app) {
        let Ok(Policy::Solid(radius)) = policy(app.world(), id) else { continue };
        if !shown(app.world(), id) { continue; }
        solids += 1;
        let at = app.world().get::<Transform>(id).expect("solid").translation.xz();
        assert!(spawn.distance(at) + 1.0e-4 >= radius + BODY_R, "C3 blocked spawn at {at:?} r {radius}");
    }
    assert!(solids > 0, "C3 saw no shown solid");
}
fn assert_tells(app: &mut App) {
    let mut tells = 0u32;
    for id in mesh_ids(app) {
        let tell = app.world().get::<LungeTell>(id).is_some() || app.world().get::<RiseTell>(id).is_some()
            || app.world().get::<DroidTell>(id).is_some() || app.world().get::<ClearedTell>(id).is_some();
        if !tell { continue; }
        tells += 1;
        let label = ancestor_name(app.world(), id).unwrap_or_else(|| format!("{id:?}"));
        assert_eq!(policy(app.world(), id).ok(), Some(Policy::NoCollide), "C6 solid telegraph {label}");
    }
    assert!(tells > 0, "C6 found no tells");
}
fn boot() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default())).set_error_handler(bevy::ecs::error::ignore)
        .init_asset::<Mesh>().init_asset::<StandardMaterial>()
        .init_asset::<crate::painterly_shader::PainterlyMaterial>()
        .insert_resource(SoftPlayerRealm { current: Some(0) })
        .insert_resource(HexTravelState { current: PlaceId::Sanctuary })
        .insert_resource(SoftPresence::default())
        .insert_resource(LocalHumanSim { active: true, pocket: 0, last_peer_write: 1.0e9 })
        .add_plugins((
            LivingEcologyPlugin, ClimatePlanePlugin, SkyBackdropPlugin, VegetationPlugin, WaterPlugin,
            MercyHarvestNodesPlugin, CombatCorePlugin, DroidPlugin, GrazerPlugin, LocalHumanSimPlugin,
            HumanPresencePlugin, HeartwoodLipPlugin, HeartwoodWardsPlugin, DepthsLandingPlugin,
        ));
    app.update();
    app
}
fn check_place(app: &mut App, place: PlaceId, realm: Option<u8>) {
    app.world_mut().resource_mut::<HexTravelState>().current = place;
    app.world_mut().resource_mut::<SoftPlayerRealm>().current = realm;
    app.update(); app.update();
    assert_declared(app);
    assert_spawn_clear(app);
    assert_tells(app);
}
#[test]
fn c1_c2_c3_c6_per_place() {
    let mut app = boot();
    for realm in [None, Some(0), Some(1), Some(2), Some(4)] {
        check_place(&mut app, PlaceId::Sanctuary, realm);
    }
    check_place(&mut app, PlaceId::Heartwood, Some(0));
    check_place(&mut app, PlaceId::Depths, Some(0));
}

macro_rules! site {
    ($path:literal, $n:literal) => { ($path, include_str!($path).matches(MESH3D).count(), $n as usize) };
}

#[test]
fn mesh3d_site_count_guard() {
    let rows = [
        site!("climate_plane.rs", 9), site!("combat_core.rs", 4), site!("depths_landing.rs", 2),
        site!("droid.rs", 2), site!("flow_weather.rs", 2), site!("grazer.rs", 1),
        site!("heartwood_lip.rs", 4), site!("heartwood_wards.rs", 1), site!("human_presence.rs", 9),
        site!("light_gen.rs", 4), site!("living_ecology.rs", 6), site!("local_human_sim.rs", 1),
        site!("mercy_harvest_nodes.rs", 1), site!("sky_backdrop.rs", 6), site!("stone_ground.rs", 1),
        site!("tend_flow_vfx.rs", 1), site!("vegetation.rs", 2), site!("water.rs", 3),
        site!("wood_expedition.rs", 3),
    ];
    for (name, got, want) in rows { assert_eq!(got, want, "{name}"); }
    let listed: Vec<&str> = rows.iter().map(|row| row.0).collect();
    fn walk(dir: &std::path::Path, listed: &[&str]) {
        for entry in std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()) {
            if entry.is_dir() { walk(&entry, listed); continue; }
            if entry.extension().and_then(|ext| ext.to_str()) != Some("rs") { continue; }
            let count = std::fs::read_to_string(&entry).unwrap().matches(MESH3D).count();
            let name = entry.file_name().unwrap().to_str().unwrap();
            assert!(count == 0 || listed.contains(&name), "C1 file gap: {name} has {count}");
        }
    }
    walk(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src").as_path(), &listed);
}
