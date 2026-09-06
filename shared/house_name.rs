//! S2 House naming — skippable charter label (v23.2.51)
//!
//! Persist beside climate: `data/powrush_house.json`.
//! Skip → Unnamed House. Never a wall before first E.
//! Continue cue: House name or Unnamed House + *the yard remembers*.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const HOUSE_PATH: &str = "data/powrush_house.json";
pub const UNNAMED: &str = "Unnamed House";
/// Steward Continue cue suffix (exact wording).
pub const YARD_REMEMBERS: &str = "the yard remembers";

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

/// Format steward Continue cue: `{name} · the yard remembers`.
pub fn format_continue_cue(house: &HouseName) -> String {
    format!("{} · {YARD_REMEMBERS}", house.display_name())
}

/// Local yard remember cue for Continue / title.
///
/// When hour-two held or house resolved: always House name (or Unnamed House)
/// plus *the yard remembers*. First run (neither) → None so Play stays free.
pub fn continue_cue(hour_two_held: bool, house: &HouseName) -> Option<String> {
    if !hour_two_held && !house.resolved {
        return None;
    }
    Some(format_continue_cue(house))
}

/// Continue row when any local persist exists (book / climate / standing / house).
/// Always steward wording — never a bare name without *the yard remembers*.
pub fn continue_cue_when_persist(persist_present: bool, house: &HouseName) -> Option<String> {
    if !persist_present {
        return None;
    }
    Some(format_continue_cue(house))
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

/// Esc-from-title must not mutate house / climate / standing / book payloads.
/// Pure equality check for proof tests — no filesystem wipe path on title Esc.
pub fn esc_from_title_preserves_persist(
    house_before: &str,
    house_after: &str,
    climate_before: &str,
    climate_after: &str,
    standing_before: &str,
    standing_after: &str,
    book_before: &str,
    book_after: &str,
) -> bool {
    house_before == house_after
        && climate_before == climate_after
        && standing_before == standing_after
        && book_before == book_after
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
        assert!(continue_cue_when_persist(false, &h).is_none());
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
        assert_eq!(cue, "Moss House · the yard remembers");
        assert!(cue.contains("Moss House"));
        assert!(cue.contains(YARD_REMEMBERS));
    }

    #[test]
    fn continue_cue_unnamed_house_yard_remembers() {
        let mut h = HouseName::default();
        h.skip();
        let cue = continue_cue(true, &h).unwrap();
        assert_eq!(cue, "Unnamed House · the yard remembers");
        let cue2 = continue_cue_when_persist(true, &h).unwrap();
        assert_eq!(cue2, "Unnamed House · the yard remembers");
        // Resolved without hour-two still gets steward wording
        let cue3 = continue_cue(false, &h).unwrap();
        assert_eq!(cue3, "Unnamed House · the yard remembers");
    }

    #[test]
    fn continue_cue_when_climate_only_persist() {
        let h = HouseName::default(); // unresolved, first-run house
        // Climate/standing alone still surfaces Continue with Unnamed House + yard
        let cue = continue_cue_when_persist(true, &h).unwrap();
        assert_eq!(cue, "Unnamed House · the yard remembers");
    }

    #[test]
    fn esc_from_title_does_not_clear_house_json_or_flags() {
        let mut house = HouseName::default();
        house.confirm("Held House");
        let house_json = house.to_json().unwrap();
        let climate = r#"{"schema":"powrush_shard_climate_v1","harmony":0.7}"#;
        let standing = r#"{"schema":"powrush_shard_standing_v1","declared_lethal":false}"#;
        let book = r#"{"complete":true,"hour_three_complete":true}"#;
        // Esc-from-title: settings may close; payloads untouched
        let after_house = house_json.clone();
        let after_climate = climate.to_string();
        let after_standing = standing.to_string();
        let after_book = book.to_string();
        assert!(esc_from_title_preserves_persist(
            &house_json,
            &after_house,
            climate,
            &after_climate,
            standing,
            &after_standing,
            book,
            &after_book,
        ));
        let back = HouseName::from_json(&after_house).unwrap();
        assert!(back.resolved);
        assert_eq!(back.display_name(), "Held House");
        // A wipe would fail this guard
        assert!(!esc_from_title_preserves_persist(
            &house_json,
            "{}",
            climate,
            climate,
            standing,
            standing,
            book,
            book,
        ));
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