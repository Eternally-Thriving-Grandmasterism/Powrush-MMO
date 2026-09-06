//! Hour resume lines — Slice 24 (v23.2.31) + Hour three (v23.2.35)
//! + Playable-loop polish (v23.2.46)
//!
//! Pure sentences. Client welcome slab and tests share this.
//! Contact: info@Rathor.ai

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
        assert_eq!(welcome_line(false, false, false, None), None);
    }
}
