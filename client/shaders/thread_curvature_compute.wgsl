// thread_curvature_compute.wgsl — GPU Compute for Valence-Driven Thread Curvature
// Mercy-gated: bend + noise + spiral pull per particle
// MIT + mercy eternal

struct Particle {
    @location(0) position: vec3<f32>,
    @location(1) velocity: vec3<f32>,
    @location(2) age: f32,
    @location(3) lifetime: f32,
    @location(4) padding: vec2<f32>, // alignment
};

struct CurvatureParams {
    time: f32,
    delta_time: f32,
    valence: f32,
    curvature_strength: f32,
    noise_frequency: f32,
    spiral_pull_strength: f32,
    max_particles: u32,
    @align(16) seed: vec4<f32>, // for noise hash
};

@group(0) @binding(0) var<uniform> params: CurvatureParams;
@group(0) @binding(1) var<storage, read_write> particles: array<Particle>;

fn hash(p: vec3<f32>) -> f32 {
    return fract(sin(dot(p, vec3<f32>(12.9898, 78.233, 45.5432))) * 43758.5453);
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    if (idx >= params.max_particles || idx >= arrayLength(&particles)) {
        return;
    }

    var p = particles[idx];

    // Early out if dead
    if (p.age >= p.lifetime) {
        return;
    }

    let t = p.age * 1.2; // time_scale baked in

    // Fast 3D pseudo-noise (GPU-friendly)
    let noise_pos = p.position * params.noise_frequency + vec3<f32>(t);
    let noise = hash(noise_pos) * 2.0 - 1.0; // -1..1 range

    // Curvature bend vector
    var bend = vec3<f32>(
        noise * params.curvature_strength * 0.9,
        sin(t * 0.8 + noise * 3.0) * params.curvature_strength * 0.6,
        0.0
    );

    // Apply bend to velocity
    p.velocity += bend * params.delta_time * 18.0;

    // Spiral pull to center on high valence & close to origin
    let dist = length(p.position);
    if (params.valence > 0.6 && dist < 6.0) {
        let to_center = normalize(-p.position) * params.spiral_pull_strength * params.valence * 0.12;
        p.velocity += to_center * params.delta_time;
    }

    // Write back
    particles[idx] = p;
}
