/*!
 * RBE Simulation Core for Powrush-MMO
 *
 * Full Render Pipeline Integration for Poisson Disk PCF
 */

use bevy::prelude::*;
use bevy::pbr::{ShadowFilteringMethod, ShadowMap};
use bevy::render::render_resource::{Shader, ShaderType, Buffer, BufferInitDescriptor, BufferUsages};
use bevy::render::renderer::RenderQueue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ... (previous structs remain)

/// Poisson Disk PCF Plugin - Handles shader loading and uniform updates
pub struct PoissonDiskPcfPlugin;

impl Plugin for PoissonDiskPcfPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PoissonDiskKernel>()
            .init_resource::<ShadowQuality>()
            .init_resource::<PoissonDiskUniformBuffer>()
            .add_systems(Startup, load_poisson_disk_shader)
            .add_systems(Update, (
                update_poisson_disk_uniform,
                update_dynamic_lighting_and_shadows,
            ));
    }
}

/// GPU buffer for Poisson Disk uniforms
#[derive(Resource, Default)]
pub struct PoissonDiskUniformBuffer {
    pub buffer: Option<Buffer>,
}

/// Load the custom Poisson Disk PCF shader
fn load_poisson_disk_shader(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shader_handle: Handle<Shader> = asset_server.load("shaders/poisson_disk_pcf.wgsl");
    commands.insert_resource(PoissonDiskShader(shader_handle));
}

#[derive(Resource)]
pub struct PoissonDiskShader(pub Handle<Shader>);

/// Update the GPU uniform buffer with current Poisson disk samples
fn update_poisson_disk_uniform(
    kernel: Res<PoissonDiskKernel>,
    mut uniform_buffer: ResMut<PoissonDiskUniformBuffer>,
    render_queue: Res<RenderQueue>,
) {
    if kernel.is_changed() || uniform_buffer.buffer.is_none() {
        let uniform = PoissonDiskUniform::from(&*kernel);

        let buffer = render_queue.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("poisson_disk_uniform_buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        uniform_buffer.buffer = Some(buffer);
    }
}

/// Main lighting + shadow update system (enhanced for custom PCF)
pub fn update_dynamic_lighting_and_shadows(
    mut query: Query<&mut DirectionalLight>,
    lighting: Res<LightingState>,
    weather: Res<WeatherState>,
    shadow_quality: Res<ShadowQuality>,
) {
    for mut light in query.iter_mut() {
        light.illuminance = lighting.light_intensity * 100_000.0;
        light.color = Color::srgb(lighting.light_color[0], lighting.light_color[1], lighting.light_color[2]);

        match weather.current {
            Weather::Clear | Weather::Heatwave => {
                light.shadows_enabled = true;

                if *shadow_quality == ShadowQuality::HighQuality {
                    // When full custom pipeline is ready, switch to:
                    // light.shadow_filtering_method = ShadowFilteringMethod::Custom;
                    // For now we use Hardware2x2 with optimized bias
                    light.shadow_filtering_method = ShadowFilteringMethod::Hardware2x2;
                    light.shadow_depth_bias = 0.012;
                    light.shadow_normal_bias = 0.4;
                } else {
                    light.shadow_filtering_method = ShadowFilteringMethod::Hardware2x2;
                    light.shadow_depth_bias = 0.015;
                    light.shadow_normal_bias = 0.5;
                }
            }
            Weather::Rain => {
                light.shadows_enabled = true;
                light.shadow_filtering_method = ShadowFilteringMethod::Hardware2x2;
                light.shadow_depth_bias = 0.04;
                light.shadow_normal_bias = 0.9;
            }
            Weather::Storm => {
                light.shadows_enabled = false;
            }
            Weather::ColdSnap => {
                light.shadows_enabled = true;
                light.shadow_filtering_method = ShadowFilteringMethod::Hardware2x2;
                light.shadow_depth_bias = 0.025;
                light.shadow_normal_bias = 0.7;
            }
        }
    }
}

// ... (rest of file)

pub struct RBESimulationPlugin;

impl Plugin for RBESimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PoissonDiskPcfPlugin);
        // ... other systems
    }
}
