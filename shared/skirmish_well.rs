//! Skirmish well — Slice 15 (v23.2.22)
//!
//! Practice travelers hold the first well. E contests. Dawn after loss.
//! Mercy aftercare. No corpse-grey. No F-key. Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Same three anchors the harvest nodes and practice travelers already use.
pub const WELL_ANCHORS: [(f32, f32, f32); 3] = [
    (3.6, 0.55, 0.0),
    (-2.4, 0.55, 3.1),
    (1.2, 0.55, -3.4),
];

pub const CONTEST_REACH: f32 = 2.4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum WellHold {
    #[default]
    Traveler,
    Human,
    Aftercare,
}

impl WellHold {
    pub fn label(self) -> &'static str {
        match self {
            WellHold::Traveler => "Mira holds",
            WellHold::Human => "yours",
            WellHold::Aftercare => "dawn after loss",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkirmishWell {
    pub index: usize,
    pub hold: WellHold,
    pub wins: u32,
    pub losses: u32,
    pub last_line: String,
}

impl Default for SkirmishWell {
    fn default() -> Self {
        Self {
            index: 0,
            hold: WellHold::Traveler,
            wins: 0,
            losses: 0,
            last_line: String::new(),
        }
    }
}

impl SkirmishWell {
    pub fn reveal(&mut self) {
        if self.last_line.is_empty() {
            self.last_line = "Well · Mira holds · E Contest".into();
        }
    }

    pub fn wants_interact(&self) -> bool {
        matches!(self.hold, WellHold::Traveler | WellHold::Aftercare)
    }

    pub fn contest(&mut self) -> &'static str {
        if self.hold != WellHold::Traveler {
            return "idle";
        }
        self.hold = WellHold::Human;
        self.wins = self.wins.saturating_add(1);
        self.last_line = "The well is yours — Mira stepped back".into();
        "won"
    }

    pub fn traveler_answers(&mut self) -> &'static str {
        if self.hold != WellHold::Human {
            return "idle";
        }
        self.hold = WellHold::Aftercare;
        self.losses = self.losses.saturating_add(1);
        self.last_line = "Dawn after loss — you still walk · E Rise".into();
        "lost"
    }

    pub fn dawn(&mut self) -> &'static str {
        if self.hold != WellHold::Aftercare {
            return "idle";
        }
        self.hold = WellHold::Traveler;
        self.last_line = "Dawn — Mira holds the well again · E Contest".into();
        "dawn"
    }

    pub fn act(&mut self) -> &'static str {
        match self.hold {
            WellHold::Traveler => self.contest(),
            WellHold::Aftercare => self.dawn(),
            WellHold::Human => "idle",
        }
    }

    pub fn slab_line(&self) -> String {
        if self.last_line.is_empty() {
            "Well · Mira holds · E Contest".into()
        } else {
            self.last_line.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contest_win_then_dawn_after_loss() {
        let mut w = SkirmishWell::default();
        assert_eq!(w.hold, WellHold::Traveler);
        assert_eq!(w.contest(), "won");
        assert_eq!(w.hold, WellHold::Human);
        assert_eq!(w.traveler_answers(), "lost");
        assert_eq!(w.hold, WellHold::Aftercare);
        assert!(w.last_line.contains("Dawn"));
        assert!(!w.last_line.to_lowercase().contains("corpse"));
        assert_eq!(w.dawn(), "dawn");
        assert_eq!(w.hold, WellHold::Traveler);
    }

    #[test]
    fn no_f_key_and_anchors_match_hour() {
        let w = SkirmishWell::default();
        let blob = format!("{w:?}").to_lowercase();
        assert!(!blob.contains("f-key"));
        assert_eq!(WELL_ANCHORS[0], (3.6, 0.55, 0.0));
        assert_eq!(WELL_ANCHORS.len(), 3);
    }
}
