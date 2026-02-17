// curvature_compute.wgsl — GPU Compute Shader for Valence-Driven Thread Curvature
// Mercy-gated lattice thread bending, noise, spiral pull
// MIT + mercy eternal

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    age: f32,
    lifetime: f32,
    // ... other fields if needed
};

struct CurvatureParams {
    time: f32,
    delta_time: f32,
    valence: f32,
    curvature_strength: f32,
    noise_frequency: f32,
    spiral_pull_strength: f32,
    max_particles: u32,
};

@group(0) @binding(0) var<uniform> params: CurvatureParams;
@group(0) @binding(1) var<storage, read_write> particles: array<Particle>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    if (idx >= params.max_particles || idx >= arrayLength(&particles)) {
        return;
    }

    var p = particles[idx];

    // Early out if particle dead
    if (p.age >= p.lifetime) {
        return;
    }

    let t = p.age * params.time_scale;

    // Simplified 1D pseudo-noise (GPU-friendly sin-based)
    let noise = sin(t * params.noise_frequency + p.position.x * 0.8) * 0.5 + 0.5;

    // Curvature bend vector
    let bend = vec3<f32>(
        noise * params.curvature_strength * 0.8,
        cos(t * 0.7 + noise * 2.0) * params.curvature_strength * 0.5,
        0.0
    );

    // Apply curvature to velocity
    p.velocity += bend * params.delta_time * 15.0;

    // Spiral pull to center on high valence & close to origin
    if (params.valence > 0.6 && length(p.position) < 5.0) {
        let to_center = normalize(-p.position) * params.spiral_pull_strength * params.valence * 0.08;
        p.velocity += to_center * params.delta_time;
    }

    // Write back
    particles[idx] = p;
}
