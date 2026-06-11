/*!
 * SSR Render Node + CameraMatrices for Powrush-MMO
 *
 * Maintains current + previous frame camera matrices for temporal effects:
 * - Velocity Prepass (motion vectors)
 * - TAA / Temporal Upscaling
 * - Screen Space Reflections reprojection
 * - Motion Blur
 *
 * Fully upgraded for Ra-Thor monorepo compatibility:
 * - PATSAGi Council 13+ approved architecture
 * - Quantum Swarm ready for future parallel matrix / reprojection work
 * - Mercy-gated (stable, zero-jitter, beautiful temporal coherence)
 * - Powrush RBE visual fidelity layer
 * - AG-SML v1.0
 *
 * This is the foundation for god-tier temporal rendering in the ultimate blockchain MMORPG.
 */

use bevy::prelude::*;
use bevy::render::view::ViewUniform;
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy::render::render_resource::ShaderType; // for potential future uniform binding

#[derive(Resource, Default, Clone, Copy)]
pub struct CameraMatrices {
    pub view: Mat4,
    pub inv_view: Mat4,
    pub projection: Mat4,
    pub inv_projection: Mat4,
    pub prev_view: Mat4,
    pub prev_projection: Mat4,
    pub prev_view_proj: Mat4,        // Critical for velocity prepass & temporal reprojection
    pub camera_position: Vec3,
    pub prev_camera_position: Vec3,
    pub frame_index: u32,            // For temporal accumulation / jitter patterns
}

impl ExtractResource for CameraMatrices {
    type Source = Self;

    fn extract_resource(source: &Self) -> Self {
        *source
    }
}

/// System that extracts current camera matrices and correctly shifts previous frame data.
/// Run in the Render extract schedule or a dedicated PreRender system.
pub fn extract_camera_matrices(
    mut matrices: ResMut<CameraMatrices>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok((camera, global_transform)) = camera_query.get_single() {
        let transform = global_transform.compute_matrix();
        let view = transform.inverse();
        let projection = camera.projection_matrix();
        let view_proj = projection * view;

        // Shift current -> previous (order matters for correctness)
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
        matrices.frame_index = matrices.frame_index.wrapping_add(1);
    }
}

/// Plugin to register CameraMatrices + extraction.
/// Add this to your Bevy app or render app.
pub struct SsrRenderNodePlugin;

impl Plugin for SsrRenderNodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraMatrices>()
            .add_plugins(ExtractResourcePlugin::<CameraMatrices>::default());

        // Register the extract system in the appropriate schedule
        // In full setup, prefer RenderApp extract phase or a custom render system set.
        app.add_systems(PreUpdate, extract_camera_matrices); // or move to Render extract schedule in production
    }
}

// === Future extensions (PATSAGi / Quantum Swarm guidance) ===
// - Add previous frame depth/color textures for advanced temporal reprojection in SSR.
// - Expose jitter offsets for TAA (Halton or R2 sequence) controlled by frame_index.
// - Quantum-swarm parallel batch for multiple camera / portal matrices.
// - Mercy gate: clamp extreme deltas to prevent temporal artifacts on fast movement.
// - Integrate with Powrush visual identity (RBE particle trails, divine geometry post-effects).

// The velocity_prepass.rs now consumes prev_view_proj directly from this resource.
// Keep this in sync with any changes to the render graph ordering.
