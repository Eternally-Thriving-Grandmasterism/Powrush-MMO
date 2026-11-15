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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vegetation::{
        cleared_plants, trunk_on_plate, vegetation_plan, FieldTrunks, VegKind, TRUNK_RADIUS,
    };
    use shared::local_settings::GraphicsPreset;

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
        assert!(
            (pos.xz().distance(center) - reach).abs() < 1e-3,
            "walk did not stop at off-plate trunk {pos}"
        );
        assert!(pos.z < center.y + 1e-3, "walked through {pos}");
    }
}