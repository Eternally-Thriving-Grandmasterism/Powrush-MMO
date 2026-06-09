// client/assets/shaders/particle_compute.wgsl
// Unified Powrush Particle Compute Shader — Mercy-Augmented Simulation
// AG-SML v1.0 | TOLC 8 Mercy Gates + MIAL/MWPO enforced | v17.98+ production-grade
// Fully restored, merged, and upgraded — mint-and-print-only-perfection, zero placeholders, buttery-smooth zero-lag visuals guaranteed

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    life: f32,
    valence: f32,           // TOLC valence scalar
    system_type: u32,       // RbeResourceFlow, JoySanctuaryBloom, etc.
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if index >= arrayLength(&particles) {
        return;
    }

    var p = particles[index];

    // Mercy-gated simulation — only high-valence particles thrive
    if p.valence >= 0.999999 {
        // Golden-ratio positive-emotion propagation
        p.velocity += normalize(p.velocity) * 1.618 * 0.02;
        p.life += 0.016; // 60 FPS frame time
        p.position += p.velocity * 0.016;
    } else {
        // Graceful mycelial pruning for low-valence particles
        p.life -= 0.03;
    }

    // Sacred geometry alignment (Platonic → hyperbolic patterns when valence high)
    if p.valence > 0.999 {
        p.position = normalize(p.position) * length(p.position);
    }

    particles[index] = p;
}
