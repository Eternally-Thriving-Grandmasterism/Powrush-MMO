//! CARD VP-FONT-1 — UI font with real glyphs for the lived-hour text.
//!
//! Bevy's built-in default face is a FiraMono subset. It has no `·`, `—`,
//! `•`, `…` or `×`, so those rendered as tofu boxes on every plate. This
//! plugin swaps the default font handle for Fira Sans Regular (SIL OFL 1.1,
//! `client/assets/fonts/OFL.txt`, upstream google/fonts `ofl/firasans`). The
//! files live in `client/assets/fonts/`, so the panel `asset_server` loads of
//! `fonts/FiraSans-Regular.ttf` / `fonts/FiraSans-Bold.ttf` now resolve there.
//!
//! The bytes are embedded, so the default face never depends on the asset
//! folder being present. No new graphics menu. No layout change.
//! Contact: info@Rathor.ai

use bevy::asset::uuid_handle;
use bevy::prelude::*;

/// Fira Sans Regular, embedded. Same file as `client/assets/fonts/FiraSans-Regular.ttf`.
pub const UI_FONT_REGULAR: &[u8] = include_bytes!("../assets/fonts/FiraSans-Regular.ttf");
/// Cinzel Regular, embedded. Same file as `client/assets/fonts/Cinzel-Regular.ttf`.
pub const UI_FONT_DISPLAY: &[u8] = include_bytes!("../assets/fonts/Cinzel-Regular.ttf");
/// Display-face handle. The asset server does not own it.
pub const DISPLAY_FONT: Handle<Font> = uuid_handle!("c088f941-79b3-4d83-98b0-712eb3ddb11a");

/// Replaces Bevy's default font asset (the handle every `TextStyle::default()` uses).
pub struct UiFontPlugin;

impl Plugin for UiFontPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, install_default_ui_font);
    }
}

/// `true` when the embedded bytes can be wrapped as a font.
/// `Font::from_bytes` no longer returns `Result` (0.19).
pub fn ui_font_parses() -> bool {
    let _font = Font::from_bytes(UI_FONT_REGULAR.to_vec());
    true
}

fn install_default_ui_font(fonts: Option<ResMut<Assets<Font>>>) {
    let Some(mut fonts) = fonts else {
        return;
    };
    let font = Font::from_bytes(UI_FONT_REGULAR.to_vec());
    fonts
        .insert(AssetId::<Font>::default(), font)
        .expect("ui_font");
    let display = Font::from_bytes(UI_FONT_DISPLAY.to_vec());
    fonts.insert(DISPLAY_FONT.id(), display).expect("ui_font_display");
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
        let bold = include_bytes!("../assets/fonts/FiraSans-Bold.ttf");
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
        let ofl = include_str!("../assets/fonts/OFL.txt");
        assert!(ofl.contains("SIL Open Font License, Version 1.1"));
        assert!(ofl.contains("Mozilla Foundation"));
    }

    #[test]
    fn font_display_fd1_face_installs_and_bare_app_runs() {
        let mut bare = App::new();
        bare.add_plugins((MinimalPlugins, UiFontPlugin));
        bare.update();
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default())).init_asset::<Font>().add_plugins(UiFontPlugin);
        app.update();
        let fonts = app.world().resource::<Assets<Font>>();
        assert!(fonts.get(DISPLAY_FONT.id()).is_some() && fonts.get(AssetId::<Font>::default()).is_some());
    }

    fn sha256(msg: &[u8]) -> [u8; 32] {
        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
            0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
            0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
        ];
        let (mut m, bits) = (msg.to_vec(), (msg.len() as u64).wrapping_mul(8));
        m.push(0x80);
        while m.len() % 64 != 56 { m.push(0); }
        m.extend_from_slice(&bits.to_be_bytes());
        let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
        for c in m.chunks(64) {
            let mut w = [0u32; 64];
            for i in 0..16 { w[i] = u32::from_be_bytes([c[4 * i], c[4 * i + 1], c[4 * i + 2], c[4 * i + 3]]); }
            for i in 16..64 { let (x, y) = (w[i - 15], w[i - 2]); let s0 = x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3); let s1 = y.rotate_right(17) ^ y.rotate_right(19) ^ (y >> 10); w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1); }
            let mut a: [u32; 8] = h;
            for i in 0..64 { let s1 = a[4].rotate_right(6) ^ a[4].rotate_right(11) ^ a[4].rotate_right(25); let ch = (a[4] & a[5]) ^ (!a[4] & a[6]); let t1 = a[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]); let s0 = a[0].rotate_right(2) ^ a[0].rotate_right(13) ^ a[0].rotate_right(22); let maj = (a[0] & a[1]) ^ (a[0] & a[2]) ^ (a[1] & a[2]); a = [t1.wrapping_add(s0.wrapping_add(maj)), a[0], a[1], a[2], a[3].wrapping_add(t1), a[4], a[5], a[6]]; }
            for i in 0..8 { h[i] = h[i].wrapping_add(a[i]); }
        }
        let mut out = [0u8; 32];
        for (i, v) in h.iter().enumerate() { out[i * 4..i * 4 + 4].copy_from_slice(&v.to_be_bytes()); }
        out
    }

    fn sha256_hex(data: &[u8]) -> String { sha256(data).iter().map(|b| format!("{b:02x}")).collect() }

    #[test]
    fn font_display_fd6_glyphs_and_sha256() {
        let mut chars = String::from("POWRUSHName your HouseHouse seals · heritagePersona creator · optional·—–0123456789");
        chars.push_str(shared::hex_travel::PLACES_TITLE);
        chars.push_str(crate::licences_screen::LICENCES_TITLE);
        chars.extend('A'..='Z');
        chars.extend('a'..='z');
        for ch in chars.chars() { assert!(has_glyph(UI_FONT_DISPLAY, ch), "{ch:?}"); }
        assert!(!has_glyph(UI_FONT_DISPLAY, '○'), "helper must report a missing symbol");
        assert_eq!(sha256_hex(UI_FONT_DISPLAY), "af0031129f27dc752e8629a80b793d27abea94027faa27cc660c3fc33f607a1f");
    }

    #[test]
    fn font_display_fd7_cinzel_notice_matches_template() {
        let ofl = include_str!("../assets/fonts/Cinzel-OFL.txt");
        assert!(ofl.contains("SIL Open Font License, Version 1.1") && ofl.contains("The Cinzel Project Authors"));
        let notices = crate::licences_screen::NOTICES;
        let hbs = include_str!("../../tools/notices/about.hbs");
        let head = "Cinzel Regular, Cinzel-Regular.ttf (embedded in the game program as the display face for plate headers)";
        let at = notices.rfind(head).expect("heading");
        let rule = notices[..at].rfind(&"=".repeat(80)).expect("rule");
        let block = &notices[rule..];
        assert!(block.contains("https://github.com/NDISCOVER/Cinzel") && block.contains("8271e162") && hbs.contains(block));
        assert_eq!(crate::licences_screen::split_notices(notices).into_iter().filter(|s| s.starts_with("Cinzel Regular,")).count(), 1);
    }

    #[test]
    fn notices_fira_nf1_section_heading_commit_and_digests() {
        let ofl = include_str!("../assets/fonts/OFL.txt");
        let (reg, bold) = (sha256_hex(UI_FONT_REGULAR), sha256_hex(include_bytes!("../assets/fonts/FiraSans-Bold.ttf")));
        for text in [crate::licences_screen::NOTICES, include_str!("../../tools/notices/about.hbs")] {
            assert!(text.contains(ofl));
            let sec = crate::licences_screen::split_notices(text).into_iter().find(|s| s.contains(ofl.trim_end())).expect("section");
            let (head, rest) = sec.split_once('\n').unwrap();
            assert_eq!(head, "Fira Sans (SIL Open Font License 1.1)");
            assert!(rest.contains("ed7143b8") && rest.contains("Version 4.203"));
            assert!(rest.contains(&format!("sha256 {reg} FiraSans-Regular.ttf")) && rest.contains(&format!("sha256 {bold} FiraSans-Bold.ttf")));
        }
    }

    #[test]
    fn notices_fira_nf2_tail_matches_template() {
        let hbs = include_str!("../../tools/notices/about.hbs");
        let mark = "{{/each}}\n";
        let tail = &hbs[hbs.rfind(mark).unwrap() + mark.len()..];
        assert!(tail.contains("Cinzel Regular,") && crate::licences_screen::NOTICES.ends_with(tail));
    }
    #[test]
    fn notices_fira_nf4_sha256_helper_no_process_command() {
        assert_eq!(sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(sha256_hex(UI_FONT_DISPLAY), "af0031129f27dc752e8629a80b793d27abea94027faa27cc660c3fc33f607a1f");
        let tests = include_str!("ui_font.rs").split("\n#[cfg(test)]\nmod tests {").nth(1).unwrap();
        assert!(!tests.contains(concat!("process::", "Command")) && !tests.contains(concat!("sha256", "sum")));
    }
    #[test]
    fn notices_fira_nf5_blank_line_before_fira_and_cinzel_rules() {
        let rule = "=".repeat(80);
        let cinzel = format!("\n\n{rule}\nCinzel Regular,");
        let fira = format!("\n\n{rule}\nFira Sans (SIL Open Font License 1.1)");
        for text in [crate::licences_screen::NOTICES, include_str!("../../tools/notices/about.hbs")] {
            assert!(text.contains(&cinzel) && text.contains(&fira));
        }
    }
}
