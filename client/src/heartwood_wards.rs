//! U8 — Heartwood Wards dress. Spawns only on Heartwood. The dress itself does not persist; only the journey note may appear in the Journey feed.

use bevy::prelude::*;

use crate::first_harvest_epiphany::wards_may_claim_use;
use crate::hex_travel::HexTravelState;
use crate::human_presence::SoftPresence;
use crate::input::PlayerInput;
use crate::skirmish_well::near_first_well;
use shared::heartwood_wards::{
    WardDress, WardVerb, WARD_POST_CENTERS, WARD_POST_SIZE, WARD_SEALS, WARD_USE_RADIUS,
};
use shared::hex_travel::PlaceId;
use crate::climate_plane::{PropMesh, PropMeshFit};

#[derive(Component)]
struct WardPost;

#[derive(Resource, Debug, Default)]
pub struct WardSession {
    pub dress: WardDress,
    pub last_line: String,
    pub near: bool,
}

pub struct HeartwoodWardsPlugin;

impl Plugin for HeartwoodWardsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WardSession>()
            .add_systems(Update, (sync_heartwood_wards, use_heartwood_wards).chain());
    }
}

fn sync_heartwood_wards(
    mut commands: Commands,
    travel: Res<HexTravelState>,
    mut session: ResMut<WardSession>,
    existing: Query<Entity, With<WardPost>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let heartwood = travel.current == PlaceId::Heartwood;
    if !heartwood {
        for entity in &existing {
            commands.entity(entity).despawn();
        }
        session.dress = WardDress::default();
        session.last_line.clear();
        return;
    }
    if !existing.is_empty() {
        return;
    }
    let mesh = meshes.add(Cuboid::new(
        WARD_POST_SIZE[0],
        WARD_POST_SIZE[1],
        WARD_POST_SIZE[2],
    ));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.62, 0.48),
        emissive: LinearRgba::new(0.02, 0.03, 0.012, 1.0),
        perceptual_roughness: 0.8,
        ..default()
    });
    for (i, center) in WARD_POST_CENTERS.iter().enumerate() {
        commands.spawn((
            (
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(Vec3::from_array(*center)),
            ),
            PropMesh { rel: "models/kenney-fantasy-town-kit/pillar-stone.glb", primitive: 0, fit: PropMeshFit::Post },
            WardPost,
            Name::new(format!("WardPost{i}")),
        ));
    }
}

/// `{place} · Well · Grove · Ember tended` from the existing ward seal words.
fn ward_place_line(place: &str) -> String {
    format!("{place} · {} tended", WARD_SEALS.join(" · "))
}

pub(crate) fn use_heartwood_wards(
    travel: Res<HexTravelState>,
    input: Res<PlayerInput>,
    presence: Res<SoftPresence>,
    mut session: ResMut<WardSession>,
    mut epiphany: Option<ResMut<crate::first_harvest_epiphany::FirstHarvestEpiphany>>,
    mut echo: Option<ResMut<crate::abundance_journey_echo::AbundanceJourneyEcho>>,
    wood: Option<Res<crate::wood_expedition::WoodClaim>>,
) {
    if travel.current != PlaceId::Heartwood {
        session.near = false;
        if let Some(epiphany) = epiphany.as_deref_mut() {
            epiphany.wards_near = false;
        }
        return;
    }
    let body = Vec2::new(presence.position.x, presence.position.z);
    let near = near_ward_post(body);
    // Same reach `mark_well_near` / `handle_well` already use. Not a new radius.
    let well_in_reach = near_first_well(&presence);
    session.near = near;
    if let Some(epiphany) = epiphany.as_deref_mut() {
        epiphany.wards_near = near;
        epiphany.well_in_reach = well_in_reach;
    }
    if !near {
        return;
    }
    // Session dress only. Do not load or write the house pack from this hook.
    if !session.dress.looked {
        session.last_line = session.dress.apply(WardVerb::Look).into();
        info!(target: "powrush::wards", "{}", session.last_line);
    }
    if wood.as_ref().is_some_and(|claim| claim.blocks()) {
        return;
    }
    // CARD WELL-BEFORE-WARDS-1 — a well in reach owns E. Wards claim Use only alone.
    if !claim_ward_use(near, well_in_reach, input.interact, &mut session) {
        return;
    }
    let Some(echo) = echo.as_mut() else {
        return;
    };
    let text = ward_place_line(travel.chip_name());
    if echo.lines.iter().any(|existing| existing.text == text) {
        return;
    }
    echo.push(crate::abundance_journey_echo::JourneyKind::Note, text);
}

/// E dresses the posts only when Wards may claim Use.
/// A well in the skirmish reach makes this return false.
pub(crate) fn claim_ward_use(
    near: bool,
    well_in_reach: bool,
    interact: bool,
    session: &mut WardSession,
) -> bool {
    if !interact || !wards_may_claim_use(near, well_in_reach) {
        return false;
    }
    session.last_line = session.dress.apply(WardVerb::Tend).into();
    info!(target: "powrush::wards", "{}", session.last_line);
    true
}

fn near_ward_post(body: Vec2) -> bool {
    WARD_POST_CENTERS
        .iter()
        .any(|c| body.distance(Vec2::new(c[0], c[2])) <= WARD_USE_RADIUS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_radius_belongs_to_posts_only() {
        let post = WARD_POST_CENTERS[0];
        assert!(near_ward_post(Vec2::new(post[0], post[2])));
        assert!(!near_ward_post(Vec2::ZERO));
    }

    #[test]
    fn ward_place_line_names_heartwood_seals() {
        assert_eq!(
            ward_place_line("Heartwood"),
            "Heartwood · Well · Grove · Ember tended"
        );
    }

    fn tend_app(place: PlaceId, interact: bool, with_echo: bool) -> App {
        let post = WARD_POST_CENTERS[0];
        let mut app = App::new();
        app.insert_resource(HexTravelState { current: place });
        app.insert_resource(SoftPresence {
            position: Vec3::new(post[0], post[1], post[2]),
            velocity: Vec3::ZERO,
            grounded: true,
            ..Default::default()
        });
        app.insert_resource(PlayerInput {
            interact,
            ..default()
        });
        app.init_resource::<WardSession>();
        if with_echo {
            app.init_resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>();
        }
        app.add_systems(Update, use_heartwood_wards);
        app
    }

    #[test]
    fn tend_near_a_post_notes_the_place_once() {
        let mut app = tend_app(PlaceId::Heartwood, true, true);
        app.update();
        {
            let echo = app
                .world()
                .resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>();
            assert_eq!(echo.lines.len(), 1);
            assert_eq!(echo.lines[0].text, ward_place_line("Heartwood"));
            assert_eq!(
                echo.lines[0].kind,
                crate::abundance_journey_echo::JourneyKind::Note
            );
        }
        app.update();
        let echo = app
            .world()
            .resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>();
        assert_eq!(echo.lines.len(), 1);
        assert_eq!(
            echo.lines[0].text,
            "Heartwood · Well · Grove · Ember tended"
        );
    }

    /// CARD WELL-BEFORE-WARDS-1 — a post with no well in reach still claims Use.
    #[test]
    fn wards_only_still_claim_use() {
        let mut app = tend_app(PlaceId::Heartwood, true, true);
        app.init_resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>();
        app.update();
        let epi = app
            .world()
            .resource::<crate::first_harvest_epiphany::FirstHarvestEpiphany>();
        assert!(epi.wards_near);
        assert!(!epi.well_in_reach);
        assert!(epi.harvest_use_is_claimed());
        assert!(crate::first_harvest_epiphany::wards_may_claim_use(
            epi.wards_near,
            epi.well_in_reach
        ));
        let session = app.world().resource::<WardSession>();
        assert_eq!(session.dress.tends, 1);
        assert_eq!(session.last_line, shared::heartwood_wards::WARDS_NOTICE);
    }

    #[test]
    fn look_and_leaving_and_missing_echo_push_nothing() {
        let mut look = tend_app(PlaceId::Heartwood, false, true);
        look.update();
        assert!(look
            .world()
            .resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>()
            .lines
            .is_empty());
        assert!(look.world().resource::<WardSession>().dress.looked);

        let mut away = tend_app(PlaceId::Sanctuary, true, true);
        away.update();
        assert!(away
            .world()
            .resource::<crate::abundance_journey_echo::AbundanceJourneyEcho>()
            .lines
            .is_empty());

        let mut bare = tend_app(PlaceId::Heartwood, true, false);
        bare.update();
        assert_eq!(
            bare.world().resource::<WardSession>().last_line,
            shared::heartwood_wards::WARDS_NOTICE
        );
    }

    #[test]
    fn wood_claim_does_not_tend() {
        let mut app = tend_app(PlaceId::Heartwood, true, false);
        app.insert_resource(crate::wood_expedition::WoodClaim::default());
        app.world_mut().resource_mut::<crate::wood_expedition::WoodClaim>().set_held(true);
        app.update();
        assert_eq!(app.world().resource::<WardSession>().dress.tends, 0);
        assert!(app.world().resource::<WardSession>().near);
        app.world_mut().resource_mut::<crate::wood_expedition::WoodClaim>().set_held(false);
        app.update();
        assert_eq!(app.world().resource::<WardSession>().dress.tends, 1);
    }
    fn ww_root() -> std::path::PathBuf { std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").canonicalize().unwrap() }
    fn ww_tick(app: &mut App, root: &std::path::Path, n: usize) { for _ in 0..n { crate::test_env::with_bevy_asset_root(root, || { app.update(); bevy::tasks::tick_global_task_pools_on_main_thread(); }); } }
    fn ww_boot(root: &std::path::Path, preset: shared::local_settings::GraphicsPreset) -> App {
        use bevy::prelude::AssetPlugin; let mut app = App::new();
        app.set_error_handler(bevy::ecs::error::ignore).add_plugins((MinimalPlugins, AssetPlugin { file_path: root.join("assets").into_os_string().into_string().unwrap(), watch_for_changes_override: Some(false), use_asset_processor_override: Some(false), ..AssetPlugin::default() }, bevy::image::ImagePlugin::default(), bevy::mesh::MeshPlugin, bevy::gltf::GltfPlugin::default(), bevy::world_serialization::WorldSerializationPlugin)).init_asset::<Image>().init_asset::<Mesh>().init_asset::<StandardMaterial>().insert_resource(crate::local_settings::LocalMeshLodFeel { preset }).insert_resource(crate::living_practice_loop::SoftPlayerRealm { current: Some(0) }).insert_resource(HexTravelState { current: PlaceId::Heartwood }).insert_resource(SoftPresence { position: Vec3::from_array(WARD_POST_CENTERS[0]), velocity: Vec3::ZERO, grounded: true, ..Default::default() }).insert_resource(PlayerInput::default()).add_plugins((crate::climate_plane::ClimatePlanePlugin, crate::heartwood_lip::HeartwoodLipPlugin, HeartwoodWardsPlugin));
        app.finish(); app.cleanup(); crate::test_env::single_thread(&mut app); app.world_mut().run_schedule(Startup); app
    }
    #[test]
    fn props_wire_1b_ward_posts_swap_world_fit() {
        let _g = crate::test_env::lock(); let root = ww_root(); let mut app = ww_boot(&root, shared::local_settings::GraphicsPreset::Medium); ww_tick(&mut app, &root, 1);
        let ids = |app: &mut App| { let mut q = app.world_mut().query::<(&Name, &Transform, &Mesh3d)>(); q.iter(app.world()).filter(|(n, _, _)| n.as_str().starts_with("WardPost")).map(|(n, t, m)| (n.as_str().to_string(), *t, m.0.id())).collect::<Vec<_>>() };
        let pre = ids(&mut app); assert_eq!(pre.len(), 3, "spawn");
        for _ in 0..900 { ww_tick(&mut app, &root, 1); if ids(&mut app).iter().all(|(_, _, h)| pre.iter().all(|p| p.2 != *h)) { break; } }
        let rows = ids(&mut app); assert!(rows.len() == 3 && rows.iter().all(|(_, _, h)| pre.iter().all(|p| p.2 != *h)) && rows.iter().all(|(_, _, h)| *h == rows[0].2), "swap");
        let meshes = app.world().resource::<Assets<Mesh>>();
        for (name, tf, id) in &rows {
            let mesh = meshes.get(*id).unwrap(); let pos = mesh.attribute(Mesh::ATTRIBUTE_POSITION).and_then(|a| a.as_float3()).unwrap();
            let mut ix: Vec<usize> = match mesh.indices() { Some(bevy::mesh::Indices::U32(i)) => i.iter().map(|&u| u as usize).collect(), Some(bevy::mesh::Indices::U16(i)) => i.iter().map(|&u| u as usize).collect(), None => (0..pos.len()).collect() }; ix.sort_unstable(); ix.dedup();
            let (mut lo, mut hi, mut rad) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN), 0_f32); for i in ix { let w = tf.transform_point(Vec3::from_array(pos[i])); lo = lo.min(w); hi = hi.max(w); rad = rad.max((w.x - tf.translation.x).hypot(w.z - tf.translation.z)); }
            let size = hi - lo; let idx: usize = name.trim_start_matches("WardPost").parse().unwrap();
            assert!((size.x - 0.28).abs() < 0.01 && (size.y - 1.4).abs() < 0.01 && (size.z - 0.28).abs() < 0.01 && lo.y.abs() < 0.02 && rad <= shared::heartwood_wards::WARD_FOOTPRINT_RADIUS + 1e-4 && (tf.translation - Vec3::from_array(WARD_POST_CENTERS[idx])).length() < 1e-4 && tf.rotation.dot(Quat::IDENTITY).abs() > 1.0 - 1e-4, "{name} {lo} {size} {rad}");
            if idx == 0 { eprintln!("PW1B post_base_y={} size={size:?} center={:?}", lo.y, tf.translation); }
        }
        { let mut p = app.world_mut().resource_mut::<SoftPresence>(); p.position = Vec3::from_array(WARD_POST_CENTERS[0]); } app.world_mut().resource_mut::<PlayerInput>().interact = true; ww_tick(&mut app, &root, 1); assert_eq!(app.world().resource::<WardSession>().dress.tends, 1);
    }
    #[test]
    fn props_wire_1b_low_mobile_keep_primitives() {
        let _g = crate::test_env::lock(); let root = ww_root();
        let cap = Mesh::from(Capsule3d::new(0.32, 2.0)).attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len(); let rib = Mesh::from(Cuboid::new(0.16, 0.16, 3.0)).attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len(); let post = Mesh::from(Cuboid::new(WARD_POST_SIZE[0], WARD_POST_SIZE[1], WARD_POST_SIZE[2])).attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len();
        for preset in [shared::local_settings::GraphicsPreset::Low, shared::local_settings::GraphicsPreset::Mobile] {
            let mut app = ww_boot(&root, preset); ww_tick(&mut app, &root, 8); let mut q = app.world_mut().query::<(&Name, &Mesh3d)>();
            let rows: Vec<_> = q.iter(app.world()).filter(|(n, _)| n.as_str().starts_with("HeartwoodWalkway") || n.as_str().starts_with("HeartwoodHangingRib") || n.as_str().starts_with("WardPost")).map(|(n, m)| (n.as_str().to_string(), m.0.id())).collect();
            assert_eq!(rows.len(), 10, "{preset:?}"); let meshes = app.world().resource::<Assets<Mesh>>();
            for (name, id) in &rows { let n = meshes.get(*id).unwrap().attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len(); let want = if name.starts_with("HeartwoodWalkway") { cap } else if name.starts_with("WardPost") { post } else { rib }; assert_eq!(n, want, "{preset:?} {name}"); }
        }
    }
    #[test]
    fn props_wire_1b_assets_licensed() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".."); let csv = include_str!("../../assets/asset-licenses.csv"); let mut total = 0_u64;
        for rel in ["assets/models/kenney-nature-kit/log_large.glb", "assets/models/kenney-fantasy-town-kit/pillar-wood.glb", "assets/models/kenney-fantasy-town-kit/pillar-stone.glb"] { assert!(csv.lines().any(|line| line.starts_with(rel) && line.split(',').nth(2) == Some("CC0-1.0")), "{rel}"); let n = std::fs::metadata(root.join(rel)).unwrap().len(); assert!(n <= 40 * 1024, "{rel} {n}"); total += n; }
        for rel in ["assets/models/kenney-nature-kit/tree_default.glb", "assets/models/kenney-nature-kit/stone_smallFlatA.glb"] { total += std::fs::metadata(root.join(rel)).unwrap().len(); }
        assert!(!csv.contains("colormap.png") && !root.join("assets/models/kenney-fantasy-town-kit/Textures").exists() && total <= 120 * 1024, "{total}");
        assert!(!crate::climate_plane::PropMesh::props_wire_authored_glb_present());
        assert!(std::process::Command::new("bash").arg(root.join("scripts/check_asset_licenses.sh")).arg(&root).status().unwrap().success());
    }
}
