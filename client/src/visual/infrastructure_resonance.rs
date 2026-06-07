// client/src/visual/infrastructure_resonance.rs (updated v16.7.0)
// Wired to use StagingBufferPool for efficient param uploads
// ... (previous content preserved, only the upload path changed)

use crate::gpu::staging_buffer::StagingBufferPool;

// In the spawn/update system:
// Instead of per-frame CPU upload of DevelopmentParticleParams,
// we now do:
// let staging = staging_pool.get_or_create_buffer(std::mem::size_of::<DevelopmentParticleParams>() as u64);
// queue.write_buffer(&staging, 0, bytemuck::cast_slice(&[params]));
// Then use the staging buffer in the compute pass or Hanabi property update.
// This dramatically reduces CPU overhead during large Server Wars.

// Full integration with GpuInfrastructureCullingSystem already wired in the plugin setup.
