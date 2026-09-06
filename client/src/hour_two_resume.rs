//! Hour-two resume lines — Slice 24 (v23.2.31)
//!
//! Pure sentences. Client welcome slab and tests share this.
//! Contact: info@Rathor.ai

/// Welcome slab copy. None = stay quiet (first boot, empty echo).
pub fn welcome_line(hour_two_held: bool, sealed: bool, last_echo: Option<&str>) -> Option<String> {
    if hour_two_held {
        return Some("Welcome back · Hour two held · the yard remembers".into());
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
    fn held_yard_beats_empty_echo() {
        let line = welcome_line(true, false, None).unwrap();
        assert!(line.contains("Hour two held"));
        assert!(line.contains("yard remembers"));
        assert!(!line.contains("Lattice"));
    }

    #[test]
    fn first_boot_stays_quiet() {
        assert_eq!(welcome_line(false, false, None), None);
    }
}
