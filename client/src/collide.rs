//! CARD COLLIDE-CORE-1 — shared rock and trunk push-out solids.

use std::f32::consts::FRAC_1_SQRT_2;

use bevy::prelude::*;

/// CARD PT-COLLIDE-1 — player body radius for the trunk push-out. Playtest-tunable placeholder.
pub(crate) const PLAYER_TRUNK_RADIUS: f32 = 0.3;

/// CARD PT-COLLIDE-1 — position-only push. Tests call this; locomotion uses [`push_out_clear`].
pub(crate) fn push_out_of_trunks(pos: &mut Vec3, trunks: impl IntoIterator<Item = (Vec2, f32)>) {
    let mut vel = Vec3::ZERO;
    push_out_clear(pos, &mut vel, trunks);
}

/// CARD MOVE-FEEL-1 M8 — push out on x/z and drop the inward part of `vel`. Tangent is kept.
pub(crate) fn push_out_clear(pos: &mut Vec3, vel: &mut Vec3, trunks: impl IntoIterator<Item = (Vec2, f32)>) {
    for (center, radius) in trunks {
        let (off, reach) = (pos.xz() - center, radius + PLAYER_TRUNK_RADIUS);
        if off.length_squared() < reach * reach {
            let n = off.try_normalize().unwrap_or(Vec2::X);
            let out = center + n * reach;
            (pos.x, pos.z) = (out.x, out.y);
            let into = vel.x * n.x + vel.z * n.y;
            if into < 0.0 {
                vel.x -= into * n.x;
                vel.z -= into * n.y;
            }
        }
    }
}

/// CARD ROCK-SHAPE-1 — a yard rock is its yawed, scaled, unjittered mesh ellipse grown by the body disk. It
/// blocks only while the feet are under the rock top (jitter envelope). The body moves to the closest ellipse
/// point and the inward part of `vel` along that ellipse normal drops (M8). Tangent is kept.
pub(crate) fn push_out_of_rock(pos: &mut Vec3, vel: &mut Vec3, spot: Vec3, stand: f32, feet: f32) {
    use crate::living_ecology::{yard_prop_yaw, YARD_ROCK_JITTER, YARD_ROCK_MESH_R, YARD_ROCK_SCALE};
    if pos.y - stand + feet >= spot.y + YARD_ROCK_MESH_R * YARD_ROCK_SCALE.y * (1.0 + YARD_ROCK_JITTER) {
        return;
    }
    let ab = Vec2::new(YARD_ROCK_SCALE.x, YARD_ROCK_SCALE.z) * YARD_ROCK_MESH_R + PLAYER_TRUNK_RADIUS;
    // Rock-local x and z in world x/z under `Quat::from_rotation_y(yaw)`.
    let (s, c) = yard_prop_yaw(spot.x, spot.z).sin_cos();
    let (u, w) = (Vec2::new(c, -s), Vec2::new(s, c));
    let off = pos.xz() - spot.xz();
    let local = Vec2::new(off.dot(u), off.dot(w));
    if (local / ab).length_squared() >= 1.0 {
        return;
    }
    // Closest point in the first quadrant: three fixed evolute steps (inside points included, < 1e-5 m).
    let (p, mut t) = (local.abs(), Vec2::splat(FRAC_1_SQRT_2));
    for _ in 0..3 {
        let e = Vec2::new(ab.x * ab.x - ab.y * ab.y, ab.y * ab.y - ab.x * ab.x) * t * t * t / ab;
        let (r, q) = ((ab * t - e).length(), (p - e).length().max(1e-6));
        t = (((p - e) * r / q + e) / ab).clamp(Vec2::ZERO, Vec2::ONE).try_normalize().unwrap_or(Vec2::X);
    }
    let on = ab * t * Vec2::new(local.x.signum(), local.y.signum());
    let out = spot.xz() + u * on.x + w * on.y;
    (pos.x, pos.z) = (out.x, out.y);
    let n = (u * (on.x / (ab.x * ab.x)) + w * (on.y / (ab.y * ab.y))).try_normalize().unwrap_or(u);
    let into = vel.x * n.x + vel.z * n.y;
    if into < 0.0 {
        vel.x -= into * n.x;
        vel.z -= into * n.y;
    }
}

/// CARD COLLIDE-SPLIT-1 — Heartwood ward posts push as their footprint disk, only where they spawn.
/// CARD CRYSTAL-SOLID-1 — yard crystal spires push as their spire disk, only where they show (realm 1/4).
pub(crate) fn prop_solids(place: Option<shared::hex_travel::PlaceId>, realm: Option<u8>) -> impl Iterator<Item = (Vec2, f32)> {
    use crate::living_ecology::{crystals_shown, YARD_CRYSTAL_R, YARD_CRYSTAL_SPOTS};
    use shared::{heartwood_wards::WARD_POST_CENTERS, hex_travel::PlaceId};
    (place == Some(PlaceId::Heartwood)).then_some(WARD_POST_CENTERS).into_iter().flatten()
        .map(|c| (Vec2::new(c[0], c[2]), shared::heartwood_wards::WARD_FOOTPRINT_RADIUS))
        .chain(crystals_shown(realm).then_some(YARD_CRYSTAL_SPOTS).into_iter().flatten().map(|s| (s.xz(), YARD_CRYSTAL_R)))
}

/// CARD CONTACT-RUMBLE-1 — ruling 7059724016: fire at a gap at or under ε_in, re-arm only above ε_out (metres).
const CONTACT_IN: f32 = 0.01; const CONTACT_OUT: f32 = 0.10;
/// CARD CONTACT-RUMBLE-1 — a static solid within `eps` of the body disk: trunk, owned stump or ward post disk, or a
/// shown yard rock (its own push-out shape and height gate, probed `eps` toward the rock spot).
fn static_within(pos: Vec3, realm: Option<u8>, discs: &[(Vec2, f32)], eps: f32) -> bool {
    use crate::living_ecology::{yard_rocks_shown, YARD_ROCK_SPOTS};
    discs.iter().any(|&(c, r)| (pos.xz() - c).length() <= r + PLAYER_TRUNK_RADIUS + eps) || yard_rocks_shown(realm).then_some(YARD_ROCK_SPOTS).into_iter().flatten().any(|spot| {
        let (probe, mut v) = (pos - (pos - spot).with_y(0.0).normalize_or_zero() * eps, Vec3::ZERO);
        let mut t = probe;
        push_out_of_rock(&mut t, &mut v, spot, 0.9, crate::human_presence::silhouette().feet);
        t != probe
    })
}
/// CARD CONTACT-RUMBLE-1 — one existing harvest-light rumble on the fixed tick the body reaches a static solid,
/// grounded or airborne. A move outside the fixed tick (teleport, respawn) or a place change re-seats without firing.
#[allow(clippy::type_complexity)]
fn rumble_static_contact(presence: Res<crate::human_presence::SoftPresence>, feel: Option<Res<crate::local_settings::LocalFeedbackFeel>>, pads: Query<Entity, With<Gamepad>>, mut rumble: MessageWriter<bevy::input::gamepad::GamepadRumbleRequest>, trees: (Option<Res<crate::living_practice_loop::SoftPlayerRealm>>, Option<Res<crate::vegetation::FieldTrunks>>, Option<Res<crate::wood_expedition::WoodHub>>, Option<Res<crate::hex_travel::HexTravelState>>), mut seat: Local<Option<(bool, Vec3, Option<shared::hex_travel::PlaceId>, Option<u8>)>>) {
    use crate::living_ecology::{yard_trees_shown, YARD_TREE_SPOTS, YARD_TRUNK_R};
    let (realm, place, at) = (trees.0.as_ref().and_then(|r| r.current), trees.3.as_ref().map(|t| t.current), presence.position);
    let yard = yard_trees_shown(realm).then_some(YARD_TREE_SPOTS).into_iter().flatten().map(|s| (s.xz(), YARD_TRUNK_R));
    let owned = crate::wood_expedition::owned_blocker(trees.2.as_deref(), place, realm);
    let discs: Vec<_> = yard.chain(trees.1.iter().flat_map(|f| f.0.iter().copied())).chain(owned).chain(prop_solids(place, realm)).collect();
    let near = |eps| static_within(at, realm, &discs, eps);
    let armed = match *seat { Some((armed, last, p, r)) if (p, r) == (place, realm) && presence.prev_tick.is_some_and(|t| t.xz().distance(last.xz()) <= CONTACT_IN) => armed, _ => !near(CONTACT_IN) };
    let fire = armed && near(CONTACT_IN);
    *seat = Some((!fire && (armed || !near(CONTACT_OUT)), at, place, realm));
    if fire && feel.is_some_and(|f| f.rumble_enabled) { crate::harvest_feel::rumble_harvest(&mut rumble, pads.iter(), false); }
}

#[derive(Component, Clone, Copy)]
pub(crate) struct BodyCollider { pub r: f32 }
#[cfg(test)]
use std::sync::atomic::{AtomicU8, Ordering};
#[cfg(test)]
pub(crate) static TEST_CREATURE_SKIP: AtomicU8 = AtomicU8::new(0);
fn squeeze_depth(pos: Vec3, realm: Option<u8>, statics: &[(Vec2, f32)]) -> f32 {
    use crate::living_ecology::{yard_rocks_shown, YARD_ROCK_SPOTS};
    let mut d = 0.0f32;
    for &(c, r) in statics {
        d = d.max(r + PLAYER_TRUNK_RADIUS - (pos.xz() - c).length());
    }
    if yard_rocks_shown(realm) {
        let feet = crate::human_presence::silhouette().feet;
        for spot in YARD_ROCK_SPOTS {
            let (mut t, mut v) = (pos, Vec3::ZERO);
            push_out_of_rock(&mut t, &mut v, spot, 0.9, feet);
            d = d.max((t.xz() - pos.xz()).length());
        }
    }
    d
}
pub struct CreatureBodyPlugin;
impl Plugin for CreatureBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<bevy::input::gamepad::GamepadRumbleRequest>().add_systems(Update, attach_body_colliders).add_systems(FixedPostUpdate, (push_player_from_creature_bodies, rumble_static_contact).chain()).add_systems(PostUpdate, clamp_creature_stop_at_contact);
    }
}
fn attach_body_colliders(mut c: Commands, q: Query<(Entity, &Name, Option<&crate::grazer::Grazer>, Option<&crate::combat_core::Predator>, Option<&crate::droid::Droid>), Without<BodyCollider>>) {
    for (e, name, g, p, d) in &q {
        let r = if name.as_str() == "ResonantDeer" { 0.16 } else if matches!(name.as_str(), "Mira" | "Ko" | "Ren") { 0.22 } else if g.is_some() { 0.5 } else if p.is_some() || d.is_some() { 0.35 } else { continue };
        c.entity(e).insert(BodyCollider { r });
    }
}
#[allow(clippy::type_complexity)]
fn push_player_from_creature_bodies(input: Res<crate::input::PlayerInput>, mut presence: ResMut<crate::human_presence::SoftPresence>, bond: Option<Res<crate::companion_bond::CompanionBond>>, bodies: Query<(&BodyCollider, &Transform, &InheritedVisibility, Option<&Name>)>, trees: (Option<Res<crate::living_practice_loop::SoftPlayerRealm>>, Option<Res<crate::vegetation::FieldTrunks>>, Option<Res<crate::wood_expedition::WoodHub>>, Option<Res<crate::hex_travel::HexTravelState>>)) {
    #[cfg(test)]
    if TEST_CREATURE_SKIP.load(Ordering::Relaxed) & 1 != 0 { return; }
    let driven = input.movement.length_squared() > 1e-8 || Vec2::new(presence.velocity.x, presence.velocity.z).length_squared() > 1e-8 || presence.prev_tick.zip(presence.curr_tick).is_some_and(|(a, b)| (a.xz() - b.xz()).length_squared() > 1e-8);
    if !driven { return; }
    let discs: Vec<_> = bodies.iter().filter(|(_, _, v, n)| v.get() && !bond.as_deref().is_some_and(|b| b.mounted && n.is_some_and(|n| n.as_str() == "ResonantDeer"))).map(|(b, t, _, _)| (t.translation.xz(), b.r)).collect();
    if discs.is_empty() { return; }
    use crate::living_ecology::{yard_trees_shown, YARD_TREE_SPOTS, YARD_TRUNK_R};
    let realm = trees.0.as_ref().and_then(|r| r.current);
    let yard = yard_trees_shown(realm).then_some(YARD_TREE_SPOTS).into_iter().flatten().map(|s| (s.xz(), YARD_TRUNK_R));
    let field = trees.1.iter().flat_map(|f| f.0.iter().copied());
    let owned = crate::wood_expedition::owned_blocker(trees.2.as_deref(), trees.3.as_ref().map(|t| t.current), realm);
    let statics: Vec<_> = yard.chain(field).chain(owned).chain(prop_solids(trees.3.as_ref().map(|t| t.current), realm)).collect();
    let before = squeeze_depth(presence.position, realm, &statics);
    let y0 = presence.position.y;
    let (mut pos, mut vel) = (presence.position, presence.velocity);
    push_out_clear(&mut pos, &mut vel, discs);
    #[cfg(test)]
    let noguard = TEST_CREATURE_SKIP.load(Ordering::Relaxed) & 2 != 0;
    #[cfg(not(test))]
    let noguard = false;
    if !noguard && (squeeze_depth(pos, realm, &statics) > before + 1e-5 || {
        let b = crate::terrain::SHELF_OUTER[crate::terrain::SHELF_COUNT - 1] - PLAYER_TRUNK_RADIUS;
        presence.position.xz().length() <= b && pos.xz().length() > b + 1e-5
    }) { return; }
    presence.position = pos;
    presence.position.y = y0;
    presence.velocity.x = vel.x;
    presence.velocity.z = vel.z;
}
fn clamp_creature_stop_at_contact(presence: Option<Res<crate::human_presence::SoftPresence>>, bond: Option<Res<crate::companion_bond::CompanionBond>>, mut q: Query<(&mut Transform, &BodyCollider, &InheritedVisibility, Option<&Name>), Without<crate::grazer::Grazer>>) {
    #[cfg(test)]
    if TEST_CREATURE_SKIP.load(Ordering::Relaxed) & 4 != 0 { return; }
    let Some(p) = presence.as_deref().map(|s| s.position) else { return };
    for (mut tf, body, vis, name) in &mut q {
        if !vis.get() || bond.as_deref().is_some_and(|b| b.mounted && name.is_some_and(|n| n.as_str() == "ResonantDeer")) { continue; } let off = tf.translation.xz() - p.xz();
        let d2 = off.length_squared();
        let reach = body.r + PLAYER_TRUNK_RADIUS;
        if d2 < reach * reach && d2 > 1e-10 {
            let on = p.xz() + off / d2.sqrt() * reach;
            (tf.translation.x, tf.translation.z) = (on.x, on.y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vegetation::{
        cleared_plants, trunk_on_plate, vegetation_plan, FieldTrunks, VegKind, TRUNK_RADIUS,
    };
    use shared::local_settings::GraphicsPreset;
    static TEST_CREATURE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// CARD COLLIDE-CORE-1 CC2 — a walk into an off-plate field trunk stops at its radius.
    #[test]
    fn collide_core_1_cc2_off_plate_trunk_stops_walk() {
        use bevy::asset::AssetPlugin;
        use bevy::prelude::*;
        use shared::local_settings::LocalSettings;
        use crate::painterly_shader::PainterlyMaterial;
        let plant = cleared_plants(GraphicsPreset::High, None)
            .unwrap()
            .into_iter()
            .find(|p| {
                p.kind == VegKind::Tree
                    && !trunk_on_plate(p.x, p.z, p.scale)
                    && {
                        let r = Vec2::new(p.x, p.z).length();
                        r > crate::terrain::COURT_RADIUS
                            && r < vegetation_plan(GraphicsPreset::High).reach
                    }
            })
            .expect("walkable off-plate trunk");
        let center = Vec2::new(plant.x, plant.z);
        let trunk_r = TRUNK_RADIUS * plant.scale;
        let reach = trunk_r + PLAYER_TRUNK_RADIUS;
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Mesh>()
            .init_asset::<StandardMaterial>()
            .init_asset::<PainterlyMaterial>()
            .add_plugins(crate::vegetation::VegetationPlugin)
            .insert_resource(crate::living_practice_loop::SoftPlayerRealm { current: Some(0) });
        let mut inner = LocalSettings::default();
        inner.set_graphics_preset(GraphicsPreset::High);
        app.insert_resource(crate::local_settings::LocalSettingsState { inner, dirty: false });
        app.update();
        let trunks = app.world().resource::<FieldTrunks>().0.clone();
        assert!(trunks.iter().any(|(c, r)| c.distance(center) < 1e-4 && (*r - trunk_r).abs() < 1e-5));
        let mut pos = Vec3::new(center.x, 0.9, center.y - reach - 1.5);
        (0..180).for_each(|_| {
            pos.z += 0.08;
            push_out_of_trunks(&mut pos, trunks.iter().copied());
        });
        assert!((pos.xz().distance(center) - reach).abs() < 1e-3 && pos.z < center.y + 1e-3, "walk off-plate trunk {pos}");
    }
    fn body_app() -> App {
        use crate::human_presence::SoftPresence;
        use crate::input::PlayerInput;
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, CreatureBodyPlugin))
            .insert_resource(SoftPresence { position: Vec3::new(0.0, 0.9, 0.0), velocity: Vec3::new(0.0, 0.0, 0.5), grounded: true, ..default() })
            .insert_resource(PlayerInput { movement: Vec2::new(0.0, 1.0), ..default() })
            .insert_resource(crate::living_practice_loop::SoftPlayerRealm { current: Some(3) });
        app
    }

    fn push_walk(app: &mut App, body_z: f32, r: f32, inward: bool) {
        let reach = r + PLAYER_TRUNK_RADIUS;
        let mut p = app.world_mut().resource_mut::<crate::human_presence::SoftPresence>();
        p.position = Vec3::new(0.0, 0.9, if inward { body_z + 0.5 } else { body_z - reach - 0.35 });
        let (step, n, vz) = if inward { (-0.04f32, 48usize, -0.5f32) } else { (0.08, 32, 0.5) };
        for _ in 0..n { let (prev, nxt) = (p.position, p.position + Vec3::new(0., 0., step)); (p.position, p.velocity, p.prev_tick, p.curr_tick) = (nxt, Vec3::new(0., 0., vz), Some(prev), Some(nxt)); drop(p); app.world_mut().run_schedule(FixedPostUpdate); p = app.world_mut().resource_mut::<crate::human_presence::SoftPresence>(); }
    }

    #[test]
    fn creature_body_fixture_1_walk_visible_bodies_contact() {
        let _g = TEST_CREATURE_LOCK.lock().unwrap(); TEST_CREATURE_SKIP.store(0, Ordering::Relaxed);
        for r in [0.16f32, 0.22f32, 0.35, 0.35, 0.5] {
            let z = r + PLAYER_TRUNK_RADIUS;
            let mut app = body_app();
            app.world_mut().spawn((BodyCollider { r }, Transform::from_xyz(0.0, 0.55, z), InheritedVisibility::VISIBLE, Visibility::Visible));
            push_walk(&mut app, z, r, false);
            let p = app.world().resource::<crate::human_presence::SoftPresence>();
            assert!((p.position.xz().distance(Vec2::new(0.0, z)) - z).abs() < 2e-3 && p.velocity.z <= 1e-3 && (p.position.y - 0.9).abs() < 1e-6, "r={r} pos={}", p.position);
        }
        let bz = crate::terrain::SHELF_OUTER[crate::terrain::SHELF_COUNT - 1] - PLAYER_TRUNK_RADIUS + 40.0;
        let mut app = body_app();
        app.world_mut().spawn((BodyCollider { r: 0.16 }, Transform::from_xyz(0.0, 0.55, bz), InheritedVisibility::VISIBLE, Visibility::Visible));
        push_walk(&mut app, bz, 0.16, true);
        let p = app.world().resource::<crate::human_presence::SoftPresence>();
        assert!((p.position.xz().distance(Vec2::new(0.0, bz)) - 0.46).abs() < 2e-3 && p.velocity.z <= 1e-3 && (p.position.y - 0.9).abs() < 1e-6, "field pos={}", p.position);
        TEST_CREATURE_SKIP.store(1, Ordering::Relaxed);
        let mut app = body_app();
        app.world_mut().spawn((BodyCollider { r: 0.16 }, Transform::from_xyz(0.0, 0.55, 0.46), InheritedVisibility::VISIBLE, Visibility::Visible));
        push_walk(&mut app, 0.46, 0.16, false);
        assert!((app.world().resource::<crate::human_presence::SoftPresence>().position.xz().distance(Vec2::new(0.0, 0.46)) - 0.46).abs() > 0.05);
    }

    #[test]
    fn creature_body_fixture_2_hidden_body_no_block() {
        let _g = TEST_CREATURE_LOCK.lock().unwrap(); TEST_CREATURE_SKIP.store(0, Ordering::Relaxed);
        let mut app = body_app();
        app.world_mut().spawn((BodyCollider { r: 0.16 }, Transform::from_xyz(0.0, 0.55, 0.46), InheritedVisibility::VISIBLE, Visibility::Visible));
        app.world_mut().spawn((BodyCollider { r: 0.16 }, Transform::from_xyz(0.42, 0.55, 0.58), InheritedVisibility::HIDDEN, Visibility::Visible));
        push_walk(&mut app, 0.46, 0.16, false);
        assert!((app.world().resource::<crate::human_presence::SoftPresence>().position.xz().distance(Vec2::new(0.0, 0.46)) - 0.46).abs() < 1e-3);
        TEST_CREATURE_SKIP.store(1, Ordering::Relaxed);
        push_walk(&mut app, 0.46, 0.16, false);
        assert!(app.world().resource::<crate::human_presence::SoftPresence>().position.z > 0.25);
    }

    #[test]
    fn creature_body_fixture_3_nov6_stationary_player_no_bounce() {
        let _g = TEST_CREATURE_LOCK.lock().unwrap(); TEST_CREATURE_SKIP.store(0, Ordering::Relaxed);
        let mut app = body_app();
        app.world_mut().spawn((BodyCollider { r: 0.16 }, Transform::from_xyz(0.0, 0.55, 0.46), InheritedVisibility::VISIBLE, Visibility::Visible));
        let still = app.world().resource::<crate::human_presence::SoftPresence>().position;
        let id = app.world_mut().query::<(Entity, &Transform)>().iter(app.world()).next().unwrap().0;
        app.world_mut().resource_mut::<crate::input::PlayerInput>().movement = Vec2::ZERO;
        app.world_mut().resource_mut::<crate::human_presence::SoftPresence>().velocity = Vec3::ZERO;
        app.world_mut().get_mut::<Transform>(id).unwrap().translation.z = 0.18;
        app.world_mut().run_schedule(PostUpdate);
        let (p, tf) = (app.world().resource::<crate::human_presence::SoftPresence>(), app.world().get::<Transform>(id).unwrap());
        assert!(p.position == still && p.velocity == Vec3::ZERO && (tf.translation.xz().distance(p.position.xz()) - 0.46).abs() < 1e-3);
        TEST_CREATURE_SKIP.store(4, Ordering::Relaxed);
        app.world_mut().get_mut::<Transform>(id).unwrap().translation.z = 0.12;
        app.world_mut().run_schedule(PostUpdate);
        assert!(app.world().get::<Transform>(id).unwrap().translation.z < 0.15);
    }

    #[test]
    fn creature_body_fixture_6_wedge_trunk_guard() {
        let _g = TEST_CREATURE_LOCK.lock().unwrap(); TEST_CREATURE_SKIP.store(0, Ordering::Relaxed);
        use crate::living_ecology::{YARD_TREE_SPOTS, YARD_TRUNK_R};
        let trunk = YARD_TREE_SPOTS[0].xz();
        let pz = trunk.y - YARD_TRUNK_R - PLAYER_TRUNK_RADIUS - 0.04;
        let mut app = body_app();
        app.world_mut().resource_mut::<crate::living_practice_loop::SoftPlayerRealm>().current = None;
        app.world_mut().spawn((BodyCollider { r: 0.16 }, Transform::from_xyz(trunk.x, 0.55, pz - 0.16), InheritedVisibility::VISIBLE, Visibility::Visible));
        let sk = [(trunk, YARD_TRUNK_R)];
        let kick = |app: &mut App, vz: f32| {
            let mut p = app.world_mut().resource_mut::<crate::human_presence::SoftPresence>();
            (p.position, p.velocity, p.prev_tick, p.curr_tick) = (Vec3::new(trunk.x, 0.9, pz), Vec3::new(0., 0., vz), Some(Vec3::new(trunk.x, 0.9, pz)), Some(Vec3::new(trunk.x, 0.9, pz)));
        };
        kick(&mut app, -0.25);
        let before = squeeze_depth(app.world().resource::<crate::human_presence::SoftPresence>().position, None, &sk);
        app.world_mut().run_schedule(FixedPostUpdate);
        assert!(squeeze_depth(app.world().resource::<crate::human_presence::SoftPresence>().position, None, &sk) <= before + 1e-3);
        TEST_CREATURE_SKIP.store(2, Ordering::Relaxed);
        kick(&mut app, -0.35);
        app.world_mut().run_schedule(FixedPostUpdate);
        assert!(squeeze_depth(app.world().resource::<crate::human_presence::SoftPresence>().position, None, &sk) > before + 1e-4);
    }

    #[test]
    fn creature_body_fixture_7_mounted_companion_deer_skip() {
        let _g = TEST_CREATURE_LOCK.lock().unwrap(); TEST_CREATURE_SKIP.store(0, Ordering::Relaxed);
        use crate::companion_bond::CompanionBond;
        let mut app = body_app();
        app.insert_resource(CompanionBond { mounted: true, ..default() });
        let s = app.world().resource::<crate::human_presence::SoftPresence>().position;
        let seat = s + Vec3::new(0.1, 0.15, 0.0);
        let id = app.world_mut().spawn((BodyCollider { r: 0.16 }, Transform::from_translation(seat), InheritedVisibility::VISIBLE, Visibility::Visible, Name::new("ResonantDeer"))).id();
        let deer0 = app.world().get::<Transform>(id).unwrap().translation;
        app.world_mut().resource_mut::<crate::human_presence::SoftPresence>().velocity = Vec3::new(0.0, 0.0, 0.5);
        app.world_mut().run_schedule(FixedPostUpdate);
        app.world_mut().run_schedule(PostUpdate);
        assert_eq!(app.world().resource::<crate::human_presence::SoftPresence>().position, s);
        assert_eq!(app.world().get::<Transform>(id).unwrap().translation, deer0);
        app.world_mut().resource_mut::<CompanionBond>().mounted = false;
        app.world_mut().get_mut::<Transform>(id).unwrap().translation = Vec3::new(0.0, 0.55, 0.46);
        push_walk(&mut app, 0.46, 0.16, false);
        assert!((app.world().resource::<crate::human_presence::SoftPresence>().position.xz().distance(Vec2::new(0.0, 0.46)) - 0.46).abs() < 2e-3);
    }

    /// CARD COLLIDE-SPLIT-1 S2 — a deer or traveler can't squeeze the player into a Heartwood ward post.
    #[test]
    fn collide_split_1_s2_body_cannot_wedge_into_ward_post() {
        let _g = TEST_CREATURE_LOCK.lock().unwrap(); TEST_CREATURE_SKIP.store(0, Ordering::Relaxed);
        use shared::heartwood_wards::{WARD_FOOTPRINT_RADIUS, WARD_POST_CENTERS};
        for (w, r) in WARD_POST_CENTERS.iter().flat_map(|w| [(w, 0.16f32), (w, 0.22)]) {
            let post = Vec2::new(w[0], w[2]);
            let pz = post.y - WARD_FOOTPRINT_RADIUS - PLAYER_TRUNK_RADIUS - 0.04;
            let mut app = body_app();
            app.insert_resource(crate::hex_travel::HexTravelState { current: shared::hex_travel::PlaceId::Heartwood });
            app.world_mut().spawn((BodyCollider { r }, Transform::from_xyz(post.x, 0.55, pz - r), InheritedVisibility::VISIBLE, Visibility::Visible));
            let (at, sk) = (Vec3::new(post.x, 0.9, pz), [(post, WARD_FOOTPRINT_RADIUS)]);
            let mut p = app.world_mut().resource_mut::<crate::human_presence::SoftPresence>();
            (p.position, p.velocity, p.prev_tick, p.curr_tick) = (at, Vec3::new(0., 0., -0.25), Some(at), Some(at));
            let before = squeeze_depth(at, Some(3), &sk);
            app.world_mut().run_schedule(FixedPostUpdate);
            let after = squeeze_depth(app.world().resource::<crate::human_presence::SoftPresence>().position, Some(3), &sk);
            assert!(after <= before + 1e-3, "post {post} r={r}: squeezed {before} -> {after}");
        }
    }


    // CARD CONTACT-RUMBLE-1 — body_app + feel + one pad. `cr_to` moves the body one fixed tick through the static push-outs
    // apply_locomotion runs (yard trunks, shown rocks, field + ward posts), runs FixedPostUpdate, and counts the Adds written.
    use bevy::input::gamepad::{GamepadRumbleIntensity as Gi, GamepadRumbleRequest as Rq};
    use shared::{hex_travel::PlaceId, local_settings::LocalSettings};
    fn cr_app(place: Option<PlaceId>, realm: Option<u8>, feel: Option<LocalSettings>, pad: bool) -> App {
        let mut app = body_app();
        app.add_message::<Rq>().insert_resource(crate::living_practice_loop::SoftPlayerRealm { current: realm });
        if let Some(current) = place { app.insert_resource(crate::hex_travel::HexTravelState { current }); }
        if let Some(s) = feel { app.insert_resource(crate::local_settings::LocalFeedbackFeel::from_settings(&s)); }
        if pad { app.world_mut().spawn(Gamepad::default()); } app
    }
    fn cr_seat(app: &mut App, at: Vec3) { let mut p = app.world_mut().resource_mut::<crate::human_presence::SoftPresence>(); (p.position, p.prev_tick, p.curr_tick) = (at, Some(at), Some(at)); }
    fn cr_to(app: &mut App, to: Vec3) -> usize {
        use crate::living_ecology::{yard_rocks_shown, yard_trees_shown, YARD_ROCK_SPOTS, YARD_TREE_SPOTS, YARD_TRUNK_R};
        let (realm, place) = (app.world().resource::<crate::living_practice_loop::SoftPlayerRealm>().current, app.world().get_resource::<crate::hex_travel::HexTravelState>().map(|t| t.current));
        let field = app.world().get_resource::<FieldTrunks>().map(|f| f.0.clone()).unwrap_or_default();
        let mut p = app.world_mut().resource_mut::<crate::human_presence::SoftPresence>();
        let (prev, mut pos, mut vel) = (p.position, to, (to - p.position) * 60.0);
        push_out_clear(&mut pos, &mut vel, yard_trees_shown(realm).then_some(YARD_TREE_SPOTS).into_iter().flatten().map(|s| (s.xz(), YARD_TRUNK_R)));
        for spot in yard_rocks_shown(realm).then_some(YARD_ROCK_SPOTS).into_iter().flatten() { push_out_of_rock(&mut pos, &mut vel, spot, 0.9, crate::human_presence::silhouette().feet); }
        push_out_clear(&mut pos, &mut vel, field.into_iter().chain(prop_solids(place, realm)));
        (p.position, p.velocity, p.prev_tick, p.curr_tick) = (pos, vel, Some(prev), Some(pos));
        drop(p); app.world_mut().run_schedule(FixedPostUpdate);
        app.world_mut().resource_mut::<Messages<Rq>>().drain().map(|q| match q { Rq::Add { intensity: Gi { strong_motor, weak_motor }, duration, .. } => assert_eq!((strong_motor, weak_motor, duration.as_millis()), (0.12, 0.28, 90)), Rq::Stop { .. } => panic!("contact never stops a pad") }).count()
    }
    fn cr_walk(app: &mut App, dir: Vec2, ticks: usize) -> usize { (0..ticks).map(|_| { let p = app.world().resource::<crate::human_presence::SoftPresence>().position; cr_to(app, p + Vec3::new(dir.x, 0.0, dir.y).normalize() * 0.06) }).sum() }
    /// (place, realm, start, walk dir): Heartwood ward post, yard trunk (-5.2, -2.4), yard rock (7.0, 0.4), all real spots.
    const CR_POST: (Option<PlaceId>, Option<u8>, Vec3, Vec2) = (Some(PlaceId::Heartwood), Some(2), Vec3::new(8.0, 0.9, 5.5), Vec2::NEG_Y);
    const CR_SOLIDS: [(Option<PlaceId>, Option<u8>, Vec3, Vec2); 3] = [CR_POST, (None, Some(0), Vec3::new(-5.2, 0.9, -0.9), Vec2::NEG_Y), (None, Some(1), Vec3::new(7.0, 0.9, -1.4), Vec2::Y)];
    fn cr_run(case: (Option<PlaceId>, Option<u8>, Vec3, Vec2), feel: Option<LocalSettings>, pad: bool, ticks: usize) -> (App, usize) {
        let mut app = cr_app(case.0, case.1, feel, pad); cr_seat(&mut app, case.2); let n = cr_walk(&mut app, case.3, ticks); (app, n)
    }
    fn cr_gap(app: &App) -> bool { let p = app.world().resource::<crate::human_presence::SoftPresence>().position.xz(); prop_solids(Some(PlaceId::Heartwood), Some(2)).any(|(c, r)| p.distance(c) <= r + PLAYER_TRUNK_RADIUS + 0.01) }

    #[test]
    fn contact_rumble_1_f1_walk_into_post_trunk_rock_one_add_each() {
        for (i, case) in CR_SOLIDS.into_iter().chain([(CR_POST.0, CR_POST.1, CR_POST.2 + Vec3::Y, CR_POST.3)]).enumerate() {
            let (app, n) = cr_run(case, Some(LocalSettings::peace_defaults()), true, 40);
            assert!(n == 1 && (i == 1 || i == 2 || cr_gap(&app)), "solid {i} (3 = airborne post): {n} Adds; one on the contact tick");
        }
    }
    /// CARD CRYSTAL-SOLID-1 — R4: walking into the shown spire (-7.2, 2.2) is one existing Add, and nothing with rumble off.
    #[test]
    fn crystal_solid_1_r4_spire_contact_one_add_silent_with_rumble_off() {
        let run = |realm, rumble| cr_run((None, Some(realm), Vec3::new(-5.2, 0.9, 2.2), Vec2::NEG_X), Some(LocalSettings { rumble, ..LocalSettings::peace_defaults() }), true, 40).1;
        assert_eq!([1, 4].map(|r| (run(r, true), run(r, false))), [(1, 0); 2], "realm 1 / 4: one Add on the contact tick, none with rumble off");
    }
    #[test]
    fn contact_rumble_1_f2_hold_and_slide_along_stays_one() {
        let (mut app, _) = cr_run(CR_POST, Some(LocalSettings::peace_defaults()), true, 40); let x0 = app.world().resource::<crate::human_presence::SoftPresence>().position.x;
        assert_eq!(cr_walk(&mut app, Vec2::NEG_Y, 30) + cr_walk(&mut app, Vec2::new(0.15, -1.0), 20), 0, "no Add after the contact tick");
        assert!(cr_gap(&app) && app.world().resource::<crate::human_presence::SoftPresence>().position.x - x0 > 0.4, "slid along the post, still in contact");
    }
    #[test]
    fn contact_rumble_1_f3_back_off_past_eps_out_and_reenter_is_two() {
        let (mut app, n) = cr_run(CR_POST, Some(LocalSettings::peace_defaults()), true, 40); let at = app.world().resource::<crate::human_presence::SoftPresence>().position;
        assert_eq!(n + cr_to(&mut app, at + Vec3::Z * 0.09) + cr_to(&mut app, at), 1, "0.09 m stays inside ε_out");
        assert_eq!(cr_walk(&mut app, Vec2::Y, 5) + cr_walk(&mut app, Vec2::NEG_Y, 20), 1, "back off past ε_out, then re-enter");
    }
    #[test]
    fn contact_rumble_1_f4_rumble_off_reduced_motion_low_write_nothing() {
        let offs = [LocalSettings { rumble: false, ..LocalSettings::peace_defaults() }, LocalSettings { reduced_motion: true, ..LocalSettings::peace_defaults() }, { let mut s = LocalSettings::peace_defaults(); s.set_graphics_preset(GraphicsPreset::Low); s }];
        for (i, s) in offs.into_iter().enumerate() {
            assert!(!crate::local_settings::LocalFeedbackFeel::from_settings(&s).rumble_enabled);
            assert_eq!(CR_SOLIDS.into_iter().map(|c| cr_run(c, Some(s.clone()), true, 40).1).sum::<usize>(), 0, "case {i} without LocalSettingsPlugin");
            let (_env, dir) = (crate::test_env::lock(), std::env::temp_dir().join(format!("powrush-cr1-f4-{i}-{}", std::process::id())));
            std::fs::create_dir_all(&dir).unwrap(); std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &dir);
            let mut app = cr_app(CR_POST.0, CR_POST.1, None, true);
            app.insert_resource(crate::lived_hour_bind::LivedHourBind::default()).insert_resource(crate::title_screen::HouseLabel { house: shared::house_name::HouseName::default(), persist_present: false, hour_two_held: false, book_held: false, settings_open: false, draft: String::new(), naming_offered: false, seals_offered: false }).add_plugins((crate::local_settings::LocalSettingsPlugin, crate::hud_presets::HudLayoutPlugin));
            app.update(); app.world_mut().resource_mut::<crate::local_settings::LocalSettingsState>().inner = s; app.update();
            app.world_mut().resource_mut::<Messages<Rq>>().clear(); // the global suppressor's edge Stop (RUMBLE-OFF-1)
            std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV); let _ = std::fs::remove_dir_all(&dir); cr_seat(&mut app, CR_POST.2);
            assert_eq!(cr_walk(&mut app, CR_POST.3, 40), 0, "case {i} with LocalSettingsPlugin");
        }
    }
    #[test]
    fn contact_rumble_1_f5_no_pad_writes_nothing() {
        assert!(CR_SOLIDS.into_iter().all(|c| cr_run(c, Some(LocalSettings::peace_defaults()), false, 40).1 == 0));
    }
    #[test]
    fn contact_rumble_1_f6_trace_same_with_rumble_on_and_off() {
        let trace = |feel: Option<LocalSettings>, pad| CR_SOLIDS.map(|c| { let (mut app, _) = cr_run(c, feel.clone(), pad, 30); (0..30).map(|_| { cr_walk(&mut app, Vec2::new(0.3, -1.0), 1); let p = app.world().resource::<crate::human_presence::SoftPresence>(); (p.position, p.velocity) }).collect::<Vec<_>>() });
        assert_eq!(trace(Some(LocalSettings::peace_defaults()), true), trace(None, false));
    }
    #[test]
    fn contact_rumble_1_f7_creature_bodies_never_rumble() {
        let _g = TEST_CREATURE_LOCK.lock().unwrap(); TEST_CREATURE_SKIP.store(0, Ordering::Relaxed);
        for (name, r, mounted) in [("Grazer", 0.5, false), ("ResonantDeer", 0.16, false), ("ResonantDeer", 0.16, true), ("Mira", 0.22, false)] {
            let mut app = cr_app(None, Some(3), Some(LocalSettings::peace_defaults()), true);
            if mounted { app.insert_resource(crate::companion_bond::CompanionBond { mounted: true, ..default() }); }
            app.world_mut().spawn((BodyCollider { r }, Name::new(name), Transform::from_xyz(0.0, 0.55, 0.0), InheritedVisibility::VISIBLE, Visibility::Visible)); cr_seat(&mut app, Vec3::new(0.0, 0.9, -1.2));
            assert_eq!(cr_walk(&mut app, Vec2::Y, 40), 0, "{name} r={r} mounted={mounted}");
        }
    }
    #[test]
    fn contact_rumble_1_f8_terrain_bounds_never_rumble() {
        use crate::{terrain::{SHELF_COUNT, SHELF_OUTER}, water::{BATH_INNER, BATH_OUTER}};
        for (r, realm) in [(BATH_INNER - PLAYER_TRUNK_RADIUS, None), (BATH_OUTER + PLAYER_TRUNK_RADIUS, None), (SHELF_OUTER[0] + PLAYER_TRUNK_RADIUS, Some(0)), (SHELF_OUTER[SHELF_COUNT - 1] - PLAYER_TRUNK_RADIUS, None)] {
            let mut app = cr_app(None, realm, Some(LocalSettings::peace_defaults()), true); cr_seat(&mut app, Vec3::new(0.0, 0.9, r - 0.5));
            assert_eq!(cr_walk(&mut app, Vec2::Y, 20), 0, "bound at r={r}");
        }
    }
    #[test]
    fn contact_rumble_1_f9_teleport_place_change_sanctuary_and_adjacent() {
        let (mut app, _) = cr_run(CR_POST, Some(LocalSettings::peace_defaults()), true, 40); let at = app.world().resource::<crate::human_presence::SoftPresence>().position;
        cr_walk(&mut app, Vec2::Y, 20); cr_seat(&mut app, at);
        assert_eq!(cr_walk(&mut app, Vec2::NEG_Y, 10), 0, "a teleport landing in contact re-seats without firing");
        app.insert_resource(crate::hex_travel::HexTravelState { current: PlaceId::Sanctuary });
        assert_eq!(cr_walk(&mut app, Vec2::NEG_Y, 30), 0, "ward gate off at Sanctuary");
        cr_seat(&mut app, at); app.insert_resource(crate::hex_travel::HexTravelState { current: PlaceId::Heartwood });
        assert_eq!(cr_walk(&mut app, Vec2::NEG_Y, 10), 0, "a place change in contact re-seats without firing");
        let mut app = cr_app(None, Some(3), Some(LocalSettings::peace_defaults()), true);
        app.insert_resource(FieldTrunks(vec![(Vec2::new(0.0, 0.0), 0.3), (Vec2::new(1.0, 0.0), 0.3)])); cr_seat(&mut app, Vec3::new(-0.2, 0.9, 1.5));
        cr_walk(&mut app, Vec2::NEG_Y, 30);
        assert_eq!(cr_walk(&mut app, Vec2::new(1.0, -0.4), 20), 0, "trunk to the touching trunk stays one contact");
    }
    #[test]
    fn contact_rumble_1_f10_read_only_one_existing_rumble_call() {
        let col = include_str!("collide.rs").split("\n#[cfg(test)]\nmod tests {").next().unwrap();
        let sys = col.find("fn rumble_static_contact(").map_or("", |a| &col[a..col.find("pub(crate) struct BodyCollider").unwrap()]);
        assert!(sys.is_empty() || (sys.contains("presence: Res<crate::human_presence::SoftPresence>") && !sys.contains("ResMut") && !sys.contains("grounded")));
        assert!(col.matches("rumble_harvest(").count() == usize::from(!sys.is_empty()) && !col.contains("GamepadRumbleIntensity") && !col.contains("from_millis"));
        let feel = include_str!("harvest_feel.rs").split("\n#[cfg(test)]").next().unwrap();
        assert!(feel.contains("(0.28, 0.52, 160)") && feel.contains("(0.12, 0.28, 90)") && feel.matches("GamepadRumbleRequest::Add").count() == 1);
    }
}
