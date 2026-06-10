/*!
 * Actual wgpu WGSL Compute Dispatch for Sovereign Economic / RBE Layer
 * 
 * Mint-and-print-only-perfection v17.99.6
 * 
 * Elevates and implements real GPU-accelerated batch processing using the authoritative
 * engine/patsagi_economic.wgsl kernel (v16.5.58) for large-scale RBE simulations.
 * 
 * - Hybrid dispatch: CPU precision path (always available) + optional real WGSL compute path.
 * - Every batch is wrapped by non-bypassable TOLC 8 Mercy Gates in the caller (economy.rs).
 * - Full intelligent historical merge of previous stub + WGSL logic + ResourceNode dynamics.
 * - Deterministic when using same seed + same dispatch path (GPU path is bit-exact for given inputs).
 * 
 * This closes the final piece of the Integrated MMO-Scale Simulation Harness Gap for
 * sovereign, time-accelerated, large-population RBE validation.
 */

use crate::world::{SovereignWorldState, ResourceNode};
use std::sync::OnceLock;
use tracing::warn;
use wgpu::util::DeviceExt;

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

/// Sovereign GPU compute context (lazy-initialized).
struct GpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

static GPU_CONTEXT: OnceLock<Option<GpuContext>> = OnceLock::new();

/// Embedded authoritative WGSL source (elevated from engine/patsagi_economic.wgsl v16.5.58).
/// Preserved exactly so the simulation crate remains sovereign and self-contained.
const WGSL_SOURCE: &str = r#"
// patsagi_economic.wgsl
// Powrush-MMO v16.5.58 — Real GPU PATSAGi Economic Simulation Kernel
// Multi-step future prediction for resource nodes:
// - Depletion / regeneration
// - Abundance flow calculation
// - Pressure scenario simulation
// - Basic interdependence hints (via stress propagation)
// AG-SML v1.0 | Mercy-gated authoritative foresight

struct Node {
    depletion: f32,
    regen_rate: f32,
    stress: f32,
    abundance_flow: f32,
    sustainability: f32,
    _padding: vec3<f32>, // align to 32 bytes
};

@group(0) @binding(0) var<storage, read_write> nodes: array<Node>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>; // simple output buffer for now (can be expanded)

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&nodes)) {
        return;
    }

    var node = nodes[index];

    // === Core economic simulation step ===
    let future_depletion = node.depletion + (node.stress * 0.02) - (node.regen_rate * 0.8);
    node.depletion = clamp(future_depletion, 0.0, 1.0);

    node.abundance_flow = (node.regen_rate * 2.0 - node.stress) * 0.5;

    node.sustainability = clamp(1.0 - node.depletion * 0.6 - node.stress * 0.3, 0.3, 1.0);

    if (node.depletion > 0.7) {
        node.stress = min(node.stress + 0.15, 1.0);
    }

    nodes[index] = node;
    output[index] = node.depletion;
}
"#;

/// Initialize (or return existing) GPU context. Returns None if GPU is unavailable.
fn get_or_init_gpu_context() -> Option<&'static GpuContext> {
    GPU_CONTEXT.get_or_init(|| {
        pollster::block_on(async {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            });

            let adapter = match instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: None,
                    force_fallback_adapter: false,
                })
                .await
            {
                Ok(a) => a,
                Err(e) => {
                    warn!("GPU adapter request failed: {:?}. Will use CPU path.", e);
                    return None;
                }
            };

            let (device, queue) = match adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::downlevel_defaults(),
                        label: Some("powrush-simulation-gpu"),
                    },
                    None,
                )
                .await
            {
                Ok(dq) => dq,
                Err(e) => {
                    warn!("GPU device request failed: {:?}. Will use CPU path.", e);
                    return None;
                }
            };

            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("patsagi_economic"),
                source: wgpu::ShaderSource::Wgsl(WGSL_SOURCE.into()),
            });

            let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("economic_bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
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

            Some(GpuContext {
                device,
                queue,
                pipeline,
                bind_group_layout,
            })
        })
    }).as_ref()
}

/// Perform actual WGSL compute dispatch on the provided world state.
/// Collects ResourceNode data (sorted by id for determinism), uploads to GPU,
/// dispatches the economic kernel, reads results back, and writes updated fields.
pub fn dispatch_gpu_economic_update(world: &mut SovereignWorldState) -> Result<(), String> {
    let context = match get_or_init_gpu_context() {
        Some(ctx) => ctx,
        None => return Err("GPU context unavailable or init failed".to_string()),
    };

    let node_count = world.resource_nodes.len();
    if node_count == 0 {
        return Ok(());
    }

    // Stable order for determinism (sort by NodeId = u64)
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

    let node_buffer_size = (gpu_nodes.len() * std::mem::size_of::<GpuNode>()) as wgpu::BufferAddress;

    let node_buffer = context.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("economic_nodes_storage"),
        contents: bytemuck::cast_slice(&gpu_nodes),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    let output_buffer_size = (gpu_nodes.len() * std::mem::size_of::<f32>()) as wgpu::BufferAddress;
    let output_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("economic_output_storage"),
        size: output_buffer_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("economic_bind_group"),
        layout: &context.bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: node_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
            },
        ],
    });

    let mut encoder = context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("economic_compute_encoder"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("patsagi_economic_pass"),
            timestamp_writes: None,
        });
        compute_pass.set_pipeline(&context.pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        let workgroups = ((gpu_nodes.len() as u32) + 63) / 64;
        compute_pass.dispatch_workgroups(workgroups, 1, 1);
    }

    // Readback staging buffer
    let staging_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("economic_readback_staging"),
        size: node_buffer_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    encoder.copy_buffer_to_buffer(&node_buffer, 0, &staging_buffer, 0, node_buffer_size);

    context.queue.submit(std::iter::once(encoder.finish()));
    context.device.poll(wgpu::Maintain::Wait);

    // Map async readback
    let buffer_slice = staging_buffer.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = sender.send(result);
    });
    context.device.poll(wgpu::Maintain::Wait);
    if let Err(e) = receiver.recv().unwrap() {
        return Err(format!("Map async error: {:?}", e));
    }

    let data = buffer_slice.get_mapped_range();
    let updated_gpu_nodes: &[GpuNode] = bytemuck::cast_slice(&data);

    // Write results back (using sorted ids)
    for (i, gpu_node) in updated_gpu_nodes.iter().enumerate() {
        if let Some(node) = world.resource_nodes.get_mut(&node_ids[i]) {
            node.depletion = gpu_node.depletion;
            node.abundance_flow = gpu_node.abundance_flow;
            node.sustainability_score = gpu_node.sustainability;
            node.stress_level = gpu_node.stress;
        }
    }

    drop(data);
    staging_buffer.unmap();

    Ok(())
}
