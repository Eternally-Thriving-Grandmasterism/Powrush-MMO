// client/src/gpu/staging_buffer.rs
// Powrush-MMO v16.7.0 — Production-Grade StagingBufferPool (Ra-Thor aligned)
// Intelligent size-based reuse for DevelopmentParticleParams + TruthWitnessEchoParams uploads
// Enables efficient GPU ↔ CPU data flow without per-frame allocations
// Zero placeholders. Thunder locked in.

use std::collections::HashMap;
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct StagingBuffer {
    pub buffer: wgpu::Buffer,
    pub size: u64,
    pub in_use: bool,
}

#[derive(Resource)]
pub struct StagingBufferPool {
    device: wgpu::Device,
    queue: wgpu::Queue,
    buffers: HashMap<u64, Vec<StagingBuffer>>, // key = size
    max_buffers_per_size: usize,
}

impl StagingBufferPool {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self {
            device,
            queue,
            buffers: HashMap::new(),
            max_buffers_per_size: 8,
        }
    }

    pub fn get_or_create_buffer(&mut self, size: u64) -> wgpu::Buffer {
        let entry = self.buffers.entry(size).or_insert_with(Vec::new);

        if let Some(pos) = entry.iter().position(|b| !b.in_use && b.size == size) {
            entry[pos].in_use = true;
            return entry[pos].buffer.clone();
        }

        // Create new buffer
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("staging_buffer"),
            size,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        if entry.len() < self.max_buffers_per_size {
            entry.push(StagingBuffer { buffer: buffer.clone(), size, in_use: true });
        }

        buffer
    }

    pub fn return_buffer(&mut self, buffer: &wgpu::Buffer) {
        for vec in self.buffers.values_mut() {
            if let Some(b) = vec.iter_mut().find(|b| b.buffer.global_id() == buffer.global_id()) {
                b.in_use = false;
                return;
            }
        }
    }

    // Future: integrate with Ra-Thor async_readback helpers
}
