// client/monitoring/gpu_timestamps.rs
// wgpu Timestamp Query System with Validation (v18.37)

use bevy::prelude::*;
use bevy::render::{
    render_graph::{Node, NodeRunError, RenderGraphContext},
    renderer::{RenderContext, RenderDevice, RenderQueue},
    RenderApp, RenderSet,
};
use std::sync::{Arc, Mutex};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Features, QuerySet, QuerySetDescriptor, QueryType};

#[derive(Clone, Debug, Default)]
pub struct TimestampValidation {
    pub is_valid: bool,
    pub last_error: Option<String>,
    pub last_gpu_time_ms: f32,
}

#[derive(Resource, Clone, Default)]
pub struct GpuTimestampState {
    pub latest: Arc<Mutex<TimestampValidation>>,
}

#[derive(Resource)]
pub struct GpuTimestampQueries {
    pub query_set: QuerySet,
    pub resolve_buffer: Buffer,
    pub read_buffer: Buffer,
    pub state: GpuTimestampState,
    pub supported: bool,
}

impl GpuTimestampQueries {
    pub fn new(device: &RenderDevice) -> Self {
        let supported = device.features().contains(Features::TIMESTAMP_QUERY);

        if !supported {
            warn!("[GPU Timestamps] TIMESTAMP_QUERY feature not supported on this device. GPU timing will be unavailable.");
        }

        let query_set = device.create_query_set(&QuerySetDescriptor {
            label: Some("gpu_timestamp_query_set"),
            ty: QueryType::Timestamp,
            count: 2,
        });

        let resolve_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("gpu_timestamp_resolve_buffer"),
            size: std::mem::size_of::<u64>() as u64 * 2,
            usage: BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let read_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("gpu_timestamp_read_buffer"),
            size: std::mem::size_of::<u64>() as u64 * 2,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            query_set,
            resolve_buffer,
            read_buffer,
            state: GpuTimestampState::default(),
            supported,
        }
    }
}

pub struct TimestampQueryNode;

impl Node for TimestampQueryNode {
    fn input(&self) -> Vec<SlotInfo> { vec![] }
    fn output(&self) -> Vec<SlotInfo> { vec![] }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let queries = world.resource::<GpuTimestampQueries>();

        if !queries.supported {
            return Ok(());
        }

        let encoder = render_context.command_encoder();
        encoder.write_timestamp(&queries.query_set, 0);
        encoder.write_timestamp(&queries.query_set, 1);

        Ok(())
    }
}

pub fn resolve_gpu_timestamps(
    queries: Res<GpuTimestampQueries>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    if !queries.supported {
        return;
    }

    let mut encoder = render_device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("resolve_gpu_timestamps"),
    });

    encoder.resolve_query_set(&queries.query_set, 0..2, &queries.resolve_buffer, 0);
    encoder.copy_buffer_to_buffer(&queries.resolve_buffer, 0, &queries.read_buffer, 0, std::mem::size_of::<u64>() as u64 * 2);

    render_queue.submit(std::iter::once(encoder.finish()));

    let buffer_slice = queries.read_buffer.slice(..);
    let state = queries.state.clone();

    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        let mut validation = TimestampValidation {
            is_valid: false,
            last_error: None,
            last_gpu_time_ms: 0.0,
        };

        if result.is_ok() {
            let data = buffer_slice.get_mapped_range();
            let timestamps: &[u64] = bytemuck::cast_slice(&data);

            if timestamps.len() >= 2 {
                let start = timestamps[0];
                let end = timestamps[1];

                if end >= start {
                    let gpu_time_ns = end - start;
                    let gpu_time_ms = gpu_time_ns as f32 / 1_000_000.0;

                    // Sanity check: GPU time should be reasonable (< 100ms per frame is already very high)
                    if gpu_time_ms > 0.0 && gpu_time_ms < 100.0 {
                        validation.is_valid = true;
                        validation.last_gpu_time_ms = gpu_time_ms;
                    } else {
                        validation.last_error = Some(format!("Unreasonable GPU time: {:.2} ms", gpu_time_ms));
                    }
                } else {
                    validation.last_error = Some("Timestamp end < start (invalid order)".to_string());
                }
            } else {
                validation.last_error = Some("Insufficient timestamp data".to_string());
            }

            drop(data);
            queries.read_buffer.unmap();
        } else {
            validation.last_error = Some("Failed to map timestamp buffer".to_string());
        }

        if let Ok(mut latest) = state.latest.lock() {
            *latest = validation;
        }
    });
}

pub struct GpuTimestampPlugin;

impl Plugin for GpuTimestampPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);

        render_app
            .init_resource::<GpuTimestampQueries>()
            .add_systems(Render, resolve_gpu_timestamps.after(RenderSet::Render));

        let mut render_graph = render_app.world_mut().resource_mut::<bevy::render::render_graph::RenderGraph>();
        render_graph.add_node("gpu_timestamp_queries", TimestampQueryNode);
    }
}

pub fn get_latest_gpu_validation(queries: &GpuTimestampQueries) -> TimestampValidation {
    if let Ok(guard) = queries.state.latest.lock() {
        guard.clone()
    } else {
        TimestampValidation { is_valid: false, last_error: Some("Lock failed".to_string()), last_gpu_time_ms: 0.0 }
    }
}