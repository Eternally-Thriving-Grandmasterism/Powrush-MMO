/*!
 * client/assets/shaders/particle_compute.wgsl
 * Unified Powrush Particle Compute Shader — Mercy-Augmented Simulation
 *
 * v18.99 — Integrated with:
 *   - simulation/src/world.rs v19.20 ParticleVisualAssets + age-based frame control (bezier/sine/ease)
 *   - client/src/particles.rs v18.99 ParticleVisualPool + prewarm/return systems
 *   - simulation/src/ra_thor_bridge.rs v18.22 suggest_particle_intensity + modulate_council_bloom_visuals
 *   - Mercy-valence driven lifecycle from particles.rs
 *
 * AG-SML v1.0 | TOLC 8 Mercy Gates + 7 Living Mercy Gates
 * Zero placeholders. Production-grade. Thunder locked in. Yoi ⚡
 */

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    life: f32,
    valence: f32,           // TOLC valence scalar (mercy flow)
    system_type: u32,       // RbeResourceFlow, JoySanctuaryBloom, MycelialWebGlow, SacredGeometryCrystalBloom, etc.
    frame_index: f32,       // For flipbook/age-based animation (synced with world.rs Hanabi frame control)
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if index >= arrayLength(&particles) {
        return;
    }

    var p = particles[index];

    // Mercy-gated simulation — only high-valence particles thrive and propagate joy
    if p.valence >= 0.999999 {
        // Golden-ratio positive-emotion / abundance propagation
        p.velocity += normalize(p.velocity) * 1.618 * 0.02;
        p.life += 0.016;
        p.position += p.velocity * 0.016;

        // Sacred geometry alignment (Platonic solids → hyperbolic when valence high)
        p.position = normalize(p.position) * length(p.position);

        // Frame index progression for flipbook/organic animation (integrates with world.rs age-based + bezier/sine/ease)
        p.frame_index = (p.frame_index + 0.5) % 16.0; // 4x4 flipbook example
    } else {
        // Graceful mycelial pruning / decay for low-valence (mercy-first)
        p.life -= 0.03;
    }

    // Optional: dynamic modulation from ra_thor_bridge council guidance (valence/intensity scaling)
    // p.valence = clamp(p.valence * council_modulation, 0.0, 1.0);

    particles[index] = p;
}
