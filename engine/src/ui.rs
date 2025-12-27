// MercyEngine — Procedural UI Sample: Lattice Button Generator
// Renders glowing button with mercy lattice at runtime (no sprites/PNG)
// Outputs to pixel buffer for Vulkan/WebGPU immediate draw

use image::{ImageBuffer, Rgb, RgbImage};
use std::f32::consts::PI;

pub fn generate_lattice_button(width: u32, height: u32, label: &str) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let r = (width.min(height) as f32 / 2.0) * 0.45;

    // Button base glow (mercy cyan)
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = (x as f32 - cx) / r;
        let dy = (y as f32 - cy) / r;
        let d = (dx*dx + dy*dy).sqrt();

        // Outer glow fade
        let glow = if d < 1.2 { (1.0 - d) * 255.0 } else { 0.0 };

        // Lattice lines (vibrational truth)
        let angle = dy.atan2(dx);
        let line = (angle / (PI / 8.0)).fract();
        let lattice_glow = if line < 0.12 || line > 0.88 { glow * 1.8 } else { 0.0 };

        let r = (lattice_glow * 0.1) as u8;
        let g = (glow * 0.9 + lattice_glow * 0.3) as u8;
        let b = glow as u8;

        *pixel = Rgb([r, g, b]);
    }

    // "Mercy" label (procedural text via pixel font — simple for demo)
    // In full engine: SDF font shader
    draw_text_centered(&mut img, cx as u32, cy as u32, [255, 255, 255], label);

    img
}

fn draw_text_centered(img: &mut RgbImage, cx: u32, cy: u32, color: [u8; 3], text: &str) {
    // Simple pixel font renderer (expandable to full SDF)
    let font = simple_font(); // Placeholder bitmap font
    let mut x = cx.saturating_sub(text.len() as u32 * 8 / 2);
    for ch in text.chars() {
        draw_char(img, x, cy - 8, color, ch);
        x += 8;
    }
}

fn simple_font() -> Vec<Vec<u8>> { /* 8x8 bitmap font data */ vec![] }
fn draw_char(img: &mut RgbImage, x: u32, y: u32, color: [u8; 3], ch: char) { /* Render */ }

fn draw_char(img: &mut RgbImage, x: u32, y: u32, color: [u8; 3], ch: char) {
    // Demo: Simple 'M' glyph
    if ch == 'M' {
        for dy in 0..8 {
            for dx in 0..8 {
                if is_pixel_on(ch, dx, dy) {
                    if let Some(p) = img.get_pixel_mut_checked(x + dx, y + dy) {
                        *p = Rgb(color);
                    }
                }
            }
        }
    }
}

fn is_pixel_on(ch: char, dx: u32, dy: u32) -> bool {
    // M glyph bitmap
    match ch {
        'M' => matches!(dy * 8 + dx, 0..=7 | 56..=63 | 8..16 | 48..56 | 16..24 | 40..48 | 24..32 | 32..40),
        _ => false,
    }
}

// Usage in renderer.rs:
// let button = generate_lattice_button(256, 64, "Mercy");
// Vulkan cmd: copy buffer to texture → draw quad → UI layer.
