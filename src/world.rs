use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_procedural_world);
    }
}

fn generate_procedural_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let perlin = Perlin::new(42);
    let size = 256;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for z in 0..size {
        for x in 0..size {
            let height = perlin.get([x as f64 / 50.0, z as f64 / 50.0]) as f32 * 20.0 + 10.0;
            vertices.push([x as f32 - size as f32 / 2.0, height, z as f32 - size as f32 / 2.0]);
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
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        }),
        ..default()
    });

    info!("Procedural world generated — mercy biomes ready");
}
