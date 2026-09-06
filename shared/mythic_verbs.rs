//! Mythic verbs — read-first after book + standing (v23.2.45)
//!
//! Crownstone Witness · Sylvaris Offer · Hybrid Attune.
//! These are not damage keys. Peace hour stays harvest. Contact: info@Rathor.ai

/// Named mythic verb. Hands still use E; the yard chooses which verb is live.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MythicVerb {
    Witness,
    Offer,
    Attune,
}

impl MythicVerb {
    pub fn label(self) -> &'static str {
        match self {
            MythicVerb::Witness => "Witness",
            MythicVerb::Offer => "Offer",
            MythicVerb::Attune => "Attune",
        }
    }

    /// Constitution: never a combat / F-row / HP verb.
    pub fn is_damage(self) -> bool {
        false
    }

    pub fn requires_hour_three(self) -> bool {
        true
    }
}

/// Gate for revealing mythic yards.
pub fn mythic_unlocked(hour_three_held: bool, embassy_seated: bool) -> bool {
    hour_three_held && embassy_seated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verbs_are_not_damage() {
        for v in [MythicVerb::Witness, MythicVerb::Offer, MythicVerb::Attune] {
            assert!(!v.is_damage());
            assert!(v.requires_hour_three());
        }
    }

    #[test]
    fn gate_needs_book_and_seat() {
        assert!(!mythic_unlocked(false, true));
        assert!(!mythic_unlocked(true, false));
        assert!(mythic_unlocked(true, true));
    }
}
