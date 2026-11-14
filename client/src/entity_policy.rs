#![cfg(test)]
//! ENTITY-CONTRACT-1a. C1 names any Mesh3d with no policy. C2: walk-throughs are only `KNOWN_GAPS` rows, so dropping a row fails C1. C3: each place's spawn is on the yard and misses shown solids. C6: tells are NoCollide.
//! ENTITY-CONTRACT-1b. C4: every free cell of the PLAY_HALF yard joins the spawn's region (a `pocket@x,z` gap row excuses one). C5: every Trigger or usable has a reached cell inside its named use radius.
//! C4 walls the sweep at PLAY_HALF itself, not the 27.7 inset (PLAY_HALF − body radius). Locomotion has no plate clamp yet, so an edge-pinned pocket here may be a false red, and pockets the inset would make wait on YARD-EDGE-1 (C7).

use bevy::prelude::*;
use shared::{heartwood_wards::WARD_USE_RADIUS, hex_travel::PlaceId, skirmish_well::{CONTEST_REACH, WELL_ANCHORS}};
use shared::threshold_shelf::THRESHOLD_SHELF_USE_RADIUS;
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
    mercy_harvest_nodes::{MercyHarvestNode, MercyHarvestNodesPlugin, HARVEST_REACH},
    sky_backdrop::{SkyBackdropPart, SkyBackdropPlugin},
    stone_ground::StoneGroundTile,
    terrain::PLAY_HALF,
    vegetation::{CamTrunk, VegetationPlugin, TRUNK_RADIUS},
    water::{BathMark, WaterPlugin},
};

/// Lived trunk radius. Push-out adds it to each solid; `human_presence` keeps that const private.
const BODY_R: f32 = 0.30;
const MESH3D: &str = concat!("Mesh3d", "(");
/// C4/C5 sweep step (m). A cell is free when its centre is outside every shown solid's push-out circle.
const STEP: f32 = 0.1;
const CELLS: usize = (2.0 * PLAY_HALF / STEP + 0.5) as usize + 1;
/// (place, dress realm, shown solids, C5 uses). Sanctuary rows pick the realm; Heartwood and Depths assert the travel sync's.
const PLACES: [(PlaceId, Option<u8>, usize, usize); 7] = [
    (PlaceId::Sanctuary, None, 11, 7), (PlaceId::Sanctuary, Some(0), 11, 7), (PlaceId::Sanctuary, Some(1), 3, 7),
    (PlaceId::Sanctuary, Some(2), 5, 7), (PlaceId::Sanctuary, Some(4), 3, 7), (PlaceId::Heartwood, Some(2), 5, 8), (PlaceId::Depths, Some(3), 0, 4),
];
/// Owning card per walk-through. The row is that mesh's only declaration.
const KNOWN_GAPS: &[(&str, &str)] = &[
    ("deer", "CREATURE-BODY-1"), ("crystal", "PROP-COLLIDE-1"), ("traveler", "CREATURE-BODY-1"),
    ("predator", "CREATURE-BODY-1"), ("droid", "CREATURE-BODY-1"), ("grazer", "CREATURE-BODY-1"),
    ("ward", "PROP-COLLIDE-1"), ("walkway", "PROP-COLLIDE-1"), ("rib", "PROP-COLLIDE-1"), ("shelf", "PROP-COLLIDE-1"),
];

#[derive(Clone, Copy, PartialEq, Debug)]
enum Policy { Solid(f32), NoCollide, Walkable, Trigger(f32), Gap }

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
    if name == "ThresholdPeaceNode" { return Some(Ok(Policy::Trigger(THRESHOLD_SHELF_USE_RADIUS))); }
    if name == "DepthsPeaceNode" { return Some(Ok(Policy::Trigger(crate::depths_landing::DEPTHS_USE_RADIUS))); }
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
            || world.get::<HumanPresence>(id).is_some()
    };
    if has(id) { return Ok(Policy::NoCollide); }
    // EC-1c: the belt is one merged mesh at the origin; `ec_1c_belt_trunks_stay_off_the_plate` is its R-C.
    if let Some(part) = world.get::<SkyBackdropPart>(id) {
        return match part {
            SkyBackdropPart::Dome | SkyBackdropPart::Ground | SkyBackdropPart::Ridge | SkyBackdropPart::Band(_) => Ok(Policy::NoCollide),
            SkyBackdropPart::BeltFull | SkyBackdropPart::BeltLow => Ok(Policy::NoCollide),
        };
    }
    if world.get::<Predator>(id).is_some() { return gap("predator"); }
    if world.get::<Droid>(id).is_some() { return gap("droid"); }
    if world.get::<Grazer>(id).is_some() { return gap("grazer"); }
    if world.get::<MercyHarvestNode>(id).is_some() { return Ok(Policy::Trigger(HARVEST_REACH)); }
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
fn shown_solids(app: &mut App) -> Vec<(Vec2, f32)> {
    let ids = mesh_ids(app);
    let world = app.world();
    let solid = |id| match policy(world, id) { Ok(Policy::Solid(r)) if shown(world, id) => Some(r), _ => None };
    ids.into_iter().filter_map(|id| Some((world.get::<Transform>(id)?.translation.xz(), solid(id)?))).collect()
}
/// Sanctuary keeps the default body; Heartwood and Depths seat it on entry.
fn spawn_of(app: &App, place: PlaceId) -> Vec3 {
    if place == PlaceId::Sanctuary { SoftPresence::default().position } else { app.world().resource::<SoftPresence>().position }
}
fn assert_spawn_clear(place: PlaceId, spawn: Vec3, solids: &[(Vec2, f32)]) {
    assert!(spawn.x.abs() <= PLAY_HALF && spawn.z.abs() <= PLAY_HALF, "C3 {place:?} spawn {spawn} off the yard");
    for &(at, radius) in solids {
        assert!(spawn.xz().distance(at) + 1.0e-4 >= radius + BODY_R, "C3 {place:?} spawn blocked at {at:?} r {radius}");
    }
}
fn cell_xz(i: usize) -> Vec2 {
    Vec2::new((i % CELLS) as f32, (i / CELLS) as f32) * STEP - Vec2::splat(PLAY_HALF)
}
fn fill(free: &[bool], seen: &mut [bool], start: usize) -> usize {
    let (mut stack, mut count) = (vec![start], 0);
    seen[start] = true;
    while let Some(i) = stack.pop() {
        count += 1;
        let (x, z) = (i % CELLS, i / CELLS);
        let next = [(x > 0).then(|| i - 1), (x + 1 < CELLS).then(|| i + 1), (z > 0).then(|| i - CELLS), (z + 1 < CELLS).then(|| i + CELLS)];
        for j in next.into_iter().flatten() {
            if free[j] && !seen[j] { seen[j] = true; stack.push(j); }
        }
    }
    count
}
/// Free cells, and the cells the body reaches from `spawn` in 4-neighbour steps.
fn sweep(solids: &[(Vec2, f32)], spawn: Vec2) -> (Vec<bool>, Vec<bool>) {
    let mut free = vec![true; CELLS * CELLS];
    let span = |v: f32, reach: f32| {
        let lo = ((v - reach + PLAY_HALF) / STEP).floor().max(0.0) as usize;
        lo..=(((v + reach + PLAY_HALF) / STEP).ceil().max(0.0) as usize).min(CELLS - 1)
    };
    for &(center, radius) in solids {
        let reach = radius + BODY_R;
        for z in span(center.y, reach) {
            for x in span(center.x, reach) {
                if cell_xz(z * CELLS + x).distance(center) < reach { free[z * CELLS + x] = false; }
            }
        }
    }
    let at = ((spawn + Vec2::splat(PLAY_HALF)) / STEP).round();
    let start = at.y as usize * CELLS + at.x as usize;
    assert!(free[start], "C4 spawn cell {:?} is blocked", cell_xz(start));
    let mut reached = vec![false; CELLS * CELLS];
    fill(&free, &mut reached, start);
    (free, reached)
}
/// Free regions the spawn's flood never reached, minus `pocket@x,z` gap rows.
fn pockets(free: &[bool], reached: &[bool]) -> Vec<String> {
    let (mut seen, mut bad) = (reached.to_vec(), Vec::new());
    for i in 0..free.len() {
        if !free[i] || seen[i] { continue; }
        let cells = fill(free, &mut seen, i);
        let at = cell_xz(i);
        let key = format!("pocket@{:.0},{:.0}", at.x, at.y);
        if gap(&key).is_err() { bad.push(format!("{key} ({cells} cells)")); }
    }
    bad
}
/// A use target: label, centre, named radius, and whether the lived check is xz (else 3D from the standing body).
type Use = (String, Vec3, f32, bool);
fn uses_of(app: &mut App, place: PlaceId) -> Vec<Use> {
    let mut uses = Vec::new();
    for id in mesh_ids(app) {
        let world = app.world();
        if !shown(world, id) { continue; }
        let label = ancestor_name(world, id).unwrap_or_else(|| format!("{id:?}"));
        let at = world.get::<Transform>(id).map(|tf| tf.translation).unwrap_or_default();
        match policy(world, id) {
            Ok(Policy::Trigger(r)) => uses.push((label, at, r, world.get::<MercyHarvestNode>(id).is_none())),
            Ok(Policy::Gap) if label.starts_with("WardPost") => uses.push((label, at, WARD_USE_RADIUS, true)),
            _ => {}
        }
    }
    let spot = |label: &str, [x, z]: [f32; 2], r| (label.to_string(), Vec3::new(x, 0.0, z), r, true);
    if place == PlaceId::Sanctuary {
        let (x, y, z) = WELL_ANCHORS[0];
        uses.push(("SkirmishWell0".into(), Vec3::new(x, y, z), CONTEST_REACH, false));
        use crate::wellsworn::{BOOK_SPOT, PLAQUE_SPOT, SPOT_REACH, STAVEHOLD_GATE_SPOT};
        uses.extend([spot("WellswornPlaque", PLAQUE_SPOT, SPOT_REACH), spot("WellswornBook", BOOK_SPOT, SPOT_REACH)]);
        uses.push(spot("WellswornGate", STAVEHOLD_GATE_SPOT, SPOT_REACH));
    }
    if place == PlaceId::Heartwood {
        use crate::heartwood_plaque::{PLAQUE_REACH, PLAQUE_SPOT};
        uses.push(spot("HeartwoodPlaque", PLAQUE_SPOT, PLAQUE_REACH));
    }
    uses
}
/// Uses with no reached cell inside their radius, sorted.
fn unreachable(uses: &[Use], reached: &[bool]) -> Vec<String> {
    let y = SoftPresence::default().position.y;
    let near = |&(_, at, r, xz): &Use, b: Vec3| if xz { b.xz().distance(at.xz()) <= r } else { b.distance(at) <= r };
    let hit = |u: &Use| (0..reached.len()).any(|i| reached[i] && near(u, cell_xz(i).extend(y).xzy()));
    let mut bad: Vec<String> = uses.iter().filter(|u| !hit(u)).map(|u| u.0.clone()).collect();
    bad.sort();
    bad
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
fn boot() -> App { boot_with(|_| {}) }
/// `pre` runs before the first update, so Startup sees what it inserts.
fn boot_with(pre: impl FnOnce(&mut App)) -> App {
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
    pre(&mut app);
    app.update();
    app
}
/// Travel first: `sync_place_dress_from_travel` rewrites the realm on a travel change, so the Sanctuary row is set after it.
/// MinimalPlugins inserts no `Visibility`; bare meshes get one so `dress_for_climate` gates yard trees and rocks per realm.
fn enter(app: &mut App, place: PlaceId, realm: Option<u8>) {
    app.world_mut().resource_mut::<HexTravelState>().current = place;
    app.update();
    if place == PlaceId::Sanctuary { app.world_mut().resource_mut::<SoftPlayerRealm>().current = realm; }
    assert_eq!(app.world().resource::<SoftPlayerRealm>().current, realm, "{place:?} dress realm");
    let mut bare = app.world_mut().query_filtered::<Entity, (With<Mesh3d>, Without<Visibility>)>();
    let bare: Vec<Entity> = bare.iter(app.world()).collect();
    for id in bare { app.world_mut().entity_mut(id).insert(Visibility::Inherited); }
    app.update(); app.update();
}
#[test]
fn c1_c2_c3_c6_per_place() {
    let mut app = boot();
    for (place, realm, solids, _) in PLACES {
        enter(&mut app, place, realm);
        assert_declared(&mut app);
        let shown = shown_solids(&mut app);
        assert_eq!(shown.len(), solids, "C3 {place:?} {realm:?} shown solids");
        assert_spawn_clear(place, spawn_of(&app, place), &shown);
        assert_tells(&mut app);
    }
}
#[test]
fn c4_c5_per_place() {
    let mut app = boot();
    for (place, realm, _, count) in PLACES {
        enter(&mut app, place, realm);
        let (free, reached) = sweep(&shown_solids(&mut app), spawn_of(&app, place).xz());
        assert_eq!(pockets(&free, &reached), Vec::<String>::new(), "C4 trapped pocket in {place:?} {realm:?}");
        let uses = uses_of(&mut app, place);
        assert_eq!(uses.len(), count, "C5 {place:?} uses {:?}", uses.iter().map(|u| &u.0).collect::<Vec<_>>());
        assert_eq!(unreachable(&uses, &reached), Vec::<String>::new(), "C5 unreachable use in {place:?} {realm:?}");
    }
}
#[test]
fn c4_c5_fail_first_fixtures() {
    let ring: Vec<(Vec2, f32)> = (0..24).map(|k| (Vec2::splat(10.0) + Vec2::from_angle(k as f32 * std::f32::consts::TAU / 24.0) * 2.0, BODY_R)).collect();
    let (free, reached) = sweep(&ring, Vec2::ZERO);
    let trapped = pockets(&free, &reached);
    assert!(trapped.len() == 1 && trapped[0].starts_with("pocket@10,9 "), "{trapped:?}");
    let (x, y, z) = WELL_ANCHORS[0];
    let node = Vec3::new(x, y, z);
    let (_, reached) = sweep(&[(node.xz(), HARVEST_REACH)], Vec2::ZERO);
    let uses = [
        ("SwallowedNode".to_string(), node, HARVEST_REACH, false),
        ("OutOfYard".to_string(), Vec3::new(PLAY_HALF + 4.0, y, 0.0), HARVEST_REACH, false),
        ("OpenNode".to_string(), Vec3::new(-10.0, y, -10.0), HARVEST_REACH, false),
    ];
    assert_eq!(unreachable(&uses, &reached), ["OutOfYard", "SwallowedNode"]);
}
/// EC-1c R-C: no belt trunk (r 0.18·s) reaches the plate inflated by BODY_R, so NoCollide on the belt hides no solid.
#[test]
fn ec_1c_belt_trunks_stay_off_the_plate() {
    use crate::sky_backdrop::{conifer_spots, CONIFER_COUNT, CONIFER_SEED};
    let on_plate: Vec<(i32, i32)> = conifer_spots(CONIFER_SEED, CONIFER_COUNT).iter()
        .filter(|(p, s)| p.x.abs().max(p.z.abs()) <= PLAY_HALF + BODY_R + 0.18 * s)
        .map(|(p, _)| (p.x.round() as i32, p.z.round() as i32)).collect();
    assert_eq!(on_plate, Vec::<(i32, i32)>::new(), "EC-1c belt trunks on the plate");
}

macro_rules! site {
    ($path:literal, $n:literal) => { ($path, include_str!($path).matches(MESH3D).count(), $n as usize) };
}

#[test]
fn mesh3d_site_count_guard() {
    let rows = [
        site!("climate_plane.rs", 10), site!("combat_core.rs", 4), site!("depths_landing.rs", 2),
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
/// PLATE-EDGE-CUE-1 P3 — in plane and stone modes the 36 rim pieces are shown NoCollide, C1 holds, and the C3/C4/C5 rows stay.
#[test]
fn plate_edge_p3_rim_is_nocollide_in_plane_and_stone_modes() {
    use crate::stone_ground::{tests::{layout, scratch, write_pack, PNG_1X1}, StoneAssetDir};
    let dir = scratch("edge-p3");
    write_pack(&dir, &layout(), PNG_1X1);
    for stone in [false, true] {
        let mut app = boot_with(|app| if stone { app.init_asset::<Image>().insert_resource(StoneAssetDir(dir.clone())); });
        for (place, realm, solids, count) in PLACES {
            enter(&mut app, place, realm);
            assert_declared(&mut app);
            let ids = mesh_ids(&mut app);
            let world = app.world();
            let edge: Vec<Entity> = ids.into_iter().filter(|&id| world.get::<Name>(id).is_some_and(|n| n.as_str() == "PlateEdge")).collect();
            let ok = edge.iter().all(|&id| policy(world, id).ok() == Some(Policy::NoCollide) && shown(world, id));
            assert!(edge.len() == 36 && ok, "stone={stone} {place:?} {realm:?} rim {} nocollide+shown {ok}", edge.len());
            let mut tiles = app.world_mut().query::<(Entity, &StoneGroundTile)>();
            let tiles: Vec<Entity> = tiles.iter(app.world()).map(|(id, _)| id).collect();
            let lit = tiles.iter().filter(|&&id| shown(app.world(), id)).count();
            let want = if stone && place == PlaceId::Sanctuary && matches!(realm, None | Some(0)) { 64 } else { 0 };
            assert_eq!((tiles.len(), lit), (if stone { 64 } else { 0 }, want), "stone={stone} {place:?} {realm:?} tiles");
            let shown = shown_solids(&mut app);
            assert_eq!(shown.len(), solids, "stone={stone} C3 {place:?} {realm:?} shown solids");
            let (free, reached) = sweep(&shown, spawn_of(&app, place).xz());
            assert_eq!(pockets(&free, &reached), Vec::<String>::new(), "stone={stone} C4 {place:?} {realm:?}");
            let uses = uses_of(&mut app, place);
            assert_eq!(uses.len(), count, "stone={stone} C5 {place:?} {realm:?} uses");
            assert_eq!(unreachable(&uses, &reached), Vec::<String>::new(), "stone={stone} C5 {place:?} {realm:?}");
        }
    }
}
