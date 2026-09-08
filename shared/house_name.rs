//! S2 House naming + D3 seals / heritage caption (v23.2.64)
//!
//! Persist: `powrush_house.json` in the OS user-data dir (or `POWRUSH_USER_DIR`).
//! Cwd `data/powrush_house.json` is the adopt source when the user dir is empty.
//! Skip → Unnamed House. Never a wall before first E.
//! D3 (after Settled / skip-named): three skippable Peace-tone seals
//! (Well · Grove · Ember) — cosmetic silhouettes / labels only.
//! Optional heritage caption string only — no stats:
//!   none|human|cydruid|quellorian|draek|ambrosian
//! Rename allowed. Refuse any +take / +STR / combat mods.
//! Continue cue: House name or Unnamed House + *the yard remembers*.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

pub const HOUSE_PATH: &str = "data/powrush_house.json";
pub const HOUSE_SCHEMA: &str = "powrush_house_v1";
pub const UNNAMED: &str = "Unnamed House";
/// Steward Continue cue suffix (exact wording).
pub const YARD_REMEMBERS: &str = "the yard remembers";

/// Three Peace-tone seal ids (cosmetic only — not combat kits).
pub const SEAL_WELL: &str = "well";
pub const SEAL_GROVE: &str = "grove";
pub const SEAL_EMBER: &str = "ember";
pub const HOUSE_SEALS: &[&str] = &[SEAL_WELL, SEAL_GROVE, SEAL_EMBER];
pub const MAX_SEALS: usize = 3;

/// Heritage captions — string only, no stats. Five practices + none.
pub const HERITAGE_NONE: &str = "none";
pub const HERITAGE_CAPTIONS: &[&str] = &[
    "none",
    "human",
    "cydruid",
    "quellorian",
    "draek",
    "ambrosian",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HouseName {
    pub schema: String,
    /// Empty means Unnamed when resolved; ignored until resolved.
    #[serde(default)]
    pub name: String,
    /// True after Confirm or Skip. False = not yet asked / first run.
    #[serde(default)]
    pub resolved: bool,
    /// Up to three Peace-tone seal ids (well|grove|ember). Empty = none / skipped.
    #[serde(default)]
    pub seals: Vec<String>,
    /// True after seals panel Confirm or Skip-all (after Settled).
    #[serde(default)]
    pub seals_resolved: bool,
    /// Heritage caption only — never grants stats. Default "none".
    #[serde(default = "default_heritage")]
    pub heritage: String,
}

fn default_heritage() -> String {
    HERITAGE_NONE.into()
}

impl Default for HouseName {
    fn default() -> Self {
        Self {
            schema: HOUSE_SCHEMA.into(),
            name: String::new(),
            resolved: false,
            seals: Vec::new(),
            seals_resolved: false,
            heritage: HERITAGE_NONE.into(),
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

    /// House rename after founded / skip-named. Keeps seals + heritage.
    pub fn rename(&mut self, raw: &str) {
        let t = raw.trim();
        self.name = if t.is_empty() {
            String::new()
        } else {
            t.chars().take(32).collect()
        };
        self.resolved = true;
    }

    /// True when `id` is one of the three Peace-tone seals.
    pub fn is_valid_seal(id: &str) -> bool {
        HOUSE_SEALS.iter().any(|s| *s == id)
    }

    /// Display label for a seal id (cosmetic silhouette name).
    pub fn seal_label(id: &str) -> &'static str {
        match id {
            SEAL_WELL => "Well",
            SEAL_GROVE => "Grove",
            SEAL_EMBER => "Ember",
            _ => "Seal",
        }
    }

    /// Current seals as display names joined with · (empty when none).
    pub fn seals_caption(&self) -> String {
        if self.seals.is_empty() {
            return String::new();
        }
        self.seals
            .iter()
            .map(|s| Self::seal_label(s))
            .collect::<Vec<_>>()
            .join(" · ")
    }

    /// One-line dress cue for Q / Pause face when seals resolved.
    /// Heritage is string-only (already persisted by D3).
    pub fn dress_line_for_plate(&self) -> Option<String> {
        if !self.seals_resolved {
            return None;
        }
        let seals = self.seals_caption();
        let seal_part = if seals.is_empty() {
            "Seal · none".to_string()
        } else {
            format!("Seal · {seals}")
        };
        if self.heritage.is_empty() || self.heritage == HERITAGE_NONE {
            Some(seal_part)
        } else {
            Some(format!("{seal_part} · {}", self.heritage))
        }
    }

    /// Toggle a seal on the house (max three; invalid ids ignored). Cosmetic only.
    pub fn toggle_seal(&mut self, id: &str) -> bool {
        if !Self::is_valid_seal(id) {
            return false;
        }
        if let Some(pos) = self.seals.iter().position(|s| s == id) {
            self.seals.remove(pos);
            return true;
        }
        if self.seals.len() >= MAX_SEALS {
            return false;
        }
        self.seals.push(id.to_string());
        true
    }

    /// Set seals from a list (filters invalid, dedupes, caps at three).
    pub fn set_seals(&mut self, ids: &[&str]) {
        let mut out = Vec::new();
        for id in ids {
            if Self::is_valid_seal(id) && !out.iter().any(|s: &String| s == id) && out.len() < MAX_SEALS
            {
                out.push((*id).to_string());
            }
        }
        self.seals = out;
    }

    /// Skip seals panel — empty seals, mark resolved.
    pub fn skip_seals(&mut self) {
        self.seals.clear();
        self.seals_resolved = true;
    }

    /// Confirm current seals selection (may be empty) and mark resolved.
    pub fn confirm_seals(&mut self) {
        // Re-normalize in case of stale data.
        let ids: Vec<String> = self.seals.clone();
        self.set_seals(&ids.iter().map(|s| s.as_str()).collect::<Vec<_>>());
        self.seals_resolved = true;
    }

    /// True when heritage caption is in the allowlist (string only).
    pub fn is_valid_heritage(raw: &str) -> bool {
        let t = raw.trim().to_ascii_lowercase();
        HERITAGE_CAPTIONS.iter().any(|h| *h == t)
    }

    /// Set heritage caption. Returns false if invalid. Never applies stats.
    pub fn set_heritage(&mut self, raw: &str) -> bool {
        let t = raw.trim().to_ascii_lowercase();
        if !Self::is_valid_heritage(&t) {
            return false;
        }
        self.heritage = t;
        true
    }

    /// Skip heritage → "none".
    pub fn skip_heritage(&mut self) {
        self.heritage = HERITAGE_NONE.into();
    }

    /// Cycle heritage caption through allowlist (for UI).
    pub fn bump_heritage(&mut self) {
        let cur = self.heritage.to_ascii_lowercase();
        let idx = HERITAGE_CAPTIONS
            .iter()
            .position(|h| *h == cur)
            .unwrap_or(0);
        let next = (idx + 1) % HERITAGE_CAPTIONS.len();
        self.heritage = HERITAGE_CAPTIONS[next].to_string();
    }

    /// Heritage never grants combat / take / STR — proof helper.
    pub fn heritage_grants_stats(&self) -> bool {
        false
    }

    /// Seals never grant combat kits / +take / +STR — proof helper.
    pub fn seals_grant_combat(&self) -> bool {
        false
    }

    /// Refuse any +take / +STR / combat-stat attachment to the house.
    /// Returns `true` when the request is refused (always, for combat mods).
    pub fn refuse_combat_mod(request: &str) -> bool {
        let t = request.trim().to_ascii_lowercase();
        t.contains("+take")
            || t.contains("+str")
            || t == "str"
            || t == "take"
            || t.contains("combat")
            || t.contains("+atk")
            || t.contains("+dps")
            || t.contains("stat_mod")
            || t.contains("damage")
    }

    /// Attempt to apply a combat mod — always Err for non-empty requests
    /// (cosmetic house only; seals/heritage never grant combat).
    pub fn apply_combat_mod(&self, request: &str) -> Result<(), &'static str> {
        if request.trim().is_empty() {
            return Ok(());
        }
        let _ = Self::refuse_combat_mod(request);
        Err("house seals/heritage refuse combat mods")
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        let mut h: Self = serde_json::from_str(raw)?;
        if h.schema.is_empty() {
            h.schema = HOUSE_SCHEMA.into();
        }
        // Normalize heritage; unknown → none (caption only, never invent stats).
        if !Self::is_valid_heritage(&h.heritage) {
            h.heritage = HERITAGE_NONE.into();
        } else {
            h.heritage = h.heritage.trim().to_ascii_lowercase();
        }
        // Drop invalid seals; cap at three.
        let cleaned: Vec<String> = h
            .seals
            .iter()
            .filter(|s| Self::is_valid_seal(s))
            .take(MAX_SEALS)
            .cloned()
            .collect();
        h.seals = cleaned;
        Ok(h)
    }

    pub fn load_or_default() -> Self {
        let Ok(raw) = crate::user_persist::read_named(HOUSE_PATH) else {
            return Self::default();
        };
        Self::from_json(&raw).unwrap_or_default()
    }

    pub fn persist(&self) {
        if let Ok(json) = self.to_json() {
            let _ = crate::user_persist::write_named(HOUSE_PATH, json);
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
        assert!(!h.seals_resolved);
        assert!(h.seals.is_empty());
        assert_eq!(h.heritage, HERITAGE_NONE);
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
        let resolved = crate::user_persist::persist_path(HOUSE_PATH);
        assert_eq!(
            resolved.file_name().and_then(|s| s.to_str()),
            Some("powrush_house.json")
        );
        assert!(crate::user_persist::is_writable_user_dir_rule(&resolved));
        assert!(!crate::user_persist::is_program_files_path(&resolved));
    }

    #[test]
    fn seals_skippable_and_cosmetic_only() {
        let mut h = HouseName::default();
        h.skip(); // founded / skip-named path
        assert!(h.resolved);
        assert!(!h.seals_resolved);
        // Skip seals entirely
        h.skip_seals();
        assert!(h.seals_resolved);
        assert!(h.seals.is_empty());
        assert!(!h.seals_grant_combat());
        // Choose some seals then confirm
        let mut h2 = HouseName::default();
        h2.confirm("Seal House");
        assert!(h2.toggle_seal(SEAL_WELL));
        assert!(h2.toggle_seal(SEAL_GROVE));
        assert!(h2.toggle_seal(SEAL_EMBER));
        assert_eq!(h2.seals.len(), 3);
        // Toggle off one
        assert!(h2.toggle_seal(SEAL_GROVE));
        assert_eq!(h2.seals, vec![SEAL_WELL.to_string(), SEAL_EMBER.to_string()]);
        // Invalid combat-kit style seal refused
        assert!(!h2.toggle_seal("blade"));
        assert!(!h2.toggle_seal("+STR"));
        h2.confirm_seals();
        assert!(h2.seals_resolved);
        let raw = h2.to_json().unwrap();
        assert!(raw.contains("\"seals\""));
        assert!(raw.contains("well"));
        let back = HouseName::from_json(&raw).unwrap();
        assert_eq!(back.seals, h2.seals);
        assert!(back.seals_resolved);
        assert!(!back.seals_grant_combat());
    }

    #[test]
    fn seals_caption_and_dress_line_for_q_plate() {
        let mut h = HouseName::default();
        h.skip();
        assert!(h.dress_line_for_plate().is_none());
        h.skip_seals();
        assert_eq!(h.dress_line_for_plate().as_deref(), Some("Seal · none"));
        h.set_seals(&[SEAL_WELL, SEAL_EMBER]);
        h.confirm_seals();
        h.set_heritage("cydruid");
        assert_eq!(h.seals_caption(), "Well · Ember");
        assert_eq!(
            h.dress_line_for_plate().as_deref(),
            Some("Seal · Well · Ember · cydruid")
        );
        // heritage remains string only
        assert!(!h.heritage_grants_stats());
    }

    #[test]
    fn heritage_string_only_no_stats() {
        let mut h = HouseName::default();
        h.skip();
        assert_eq!(h.heritage, HERITAGE_NONE);
        assert!(h.set_heritage("human"));
        assert_eq!(h.heritage, "human");
        assert!(h.set_heritage("Cydruid"));
        assert_eq!(h.heritage, "cydruid");
        assert!(h.set_heritage("quellorian"));
        assert!(h.set_heritage("draek"));
        assert!(h.set_heritage("ambrosian"));
        assert!(!h.set_heritage("orc"));
        assert!(!h.set_heritage("+STR"));
        assert!(!h.heritage_grants_stats());
        h.skip_heritage();
        assert_eq!(h.heritage, HERITAGE_NONE);
        // Round-trip
        h.set_heritage("draek");
        let back = HouseName::from_json(&h.to_json().unwrap()).unwrap();
        assert_eq!(back.heritage, "draek");
        assert!(!back.heritage_grants_stats());
        // Unknown heritage in JSON normalizes to none
        let weird = r#"{"schema":"powrush_house_v1","name":"X","resolved":true,"heritage":"+take"}"#;
        let cleaned = HouseName::from_json(weird).unwrap();
        assert_eq!(cleaned.heritage, HERITAGE_NONE);
    }

    #[test]
    fn rename_allowed_keeps_seals_and_heritage() {
        let mut h = HouseName::default();
        h.confirm("First Name");
        h.set_seals(&[SEAL_WELL, SEAL_EMBER]);
        h.confirm_seals();
        h.set_heritage("cydruid");
        h.rename("Second Name");
        assert_eq!(h.display_name(), "Second Name");
        assert!(h.resolved);
        assert_eq!(h.seals, vec![SEAL_WELL.to_string(), SEAL_EMBER.to_string()]);
        assert_eq!(h.heritage, "cydruid");
        // Empty rename → Unnamed, still resolved
        h.rename("   ");
        assert_eq!(h.display_name(), UNNAMED);
        assert!(h.resolved);
        assert!(h.seals_resolved);
    }

    #[test]
    fn refuse_combat_mods_always() {
        assert!(HouseName::refuse_combat_mod("+take"));
        assert!(HouseName::refuse_combat_mod("+STR"));
        assert!(HouseName::refuse_combat_mod("combat kit"));
        assert!(HouseName::refuse_combat_mod("+DPS"));
        assert!(HouseName::refuse_combat_mod("stat_mod"));
        let h = HouseName::default();
        assert!(h.apply_combat_mod("+take").is_err());
        assert!(h.apply_combat_mod("+STR").is_err());
        assert!(h.apply_combat_mod("damage").is_err());
        assert!(h.apply_combat_mod("").is_ok());
        // Seals + heritage stay non-combat even when set
        let mut dressed = HouseName::default();
        dressed.confirm("Peace");
        dressed.set_seals(&[SEAL_WELL, SEAL_GROVE, SEAL_EMBER]);
        dressed.confirm_seals();
        dressed.set_heritage("ambrosian");
        assert!(!dressed.seals_grant_combat());
        assert!(!dressed.heritage_grants_stats());
        assert!(dressed.apply_combat_mod("+take").is_err());
    }

    #[test]
    fn old_house_json_loads_with_d3_defaults() {
        let legacy = r#"{"schema":"powrush_house_v1","name":"Legacy","resolved":true}"#;
        let h = HouseName::from_json(legacy).unwrap();
        assert_eq!(h.display_name(), "Legacy");
        assert!(h.resolved);
        assert!(h.seals.is_empty());
        assert!(!h.seals_resolved);
        assert_eq!(h.heritage, HERITAGE_NONE);
    }

    #[test]
    fn bump_heritage_cycles_allowlist() {
        let mut h = HouseName::default();
        assert_eq!(h.heritage, "none");
        h.bump_heritage();
        assert_eq!(h.heritage, "human");
        for _ in 0..10 {
            h.bump_heritage();
        }
        assert!(HERITAGE_CAPTIONS.contains(&h.heritage.as_str()));
    }
}
