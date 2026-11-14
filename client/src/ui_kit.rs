//! CARD UI-KIT-1 — shared style tokens for the lived-hour plates.
//!
//! Consts and pure helpers only, no plugin. The regal palette moved here from
//! `title_screen` value for value; `title_screen` re-exports the `TITLE_*` names.
//! Sizes are logical px. `TextFont` stores `logical / 1.2`, so Bevy's default
//! line box equals the logical size, which the 720p plate budgets rely on.
//! The face stays the embedded Fira Sans Regular (`ui_font`).
//! Contact: info@Rathor.ai

use bevy::prelude::*;
use bevy::ui_widgets::Button;

// <title-palette> — every colour literal in this file (outside tests) lives below.
// --- High-contrast title palette (opaque — no alpha-on-fog) -----------------
// Soft GPU / Mesa must read Play · Continue · Online · Settings before the yard.
// CARD VP-UI-REGAL-1 — regal crest: polished gold frames, royal purple fills,
// near-black purple-tinted plates, warm cream / champagne text. Online grey stays.
/// Opaque near-black plate behind menu text, faint royal purple tint (light-on-dark).
pub const TITLE_PLATE_BG: Color = Color::srgb(0.07, 0.05, 0.09);
/// Full-screen dimmer over the world (opaque near-black, faint purple — stranger reads the door).
pub const TITLE_DIM_BG: Color = Color::srgb(0.04, 0.025, 0.05);
/// Primary menu / title text — warm cream, high contrast on TITLE_PLATE_BG.
pub const TITLE_TEXT_PRIMARY: Color = Color::srgb(0.98, 0.95, 0.86);
/// Secondary cue / subtitle text — pale gold / champagne.
pub const TITLE_TEXT_SECONDARY: Color = Color::srgb(0.90, 0.82, 0.62);
/// Enabled button fill (opaque deep royal purple).
pub const TITLE_BTN_BG: Color = Color::srgb(0.19, 0.08, 0.30);
/// Enabled button label — warm champagne cream.
pub const TITLE_BTN_FG: Color = Color::srgb(0.98, 0.94, 0.84);
/// Disabled / Online-grey fill.
pub const TITLE_BTN_DISABLED_BG: Color = Color::srgb(0.10, 0.11, 0.12);
/// Disabled / Online-grey label.
pub const TITLE_BTN_DISABLED_FG: Color = Color::srgb(0.55, 0.58, 0.60);
/// Plate + button border (opaque polished gold).
pub const TITLE_BORDER: Color = Color::srgb(0.84, 0.69, 0.32);
/// CARD VP-UI-REGAL-2 — active Settings tab fill: a lighter royal purple than
/// TITLE_BTN_BG (still purple: blue and red above green).
pub const TITLE_BTN_ACTIVE_BG: Color = Color::srgb(0.32, 0.16, 0.48);
/// CARD VP-UI-REGAL-2 — inactive Settings tab border: quiet champagne gold,
/// dimmer than TITLE_BORDER (not bronze / brown / beige).
pub const TITLE_BORDER_MUTED: Color = Color::srgb(0.66, 0.58, 0.36);
// </title-palette> — every colour literal in this file (outside tests) lives above.

/// Type scale, logical px. Title wordmark.
pub const TYPE_DISPLAY: f32 = 28.0;
/// Pause cue, pickup flash.
pub const TYPE_CUE: f32 = 16.0;
/// Menu buttons and settings rows.
pub const TYPE_BODY: f32 = 15.0;
/// Tabs, guide line, satchel header.
pub const TYPE_LABEL: f32 = 14.0;
/// Satchel body.
pub const TYPE_READ: f32 = 13.5;
/// Banner copy, watch strip.
pub const TYPE_SMALL: f32 = 13.0;
/// Banner Dismiss.
pub const TYPE_CAPTION: f32 = 12.0;
/// Hints.
pub const TYPE_HINT: f32 = 11.0;
/// Text-scale clamp for every scaled label.
pub const TYPE_MIN: f32 = 11.0;
pub const TYPE_MAX: f32 = 22.0;

/// Spacing scale, px.
pub const SPACE_XS: f32 = 4.0;
pub const SPACE_S: f32 = 6.0;
pub const SPACE_M: f32 = 8.0;
pub const SPACE_L: f32 = 10.0;
pub const SPACE_XL: f32 = 12.0;
pub const SPACE_2XL: f32 = 14.0;

/// Border widths, px. Buttons, rows and HUD strips.
pub const BORDER_HAIRLINE: f32 = 1.0;
/// Pause plate and satchel.
pub const BORDER_PLATE: f32 = 1.5;

/// `base × text_scale`, clamped to the type range.
pub fn scaled_px(base: f32, text_scale: f32) -> f32 {
    (base * text_scale).clamp(TYPE_MIN, TYPE_MAX)
}

/// `TextFont` for a logical size.
pub fn font(logical_px: f32) -> TextFont {
    TextFont {
        font_size: FontSize::Px(logical_px / 1.2),
        ..default()
    }
}

/// Opaque plate fill and gold frame (Title contrast law), beside the plate's own `Node`.
pub fn plate() -> (BackgroundColor, BorderColor) {
    (BackgroundColor(TITLE_PLATE_BG), BorderColor::all(TITLE_BORDER))
}

/// Button chrome beside the site's own `Node`: gold frame on `bg`. Disabled passes the Online grey.
pub fn button(bg: Color) -> (Button, Interaction, BorderColor, BackgroundColor) {
    (Button, Interaction::default(), BorderColor::all(TITLE_BORDER), BackgroundColor(bg))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prod(src: &str) -> &str {
        src.split("\n#[cfg(test)]\nmod tests {").next().expect("non-test prefix")
    }

    /// `Color::…` literals, not `BorderColor::all` style token wrappers.
    fn colour_literals(src: &str) -> usize {
        src.match_indices("Color::")
            .filter(|(i, _)| !src[..*i].ends_with(|c: char| c.is_alphanumeric()))
            .count()
    }

    /// Body of `fn name` in `src`, up to the next top-level `}`.
    fn fn_body<'a>(src: &'a str, name: &str) -> &'a str {
        let decl = format!("fn {name}");
        let (start, _) = src.match_indices(&decl).find(|(i, _)| src[i + decl.len()..].starts_with(['(', '<'])).unwrap_or_else(|| panic!("{decl}"));
        let end = src[start..].find("\n}\n").expect("fn end");
        &src[start..start + end]
    }

    /// VP-UI-REGAL-2 in its new home: one marker pair, every colour literal inside it.
    #[test]
    fn palette_block_owns_every_colour_literal() {
        let body = prod(include_str!("ui_kit.rs"));
        let open = body.find("// <title-palette>").expect("open marker");
        let close = body.find("// </title-palette>").expect("close marker");
        assert!(open < close);
        assert_eq!(body.matches("// <title-palette>").count(), 1);
        let outside = format!("{}{}", &body[..open], &body[close..]);
        assert_eq!(colour_literals(&outside), 0, "colour literal outside the palette");
        assert_eq!(body[open..close].matches("pub const TITLE_").count(), 11);
    }

    #[test]
    fn palette_values_are_the_regal_crest() {
        assert_eq!(TITLE_PLATE_BG, Color::srgb(0.07, 0.05, 0.09));
        assert_eq!(TITLE_DIM_BG, Color::srgb(0.04, 0.025, 0.05));
        assert_eq!(TITLE_BTN_BG, Color::srgb(0.19, 0.08, 0.30));
        assert_eq!(TITLE_BORDER, Color::srgb(0.84, 0.69, 0.32));
        assert_eq!(TITLE_BTN_DISABLED_BG, Color::srgb(0.10, 0.11, 0.12));
        assert_eq!(TITLE_BTN_DISABLED_FG, Color::srgb(0.55, 0.58, 0.60));
        assert_eq!(crate::title_screen::TITLE_BTN_ACTIVE_BG, TITLE_BTN_ACTIVE_BG);
    }

    /// The scale is today's sizes, largest first, inside the text-scale clamp.
    #[test]
    fn type_scale_is_ordered_and_inside_the_clamp() {
        let scale = [TYPE_CUE, TYPE_BODY, TYPE_LABEL, TYPE_READ, TYPE_SMALL, TYPE_CAPTION, TYPE_HINT];
        assert!(TYPE_DISPLAY > TYPE_MAX && scale.windows(2).all(|w| w[0] > w[1]));
        assert!(scale.iter().all(|px| (TYPE_MIN..=TYPE_MAX).contains(px)));
        for px in scale {
            assert_eq!(scaled_px(px, 1.0), px);
            assert_eq!(font(px).font_size, FontSize::Px(px / 1.2));
        }
        assert_eq!((scaled_px(15.0, 0.1), scaled_px(15.0, 5.0)), (TYPE_MIN, TYPE_MAX));
        let space = [SPACE_XS, SPACE_S, SPACE_M, SPACE_L, SPACE_XL, SPACE_2XL];
        assert!(space.windows(2).all(|w| w[0] < w[1]) && BORDER_HAIRLINE < BORDER_PLATE);
    }

    #[test]
    fn button_helpers_keep_online_grey_and_plate_is_opaque() {
        let (_, interaction, frame, bg) = button(TITLE_BTN_DISABLED_BG);
        assert_eq!((interaction, frame, bg.0), (Interaction::None, BorderColor::all(TITLE_BORDER), TITLE_BTN_DISABLED_BG));
        let (plate_bg, plate_frame) = plate();
        assert_eq!((plate_bg.0.to_srgba().alpha, plate_frame), (1.0, BorderColor::all(TITLE_BORDER)));
        assert!(shared::title_house_proof::online_row_is_honest_disabled(
            shared::title_house_proof::ONLINE_STUB_LABEL,
            false
        ));
    }

    /// Pause and satchel spawn sites read the tokens: no numeric font size,
    /// colour literal or border width is written inside them.
    #[test]
    fn pause_and_satchel_sites_read_tokens() {
        let title = prod(include_str!("title_screen.rs"));
        let satchel = prod(include_str!("human_inventory.rs"));
        let sites = [
            fn_body(title, "spawn_menu_btn"),
            fn_body(title, "spawn_title_plate_btn"),
            fn_body(title, "spawn_settings_stub"),
            fn_body(title, "spawn_comfort_graphics_banner_row"),
            fn_body(title, "spawn_pause_tab_btn"),
            fn_body(title, "spawn_settings_row_padded"),
            fn_body(satchel, "spawn_inventory_surfaces"),
        ];
        let token_slots = [
            "FontSize::Px(",
            "border: UiRect::all(Val::Px(",
            "padding: UiRect::all(Val::Px(",
            "padding: UiRect::axes(Val::Px(",
            "row_gap: Val::Px(",
            "column_gap: Val::Px(",
        ];
        for site in sites {
            let head = &site[..site.find('(').unwrap_or(40)];
            for slot in token_slots {
                for (i, _) in site.match_indices(slot) {
                    let arg = &site[i + slot.len()..];
                    assert!(!arg.starts_with(|c: char| c.is_ascii_digit()), "{head}: literal after {slot}");
                }
            }
            assert_eq!(colour_literals(site), 0, "{head}: colour literal");
        }
    }
}
