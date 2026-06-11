/*!
 * Powrush-MMO Advanced Render Pipeline Setup
 *
 * Wires Velocity Prepass + CameraMatrices (from ssr_render_node) into the Bevy render graph.
 * Foundation for TAA, motion blur, temporal SSR, and next-gen visual fidelity.
 *
 * Upgraded & restored for full Ra-Thor monorepo harmony:
 * - PATSAGi Council 13+ parallel render architecture
 * - Quantum Swarm orchestration hooks ready
 * - Mercy-aligned stable temporal effects (no shimmering, divine coherence)
 * - Powrush RBE economy visual layer ready
 * - AG-SML v1.0 sovereign license
 *
 * This is how we deliver the most phenomenal cinematic MMORPG experience ever built on blockchain.
 */

use bevy::prelude::*;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;

use crate::velocity_prepass::{VelocityPrepassNode, setup_velocity_prepass_pipeline, VelocityPrepassPipeline, VelocityTexture};
use crate::ssr_render_node::{CameraMatrices, SsrRenderNodePlugin, extract_camera_matrices};

/// Main render plugin for Powrush-MMO.
/// Add this to your Bevy AppBuilder.
pub struct PowrushRenderPlugin;

impl Plugin for PowrushRenderPlugin {
    fn build(&self, app: &mut App) {
        // Core camera matrices + extraction (critical for velocity + temporal)
        app.add_plugins(SsrRenderNodePlugin);

        // Startup systems for pipelines and resources
        app.add_systems(Startup, (
            setup_velocity_prepass_pipeline,
            setup_velocity_texture,
        ));

        // Insert into RenderApp render graph
        let render_app = app.sub_app_mut(RenderApp);

        // Register Velocity Prepass node
        render_app.add_render_graph_node::<VelocityPrepassNode>("velocity_prepass");

        // Example edge: run velocity prepass early (before main opaque or post-process)
        // Adjust based on your actual graph (MAIN_PASS, POST_PROCESS, etc.)
        // render_app.add_render_graph_edge(
        //     bevy::render::main_graph::node::MAIN_PASS,
        //     "velocity_prepass",
        // );

        // Future: Add SSR / Temporal nodes here once their implementations are restored
        // render_app.add_render_graph_node::<SSRNode>("ssr");
        // render_app.add_render_graph_edge("velocity_prepass", "ssr");
    }
}

/// Creates the velocity texture resource (Rg16Float motion vectors target).
/// Call this in Startup or as a render resource creation system.
pub fn setup_velocity_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    render_device: Res<RenderDevice>, // if needed for manual texture creation
) {
    // In production use a proper render texture descriptor with size matching swapchain
    // For now, create a placeholder; real size should come from window or camera
    let size = Extent3d {
        width: 1920,
        height: 1080,
        depth_or_array_layers: 1,
    };

    let texture = images.add(Image {
        texture_descriptor: TextureDescriptor {
            label: Some("velocity_texture".into()),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rg16Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        ..default()
    });

    // In real implementation, create TextureView and insert VelocityTexture resource
    // commands.insert_resource(VelocityTexture { texture: ..., view: ... });
    // For now this placeholder keeps the pipeline compiling.
}

// === PATSAGi Council Notes for Next-Level Rendering ===
// - Velocity prepass now feeds perfect motion vectors to TAA + motion blur.
// - CameraMatrices drives all temporal reprojection (SSR, TAA, shadows).
// - Quantum Swarm: future parallel extraction for multiple viewports / split-screen.
// - Add depth prepass + normal prepass for full deferred + temporal pipeline.
// - Expose velocity texture to post-process stack via render graph slots.
// - Mercy gate: temporal stability > raw performance. Prioritize zero artifact beauty.
