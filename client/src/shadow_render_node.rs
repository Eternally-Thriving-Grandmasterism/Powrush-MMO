/*!
 * shadow_render_node.rs
 * Powrush-MMO — Temporal Poisson Disk with Rotated Kernel Sequence
 */

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext},
    renderer::RenderContext,
    RenderApp,
};
use bevy::pbr::ShadowPass;
use bevy::render::render_resource::Shader;
use std::f32::consts::PI;

// ... (previous enums and resources remain)

/// Temporal Poisson Disk Kernel Sequence
///
/// Holds multiple rotated versions of a base Poisson disk kernel.
/// This provides excellent temporal stability when combined with TAA.
#[derive(Resource)]
pub struct TemporalPoissonDisk {
    pub kernels: Vec<PoissonDiskKernel>,
    pub current_index: usize,
}

impl TemporalPoissonDisk {
    /// Create a sequence of rotated kernels from a base kernel
    pub fn from_base_kernel(base: &PoissonDiskKernel, count: usize) -> Self {
        let mut kernels = Vec::with_capacity(count);

        for i in 0..count {
            let angle = (i as f32 / count as f32) * 2.0 * PI;
            let cos = angle.cos();
            let sin = angle.sin();

            let rotated_samples: Vec<[f32; 2]> = base
                .samples
                .iter()
                .map(|&[x, y]| {
                    // Rotate around origin
                    let rx = x * cos - y * sin;
                    let ry = x * sin + y * cos;
                    [rx, ry]
                })
                .collect();

            kernels.push(PoissonDiskKernel {
                samples: rotated_samples,
            });
        }

        Self {
            kernels,
            current_index: 0,
        }
    }

    /// Get the kernel for the current frame and advance the index
    pub fn next_kernel(&mut self) -> &PoissonDiskKernel {
        let kernel = &self.kernels[self.current_index];
        self.current_index = (self.current_index + 1) % self.kernels.len();
        kernel
    }

    /// Get current kernel without advancing (useful for debugging)
    pub fn current_kernel(&self) -> &PoissonDiskKernel {
        &self.kernels[self.current_index]
    }
}

impl Default for TemporalPoissonDisk {
    fn default() -> Self {
        // Create from the default PoissonDiskKernel with 12 rotations
        let base = PoissonDiskKernel::default();
        Self::from_base_kernel(&base, 12)
    }
}

/// System that updates shadow sampling to use temporal Poisson Disk when high quality is enabled
pub fn update_temporal_poisson_disk_shadows(
    mut shadow_quality: ResMut<ShadowQualityState>,
    mut temporal: ResMut<TemporalPoissonDisk>,
    mut shadow_filtering: ResMut<ShadowFilteringMethod>,
) {
    if shadow_quality.is_high_quality {
        if *shadow_filtering != ShadowFilteringMethod::PoissonDisk {
            *shadow_filtering = ShadowFilteringMethod::PoissonDisk;
        }

        // Advance to next rotated kernel every frame
        let _current_kernel = temporal.next_kernel();

        // In a full implementation, you would now bind _current_kernel
        // to the GPU uniform buffer for the shadow sampling shader.
    } else {
        if *shadow_filtering != ShadowFilteringMethod::Hardware2x2 {
            *shadow_filtering = ShadowFilteringMethod::Hardware2x2;
        }
    }
}

// ... (rest of the file remains the same)

pub struct CustomShadowNodePlugin;

impl Plugin for CustomShadowNodePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();

        render_graph.add_node(
            "poisson_disk_shadow_node",
            PoissonDiskShadowNode::new(&mut render_app.world),
        );

        let shader_handle = render_app.world.resource::<AssetServer>().load("shaders/poisson_disk_pcf.wgsl");
        render_app.world.insert_resource(PoissonDiskPcfShader(shader_handle));
        render_app.world.init_resource::<ShadowShaderSpecialization>();
        render_app.world.init_resource::<ShadowQualityState>();
        render_app.world.init_resource::<TemporalPoissonDisk>(); // <-- Added

        app.add_systems(Update, (
            finalize_shadow_specialization,
            update_temporal_poisson_disk_shadows,
        ));
    }
}
