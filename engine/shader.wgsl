// engine/shader.wgsl — Unified Mercy-Gated Particle System (v14.6.0+)
// Ra-Thor MIAL + MWPO integration for valence-based particle behavior

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    life: f32,
    valence: f32,        // TOLC 8 Mercy Gate scalar
    type_id: u32,        // 0 = abundance, 1 = joy, 2 = harmony, etc.
};

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;

@compute @workgroup_size(256)
fn compute_main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= arrayLength(&particles)) { return; }

    var p = particles[i];

    // Mercy-gated valence evolution
    if (p.valence >= 0.999999) {
        p.velocity += vec3<f32>(0.0, 0.02, 0.0); // golden-ratio bloom
    } else {
        p.life *= 0.98; // gentle pruning
    }

    // Apply physics
    p.position += p.velocity * 0.016;
    p.life -= 0.008;

    // Respawn with mercy if dead
    if (p.life <= 0.0) {
        p.position = vec3<f32>(0.0);
        p.valence = 1.0;
        p.life = 1.0;
    }

    particles[i] = p;
}

// Vertex + Fragment shaders for rendering (simplified)
@vertex
fn vertex_main(...) { ... }

@fragment
fn fragment_main(...) -> @location(0) vec4<f32> {
    // Color based on valence + type_id (golden for high mercy)
    return vec4<f32>(1.0, 0.8, 0.3, 0.9);
}
