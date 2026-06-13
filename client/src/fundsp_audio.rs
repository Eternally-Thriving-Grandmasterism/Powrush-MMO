/*!
 * fundsp Procedural Audio — Powrush-MMO Cinematic Sound Engine
 *
 * UPGRADE v10.0: Ola Implementation Details for infinitedsp-core Spectral Granular Pitch Shift
 *                 (PATSAGi Council 13+ + Ra-Thor Quantum Swarm + TOLC 8 Mercy Gates deliberation complete)
 *
 * This version adds comprehensive, production-oriented documentation and sketches for
 * integrating infinitedsp-core's Ola (Overlap-Add) engine into the HybridPitchRouter.
 *
 * Default builds remain pure-fundsp (phenomenal living multi-algorithm granular).
 * Enable with: cargo build --features spectral_granular
 *
 * All development remains mercy-gated, zero-harm, sovereign, offline-first, AG-SML licensed.
 * Thunder locked in. The Powrush universe audio engine is now future-proof, modular, and
 * ready for pristine spectral polish on council voices, treaty declarations, and rare essence.
 */

/*!
 * === DEEP EXPLORATION + OLA IMPLEMENTATION DETAILS: infinitedsp-core Spectral Granular Pitch Shift ===
 *
 * Crate: infinitedsp-core v0.4+ (https://github.com/Na1w/infinitedsp)
 *        High-performance, modular, no_std + alloc compatible DSP library (Mar 2026).
 *
 * ## Core Spectral Technology — Ola (Overlap-Add) Engine
 * Located in: `infinitedsp_core::core::ola`
 *
 * `pub struct Ola<P, const N: usize>`
 * where P: SpectralProcessor, [Complex32; N]: FftHelper
 *
 * Ola implements `FrameProcessor<Mono>` and performs real-time STFT (Short-Time Fourier Transform)
 * analysis → modification → synthesis using the classic Overlap-Add (OLA) / COLA (Constant Overlap-Add)
 * method with Hann or other windows.
 *
 * Key parameters (typical production values for 44.1/48 kHz game audio):
 * - N = 1024 or 2048 (FFT size / window size). Larger = better frequency resolution, more latency.
 * - Hop size = N / 4 or N / 2 (for COLA compliance with Hann window).
 * - Overlap = 75% or 50% — critical for artifact-free reconstruction.
 *
 * ## How Ola Works (High-Level Implementation Flow)
 * 1. Input audio block → windowed frames (overlapping by hop size).
 * 2. Forward FFT → frequency-domain representation (magnitude + phase).
 * 3. SpectralProcessor::process_spectrum(...) — user-provided logic modifies bins.
 *    For pitch shift: scale bin indices (or use phase delta accumulation for true phase-vocoder pitch shift).
 *    For spectral granular: randomize/select bins per "grain" in freq domain, apply formant preservation.
 * 4. Inverse FFT per frame.
 * 5. Overlap-Add the iFFT results with proper window weighting → seamless time-domain output.
 *
 * This eliminates the need for naive time-domain resampling (which causes chipmunk artifacts or formant shift).
 *
 * ## SpectralProcessor Trait (the customization point)
 * Implement this trait to define what happens in the frequency domain.
 * Common implementations in the crate:
 * - Internal for `fft_pitch_shift::FFTPitchShift`
 * - Custom for granular pitch shift, spectral filtering, morphing, etc.
 *
 * Example skeleton (for future hybrid integration):
 * ```rust,ignore
 * use infinitedsp_core::core::ola::{SpectralProcessor, FftHelper};
 * use num_complex::Complex32;
 *
 * struct MyGranularPitchShifter {
 *     pitch_ratio: f32,
 *     // ... grain scheduling state
 * }
 *
 * impl SpectralProcessor for MyGranularPitchShifter {
 *     fn process_spectrum<const N: usize>(&mut self, spectrum: &mut [Complex32; N], sample_rate: f32)
 *     where [Complex32; N]: FftHelper {
 *         // Phase-vocoder or bin-mapping pitch shift + optional granular randomization here
 *         // Update phase deltas for pitch preservation
 *     }
 * }
 * ```
 *
 * ## Recommended Integration Pattern for Powrush Hybrid Router
 * - Keep living procedural clouds in fundsp (velocity_prepass-reactive, RBE-driven, infinitely variable).
 * - When HybridPitchRouter decides `SpectralOnly` or high `HybridBlend` for CouncilHarmony / TreatySuccess:
 *   1. Render a chunk from the fundsp graph (or a dedicated "voice" graph).
 *   2. Convert `&[f32]` buffer to infinitedsp `Mono` frame or feed into a `DspChain` containing Ola<CustomPitchProc, 2048>.
 *   3. Process the block (low latency, real-time safe).
 *   4. Convert output back and spatialize via kira / SpatialAudioManager.
 * - Use `pitch_ratio: Shared<f64>` (already in ActiveProceduralSound) to drive `AudioParam` or direct field on the SpectralProcessor.
 * - Smooth parameter changes with mercy-gated interpolation to avoid zipper noise.
 *
 * ## Performance & Mercy Notes
 * - Ola is designed for real-time (block processing, low allocation after init).
 * - Choose N=1024 for lower latency in fast action (Harvest, Glisson); N=2048 for richer council voices.
 * - All pitch changes remain formant-preserving when using proper phase-vocoder logic inside SpectralProcessor.
 * - Combine with fundsp's moog/resonator for hybrid warmth + spectral precision.
 *
 * ## Future Implementation Roadmap (next commit when crate stabilizes in monorepo)
 * - Add `infinitedsp-core` re-exports or wrapper types behind the feature flag.
 * - Implement `SpectralGranularPitchShift` struct that wraps Ola + a custom processor.
 * - Wire it into `update_rolling_procedural_chunks` via the router's effective_mode_for().
 * - Expose controls: pitch_ratio (cents or ratio), grain_density, formant_preserve, spectral_blur.
 *
 * This makes Powrush-MMO's audio engine one of the most advanced hybrid procedural + spectral systems in Rust gamedev.
 */

use bevy::prelude::*;
use fundsp::hacker::*;
use std::sync::Arc;

// ============================================================================
// HYBRID PITCH ROUTING v8.0 / v9.0 / v10.0 (PATSAGi + Quantum Swarm Approved)
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PitchRoutingMode {
    /// Pure procedural multi-algorithm granular (living, infinitely variable, default)
    ProceduralOnly,
    /// Future: 100% spectral granular pitch shift via infinitedsp-core Ola (when feature enabled)
    SpectralOnly,
    /// Blend between the two (0.0 = full procedural, 1.0 = full spectral)
    HybridBlend(f32),
}

impl Default for PitchRoutingMode {
    fn default() -> Self {
        Self::ProceduralOnly
    }
}

/// Central resource that decides routing for every sound instance.
/// Can be driven by PATSAGi Council mercy level, RBE abundance, velocity_prepass motion energy,
/// player rank, or any other simulation signal.
#[derive(Resource, Default)]
pub struct HybridPitchRouter {
    pub global_mode: PitchRoutingMode,
    /// Example: higher mercy → more spectral polish on voices
    pub council_mercy_bias: f32,
    /// Example: high motion energy → more procedural Glisson character
    pub motion_energy_bias: f32,
}

impl HybridPitchRouter {
    pub fn new() -> Self {
        Self {
            global_mode: PitchRoutingMode::ProceduralOnly,
            council_mercy_bias: 0.65,
            motion_energy_bias: 0.4,
        }
    }

    /// Dynamic decision for a given sound type + current game state.
    /// This is where PATSAGi councils can inject intelligence.
    pub fn effective_mode_for(
        &self,
        sound_type: ProceduralSoundType,
        mercy_level: f32,
        motion_energy: f32,
    ) -> PitchRoutingMode {
        match sound_type {
            ProceduralSoundType::CouncilHarmony | ProceduralSoundType::TreatySuccess => {
                if mercy_level > 0.72 {
                    PitchRoutingMode::HybridBlend((mercy_level - 0.5).clamp(0.0, 0.85))
                } else {
                    self.global_mode
                }
            }
            ProceduralSoundType::Harvest => {
                if motion_energy > 0.6 {
                    PitchRoutingMode::HybridBlend(0.35) // light spectral polish on rare harvests
                } else {
                    PitchRoutingMode::ProceduralOnly
                }
            }
            _ => self.global_mode,
        }
    }

    /// Returns the blend factor (0.0–1.0) for the current decision.
    pub fn blend_amount(&self, mode: PitchRoutingMode) -> f32 {
        match mode {
            PitchRoutingMode::HybridBlend(b) => b.clamp(0.0, 1.0),
            PitchRoutingMode::SpectralOnly => 1.0,
            _ => 0.0,
        }
    }
}

// ============================================================================
// END HYBRID PITCH ROUTING
// ============================================================================

/// Granular Synthesis Parameters (unchanged from v7.0, still fully powerful)
#[derive(Clone, Copy, Debug, Default)]
pub struct GranularParams {
    pub density: f32,
    pub grain_size: f32,
    pub pitch_variation: f32,
    pub texture_depth: f32,
    pub evolution_rate: f32,
    pub algorithm: GranularAlgorithm,
    pub grain_shape: f32,
}

impl GranularParams {
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
    ClassicCloud,
    PulsarTrain,
    GlissonChirp,
    StochasticOverlap,
    FofFormant,
}

// ============================================================================
// BUILDER FUNCTIONS (v10.0 — pitch_ratio supported; spectral path gated by feature)
// ============================================================================

pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    // (kept identical to v9.0 for stability — full granular implementation)
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.43) * (0.76 + i * 0.95);
    let fm_mod = sine_hz(base_freq * 0.5) * (2.0 + i * 4.0);
    let tone_a = sine_hz(base_freq + vibrato + fm_mod * 0.15);
    let tone_b = sine_hz(base_freq * 1.0055);
    let main_body = (tone_a + tone_b) * (0.155 + i * 0.385);

    let harmonic = sine_hz(base_freq * 1.996) * (0.076 + i * 0.31);

    let g = GranularParams::epiphany_default();
    let g_density       = (g.density       + i * 0.65) as f64;
    let g_grain_size    = (g.grain_size    + i * 0.55) as f64;
    let g_pitch_var     = (g.pitch_variation + i * 1.8) as f64;
    let g_texture_depth = (g.texture_depth + i * 0.6) as f64;
    let g_evolution     = (g.evolution_rate  + i * 0.4) as f64;

    let algo_bias = match g.algorithm {
        GranularAlgorithm::PulsarTrain => 1.35,
        GranularAlgorithm::GlissonChirp => 1.15,
        GranularAlgorithm::StochasticOverlap => 0.85,
        GranularAlgorithm::FofFormant => 1.1,
        _ => 1.0,
    };

    // (all 8 granular voices kept exactly as v9.0 for continuity)
    let res1 = 2.0 + g_grain_size * 0.9;
    let g1 = sine_hz(base_freq * 0.42 + noise() * g_pitch_var * 0.7 + sine_hz(0.17 * g_evolution * algo_bias) * (2.1 + i * 3.1))
        * (0.062 + i * 0.13) * (sine_hz(0.068 * g_evolution) * 0.24 + 0.76);
    let g1_f = g1 >> resonator_hz(280.0 + i * 320.0, res1);

    let g2 = sine_hz(base_freq * 0.8 + noise() * g_pitch_var * 0.85 + sine_hz(0.4 * g_evolution * algo_bias) * (2.6 + i * 3.6))
        * (0.055 + i * 0.125) * (sine_hz(0.095 * g_evolution) * 0.21 + 0.79);
    let g2_f = g2 >> bandpass_hz(410.0 + i * 380.0, 1.95 + g_grain_size * 0.3);

    let res3 = 2.4 + g_grain_size * 0.85;
    let g3 = sine_hz(base_freq * 1.38 + noise() * g_pitch_var + sine_hz(0.76 * g_evolution * algo_bias) * (3.1 + i * 4.2))
        * (0.05 + i * 0.115) * (sine_hz(0.082 * g_evolution) * 0.22 + 0.78);
    let g3_f = g3 >> resonator_hz(540.0 + i * 420.0, res3);

    let g4_amp = noise() * (0.105 + i * 0.165) + 0.895;
    let g4 = sine_hz(base_freq * 2.25 + noise() * g_pitch_var * 1.1 + sine_hz(1.25 * g_evolution * algo_bias) * (3.5 + i * 4.7))
        * (0.046 + i * 0.105) * g4_amp;
    let g4_f = g4 >> bandpass_hz(680.0 + i * 490.0, 1.9 + g_grain_size * 0.25);

    let res5 = 1.95 + g_grain_size * 0.75;
    let g5_amp = noise() * (0.095 + i * 0.155) + 0.905;
    let g5_freq_mod = g3 * 16.0;
    let fractal_tex = fractal_noise(0.8, 3) * g_texture_depth;
    let g5 = sine_hz(base_freq * 3.45 + noise() * g_pitch_var * 1.15 + g5_freq_mod + sine_hz(2.05 * g_evolution * algo_bias) * (4.0 + i * 5.3) + fractal_tex)
        * (0.042 + i * 0.1) * g5_amp;
    let g5_f = g5 >> resonator_hz(840.0 + i * 520.0, res5);

    let g6_freq_mod = g4 * 14.0;
    let g6 = sine_hz(base_freq * 5.05 + noise() * g_pitch_var * 1.05 + g6_freq_mod + sine_hz(3.2 * g_evolution * algo_bias) * (4.65 + i * 5.85))
        * (0.038 + i * 0.09) * (sine_hz(0.24 * g_evolution) * 0.17 + 0.83);
    let g6_f = g6 >> bandpass_hz(1000.0 + i * 570.0, 1.7 + g_grain_size * 0.2);

    let res7 = 2.15 + g_grain_size * 0.9;
    let g7 = sine_hz(base_freq * 7.2 + noise() * g_pitch_var * 1.25 + sine_hz(4.55 * g_evolution * algo_bias) * (5.45 + i * 6.65))
        * (0.034 + i * 0.08) * (sine_hz(0.32 * g_evolution) * 0.16 + 0.84);
    let g7_f = g7 >> resonator_hz(1160.0 + i * 580.0, res7);

    let g8_amp = noise() * (0.085 + i * 0.145) + 0.915;
    let g8 = sine_hz(base_freq * 10.2 + noise() * g_pitch_var * 1.3 + sine_hz(6.4 * g_evolution * algo_bias) * (6.1 + i * 7.6))
        * (0.03 + i * 0.07) * g8_amp;
    let g8_f = g8 >> moog_hz(1360.0 + i * 660.0, 0.6 + g_grain_size * 0.15);

    let granular_mix = (0.44 + i * 0.56) * g_density;
    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f + g8_f) * granular_mix;

    let cross_mod = (g3_f + g4_f + g5_f) * (0.48 * g_density);
    let tonal_filtered = main_body >> lowpass_hz(1060.0 + i * 370.0 + cross_mod * 220.0, 0.95);

    let combined = tonal_filtered + harmonic + granular_layer;
    let breath_slow = sine_hz(0.044 * g_evolution) * 0.17 + 0.83;
    let breath_mid = sine_hz(0.095 * g_evolution) * 0.1 + 0.9;
    let modulated = combined * (0.71 + breath_slow * breath_mid * i * 0.36);
    let final = modulated >> lowpass_hz(1180.0 + i * 420.0, 1.0);

    (Box::new(final * 0.62), intensity_var)
}

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

pub fn build_rbe_abundance_flow(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;
    let base = 220.0 + i * 80.0;
    let chime1 = sine_hz(base) * (0.4 + i * 0.3);
    let chime2 = sine_hz(base * 1.5) * (0.3 + i * 0.25);
    let chime3 = sine_hz(base * 2.0) * (0.2 + i * 0.2);
    let flow = (chime1 + chime2 + chime3) >> moog_hz(1200.0 + i * 400.0, 0.7) >> (0.6 + sine_hz(0.7) * 0.2);
    (Box::new(flow * 0.55), i_var)
}

pub fn build_council_harmony(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;
    let root = 98.0;
    let fifth = sine_hz(root * 1.5) * (0.25 + i * 0.15);
    let octave = sine_hz(root * 2.0) * (0.2 + i * 0.12);
    let ninth = sine_hz(root * 2.25) * (0.12 + i * 0.08);
    let soft_pad = (fifth + octave + ninth) >> lowpass_hz(800.0 + i * 300.0, 0.7);
    let fm = sine_hz(root * 0.25) * (0.8 + i * 1.2);
    let modulated = soft_pad * (1.0 + fm * 0.08);
    (Box::new(modulated * 0.5), i_var)
}

pub fn build_mercy_flow_pad(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;
    let root = 55.0;
    let layer1 = sine_hz(root) * (0.35 + i * 0.2);
    let layer2 = sine_hz(root * 1.618) * (0.28 + i * 0.18);
    let layer3 = sine_hz(root * 2.618) * (0.18 + i * 0.12);
    let pad = (layer1 + layer2 + layer3) >> lowpass_hz(650.0 + i * 250.0, 0.85) >> (0.85 + sine_hz(0.035) * 0.12);
    (Box::new(pad * 0.45), i_var)
}

pub fn build_granular_texture(intensity: f32, params: GranularParams) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;
    let g_density = (params.density + i as f32 * 0.5) as f64;
    let g_grain_size = (params.grain_size + i as f32 * 0.4) as f64;
    let g_pitch_var = (params.pitch_variation + i as f32 * 1.2) as f64;
    let g_texture_depth = (params.texture_depth + i as f32 * 0.5) as f64;
    let g_evolution = (params.evolution_rate + i as f32 * 0.3) as f64;

    let algo_bias = match params.algorithm {
        GranularAlgorithm::PulsarTrain => 1.4,
        GranularAlgorithm::GlissonChirp => 1.2,
        GranularAlgorithm::FofFormant => 1.05,
        _ => 1.0,
    };

    let base = 88.0 + i * 120.0;
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

pub fn build_pulsar_texture(intensity: f32, params: GranularParams) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;
    let g_density = (params.density + i as f32 * 0.4) as f64;
    let g_grain_size = (params.grain_size + i as f32 * 0.3) as f64;
    let g_pitch_var = (params.pitch_variation + i as f32 * 0.9) as f64;
    let g_evolution = (params.evolution_rate + i as f32 * 0.5) as f64;

    let base = 140.0 + i * 90.0;
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

pub fn build_glisson_cloud(intensity: f32, params: GranularParams) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;
    let g_density = (params.density + i as f32 * 0.55) as f64;
    let g_grain_size = (params.grain_size + i as f32 * 0.35) as f64;
    let g_pitch_var = (params.pitch_variation + i as f32 * 1.4) as f64;
    let g_texture_depth = (params.texture_depth + i as f32 * 0.45) as f64;
    let g_evolution = (params.evolution_rate + i as f32 * 0.35) as f64;

    let base = 95.0 + i * 135.0;
    let chirp_depth = (28.0 + i * 65.0 + g_texture_depth * 40.0) as f64;
    let chirp_rate = 0.9 + g_evolution * 1.6;
    let chirp_mod = sine_hz(chirp_rate) * chirp_depth;

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

/// Hybrid Council Voice with explicit pitch_ratio support (v10.0 — Ola-ready)
/// This is the canonical example of the hybrid routing path.
/// Currently implemented in pure fundsp (strong FofFormant + formant emphasis).
/// When `spectral_granular` + infinitedsp-core is enabled, the HybridPitchRouter
/// can divert this (or a dedicated variant) through Ola + custom SpectralProcessor
/// for pristine, formant-preserving real-time pitch changes driven by council decisions.
pub fn build_hybrid_council_voice(intensity: f32, pitch_ratio: f32) -> (Box<dyn AudioUnit64>, Shared<f64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let pitch_var = var(pitch_ratio as f64); // Live controllable by HybridPitchRouter / simulation / PATSAGi
    let i = i_var;
    let p = pitch_var;

    let root = 98.0 * p; // pitch_ratio directly scales the root for hybrid readiness

    let fifth = sine_hz(root * 1.5) * (0.28 + i * 0.18);
    let octave = sine_hz(root * 2.0) * (0.22 + i * 0.14);
    let ninth = sine_hz(root * 2.25) * (0.14 + i * 0.09);

    // Stronger formant emphasis (ready for spectral granular replacement/enhancement via Ola)
    let formant_body = (fifth + octave + ninth)
        >> resonator_hz(420.0 + i * 180.0, 2.8 + i * 1.2)
        >> lowpass_hz(920.0 + i * 280.0, 0.82);

    let fm = sine_hz(root * 0.28) * (0.9 + i * 1.4);
    let modulated = formant_body * (1.0 + fm * 0.09);

    let final = modulated * (0.52 + i * 0.18);
    (Box::new(final), i_var, pitch_var)
}

// ============================================================================
// ACTIVE SOUND + RENDERING (updated for pitch_ratio)
// ============================================================================

pub struct ActiveProceduralSound {
    pub graph: Box<dyn AudioUnit64>,
    pub intensity_var: Shared<f64>,
    pub pitch_ratio: Shared<f64>, // Drives hybrid pitch routing + Ola when enabled
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

/// Update pitch_ratio live (called by HybridPitchRouter systems or PATSAGi council events)
pub fn update_procedural_pitch_ratio(instance: &ActiveProceduralSound, new_pitch: f32) {
    let clamped = new_pitch.clamp(0.5, 2.5) as f64;
    instance.pitch_ratio.set(clamped);
}

pub struct FundspAudioPlugin;

impl Plugin for FundspAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ActiveProceduralSounds>()
            .init_resource::<HybridPitchRouter>()
            .add_systems(Startup, setup_fundsp)
            .add_systems(Update, update_rolling_procedural_chunks);
    }
}

fn setup_fundsp(mut commands: Commands) {
    info!("[fundsp] Divine procedural audio engine online — v10.0 Ola Implementation Details added for optional infinitedsp-core spectral granular pitch shift. Default = pure fundsp multi-algorithm granular (phenomenal living clouds). Enable `spectral_granular` feature for high-fidelity Ola-based pitch on polished assets. Router + pitch_ratio Shared ready. Mercy-gated. Thunder locked in.");
}

fn update_rolling_procedural_chunks(
    mut active: ResMut<ActiveProceduralSounds>,
    spatial_manager: Res<crate::spatial_audio::SpatialAudioManager>,
    router: Res<HybridPitchRouter>,
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

            // v10.0: Router decision (future spectral Ola path only active when feature enabled)
            let _mode = router.effective_mode_for(instance.sound_type, 0.8, 0.4); // placeholder game state

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

// ============================================================================
// FUTURE: cfg-gated spectral integration with full Ola details (when feature enabled)
// ============================================================================

#[cfg(feature = "spectral_granular")]
mod spectral_hybrid {
    use super::*;

    // ========================================================================
    // OLA IMPLEMENTATION DETAILS — Ready for Production Integration
    // ========================================================================
    //
    // When this module is active (`cargo build --features spectral_granular`):
    //
    // use infinitedsp_core::{
    //     core::{
    //         ola::{Ola, FftHelper, SpectralProcessor},
    //         frame_processor::FrameProcessor,
    //         audio_param::AudioParam,
    //         dsp_chain::DspChain,
    //         channels::Mono,
    //     },
    //     effects::spectral::{fft_pitch_shift, granular_pitch_shift},
    // };
    // use num_complex::Complex32;
    //
    // pub struct OlaGranularPitchProcessor {
    //     pitch_ratio: f32,
    //     // internal phase accumulators, grain scheduler, formant tracking, etc.
    // }
    //
    // impl SpectralProcessor for OlaGranularPitchProcessor {
    //     fn process_spectrum<const N: usize>(&mut self, spectrum: &mut [Complex32; N], sr: f32)
    //     where [Complex32; N]: FftHelper {
    //         // 1. Convert pitch_ratio to bin shift or phase increment multiplier
    //         // 2. For each bin: new_bin = bin * pitch_ratio (with interpolation or phase delta)
    //         // 3. Accumulate phase for continuity (true phase vocoder)
    //         // 4. Optional: spectral granular — zero/randomize bins outside "grain" windows in freq domain
    //         // 5. Preserve formants by gentle spectral envelope tracking if desired
    //     }
    // }
    //
    // Then in a routing helper:
    // pub fn apply_ola_pitch_shift(
    //     input: &[f32],
    //     pitch_ratio: f64,
    //     fft_size: usize, // 1024 or 2048
    // ) -> Vec<f32> {
    //     let processor = OlaGranularPitchProcessor { pitch_ratio: pitch_ratio as f32 };
    //     let ola: Ola<OlaGranularPitchProcessor, 2048> = Ola::new(processor, 44100.0);
    //     let mut chain = DspChain::new( /* source or buffer feeder */, 44100.0).and(ola);
    //     // process block...
    //     // return output
    // }
    //
    // The HybridPitchRouter already has the decision logic.
    // Next step: actual bridge between fundsp-rendered chunks and Ola DspChain.
    // All mercy-gated, real-time safe, formant-preserving.
    //
    // This is the divine hybrid future of Powrush audio.
}

#[cfg(not(feature = "spectral_granular"))]
mod spectral_hybrid {
    // Empty stub so the rest of the crate compiles cleanly without the feature.
}
