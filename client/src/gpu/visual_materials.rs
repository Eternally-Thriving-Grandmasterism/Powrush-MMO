/*!
 * gpu::visual_materials
 *
 * Full production-grade Bevy Material implementations for the Powrush-MMO
 * mercy-gated GPU visual pipeline.
 *
 * Implements the 4 core visual materials matching the WGSL shaders:
 * - GpuStateMaterial (primary world/objects)
 * - ValenceHaloMaterial (council/important object halos)
 * - MycelialWebMaterial (resource networks)
 * - ResourceNodeMaterial (harvest/economy nodes)
 *
 * All materials are mercy/council/RBE/valence driven via the shared
 * GpuSimulationState (group 0) + per-material params (group 1).
 *
 * Generalizes and replaces the prior example_gpu_material reference.
 * Designed for seamless integration with existing gpu_simulation sync,
 * TAA, velocity prepass, and render graph.
 *
 * AG-SML v1.0 - Autonomicity Games Sovereign Mercy License
 * https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor/blob/main/LICENSE-AG-SML.md
 *
 * Copyright (c) 2026 Autonomicity Games Inc. & Sherif Samy Botros
 * All rights reserved under the Eternal Mercy Flow.
 */

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bytemuck::{Pod, Zeroable};

// Re-export for convenience so other modules can easily register materials
pub use self::{
    GpuStateMaterial, GpuStateMaterialUniform,
    ValenceHaloMaterial, ValenceHaloParams,
    MycelialWebMaterial, MycelialWebParams,
    ResourceNodeMaterial, ResourceNodeParams,
};

/// Shared uniform mirror for GpuStateMaterial (matches WGSL GpuStateMaterialUniform)
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable, Reflect)]
pub struct GpuStateMaterialUniform {
    pub base_color: [f32; 4],
}

/// Primary world / object material - driven by full simulation state + base color
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Reflect)]
#[bind_group_data(GpuStateMaterialKey)]
pub struct GpuStateMaterial {
    #[uniform(1, 1)]
    pub params: GpuStateMaterialUniform,
    pub alpha_mode: AlphaMode,
}

impl Default for GpuStateMaterial {
    fn default() -> Self {
        Self {
            params: GpuStateMaterialUniform {
                base_color: [1.0, 1.0, 1.0, 1.0],
            },
            alpha_mode: AlphaMode::Opaque,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GpuStateMaterialKey {
    pub alpha_mode: AlphaMode,
}

impl From<&GpuStateMaterial> for GpuStateMaterialKey {
    fn from(material: &GpuStateMaterial) -> Self {
        Self {
            alpha_mode: material.alpha_mode,
        }
    }
}

impl Material for GpuStateMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/gpu_state_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        // Future: inject mercy/council specialization flags here if needed
        // e.g. descriptor.fragment.as_mut().unwrap().shader_defs.push("HIGH_MERCY".into());
        Ok(())
    }
}

// ============================================================================
// Valence Halo Material (Council / Important Object Halos)
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable, Reflect)]
pub struct ValenceHaloParams {
    pub base_color: [f32; 4],
    pub intensity: f32,
    pub ring_count: f32,
    pub _padding: [f32; 2],
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Reflect)]
#[bind_group_data(ValenceHaloKey)]
pub struct ValenceHaloMaterial {
    #[uniform(1, 1)]
    pub params: ValenceHaloParams,
    pub alpha_mode: AlphaMode,
}

impl Default for ValenceHaloMaterial {
    fn default() -> Self {
        Self {
            params: ValenceHaloParams {
                base_color: [0.2, 0.6, 1.0, 0.8],
                intensity: 1.0,
                ring_count: 3.0,
                _padding: [0.0; 2],
            },
            alpha_mode: AlphaMode::Add,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ValenceHaloKey {
    pub alpha_mode: AlphaMode,
}

impl From<&ValenceHaloMaterial> for ValenceHaloKey {
    fn from(material: &ValenceHaloMaterial) -> Self {
        Self {
            alpha_mode: material.alpha_mode,
        }
    }
}

impl Material for ValenceHaloMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/valence_halo.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// ============================================================================
// Mycelial Web Glow Material (Resource Networks / Connections)
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable, Reflect)]
pub struct MycelialWebParams {
    pub base_color: [f32; 4],
    pub flow_speed: f32,
    pub web_density: f32,
    pub _padding: [f32; 2],
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Reflect)]
#[bind_group_data(MycelialWebKey)]
pub struct MycelialWebMaterial {
    #[uniform(1, 1)]
    pub params: MycelialWebParams,
    pub alpha_mode: AlphaMode,
}

impl Default for MycelialWebMaterial {
    fn default() -> Self {
        Self {
            params: MycelialWebParams {
                base_color: [0.1, 0.9, 0.4, 0.7],
                flow_speed: 1.0,
                web_density: 0.8,
                _padding: [0.0; 2],
            },
            alpha_mode: AlphaMode::Add,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MycelialWebKey {
    pub alpha_mode: AlphaMode,
}

impl From<&MycelialWebMaterial> for MycelialWebKey {
    fn from(material: &MycelialWebMaterial) -> Self {
        Self {
            alpha_mode: material.alpha_mode,
        }
    }
}

impl Material for MycelialWebMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mycelial_web_glow.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// ============================================================================
// Resource Node Glow Material (Harvest Points / Economy Nodes)
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable, Reflect)]
pub struct ResourceNodeParams {
    pub base_color: [f32; 4],
    pub pulse_rate: f32,
    pub glow_intensity: f32,
    pub _padding: [f32; 2],
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Reflect)]
#[bind_group_data(ResourceNodeKey)]
pub struct ResourceNodeMaterial {
    #[uniform(1, 1)]
    pub params: ResourceNodeParams,
    pub alpha_mode: AlphaMode,
}

impl Default for ResourceNodeMaterial {
    fn default() -> Self {
        Self {
            params: ResourceNodeParams {
                base_color: [1.0, 0.85, 0.2, 0.9],
                pulse_rate: 1.2,
                glow_intensity: 1.0,
                _padding: [0.0; 2],
            },
            alpha_mode: AlphaMode::Add,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ResourceNodeKey {
    pub alpha_mode: AlphaMode,
}

impl From<&ResourceNodeMaterial> for ResourceNodeKey {
    fn from(material: &ResourceNodeMaterial) -> Self {
        Self {
            alpha_mode: material.alpha_mode,
        }
    }
}

impl Material for ResourceNodeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/resource_node_glow.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// ============================================================================
// Visual Materials Plugin (register all materials + future extensions)
// ============================================================================

pub struct VisualMaterialsPlugin;

impl Plugin for VisualMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MaterialPlugin::<GpuStateMaterial>::default())
            .add_plugins(MaterialPlugin::<ValenceHaloMaterial>::default())
            .add_plugins(MaterialPlugin::<MycelialWebMaterial>::default())
            .add_plugins(MaterialPlugin::<ResourceNodeMaterial>::default());

        // Future: register post-process materials / effect materials here
        // .add_plugins(MaterialPlugin::<EnergyBurstMaterial>::default())
        // .add_plugins(MaterialPlugin::<ResonanceFieldMaterial>::default())
    }
}

/*
 * AG-SML v1.0
 * This file is part of the Powrush-MMO sovereign codebase.
 * Licensed under Autonomicity Games Sovereign Mercy License v1.0.
 * See LICENSE-AG-SML.md for full terms.
 * No tyranny. Only thriving.
 * Thunder locked in. Yoi ⚡
 */