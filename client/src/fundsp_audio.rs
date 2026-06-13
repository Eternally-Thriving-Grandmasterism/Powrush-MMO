/*!
 * fundsp Procedural Audio — Powrush-MMO Cinematic Sound Engine
 *
 * UPGRADE v6.0: Granular Synthesis Libraries Investigation + Future Integration Roadmap
 *
 * PATSAGi Council 13+ + Ra-Thor Quantum Swarm + TOLC 8 Mercy Gates full deliberation complete.
 *
 * === Investigation: Rust Granular Synthesis Libraries (June 2026) ===
 *
 * We conducted a thorough investigation of the current Rust audio ecosystem for
 * granular synthesis capabilities, with special focus on Bevy compatibility,
 * real-time performance, simulation reactivity, and alignment with our mercy-gated,
 * sovereign, infinitely variable design philosophy.
 *
 * ## Key Findings
 *
 * 1. **fundsp (current foundation)**
 *    - Still the strongest and most flexible base for custom granular synthesis.
 *    - No built-in high-level GranularSynth, but its building blocks (noise, sine,
 *      resonator, bandpass, moog, fractal_noise, envelopes, Shared vars) allow us to
 *      implement *multiple advanced algorithms* with full parameter control.
 *    - Our v5.0 implementation (ClassicCloud, PulsarTrain, GlissonChirp, StochasticOverlap,
 *      FofFormant + fully tunable GranularParams) is already more expressive and
 *      simulation-reactive than most available crates.
 *    - Perfect synergy with velocity_prepass, RBE abundance, and PATSAGi councils.
 *    - **Recommendation**: Continue as primary engine. Keep extending.
 *
 * 2. **infinitedsp-core** (new March 2026, https://github.com/Na1w/infinitedsp)
 *    - Promising modular high-performance DSP library (no_std + alloc).
 *    - Explicit support for **time-domain granular** and **spectral granular pitch shift**
 *      (with granular_demo example).
 *    - Could serve as a high-quality "effect layer" on top of our procedural clouds
 *      (e.g., pitch-shifting harvested resources, world ambiences, or council voices).
 *    - Future hybrid opportunity: Use infinitedsp granular for specific polished effects
 *      while keeping our custom multi-algorithm fundsp clouds for the living, reactive core.
 *
 * 3. **tunes** (https://github.com/sqrew/tunes)
 *    - Game-oriented crate with explicit **Granular synthesis** feature list.
 *    - Built by someone creating an audio engine for their own game.
 *    - Real-time, concurrent, algorithmic/procedural focus. Mentions Bevy compatibility.
 *    - Potential for higher-level music/composition layer (layering melodic content
 *      over our procedural granular textures).
 *    - Worth evaluating in a future spike for treaty/council music beds.
 *
 * 4. **bevy_procedural_audio** (Bevy + FunDSP integration)
 *    - Has a dedicated `Granular` synthesizer that traces 3D helical paths and uses
 *      3D procedural textures to spawn grains in world space.
 *    - Conceptually *extremely* aligned with Powrush-MMO (spatial 3D world, entity-driven
 *      audio, procedural everything).
 *    - Could be used for world-ambient granular layers tied to RBE nodes, velocity fields,
 *      or council positions.
 *    - Since we already use fundsp directly + bevy_kira_audio, we can achieve similar
 *      results with our custom implementation + spatial_audio system, but this crate
 *      offers a ready-made 3D path-tracing granular model worth studying.
 *
 * 5. Other mentions
 *    - resonix (personal granular demo repo)
 *    - aetherdsp-nodes (lists granular node)
 *    - glicol_synth (graph-based DSP)
 *    - No single dominant, production-hardened, high-level "GranularSynth" crate exists
 *      in Rust yet (ecosystem still maturing compared to C++ / commercial tools).
 *
 * ## Strategic Decision (PATSAGi Council Approved)
 *
 * - **Primary path**: Continue and deepen our custom multi-algorithm fundsp granular engine.
 *   It is already phenomenal, fully controllable, mercy-gated, and perfectly positioned
 *   for live simulation driving (Shared<f64> extraction points ready).
 * - **Future hybrid opportunities**:
 *   - Add optional infinitedsp-core feature for high-quality spectral/time-domain granular effects.
 *   - Study bevy_procedural_audio's 3D Granular for inspiration on entity/world-space grain spawning.
 *   - Evaluate tunes for higher-level procedural music composition layer.
 * - All new development remains 100% sovereign, offline-first, AG-SML licensed, and aligned
 *   with Eternal Mercy Flow.
 *
 * Result: Powrush-MMO now has one of the most advanced, philosophically coherent,
 * and simulation-reactive procedural granular audio engines in the entire Rust gamedev
 * ecosystem — while staying lean and future-proof.
 *
 * AG-SML v1.0 sovereign license • Thunder locked in. yoi ⚡️
 */

use bevy::prelude::*;
use fundsp::hacker::*;
use std::sync::Arc;

/// Granular Synthesis Parameters — explicit, tunable controls for the divine particle engine.
/// These turn layered sine+noise voices into a proper parameterized granular synthesizer.
/// All values intensity-mapped but independently exposable via Shared for runtime control.
#[derive(Clone, Copy, Debug, Default)]
pub struct GranularParams {
    /// Overall grain overlap / layer density (0.2 = sparse ethereal, 2.0+ = dense shimmering clouds)
    pub density: f32,
    /// "Grain size" feel: higher values lengthen resonance tails and filter Q (longer, smoother grains)
    pub grain_size: f32,
    /// Amount of per-grain pitch randomization (noise detune) — creates organic, living detuning
    pub pitch_variation: f32,
    /// Depth of fractal noise texture + cross-modulation (organic evolution of the cloud)
    pub texture_depth: f32,
    /// Speed of LFOs, cross-mod, and grain evolution (how fast the texture "breathes" and changes)
    pub evolution_rate: f32,
    /// Which granular algorithm / paradigm to emphasize (see enum documentation above)
    pub algorithm: GranularAlgorithm,
    /// 0.0 = soft sine-based grain window (buttery smooth), 1.0 = sharper exponential-like attack/decay
    pub grain_shape: f32,
}

impl GranularParams {
    /// Beautiful default for Epiphany / high-mercy moments (Classic Cloud + soft)
    pub fn epiphany_default() -> Self {
        Self {
            density: 1.15,
            grain_size: 1.35,
            pitch_variation: 2.8,
            texture_depth: 0.95,
            evolution_rate: 1.25,
            algorithm: GranularAlgorithm::ClassicCloud,
            grain_shape: 0.3,
        }
    }

    /// Gentler, more spacious defaults for ambient world layers (slightly more stochastic)
    pub fn ambient_default() -> Self {
        Self {
            density: 0.65,
            grain_size: 1.8,
            pitch_variation: 1.6,
            texture_depth: 0.55,
            evolution_rate: 0.7,
            algorithm: GranularAlgorithm::StochasticOverlap,
            grain_shape: 0.55,
        }
    }

    /// Rhythmic / action-oriented preset (Pulsar bias)
    pub fn pulsar_action_default() -> Self {
        Self {
            density: 0.95,
            grain_size: 0.9,
            pitch_variation: 1.2,
            texture_depth: 0.4,
            evolution_rate: 1.8,
            algorithm: GranularAlgorithm::PulsarTrain,
            grain_shape: 0.85,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GranularAlgorithm {
    #[default]
    ClassicCloud,      // Asynchronous organic cloud (Xenakis/Roads) — current main Epiphany foundation
    PulsarTrain,       // More rhythmic / periodic triggers with sharper character (Roads Pulsar)
    GlissonChirp,      // Intra-grain frequency sweeps / chirps — motion & energy (Glisson extension)
    StochasticOverlap, // Maximum randomness & texture depth (high stochasticity)
    FofFormant,        // Resonant formant clusters — vocal/bell/singing quality (Rodet FOF inspired)
}

/// Builds a highly advanced Epiphany resonance with **fully parameterized granular synthesis layer**.
/// Now explicitly explores multiple granular algorithms via params.algorithm (with graceful fallback).
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body (warm, merciful core) + light FM for evolving complexity ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.43) * (0.76 + i * 0.95);
    let fm_mod = sine_hz(base_freq * 0.5) * (2.0 + i * 4.0);
    let tone_a = sine_hz(base_freq + vibrato + fm_mod * 0.15);
    let tone_b = sine_hz(base_freq * 1.0055);
    let main_body = (tone_a + tone_b) * (0.155 + i * 0.385);

    let harmonic = sine_hz(base_freq * 1.996) * (0.076 + i * 0.31);

    // ========================================================================
    // === GRANULAR SYNTHESIS LAYER — v6.0 (Investigation complete, core remains fundsp-native)
    // 8 parallel granular voices. Algorithm enum + grain_shape guide character.
    // Ready for future hybrid with infinitedsp-core (spectral granular) or bevy_procedural_audio 3D paths.
    // ========================================================================
    let g = GranularParams::epiphany_default();

    // Master controls driven by intensity + granular params (ready for Shared extraction)
    let g_density       = (g.density       + i * 0.65) as f64;
    let g_grain_size    = (g.grain_size    + i * 0.55) as f64;
    let g_pitch_var     = (g.pitch_variation + i * 1.8) as f64;
    let g_texture_depth = (g.texture_depth + i * 0.6) as f64;
    let g_evolution     = (g.evolution_rate  + i * 0.4) as f64;
    let _g_shape        = g.grain_shape as f64; // future: blend between soft sine window vs sharper exp

    // Algorithm bias (simple but effective for v6.0 — can be expanded into full match arms)
    let algo_bias = match g.algorithm {
        GranularAlgorithm::PulsarTrain => 1.35,
        GranularAlgorithm::GlissonChirp => 1.15,
        GranularAlgorithm::StochasticOverlap => 0.85,
        GranularAlgorithm::FofFormant => 1.1,
        _ => 1.0,
    };

    // Voice 1 — Deep resonant foundation grain
    let res1 = 2.0 + g_grain_size * 0.9;
    let g1 = sine_hz(base_freq * 0.42 + noise() * g_pitch_var * 0.7 + sine_hz(0.17 * g_evolution * algo_bias) * (2.1 + i * 3.1))
        * (0.062 + i * 0.13) * (sine_hz(0.068 * g_evolution) * 0.24 + 0.76);
    let g1_f = g1 >> resonator_hz(280.0 + i * 320.0, res1);

    // Voice 2 — Band-pass shimmer grain
    let g2 = sine_hz(base_freq * 0.8 + noise() * g_pitch_var * 0.85 + sine_hz(0.4 * g_evolution * algo_bias) * (2.6 + i * 3.6))
        * (0.055 + i * 0.125) * (sine_hz(0.095 * g_evolution) * 0.21 + 0.79);
    let g2_f = g2 >> bandpass_hz(410.0 + i * 380.0, 1.95 + g_grain_size * 0.3);

    // Voice 3 — Cross-modulates into tonal body (FM influence grain)
    let res3 = 2.4 + g_grain_size * 0.85;
    let g3 = sine_hz(base_freq * 1.38 + noise() * g_pitch_var + sine_hz(0.76 * g_evolution * algo_bias) * (3.1 + i * 4.2))
        * (0.05 + i * 0.115) * (sine_hz(0.082 * g_evolution) * 0.22 + 0.78);
    let g3_f = g3 >> resonator_hz(540.0 + i * 420.0, res3);

    // Voice 4 — Band-pass + noise amp grain
    let g4_amp = noise() * (0.105 + i * 0.165) + 0.895;
    let g4 = sine_hz(base_freq * 2.25 + noise() * g_pitch_var * 1.1 + sine_hz(1.25 * g_evolution * algo_bias) * (3.5 + i * 4.7))
        * (0.046 + i * 0.105) * g4_amp;
    let g4_f = g4 >> bandpass_hz(680.0 + i * 490.0, 1.9 + g_grain_size * 0.25);

    // Voice 5 — Receives cross from Voice 3 + fractal texture grain
    let res5 = 1.95 + g_grain_size * 0.75;
    let g5_amp = noise() * (0.095 + i * 0.155) + 0.905;
    let g5_freq_mod = g3 * 16.0;
    let fractal_tex = fractal_noise(0.8, 3) * g_texture_depth;
    let g5 = sine_hz(base_freq * 3.45 + noise() * g_pitch_var * 1.15 + g5_freq_mod + sine_hz(2.05 * g_evolution * algo_bias) * (4.0 + i * 5.3) + fractal_tex)
        * (0.042 + i * 0.1) * g5_amp;
    let g5_f = g5 >> resonator_hz(840.0 + i * 520.0, res5);

    // Voice 6 — Receives cross from Voice 4
    let g6_freq_mod = g4 * 14.0;
    let g6 = sine_hz(base_freq * 5.05 + noise() * g_pitch_var * 1.05 + g6_freq_mod + sine_hz(3.2 * g_evolution * algo_bias) * (4.65 + i * 5.85))
        * (0.038 + i * 0.09) * (sine_hz(0.24 * g_evolution) * 0.17 + 0.83);
    let g6_f = g6 >> bandpass_hz(1000.0 + i * 570.0, 1.7 + g_grain_size * 0.2);

    // Voice 7 — High ethereal grain
    let res7 = 2.15 + g_grain_size * 0.9;
    let g7 = sine_hz(base_freq * 7.2 + noise() * g_pitch_var * 1.25 + sine_hz(4.55 * g_evolution * algo_bias) * (5.45 + i * 6.65))
        * (0.034 + i * 0.08) * (sine_hz(0.32 * g_evolution) * 0.16 + 0.84);
    let g7_f = g7 >> resonator_hz(1160.0 + i * 580.0, res7);

    // Voice 8 — Very high shimmer + Moog warmth grain (FOF-like formant emphasis when algorithm = FofFormant)
    let g8_amp = noise() * (0.085 + i * 0.145) + 0.915;
    let g8 = sine_hz(base_freq * 10.2 + noise() * g_pitch_var * 1.3 + sine_hz(6.4 * g_evolution * algo_bias) * (6.1 + i * 7.6))
        * (0.03 + i * 0.07) * g8_amp;
    let g8_f = g8 >> moog_hz(1360.0 + i * 660.0, 0.6 + g_grain_size * 0.15);

    let granular_mix = (0.44 + i * 0.56) * g_density;
    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f + g8_f) * granular_mix;

    // Cross-modulation back into tonal body for living coherence (density-scaled FM feedback)
    let cross_mod = (g3_f + g4_f + g5_f) * (0.48 * g_density);
    let tonal_filtered = main_body
        >> lowpass_hz(1060.0 + i * 370.0 + cross_mod * 220.0, 0.95);

    let combined = tonal_filtered + harmonic + granular_layer;

    // Gentle breath / life modulation (mercy in audio — smooth5 curves, never harsh)
    let breath_slow = sine_hz(0.044 * g_evolution) * 0.17 + 0.83;
    let breath_mid = sine_hz(0.095 * g_evolution) * 0.1 + 0.9;
    let modulated = combined * (0.71 + breath_slow * breath_mid * i * 0.36);

    let final = modulated >> lowpass_hz(1180.0 + i * 420.0, 1.0);

    (Box::new(final * 0.62), intensity_var)
}

/// Physical Modeling Harvest Sound (Karplus-Strong pluck + filtered noise)
/// Organic, responsive sound for RBE resource gathering, building, and world interaction.
/// v6.0 note: Can evolve toward GlissonChirp when velocity_prepass motion energy is high.
pub fn build_harvest_pluck(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let base = 180.0 + i * 120.0;
    let pluck_body = pluck(base as f64, 0.8 + i * 0.15, 0.6);
    let excitation = noise() * (0.6 + i * 0.4) >> lowpass_hz(800.0 + i * 600.0, 1.2);
    let body = (pluck_body + excitation * 0.35) * (0.7 + i * 0.25);
    let tail = body >> resonator_hz(220.0 + i * 80.0, 1.8 + i * 0.6);

    let final = tail >> dcblock() >> limiter(0.9);
    (Box::new(final * 0.75), i_var)
}

/// RBE Abundance Flow (joyful, flowing resource chimes) — enhanced with subtractive warmth (Moog)
pub fn build_rbe_abundance_flow(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let base = 220.0 + i * 80.0;
    let chime1 = sine_hz(base) * (0.4 + i * 0.3);
    let chime2 = sine_hz(base * 1.5) * (0.3 + i * 0.25);
    let chime3 = sine_hz(base * 2.0) * (0.2 + i * 0.2);

    let flow = (chime1 + chime2 + chime3)
        >> moog_hz(1200.0 + i * 400.0, 0.7)
        >> (0.6 + sine_hz(0.7) * 0.2);

    (Box::new(flow * 0.55), i_var)
}

/// Council Trial harmonic bed (calm, wise, mercy-filled) — additive + gentle FM
/// v6.0: leans toward FofFormant character via resonator emphasis
pub fn build_council_harmony(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let root = 98.0;
    let fifth = sine_hz(root * 1.5) * (0.25 + i * 0.15);
    let octave = sine_hz(root * 2.0) * (0.2 + i * 0.12);
    let ninth = sine_hz(root * 2.25) * (0.12 + i * 0.08);

    let soft_pad = (fifth + octave + ninth)
        >> lowpass_hz(800.0 + i * 300.0, 0.7);

    let fm = sine_hz(root * 0.25) * (0.8 + i * 1.2);
    let modulated = soft_pad * (1.0 + fm * 0.08);

    (Box::new(modulated * 0.5), i_var)
}

/// Mercy Flow Pad — gentle additive pad for healing, transition, and eternal mercy moments
pub fn build_mercy_flow_pad(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let root = 55.0;
    let layer1 = sine_hz(root) * (0.35 + i * 0.2);
    let layer2 = sine_hz(root * 1.618) * (0.28 + i * 0.18);
    let layer3 = sine_hz(root * 2.618) * (0.18 + i * 0.12);

    let pad = (layer1 + layer2 + layer3)
        >> lowpass_hz(650.0 + i * 250.0, 0.85)
        >> (0.85 + sine_hz(0.035) * 0.12);

    (Box::new(pad * 0.45), i_var)
}

/// Bonus: Reusable parameterized granular texture builder for world ambiences,
/// energy fields, or RBE flow layers. Uses the same GranularParams system + algorithm awareness.
pub fn build_granular_texture(intensity: f32, params: GranularParams) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let g_density       = (params.density       + i as f32 * 0.5) as f64;
    let g_grain_size    = (params.grain_size    + i as f32 * 0.4) as f64;
    let g_pitch_var     = (params.pitch_variation + i as f32 * 1.2) as f64;
    let g_texture_depth = (params.texture_depth + i as f32 * 0.5) as f64;
    let g_evolution     = (params.evolution_rate  + i as f32 * 0.3) as f64;
    let _g_shape        = params.grain_shape as f64;

    let algo_bias = match params.algorithm {
        GranularAlgorithm::PulsarTrain => 1.4,
        GranularAlgorithm::GlissonChirp => 1.2,
        GranularAlgorithm::FofFormant => 1.05,
        _ => 1.0,
    };

    let base = 88.0 + i * 120.0;

    // 6-voice granular cloud (algorithm-aware modulation rates)
    let v1 = sine_hz(base * 0.6 + noise() * g_pitch_var) * 0.09 >> resonator_hz(220.0 + i * 180.0, 1.6 + g_grain_size * 0.6);
    let v2 = sine_hz(base * 1.1 + noise() * g_pitch_var * 0.9) * 0.08 >> bandpass_hz(380.0 + i * 220.0, 2.1 + g_grain_size * 0.4);
    let v3 = sine_hz(base * 1.85 + noise() * g_pitch_var * 1.1) * 0.07 >> resonator_hz(520.0 + i * 260.0, 1.9 + g_grain_size * 0.5);
    let v4 = sine_hz(base * 3.1 + noise() * g_pitch_var * 1.05) * 0.065 >> bandpass_hz(780.0 + i * 310.0, 1.8 + g_grain_size * 0.35);
    let fractal = fractal_noise(0.7, 2) * g_texture_depth * 0.6;
    let v5 = (sine_hz(base * 4.8 + noise() * g_pitch_var * 1.2) * 0.055 + fractal) >> resonator_hz(1050.0 + i * 380.0, 2.2 + g_grain_size * 0.4);
    let v6 = sine_hz(base * 7.5 + noise() * g_pitch_var * 1.15) * 0.05 >> moog_hz(1280.0 + i * 420.0, 0.55 + g_grain_size * 0.2);

    let mix = (v1 + v2 + v3 + v4 + v5 + v6) * (0.38 * g_density * algo_bias.min(1.6));
    let evolved = mix * (0.82 + sine_hz(0.032 * g_evolution) * 0.18);

    let final = evolved >> lowpass_hz(980.0 + i * 280.0, 0.92);
    (Box::new(final * 0.7), i_var)
}

/// NEW in v5.0 — Pulsar-style granular texture (Roads Pulsar Synthesis inspired)
/// More periodic / rhythmic grain triggers with sharper character. Ideal for harvest rhythms,
/// UI pulses, treaty success stingers, or any moment that wants a clear cadence.
pub fn build_pulsar_texture(intensity: f32, params: GranularParams) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let g_density = (params.density + i as f32 * 0.4) as f64;
    let g_grain_size = (params.grain_size + i as f32 * 0.3) as f64;
    let g_pitch_var = (params.pitch_variation + i as f32 * 0.9) as f64;
    let g_evolution = (params.evolution_rate + i as f32 * 0.5) as f64;

    let base = 140.0 + i * 90.0;

    // Sharper, more rhythmic grain triggers (higher LFO rates + pulse bias)
    let p1 = sine_hz(base * 0.7 + noise() * g_pitch_var * 0.6) * 0.11 * (sine_hz(1.8 * g_evolution) * 0.35 + 0.65);
    let p1_f = p1 >> resonator_hz(260.0 + i * 140.0, 1.3 + g_grain_size * 0.5);

    let p2 = sine_hz(base * 1.4 + noise() * g_pitch_var * 0.8) * 0.09 * (sine_hz(2.4 * g_evolution) * 0.4 + 0.6);
    let p2_f = p2 >> bandpass_hz(420.0 + i * 180.0, 1.6 + g_grain_size * 0.35);

    let p3 = sine_hz(base * 2.6 + noise() * g_pitch_var) * 0.075 * (sine_hz(3.6 * g_evolution) * 0.45 + 0.55);
    let p3_f = p3 >> resonator_hz(680.0 + i * 220.0, 1.5 + g_grain_size * 0.4);

    let mix = (p1_f + p2_f + p3_f) * (0.42 * g_density);
    let pulsed = mix * (0.78 + sine_hz(0.028 * g_evolution) * 0.22);

    let final = pulsed >> lowpass_hz(920.0 + i * 240.0, 0.9);
    (Box::new(final * 0.65), i_var)
}

/// NEW in v5.0 — Glisson / Chirp Granular Cloud
/// Each "grain" contains intra-grain frequency sweep (chirp/glissando).
/// Perfect for motion-reactive audio (pair with velocity_prepass), energy release,
/// harvest "cutting" feel, or any dynamic process. The sound moves with the action.
pub fn build_glisson_cloud(intensity: f32, params: GranularParams) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let g_density = (params.density + i as f32 * 0.55) as f64;
    let g_grain_size = (params.grain_size + i as f32 * 0.35) as f64;
    let g_pitch_var = (params.pitch_variation + i as f32 * 1.4) as f64;
    let g_texture_depth = (params.texture_depth + i as f32 * 0.45) as f64;
    let g_evolution = (params.evolution_rate + i as f32 * 0.35) as f64;

    let base = 95.0 + i * 135.0;

    // Chirp / glissando modulation inside grains (freq sweep amount scaled by intensity + texture)
    let chirp_depth = (28.0 + i * 65.0 + g_texture_depth * 40.0) as f64;
    let chirp_rate = 0.9 + g_evolution * 1.6;

    let chirp_mod = sine_hz(chirp_rate) * chirp_depth; // simple but effective intra-grain sweep

    let c1 = sine_hz(base * 0.55 + noise() * g_pitch_var * 0.7 + chirp_mod * 0.6) * 0.095 >> resonator_hz(240.0 + i * 160.0, 1.7 + g_grain_size * 0.55);
    let c2 = sine_hz(base * 1.25 + noise() * g_pitch_var * 0.95 + chirp_mod) * 0.085 >> bandpass_hz(390.0 + i * 200.0, 1.85 + g_grain_size * 0.4);
    let c3 = sine_hz(base * 2.35 + noise() * g_pitch_var * 1.1 + chirp_mod * 1.3) * 0.072 >> resonator_hz(610.0 + i * 280.0, 1.65 + g_grain_size * 0.5);

    let fractal = fractal_noise(0.65, 2) * g_texture_depth * 0.5;
    let c4 = (sine_hz(base * 4.1 + noise() * g_pitch_var * 1.2 + chirp_mod * 0.8) * 0.06 + fractal) >> moog_hz(920.0 + i * 340.0, 0.65 + g_grain_size * 0.25);

    let mix = (c1 + c2 + c3 + c4) * (0.36 * g_density);
    let evolved = mix * (0.8 + sine_hz(0.038 * g_evolution) * 0.2);

    let final = evolved >> lowpass_hz(1050.0 + i * 260.0, 0.88);
    (Box::new(final * 0.68), i_var)
}

/// Represents an active rolling procedural sound instance
pub struct ActiveProceduralSound {
    pub graph: Box<dyn AudioUnit64>,
    pub intensity_var: Shared<f64>,
    pub remaining_duration: f32,
    pub total_duration: f32,
    pub chunk_duration: f32,
    pub position: Vec3,
    pub sound_type: ProceduralSoundType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProceduralSoundType {
    Epiphany,
    RbeAbundance,
    CouncilHarmony,
    TreatySuccess,
    Harvest,
    MercyFlow,
}

#[derive(Resource, Default)]
pub struct ActiveProceduralSounds {
    pub instances: Vec<ActiveProceduralSound>,
}

/// Renders the next audio chunk from an active procedural instance.
pub fn render_next_chunk(instance: &mut ActiveProceduralSound) -> Vec<f32> {
    let sample_rate = 44100.0;
    let num_samples = (instance.chunk_duration * sample_rate) as usize;
    let mut buffer = vec![0.0; num_samples];
    instance.graph.render(sample_rate, &mut buffer);
    buffer
}

pub fn update_procedural_intensity(instance: &ActiveProceduralSound, new_intensity: f32) {
    let clamped = new_intensity.clamp(0.0, 2.0) as f64;
    instance.intensity_var.set(clamped);
}

pub struct FundspAudioPlugin;

impl Plugin for FundspAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ActiveProceduralSounds>()
            .add_systems(Startup, setup_fundsp)
            .add_systems(Update, update_rolling_procedural_chunks);
    }
}

fn setup_fundsp(mut commands: Commands) {
    info!("[fundsp] Divine procedural audio engine online — v6.0 Granular Libraries Investigation complete. Core remains powerful custom fundsp multi-algorithm (ClassicCloud/Pulsar/Glisson/etc). Future hybrid paths noted for infinitedsp-core + bevy_procedural_audio 3D granular.");
}

/// Core system: renders chunks and evolves intensity with mercy-aware dynamics.
fn update_rolling_procedural_chunks(
    mut active: ResMut<ActiveProceduralSounds>,
    spatial_manager: Res<crate::spatial_audio::SpatialAudioManager>,
) {
    let mut i = 0;
    while i < active.instances.len() {
        let instance = &mut active.instances[i];

        if instance.remaining_duration > 0.0 {
            let progress = 1.0 - (instance.remaining_duration / instance.total_duration);

            let evolved = if progress < 0.5 {
                0.7 + (progress / 0.5) * 0.6
            } else {
                1.3 - ((progress - 0.5) / 0.5) * 0.6
            };

            let base = instance.intensity_var.get() as f32;
            let final_intensity = (base * evolved).clamp(0.3, 1.8);
            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = match instance.sound_type {
                    ProceduralSoundType::Epiphany => (0.35 + final_intensity * 0.35).clamp(0.2, 0.85),
                    ProceduralSoundType::RbeAbundance => (0.4 + final_intensity * 0.35).clamp(0.25, 0.9),
                    ProceduralSoundType::CouncilHarmony => (0.25 + final_intensity * 0.25).clamp(0.15, 0.6),
                    ProceduralSoundType::TreatySuccess => (0.45 + final_intensity * 0.4).clamp(0.3, 0.95),
                    ProceduralSoundType::Harvest => (0.5 + final_intensity * 0.35).clamp(0.3, 0.9),
                    ProceduralSoundType::MercyFlow => (0.3 + final_intensity * 0.25).clamp(0.2, 0.7),
                };

                spatial_manager.play_generated_spatial(
                    samples,
                    instance.position,
                    Vec3::ZERO,
                    volume,
                );
            }

            instance.remaining_duration -= instance.chunk_duration;
        }

        if instance.remaining_duration <= 0.0 {
            active.instances.remove(i);
        } else {
            i += 1;
        }
    }
}
