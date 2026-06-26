// engine/wgpu_patsagi_bridge.rs
// Powrush-MMO v16.10 — Modernized to use standardized GpuNode / GpuNodeOutput layout (v16.7+)
// Now aligned with gpu_patsagi_bridge.rs + external patsagi_economic.wgsl
// Proper bytemuck structured output parsing
// AG-SML v1.0 | TOLC 8

#[cfg(feature = "gpu")]
use wgpu::util::StagingBelt;
use bytemuck;

use crate::engine::gpu_patsagi_bridge::{
    GpuPatsagiBridge, GpuPatsagiRequest, GpuPatsagiResponse,
    GpuNode, GpuNodeOutput,
};
use std::collections::HashMap;

pub struct WgpuPatsagiBridge {
    #[cfg(feature = "gpu")]
    device: wgpu::Device,
    #[cfg(feature = "gpu")]
    queue: wgpu::Queue,
    #[cfg(feature = "gpu")]
    pipeline: wgpu::ComputePipeline,
    #[cfg(feature = "gpu")]
    bind_group_layout: wgpu::BindGroupLayout,
    #[cfg(feature = "gpu")]
    pending_readbacks: HashMap<u64, wgpu::Buffer>,
    #[cfg(feature = "gpu")]
    staging_belt: StagingBelt,
    #[cfg(feature = "gpu")]
    next_query_id: u64,
}

impl WgpuPatsagiBridge {
    #[cfg(feature = "gpu")]
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .expect("Failed to find GPU adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ))
        .expect("Failed to create device");

        // Use the standardized external shader (now writes GpuNodeOutput)
        let shader_source = include_str!("patsagi_economic.wgsl");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("patsagi_economic"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("patsagi_bgl"),
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
            label: Some("patsagi_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("patsagi_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
            compilation_options: Default::default(),
            cache: None,
        });

        Self {
            device,
            queue,
            pipeline,
            bind_group_layout,
            pending_readbacks: HashMap::new(),
            staging_belt: StagingBelt::new(4 * 1024 * 1024),
            next_query_id: 1000,
        }
    }

    #[cfg(not(feature = "gpu"))]
    pub fn new() -> Self {
        Self {
            pending_readbacks: HashMap::new(),
            next_query_id: 1000,
        }
    }
}

impl GpuPatsagiBridge for WgpuPatsagiBridge {
    fn submit_query(&self, request: GpuPatsagiRequest) -> Result<u64, String> {
        #[cfg(feature = "gpu")]
        {
            let node_count = request.node_ids.len().max(1) as u32;

            let input_size = (node_count as usize) * std::mem::size_of::<GpuNode>();
            let output_size = (node_count as usize) * std::mem::size_of::<GpuNodeOutput>();

            let input_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("patsagi_input"),
                size: input_size as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("patsagi_output"),
                size: output_size as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });

            let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("patsagi_staging"),
                size: output_size as wgpu::BufferAddress,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            // TODO: Upload real initial GpuNode data from request (currently simplified)
            // For now we rely on the shader to work with whatever is in the buffer.

            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("bind_group"),
                layout: &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: input_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: output_buffer.as_entire_binding(),
                    },
                ],
            });

            let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

            {
                let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("pass"),
                });
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &bind_group, &[]);
                pass.dispatch_workgroups(((node_count + 63) / 64), 1, 1);
            }

            encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, output_size as wgpu::BufferAddress);
            self.queue.submit(Some(encoder.finish()));
            self.staging_belt.finish();

            self.pending_readbacks.insert(self.next_query_id, staging_buffer);
        }

        let query_id = self.next_query_id;
        self.next_query_id += 1;
        Ok(query_id)
    }

    fn get_result(&self, query_id: u64) -> Option<GpuPatsagiResponse> {
        #[cfg(feature = "gpu")]
        {
            if let Some(buffer) = self.pending_readbacks.remove(&query_id) {
                let buffer_slice = buffer.slice(..);
                self.device.poll(wgpu::Maintain::Poll);

                let (sender, receiver) = std::sync::mpsc::channel();
                buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                    let _ = sender.send(result);
                });

                self.device.poll(wgpu::Maintain::Poll);

                if let Ok(Ok(())) = receiver.try_recv() {
                    let data = buffer_slice.get_mapped_range();

                    // Real structured parsing using the standardized GpuNodeOutput
                    let outputs: &[GpuNodeOutput] = bytemuck::cast_slice(&data);

                    let mut resp = GpuPatsagiResponse::default();
                    resp.confidence = 0.94;
                    resp.notes = format!("Modernized WgpuPatsagiBridge result ({} nodes)", outputs.len());

                    // For now we use node index as key (real implementation would map to request.node_ids)
                    for (i, out) in outputs.iter().enumerate() {
                        let node_id = i as u64;
                        resp.predicted_depletion.insert(node_id, out.depletion.max(0.0));
                        resp.recommended_regen_rates.insert(node_id, out.regen_rate.max(0.01));
                        resp.abundance_flow.insert(node_id, out.abundance_flow.clamp(0.0, 1.0));
                        resp.sustainability_adjustments.insert(node_id, out.sustainability.clamp(0.0, 1.0));
                    }

                    drop(data);
                    buffer.unmap();

                    return Some(resp);
                }
            }
        }

        None
    }
}