//! Hour resume lines — Slice 24 (v23.2.31) + Hour three (v23.2.35)
//! + Playable-loop polish (v23.2.46)
//!
//! Pure sentences. Client welcome slab and tests share this.
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
pub fn welcome_line(
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
        let line = welcome_line(true, true, false, None).unwrap();
        assert!(line.contains("Hour three held"));
        assert!(line.contains("book is yours"));
        assert!(!line.to_lowercase().contains("lethal"));
        assert!(!line.to_lowercase().contains("digit"));
    }

    #[test]
    fn held_yard_beats_empty_echo() {
        let line = welcome_line(false, true, false, None).unwrap();
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
        let line = welcome_line(false, true, false, None).unwrap();
        assert!(hour_two_welcome_reward(Some(line.as_str())));
        assert_eq!(welcome_glow_from_line(Some(line.as_str())), 1.0);
        assert!(line.contains("Hour two held"));
        assert!(line.contains("the yard remembers"));
    }

    #[test]
    fn other_welcome_lines_stay_mute() {
        let three = welcome_line(true, true, false, None).unwrap();
        assert!(!hour_two_welcome_reward(Some(three.as_str())));
        assert_eq!(welcome_glow_from_line(Some(three.as_str())), 0.0);
        let sealed = welcome_line(false, false, true, None).unwrap();
        assert!(!hour_two_welcome_reward(Some(sealed.as_str())));
        assert_eq!(welcome_glow_from_line(Some(sealed.as_str())), 0.0);
        let echo = welcome_line(false, false, false, Some("tend")).unwrap();
        assert!(!hour_two_welcome_reward(Some(echo.as_str())));
        assert_eq!(welcome_glow_from_line(Some(echo.as_str())), 0.0);
        assert!(!hour_two_welcome_reward(None));
        assert_eq!(welcome_glow_from_line(None), 0.0);
    }
}
