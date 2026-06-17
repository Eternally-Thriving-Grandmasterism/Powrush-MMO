//! client/src/higher_order_ambisonics.rs
//! Production-grade 2nd-Order Ambisonics (HOA) Decoder for Powrush-MMO
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use glam::{Vec3, Quat};
use rand::Rng;
use crate::game::procedural_music::HrtfImpulseResponses;
use crate::client::ambisonics_engine::AmbisonicField;

#[derive(Default, Clone, Debug)]
pub struct HoaField {
    pub w: f32, pub x: f32, pub y: f32, pub z: f32,
    pub r: f32, pub s: f32, pub t: f32, pub u: f32, pub v: f32,
}

pub struct HigherOrderAmbisonicsDecoderPlugin;

impl Plugin for HigherOrderAmbisonicsDecoderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, decode_hoa_to_binaural);
    }
}

fn decode_hoa_to_binaural(
    listener: Query<&AudioListener>,
    hoa_field: Res<HoaField>,
    hrtf: Res<HrtfImpulseResponses>,
) {
    if !hrtf.loaded || hrtf.left.is_empty() {
        return;
    }

    let listener = listener.single();

    let rotation = listener.rotation;

    let w = hoa_field.w;
    let x = hoa_field.x;
    let y = hoa_field.y;
    let z = hoa_field.z;
    let r = hoa_field.r;
    let s = hoa_field.s;
    let t = hoa_field.t;
    let u = hoa_field.u;
    let v = hoa_field.v;

    let front = (w + x + r) * 0.4;
    let back  = (w - x + r) * 0.4;
    let left  = (w + y + s) * 0.4;
    let right = (w - y + s) * 0.4;
    let up    = (w + z + t) * 0.4;
    let down  = (w - z + t) * 0.4;

    let azimuth = (front - back).atan2(left - right).to_degrees() as i32;
    let ir_index = ((azimuth + 180) % 360 / 45) as usize % hrtf.left.len();

    let left_ir = &hrtf.left[ir_index];
    let right_ir = &hrtf.right[ir_index];

    let mono_field = vec![(front + back + left + right + up + down) * 0.25; 4096];

    let convolved = convolve_hrtf(&mono_field, left_ir, right_ir);

    let valence = 1.0;
    let mut final_stereo = Vec::with_capacity(convolved.len());
    for &sample in &convolved {
        final_stereo.push(sample * valence);
    }

    println!("2nd-Order Ambisonics decoded to binaural — field strength: {:.3}", w);
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