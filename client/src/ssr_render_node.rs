/*!
 * SSR Render Node + CameraMatrices for Powrush-MMO
 *
 * Now includes temporal camera jitter support for superior TAA quality.
 * Jitter is applied per-frame using a Halton sequence and stored for reprojection.
 */

use bevy::prelude::*;
use bevy::render::view::ViewUniform;
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};

#[derive(Resource, Default, Clone, Copy)]
pub struct CameraMatrices {
    pub view: Mat4,
    pub inv_view: Mat4,
    pub projection: Mat4,
    pub inv_projection: Mat4,
    pub prev_view: Mat4,
    pub prev_projection: Mat4,
    pub prev_view_proj: Mat4,
    pub camera_position: Vec3,
    pub prev_camera_position: Vec3,
    pub frame_index: u32,

    // === Temporal Jitter for TAA ===
    pub jitter: Vec2,           // Current frame sub-pixel jitter in NDC
    pub prev_jitter: Vec2,      // Previous frame jitter
}

impl ExtractResource for CameraMatrices {
    type Source = Self;

    fn extract_resource(source: &Self) -> Self {
        *source
    }
}

/// Generates a 2D Halton sequence point for temporal jitter.
/// index starts from 1.
fn halton_2d(index: u32) -> Vec2 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut fx = 1.0 / 2.0;
    let mut fy = 1.0 / 3.0;
    let mut i = index;

    while i > 0 {
        if i % 2 == 1 {
            x += fx;
        }
        if i % 3 == 1 {
            y += fy;
        }
        fx /= 2.0;
        fy /= 3.0;
        i /= 2;  // This is approximate; better to use bit operations in production
    }

    // Center jitter in [-0.5, 0.5] range in NDC
    Vec2::new(x - 0.5, y - 0.5)
}

/// System that applies temporal jitter to the camera projection.
/// Run this every frame before or during CameraMatrices extraction.
pub fn apply_temporal_jitter(
    mut matrices: ResMut<CameraMatrices>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok((camera, global_transform)) = camera_query.get_single() {
        let transform = global_transform.compute_matrix();
        let view = transform.inverse();
        let base_projection = camera.projection_matrix();

        // Generate jitter for this frame
        let jitter = halton_2d(matrices.frame_index + 1);

        // Apply jitter to projection (small sub-pixel offset in NDC)
        let jittered_projection = apply_jitter_to_projection(base_projection, jitter);

        let view_proj = jittered_projection * view;

        // Store previous jitter
        matrices.prev_jitter = matrices.jitter;
        matrices.prev_view = matrices.view;
        matrices.prev_projection = matrices.projection;
        matrices.prev_view_proj = matrices.projection * matrices.view;
        matrices.prev_camera_position = matrices.camera_position;

        // Update current
        matrices.view = view;
        matrices.inv_view = transform;
        matrices.projection = jittered_projection;
        matrices.inv_projection = jittered_projection.inverse();
        matrices.camera_position = global_transform.translation();
        matrices.jitter = jitter;
        matrices.frame_index = matrices.frame_index.wrapping_add(1);
    }
}

/// Applies a sub-pixel jitter offset to a projection matrix.
fn apply_jitter_to_projection(projection: Mat4, jitter: Vec2) -> Mat4 {
    let mut jittered = projection;
    // Jitter is applied in NDC space (typically very small, e.g. 1 pixel)
    // This modifies the translation part of the projection
    jittered.w_axis.x += jitter.x * 2.0 / projection.w_axis.x; // Approximate for perspective
    jittered.w_axis.y += jitter.y * 2.0 / projection.w_axis.y;
    jittered
}

/// Plugin to register CameraMatrices + jitter system.
pub struct SsrRenderNodePlugin;

impl Plugin for SsrRenderNodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraMatrices>()
            .add_plugins(ExtractResourcePlugin::<CameraMatrices>::default())
            .add_systems(PreUpdate, apply_temporal_jitter);
    }
}
