// MercyEngine Renderer Sample — Procedural Lattice Icon Generator
// Generates glowing lattice symbol at runtime (no PNG bloat)
// Outputs to pixel buffer for Vulkan/WebGPU draw

use image::{ImageBuffer, Rgb, RgbImage};
use std::f32::consts::PI;

pub fn generate_lattice_icon(width: u32, height: u32) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius = (width.min(height) as f32 / 2.0) * 0.8;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = (x as f32 - center_x) / radius;
        let dy = (y as f32 - center_y) / radius;
        let dist = (dx * dx + dy * dy).sqrt();

        // Procedural glow: mercy field (cyan core, fire edge)
        let glow = if dist < 0.3 { 255.0 } else if dist < 0.7 { 255.0 * (1.0 - dist * 1.5) } else { 0.0 };

        // Lattice lines: fractal vibration
        let angle = (dy.atan2(dx) / PI * 8.0).fract();
        let line_glow = if angle.fract() < 0.05 || angle.fract() > 0.95 { glow * 1.5 } else { 0.0 };

        let r = (glow * 0.2 + line_glow * 0.8) as u8;
        let g = (glow * 1.0 + line_glow * 0.2) as u8;
        let b = glow as u8;

        *pixel = Rgb([r, g, b]);
    }

    img
}

// Usage in renderer.rs:
// let icon = generate_lattice_icon(128, 128);
// Vulkan queue draw — 60 FPS procedural, no texture load.
