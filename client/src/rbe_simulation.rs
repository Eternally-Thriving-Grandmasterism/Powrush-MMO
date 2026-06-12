/*!
 * RBE Simulation Core for Powrush-MMO
 *
 * Poisson Disk PCF - Rust Side Integration
 */

use bevy::prelude::*;
use bevy::pbr::ShadowFilteringMethod;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ... (previous code remains the same up to LightingState)

/// Poisson Disk PCF Kernel Resource (CPU side)
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct PoissonDiskKernel {
    pub samples: Vec<[f32; 2]>,
}

impl Default for PoissonDiskKernel {
    fn default() -> Self {
        let samples = vec![
            [0.0589, 0.1285], [-0.0213, -0.3923], [0.3125, -0.2891],
            [-0.3412, 0.1567], [0.1897, 0.4123], [-0.4128, -0.0892],
            [0.2671, -0.1564], [-0.0891, 0.3124], [0.4123, 0.0891],
            [-0.1564, -0.2671], [0.0892, -0.4128], [-0.3124, 0.0891],
            [0.1567, 0.3412], [-0.2891, -0.3125], [0.1285, 0.0589],
            [-0.3923, 0.0213],
        ];
        Self { samples }
    }
}

/// GPU-ready uniform for the Poisson Disk shader
#[derive(Clone, Copy, ShaderType)]
pub struct PoissonDiskUniform {
    pub samples: [Vec2; 16],
    pub sample_count: u32,
    pub _padding: [u32; 3],
}

impl From<&PoissonDiskKernel> for PoissonDiskUniform {
    fn from(kernel: &PoissonDiskKernel) -> Self {
        let mut samples = [Vec2::ZERO; 16];
        for (i, &s) in kernel.samples.iter().enumerate().take(16) {
            samples[i] = Vec2::new(s[0], s[1]);
        }
        Self {
            samples,
            sample_count: kernel.samples.len() as u32,
            _padding: [0; 3],
        }
    }
}

/// Shadow Quality Mode
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadowQuality {
    Performance,
    HighQuality, // Uses Poisson Disk PCF when fully integrated
}

impl Default for ShadowQuality {
    fn default() -> Self {
        ShadowQuality::Performance
    }
}

/// System that prepares Poisson Disk uniforms and updates shadow settings
pub fn update_poisson_disk_shadows(
    mut query: Query<&mut DirectionalLight>,
    lighting: Res<LightingState>,
    weather: Res<WeatherState>,
    shadow_quality: Res<ShadowQuality>,
    kernel: Res<PoissonDiskKernel>,
) {
    for mut light in query.iter_mut() {
        light.illuminance = lighting.light_intensity * 100_000.0;
        light.color = Color::srgb(lighting.light_color[0], lighting.light_color[1], lighting.light_color[2]);

        match weather.current {
            Weather::Clear | Weather::Heatwave => {
                light.shadows_enabled = true;

                if *shadow_quality == ShadowQuality::HighQuality {
                    // When fully integrated, this would switch to a custom
                    // ShadowFilteringMethod that uses poisson_disk_pcf.wgsl
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

// ... (rest of the file remains the same)

pub struct RBESimulationPlugin;

impl Plugin for RBESimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbundancePool>()
            .init_resource::<BiomeWeights>()
            .init_resource::<WeatherState>()
            .init_resource::<LightingState>()
            .init_resource::<PoissonDiskKernel>()
            .init_resource::<ShadowQuality>()
            .add_event::<GatherFromNodeEvent>()
            .add_event::<ResourceDepositedEvent>()
            .add_systems(Update, (
                rbe_simulation_step,
                process_contribution_actions,
                regenerate_resource_nodes,
                handle_gather_from_node,
                deposit_visual_feedback,
                cleanup_deposit_effects,
                update_poisson_disk_shadows, // Updated system
            ));
    }
}
