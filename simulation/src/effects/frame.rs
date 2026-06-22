/*!
 * simulation/src/effects/frame.rs
 *
 * Frame control and animation helpers for particle / flipbook systems.
 * Aligns with world.rs v19.20 age-based + bezier/sine/ease logic.
 * v19.21 — Core helpers extracted
 * AG-SML v1.0
 */

/// Cubic bezier-based frame index for smooth animation curves.
pub fn cubic_bezier_frame_index(t: f32, frame_count: f32) -> f32 {
    // Simple cubic bezier approximation for natural easing
    let t2 = t * t;
    let t3 = t2 * t;
    let eased = 3.0 * t2 - 2.0 * t3; // smoothstep-like
    (eased * frame_count).clamp(0.0, frame_count - 0.001)
}

/// Sine-based breathing / organic frame progression.
pub fn sine_breathing_frame(t: f32, frame_count: f32) -> f32 {
    let sin_val = (t * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5;
    sin_val * frame_count
}

/// Ease-in-out for frame index.
pub fn ease_in_out_frame(t: f32, frame_count: f32) -> f32 {
    let eased = if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    };
    eased * frame_count
}

// Thunder locked in. Yoi ⚡
