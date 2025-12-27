use bevy::prelude::*;

pub struct LODPlugin;

impl Plugin for LODPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lod_system);
    }
}

fn lod_system(
    camera: Query<&Transform, With<Camera>>,
    mut query: Query<(&mut Visible, &Transform), With<LatticeNode>>,
) {
    let cam_pos = camera.single().translation;
    for (mut visible, trans) in &mut query {
        let dist = cam_pos.distance(trans.translation);
        visible.is_visible = dist < 100.0;  // High detail
        // Low detail: hide or swap mesh at >100
    }
}
