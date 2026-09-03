//! Species redemption v0 — Slice 12 (v23.2.16)
//!
//! One tend offered to the Sylvaris grove. Five progresses exist; only Sylvaris moves.
//! Not boarding, not Hivelord, not a Brood Spire. Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Five enslaved-species progresses + moral weight. v0 only ticks Sylvaris.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpeciesRedemptionState {
    pub veythari: f32,
    pub korrath: f32,
    pub sylvaris: f32,
    pub luminari: f32,
    pub voidweaver: f32,
    pub moral_weight: f32,
    pub events: u32,
    pub last_line: String,
}

impl Default for SpeciesRedemptionState {
    fn default() -> Self {
        Self {
            veythari: 0.0,
            korrath: 0.0,
            sylvaris: 0.0,
            luminari: 0.0,
            voidweaver: 0.0,
            moral_weight: 0.0,
            events: 0,
            last_line: String::new(),
        }
    }
}

impl SpeciesRedemptionState {
    pub fn reveal(&mut self) {
        if self.last_line.is_empty() {
            self.last_line = "Sylvaris grove · progress 0 · E Offer a tend".into();
        }
    }

    /// One local tend. Does not free a unit. Does not fire a Hivelord.
    pub fn offer_tend(&mut self) -> &'static str {
        if self.events > 0 {
            return "idle";
        }
        self.sylvaris = (self.sylvaris + 0.25).min(1.0);
        self.moral_weight = (self.moral_weight + 0.10).min(1.0);
        self.events = 1;
        self.last_line = "A tend offered — the grove answers · Sylvaris 0.25".into();
        "tended"
    }

    pub fn slab_line(&self) -> String {
        if self.last_line.is_empty() {
            "Sylvaris grove · progress 0 · E Offer a tend".into()
        } else {
            self.last_line.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_zero_five() {
        let s = SpeciesRedemptionState::default();
        assert_eq!(s.sylvaris, 0.0);
        assert_eq!(s.veythari, 0.0);
        assert_eq!(s.events, 0);
        let blob = format!("{s:?}").to_lowercase();
        assert!(!blob.contains("hivelord"));
        assert!(!blob.contains("brood"));
    }

    #[test]
    fn one_tend_moves_sylvaris_only() {
        let mut s = SpeciesRedemptionState::default();
        assert_eq!(s.offer_tend(), "tended");
        assert!((s.sylvaris - 0.25).abs() < 0.001);
        assert_eq!(s.veythari, 0.0);
        assert_eq!(s.korrath, 0.0);
        assert!(s.moral_weight > 0.0);
        assert_eq!(s.offer_tend(), "idle");
    }
}
