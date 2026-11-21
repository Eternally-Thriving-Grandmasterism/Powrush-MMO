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
pub(crate) fn prop_solids(place: Option<shared::hex_travel::PlaceId>) -> impl Iterator<Item = (Vec2, f32)> {
    use shared::{heartwood_wards::WARD_POST_CENTERS, hex_travel::PlaceId};
    (place == Some(PlaceId::Heartwood)).then_some(WARD_POST_CENTERS).into_iter().flatten()
        .map(|c| (Vec2::new(c[0], c[2]), shared::heartwood_wards::WARD_FOOTPRINT_RADIUS))
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
        app.add_systems(Update, attach_body_colliders).add_systems(FixedPostUpdate, push_player_from_creature_bodies).add_systems(PostUpdate, clamp_creature_stop_at_contact);
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
    let statics: Vec<_> = yard.chain(field).chain(owned).chain(prop_solids(trees.3.as_ref().map(|t| t.current))).collect();
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

}
