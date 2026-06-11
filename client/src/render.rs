//! client/src/render.rs
//! Powrush-MMO Rendering Pipeline — WebGPU + mercy-gated visuals

use bevy::prelude::*;
use bevy::render::render_graph::{RenderGraph, RenderGraphApp};
use bevy::render::RenderApp;

use crate::ssr_render_node::{SSRNode, SSRPipeline, SSRUniformBuffer, setup_ssr_pipeline, SSRSettings};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SSRSettings>()
            .add_systems(Startup, setup_ssr_pipeline)
            .add_plugins(SSRNodePlugin); // We'll define this below or inline

        // Insert SSR node into the render graph
        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_render_graph_node::<SSRNode>("ssr_post_process");
        
        // Connect after main 3D pass
        render_app.add_render_graph_edge(
            bevy::render::main_graph::node::MAIN_PASS,
            "ssr_post_process",
        );

        // Optional: connect before UI or final output if needed
        // render_app.add_render_graph_edge("ssr_post_process", bevy::render::main_graph::node::UI_PASS);
    }
}

// Simple plugin to register the SSR node type
pub struct SSRNodePlugin;

impl Plugin for SSRNodePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SSRPipeline>()
            .init_resource::<SSRUniformBuffer>();
    }
}

// Existing mercy-gated rendering code...
fn setup_render_pipeline(mut commands: Commands) {
    info!("Powrush-MMO render pipeline initialized — mercy visuals awakening ⚡️");
}

fn update_mercy_gated_rendering(
    mut query: Query<&mut crate::particles::ParticleSystem>,
    time: Res<Time>,
) {
    for mut system in &mut query {
        if system.valence >= 0.999999 {
            system.particle_count = (system.particle_count as f32 * 1.618).min(32768.0) as u32;
        }
    }
}
