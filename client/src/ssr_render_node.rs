/*!
 * SSR Render Node + CameraMatrices for Powrush-MMO
 *
 * Refined temporal camera jitter with more precise NDC-space application.
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

    // Temporal Jitter (in NDC space)
    pub jitter: Vec2,
    pub prev_jitter: Vec2,
}

impl ExtractResource for CameraMatrices {
    type Source = Self;

    fn extract_resource(source: &Self) -> Self {
        *source
    }
}

/// Generates a 2D Halton sequence (base 2 and 3) for high-quality temporal sampling.
fn halton_2d(index: u32) -> Vec2 {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut fx = 1.0f32;
    let mut fy = 1.0f32;
    let mut i = index;

    // Base 2 for X
    fx = 0.5;
    while i > 0 {
        if i & 1 == 1 {
            x += fx;
        }
        fx *= 0.5;
        i >>= 1;
    }

    // Base 3 for Y
    i = index;
    fy = 1.0 / 3.0;
    while i > 0 {
        if i % 3 == 1 {
            y += fy;
        }
        fy /= 3.0;
        i /= 3;
    }

    // Return in [-0.5, 0.5] NDC range
    Vec2::new(x - 0.5, y - 0.5)
}

/// Applies sub-pixel jitter to a projection matrix in NDC space.
/// This is the standard precise method used in most TAA implementations.
fn apply_jitter_to_projection(projection: Mat4, jitter: Vec2) -> Mat4 {
    let mut p = projection;

    // Jitter is applied by offsetting the third column (translation in clip space).
    // This shifts the frustum slightly without changing near/far planes significantly.
    // The values are in NDC [-1, 1] range.
    p.x_axis.z += jitter.x;
    p.y_axis.z += jitter.y;

    p
}

/// System that applies temporal jitter to the camera for TAA.
pub fn apply_temporal_jitter(
    mut matrices: ResMut<CameraMatrices>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok((camera, global_transform)) = camera_query.get_single() {
        let transform = global_transform.compute_matrix();
        let view = transform.inverse();
        let base_projection = camera.projection_matrix();

        // Generate high-quality jitter offset
        let jitter = halton_2d(matrices.frame_index + 1);

        // Apply jitter to get the jittered projection for this frame
        let jittered_projection = apply_jitter_to_projection(base_projection, jitter);

        let view_proj = jittered_projection * view;

        // Store previous state (including previous jitter)
        matrices.prev_jitter = matrices.jitter;
        matrices.prev_view = matrices.view;
        matrices.prev_projection = matrices.projection;
        matrices.prev_view_proj = matrices.projection * matrices.view;
        matrices.prev_camera_position = matrices.camera_position;

        // Update current frame
        matrices.view = view;
        matrices.inv_view = transform;
        matrices.projection = jittered_projection;
        matrices.inv_projection = jittered_projection.inverse();
        matrices.camera_position = global_transform.translation();
        matrices.jitter = jitter;
        matrices.frame_index = matrices.frame_index.wrapping_add(1);
    }
}

pub struct SsrRenderNodePlugin;

impl Plugin for SsrRenderNodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraMatrices>()
            .add_plugins(ExtractResourcePlugin::<CameraMatrices>::default())
            .add_systems(PreUpdate, apply_temporal_jitter);
    }
}
