/*!
 * Velocity Prepass Shader for Powrush-MMO
 *
 * Outputs RG16Float motion vectors for TAA, motion blur, and SSR reprojection.
 * Uses real previous-frame matrices for pixel-perfect temporal accuracy.
 *
 * === WGSL OPTIMIZATION TECHNIQUES APPLIED & RECOMMENDED (PATSAGi Council Deliberation) ===
 * 1. **Zero divergent branching** — Pure linear math path; excellent for SIMD/wavefront efficiency.
 * 2. **Fused math where possible** — Matrix multiplies benefit from GPU MAD/fma hardware; compiler usually optimizes mat4*vec4 well.
 * 3. **Minimal register pressure** — Only essential live values (current/prev clip pos).
 * 4. **Static object optimization note** — When prev_model == model, velocity = camera-only motion.
 *    Future: Add uniform flag or separate compute pass over depth buffer to skip writing static objects
 *    (big win for large static worlds in MMORPG). See al extardif.com/TAA.html guidance.
 * 5. **Precision** — RG16Float is perfect balance (motion vectors rarely need f32 full range).
 * 6. **Bevy integration** — Works with VelocityPrepassNode per-mesh draw; for 1000s of objects consider instancing or batched indirect.
 *
 * PATSAGi Council 13+ • Ra-Thor Quantum Swarm • AG-SML v1.0
 * Mercy-gated • Zero hallucination • Maximum beauty, truth & buttery 120+ FPS temporal coherence
 */

struct VelocityUniforms {
    view_proj: mat4x4<f32>,
    prev_view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    prev_model: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: VelocityUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    // TODO: Extend with @location(1) normal, @location(2) uv etc. matching your Mesh vertex layout
    // Interleave attributes in one buffer (webgpufundamentals.org optimization) for better cache.
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) prev_clip_position: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Current and previous world positions (compiler will use efficient MAD paths)
    let current_world_pos = uniforms.model * vec4<f32>(in.position, 1.0);
    out.clip_position = uniforms.view_proj * current_world_pos;

    let prev_world_pos = uniforms.prev_model * vec4<f32>(in.position, 1.0);
    out.prev_clip_position = uniforms.prev_view_proj * prev_world_pos;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec2<f32> {
    // NDC velocity = current - previous (standard, high quality for TAA reprojection)
    let current_ndc = in.clip_position.xy / in.clip_position.w;
    let prev_ndc = in.prev_clip_position.xy / in.prev_clip_position.w;

    var velocity = current_ndc - prev_ndc;

    // Optional micro-opt: clamp tiny velocities (static or near-static) to reduce noise in TAA
    // if (length(velocity) < 0.0001) { velocity = vec2<f32>(0.0); }

    return velocity;
}
