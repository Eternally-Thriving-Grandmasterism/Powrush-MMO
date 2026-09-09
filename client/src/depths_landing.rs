//! U11 — Depths landing. One Peace disk. Spawns only on Depths. No Take. No listen.

use bevy::prelude::*;

use shared::hex_travel::{depths_is_market, depths_is_one_turn, depths_mesh_on_sanctuary, PlaceId};

use crate::hex_travel::HexTravelState;

#[derive(Component)]
struct DepthsLanding;

pub struct DepthsLandingPlugin;

impl Plugin for DepthsLandingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_depths_landing);
    }
}

fn sync_depths_landing(
    mut commands: Commands,
    travel: Res<HexTravelState>,
    existing: Query<Entity, With<DepthsLanding>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let here = travel.current == PlaceId::Depths;
    if !here {
        for entity in &existing {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }
    if !existing.is_empty() {
        return;
    }
    debug_assert!(depths_is_one_turn());
    debug_assert!(!depths_is_market());
    debug_assert!(!depths_mesh_on_sanctuary());
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cylinder::new(1.6, 0.05)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.12, 0.16, 0.22),
                emissive: LinearRgba::new(0.01, 0.02, 0.03, 1.0),
                perceptual_roughness: 0.9,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.02, 0.0)),
            ..default()
        },
        DepthsLanding,
        Name::new("DepthsLanding"),
    ));
}
