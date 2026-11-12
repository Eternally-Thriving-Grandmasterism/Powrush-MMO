//! STONE-GROUND-1 — Sanctuary yard stone when private stone-painterly-08 is present.
//! Private-path use only. Missing, pointer, bad layout, Low, and Mobile keep today's plane.

use std::path::{Path, PathBuf};

use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::prelude::*;

use shared::local_settings::GraphicsPreset;

use crate::local_settings::LocalMeshLodFeel;

const GRID: usize = 8;
const TILES: usize = 16;
const CELL_M: f32 = 7.0;
const YARD_M: f32 = 56.0;
pub(crate) const STONE_ROUGHNESS: f32 = 0.90; // README range 0.84–0.97
const PNG_MAGIC: &[u8] = b"\x89PNG\r\n\x1a\n";
const PACK: &str = "assets/private/textures/stone-painterly-08";

#[derive(Component, Clone, Copy)]
pub(crate) struct StoneGroundTile {
    #[allow(dead_code)] // placement tests read the index; the lib build does not
    pub index: u8,
}

#[derive(Resource)]
pub(crate) struct StoneAssetDir(pub PathBuf);

struct StoneSet {
    grid: [[u8; GRID]; GRID],
    albedo: Vec<Image>,
    normal: Vec<Image>,
}

fn asset_root() -> PathBuf {
    if let Ok(root) = std::env::var("BEVY_ASSET_ROOT") {
        PathBuf::from(root)
    } else if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest)
    } else {
        std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf)).unwrap_or_else(|| PathBuf::from("."))
    }
}

fn tier_allows(feel: Option<&LocalMeshLodFeel>) -> bool {
    feel.map(|row| row.preset).unwrap_or(GraphicsPreset::Medium).mesh_lod() != shared::local_settings::MeshLod::Low
}

fn tile_index(cell: &serde_json::Value) -> Option<u8> {
    let n = cell.as_u64()?;
    (n < TILES as u64).then_some(n as u8)
}

fn read_grid(node: &serde_json::Value) -> Option<[[u8; GRID]; GRID]> {
    let arr = node.as_array()?;
    let mut grid = [[0u8; GRID]; GRID];
    if arr.len() == GRID {
        for (row, item) in arr.iter().enumerate() {
            let cols = item.as_array()?;
            if cols.len() != GRID {
                return None;
            }
            for (col, cell) in cols.iter().enumerate() {
                grid[row][col] = tile_index(cell)?;
            }
        }
        return Some(grid);
    }
    None
}

fn map_path(dir: &Path, kind: &str, index: u8) -> PathBuf {
    // Tile bits are NW NE SW SE: index 0 is v0000, 5 is v0101, 15 is v1111.
    dir.join(format!("stone-painterly-08_v{index:04b}_{kind}.png"))
}

fn decode_png(bytes: &[u8], srgb: bool) -> Option<Image> {
    if !bytes.starts_with(PNG_MAGIC) {
        return None;
    }
    Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        srgb,
        ImageSampler::Default,
        RenderAssetUsages::default(),
    )
    .ok()
}

fn load_set(dir: &Path) -> Option<StoneSet> {
    let raw = std::fs::read(dir.join("layout.json")).ok()?;
    if raw.starts_with(b"version https://git-lfs.github.com/spec/v1") {
        return None;
    }
    let value: serde_json::Value = serde_json::from_slice(&raw).ok()?;
    if value.get("cells").and_then(|n| n.as_u64()) != Some(GRID as u64) {
        return None;
    }
    let grid = read_grid(value.get("indices")?)?;
    let mut albedo = Vec::with_capacity(TILES);
    let mut normal = Vec::with_capacity(TILES);
    for index in 0..TILES as u8 {
        let color = std::fs::read(map_path(dir, "albedo", index)).ok()?;
        let normals = std::fs::read(map_path(dir, "normal-gl", index)).ok()?;
        albedo.push(decode_png(&color, true)?);
        normal.push(decode_png(&normals, false)?);
    }
    Some(StoneSet { grid, albedo, normal })
}

fn cell_center(col: usize, row: usize) -> Vec3 {
    // Row 0 is north (-Z). Column 0 is west (-X). 8×7 m covers the 56 m plane.
    let x = -YARD_M * 0.5 + CELL_M * (col as f32 + 0.5);
    let z = -YARD_M * 0.5 + CELL_M * (row as f32 + 0.5);
    Vec3::new(x, 0.0, z)
}

pub(crate) fn place_sanctuary_stone(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: Option<&mut Assets<Image>>,
    feel: Option<&LocalMeshLodFeel>,
    dir: Option<&Path>,
) -> bool {
    let Some(images) = images else { return false };
    if !tier_allows(feel) {
        return false;
    }
    let owned;
    let root = if let Some(dir) = dir {
        dir
    } else {
        owned = asset_root().join(PACK);
        owned.as_path()
    };
    let Some(set) = load_set(root) else { return false };
    let mut mesh: Mesh = Plane3d::default().mesh().size(CELL_M, CELL_M).into();
    let _ = mesh.generate_tangents();
    let mesh = meshes.add(mesh);
    let mut mats = Vec::with_capacity(TILES);
    for (albedo, normal) in set.albedo.into_iter().zip(set.normal) {
        mats.push(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(images.add(albedo)),
            normal_map_texture: Some(images.add(normal)),
            metallic: 0.0,
            perceptual_roughness: STONE_ROUGHNESS,
            flip_normal_map_y: false,
            ..default()
        }));
    }
    for row in 0..GRID {
        for col in 0..GRID {
            let index = set.grid[row][col];
            commands.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(mats[index as usize].clone()),
                Transform::from_translation(cell_center(col, row)),
                Visibility::Visible,
                StoneGroundTile { index },
            ));
        }
    }
    info!(target: "powrush::climate", "sanctuary yard using private stone-painterly-08");
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::climate_plane::ClimatePlanePlugin;
    use crate::living_practice_loop::SoftPlayerRealm;
    use shared::local_settings::GraphicsPreset;

    const PNG_1X1: &[u8] = &[
        137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0, 0, 0,
        144, 119, 83, 222, 0, 0, 0, 12, 73, 68, 65, 84, 120, 156, 99, 248, 207, 192, 0, 0, 3, 1, 1, 0, 201, 254, 146, 239, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
    ];
    const POINTER: &[u8] = b"version https://git-lfs.github.com/spec/v1\noid sha256:abc\nsize 1\n";

    fn scratch(label: &str) -> PathBuf {
        static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("powrush-sg1-{label}-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        path
    }
    fn layout() -> String {
        let rows: Vec<_> = (0..GRID).map(|row| {
            let cols: Vec<_> = (0..GRID).map(|col| ((row * GRID + col) % TILES).to_string()).collect();
            format!("[{}]", cols.join(","))
        }).collect();
        let z = "[0,0,0,0,0,0,0,0,0]";
        let corners = std::iter::repeat(z).take(9).collect::<Vec<_>>().join(",");
        format!(
            r#"{{"seed":1,"cells":8,"corners":[{corners}],"indices":[{}],"metrics":{{}},"note":"row 0 north, col 0 west"}}"#,
            rows.join(","),
        )
    }
    fn write_pack(dir: &Path, body: &str, bytes: &[u8]) {
        std::fs::write(dir.join("layout.json"), body).unwrap();
        for index in 0..TILES as u8 {
            for kind in ["albedo", "normal-gl"] {
                std::fs::write(map_path(dir, kind, index), bytes).unwrap();
            }
        }
    }
    fn boot(dir: &Path, preset: Option<GraphicsPreset>) -> App {
        let mut app = App::new();
        app.add_plugins(bevy::MinimalPlugins)
            .init_resource::<Assets<Mesh>>().init_resource::<Assets<StandardMaterial>>()
            .init_resource::<Assets<Image>>().init_resource::<SoftPlayerRealm>()
            .insert_resource(StoneAssetDir(dir.to_path_buf())).add_plugins(ClimatePlanePlugin);
        if let Some(preset) = preset { app.insert_resource(LocalMeshLodFeel { preset }); }
        app.update();
        app
    }
    fn placeholder(app: &mut App) {
        assert_eq!(stone_n(app), 0);
        assert_ne!(yard_vis(app), Visibility::Hidden);
    }
    fn stone_n(app: &mut App) -> usize {
        let mut q = app.world_mut().query::<&StoneGroundTile>();
        q.iter(app.world()).count()
    }
    fn yard_vis(app: &mut App) -> Visibility {
        let mut q = app.world_mut().query::<(&Name, &Visibility)>();
        q.iter(app.world()).find(|(name, _)| name.as_str() == "ClimateGround").map(|(_, vis)| *vis).unwrap_or(Visibility::Inherited)
    }
    #[test]
    fn png_constant_decodes_and_a_pointer_does_not() {
        let color = decode_png(PNG_1X1, true).expect("srgb");
        let linear = decode_png(PNG_1X1, false).expect("linear");
        assert_eq!((color.width(), color.height(), linear.width(), linear.height()), (1, 1, 1, 1));
        assert_ne!(color.texture_descriptor.format, linear.texture_descriptor.format);
        assert!(decode_png(POINTER, true).is_none());
        assert!(decode_png(b"\x89PNG\r\n\x1a\ntruncated", true).is_none());
        assert!((0.84..=0.97).contains(&STONE_ROUGHNESS));
        let name = |kind: &str, index| map_path(Path::new("d"), kind, index);
        assert_eq!(name("albedo", 0).file_name().unwrap(), "stone-painterly-08_v0000_albedo.png");
        assert_eq!(name("albedo", 5).file_name().unwrap(), "stone-painterly-08_v0101_albedo.png");
        assert_eq!(name("normal-gl", 15).file_name().unwrap(), "stone-painterly-08_v1111_normal-gl.png");
    }
    #[test]
    fn absent_pack_keeps_the_placeholder() {
        placeholder(&mut boot(&scratch("absent"), None));
    }
    #[test]
    fn lfs_pointer_keeps_the_placeholder() {
        let dir = scratch("pointer");
        write_pack(&dir, &layout(), POINTER);
        placeholder(&mut boot(&dir, None));
    }

    #[test]
    fn bad_layout_keeps_the_placeholder() {
        let dir = scratch("bad");
        let zero = "[0,0,0,0,0,0,0,0]";
        let rest = std::iter::repeat(zero).take(7).collect::<Vec<_>>().join(",");
        let bodies = [
            "not-json".to_string(),
            r#"{"cells":8,"grid":[[0]]}"#.to_string(),
            format!(r#"{{"cells":9,"indices":[{zero},{rest}]}}"#),
            format!(r#"{{"cells":8,"indices":[[16,0,0,0,0,0,0,0],{rest}]}}"#),
            format!(r#"{{"cells":8,"indices":[[0,1],{rest}]}}"#),
        ];
        for body in &bodies {
            write_pack(&dir, body, PNG_1X1);
            assert!(load_set(&dir).is_none(), "{body}");
        }
        placeholder(&mut boot(&dir, None));
    }

    #[test]
    fn low_and_mobile_keep_the_placeholder() {
        let dir = scratch("low");
        write_pack(&dir, &layout(), PNG_1X1);
        for preset in [GraphicsPreset::Low, GraphicsPreset::Mobile] {
            let mut app = boot(&dir, Some(preset));
            assert_eq!(stone_n(&mut app), 0, "{preset:?}");
            assert_ne!(yard_vis(&mut app), Visibility::Hidden, "{preset:?}");
        }
    }

    #[test]
    fn present_set_places_stone_and_other_realms_recolour() {
        let dir = scratch("present");
        write_pack(&dir, &layout(), PNG_1X1);
        let mut app = boot(&dir, Some(GraphicsPreset::High));
        let mut rows = {
            let world = app.world_mut();
            let mut tiles = world.query::<(&StoneGroundTile, &Transform, &Visibility, &MeshMaterial3d<StandardMaterial>)>();
            let mut rows: Vec<_> = tiles.iter(world).map(|(t, tf, v, m)| (t.index, tf.translation, *v, m.0.clone())).collect();
            rows.sort_by(|a, b| (a.1.z, a.1.x).partial_cmp(&(b.1.z, b.1.x)).unwrap());
            rows
        };
        assert_eq!(rows.len(), 64);
        let mut by_index: [Option<Handle<StandardMaterial>>; TILES] = std::array::from_fn(|_| None);
        for (i, (index, pos, vis, handle)) in rows.iter().enumerate() {
            let (row, col) = (i / GRID, i % GRID);
            assert_eq!((*pos, *index as usize, *vis), (cell_center(col, row), (row * GRID + col) % TILES, Visibility::Visible));
            match &by_index[*index as usize] {
                Some(prev) => assert_eq!(prev, handle),
                None => by_index[*index as usize] = Some(handle.clone()),
            }
        }
        assert_eq!(by_index.iter().flatten().count(), TILES);
        assert!(by_index.iter().flatten().enumerate().all(|(i, h)| by_index.iter().flatten().skip(i + 1).all(|o| o != h)));
        let sample = by_index[0].clone().unwrap();
        {
            let mat = app.world().resource::<Assets<StandardMaterial>>().get(&sample).unwrap();
            assert_eq!(mat.base_color, Color::WHITE);
            assert!(mat.base_color_texture.is_some() && mat.normal_map_texture.is_some());
            assert!(mat.occlusion_texture.is_none() && mat.metallic_roughness_texture.is_none());
            assert_eq!((mat.metallic, mat.perceptual_roughness, mat.flip_normal_map_y), (0.0, STONE_ROUGHNESS, false));
        }
        assert_eq!(yard_vis(&mut app), Visibility::Hidden);
        for (realm, want) in [(2u8, Color::srgb(0.20, 0.15, 0.07)), (3, Color::srgb(0.04, 0.08, 0.09))] {
            app.world_mut().resource_mut::<SoftPlayerRealm>().current = Some(realm);
            app.update();
            assert!(stone_hidden(&mut app), "{realm}");
            assert_eq!(yard_vis(&mut app), Visibility::Visible);
            assert_eq!(yard_color(&mut app), want, "{realm}");
            assert_eq!(mat_color(&mut app, &sample), Color::WHITE);
        }
        app.world_mut().resource_mut::<SoftPlayerRealm>().current = Some(0);
        app.update();
        assert_eq!(yard_vis(&mut app), Visibility::Hidden);
        assert!(!stone_hidden(&mut app));
        assert_eq!(mat_color(&mut app, &sample), Color::WHITE);
    }

    fn stone_hidden(app: &mut App) -> bool {
        let mut q = app.world_mut().query::<(&Visibility, &StoneGroundTile)>();
        q.iter(app.world()).all(|(vis, _)| *vis == Visibility::Hidden)
    }

    fn yard_color(app: &mut App) -> Color {
        let handle = {
            let mut q = app.world_mut().query::<(&Name, &MeshMaterial3d<StandardMaterial>)>();
            q.iter(app.world()).find(|(n, _)| n.as_str() == "ClimateGround").unwrap().1.0.clone()
        };
        app.world().resource::<Assets<StandardMaterial>>().get(&handle).unwrap().base_color
    }

    fn mat_color(app: &mut App, handle: &Handle<StandardMaterial>) -> Color {
        app.world().resource::<Assets<StandardMaterial>>().get(handle).unwrap().base_color
    }
}
