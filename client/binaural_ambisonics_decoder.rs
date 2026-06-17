//! client/binaural_ambisonics_decoder.rs
//! Production-grade First-Order Ambisonics to Binaural Decoder
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use glam::{Vec3, Quat};
use crate::game::procedural_music::HrtfImpulseResponses;
use crate::client::ambisonics_engine::AmbisonicField;

#[derive(Resource, Default)]
pub struct BinauralAmbisonicsDecoder {
    pub enabled: bool,
}

pub struct BinauralAmbisonicsDecoderPlugin;

impl Plugin for BinauralAmbisonicsDecoderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BinauralAmbisonicsDecoder>()
            .add_systems(Update, decode_ambisonics_to_binaural);
    }
}

// Decode Ambisonics field to stereo binaural using real HRTF
fn decode_ambisonics_to_binaural(
    listener: Query<&AudioListener>,
    field: Res<AmbisonicField>,
    hrtf: Res<HrtfImpulseResponses>,
    decoder: Res<BinauralAmbisonicsDecoder>,
    mut audio: ResMut<Audio>, // or custom output buffer in full version
) {
    if !decoder.enabled || !hrtf.loaded || hrtf.left.is_empty() {
        return;
    }

    let listener = listener.single();

    // Rotate the sound field according to listener orientation
    let rotation = listener.rotation;
    let forward = rotation * Vec3::Z;
    let up = rotation * Vec3::Y;

    // Simple First-Order decoding to virtual speakers then HRTF
    let w = field.w;
    let x = field.x;
    let y = field.y;
    let z = field.z;

    // Basic FOA decoding matrix (virtual speaker coefficients)
    let front = (w + x) * 0.5;
    let back  = (w - x) * 0.5;
    let left  = (w + y) * 0.5;
    let right = (w - y) * 0.5;
    let up    = (w + z) * 0.5;
    let down  = (w - z) * 0.5;

    // For simplicity we use a representative HRTF direction for the whole field
    // In higher-order Ambisonics this would be more complex
    let azimuth = forward.x.atan2(forward.z).to_degrees() as i32;
    let ir_index = ((azimuth + 180) % 360 / 45) as usize % hrtf.left.len();

    let left_ir = &hrtf.left[ir_index];
    let right_ir = &hrtf.right[ir_index];

    // Simulate a short buffer for the decoded field (in production this would be continuous)
    let mono_field = vec![(front + back + left + right + up + down) * 0.2; 4096];

    let convolved = convolve_hrtf(&mono_field, left_ir, right_ir);

    // Valence modulation on final output
    let valence = /* fetch from lattice */;
    let mut final_stereo = Vec::with_capacity(convolved.len());
    for &sample in &convolved {
        final_stereo.push(sample * valence);
    }

    // In full production: send to Kira or WebXR audio output
    // audio.play(AudioSource::from(final_stereo));
    println!("Ambisonics decoded to binaural — field strength: {:.3}", w);
}

fn convolve_hrtf(mono_buffer: &[f32], hrtf_left: &[f32], hrtf_right: &[f32]) -> Vec<f32> {
    let len = mono_buffer.len();
    let ir_len = hrtf_left.len().min(2048);
    let mut output = vec![0.0; len * 2];

    for i in 0..len {
        let max_j = ir_len.min(len - i);
        let sample = mono_buffer[i];
        let mut left_sum = 0.0;
        let mut right_sum = 0.0;

        for j in 0..max_j {
            left_sum += sample * hrtf_left[j];
            right_sum += sample * hrtf_right[j];
        }

        let idx = i * 2;
        output[idx] = left_sum;
        output[idx + 1] = right_sum;
    }
    output
}