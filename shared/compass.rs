//! Compass tells — Slice 14 (v23.2.18)
//!
//! Live W at 20 and 60. Peace is silent 0. UI never shows the formula.
//! Contact: info@Rathor.ai

use crate::space_law::{HexFlag, WarrantWeight};

/// Climate tell. None in Peace. None below 20.
pub fn tell(weight: &WarrantWeight, hex: HexFlag) -> Option<&'static str> {
    if hex == HexFlag::Peace {
        return None;
    }
    let w = weight.live(hex);
    if w >= 60.0 {
        Some("Compass · 60 — the air thickens")
    } else if w >= 20.0 {
        Some("Compass · 20 — a cited wind")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peace_is_silent() {
        let mut w = WarrantWeight::default();
        w.x = 40.0;
        assert_eq!(tell(&w, HexFlag::Peace), None);
    }

    #[test]
    fn twenty_and_sixty() {
        let mut w = WarrantWeight::default();
        w.x = 10.0; // 2.00 * 10 = 20
        assert_eq!(tell(&w, HexFlag::Frontier), Some("Compass · 20 — a cited wind"));
        w.x = 30.0; // 60
        assert_eq!(tell(&w, HexFlag::Frontier), Some("Compass · 60 — the air thickens"));
        w.x = 0.0;
        assert_eq!(tell(&w, HexFlag::Frontier), None);
    }
}
