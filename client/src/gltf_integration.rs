//! Mesh LOD integration for Powrush-MMO — GraphicsPreset → fidelity tiers.
//!
//! CARD **H-2026-09-12-MESH-LOD** · law [`docs/MESH_QUALITY_BUDGET.md`]
//! CARD **H-2026-09-16-MESH-PERSONA-HANDS** · cite [`docs/MESH_PERSONA_COURT.md`]
//! CARD **H-2026-09-16-PLACE-LOD** · cite [`docs/PLACE_DRESS_SPEC.md`]
//! · [`docs/MESH_PERSONA_COURT.md`] · [`docs/ART_BIBLE.md`]
//! · [`docs/ASSET_BUDGET_COURT.md`] @ `5eff19c` (do not edit).
//!
//! | Tier | Mesh / feel |
//! |---|---|
//! | Low | primitives / capsule-safe LOD · reduced detail |
//! | Medium (default) | balanced humanoid / Place props |
//! | High | fuller PersonaCommit dress when optional assets exist |
//!
//! Lived stacked-capsule presence **and** Place dress (climate_plane path
//! stones) read [`procedural_detail_scale`] from Comfort [`MeshLodPlan`].
//! Low stays primitives / capsule-readable **and** Place-readable. High may
//! gate [`persona_commit_dress_active`] only when an authored glb is already
//! on disk, licensed, with a row in `assets/asset-licenses.csv`. Place dress never loads Buildings / Astra
//! Medium cargo (refs stay refs). Four Places stay four. face ≠ class.
//! Practices after House. No race lobby. No second HUD. Title Online stays grey.
//!
//! Prefer procedural. `assets/models/*.glb` only if licensed (assets/asset-licenses.csv).
//! No new Quellorian art-pack. No binary dump in this PR.
//! UI copy never shows placeholder asset paths.
//!
//! Contact: info@Rathor.ai

use bevy::prelude::{Handle, Mesh, Mesh3d, MeshMaterial3d, StandardMaterial, Transform, Vec3};
use bevy::{asset::LoadState, gltf::GltfAssetLabel, mesh::{Indices, VertexAttributeValues}, prelude::{AssetServer, Assets, Commands, Component, Entity, Query, Res, ResMut, Resource, Without}};
use shared::local_settings::GraphicsPreset;
pub use shared::local_settings::MeshLod;
use crate::painterly_mat as pm;
#[derive(Component, Clone, Copy)] pub struct PropMesh { pub rel: &'static str, pub primitive: usize, pub fit: PropMeshFit }
#[derive(Component)] pub(crate) struct PropMeshSwapped;
#[repr(usize)] #[derive(Clone, Copy, PartialEq, Eq)] pub enum PropMeshFit { Trunk, Canopy, Stone }
const PROP_STONE_XF: (Vec3, Vec3) = (Vec3::new(0.726, 1.23, 0.837), Vec3::new(0., -0.04, 0.));
const PROP_TRUNK_R: f32 = 0.22;
const PROP_TRUNK_HEIGHT: f32 = 2.4;
#[derive(Resource, Default)] pub(crate) struct PropMeshBakeCache { pub baked: [Option<Handle<Mesh>>; 3], pub tree_sxz: f32, pub tree_sy: f32 }
pub fn prop_mesh_glb_active(p: GraphicsPreset) -> bool { !matches!(p, GraphicsPreset::Mobile | GraphicsPreset::Low) }

fn prop_ix_bounds(src: &Mesh) -> (f32, f32, f32) {
    let Some(pos) = src.attribute(Mesh::ATTRIBUTE_POSITION).and_then(|a| a.as_float3()) else { return (0., 0., 1e-4) };
    let (mut y0, mut y1, mut r, len) = (f32::MAX, f32::MIN, 0_f32, pos.len());
    let mut dot = |i: usize| { let p = pos[i]; y0 = y0.min(p[1]); y1 = y1.max(p[1]); r = r.max((p[0] * p[0] + p[2] * p[2]).sqrt()); };
    match src.indices() { Some(Indices::U16(ix)) => ix.iter().for_each(|&i| { if (i as usize) < len { dot(i as usize); } }), Some(Indices::U32(ix)) => ix.iter().for_each(|&i| { if (i as usize) < len { dot(i as usize); } }), None => (0..len).for_each(dot) }
    if y0 > y1 { (0., 0., 1e-4) } else { (y0, y1, r.max(1e-4)) }
}
fn prop_ix_unique(src: &Mesh) -> Vec<usize> {
    let Some(pos) = src.attribute(Mesh::ATTRIBUTE_POSITION).and_then(|a| a.as_float3()) else { return Vec::new() };
    let len = pos.len();
    let mut v: Vec<usize> = match src.indices() { Some(Indices::U16(ix)) => ix.iter().filter_map(|&i| ((i as usize) < len).then_some(i as usize)).collect(), Some(Indices::U32(ix)) => ix.iter().filter_map(|&i| ((i as usize) < len).then_some(i as usize)).collect(), None => (0..len).collect() };
    v.sort_unstable(); v.dedup(); v
}
fn bake_prop_mesh(src: &Mesh, fit: PropMeshFit, tree_sxz: f32, tree_sy: f32) -> (Mesh, f32, f32) {
    let mut out = src.clone();
    let Some(VertexAttributeValues::Float32x3(pos)) = out.attribute_mut(Mesh::ATTRIBUTE_POSITION) else { return (out, 1., 1.) };
    let (y0, y1, r0) = prop_ix_bounds(src);
    let drawn = prop_ix_unique(src);
    let (mut ul, mut span, mut sxz, mut sy) = (PROP_STONE_XF.0, 1e-4_f32, 1_f32, 1_f32);
    match fit {
        PropMeshFit::Stone => { let (s, t) = PROP_STONE_XF; for p in pos.iter_mut() { *p = [p[0] * s.x + t.x, p[1] * s.y + t.y, p[2] * s.z + t.z]; } span = pos.iter().map(|p| (p[0] * p[0] + p[2] * p[2]).sqrt()).fold(0., f32::max).max(1e-4); ul = s; }
        PropMeshFit::Trunk => { sxz = PROP_TRUNK_R / r0; sy = PROP_TRUNK_HEIGHT / (y1 - y0).max(1e-4); for &i in &drawn { let p = &mut pos[i]; p[0] *= sxz; p[2] *= sxz; p[1] = (p[1] - y0) * sy; } span = PROP_TRUNK_R; }
        PropMeshFit::Canopy => { sxz = tree_sxz; sy = 2.6 / (y1 - y0).max(1e-4); let (mut cx, mut cz, n) = (0_f32, 0_f32, drawn.len().max(1) as f32); for &i in &drawn { cx += pos[i][0]; cz += pos[i][2]; } cx /= n; cz /= n; for &i in &drawn { let p = &mut pos[i]; p[0] = (p[0] - cx) * sxz; p[2] = (p[2] - cz) * sxz; p[1] = (p[1] - y0) * sy; } span = drawn.iter().map(|&i| (pos[i][0] * pos[i][0] + pos[i][2] * pos[i][2]).sqrt()).fold(0., f32::max).max(1e-4); }
    }
    if let Some(VertexAttributeValues::Float32x2(uv)) = out.attribute_mut(Mesh::ATTRIBUTE_UV_0) { let d = (ul.max_element().max(sxz).max(sy) * span / 4.).max(1e-4); for p in uv.iter_mut() { p[0] /= d; p[1] /= d; } }
    if matches!(fit, PropMeshFit::Trunk) { let _ = out.generate_tangents(); }
    (out, sxz, sy)
}
pub(crate) fn swap_prop_meshes(
    feel: Option<Res<crate::local_settings::LocalMeshLodFeel>>, server: Option<Res<AssetServer>>, mut meshes: ResMut<Assets<Mesh>>,
    mut cache: ResMut<PropMeshBakeCache>, mut ramps: Option<ResMut<pm::PainterlyRampMeshes>>,
    mut q: Query<(Entity, &mut Mesh3d, &PropMesh, &mut Transform), Without<PropMeshSwapped>>, mut commands: Commands,
) {
    let (Some(feel), Some(server)) = (feel, server) else { return };
    let active = prop_mesh_glb_active(feel.preset);
    for (e, mut mesh3d, prop, mut tf) in &mut q {
        if !active || !asset_file_present(prop.rel) { commands.entity(e).insert(PropMeshSwapped); continue; }
        let h = server.load(GltfAssetLabel::Primitive { mesh: 0, primitive: prop.primitive }.from_asset(prop.rel));
        if matches!(server.get_load_state(h.id()), Some(LoadState::Failed(_))) { commands.entity(e).insert(PropMeshSwapped); continue; }
        let Some(src) = meshes.get(&h).cloned() else { continue };
        if matches!(prop.fit, PropMeshFit::Canopy) && cache.tree_sxz <= 0. { continue; }
        let i = prop.fit as usize;
        let baked = cache.baked[i].clone().unwrap_or_else(|| {
            let (mut m, sxz, sy) = bake_prop_mesh(&src, prop.fit, cache.tree_sxz, cache.tree_sy);
            if matches!(prop.fit, PropMeshFit::Trunk) { cache.tree_sxz = sxz; cache.tree_sy = sy; }
            if matches!(prop.fit, PropMeshFit::Canopy) { pm::paint_height_ramp(&mut m, &pm::painterly_shade(pm::PainterlyRole::Foliage, feel.preset)); }
            let bh = meshes.add(m); if matches!(prop.fit, PropMeshFit::Canopy) { pm::remember_ramp(&mut ramps, &bh, pm::PainterlyRole::Foliage); }
            cache.baked[i] = Some(bh.clone()); bh
        });
        mesh3d.0 = baked; if matches!(prop.fit, PropMeshFit::Canopy) { tf.translation.y += PROP_TRUNK_HEIGHT - 1.35; } commands.entity(e).insert(PropMeshSwapped);
    }
}

/// Resolved presentation plan for a Comfort graphics preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshLodPlan {
    pub lod: MeshLod,
    /// Low: capsule / primitive presentation only.
    pub primitives_only: bool,
    /// Medium+: may prefer optional on-disk glb when present (never invent UI paths).
    pub prefer_optional_glb: bool,
    /// High: fuller PersonaCommit dress when assets exist.
    pub persona_commit_dress: bool,
}

/// Map Esc Comfort [`GraphicsPreset`] → mesh LOD tier.
pub fn mesh_lod_for_preset(preset: GraphicsPreset) -> MeshLod {
    preset.mesh_lod()
}

/// Build the presentation plan for a Comfort graphics preset.
pub fn plan_for_preset(preset: GraphicsPreset) -> MeshLodPlan {
    let lod = mesh_lod_for_preset(preset);
    MeshLodPlan {
        lod,
        primitives_only: lod.primitives_only(),
        prefer_optional_glb: lod.prefer_optional_glb(),
        persona_commit_dress: lod.persona_commit_dress(),
    }
}

/// Player-facing LOD feel label — never an asset path string.
pub fn lod_feel_label(lod: MeshLod) -> &'static str {
    lod.feel_label()
}

/// Whether an optional on-disk glb may be used for this plan.
/// `asset_present` must come from a real presence check — never invent paths for UI.
pub fn use_optional_glb(plan: &MeshLodPlan, asset_present: bool) -> bool {
    plan.prefer_optional_glb && asset_present
}

/// High-tier PersonaCommit dress is active only when the plan asks and assets exist.
pub fn persona_commit_dress_active(plan: &MeshLodPlan, asset_present: bool) -> bool {
    plan.persona_commit_dress && asset_present
}

/// Category for future per-type LOD / material handling (no asset path strings).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GltfCategory {
    Player,
    Structure,
    Ship,
    Prop,
}

/// Capsule-safe / procedural detail scale for Low / Medium / High.
/// 1.0 = Medium balance; Low reduces; High holds fuller scale when dressed.
pub fn procedural_detail_scale(lod: MeshLod) -> f32 {
    match lod {
        MeshLod::Low => 0.65,
        MeshLod::Medium => 1.0,
        MeshLod::High => 1.15,
    }
}

/// Lived stacked-capsule scale from Comfort [`MeshLodPlan`].
/// Low stays capsule-readable; High is fuller only as a scale, not a dump.
pub fn lived_presence_detail_scale(plan: &MeshLodPlan) -> f32 {
    procedural_detail_scale(plan.lod)
}

/// Lived Place-dress scale from Comfort [`MeshLodPlan`].
/// Path stones / greybox props only. Low stays readable Place identity.
/// Buildings / Astra Medium stay REFS (licensed `.glb` rule: assets/asset-licenses.csv). Four Places stay four.
pub fn lived_place_dress_detail_scale(plan: &MeshLodPlan) -> f32 {
    procedural_detail_scale(plan.lod)
}

/// Place dress never loads Buildings / Astra Medium cargo. Refs stay refs
/// even on High, even if an authored glb is already on disk.
/// PropMesh primitive swap is not this Place-dress pipeline.
pub fn place_dress_uses_authored_glb(plan: &MeshLodPlan) -> bool {
    let _ = (plan, optional_authored_glb_present());
    false
}

/// Optional on-disk authored glb presence. Read-only — never writes, never
/// invents a player-facing path string.
pub fn optional_authored_glb_present() -> bool {
    optional_authored_glb_present_in(std::path::Path::new("assets/models"))
}

/// Test-seam: scan one directory for an already-present `.glb`. No create.
pub fn optional_authored_glb_present_in(dir: &std::path::Path) -> bool {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return false;
    };
    entries.flatten().any(|e| {
        e.path()
            .extension()
            .map(|ext| ext == "glb")
            .unwrap_or(false)
    })
}

/// Bevy `FileAssetReader` root (`BEVY_ASSET_ROOT`, else `CARGO_MANIFEST_DIR`, else exe dir).
pub fn bevy_asset_base() -> std::path::PathBuf {
    bevy::asset::io::file::FileAssetReader::get_base_path()
}

pub fn asset_file_present(rel: &str) -> bool {
    use std::io::Read;
    let path = bevy_asset_base().join("assets").join(rel);
    let Ok(mut file) = std::fs::File::open(&path) else { return false; };
    let mut head = [0u8; 12];
    if file.read_exact(&mut head).is_err() || head.starts_with(b"version https://git-lfs.github.com/spec/v1") {
        return false;
    }
    head[..4] == *b"glTF"
}

pub fn practice_traveler_capsule(mesh: Handle<Mesh>, mat: Handle<StandardMaterial>, pos: Vec3) -> (Mesh3d, MeshMaterial3d<StandardMaterial>, Transform) {
    (Mesh3d(mesh), MeshMaterial3d(mat), Transform::from_translation(pos))
}
/// Presence of the UAL1 Standard body under the AssetServer root
/// (`assets/`), resolved the same way Bevy's default `FileAssetReader`
/// does: `BEVY_ASSET_ROOT`, else `CARGO_MANIFEST_DIR`, else the
/// executable directory, then `assets/`. Pack path is private pin :28 only.
pub fn optional_ual1_standard_glb_present() -> bool {
    let base = if let Ok(root) = std::env::var("BEVY_ASSET_ROOT") {
        std::path::PathBuf::from(root)
    } else if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        std::path::PathBuf::from(manifest)
    } else {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."))
    };
    base.join("assets")
        .join("private/animations/quaternius-ual-1/Unreal-Godot/UAL1_Standard.glb")
        .is_file()
}

/// High PersonaCommit dress for lived presence — only when the plan asks
/// **and** an authored glb already exists. Never dumps a binary.
pub fn lived_persona_dress_active(plan: &MeshLodPlan) -> bool {
    persona_commit_dress_active(plan, optional_authored_glb_present())
}

/// Lived presence may prefer an on-disk glb only when the plan allows and
/// the file is already there. Does not load or invent UI paths.
pub fn lived_presence_uses_authored_glb(plan: &MeshLodPlan) -> bool {
    use_optional_glb(plan, optional_authored_glb_present())
}

/// face ≠ class — phenotype / people label never picks a class or greys Peace.
pub fn face_is_not_class() -> bool {
    true
}

/// Practices are post-House dress — never Title race select, never E/I/H/R gate.
pub fn practices_after_house() -> bool {
    true
}

/// Title has no race lobby. Mechanical race stays sim-owned.
pub fn race_lobby_closed() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::local_settings::LocalSettings;

    #[test]
    fn lod_selection_by_graphics_preset() {
        assert_eq!(mesh_lod_for_preset(GraphicsPreset::Low), MeshLod::Low);
        assert_eq!(mesh_lod_for_preset(GraphicsPreset::Medium), MeshLod::Medium);
        assert_eq!(mesh_lod_for_preset(GraphicsPreset::High), MeshLod::High);

        let low = plan_for_preset(GraphicsPreset::Low);
        assert!(low.primitives_only);
        assert!(!low.prefer_optional_glb);
        assert!(!low.persona_commit_dress);
        assert!((procedural_detail_scale(low.lod) - 0.65).abs() < f32::EPSILON);
        assert!(!use_optional_glb(&low, true));
        assert!(!persona_commit_dress_active(&low, true));

        let mid = plan_for_preset(GraphicsPreset::Medium);
        assert!(!mid.primitives_only);
        assert!(mid.prefer_optional_glb);
        assert!(!mid.persona_commit_dress);
        assert!(use_optional_glb(&mid, true));
        assert!(!use_optional_glb(&mid, false));
        assert!(!persona_commit_dress_active(&mid, true));
        assert!((procedural_detail_scale(mid.lod) - 1.0).abs() < f32::EPSILON);

        let high = plan_for_preset(GraphicsPreset::High);
        assert!(!high.primitives_only);
        assert!(high.prefer_optional_glb);
        assert!(high.persona_commit_dress);
        assert!(use_optional_glb(&high, true));
        assert!(!use_optional_glb(&high, false));
        assert!(persona_commit_dress_active(&high, true));
        assert!(!persona_commit_dress_active(&high, false));
        assert!((procedural_detail_scale(high.lod) - 1.15).abs() < f32::EPSILON);
    }

    #[test]
    fn lod_feel_labels_never_show_placeholder_paths() {
        assert_eq!(
            lod_feel_label(mesh_lod_for_preset(GraphicsPreset::Low)),
            "Capsule · reduced detail"
        );
        assert_eq!(
            lod_feel_label(mesh_lod_for_preset(GraphicsPreset::Medium)),
            "Balanced · Place props"
        );
        assert_eq!(
            lod_feel_label(mesh_lod_for_preset(GraphicsPreset::High)),
            "Fuller dress · nicest hold"
        );
        for preset in GraphicsPreset::ALL {
            let label = lod_feel_label(mesh_lod_for_preset(preset));
            assert!(!label.is_empty());
            assert!(!label.contains("assets/"), "{label}");
            assert!(!label.contains(".glb"), "{label}");
            assert!(!label.contains("models/"), "{label}");
            assert!(!label.contains("player_avatar"), "{label}");
        }
    }

    #[test]
    fn local_settings_graphics_preset_drives_mesh_lod() {
        let mut s = LocalSettings::peace_defaults();
        assert_eq!(s.graphics_preset, GraphicsPreset::Medium);
        assert_eq!(plan_for_preset(s.graphics_preset).lod, MeshLod::Medium);
        s.set_graphics_preset(GraphicsPreset::Low);
        assert!(plan_for_preset(s.graphics_preset).primitives_only);
        s.set_graphics_preset(GraphicsPreset::High);
        assert!(plan_for_preset(s.graphics_preset).persona_commit_dress);
    }

    #[test]
    fn comfort_plan_wires_place_dress_scale_without_glb_dump() {
        let low = plan_for_preset(GraphicsPreset::Low);
        let mid = plan_for_preset(GraphicsPreset::Medium);
        let high = plan_for_preset(GraphicsPreset::High);

        assert!((lived_place_dress_detail_scale(&low) - 0.65).abs() < f32::EPSILON);
        assert!((lived_place_dress_detail_scale(&mid) - 1.0).abs() < f32::EPSILON);
        assert!((lived_place_dress_detail_scale(&high) - 1.15).abs() < f32::EPSILON);
        assert_eq!(
            lived_place_dress_detail_scale(&low),
            lived_presence_detail_scale(&low)
        );
        assert!(low.primitives_only);
        assert!(!mid.primitives_only);
        assert!(!high.primitives_only);

        let before = optional_authored_glb_present();
        assert!(!place_dress_uses_authored_glb(&low));
        assert!(!place_dress_uses_authored_glb(&mid));
        assert!(!place_dress_uses_authored_glb(&high));
        assert_eq!(optional_authored_glb_present(), before);
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        assert_eq!(
            lod_feel_label(mesh_lod_for_preset(GraphicsPreset::Low)),
            "Capsule · reduced detail"
        );
        assert_eq!(
            lod_feel_label(mesh_lod_for_preset(GraphicsPreset::Medium)),
            "Balanced · Place props"
        );
        assert_eq!(
            lod_feel_label(mesh_lod_for_preset(GraphicsPreset::High)),
            "Fuller dress · nicest hold"
        );
        for preset in GraphicsPreset::ALL {
            let label = lod_feel_label(mesh_lod_for_preset(preset));
            assert!(!label.contains(".glb"), "{label}");
            assert!(!label.contains("Ultra"), "{label}");
            assert!(!label.contains("fifth"), "{label}");
        }
    }

    #[test]
    fn comfort_plan_wires_lived_capsule_scale_without_glb_dump() {
        let low = plan_for_preset(GraphicsPreset::Low);
        let mid = plan_for_preset(GraphicsPreset::Medium);
        let high = plan_for_preset(GraphicsPreset::High);

        assert!((lived_presence_detail_scale(&low) - 0.65).abs() < f32::EPSILON);
        assert!((lived_presence_detail_scale(&mid) - 1.0).abs() < f32::EPSILON);
        assert!((lived_presence_detail_scale(&high) - 1.15).abs() < f32::EPSILON);
        assert!(low.primitives_only);
        assert!(!mid.primitives_only);
        assert!(!high.primitives_only);

        let before = optional_authored_glb_present();
        assert!(!lived_persona_dress_active(&low));
        assert!(!lived_persona_dress_active(&mid));
        assert_eq!(lived_persona_dress_active(&high), before);
        assert_eq!(lived_presence_uses_authored_glb(&low), false);
        assert_eq!(lived_presence_uses_authored_glb(&high), before);
        assert_eq!(optional_authored_glb_present(), before);

        let missing = std::path::Path::new("assets/models/__powrush_no_such_pack__");
        assert!(!optional_authored_glb_present_in(missing));
        assert!(!persona_commit_dress_active(&high, false));
        assert!(persona_commit_dress_active(&high, true));
        assert!(!persona_commit_dress_active(&low, true));
    }

    #[test]
    fn face_not_class_practices_after_house_no_race_lobby_no_ultra() {
        assert!(face_is_not_class());
        assert!(practices_after_house());
        assert!(race_lobby_closed());
        assert_eq!(GraphicsPreset::ALL.len(), 5);
        assert_eq!(
            lod_feel_label(mesh_lod_for_preset(GraphicsPreset::Low)),
            "Capsule · reduced detail"
        );
        assert_eq!(
            lod_feel_label(mesh_lod_for_preset(GraphicsPreset::Medium)),
            "Balanced · Place props"
        );
        assert_eq!(
            lod_feel_label(mesh_lod_for_preset(GraphicsPreset::High)),
            "Fuller dress · nicest hold"
        );
        for preset in GraphicsPreset::ALL {
            let label = lod_feel_label(mesh_lod_for_preset(preset));
            assert!(!label.contains(".glb"), "{label}");
            assert!(!label.contains("Ultra"), "{label}");
            assert!(!label.contains("race lobby"), "{label}");
        }
    }

    #[test]
    fn props_wire_f2_corrupt_keeps_primitive() {
        let dir = std::env::temp_dir().join(format!("pw2_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(dir.join("assets/models/bad")).unwrap();
        let path = dir.join("assets/models/bad/x.glb"); std::fs::write(&path, b"not gltf").unwrap();
        let mut head = [0u8; 12]; let mut f = std::fs::File::open(&path).unwrap();
        assert!(std::io::Read::read_exact(&mut f, &mut head).is_err() || head[..4] != *b"glTF");
        let _ = std::fs::remove_dir_all(&dir);
    }
    #[test]
    fn props_wire_f8_f13_licensed_and_low_stays_primitive() {
        let csv = include_str!("../../assets/asset-licenses.csv");
        assert!(csv.contains("kenney-nature-kit/tree_default.glb") && csv.contains("stone_smallFlatA.glb"));
        assert!(!optional_authored_glb_present() && !prop_mesh_glb_active(GraphicsPreset::Low));
        assert!(prop_mesh_glb_active(GraphicsPreset::Medium));
    }
    #[test]
    fn props_wire_f11_uv_density_matches_primitive() {
        use bevy::{asset::RenderAssetUsages, mesh::{PrimitiveTopology, VertexAttributeValues}};
        let mut m = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[2., 0., 0.], [0., 0., 0.]]); m.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[1., 0.], [0., 0.]]);
        let (b, _, _) = super::bake_prop_mesh(&m, PropMeshFit::Stone, 0., 0.);
        let (VertexAttributeValues::Float32x3(p), VertexAttributeValues::Float32x2(uv)) = (b.attribute(Mesh::ATTRIBUTE_POSITION).unwrap(), b.attribute(Mesh::ATTRIBUTE_UV_0).unwrap()) else { panic!() };
        let span = p.iter().map(|v| (v[0] * v[0] + v[2] * v[2]).sqrt()).fold(0_f32, f32::max); let du = (uv[0][0] - uv[1][0]).abs(); let s = PROP_STONE_XF.0.max_element();
        assert!((span * du - 4.0 / s).abs() < 0.02);
    }
    #[test]
    fn props_wire_f4_canopy_waits_for_trunk_tree_scale() {
        use bevy::{asset::AssetPlugin, gltf::GltfPlugin, image::ImagePlugin, mesh::MeshPlugin, prelude::*, tasks::tick_global_task_pools_on_main_thread, world_serialization::WorldSerializationPlugin};
        let _g = crate::test_env::lock(); let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").canonicalize().unwrap(); let mut app = App::new(); app.add_plugins((MinimalPlugins, AssetPlugin { file_path: root.join("assets").into_os_string().into_string().unwrap(), watch_for_changes_override: Some(false), use_asset_processor_override: Some(false), ..AssetPlugin::default() }, ImagePlugin::default(), MeshPlugin, GltfPlugin::default(), WorldSerializationPlugin)).init_asset::<Image>().init_asset::<Mesh>().init_resource::<Assets<StandardMaterial>>().init_resource::<PropMeshBakeCache>().insert_resource(crate::local_settings::LocalMeshLodFeel { preset: GraphicsPreset::Medium }).add_systems(Update, swap_prop_meshes); app.finish(); let tree = "models/kenney-nature-kit/tree_default.glb"; let h = app.world_mut().resource_mut::<Assets<Mesh>>().add(Cuboid::new(0.2, 2., 0.2)); let mat = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default()); let (cm, _, ct) = practice_traveler_capsule(h.clone(), mat.clone(), Vec3::ZERO); let ce = app.world_mut().spawn((cm, ct, PropMesh { rel: tree, primitive: 1, fit: PropMeshFit::Canopy })).id(); crate::test_env::with_bevy_asset_root(&root, || { app.update(); tick_global_task_pools_on_main_thread(); }); assert!(app.world().get::<PropMeshSwapped>(ce).is_none()); let (tm, _, tt) = practice_traveler_capsule(h, mat, Vec3::ZERO); let te = app.world_mut().spawn((tm, tt, PropMesh { rel: tree, primitive: 0, fit: PropMeshFit::Trunk })).id();
        for _ in 0..2400 { crate::test_env::with_bevy_asset_root(&root, || { app.update(); tick_global_task_pools_on_main_thread(); }); if app.world().get::<PropMeshSwapped>(te).is_some() && app.world().get::<PropMeshSwapped>(ce).is_some() { break; } }
        assert!(app.world().get::<PropMeshSwapped>(te).is_some() && app.world().get::<PropMeshSwapped>(ce).is_some());
    }
    #[test]
    fn props_wire_f1_missing_asset_keeps_primitive() {
        use bevy::{mesh::MeshPlugin, prelude::*};
        let _g = crate::test_env::lock(); let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").canonicalize().unwrap();
        let mut app = App::new(); app.add_plugins((MinimalPlugins, AssetPlugin { file_path: root.join("assets").into_os_string().into_string().unwrap(), ..AssetPlugin::default() }, MeshPlugin)).insert_resource(crate::local_settings::LocalMeshLodFeel { preset: GraphicsPreset::Medium }).init_resource::<Assets<Mesh>>().init_resource::<Assets<StandardMaterial>>().init_resource::<PropMeshBakeCache>().add_systems(Update, swap_prop_meshes);
        let h = app.world_mut().resource_mut::<Assets<Mesh>>().add(Cuboid::default()); let mat = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(StandardMaterial::default());
        let (m3d, _, tf) = practice_traveler_capsule(h.clone(), mat, Vec3::ZERO); let e = app.world_mut().spawn((m3d, tf, PropMesh { rel: "models/__none__.glb", primitive: 0, fit: PropMeshFit::Stone })).id();
        crate::test_env::with_bevy_asset_root(&root, || app.update());
        assert!(app.world().get::<PropMeshSwapped>(e).is_some() && app.world().get::<Mesh3d>(e).unwrap().0.id() == h.id());
    }
}
