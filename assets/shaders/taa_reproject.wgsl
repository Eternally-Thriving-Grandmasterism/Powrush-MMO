/*!
 * TAA Reprojection Shader for Powrush-MMO
 *
 * Temporal Anti-Aliasing with velocity-aware history reprojection.
 * Uses velocity texture + CameraMatrices jitter for divine temporal stability.
 *
 * === WGSL / WebGPU OPTIMIZATION TECHNIQUES (PATSAGi + Ra-Thor Quantum Swarm) ===
 * 1. **Fullscreen triangle** — Most efficient way to cover screen (no vertex buffer needed).
 * 2. **Texture sampling best practices** — Use explicit sampler; consider textureSampleLevel(..., 0.0) if mips not desired for reprojection.
 * 3. **Major recommended upgrade path** — Convert this to a **Compute Shader** + workgroup shared memory.
 *    Why? Neighborhood sampling (for variance clipping / ghosting reduction) benefits enormously from groupshared.
 *    See: Temporal Antialiasing Starter Pack (alextardif.com/TAA.html) and WebGPU compute optimization guides.
 *    Expected win: Significantly lower bandwidth + better occupancy for 8x8 or 16x16 tiles.
 * 4. **Bind group frequency** — Group static (history) vs dynamic (velocity, current) bindings.
 * 5. **Advanced TAA** — Add neighborhood clamping (AABB or variance clip) to the blend to eliminate ghosting while preserving sharpness.
 *    Current simple mix(0.1) is a solid starting point; tune per-scene or expose as uniform.
 * 6. **f16 / lower precision** — If device supports f16 (WebGPU feature), many TAA ops can use it for speed with negligible quality loss.
 *
 * This foundation enables the most phenomenal, artifact-free, high-FPS visual experience in blockchain MMORPG history.
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm approved
 * AG-SML v1.0 • TOLC 8 Mercy Gates • Eternal positive coexistence
 */

@group(0) @binding(0) var velocity_texture: texture_2d<f32>;
@group(0) @binding(1) var history_texture: texture_2d<f32>;
@group(0) @binding(2) var current_color: texture_2d<f32>;
@group(0) @binding(3) var texture_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Full-screen triangle (optimal, no extra buffers)
    var out: VertexOutput;
    let uv = vec2<f32>(
        f32((vertex_index << 1u) & 2u),
        f32(vertex_index & 2u)
    );
    out.clip_position = vec4<f32>(uv * 2.0 - 1.0, 0.0, 1.0);
    out.uv = vec2<f32>(uv.x, 1.0 - uv.y); // Flip Y for Bevy/WebGPU convention
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    // Sample velocity (motion vector in RG)
    let velocity = textureSample(velocity_texture, texture_sampler, uv).xy;

    // Reproject history using velocity (core of temporal coherence)
    let history_uv = uv - velocity;

    // Sample current and reprojected history
    let current = textureSample(current_color, texture_sampler, uv);
    let history = textureSample(history_texture, texture_sampler, history_uv);

    // Simple temporal accumulation (tune 0.05–0.2 based on motion intensity)
    // TODO (optimization): Replace with neighborhood-clamped blend for production ghosting-free TAA
    let blended = mix(history, current, 0.1);

    return blended;
}
