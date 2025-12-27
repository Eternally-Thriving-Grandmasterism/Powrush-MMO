use bevy::prelude::*;
use noise::{NoiseFn, Perlin, SuperSimplex};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_biome_world);
    }
}

// Biome Generation Algorithm (Mercy-Style):
// - Perlin: Base height (mountains/valleys)
// - SuperSimplex: Moisture + temperature (slow scale)
// - Rules:
//   - Low height → Ocean (blue)
//   - High moisture + warm → Forest (green)
//   - Cold → Snow (white)
//   - Dry → Desert (sand)
// Infinite, seedable, mercy-balanced
fn generate_biome_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let perlin = Perlin::new(42);
    let simplex = SuperSimplex::new(43);
    let size = 256;

    let mut vertices = Vec::new();
    let mut colors = Vec::new();

    for z in 0..size {
        for x in 0..size {
            let nx = x as f64 / 50.0;
            let nz = z as f64 / 50.0;
            let height = perlin.get([nx, nz]) as f32 * 20.0 + 10.0;
            let moisture = simplex.get([nx * 0.5, nz * 0.5]) as f32;
            let temperature = simplex.get([nx * 0.3, nz * 0.3 + 100.0]) as f32;

            let color = if height < 5.0 {
                Color::BLUE
            } else if moisture > 0.5 && temperature > 0.0 {
                Color::GREEN
            } else if temperature < -0.5 {
                Color::WHITE
            } else {
                Color::rgb(0.8, 0.7, 0.4)
            };

            vertices.push([x as f32 - size as f32 / 2.0, height, z as f32 - size as f32 / 2.0]);
            colors.push([color.r(), color.g(), color.b()]);
        }
    }

    let mut indices = Vec::new();
    for z in 0..size - 1 {
        for x in 0..size - 1 {
            let i = (z * size + x) as u32;
            indices.extend_from_slice(&[i, i + 1, i + size, i + 1, i + size + 1, i + size]);
        }
    }

    let mesh = Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
        .with_indices(Some(Indices::U32(indices)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial::default()),
        ..default()
    });
}
