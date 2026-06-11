/*!
 * fundsp Procedural Audio Prototype
 *
 * Further developed granular-style layer with more voices
 * and cross-modulation between granular texture and tonal body.
 */

use bevy::prelude::*;
use fundsp::hacker::*;

/// Builds a rich Epiphany resonance with an enhanced granular-style layer.
pub fn build_epiphany_resonance(intensity: f32) -> (Box<dyn AudioUnit64>, Shared<f64>) {
    let intensity_var = var(intensity as f64);
    let i = intensity_var;

    // === Main Tonal Body ===
    let base_freq = 62.0 + i * 155.0;
    let vibrato = sine_hz(0.55) * (1.1 + i * 1.4);
    let tone_a = sine_hz(base_freq + vibrato);
    let tone_b = sine_hz(base_freq * 1.009);
    let main_body = (tone_a + tone_b) * (0.20 + i * 0.34);

    let harmonic = sine_hz(base_freq * 2.01) * (0.10 + i * 0.24);

    // === Enhanced Granular-Style Texture Layer (5 voices) ===
    // More voices + cross-modulation for a denser, more alive cloud

    // Voice 1 - Low, slow movement
    let g1_freq = base_freq * 0.48 + sine_hz(0.35) * (2.5 + i * 3.5);
    let g1 = sine_hz(g1_freq) * (0.08 + i * 0.17);
    let g1_filt = g1 >> lowpass_hz(380.0 + i * 420.0, 1.7);

    // Voice 2 - Low-mid
    let g2_freq = base_freq * 0.95 + sine_hz(0.65) * (3.0 + i * 4.0);
    let g2 = sine_hz(g2_freq) * (0.07 + i * 0.16);
    let g2_filt = g2 >> lowpass_hz(520.0 + i * 480.0, 1.6);

    // Voice 3 - Mid (cross-modulates main body filter later)
    let g3_freq = base_freq * 1.6 + sine_hz(1.1) * (3.8 + i * 5.0);
    let g3 = sine_hz(g3_freq) * (0.065 + i * 0.15);
    let g3_filt = g3 >> lowpass_hz(720.0 + i * 580.0, 1.5);

    // Voice 4 - Upper-mid
    let g4_freq = base_freq * 2.7 + sine_hz(1.8) * (4.2 + i * 5.5);
    let g4 = sine_hz(g4_freq) * (0.06 + i * 0.14);
    let g4_filt = g4 >> lowpass_hz(880.0 + i * 620.0, 1.45);

    // Voice 5 - High, fast movement
    let g5_freq = base_freq * 4.2 + sine_hz(3.2) * (5.0 + i * 6.5);
    let g5 = sine_hz(g5_freq) * (0.055 + i * 0.13);
    let g5_filt = g5 >> lowpass_hz(1050.0 + i * 700.0, 1.4);

    let granular_layer = (g1_filt + g2_filt + g3_filt + g4_filt + g5_filt)
        * (0.62 + i * 0.38);

    // === Cross-Modulation: Granular layer influences main tonal filter ===
    // Use a simplified version of the granular sum to modulate the main body
    let cross_mod = (g3_filt + g4_filt) * 0.4;
    let tonal_with_cross = main_body
        >> lowpass_hz(1250.0 + i * 480.0 + cross_mod * 180.0, 1.3);

    // === Combine everything ===
    let combined = tonal_with_cross + harmonic + granular_layer;

    // === Multi-layer Amplitude Modulation ===
    let breath_slow = sine_hz(0.065) * 0.23 + 0.77;
    let breath_mid = sine_hz(0.14) * 0.15 + 0.85;
    let modulated = combined * (0.78 + breath_slow * breath_mid * i * 0.29);

    // Final shaping
    let final = modulated >> lowpass_hz(1380.0 + i * 520.0, 1.0);

    (Box::new(final * 0.69), intensity_var)
}

/// Represents an active rolling procedural Epiphany resonance.
pub struct ActiveEpiphanyResonance {
    pub graph: Box<dyn AudioUnit64>,
    pub intensity_var: Shared<f64>,
    pub remaining_duration: f32,
    pub total_duration: f32,
    pub chunk_duration: f32,
    pub position: Vec3,
}

#[derive(Resource, Default)]
pub struct ActiveProceduralEpiphanies {
    pub instances: Vec<ActiveEpiphanyResonance>,
}

/// Renders the next chunk from an active instance.
pub fn render_next_chunk(instance: &mut ActiveEpiphanyResonance) -> Vec<f32> {
    let sample_rate = 44100.0;
    let num_samples = (instance.chunk_duration * sample_rate) as usize;
    let mut buffer = vec![0.0; num_samples];
    instance.graph.render(sample_rate, &mut buffer);
    buffer
}

/// Update intensity of a running Epiphany resonance.
pub fn update_epiphany_intensity(instance: &ActiveEpiphanyResonance, new_intensity: f32) {
    let clamped = new_intensity.clamp(0.0, 1.0) as f64;
    instance.intensity_var.set(clamped);
}

pub struct FundspAudioPlugin;

impl Plugin for FundspAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ActiveProceduralEpiphanies>()
            .add_systems(Startup, setup_fundsp)
            .add_systems(Update, update_rolling_chunks);
    }
}

fn setup_fundsp(mut commands: Commands) {
    info!("[fundsp] Enhanced granular layer with cross-modulation");
}

/// System that renders chunks and evolves intensity automatically.
fn update_rolling_chunks(
    mut active: ResMut<ActiveProceduralEpiphanies>,
    spatial_manager: Res<crate::spatial_audio::SpatialAudioManager>,
) {
    let mut i = 0;
    while i < active.instances.len() {
        let instance = &mut active.instances[i];

        if instance.remaining_duration > 0.0 {
            let progress = 1.0 - (instance.remaining_duration / instance.total_duration);

            let evolved = if progress < 0.55 {
                0.65 + (progress / 0.55) * 0.55
            } else {
                1.2 - ((progress - 0.55) / 0.45) * 0.5
            };

            let base_intensity = instance.intensity_var.get() as f32;
            let final_intensity = (base_intensity * evolved).clamp(0.35, 1.2);

            instance.intensity_var.set(final_intensity as f64);

            let samples = render_next_chunk(instance);

            if !samples.is_empty() {
                let volume = (0.37 + final_intensity * 0.33).clamp(0.30, 0.74);

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
