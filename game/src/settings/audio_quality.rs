/*!
 * Audio Quality & Performance Settings (with IR Metrics)
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;

#[derive(Resource, Clone, Copy, Default)]
pub struct AudioQualitySettings {
    pub master_quality: f32,
    pub convolution_quality: f32,
    pub use_early_only_ir: bool,
    pub early_reflection_mix: f32,
    pub late_tail_quality: f32,
    pub late_tail_mix: f32,
    pub convolution_max_distance: f32,
    pub enable_distance_lod: bool,
    pub crossfade_duration: f32,
    pub early_reflection_target_duration: f32,

    /// IR truncation logging level: 0 = off, 1 = info only, 2 = debug
    pub ir_metrics_level: u8,
}

impl Default for AudioQualitySettings {
    fn default() -> Self {
        Self {
            master_quality: 0.85,
            convolution_quality: 0.65,
            use_early_only_ir: true,
            early_reflection_mix: 0.85,
            late_tail_quality: 0.75,
            late_tail_mix: 0.9,
            convolution_max_distance: 180.0,
            enable_distance_lod: true,
            crossfade_duration: 0.28,
            early_reflection_target_duration: 0.12,
            ir_metrics_level: 1, // info level by default
        }
    }
}

impl AudioQualitySettings {
    pub fn get_convolution_mix_multiplier(&self, distance_to_focus: f32) -> f32 {
        if self.convolution_quality <= 0.01 { return 0.0; }
        let quality = self.convolution_quality * self.master_quality;
        if !self.enable_distance_lod { return quality; }
        let distance_factor = (1.0 - (distance_to_focus / self.convolution_max_distance).clamp(0.0, 1.0)).powf(0.7);
        (quality * distance_factor).clamp(0.0, 1.0)
    }

    pub fn get_early_mix(&self) -> f32 {
        (self.early_reflection_mix * self.master_quality).clamp(0.0, 1.0)
    }

    pub fn get_late_mix(&self) -> f32 {
        (self.late_tail_mix * self.master_quality * self.late_tail_quality).clamp(0.0, 1.0)
    }

    pub fn should_log_ir_info(&self) -> bool { self.ir_metrics_level >= 1 }
    pub fn should_log_ir_debug(&self) -> bool { self.ir_metrics_level >= 2 }
}
