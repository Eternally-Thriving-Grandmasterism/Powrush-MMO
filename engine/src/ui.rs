// MercyEngine — Procedural UI Sample: Lattice Button Generator
// Renders glowing button with mercy lattice at runtime (no sprites/PNG)
// Outputs to pixel buffer for Vulkan/WebGPU immediate draw

use image::{ImageBuffer, Rgb, RgbImage};
use std::collections::HashMap;
use std::f32::consts::PI;

pub fn generate_lattice_button(width: u32, height: u32, label: &str) -> RgbImage {
    let mut img = ImageBuffer::new(width, height);
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let r = (width.min(height) as f32 / 2.0) * 0.45;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = (x as f32 - cx) / r;
        let dy = (y as f32 - cy) / r;
        let d = (dx*dx + dy*dy).sqrt();

        let glow = if d < 1.2 { (1.0 - d) * 255.0 } else { 0.0 };

        let angle = dy.atan2(dx);
        let line = (angle / (PI / 8.0)).fract();
        let lattice_glow = if line < 0.12 || line > 0.88 { glow * 1.8 } else { 0.0 };

        let r = (lattice_glow * 0.1) as u8;
        let g = (glow * 0.9 + lattice_glow * 0.3) as u8;
        let b = glow as u8;

        *pixel = Rgb([r, g, b]);
    }

    let font = SimpleBitmapFont::new();
    draw_text_centered(&mut img, cx as u32, cy as u32, [255, 255, 255], label, &font);

    img
}

/// Trait for pixel-based fonts with batching and pre-rendering support
pub trait PixelFont {
    fn draw_char(&self, img: &mut RgbImage, x: u32, y: u32, color: [u8; 3], ch: char);

    fn draw_text(&self, img: &mut RgbImage, x: u32, y: u32, color: [u8; 3], text: &str) {
        if text.is_empty() { return; }
        let char_width = 8u32;
        let text_width = text.len() as u32 * char_width;
        let text_height = 8u32;

        let mut text_atlas: RgbImage = ImageBuffer::new(text_width, text_height);
        let mut atlas_x = 0u32;
        for ch in text.chars() {
            self.draw_char(&mut text_atlas, atlas_x, 0, color, ch);
            atlas_x += char_width;
        }
        let _ = img.copy_from(&text_atlas, x, y);
    }

    fn pre_render_text(&self, text: &str, color: [u8; 3]) -> RgbImage {
        if text.is_empty() {
            return ImageBuffer::new(0, 0);
        }
        let char_width = 8u32;
        let text_width = text.len() as u32 * char_width;
        let text_height = 8u32;

        let mut atlas: RgbImage = ImageBuffer::new(text_width, text_height);
        let mut atlas_x = 0u32;
        for ch in text.chars() {
            self.draw_char(&mut atlas, atlas_x, 0, color, ch);
            atlas_x += char_width;
        }
        atlas
    }
}

/// Dynamic 8x8 bitmap font that supports runtime glyph loading
pub struct SimpleBitmapFont {
    glyphs: HashMap<char, [[bool; 8]; 8]>,
}

impl SimpleBitmapFont {
    pub fn new() -> Self {
        let mut font = Self { glyphs: HashMap::new() };
        font.load_glyph('M', [
            [true, false, false, false, false, false, false, true],
            [true, true, false, false, false, false, true, true],
            [true, false, true, false, false, true, false, true],
            [true, false, false, true, true, false, false, true],
            [true, false, false, true, true, false, false, true],
            [true, false, false, false, false, false, false, true],
            [true, false, false, false, false, false, false, true],
            [true, false, false, false, false, false, false, true],
        ]);
        font
    }

    pub fn load_glyph(&mut self, ch: char, glyph: [[bool; 8]; 8]) {
        self.glyphs.insert(ch, glyph);
    }
}

impl PixelFont for SimpleBitmapFont {
    #[inline]
    fn draw_char(&self, img: &mut RgbImage, x: u32, y: u32, color: [u8; 3], ch: char) {
        if let Some(glyph) = self.glyphs.get(&ch) {
            let max_x = x + 7;
            let max_y = y + 7;
            if max_x >= img.width() || max_y >= img.height() {
                for dy in 0..8 {
                    for dx in 0..8 {
                        if glyph[dy][dx] {
                            if let Some(pixel) = img.get_pixel_mut_checked(x + dx as u32, y + dy as u32) {
                                *pixel = Rgb(color);
                            }
                        }
                    }
                }
                return;
            }
            for dy in 0..8 {
                for dx in 0..8 {
                    if glyph[dy][dx] {
                        let pixel = img.get_pixel_mut(x + dx as u32, y + dy as u32);
                        *pixel = Rgb(color);
                    }
                }
            }
        }
    }

    fn draw_text(&self, img: &mut RgbImage, x: u32, y: u32, color: [u8; 3], text: &str) {
        if text.is_empty() { return; }
        let char_width = 8u32;
        let text_width = text.len() as u32 * char_width;
        let text_height = 8u32;

        let mut text_atlas: RgbImage = ImageBuffer::new(text_width, text_height);
        let mut atlas_x = 0u32;
        for ch in text.chars() {
            <Self as PixelFont>::draw_char(self, &mut text_atlas, atlas_x, 0, color, ch);
            atlas_x += char_width;
        }
        let _ = img.copy_from(&text_atlas, x, y);
    }

    fn pre_render_text(&self, text: &str, color: [u8; 3]) -> RgbImage {
        if text.is_empty() {
            return ImageBuffer::new(0, 0);
        }
        let char_width = 8u32;
        let text_width = text.len() as u32 * char_width;
        let text_height = 8u32;

        let mut atlas: RgbImage = ImageBuffer::new(text_width, text_height);
        let mut atlas_x = 0u32;
        for ch in text.chars() {
            <Self as PixelFont>::draw_char(self, &mut atlas, atlas_x, 0, color, ch);
            atlas_x += char_width;
        }
        atlas
    }
}

/// Draw a pre-rendered text atlas (very fast - single blit)
pub fn draw_pre_rendered_text(img: &mut RgbImage, x: u32, y: u32, atlas: &RgbImage) {
    let _ = img.copy_from(atlas, x, y);
}

/// Draw centered text using a pre-rendered atlas when possible
pub fn draw_text_centered(img: &mut RgbImage, cx: u32, cy: u32, color: [u8; 3], text: &str, font: &dyn PixelFont) {
    if text.is_empty() { return; }
    let atlas = font.pre_render_text(text, color);
    let start_x = cx.saturating_sub(atlas.width() / 2);
    let start_y = cy.saturating_sub(4);
    draw_pre_rendered_text(img, start_x, start_y, &atlas);
}

/// Simple cache for pre-rendered text atlases.
/// Useful for UI labels that don't change often.
pub struct TextAtlasCache {
    cache: HashMap<String, RgbImage>,
}

impl TextAtlasCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get a cached atlas or render and cache it.
    pub fn get_or_render(
        &mut self,
        font: &dyn PixelFont,
        text: &str,
        color: [u8; 3],
    ) -> &RgbImage {
        self.cache.entry(text.to_string()).or_insert_with(|| {
            font.pre_render_text(text, color)
        })
    }

    /// Draw using a cached atlas (creates it if missing).
    pub fn draw(
        &mut self,
        img: &mut RgbImage,
        x: u32,
        y: u32,
        text: &str,
        color: [u8; 3],
        font: &dyn PixelFont,
    ) {
        let atlas = self.get_or_render(font, text, color);
        draw_pre_rendered_text(img, x, y, atlas);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

// Recommended high-performance pattern:
// let mut cache = TextAtlasCache::new();
// let font = SimpleBitmapFont::new();
// cache.draw(&mut target, x, y, "Mercy", [255, 255, 255], &font);
