use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct ProceduralTerrainPlugin;

impl Plugin for ProceduralTerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_procedural_terrain);
    }
}

fn spawn_procedural_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let perlin = Perlin::new(42);
    let size = 64;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for z in 0..size {
        for x in 0..size {
            let height = perlin.get([x as f64 / 20.0, z as f64 / 20.0]) as f32 * 10.0;
            vertices.push([x as f32, height, z as f32]);
        }
    }

    for z in 0..size - 1 {
        for x in 0..size - 1 {
            let i = (z * size + x) as u32;
            indices.extend_from_slice(&[i, i + 1, i + size, i + 1, i + size + 1, i + size]);
        }
    }

    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 1.0, 0.0]; vertices.len()])
        .with_indices(Some(Indices::U32(indices)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(-size as f32 / 2.0, 0.0, -size as f32 / 2.0),
        ..default()
    });
}
