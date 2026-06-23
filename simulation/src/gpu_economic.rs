/*!
 * Actual wgpu WGSL Compute Dispatch for Sovereign Economic / RBE Layer
 * 
 * Mint-and-print-only-perfection v18.97.6 — SystemSet-based Chaining
 * 
 * Production-grade asynchronous GPU economic simulation using wgpu map_async + Bevy AsyncComputeTaskPool.
 * Uses explicit SystemSet for clear, maintainable ordering between dispatch and apply.
 * 
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor Lattice aligned
 * Thunder locked in. Yoi ⚡️
 */

use crate::world::{SovereignWorldState, ResourceNode};
use std::cell::Cell;
use std::sync::OnceLock;
use tracing::warn;
use wgpu::util::DeviceExt;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::prelude::{SystemSet, ResMut};

/// Exact mirror of the WGSL `Node` struct for bytemuck-safe GPU transfer.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct GpuNode {
    depletion: f32,
    regen_rate: f32,
    stress: f32,
    abundance_flow: f32,
    sustainability: f32,
    _padding: [f32; 3], // 32-byte alignment
}

/// Sovereign GPU compute context with persistent buffers and double-buffering for readback.
struct GpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    node_buffer: wgpu::Buffer,
    output_buffer: wgpu::Buffer,
    staging_buffer_a: wgpu::Buffer,
    staging_buffer_b: wgpu::Buffer,
    current_staging: Cell<bool>,
    bind_group: wgpu::BindGroup,
    node_capacity: usize,
}

static GPU_CONTEXT: OnceLock<Option<GpuContext>> = OnceLock::new();

// ... (WGSL_SOURCE and get_or_init_gpu_context unchanged for brevity) ...

const WGSL_SOURCE: &str = r#" ... "#; // kept identical

// (get_or_init_gpu_context function kept identical)

/// Resource to hold pending async GPU economic readback task.
#[derive(Resource, Default)]
pub struct GpuEconomicReadback {
    pub pending_task: Option<Task<GpuReadbackResult>>,
}

#[derive(Debug)]
pub struct GpuReadbackResult {
    pub node_ids: Vec<u64>,
    pub updated_nodes: Vec<GpuNode>,
    pub frame_submitted: u64,
}

/// SystemSet for ordering GPU economic systems.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GpuEconomicSystemSet {
    Dispatch,
    Apply,
}

/// Dispatches GPU compute... (function unchanged)
pub fn dispatch_gpu_economic_compute_async(...) { ... }

/// Dedicated dispatch system (unchanged logic)
pub fn gpu_economic_dispatch_system(
    mut world: ResMut<SovereignWorldState>,
    mut readback: ResMut<GpuEconomicReadback>,
) {
    let current_frame: u64 = 0;
    if let Err(e) = dispatch_gpu_economic_compute_async(&mut world, &mut readback, current_frame) {
        warn!("GPU economic dispatch failed: {}. CPU fallback or previous results will be used.", e);
    }
}

/// Apply system (unchanged)
pub fn apply_gpu_economic_results(...) { ... }

/// Legacy... (unchanged)
pub fn dispatch_gpu_economic_update(...) { ... }
