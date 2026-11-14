//! CARD VP-TERRAIN-1 — terraces and terrain shaping.
//!
//! Stepped, readable shelves around the Sanctuary yard. The shelves are a
//! visual mesh only. The play footprint (the 56×56 yard plate the player
//! stands on) stays at the old valley-floor height, so the fixed stand
//! height still meets flat ground. Nothing here writes collision or the
//! STAND clamp.
//!
//! Shelf radii and heights are the same on every preset, so the shape is
//! deterministic. Angular segments are the detail knob, the same split
//! VP-VEG-1 uses: Mobile, Low, and Medium stay under [`CHEAP_VERTEX_CAP`].
//! High and Ultra spend more vertices on a rounder rim. Ground hues are
//! not retuned.
//!
//! | Preset | Segments | Vertices | Cheap |
//! |---|---|---|---|
//! | Mobile | 8 | 504 | yes |
//! | Low | 12 | 756 | yes |
//! | Medium | 16 | 1008 | yes |
//! | High | 32 | 2016 | no |
//! | Ultra | 48 | 3024 | no |
//!
//! Contact: info@Rathor.ai

use std::f32::consts::TAU;

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use shared::local_settings::GraphicsPreset;

/// Half-extent of the Sanctuary yard plate (`climate_plane` 56×56). The
/// player stands on this square. Shelf height here is the old valley floor.
pub const PLAY_HALF: f32 = 28.0;
/// World Y of that floor. Same value as `sky_backdrop::GROUND_Y`.
pub const PLAY_FLOOR_Y: f32 = -0.02;
/// Outer radius of the flat court. The yard's corner (~39.6 m) sits inside
/// this disk, so the first riser is around the play area, not under it.
pub const COURT_RADIUS: f32 = 42.0;
/// How many shelves, including the court. Same on every preset.
pub const SHELF_COUNT: usize = 6;
/// Outer radius of each shelf, metres. Index 0 is the court.
pub const SHELF_OUTER: [f32; SHELF_COUNT] = [COURT_RADIUS, 70.0, 110.0, 165.0, 250.0, 860.0];
/// World Y of each tread. Each step is lower than the one inside it.
pub const SHELF_Y: [f32; SHELF_COUNT] = [-0.02, -1.40, -2.90, -4.50, -6.00, -7.20];
/// Mobile, Low, and Medium terrace meshes stay at or under this.
pub const CHEAP_VERTEX_CAP: usize = 1008;

/// Preset → rim resolution. The shelf steps themselves do not change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerrainPlan {
    /// Vertices around each ring. At least 3.
    pub segments: u32,
    /// Mobile, Low, and Medium.
    pub cheap: bool,
}

impl TerrainPlan {
    pub const fn triangles(self) -> u32 {
        let later = (SHELF_COUNT as u32) - 1;
        // Court disk, plus an annulus and a riser for every shelf after it.
        self.segments * (1 + later * 4)
    }

    pub const fn vertices(self) -> u32 {
        self.triangles() * 3
    }
}

/// Density of the terrace rim for one graphics preset.
///
/// The rows in the module table are this function.
pub const fn terrain_plan(preset: GraphicsPreset) -> TerrainPlan {
    match preset {
        GraphicsPreset::Mobile => TerrainPlan {
            segments: 8,
            cheap: true,
        },
        GraphicsPreset::Low => TerrainPlan {
            segments: 12,
            cheap: true,
        },
        GraphicsPreset::Medium => TerrainPlan {
            segments: 16,
            cheap: true,
        },
        GraphicsPreset::High => TerrainPlan {
            segments: 32,
            cheap: false,
        },
        GraphicsPreset::Ultra => TerrainPlan {
            segments: 48,
            cheap: false,
        },
    }
}

/// True when `(x, z)` is on the yard plate the player stands on.
pub fn play_footprint(x: f32, z: f32) -> bool {
    x.abs() <= PLAY_HALF && z.abs() <= PLAY_HALF
}

/// Which shelf `(x, z)` stands on. The last shelf also covers everything
/// past [`SHELF_OUTER`].
pub fn terrace_shelf(x: f32, z: f32) -> usize {
    let r = (x * x + z * z).sqrt();
    for (index, outer) in SHELF_OUTER.iter().enumerate() {
        if r <= *outer {
            return index;
        }
    }
    SHELF_COUNT - 1
}

/// World Y of the tread at `(x, z)`.
///
/// Stepped: one height per shelf, constant across that shelf. Deterministic:
/// no seed and no preset. The play footprint is shelf 0, at [`PLAY_FLOOR_Y`].
pub fn terrace_height(x: f32, z: f32) -> f32 {
    SHELF_Y[terrace_shelf(x, z)]
}

/// Metres the tread sits below the play-area floor. Zero on the court.
/// Negative on the lower shelves. Props that used to sit on the flat floor
/// add this and stay planted when the shelf drops.
pub fn shelf_offset(x: f32, z: f32) -> f32 {
    terrace_height(x, z) - PLAY_FLOOR_Y
}

/// One mesh: a flat court, then a vertical riser and a flat annulus per
/// lower shelf. Vertex Y is only ever a [`SHELF_Y`] entry.
pub fn terrace_mesh(plan: TerrainPlan) -> Mesh {
    let n = plan.segments.max(3);
    let mut positions = Vec::with_capacity(plan.vertices() as usize);
    let court_y = SHELF_Y[0];
    let court_r = SHELF_OUTER[0];
    let center = [0.0, court_y, 0.0];
    // CARD VIS-TERRAIN-1 — seeded radial wobble on rings 165 and 250 only.
    // Keyed on (ring index, i % n): the riser and tread sharing a ring get the
    // same radius (watertight) and index n reuses index 0 (the rim closes).
    // Rings 42, 70, 110 and 860 stay exact so seated props keep their step.
    let wobble = |ring_ix: usize, radius: f32, i: u32| rim_wobble(ring_ix, radius, i, n);
    for i in 0..n {
        // Decreasing angle so the face points up (Y-up, right-handed).
        push_tri(
            &mut positions,
            center,
            ring(court_r, court_y, i + 1, n),
            ring(court_r, court_y, i, n),
        );
    }
    for shelf in 1..SHELF_COUNT {
        let inner_r = SHELF_OUTER[shelf - 1];
        let outer_r = SHELF_OUTER[shelf];
        let y_top = SHELF_Y[shelf - 1];
        let y = SHELF_Y[shelf];
        for i in 0..n {
            let b0 = ring(wobble(shelf - 1, inner_r, i), y, i, n);
            let b1 = ring(wobble(shelf - 1, inner_r, i + 1), y, i + 1, n);
            let t0 = ring(wobble(shelf - 1, inner_r, i), y_top, i, n);
            let t1 = ring(wobble(shelf - 1, inner_r, i + 1), y_top, i + 1, n);
            push_tri(&mut positions, b0, t0, t1);
            push_tri(&mut positions, b0, t1, b1);
            let i0 = ring(wobble(shelf - 1, inner_r, i), y, i, n);
            let i1 = ring(wobble(shelf - 1, inner_r, i + 1), y, i + 1, n);
            let o0 = ring(wobble(shelf, outer_r, i), y, i, n);
            let o1 = ring(wobble(shelf, outer_r, i + 1), y, i + 1, n);
            push_tri(&mut positions, i0, o1, o0);
            push_tri(&mut positions, i0, i1, o1);
        }
    }
    let count = positions.len() as u32;
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32((0..count).collect()));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    mesh
}

/// Radius of vertex `i` of rim `ring_ix` on an `n`-segment mesh.
fn rim_wobble(ring_ix: usize, radius: f32, i: u32, n: u32) -> f32 {
    if ring_ix != 3 && ring_ix != 4 {
        return radius;
    }
    let mut h = (ring_ix as u32).wrapping_mul(0x9E37_79B9) ^ (i % n).wrapping_mul(0x85EB_CA6B);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297A_2D39);
    h ^= h >> 15;
    let unit = (h as f32 / u32::MAX as f32) * 2.0 - 1.0;
    // Amplitude 0.02: at most ±2% of the ring radius.
    radius * (1.0 + 0.02 * unit)
}

/// CARD TREE-SEAT-1 — world Y of the tread [`terrace_mesh`] draws at
/// `(x, z)` for `plan`: the first rim whose chord over that sector holds the
/// point. Same vertices and wobble as the mesh, unlike the circle model.
pub fn tread_y_mesh(x: f32, z: f32, plan: TerrainPlan) -> f32 {
    let n = plan.segments.max(3);
    let i = ((z.atan2(x).rem_euclid(TAU) / TAU * n as f32) as u32).min(n - 1);
    for (ix, &rim) in SHELF_OUTER.iter().enumerate() {
        let a = ring(rim_wobble(ix, rim, i, n), 0.0, i, n);
        let b = ring(rim_wobble(ix, rim, i + 1, n), 0.0, i + 1, n);
        if (b[0] - a[0]) * (z - a[2]) - (b[2] - a[2]) * (x - a[0]) >= 0.0 {
            return SHELF_Y[ix];
        }
    }
    SHELF_Y[SHELF_COUNT - 1]
}

/// CARD TREE-SEAT-1 — true when a prop of footprint radius `margin` at
/// `(x, z)` misses the moat and every riser band [R·cos(π/12) − margin,
/// R + margin] for R = 42 / 70 / 110. Outside those bands [`shelf_offset`]
/// equals [`tread_y_mesh`] under the whole footprint on every rim of 12+
/// segments (Low and up), so a seat stays right across a preset switch.
/// The wobbled rims (165, 250) lie past anything seated.
pub fn seat_clear(x: f32, z: f32, margin: f32) -> bool {
    let r = (x * x + z * z).sqrt();
    let chord = (std::f32::consts::PI / 12.0).cos();
    let riser = SHELF_OUTER[..3]
        .iter()
        .any(|&rim| r >= rim * chord - margin && r <= rim + margin);
    let moat = r >= crate::water::BATH_INNER - margin && r <= crate::water::BATH_OUTER + margin;
    !riser && !moat
}

fn ring(radius: f32, y: f32, index: u32, segments: u32) -> [f32; 3] {
    let angle = index as f32 / segments as f32 * TAU;
    [angle.cos() * radius, y, angle.sin() * radius]
}

fn push_tri(positions: &mut Vec<[f32; 3]>, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    positions.push(a);
    positions.push(b);
    positions.push(c);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn positions(mesh: &Mesh) -> Vec<[f32; 3]> {
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|values| values.as_float3())
            .unwrap()
            .to_vec()
    }

    fn normals(mesh: &Mesh) -> Vec<[f32; 3]> {
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
            .and_then(|values| values.as_float3())
            .unwrap()
            .to_vec()
    }

    #[test]
    fn terrace_height_is_stepped_and_deterministic() {
        assert_eq!(SHELF_Y[0], PLAY_FLOOR_Y);
        assert_eq!(SHELF_OUTER[0], COURT_RADIUS);
        let yard_corner = Vec2::new(PLAY_HALF, PLAY_HALF).length();
        assert!(
            COURT_RADIUS > yard_corner,
            "court {COURT_RADIUS} does not cover the yard corner {yard_corner}"
        );
        for window in SHELF_Y.windows(2) {
            assert!(window[1] < window[0], "shelves only step down");
        }
        for window in SHELF_OUTER.windows(2) {
            assert!(window[1] > window[0]);
        }

        let court = terrace_height(3.0, -4.0);
        assert_eq!(court, PLAY_FLOOR_Y);
        assert_eq!(terrace_height(3.0, -4.0), court);
        assert_eq!(terrace_shelf(10.0, 10.0), 0);

        // One shelf, two angles, same height. The next radius is the next step.
        let on_shelf = 55.0;
        assert!(on_shelf > SHELF_OUTER[0] && on_shelf < SHELF_OUTER[1]);
        let stepped = terrace_height(on_shelf, 0.0);
        assert_eq!(stepped, SHELF_Y[1]);
        assert!(stepped < court);
        for step in 0..8 {
            let angle = step as f32 / 8.0 * TAU;
            assert_eq!(
                terrace_height(angle.cos() * on_shelf, angle.sin() * on_shelf),
                stepped
            );
        }
        assert_eq!(terrace_height(SHELF_OUTER[1], 0.0), SHELF_Y[1]);
        assert!(terrace_height(SHELF_OUTER[1] + 0.05, 0.0) < stepped);

        let mut prev = terrace_height(0.0, 0.0);
        for i in 0..400 {
            let r = i as f32 * 3.0;
            let h = terrace_height(r, 1.5);
            assert!(h <= prev + 1.0e-5, "rose at r {r}: {h} after {prev}");
            prev = h;
        }
        // Past the last rim the height stays on that shelf.
        assert_eq!(terrace_height(4_000.0, -4_000.0), SHELF_Y[SHELF_COUNT - 1]);
        assert_eq!(terrace_height(-120.0, 40.0), terrace_height(-120.0, 40.0));

        // Detail does not move the steps.
        let sample = terrace_height(180.0, -20.0);
        for preset in GraphicsPreset::ALL {
            let _plan = terrain_plan(preset);
            assert_eq!(terrace_height(180.0, -20.0), sample);
        }
    }

    /// Core ruling #738 comment 6402018728: the shelves stay off the
    /// standing ground. The footprint height is the old valley floor.
    #[test]
    fn play_footprint_height_is_unchanged() {
        assert_eq!(PLAY_FLOOR_Y, crate::sky_backdrop::GROUND_Y);
        assert_eq!(PLAY_HALF * 2.0, 56.0);
        // The outer tread still covers the old 1200 m valley square.
        let half = crate::sky_backdrop::GROUND_SIZE * 0.5;
        let valley_corner = Vec2::new(half, half).length();
        assert!(
            SHELF_OUTER[SHELF_COUNT - 1] >= valley_corner,
            "outer shelf misses the old valley square"
        );
        let xs = [-PLAY_HALF, -20.0, -8.0, 0.0, 8.0, 14.0, PLAY_HALF];
        let zs = [-PLAY_HALF, -14.0, 0.0, 14.0, PLAY_HALF];
        for x in xs {
            for z in zs {
                assert!(play_footprint(x, z), "{x},{z}");
                assert_eq!(terrace_height(x, z), PLAY_FLOOR_Y, "{x},{z}");
                assert_eq!(shelf_offset(x, z), 0.0, "{x},{z}");
                assert_eq!(terrace_shelf(x, z), 0, "{x},{z}");
            }
        }
        // Spawn, boot camera, and the plate corners.
        assert_eq!(terrace_height(0.0, 0.0), PLAY_FLOOR_Y);
        assert_eq!(terrace_height(0.0, 14.0), PLAY_FLOOR_Y);
        assert_eq!(terrace_height(PLAY_HALF, PLAY_HALF), PLAY_FLOOR_Y);
        assert_eq!(terrace_height(-PLAY_HALF, PLAY_HALF), PLAY_FLOOR_Y);
        // The first riser is outside the square.
        let past = COURT_RADIUS + 1.0;
        assert!(!play_footprint(past, 0.0));
        assert!(terrace_height(past, 0.0) < PLAY_FLOOR_Y);
        assert!(shelf_offset(past, 0.0) < 0.0);
    }

    #[test]
    fn mobile_low_and_medium_stay_cheap() {
        let medium = terrain_plan(GraphicsPreset::Medium);
        for preset in [
            GraphicsPreset::Mobile,
            GraphicsPreset::Low,
            GraphicsPreset::Medium,
        ] {
            let plan = terrain_plan(preset);
            assert!(plan.cheap, "{preset:?}");
            assert!(plan.segments <= 16, "{preset:?}");
            assert!(plan.segments <= medium.segments);
            let mesh = terrace_mesh(plan);
            let verts = mesh.count_vertices();
            assert_eq!(verts, plan.vertices() as usize, "{preset:?}");
            assert!(verts <= CHEAP_VERTEX_CAP, "{preset:?} {verts}");
            assert_shelf_mesh(&mesh);
        }
        assert!(
            terrain_plan(GraphicsPreset::Mobile).segments
                < terrain_plan(GraphicsPreset::Low).segments
        );
        assert!(
            terrain_plan(GraphicsPreset::Low).segments
                < terrain_plan(GraphicsPreset::Medium).segments
        );
        assert_eq!(
            terrain_plan(GraphicsPreset::Medium).vertices() as usize,
            CHEAP_VERTEX_CAP
        );

        let high = terrain_plan(GraphicsPreset::High);
        let ultra = terrain_plan(GraphicsPreset::Ultra);
        assert!(!high.cheap && !ultra.cheap);
        assert!(high.segments > medium.segments);
        assert!(ultra.segments > high.segments);
        let high_mesh = terrace_mesh(high);
        let ultra_mesh = terrace_mesh(ultra);
        assert!(high_mesh.count_vertices() > CHEAP_VERTEX_CAP);
        assert!(ultra_mesh.count_vertices() > high_mesh.count_vertices());
        assert_eq!(high_mesh.count_vertices(), high.vertices() as usize);
        assert_eq!(ultra_mesh.count_vertices(), ultra.vertices() as usize);
        assert_shelf_mesh(&high_mesh);
        assert_shelf_mesh(&ultra_mesh);
    }

    fn assert_shelf_mesh(mesh: &Mesh) {
        let pos = positions(mesh);
        let nrm = normals(mesh);
        assert_eq!(pos.len(), nrm.len());
        assert!(!pos.is_empty());
        let mut saw_court_center = false;
        for (index, p) in pos.iter().enumerate() {
            assert!(
                SHELF_Y.iter().any(|y| (p[1] - y).abs() < 1.0e-4),
                "vertex off a shelf: {p:?}"
            );
            if play_footprint(p[0], p[2]) {
                assert!(
                    (p[1] - PLAY_FLOOR_Y).abs() < 1.0e-4,
                    "footprint vertex moved: {p:?}"
                );
            }
            if p[0].abs() < 1.0e-4 && p[2].abs() < 1.0e-4 {
                saw_court_center = true;
                assert!((p[1] - PLAY_FLOOR_Y).abs() < 1.0e-4);
            }
            let normal = nrm[index];
            let flat = normal[1] > 0.9;
            let wall = normal[1].abs() < 0.15;
            assert!(flat || wall, "normal {normal:?} at {p:?}");
            if wall {
                let outward = normal[0] * p[0] + normal[2] * p[2];
                assert!(outward > 0.0, "riser faces inward at {p:?} {normal:?}");
            }
        }
        assert!(saw_court_center);
    }

    #[test]
    fn terrace_wobble_is_deterministic() {
        for preset in [GraphicsPreset::Medium, GraphicsPreset::Ultra] {
            let plan = terrain_plan(preset);
            assert_eq!(
                positions(&terrace_mesh(plan)),
                positions(&terrace_mesh(plan)),
                "{preset:?}"
            );
        }
    }

    #[test]
    fn terrace_wobble_only_on_rings_165_and_250() {
        for preset in [GraphicsPreset::Mobile, GraphicsPreset::Medium, GraphicsPreset::Ultra] {
            let plan = terrain_plan(preset);
            let n = plan.segments;
            let pos = positions(&terrace_mesh(plan));
            for (shelf, &r) in SHELF_OUTER.iter().enumerate() {
                if shelf == 3 || shelf == 4 {
                    let mut moved = false;
                    for p in &pos {
                        let pr = (p[0] * p[0] + p[2] * p[2]).sqrt();
                        if (pr / r - 1.0).abs() < 0.03 {
                            assert!(
                                (pr / r - 1.0).abs() <= 0.02 + 1.0e-4,
                                "{preset:?} r {r}: {pr}"
                            );
                            moved |= (pr - r).abs() > 1.0e-3;
                        }
                    }
                    assert!(moved, "{preset:?} ring {r} never wobbles");
                } else {
                    for i in 0..n {
                        let exact = ring(r, SHELF_Y[shelf], i, n);
                        assert!(pos.contains(&exact), "{preset:?} ring {r} index {i} moved");
                    }
                }
            }
        }
    }

    /// CARD TREE-SEAT-1 — mid-sector just inside a rim the circle and the mesh
    /// disagree; that spot is banded. Outside the bands they agree from Low up.
    #[test]
    fn tree_seat_1_mesh_tread_matches_circle_outside_bands() {
        for preset in GraphicsPreset::ALL {
            let plan = terrain_plan(preset);
            let half = std::f32::consts::PI / plan.segments as f32;
            for (shelf, &rim) in SHELF_OUTER[..3].iter().enumerate() {
                let (x, z) = (half.cos() * (rim - 0.05), half.sin() * (rim - 0.05));
                assert_eq!(terrace_height(x, z), SHELF_Y[shelf], "{preset:?} {rim}");
                assert_eq!(tread_y_mesh(x, z, plan), SHELF_Y[shelf + 1], "{preset:?} {rim}");
                assert_eq!(tread_y_mesh(rim - 0.05, 0.0, plan), SHELF_Y[shelf], "{preset:?} {rim}");
                assert!(!seat_clear(x, z, 0.0) && !seat_clear(rim + 0.5, 0.0, 1.0));
            }
            assert!(seat_clear(0.0, 0.0, 2.0) && seat_clear(90.0, 0.0, 2.0));
            assert!(!seat_clear(0.0, 57.0, 0.0), "moat");
            let mut agreed = 0;
            for k in 0..30_000u32 {
                let (a, r) = (k as f32 * 2.399_963, 1.0 + (k % 1500) as f32 * 0.1);
                let (x, z) = (a.cos() * r, a.sin() * r);
                if plan.segments >= 12 && seat_clear(x, z, 0.0) {
                    assert_eq!(tread_y_mesh(x, z, plan), terrace_height(x, z), "{preset:?} {x},{z}");
                    agreed += 1;
                }
            }
            assert!(plan.segments < 12 || agreed > 15_000, "{preset:?} {agreed}");
        }
    }
}
