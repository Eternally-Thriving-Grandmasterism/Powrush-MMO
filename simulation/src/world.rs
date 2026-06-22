/*!
 * Named presets for different phase/invariant profiles + time-varying phase.
 */

#[derive(Clone, Copy, Debug)]
pub enum LissajousKnotPreset {
    TrefoilLike,      // Low asymmetry, classic (2,3)-like
    HighWrithe,       // Strong asymmetry → high chirality
    Symmetric,        // Low writhe, more reversible
    Complex5_3_4,     // Rich multi-looped with breathing
}

impl LissajousKnotPreset {
    pub fn radial_values(&self) -> (f32, f32, f32) {
        match self {
            LissajousKnotPreset::TrefoilLike => (0.12, 0.25, 0.18),   // Gentle phase
            LissajousKnotPreset::HighWrithe  => (0.08, 0.45, 0.70),   // Strong asymmetry
            LissajousKnotPreset::Symmetric   => (0.30, 0.32, 0.31),   // Very balanced
            LissajousKnotPreset::Complex5_3_4 => (0.15, 0.35, 0.55),  // Current rich variant
        }
    }
}

// Example usage in setup:
// let preset = LissajousKnotPreset::HighWrithe;
// let (rx, ry, rz) = preset.radial_values();
// Then use rx, ry, rz in the three InitVelocityTangentModifier calls.

// Time-varying phase example (can be driven from a system)
pub fn update_knot_phase(
    time: Res<Time>,
    mut particle_effects: ResMut<PolicyParticleEffects>,
) {
    // Slowly oscillate one radial component over time
    let t = time.elapsed_secs() * 0.15;
    let oscillation = (t.sin() * 0.15 + 0.35).clamp(0.15, 0.55);

    // In a full implementation, we would recreate or modulate the EffectAsset
    // For now this demonstrates the concept of dynamic phase.
}
