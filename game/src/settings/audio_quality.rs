/*!
 * Audio Quality & Performance Settings
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct AudioQualitySettings {
    pub master_quality: f32,
    pub convolution_quality: f32,
    pub convolution_max_distance: f32,
    pub enable_distance_lod: bool,
    /// Crossfade duration in seconds when switching impulse responses
    pub crossfade_duration: f32,
}

impl Default for AudioQualitySettings {
    fn default() -> Self {
        Self {
            master_quality: 0.85,
            convolution_quality: 0.7,
            convolution_max_distance: 180.0,
            enable_distance_lod: true,
            crossfade_duration: 0.28,
        }
    }
}

impl AudioQualitySettings {
    pub fn get_convolution_mix_multiplier(&self, distance_to_focus: f32) -> f32 {
        if self.convolution_quality <= 0.01 {
            return 0.0;
        }
        let quality = self.convolution_quality * self.master_quality;
        if !self.enable_distance_lod {
            return quality;
        }
        let distance_factor = (1.0 - (distance_to_focus / self.convolution_max_distance).clamp(0.0, 1.0)).powf(0.7);
        (quality * distance_factor).clamp(0.0, 1.0)
    }
}
