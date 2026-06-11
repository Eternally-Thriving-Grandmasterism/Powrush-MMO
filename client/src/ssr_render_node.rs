/*!
 * CameraMatrices now stores previous view_proj for velocity prepass temporal accuracy.
 */

use bevy::prelude::*;
use bevy::render::view::ViewUniform;

#[derive(Resource, Default)]
pub struct CameraMatrices {
    pub view: Mat4,
    pub inv_view: Mat4,
    pub projection: Mat4,
    pub inv_projection: Mat4,
    pub prev_view: Mat4,
    pub prev_projection: Mat4,
    pub prev_view_proj: Mat4,        // NEW: for velocity prepass
    pub camera_position: Vec3,
    pub prev_camera_position: Vec3,
}

/// Extracts current camera matrices and stores previous frame data.
pub fn extract_camera_matrices(
    mut matrices: ResMut<CameraMatrices>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok((camera, global_transform)) = camera_query.get_single() {
        let transform = global_transform.compute_matrix();
        let view = transform.inverse();
        let projection = camera.projection_matrix();
        let view_proj = projection * view;

        // Store previous frame data
        matrices.prev_view = matrices.view;
        matrices.prev_projection = matrices.projection;
        matrices.prev_view_proj = matrices.projection * matrices.view; // previous view_proj
        matrices.prev_camera_position = matrices.camera_position;

        // Update current frame
        matrices.view = view;
        matrices.inv_view = transform;
        matrices.projection = projection;
        matrices.inv_projection = projection.inverse();
        matrices.camera_position = global_transform.translation();
    }
}
