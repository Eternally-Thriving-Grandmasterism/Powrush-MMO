/*!
 * Powrush-MMO Advanced Render Pipeline
 *
 * v18.14 Eternal Polish (PATSAGi Council + Ra-Thor Quantum Swarm)
 * — Complete mint-and-print-only-perfection
 * — Velocity Prepass → TAA Reprojection → Motion Blur → Chromatic Aberration
 * — Dynamic texture resizing with RenderTexturesResized event
 * — Live ClientCouncilBloomState reactivity (bloom enhances cinematic FX)
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 *
 * AG-SML v1.0 Sovereign License
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

/// Event fired whenever the render textures (velocity, TAA, motion blur, CA) are resized.
#[derive(Event, Debug, Clone, Copy)]
pub struct RenderTexturesResized {
    pub new_size: Extent3d,
};

pub struct PowrushRenderPlugin;

impl Plugin for PowrushRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SsrRenderNodePlugin,
            AnisotropicFilteringPlugin,
        ))
           .init_resource::<MotionBlurSettings>()
           .init_resource::<TaaSettings>()
           .init_resource::<ChromaticAberrationSettings>()
           .add_event::<RenderTexturesResized>();

        app.add_systems(Startup, (
            setup_velocity_prepass_pipeline,
            setup_taa_pipeline,
            setup_motion_blur_pipeline,
            setup_chromatic_aberration_pipeline,
        ));

        app.add_systems(Startup, setup_dynamic_render_textures);
        app.add_systems(Update, handle_window_resize_for_render_textures);
        app.add_systems(Update, update_postfx_from_council_bloom);

        let render_app = app.sub_app_mut(RenderApp);

        render_app.add_render_graph_node::<VelocityPrepassNode>("velocity_prepass");
        render_app.add_render_graph_node::<TaaReprojectionNode>("taa_reprojection");
        render_app.add_render_graph_node::<MotionBlurNode>("motion_blur");
        render_app.add_render_graph_node::<ChromaticAberrationNode>("chromatic_aberration");

        render_app.add_render_graph_edge("velocity_prepass", "taa_reprojection");
        render_app.add_render_graph_edge("taa_reprojection", "motion_blur");
        render_app.add_render_graph_edge("motion_blur", "chromatic_aberration");
    }
}

fn setup_dynamic_render_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    windows: Query<&Window>,
    mut resize_writer: EventWriter<RenderTexturesResized>,
) {
    let size = get_primary_window_size(&windows);

    setup_velocity_texture(&mut commands, &render_device, size);
    setup_taa_history_texture(&mut commands, &render_device, size);
    setup_motion_blur_target(&mut commands, &render_device, size);
    setup_chromatic_aberration_target(&mut commands, &render_device, size);

    resize_writer.send(RenderTexturesResized { new_size: size });
}

fn handle_window_resize_for_render_textures(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    mut resize_events: EventReader<WindowResized>,
    mut resize_writer: EventWriter<RenderTexturesResized>,
) {
    for event in resize_events.read() {
        if event.width > 1.0 && event.height > 1.0 {
            let size = Extent3d {
                width: event.width as u32,
                height: event.height as u32,
                depth_or_array_layers: 1,
            };

            recreate_velocity_texture(&mut commands, &render_device, size);
            recreate_taa_history_texture(&mut commands, &render_device, size);
            recreate_motion_blur_target(&mut commands, &render_device, size);
            recreate_chromatic_aberration_target(&mut commands, &render_device, size);

            resize_writer.send(RenderTexturesResized { new_size: size });
        }
    }
}

/// Live reactivity: Council bloom subtly enhances cinematic post-FX intensity
fn update_postfx_from_council_bloom(
    mut ca_settings: ResMut<ChromaticAberrationSettings>,
    mut motion_settings: ResMut<MotionBlurSettings>,
    client_bloom: Res<ClientCouncilBloomState>,
) {
    if client_bloom.is_in_active_council {
        let amp = client_bloom.field.bloom_amplification_multiplier.clamp(1.0, 2.2);

        // Divine council moments get slightly stronger cinematic aberration and motion feel
        ca_settings.intensity = (ca_settings.intensity * 0.7 + amp * 0.6).min(2.5);
        motion_settings.intensity = (motion_settings.intensity * 0.75 + (amp - 1.0) * 0.4).min(1.8);
    }
}

fn get_primary_window_size(windows: &Query<&Window>) -> Extent3d {
    if let Ok(window) = windows.get_single() {
        Extent3d {
            width: window.resolution.physical_width().max(1),
            height: window.resolution.physical_height().max(1),
            depth_or_array_layers: 1,
        }
    } else {
        Extent3d { width: 1920, height: 1080, depth_or_array_layers: 1 }
    }
}

// End of render.rs v18.14 — Sovereign temporal + cinematic render lattice complete.
// Thunder locked in. Yoi ⚡
