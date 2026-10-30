//! ENEMY-GRAZER-1 — one Heartwood Grazer that flees and never fights (COMBAT_AND_PVP.md:477).
//! Client-local, never saved. Cannot be struck, deals 0, pays 0.

use bevy::prelude::*;
use crate::combat_core::{predator_present, PREDATOR_HOME, PREDATOR_NAME, REACH};
use crate::{hex_travel::HexTravelState, human_presence::SoftPresence, terrain::PLAY_HALF, title_screen::LaunchDoor};

/// Teeth band, far from the predator and clear of lamp, pond, Wards and threshold node.
pub const GRAZER_HOME: [f32; 2] = [-12.0, 14.0];
/// Playtest-tunable. Sherif may veto.
pub const FLEE_RADIUS: f32 = 6.0;
pub const FLEE_SPEED: f32 = 3.0;
/// Extra distance past `REACH` the grazer keeps from `PREDATOR_HOME`, so it never looks struck.
pub const RING_MARGIN: f32 = 0.5;
pub const EDGE_MARGIN: f32 = 1.0;
pub const GRAZER_COLOR: Color = Color::srgb(0.62, 0.66, 0.44);
const FALLBACK_DIR: Vec2 = Vec2::X;

// The predator sits 18 m from centre, so the ring plus edge margin must fit before the edge.
const _: () = assert!(REACH + RING_MARGIN + EDGE_MARGIN < PLAY_HALF - 18.0);

#[derive(Component)]
pub struct Grazer;

fn dir_or_fallback(v: Vec2) -> Vec2 {
    Some(v.normalize_or_zero()).filter(|d| *d != Vec2::ZERO).unwrap_or(FALLBACK_DIR)
}

/// Within `FLEE_RADIUS` of the player, move directly away at `FLEE_SPEED`; otherwise stay.
pub fn flee_step(grazer: Vec2, player: Vec2, dt: f32) -> Vec2 {
    let near = dt > 0.0 && grazer.distance(player) <= FLEE_RADIUS;
    if near { grazer + dir_or_fallback(grazer - player) * FLEE_SPEED * dt } else { grazer }
}

/// Inside `REACH + RING_MARGIN` of `PREDATOR_HOME`, push radially out to that ring.
pub fn push_out(p: Vec2) -> Vec2 {
    let (home, ring) = (Vec2::from_array(PREDATOR_HOME), REACH + RING_MARGIN);
    if p.distance(home) >= ring { p } else { home + dir_or_fallback(p - home) * ring }
}

pub fn clamp_plate(p: Vec2) -> Vec2 {
    p.clamp(Vec2::splat(EDGE_MARGIN - PLAY_HALF), Vec2::splat(PLAY_HALF - EDGE_MARGIN))
}

/// One frame: flee, then predator push-out, then edge clamp. Pure.
pub fn grazer_step(grazer: Vec2, player: Vec2, dt: f32) -> Vec2 {
    clamp_plate(push_out(flee_step(grazer, player, dt)))
}

pub struct GrazerPlugin;

impl Plugin for GrazerPlugin {
    fn build(&self, app: &mut App) { app.add_systems(Startup, spawn_grazer).add_systems(Update, grazer_flee); }
}

fn spawn_grazer(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>, mut mats: ResMut<Assets<StandardMaterial>>) {
    cmd.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(mats.add(GRAZER_COLOR)),
        Transform::from_xyz(GRAZER_HOME[0], 0.8, GRAZER_HOME[1]),
        (Visibility::Hidden, Grazer, Name::new(PREDATOR_NAME)),
    ));
}

fn grazer_flee(
    time: Res<Time>,
    door: Option<Res<LaunchDoor>>,
    travel: Option<Res<HexTravelState>>,
    presence: Option<Res<SoftPresence>>,
    mut grazers: Query<(&mut Transform, &mut Visibility), With<Grazer>>,
) {
    let Some(travel) = travel else { return };
    let (show, in_yard) = (predator_present(travel.current), door.as_deref().copied() == Some(LaunchDoor::InYard));
    for (mut tf, mut vis) in &mut grazers {
        vis.set_if_neq(if show { Visibility::Visible } else { Visibility::Hidden });
        let (true, true, Some(p)) = (show, in_yard, presence.as_deref()) else { continue };
        let n = grazer_step(tf.translation.xz(), p.position.xz(), time.delta_secs());
        (tf.translation.x, tf.translation.z) = (n.x, n.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat_core::{in_reach, CombatState};
    use shared::heartwood_lamp::{LAMP_DISK_CENTER, LAMP_DISK_RADIUS, WATER_POND_CENTER, WATER_POND_RADIUS};
    use shared::{heartwood_wards::{WARD_POST_CENTERS, WARD_USE_RADIUS}, hex_travel::PlaceId};
    use shared::threshold_shelf::{THRESHOLD_NODE_CENTER, THRESHOLD_SHELF_USE_RADIUS};

    fn safe(p: Vec2) -> bool {
        !in_reach(PlaceId::Heartwood, &CombatState::default(), p.x, p.y)
    }

    #[test]
    fn flee_in_out_of_range_zero_distance_and_dt_zero() {
        let g = Vec2::new(2.0, 0.0);
        assert!((flee_step(g, Vec2::ZERO, 0.5) - Vec2::new(2.0 + FLEE_SPEED * 0.5, 0.0)).length() < 1e-5);
        assert_eq!(flee_step(Vec2::new(FLEE_RADIUS + 1.0, 0.0), Vec2::ZERO, 0.5).x, FLEE_RADIUS + 1.0);
        assert!(flee_step(g, g, 0.5).is_finite() && flee_step(g, g, 0.5) != g);
        assert_eq!(flee_step(g, Vec2::ZERO, 0.0), g);
    }

    #[test]
    fn corners_clamp_inside_plate() {
        let h = PLAY_HALF - EDGE_MARGIN;
        for s in [Vec2::new(1., 1.), Vec2::new(-1., 1.), Vec2::new(1., -1.), Vec2::new(-1., -1.)] {
            let mut g = s * (h - 0.2);
            for _ in 0..200 {
                g = grazer_step(g, g - s, 0.1);
                assert!(g.x.abs() <= h && g.y.abs() <= h && g.is_finite());
            }
            assert!((g - s * h).length() < 1e-4, "pinned in the corner");
        }
    }

    #[test]
    fn push_out_leaves_reach_and_center_has_no_nan() {
        let home = Vec2::from_array(PREDATOR_HOME);
        let p = push_out(home + Vec2::new(0.5, -0.3));
        assert!(safe(p) && (p.distance(home) - (REACH + RING_MARGIN)).abs() < 1e-4);
        assert!(push_out(home).is_finite() && safe(push_out(home)));
    }

    #[test]
    fn ring_sits_inside_clamp_box() {
        let (home, r, h) = (Vec2::from_array(PREDATOR_HOME), REACH + RING_MARGIN, PLAY_HALF - EDGE_MARGIN);
        assert!(home.x.abs() + r < h && home.y.abs() + r < h);
        let g = grazer_step(home + Vec2::new(-0.1, 0.0), home + Vec2::new(1.0, 0.0), 0.1);
        assert!(safe(g) && g.x.abs() <= h && g.y.abs() <= h);
    }

    #[test]
    fn pinned_against_ring_settles_without_jitter() {
        let home = Vec2::from_array(PREDATOR_HOME);
        let (player, mut g, mut last, mut touched) =
            (home + Vec2::new(5.0, 0.4), home + Vec2::new(3.5, 0.2), Vec2::ZERO, false);
        for i in 0..3000 {
            let next = grazer_step(g, player, 1.0 / 60.0);
            assert!(safe(next) && (next - g).dot(last) >= 0.0, "step {i}: in reach or direction reversed");
            touched |= (next.distance(home) - (REACH + RING_MARGIN)).abs() < 1e-4;
            (g, last) = (next, next - g);
        }
        assert!(touched && last == Vec2::ZERO && g.distance(home) >= REACH + RING_MARGIN - 1e-4, "settled at rest");
    }

    #[test]
    fn spawn_clear_of_teaching_ground_and_predator() {
        let home = Vec2::from_array(GRAZER_HOME);
        let clear = |c: Vec2, r: f32| home.distance(c) > r + REACH;
        assert!(clear(Vec2::from_array(LAMP_DISK_CENTER), LAMP_DISK_RADIUS));
        assert!(clear(Vec2::from_array(WATER_POND_CENTER), WATER_POND_RADIUS));
        assert!(WARD_POST_CENTERS.iter().all(|w| clear(Vec2::new(w[0], w[2]), WARD_USE_RADIUS)));
        let node = Vec2::new(THRESHOLD_NODE_CENTER[0], THRESHOLD_NODE_CENTER[2]);
        assert!(clear(node, THRESHOLD_SHELF_USE_RADIUS));
        assert!(home.x.abs() + REACH < PLAY_HALF && home.y.abs() + REACH < PLAY_HALF);
        assert!(home.distance(Vec2::from_array(PREDATOR_HOME)) > REACH + FLEE_RADIUS);
    }
}
