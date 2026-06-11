/*!
 * Simple system to store previous frame transforms for temporal techniques.
 * Start with camera (already handled in CameraMatrices) and add objects as needed.
 */

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PreviousTransforms {
    // For now we focus on camera. Add HashMap<Entity, Mat4> or similar for objects later.
    pub camera_view: Mat4,
    pub camera_projection: Mat4,
}

/// Run this every frame (before rendering) to store previous camera data.
pub fn store_previous_camera(
    mut prev: ResMut<PreviousTransforms>,
    matrices: Res<CameraMatrices>,
) {
    prev.camera_view = matrices.view;
    prev.camera_projection = matrices.projection;
}

// Example for objects (add this when you have a list of entities that need velocity):
// #[derive(Component)]
// pub struct PreviousGlobalTransform(pub GlobalTransform);
//
// pub fn store_previous_transforms(
//     mut query: Query<(&GlobalTransform, &mut PreviousGlobalTransform)>,
// ) {
//     for (current, mut previous) in &mut query {
//         previous.0 = *current;
//     }
// }
