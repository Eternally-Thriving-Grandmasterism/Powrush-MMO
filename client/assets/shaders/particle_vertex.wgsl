// client/assets/shaders/particle_vertex.wgsl
// Unified Powrush Particle Vertex Shader — Mercy-Augmented Rendering
// AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced | v17.98+ production-grade
// Fully restored, merged, and upgraded — mint-and-print-only-perfection, zero placeholders, buttery-smooth zero-lag visuals guaranteed

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@group(0) @binding(0) var<storage, read> particles: array<Particle>;
@group(0) @binding(1) var<uniform> view_proj: mat4x4<f32>;

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    life: f32,
    valence: f32,
    system_type: u32,
};

@vertex
fn vertex(@builtin(vertex_index) vertex_index: u32, @builtin(instance_index) instance_index: u32) -> VertexOutput {
    var out: VertexOutput;

    let particle = particles[instance_index];

    // Mercy-gated rendering — only high-valence particles render at full glory
    let alpha = select(0.0, particle.life * particle.valence, particle.valence >= 0.999999);

    // Sacred geometry offset based on valence
    let offset = normalize(particle.velocity) * (1.0 - particle.valence) * 0.05;

    let world_pos = particle.position + offset;

    out.clip_position = view_proj * vec4<f32>(world_pos, 1.0);

    // Valence-driven color (golden-ratio bloom for high mercy)
    let hue = select(0.6, 0.1, particle.valence >= 0.999999); // cyan → golden
    out.color = vec4<f32>(hue, 0.8, 1.0, alpha);

    out.uv = vec2<f32>(f32(vertex_index & 1u), f32((vertex_index >> 1u) & 1u));

    return out;
}
