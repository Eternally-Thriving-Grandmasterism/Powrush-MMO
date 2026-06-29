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

    // Button base glow (mercy cyan) + lattice lines
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = (x as f32 - cx) / r;
        let dy = (y as f32 - cy) / r;
        let d = (dx*dx + dy*dy).sqrt();

        // Outer glow fade
        let glow = if d < 1.2 { (1.0 - d) * 255.0 } else { 0.0 };

        // Lattice lines (vibrational truth pattern)
        let angle = dy.atan2(dx);
        let line = (angle / (PI / 8.0)).fract();
        let lattice_glow = if line < 0.12 || line > 0.88 { glow * 1.8 } else { 0.0 };

        let r = (lattice_glow * 0.1) as u8;
        let g = (glow * 0.9 + lattice_glow * 0.3) as u8;
        let b = glow as u8;

        *pixel = Rgb([r, g, b]);
    }

    // "Mercy" label using procedural pixel font
    draw_text_centered(&mut img, cx as u32, cy as u32, [255, 255, 255], label);

    img
}

/// Draw centered text using simple 8x8 pixel font
fn draw_text_centered(img: &mut RgbImage, cx: u32, cy: u32, color: [u8; 3], text: &str) {
    let mut x = cx.saturating_sub((text.len() as u32 * 8) / 2);
    for ch in text.chars() {
        draw_char(img, x, cy.saturating_sub(4), color, ch);
        x += 8;
    }
}

/// Simple 8x8 bitmap font data (demo quality)
fn simple_font(ch: char) -> Option<[[bool; 8]; 8]> {
    match ch {
        'M' => Some([
            [true, false, false, false, false, false, false, true],
            [true, true, false, false, false, false, true, true],
            [true, false, true, false, false, true, false, true],
            [true, false, false, true, true, false, false, true],
            [true, false, false, true, true, false, false, true],
            [true, false, false, false, false, false, false, true],
            [true, false, false, false, false, false, false, true],
            [true, false, false, false, false, false, false, true],
        ]),
        _ => None,
    }
}

/// Draw a single character at position
fn draw_char(img: &mut RgbImage, x: u32, y: u32, color: [u8; 3], ch: char) {
    if let Some(glyph) = simple_font(ch) {
        for dy in 0..8 {
            for dx in 0..8 {
                if glyph[dy][dx] {
                    if let Some(pixel) = img.get_pixel_mut_checked(x + dx as u32, y + dy as u32) {
                        *pixel = Rgb(color);
                    }
                }
            }
        }
    }
}

// Usage example (in renderer or UI layer):
// let button = generate_lattice_button(256, 64, "Mercy");
// Then upload to texture and draw quad.
