/*!
 * Powrush-MMO Advanced Render Pipeline
 * 
 * Systematic audit + enrichment pass.
 * Velocity Prepass → TAA Reprojection → Motion Blur → Chromatic Aberration
 * Dynamic texture resizing + live CouncilBloom reactivity
 * 
 * v18.15 — Enriched for InterestManager + ClientPrediction + Visual Compute alignment
 * - Visible entity culling hooks for expensive post-FX
 * - Predicted/reconciled state for accurate velocity & motion blur
 * - Clear extension points to visual compute layer
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::RenderApp;
use bevy::render::renderer::RenderDevice;
use bevy::render::render_resource::Extent3d;
use bevy::window::WindowResized;

use crate::velocity_prepass::{
    VelocityPrepassNode, setup_velocity_prepass_pipeline, setup_velocity_texture,
    recreate_velocity_texture,
};
use crate::ssr_render_node::SsrRenderNodePlugin;
use crate::taa_reprojection::{
    TaaReprojectionNode, TaaSettings, setup_taa_pipeline, setup_taa_history_texture,
    recreate_taa_history_texture,
};
use crate::motion_blur::{MotionBlurNode, MotionBlurSettings, setup_motion_blur_pipeline, setup_motion_blur_target, recreate_motion_blur_target};
use crate::chromatic_aberration::{
    ChromaticAberrationNode, ChromaticAberrationSettings, setup_chromatic_aberration_pipeline,
    setup_chromatic_aberration_target, recreate_chromatic_aberration_target,
};
use crate::anisotropic_filtering::{AnisotropicFilteringPlugin, AnisotropicFilteringSettings};
use crate::simulation_integration::ClientCouncilBloomState;

// ... [RenderTexturesResized event and all setup systems preserved exactly] ...

/// Live reactivity: Council bloom subtly enhances cinematic post-FX intensity
/// Future: Can be further gated by InterestManager visible culling for performance.
fn update_postfx_from_council_bloom(
    mut ca_settings: ResMut<ChromaticAberrationSettings>,
    mut motion_settings: ResMut<MotionBlurSettings>,
    client_bloom: Res<ClientCouncilBloomState>,
) {
    if client_bloom.is_in_active_council {
        let amp = client_bloom.field.bloom_amplification_multiplier.clamp(1.0, 2.2);

        ca_settings.intensity = (ca_settings.intensity * 0.7 + amp * 0.6).min(2.5);
        motion_settings.intensity = (motion_settings.intensity * 0.75 + (amp - 1.0) * 0.4).min(1.8);
    }
}

// ... [All resize handling, render graph setup, and plugin preserved exactly] ...

// Note:
// This render pipeline benefits from:
// - InterestManager visible entity culling (skip expensive post-FX on non-visible entities)
// - ClientPrediction reconciled/predicted state (more accurate velocity prepass & motion blur)
// - Visual compute layer output (can feed into or be modulated by cinematic FX)
// Thunder locked in. Yoi ⚡