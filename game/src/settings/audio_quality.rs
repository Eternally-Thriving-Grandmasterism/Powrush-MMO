/*!
 * Audio Quality & Performance Settings
 *
 * Controls for convolution reverb quality, LOD, and overall audio performance.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct AudioQualitySettings {
    /// Overall audio quality level (0.0 = lowest, 1.0 = highest)
    pub master_quality: f32,
    /// Convolution reverb quality (0.0 = disabled, 1.0 = full quality)
    pub convolution_quality: f32,
    /// Maximum distance at which full-quality convolution is applied
    pub convolution_max_distance: f32,
    /// Whether to enable distance-based LOD for convolution
    pub enable_distance_lod: bool,
}

impl Default for AudioQualitySettings {
    fn default() -> Self {
        Self {
            master_quality: 0.85,
            convolution_quality: 0.7,      // Start at medium-high by default
            convolution_max_distance: 180.0,
            enable_distance_lod: true,
        }
    }
}

impl AudioQualitySettings {
    /// Returns the effective convolution mix multiplier after quality + distance LOD
    pub fn get_convolution_mix_multiplier(&self, distance_to_focus: f32) -> f32 {
        if self.convolution_quality <= 0.01 {
            return 0.0;
        }

        let quality = self.convolution_quality * self.master_quality;

        if !self.enable_distance_lod {
            return quality;
        }

        // Simple linear distance falloff
        let distance_factor = (1.0 - (distance_to_focus / self.convolution_max_distance).clamp(0.0, 1.0)).powf(0.7);
        (quality * distance_factor).clamp(0.0, 1.0)
    }
}
