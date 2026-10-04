//! CARD VP-FONT-1 — UI font with real glyphs for the lived-hour text.
//!
//! Bevy's built-in default face is a FiraMono subset. It has no `·`, `—`,
//! `•`, `…` or `×`, so those rendered as tofu boxes on every plate. This
//! plugin swaps the default font handle for Fira Sans Regular (SIL OFL 1.1,
//! `assets/fonts/OFL.txt`, upstream google/fonts `ofl/firasans`). Panels that
//! already load `fonts/FiraSans-Regular.ttf` / `fonts/FiraSans-Bold.ttf`
//! now find those files on disk.
//!
//! The bytes are embedded, so the default face never depends on the asset
//! folder being present. No new graphics menu. No layout change.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

/// Fira Sans Regular, embedded. Same file as `assets/fonts/FiraSans-Regular.ttf`.
pub const UI_FONT_REGULAR: &[u8] = include_bytes!("../../assets/fonts/FiraSans-Regular.ttf");

/// Replaces Bevy's default font asset (the handle every `TextStyle::default()` uses).
pub struct UiFontPlugin;

impl Plugin for UiFontPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, install_default_ui_font);
    }
}

/// `true` when the embedded bytes parse as a font.
pub fn ui_font_parses() -> bool {
    Font::try_from_bytes(UI_FONT_REGULAR.to_vec()).is_ok()
}

fn install_default_ui_font(fonts: Option<ResMut<Assets<Font>>>) {
    let Some(mut fonts) = fonts else {
        return;
    };
    match Font::try_from_bytes(UI_FONT_REGULAR.to_vec()) {
        Ok(font) => fonts.insert(AssetId::<Font>::default(), font),
        Err(_) => warn!(target: "powrush::ui_font", "embedded UI font did not parse; keeping Bevy default"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn be16(b: &[u8], o: usize) -> u16 {
        u16::from_be_bytes([b[o], b[o + 1]])
    }
    fn be32(b: &[u8], o: usize) -> u32 {
        u32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
    }

    /// Minimal `cmap` format-4 lookup (BMP only). Test helper, not shipped logic.
    fn has_glyph(font: &[u8], ch: char) -> bool {
        let c = ch as u32;
        if c > 0xFFFF {
            return false;
        }
        let num_tables = be16(font, 4) as usize;
        let mut cmap = None;
        for i in 0..num_tables {
            let rec = 12 + i * 16;
            if &font[rec..rec + 4] == b"cmap" {
                cmap = Some(be32(font, rec + 8) as usize);
            }
        }
        let cmap = cmap.expect("cmap table");
        let n = be16(font, cmap + 2) as usize;
        for i in 0..n {
            let rec = cmap + 4 + i * 8;
            let (pid, eid) = (be16(font, rec), be16(font, rec + 2));
            let sub = cmap + be32(font, rec + 4) as usize;
            if !(pid == 3 && eid == 1 || pid == 0) || be16(font, sub) != 4 {
                continue;
            }
            let segx2 = be16(font, sub + 6) as usize;
            let ends = sub + 14;
            let starts = ends + segx2 + 2;
            let deltas = starts + segx2;
            let ranges = deltas + segx2;
            for s in 0..segx2 / 2 {
                let end = be16(font, ends + s * 2) as u32;
                if c > end {
                    continue;
                }
                let start = be16(font, starts + s * 2) as u32;
                if c < start {
                    return false;
                }
                let delta = be16(font, deltas + s * 2);
                let ro = be16(font, ranges + s * 2) as usize;
                let gid = if ro == 0 {
                    (c as u16).wrapping_add(delta)
                } else {
                    let at = ranges + s * 2 + ro + (c - start) as usize * 2;
                    let g = be16(font, at);
                    if g == 0 { 0 } else { g.wrapping_add(delta) }
                };
                return gid != 0;
            }
            return false;
        }
        false
    }

    #[test]
    fn embedded_ui_font_parses() {
        assert!(ui_font_parses());
    }

    /// Separators and marks the lived-hour plates use, including the swaps
    /// for glyphs Fira Sans lacks (° ◊ ‖ + replace ○ ◇/◎ ❚❚ ⊕).
    #[test]
    fn ui_font_covers_lived_hour_marks() {
        for ch in "·—–•…×→←−°◊‖+()ÉçñAz09".chars() {
            assert!(has_glyph(UI_FONT_REGULAR, ch), "missing glyph {ch:?}");
        }
        let bold = include_bytes!("../../assets/fonts/FiraSans-Bold.ttf");
        for ch in "·—•…×→".chars() {
            assert!(has_glyph(bold, ch), "bold missing glyph {ch:?}");
        }
    }

    /// The helper is honest: it reports glyphs Fira Sans really lacks.
    #[test]
    fn cmap_helper_reports_missing_symbols() {
        for ch in "○●◎◇⊕❚".chars() {
            assert!(!has_glyph(UI_FONT_REGULAR, ch), "{ch:?} unexpectedly present");
        }
    }

    #[test]
    fn plugin_installs_without_asset_server() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(UiFontPlugin);
        app.update();
    }

    #[test]
    fn plugin_replaces_default_font_asset() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin::default())
            .init_asset::<Font>()
            .add_plugins(UiFontPlugin);
        app.update();
        let fonts = app.world().resource::<Assets<Font>>();
        assert!(fonts.get(AssetId::<Font>::default()).is_some());
    }

    #[test]
    fn licence_file_ships_beside_the_fonts() {
        let ofl = include_str!("../../assets/fonts/OFL.txt");
        assert!(ofl.contains("SIL Open Font License, Version 1.1"));
        assert!(ofl.contains("Mozilla Foundation"));
    }
}
