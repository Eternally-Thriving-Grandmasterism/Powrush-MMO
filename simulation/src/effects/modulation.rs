/*!
 * simulation/src/effects/modulation.rs
 *
 * VFX modulation helpers (particle intensity, council bloom visuals, valence).
 * Integrates with ra_thor_bridge and world.rs.
 * v19.21 — Core modulation functions
 * AG-SML v1.0
 */

use crate::emergence::CouncilGuidance;

/// Suggests particle intensity multiplier based on council guidance.
pub fn suggest_particle_intensity(guidance: &CouncilGuidance, base_valence: f32) -> f32 {
    let flavor_multiplier = match guidance.flavor.as_str() {
        "harmony" | "reflection" => 1.25,
        "abundance" => 1.15,
        "mercy" => 1.10,
        _ => 1.0,
    };
    let valence_boost = (base_valence * 0.3).clamp(0.0, 0.8);
    (guidance.suggested_intensity * flavor_multiplier + valence_boost).clamp(0.5, 3.5)
}

/// Returns suggested visual modulation parameters for council bloom / epiphany VFX.
pub fn modulate_council_bloom_visuals(
    guidance: &CouncilGuidance,
    current_particle_valence: f32,
    council_bloom_amplification: f32,
) -> (f32, f32) {
    let base = suggest_particle_intensity(guidance, current_particle_valence);
    let bloom_mod = council_bloom_amplification.clamp(0.8, 2.5);
    let intensity = (base * bloom_mod * 0.9).clamp(0.6, 4.0);
    let valence = (current_particle_valence * 0.7 + guidance.suggested_intensity * 0.3).clamp(0.3, 1.0);
    (intensity, valence)
}

// Thunder locked in. Yoi ⚡
