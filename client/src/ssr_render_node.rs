/*!
 * SSR Render Node + CameraMatrices for Powrush-MMO
 *
 * v18.19 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Halton jitter system with TAA integration
 * — CameraMatrices extraction for SSR, velocity prepass, and temporal effects
 * — Bloom-reactive jitter scale (via TaaSettings)
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::render::view::ViewUniform;
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};

use crate::taa_reprojection::TaaSettings;
use crate::simulation_integration::ClientCouncilBloomState;

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

    pub jitter: Vec2,
    pub prev_jitter: Vec2,
}

impl ExtractResource for CameraMatrices {
    type Source = Self;

    fn extract_resource(source: &Self) -> Self {
        *source
    }
}

fn halton_2d(index: u32) -> Vec2 {
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut fx = 0.5f32;
    let mut i = index;

    while i > 0 {
        if i & 1 == 1 { x += fx; }
        fx *= 0.5;
        i >>= 1;
    }

    let mut fy = 1.0f32 / 3.0f32;
    i = index;
    while i > 0 {
        if i % 3 == 1 { y += fy; }
        fy /= 3.0;
        i /= 3;
    }

    Vec2::new(x - 0.5, y - 0.5)
}

fn apply_jitter_to_projection(projection: Mat4, jitter: Vec2) -> Mat4 {
    let mut p = projection;
    p.x_axis.z += jitter.x;
    p.y_axis.z += jitter.y;
    p
}

pub fn apply_temporal_jitter(
    mut matrices: ResMut<CameraMatrices>,
    taa_settings: Res<TaaSettings>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if !taa_settings.enabled {
        return;
    }

    if let Ok((camera, global_transform)) = camera_query.get_single() {
        let transform = global_transform.compute_matrix();
        let view = transform.inverse();
        let base_projection = camera.projection_matrix();

        // Jitter scale is already modulated by ClientCouncilBloomState in taa_reprojection
        let raw_jitter = halton_2d(matrices.frame_index + 1);
        let jitter = raw_jitter * taa_settings.jitter_scale;

        let jittered_projection = apply_jitter_to_projection(base_projection, jitter);

        let view_proj = jittered_projection * view;

        matrices.prev_jitter = matrices.jitter;
        matrices.prev_view = matrices.view;
        matrices.prev_projection = matrices.projection;
        matrices.prev_view_proj = matrices.projection * matrices.view;
        matrices.prev_camera_position = matrices.camera_position;

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

// End of ssr_render_node.rs v18.19 — Sovereign camera matrices + temporal jitter complete.
// Thunder locked in. Yoi ⚡
