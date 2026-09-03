//! War week — Slice 9 (v23.2.13)
//!
//! Score = tons delivered + nodes restored. Not loot. Not lethal.
//! Declared hex stays Frontier jurisdiction. Local graph.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WarWeek {
    pub declared: bool,
    pub tons: f32,
    pub restored: u32,
    pub last_line: String,
}

impl WarWeek {
    pub fn score(&self) -> f32 {
        self.tons + self.restored as f32
    }

    pub fn declare(&mut self) -> &'static str {
        if self.declared {
            return "idle";
        }
        self.declared = true;
        self.last_line = "War week declared — score is tons plus restored nodes".into();
        "declared"
    }

    /// Ingest from the living graph: logi crates are tons, mend is restored.
    pub fn ingest(&mut self, tons: f32, restored: u32) {
        self.tons = tons;
        self.restored = restored;
        if self.declared {
            self.last_line = self.line();
        }
    }

    pub fn line(&self) -> String {
        if !self.declared {
            return "Tab Chart · declare War week (tons + restored, not loot)".into();
        }
        format!(
            "War week · {:.0} tons · {} restored · score {:.0} · hex gone green",
            self.tons,
            self.restored,
            self.score()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_is_tons_plus_restored() {
        let mut w = WarWeek::default();
        assert_eq!(w.declare(), "declared");
        w.ingest(3.0, 2);
        assert_eq!(w.score(), 5.0);
        assert!(w.line().contains("3 tons"));
        assert!(w.line().contains("2 restored"));
        assert!(!w.line().to_lowercase().contains("kill"));
        assert!(!w.line().to_lowercase().contains("loot"));
    }

    #[test]
    fn undeclared_has_zero_score() {
        let w = WarWeek::default();
        assert!(!w.declared);
        assert_eq!(w.score(), 0.0);
    }
}
