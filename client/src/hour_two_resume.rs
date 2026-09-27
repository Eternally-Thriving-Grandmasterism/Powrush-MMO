//! Hour resume lines — Slice 24 (v23.2.31) + Hour three (v23.2.35)
//! + Playable-loop polish (v23.2.46)
//!
//! Pure sentences. Client welcome slab and tests share this.
//!
//! CARD FLESH-HOUR-TWO-RESUME — `welcome_line` may name the Place
//! (`PlaceId::chip_name` from the saved current hex, the same file
//! `HexTravelState` boots from). Absent travel keeps each sentence byte
//! for byte. First Play stays `None`, so WAVE-C4 glow stays 0.
//! `first_harvest_epiphany.rs` paints the slab and stays spent.
//! Peak memory, cited: walked · tended · week was the bill · yard remembered.
//!
//! Contact: info@Rathor.ai

/// Soft reward beat only when the welcome line names Hour two held.
/// First boot (`None`) and other echoes stay mute — no glow without the held pack.
pub fn hour_two_welcome_reward(line: Option<&str>) -> bool {
    line.map(|s| s.contains("Hour two held")).unwrap_or(false)
}

/// Welcome-slab rim breath. First Play stays at 0 — no XP sparkle chrome
/// unless [`hour_two_welcome_reward`] is true.
pub fn welcome_glow_from_line(line: Option<&str>) -> f32 {
    if hour_two_welcome_reward(line) {
        1.0
    } else {
        0.0
    }
}

/// Welcome slab copy. None = stay quiet (first boot, empty echo).
/// A saved current hex prefixes `chip_name`. No file keeps each sentence exact.
pub fn welcome_line(
    hour_three_held: bool,
    hour_two_held: bool,
    sealed: bool,
    last_echo: Option<&str>,
) -> Option<String> {
    let bare = bare_welcome_line(hour_three_held, hour_two_held, sealed, last_echo)?;
    Some(with_place(bare, saved_hex_chip()))
}

fn with_place(bare: String, place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {bare}"),
        None => bare,
    }
}

/// `{place} · {sentence}` when a chip is present. `None` returns the bare sentence.
fn welcome_line_at(
    hour_three_held: bool,
    hour_two_held: bool,
    sealed: bool,
    last_echo: Option<&str>,
    place: Option<&str>,
) -> Option<String> {
    let bare = bare_welcome_line(hour_three_held, hour_two_held, sealed, last_echo)?;
    Some(with_place(bare, place))
}

/// Saved place chip, or nothing when the current-hex file is absent.
fn saved_hex_chip() -> Option<&'static str> {
    shared::hex_travel::read_current_named().map(|id| id.chip_name())
}

fn bare_welcome_line(
    hour_three_held: bool,
    hour_two_held: bool,
    sealed: bool,
    last_echo: Option<&str>,
) -> Option<String> {
    if hour_three_held {
        // Book held. Lethal stays discoverable on Ledger 3 — not shouted here.
        return Some("Welcome back · Hour three held · the book is yours".into());
    }
    if hour_two_held {
        return Some(
            "Welcome back · Hour two held · the yard remembers · climate on the slab".into(),
        );
    }
    if sealed {
        return Some(
            "Welcome back · your sealed practice still travels with you · J to remember".into(),
        );
    }
    if let Some(last) = last_echo {
        return Some(format!("Welcome back · last echo: {last} · J to open journey"));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hour_three_beats_hour_two() {
        let line = welcome_line_at(true, true, false, None, None).unwrap();
        assert!(line.contains("Hour three held"));
        assert!(line.contains("book is yours"));
        assert!(!line.to_lowercase().contains("lethal"));
        assert!(!line.to_lowercase().contains("digit"));
    }

    #[test]
    fn held_yard_beats_empty_echo() {
        let line = welcome_line_at(false, true, false, None, None).unwrap();
        assert!(line.contains("Hour two held"));
        assert!(line.contains("yard remembers"));
        assert!(line.contains("climate"));
        assert!(!line.contains("Lattice"));
        assert!(!line.to_lowercase().contains("lethal"));
    }

    #[test]
    fn first_boot_stays_quiet() {
        // First Play: no held pack → no welcome line → no reward glow / XP sparkle.
        assert_eq!(welcome_line(false, false, false, None), None);
        assert_eq!(welcome_line_at(false, false, false, None, None), None);
        assert!(!hour_two_welcome_reward(None));
        assert_eq!(welcome_glow_from_line(None), 0.0);
        assert_eq!(welcome_glow_from_line(Some("")), 0.0);
        assert_eq!(
            welcome_glow_from_line(Some("Welcome back · last echo: tend · J to open journey")),
            0.0
        );
    }

    #[test]
    fn hour_two_held_line_earns_reward_glow() {
        let line = welcome_line_at(false, true, false, None, None).unwrap();
        assert!(hour_two_welcome_reward(Some(line.as_str())));
        assert_eq!(welcome_glow_from_line(Some(line.as_str())), 1.0);
        assert!(line.contains("Hour two held"));
        assert!(line.contains("the yard remembers"));
    }

    /// Playtest H2-RESUME: the welcome sentence is the three-beat slab, not WASD.
    #[test]
    fn h2_resume_welcome_names_the_yard() {
        let line = welcome_line_at(false, true, false, None, None).unwrap();
        assert!(line.contains("Welcome back"));
        assert!(line.contains("Hour two held"));
        assert!(line.contains("the yard remembers"));
        assert!(!line.to_lowercase().contains("wasd"));
        assert_eq!(welcome_glow_from_line(Some(line.as_str())), 1.0);
        assert_eq!(welcome_glow_from_line(None), 0.0);
    }

    #[test]
    fn other_welcome_lines_stay_mute() {
        let three = welcome_line_at(true, true, false, None, None).unwrap();
        assert!(!hour_two_welcome_reward(Some(three.as_str())));
        assert_eq!(welcome_glow_from_line(Some(three.as_str())), 0.0);
        let sealed = welcome_line_at(false, false, true, None, None).unwrap();
        assert!(!hour_two_welcome_reward(Some(sealed.as_str())));
        assert_eq!(welcome_glow_from_line(Some(sealed.as_str())), 0.0);
        let echo = welcome_line_at(false, false, false, Some("tend"), None).unwrap();
        assert!(!hour_two_welcome_reward(Some(echo.as_str())));
        assert_eq!(welcome_glow_from_line(Some(echo.as_str())), 0.0);
        assert!(!hour_two_welcome_reward(None));
        assert_eq!(welcome_glow_from_line(None), 0.0);
    }

    /// CARD FLESH-HOUR-TWO-RESUME — Place prefixes each sentence; no travel keeps it exact.
    #[test]
    fn flesh_hour_two_resume_names_chip_or_keeps_sentence() {
        use shared::hex_travel::PlaceId;

        let hour_three = "Welcome back · Hour three held · the book is yours";
        let hour_two = "Welcome back · Hour two held · the yard remembers · climate on the slab";
        let sealed = "Welcome back · your sealed practice still travels with you · J to remember";
        let echo = "Welcome back · last echo: tend · J to open journey";

        assert_eq!(
            welcome_line_at(true, true, false, None, None).as_deref(),
            Some(hour_three)
        );
        assert_eq!(
            welcome_line_at(false, true, false, None, None).as_deref(),
            Some(hour_two)
        );
        assert_eq!(
            welcome_line_at(false, false, true, None, None).as_deref(),
            Some(sealed)
        );
        assert_eq!(
            welcome_line_at(false, false, false, Some("tend"), None).as_deref(),
            Some(echo)
        );
        assert_eq!(welcome_line_at(false, false, false, None, None), None);
        assert_eq!(welcome_glow_from_line(None), 0.0);

        let cases = [
            (PlaceId::Sanctuary, "Sanctuary Prime"),
            (PlaceId::Heartwood, "Heartwood"),
            (PlaceId::Depths, "Depths"),
        ];
        for (id, name) in cases {
            assert_eq!(id.chip_name(), name);
            let chip = id.chip_name();
            assert_eq!(
                welcome_line_at(true, true, false, None, Some(chip)).unwrap(),
                format!("{name} · {hour_three}")
            );
            assert_eq!(
                welcome_line_at(false, true, false, None, Some(chip)).unwrap(),
                format!("{name} · {hour_two}")
            );
            assert_eq!(
                welcome_line_at(false, false, true, None, Some(chip)).unwrap(),
                format!("{name} · {sealed}")
            );
            assert_eq!(
                welcome_line_at(false, false, false, Some("tend"), Some(chip)).unwrap(),
                format!("{name} · {echo}")
            );
        }

        let prefixed_two = welcome_line_at(false, true, false, None, Some("Heartwood")).unwrap();
        assert_eq!(welcome_glow_from_line(Some(prefixed_two.as_str())), 1.0);
        for line in [
            welcome_line_at(true, true, false, None, Some("Depths")).unwrap(),
            welcome_line_at(false, false, true, None, Some("Sanctuary Prime")).unwrap(),
            welcome_line_at(false, false, false, Some("tend"), Some("Heartwood")).unwrap(),
        ] {
            assert_eq!(
                welcome_glow_from_line(Some(line.as_str())),
                0.0,
                "non-hour-two welcome must not light glow: {line}"
            );
        }

        for sample in [
            welcome_line_at(false, true, false, None, Some(PlaceId::Sanctuary.chip_name())).unwrap(),
            welcome_line_at(true, true, false, None, Some(PlaceId::Heartwood.chip_name())).unwrap(),
            welcome_line_at(false, false, true, None, Some(PlaceId::Depths.chip_name())).unwrap(),
            welcome_line_at(false, true, false, None, None).unwrap(),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("xp"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!low.contains("online"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }
}
