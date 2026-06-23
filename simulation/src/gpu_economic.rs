/*!
 * Actual wgpu WGSL Compute Dispatch for Sovereign Economic / RBE Layer
 * 
 * Mint-and-print-only-perfection v18.97.6 — SystemSet-based Chaining + Telemetry
 * 
 * Production-grade asynchronous GPU economic simulation using wgpu map_async + Bevy AsyncComputeTaskPool.
 * Uses explicit SystemSet for clear, maintainable ordering.
 * Includes basic telemetry system for monitoring GPU economic health.
 * 
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor Lattice aligned
 * Thunder locked in. Yoi ⚡️
 */

use crate::world::{SovereignWorldState, ResourceNode};
use std::cell::Cell;
use std::sync::OnceLock;
use tracing::{warn, info};
use wgpu::util::DeviceExt;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::prelude::{SystemSet, ResMut, Resource};

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

/// Embedded authoritative WGSL source (elevated from engine/patsagi_economic.wgsl v16.5.58).
const WGSL_SOURCE: &str = r#"
// patsagi_economic.wgsl v16.5.58 - Powrush-MMO Sovereign Economic Kernel
struct Node {
    depletion: f32,
    regen_rate: f32,
    stress: f32,
    abundance_flow: f32,
    sustainability: f32,
    _padding: vec3<f32>,
};

@group(0) @binding(0) var<storage, read_write> nodes: array<Node>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&nodes)) { return; }
    var node = nodes[index];
    let future_depletion = node.depletion + (node.stress * 0.02) - (node.regen_rate * 0.8);
    node.depletion = clamp(future_depletion, 0.0, 1.0);
    node.abundance_flow = (node.regen_rate * 2.0 - node.stress) * 0.5;
    node.sustainability = clamp(1.0 - node.depletion * 0.6 - node.stress * 0.3, 0.3, 1.0);
    if (node.depletion > 0.7) { node.stress = min(node.stress + 0.15, 1.0); }
    nodes[index] = node;
    output[index] = node.depletion;
}
"#;

fn get_or_init_gpu_context(node_count: usize) -> Option<&'static GpuContext> {
    GPU_CONTEXT.get_or_init(|| {
        pollster::block_on(async {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            });
            let adapter = match instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            }).await {
                Ok(a) => a,
                Err(e) => { warn!("GPU adapter request failed: {:?}. Will use CPU path.", e); return None; }
            };
            let (device, queue) = match adapter.request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                label: Some("powrush-simulation-gpu"),
            }, None).await {
                Ok(dq) => dq,
                Err(e) => { warn!("GPU device request failed: {:?}. Will use CPU path.", e); return None; }
            };
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("patsagi_economic"),
                source: wgpu::ShaderSource::Wgsl(WGSL_SOURCE.into()),
            });
            let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("economic_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
                    wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
                ],
            });
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("economic_pipeline_layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
            let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("patsagi_economic_pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point: "main",
                compilation_options: Default::default(),
            });

            let node_buffer_size = (node_count.max(1024) * std::mem::size_of::<GpuNode>()) as wgpu::BufferAddress;
            let output_buffer_size = (node_count.max(1024) * std::mem::size_of::<f32>()) as wgpu::BufferAddress;

            let node_buffer = device.create_buffer(&wgpu::BufferDescriptor { label: Some("node_buffer_persistent"), size: node_buffer_size, usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
            let output_buffer = device.create_buffer(&wgpu::BufferDescriptor { label: Some("output_buffer_persistent"), size: output_buffer_size, usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC, mapped_at_creation: false });
            let staging_a = device.create_buffer(&wgpu::BufferDescriptor { label: Some("staging_a"), size: node_buffer_size, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
            let staging_b = device.create_buffer(&wgpu::BufferDescriptor { label: Some("staging_b"), size: node_buffer_size, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });

            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("economic_bind_group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: node_buffer.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: output_buffer.as_entire_binding() },
                ],
            });

            Some(GpuContext {
                device, queue, pipeline, bind_group_layout,
                node_buffer, output_buffer,
                staging_buffer_a: staging_a, staging_buffer_b: staging_b,
                current_staging: Cell::new(false),
                bind_group,
                node_capacity: node_count,
            })
        })
    }).as_ref()
}

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

/// SystemSet for clear, maintainable ordering of GPU economic systems.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GpuEconomicSystemSet {
    Dispatch,
    Apply,
    Telemetry,
}

/// Dispatches GPU compute and spawns a non-blocking async task for readback using AsyncComputeTaskPool.
pub fn dispatch_gpu_economic_compute_async(
    world: &mut SovereignWorldState,
    readback: &mut GpuEconomicReadback,
    current_frame: u64,
) -> Result<(), String> {
    let node_count = world.resource_nodes.len();
    if node_count == 0 {
        return Ok(());
    }

    if readback.pending_task.is_some() {
        return Ok(());
    }

    let context = get_or_init_gpu_context(node_count)
        .ok_or("GPU context unavailable")?;

    let mut entries: Vec<_> = world.resource_nodes.iter().collect();
    entries.sort_by_key(|(id, _)| *id);

    let mut gpu_nodes: Vec<GpuNode> = Vec::with_capacity(node_count);
    let mut node_ids: Vec<u64> = Vec::with_capacity(node_count);

    for (id, node) in &entries {
        node_ids.push(*id);
        gpu_nodes.push(GpuNode {
            depletion: node.depletion,
            regen_rate: node.regen_rate,
            stress: node.stress_level,
            abundance_flow: node.abundance_flow,
            sustainability: node.sustainability_score,
            _padding: [0.0; 3],
        });
    }

    context.queue.write_buffer(&context.node_buffer, 0, bytemuck::cast_slice(&gpu_nodes));

    let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("economic_compute_encoder_async"),
    });

    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("patsagi_economic_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&context.pipeline);
        pass.set_bind_group(0, &context.bind_group, &[]);
        let workgroups = ((gpu_nodes.len() as u32) + 63) / 64;
        pass.dispatch_workgroups(workgroups, 1, 1);
    }

    let use_a = context.current_staging.get();
    let staging = if use_a {
        &context.staging_buffer_a
    } else {
        &context.staging_buffer_b
    };

    encoder.copy_buffer_to_buffer(
        &context.node_buffer,
        0,
        staging,
        0,
        (gpu_nodes.len() * std::mem::size_of::<GpuNode>()) as u64,
    );

    context.queue.submit(std::iter::once(encoder.finish()));

    context.current_staging.set(!use_a);

    let task_pool = AsyncComputeTaskPool::get();
    let staging_buffer = staging.clone();
    let node_ids_clone = node_ids.clone();

    let task: Task<GpuReadbackResult> = task_pool.spawn(async move {
        let buffer_slice = staging_buffer.slice(..);

        let (tx, rx) = async_channel::bounded(1);

        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = tx.try_send(result);
        });

        match rx.recv().await {
            Ok(Ok(())) => {
                let data = buffer_slice.get_mapped_range();
                let updated_nodes: Vec<GpuNode> = bytemuck::cast_slice(&data).to_vec();

                drop(data);
                staging_buffer.unmap();

                GpuReadbackResult {
                    node_ids: node_ids_clone,
                    updated_nodes,
                    frame_submitted: current_frame,
                }
            }
            Ok(Err(e)) => {
                warn!("GPU buffer mapping failed: {:?}", e);
                GpuReadbackResult {
                    node_ids: node_ids_clone,
                    updated_nodes: vec![],
                    frame_submitted: current_frame,
                }
            }
            Err(_) => {
                GpuReadbackResult {
                    node_ids: node_ids_clone,
                    updated_nodes: vec![],
                    frame_submitted: current_frame,
                }
            }
        }
    });

    readback.pending_task = Some(task);
    Ok(())
}

/// Dedicated Bevy system that submits GPU economic compute work every frame.
pub fn gpu_economic_dispatch_system(
    mut world: ResMut<SovereignWorldState>,
    mut readback: ResMut<GpuEconomicReadback>,
) {
    let current_frame: u64 = 0;
    if let Err(e) = dispatch_gpu_economic_compute_async(&mut world, &mut readback, current_frame) {
        warn!("GPU economic dispatch failed: {}. CPU fallback or previous results will be used.", e);
    }
}

/// Non-blocking system to poll and apply completed GPU readback results.
pub fn apply_gpu_economic_results(
    mut readback: ResMut<GpuEconomicReadback>,
    mut world: ResMut<SovereignWorldState>,
) {
    if let Some(mut task) = readback.pending_task.take() {
        match task.poll_once() {
            bevy::tasks::PollOnceResult::Ready(result) => {
                for (i, gpu_node) in result.updated_nodes.iter().enumerate() {
                    if let Some(node) = world.resource_nodes.get_mut(&result.node_ids[i]) {
                        node.depletion = gpu_node.depletion;
                        node.abundance_flow = gpu_node.abundance_flow;
                        node.sustainability_score = gpu_node.sustainability;
                        node.stress_level = gpu_node.stress;
                    }
                }
            }
            bevy::tasks::PollOnceResult::Pending(returned_task) => {
                readback.pending_task = Some(returned_task);
            }
        }
    }
}

/// Simple telemetry system for the GPU economic layer.
/// Logs key metrics every frame for monitoring and debugging.
pub fn gpu_economic_telemetry_system(
    readback: ResMut<GpuEconomicReadback>,
    world: ResMut<SovereignWorldState>,
) {
    let node_count = world.resource_nodes.len();
    let has_pending = readback.pending_task.is_some();

    if node_count > 0 || has_pending {
        info!(
            target: "gpu_economic",
            node_count = node_count,
            has_pending_readback = has_pending,
            "GPU Economic Telemetry"
        );
    }
}

/// Legacy blocking version kept for CPU fallback / testing.
pub fn dispatch_gpu_economic_update(world: &mut SovereignWorldState) -> Result<(), String> {
    let node_count = world.resource_nodes.len();
    if node_count == 0 { return Ok(()); }

    let context = match get_or_init_gpu_context(node_count) {
        Some(ctx) => ctx,
        None => return Err("GPU context unavailable or init failed".to_string()),
    };

    let mut entries: Vec<_> = world.resource_nodes.iter().collect();
    entries.sort_by_key(|(id, _)| *id);

    let mut gpu_nodes: Vec<GpuNode> = Vec::with_capacity(node_count);
    let mut node_ids: Vec<u64> = Vec::with_capacity(node_count);
    for (id, node) in &entries {
        node_ids.push(*id);
        gpu_nodes.push(GpuNode {
            depletion: node.depletion, regen_rate: node.regen_rate, stress: node.stress_level,
            abundance_flow: node.abundance_flow, sustainability: node.sustainability_score, _padding: [0.0; 3],
        });
    }

    context.queue.write_buffer(&context.node_buffer, 0, bytemuck::cast_slice(&gpu_nodes));

    let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("economic_compute_encoder") });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("patsagi_economic_pass"), timestamp_writes: None });
        compute_pass.set_pipeline(&context.pipeline);
        compute_pass.set_bind_group(0, &context.bind_group, &[]);
        let workgroups = ((gpu_nodes.len() as u32) + 63) / 64;
        compute_pass.dispatch_workgroups(workgroups, 1, 1);
    }

    let staging = if context.current_staging.get() { &context.staging_buffer_a } else { &context.staging_buffer_b };
    encoder.copy_buffer_to_buffer(&context.node_buffer, 0, staging, 0, (gpu_nodes.len() * std::mem::size_of::<GpuNode>()) as u64);

    context.queue.submit(std::iter::once(encoder.finish()));
    context.device.poll(wgpu::Maintain::Wait);

    let buffer_slice = staging.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| { let _ = sender.send(result); });
    context.device.poll(wgpu::Maintain::Wait);

    if let Err(e) = receiver.recv().unwrap() { return Err(format!("Map async error: {:?}", e)); }

    let data = buffer_slice.get_mapped_range();
    let updated_gpu_nodes: &[GpuNode] = bytemuck::cast_slice(&data);

    for (i, gpu_node) in updated_gpu_nodes.iter().enumerate() {
        if let Some(node) = world.resource_nodes.get_mut(&node_ids[i]) {
            node.depletion = gpu_node.depletion;
            node.abundance_flow = gpu_node.abundance_flow;
            node.sustainability_score = gpu_node.sustainability;
            node.stress_level = gpu_node.stress;
        }
    }

    drop(data);
    staging.unmap();

    context.current_staging.set(!context.current_staging.get());

    Ok(())
}
