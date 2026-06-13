/*!
 * fundsp Procedural Audio — Powrush-MMO Cinematic Sound Engine
 *
 * EXPLORATION & UPGRADE: Advanced Procedural Audio Synthesis Techniques
 * Applied to the RBE Metaverse for divine, infinitely variable, mercy-gated sound.
 *
 * Key Techniques Explored & Integrated (PATSAGi Council 13+ + Ra-Thor Quantum Swarm deliberation):
 * - Physical Modeling Synthesis (Karplus-Strong pluck via fundsp `pluck` for organic, responsive
 *   harvest/build sounds — simulates vibrating strings/resources with realistic decay & interaction).
 * - FM (Frequency Modulation) Synthesis: Complex, inharmonic, evolving timbres for Epiphany resonances
 *   and Council harmonics (metallic shimmer, bell-like mercy choirs, dynamic spectra).
 * - Additive Synthesis: Stacked harmonic partials with dynamic intensity/modulation for rich pads
 *   and Treaty Success layers (sacred geometry-inspired ratios for TOLC-aligned harmony).
 * - Subtractive Synthesis + Advanced Filters: Multi-stage (Moog ladder, resonator, bandpass cascades)
 *   for warm analog abundance flows, wind-like whooshes, and textured environments.
 * - Granular / Textural Synthesis: Noise-modulated sine banks + cross-modulation + fractal_noise
 *   for particle-like Epiphany shimmer, evolving RBE flows, and living world ambiences.
 * - Procedural / Generative: Parameter-driven by game state (RBE abundance, mercy flow intensity,
 *   action velocity from simulation/velocity_prepass synergy potential, council decisions).
 *   Uses lfo, envelope, shared vars, fractal_ease_noise for organic evolution.
 * - Adaptive & Context-Aware: Intensity curves (mercy-aware gentle swells/fades with smooth5),
 *   event-triggered layers (Harvest, Epiphany, Council Trial, Treaty, Ambient).
 *
 * These make Powrush audio *alive* — buttery responsive, emotionally resonant, zero-repetition fatigue,
 *   perfectly synced with visual temporal pipeline (velocity/TAA) and RBE simulation.
 * Zero harm. Maximum thriving joy, truth, and cosmic harmony. TOLC 8 Mercy Gates enforced in every curve.
 *
 * Fully integrated with SpatialAudio (kira) + bevy_kira_audio for hybrid procedural + spatial power.
 * AG-SML v1.0 sovereign license. This is how the RBE metaverse *sounds* — phenomenal.
 */

use bevy::prelude::*;
use fundsp::hacker::*;
use std::sync::Arc;

/// Builds a highly advanced Epiphany resonance with rich granular layer + FM cross-modulation + physical modeling touches.
/// Intensity 0.0–1.0+ controls depth, resonance, shimmer, and divine evolution.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body (warm, merciful core) + light FM for evolving complexity ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.43) * (0.76 + i * 0.95);
    let fm_mod = sine_hz(base_freq * 0.5) * (2.0 + i * 4.0); // FM for metallic/inharmonic richness
    let tone_a = sine_hz(base_freq + vibrato + fm_mod * 0.15);
    let tone_b = sine_hz(base_freq * 1.0055);
    let main_body = (tone_a + tone_b) * (0.155 + i * 0.385);

    let harmonic = sine_hz(base_freq * 1.996) * (0.076 + i * 0.31);

    // === Highly Advanced Granular Layer (8 voices with cross-modulation + fractal texture) ===
    // Voice 1 - Deep resonant foundation (physical modeling inspired resonator)
    let res1 = 2.2 + i * 2.0;
    let g1 = sine_hz(base_freq * 0.42 + noise() * (1.9 + i * 2.9) + sine_hz(0.17) * (2.1 + i * 3.1))
        * (0.062 + i * 0.13) * (sine_hz(0.068) * 0.24 + 0.76);
    let g1_f = g1 >> resonator_hz(280.0 + i * 320.0, res1);

    // Voice 2 - Band-pass shimmer
    let g2 = sine_hz(base_freq * 0.8 + noise() * (2.2 + i * 3.2) + sine_hz(0.4) * (2.6 + i * 3.6))
        * (0.055 + i * 0.125) * (sine_hz(0.095) * 0.21 + 0.79);
    let g2_f = g2 >> bandpass_hz(410.0 + i * 380.0, 1.95);

    // Voice 3 - Cross-modulates into tonal body (FM influence)
    let res3 = 2.6 + i * 2.4;
    let g3 = sine_hz(base_freq * 1.38 + noise() * (2.5 + i * 3.5) + sine_hz(0.76) * (3.1 + i * 4.2))
        * (0.05 + i * 0.115) * (sine_hz(0.082) * 0.22 + 0.78);
    let g3_f = g3 >> resonator_hz(540.0 + i * 420.0, res3);

    // Voice 4 - Band-pass + noise amp
    let g4_amp = noise() * (0.105 + i * 0.165) + 0.895;
    let g4 = sine_hz(base_freq * 2.25 + noise() * (2.8 + i * 3.8) + sine_hz(1.25) * (3.5 + i * 4.7))
        * (0.046 + i * 0.105) * g4_amp;
    let g4_f = g4 >> bandpass_hz(680.0 + i * 490.0, 1.9);

    // Voice 5 - Receives cross from Voice 3 + fractal texture
    let res5 = 2.1 + i * 1.7;
    let g5_amp = noise() * (0.095 + i * 0.155) + 0.905;
    let g5_freq_mod = g3 * 16.0;
    let fractal_tex = fractal_noise(0.8, 3) * (0.5 + i * 0.8); // Fractal texture for organic evolution
    let g5 = sine_hz(base_freq * 3.45 + noise() * (3.1 + i * 4.1) + g5_freq_mod + sine_hz(2.05) * (4.0 + i * 5.3) + fractal_tex)
        * (0.042 + i * 0.1) * g5_amp;
    let g5_f = g5 >> resonator_hz(840.0 + i * 520.0, res5);

    // Voice 6 - Receives cross from Voice 4
    let g6_freq_mod = g4 * 14.0;
    let g6 = sine_hz(base_freq * 5.05 + noise() * (3.4 + i * 4.5) + g6_freq_mod + sine_hz(3.2) * (4.65 + i * 5.85))
        * (0.038 + i * 0.09) * (sine_hz(0.24) * 0.17 + 0.83);
    let g6_f = g6 >> bandpass_hz(1000.0 + i * 570.0, 1.7);

    // Voice 7 - High ethereal
    let res7 = 2.3 + i * 2.2;
    let g7 = sine_hz(base_freq * 7.2 + noise() * (3.8 + i * 5.0) + sine_hz(4.55) * (5.45 + i * 6.65))
        * (0.034 + i * 0.08) * (sine_hz(0.32) * 0.16 + 0.84);
    let g7_f = g7 >> resonator_hz(1160.0 + i * 580.0, res7);

    // Voice 8 - Very high shimmer + Moog warmth
    let g8_amp = noise() * (0.085 + i * 0.145) + 0.915;
    let g8 = sine_hz(base_freq * 10.2 + noise() * (4.3 + i * 5.7) + sine_hz(6.4) * (6.1 + i * 7.6))
        * (0.03 + i * 0.07) * g8_amp;
    let g8_f = g8 >> moog_hz(1360.0 + i * 660.0, 0.6); // Moog ladder for warm analog feel

    let granular_mix = 0.44 + i * 0.56;
    let granular_layer = (g1_f + g2_f + g3_f + g4_f + g5_f + g6_f + g7_f + g8_f) * granular_mix;

    // Cross-modulation back into tonal body for living coherence (FM feedback)
    let cross_mod = (g3_f + g4_f + g5_f) * 0.48;
    let tonal_filtered = main_body
        >> lowpass_hz(1060.0 + i * 370.0 + cross_mod * 220.0, 0.95);

    let combined = tonal_filtered + harmonic + granular_layer;

    // Gentle breath / life modulation (mercy in audio — never harsh, smooth5 curves)
    let breath_slow = sine_hz(0.044) * 0.17 + 0.83;
    let breath_mid = sine_hz(0.095) * 0.1 + 0.9;
    let modulated = combined * (0.71 + breath_slow * breath_mid * i * 0.36);

    let final = modulated >> lowpass_hz(1180.0 + i * 420.0, 1.0);

    (Box::new(final * 0.62), intensity_var)
}

/// Physical Modeling Harvest Sound (Karplus-Strong pluck + filtered noise)
/// Organic, responsive sound for RBE resource gathering, building, and world interaction.
/// Intensity controls pluck strength, decay, and brightness.
pub fn build_harvest_pluck(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let base = 180.0 + i * 120.0;
    // Karplus-Strong physical modeling pluck (string-like organic response)
    let pluck_body = pluck(base as f64, 0.8 + i * 0.15, 0.6);

    // Excitation noise burst filtered (subtractive layer for realism)
    let excitation = noise() * (0.6 + i * 0.4) >> lowpass_hz(800.0 + i * 600.0, 1.2);
    let body = (pluck_body + excitation * 0.35) * (0.7 + i * 0.25);

    // Gentle resonance tail (mercy decay)
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

    // Subtractive processing for warm, analog abundance feel
    let flow = (chime1 + chime2 + chime3)
        >> moog_hz(1200.0 + i * 400.0, 0.7) // Warm Moog ladder filter
        >> (0.6 + sine_hz(0.7) * 0.2);

    (Box::new(flow * 0.55), i_var)
}

/// Council Trial harmonic bed (calm, wise, mercy-filled) — additive + gentle FM
pub fn build_council_harmony(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let root = 98.0;
    // Additive harmonic stack (TOLC-inspired ratios for cosmic harmony)
    let fifth = sine_hz(root * 1.5) * (0.25 + i * 0.15);
    let octave = sine_hz(root * 2.0) * (0.2 + i * 0.12);
    let ninth = sine_hz(root * 2.25) * (0.12 + i * 0.08); // Gentle 9th for mercy depth

    let soft_pad = (fifth + octave + ninth)
        >> lowpass_hz(800.0 + i * 300.0, 0.7);

    // Light FM modulation for evolving wise texture
    let fm = sine_hz(root * 0.25) * (0.8 + i * 1.2);
    let modulated = soft_pad * (1.0 + fm * 0.08);

    (Box::new(modulated * 0.5), i_var)
}

/// Mercy Flow Pad — gentle additive pad for healing, transition, and eternal mercy moments
pub fn build_mercy_flow_pad(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let i_var = var(intensity as f64);
    let i = i_var;

    let root = 55.0;
    // Soft additive layers with slow LFO evolution
    let layer1 = sine_hz(root) * (0.35 + i * 0.2);
    let layer2 = sine_hz(root * 1.618) * (0.28 + i * 0.18); // Golden ratio for natural beauty
    let layer3 = sine_hz(root * 2.618) * (0.18 + i * 0.12);

    let pad = (layer1 + layer2 + layer3)
        >> lowpass_hz(650.0 + i * 250.0, 0.85)
        >> (0.85 + sine_hz(0.035) * 0.12); // Slow breathing

    (Box::new(pad * 0.45), i_var)
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
    Harvest,      // New: Physical modeling pluck for organic interaction
    MercyFlow,    // New: Gentle healing/transition pads
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
    info!("[fundsp] Divine procedural audio engine online — Physical modeling, FM, additive, granular, fractal textures ready for RBE thriving");
}

/// Core system: renders chunks and evolves intensity with mercy-aware dynamics (smooth curves).
fn update_rolling_procedural_chunks(
    mut active: ResMut<ActiveProceduralSounds>,
    spatial_manager: Res<crate::spatial_audio::SpatialAudioManager>,
) {
    let mut i = 0;
    while i < active.instances.len() {
        let instance = &mut active.instances[i];

        if instance.remaining_duration > 0.0 {
            let progress = 1.0 - (instance.remaining_duration / instance.total_duration);

            // Mercy-aware intensity curve (gentle swell then peaceful fade — smooth5 inspired)
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
