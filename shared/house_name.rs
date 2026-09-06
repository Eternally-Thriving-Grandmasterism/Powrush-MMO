//! S2 House naming — skippable charter label (v23.2.50)
//!
//! Persist beside climate: `data/powrush_house.json`.
//! Skip → Unnamed House. Never a wall before first E.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const HOUSE_PATH: &str = "data/powrush_house.json";
pub const UNNAMED: &str = "Unnamed House";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HouseName {
    pub schema: String,
    /// Empty means Unnamed when resolved; ignored until resolved.
    #[serde(default)]
    pub name: String,
    /// True after Confirm or Skip. False = not yet asked / first run.
    #[serde(default)]
    pub resolved: bool,
}

impl Default for HouseName {
    fn default() -> Self {
        Self {
            schema: "powrush_house_v1".into(),
            name: String::new(),
            resolved: false,
        }
    }
}

impl HouseName {
    pub fn display_name(&self) -> &str {
        let trimmed = self.name.trim();
        if trimmed.is_empty() {
            UNNAMED
        } else {
            trimmed
        }
    }

    /// First run / Play: no wall. Naming is optional after Settled.
    pub fn blocks_hands(&self) -> bool {
        false
    }

    pub fn confirm(&mut self, raw: &str) {
        let t = raw.trim();
        self.name = if t.is_empty() {
            String::new()
        } else {
            t.chars().take(32).collect()
        };
        self.resolved = true;
    }

    pub fn skip(&mut self) {
        self.name.clear();
        self.resolved = true;
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    pub fn load_or_default() -> Self {
        let Ok(raw) = fs::read_to_string(HOUSE_PATH) else {
            return Self::default();
        };
        Self::from_json(&raw).unwrap_or_default()
    }

    pub fn persist(&self) {
        if let Some(parent) = Path::new(HOUSE_PATH).parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = self.to_json() {
            let _ = fs::write(HOUSE_PATH, json);
        }
    }
}

/// Local yard remember cue for Continue / title.
pub fn continue_cue(hour_two_held: bool, house: &HouseName) -> Option<String> {
    if !hour_two_held && !house.resolved {
        return None;
    }
    let name = house.display_name();
    if hour_two_held {
        Some(format!("{name} · the yard remembers"))
    } else {
        Some(name.to_string())
    }
}

/// Persist exists for Continue when any local yard file is present.
pub fn local_persist_present(
    hour_two_exists: bool,
    climate_exists: bool,
    standing_exists: bool,
    house_resolved: bool,
) -> bool {
    hour_two_exists || climate_exists || standing_exists || house_resolved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_run_has_no_name_wall() {
        let h = HouseName::default();
        assert!(!h.blocks_hands());
        assert!(!h.resolved);
        assert_eq!(h.display_name(), UNNAMED);
        // Play may enter Hands without naming
        assert!(continue_cue(false, &h).is_none());
    }

    #[test]
    fn skip_becomes_unnamed_house() {
        let mut h = HouseName::default();
        h.skip();
        assert!(h.resolved);
        assert_eq!(h.display_name(), UNNAMED);
        let raw = h.to_json().unwrap();
        let back = HouseName::from_json(&raw).unwrap();
        assert_eq!(back.display_name(), UNNAMED);
        assert!(back.resolved);
    }

    #[test]
    fn confirm_keeps_name_and_roundtrips() {
        let mut h = HouseName::default();
        h.confirm("  Steward Ridge  ");
        assert_eq!(h.display_name(), "Steward Ridge");
        let back = HouseName::from_json(&h.to_json().unwrap()).unwrap();
        assert_eq!(back.display_name(), "Steward Ridge");
        assert!(back.resolved);
    }

    #[test]
    fn continue_cue_names_yard_when_hour_two_held() {
        let mut h = HouseName::default();
        h.confirm("Moss House");
        let cue = continue_cue(true, &h).unwrap();
        assert!(cue.contains("Moss House"));
        assert!(cue.contains("yard remembers"));
    }

    #[test]
    fn local_persist_detects_climate_or_hour() {
        assert!(!local_persist_present(false, false, false, false));
        assert!(local_persist_present(true, false, false, false));
        assert!(local_persist_present(false, true, false, false));
        assert!(local_persist_present(false, false, true, false));
        assert!(local_persist_present(false, false, false, true));
    }

    #[test]
    fn house_path_beside_climate() {
        assert_eq!(HOUSE_PATH, "data/powrush_house.json");
        assert!(HOUSE_PATH.starts_with("data/powrush_"));
    }
}
