//! S2 House naming + D3 seals / heritage caption (v23.2.64)
//!
//! Persist: `powrush_house.json` in the OS user-data dir (or `POWRUSH_USER_DIR`).
//! Cwd `data/powrush_house.json` is the adopt source when the user dir is empty.
//! Schema stays `powrush_house_v1`. `seed` defaults to 0 on old blobs.
//! Last-good save: `.tmp` rename over the live file; `.bak` only from a parseable house.
//! Skip → Unnamed House. Never a wall before first E.
//! D3 (after Settled / skip-named): three skippable Peace-tone seals
//! (Well · Grove · Ember) — cosmetic silhouettes / labels only.
//! Optional heritage caption string only — no stats:
//!   none|human|cydruid|quellorian|draek|ambrosian
//! Rename allowed. Refuse any +take / +STR / combat mods.
//! Continue cue: House name or Unnamed House + *the yard remembers*.
//! Contact: info@Rathor.ai

pub mod name_rite;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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
    /// Local name-rite seed. Old `powrush_house_v1` blobs omit it and load as 0.
    #[serde(default)]
    pub seed: u64,
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
            seed: 0,
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
        Self::load_from_path(&crate::user_persist::persist_path(HOUSE_PATH))
    }

    /// Read the live house file. Missing or unparseable live bytes fall back to `.bak`.
    fn load_from_path(path: &Path) -> Self {
        if let Some(house) = read_valid_house(path) {
            return house;
        }
        read_valid_house(&sibling_path(path, ".bak")).unwrap_or_default()
    }

    /// Save this house. `&self` stays so existing callers are unchanged.
    ///
    /// When `self.seed` is 0 the stored seed is reused in order: non-zero seed
    /// on a parsed live file, else non-zero seed on `.bak` (corrupt or missing
    /// live file included), else one new local mint. The mint is written into
    /// the blob. A live file that does not parse is never copied over `.bak`.
    pub fn persist(&self) {
        self.persist_to_path(&crate::user_persist::persist_path(HOUSE_PATH));
    }

    fn persist_to_path(&self, path: &Path) {
        let mut saving = self.clone();
        saving.seed = self.effective_seed(path);
        let Ok(json) = saving.to_json() else {
            return;
        };
        let _ = write_last_good(path, &json);
    }

    fn effective_seed(&self, path: &Path) -> u64 {
        if self.seed != 0 {
            return self.seed;
        }
        if let Some(seed) = nonzero_seed_on(path) {
            return seed;
        }
        if let Some(seed) = nonzero_seed_on(&sibling_path(path, ".bak")) {
            return seed;
        }
        mint_stable_local_seed(path)
    }
}

fn sibling_path(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(suffix);
    PathBuf::from(name)
}

fn read_valid_house(path: &Path) -> Option<HouseName> {
    let raw = fs::read_to_string(path).ok()?;
    HouseName::from_json(&raw).ok()
}

fn nonzero_seed_on(path: &Path) -> Option<u64> {
    let house = read_valid_house(path)?;
    if house.seed == 0 {
        None
    } else {
        Some(house.seed)
    }
}

/// `.tmp` then rename. Copy live → `.bak` only when live parses as `HouseName`.
fn write_last_good(path: &Path, json: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    if read_valid_house(path).is_some() {
        fs::copy(path, sibling_path(path, ".bak"))?;
    }
    let tmp = sibling_path(path, ".tmp");
    fs::write(&tmp, json)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

fn mint_stable_local_seed(live: &Path) -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    let disk = fnv1a64(&local_persist_bytes(live));
    let mut hasher = RandomState::new().build_hasher();
    disk.hash(&mut hasher);
    nonzero_seed(hasher.finish() ^ disk)
}

fn nonzero_seed(raw: u64) -> u64 {
    if raw == 0 {
        1
    } else {
        raw
    }
}

/// FNV-1a 64-bit. Inline so the house file stays std-only.
fn fnv1a64(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;
    let mut hash = OFFSET;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

fn local_persist_bytes(live: &Path) -> Vec<u8> {
    let mut out = Vec::new();
    let Some(dir) = live.parent().filter(|p| !p.as_os_str().is_empty()) else {
        return out;
    };
    let Ok(rd) = fs::read_dir(dir) else {
        return out;
    };
    let mut files: Vec<PathBuf> = rd
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();
    files.sort();
    for path in files {
        if let Ok(bytes) = fs::read(&path) {
            out.extend_from_slice(&bytes);
        }
    }
    out
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
        assert_eq!(h.seed, 0);
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

    fn scratch(tag: &str) -> std::path::PathBuf {
        use std::sync::atomic::{AtomicU64, Ordering};
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "powrush-name-rite-seed-{}-{}-{tag}",
            std::process::id(),
            n
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    #[test]
    fn old_v1_blob_without_seed_loads_zero() {
        assert_eq!(HOUSE_SCHEMA, "powrush_house_v1");
        assert_eq!(HouseName::default().seed, 0);
        let legacy = r#"{"schema":"powrush_house_v1","name":"Legacy","resolved":true}"#;
        let parsed = HouseName::from_json(legacy).unwrap();
        assert_eq!(parsed.seed, 0);
        assert_eq!(parsed.schema, HOUSE_SCHEMA);
        assert_eq!(parsed.display_name(), "Legacy");
        let raw = parsed.to_json().unwrap();
        assert!(raw.contains("powrush_house_v1"));
        assert!(!raw.contains("powrush_house_v2"));

        let dir = scratch("legacy");
        let path = dir.join("powrush_house.json");
        std::fs::write(&path, legacy).unwrap();
        let loaded = HouseName::load_from_path(&path);
        assert_eq!(loaded.seed, 0);
        assert_eq!(loaded.display_name(), "Legacy");
        assert!(loaded.resolved);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn first_persist_mints_nonzero_seed_and_later_zero_keeps_it() {
        let dir = scratch("keep");
        let path = dir.join("powrush_house.json");
        let mut house = HouseName::default();
        house.skip();
        assert_eq!(house.seed, 0);
        house.persist_to_path(&path);
        let first = HouseName::load_from_path(&path);
        assert_ne!(first.seed, 0);
        assert_eq!(first.schema, "powrush_house_v1");
        assert_eq!(first.display_name(), UNNAMED);

        let bak = super::sibling_path(&path, ".bak");
        let mut decoy = first.clone();
        decoy.seed = first.seed.wrapping_add(1).max(1);
        if decoy.seed == first.seed {
            decoy.seed = 3;
        }
        std::fs::write(&bak, decoy.to_json().unwrap()).unwrap();

        let mut again = HouseName::default();
        again.skip();
        assert_eq!(again.seed, 0);
        again.persist_to_path(&path);
        let second = HouseName::load_from_path(&path);
        assert_eq!(second.seed, first.seed);
        assert_ne!(second.seed, decoy.seed);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn corrupt_main_falls_back_to_bak_and_save_keeps_bak() {
        let dir = scratch("corrupt");
        let path = dir.join("powrush_house.json");
        let mut good = HouseName::default();
        good.confirm("Kept");
        good.seed = 42;
        let good_json = good.to_json().unwrap();
        let bak = super::sibling_path(&path, ".bak");
        std::fs::write(&bak, &good_json).unwrap();
        let bad = b"{{this-is-not-a-house";
        std::fs::write(&path, bad).unwrap();

        let loaded = HouseName::load_from_path(&path);
        assert_eq!(loaded.display_name(), "Kept");
        assert_eq!(loaded.seed, 42);
        assert_eq!(loaded.schema, HOUSE_SCHEMA);

        loaded.persist_to_path(&path);

        let bak_bytes = std::fs::read(&bak).unwrap();
        assert_eq!(bak_bytes, good_json.as_bytes());
        assert_ne!(bak_bytes, bad);
        let main = std::fs::read(&path).unwrap();
        assert_ne!(main, bad);
        let main_house = HouseName::from_json(std::str::from_utf8(&main).unwrap()).unwrap();
        assert_eq!(main_house.display_name(), "Kept");
        assert_eq!(main_house.seed, 42);

        let missing = dir.join("powrush_house_missing.json");
        std::fs::write(super::sibling_path(&missing, ".bak"), &good_json).unwrap();
        let from_missing = HouseName::load_from_path(&missing);
        assert_eq!(from_missing.seed, 42);
        assert_eq!(from_missing.display_name(), "Kept");
        let mut fresh = HouseName::default();
        fresh.skip();
        assert_eq!(fresh.seed, 0);
        fresh.persist_to_path(&missing);
        assert_eq!(HouseName::load_from_path(&missing).seed, 42);
        assert_eq!(
            std::fs::read(super::sibling_path(&missing, ".bak")).unwrap(),
            good_json.as_bytes()
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn minted_seed_is_never_zero() {
        assert_eq!(super::nonzero_seed(0), 1);
        assert_eq!(super::nonzero_seed(1), 1);
        assert_eq!(super::nonzero_seed(u64::MAX), u64::MAX);
        let dir = scratch("mint");
        let path = dir.join("powrush_house.json");
        for _ in 0..32 {
            assert_ne!(super::mint_stable_local_seed(&path), 0);
        }
        let mut house = HouseName::default();
        house.confirm("Mint House");
        assert_eq!(house.seed, 0);
        house.persist_to_path(&path);
        let loaded = HouseName::load_from_path(&path);
        assert_ne!(loaded.seed, 0);
        assert_eq!(loaded.schema, HOUSE_SCHEMA);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn corrupt_main_load_and_persist_keep_bak_seed() {
        let dir = scratch("bak-seed");
        let path = dir.join("powrush_house.json");
        const BAK_SEED: u64 = 0x00C0_FFEE;
        let mut bak_house = HouseName::default();
        bak_house.confirm("Bak House");
        bak_house.seed = BAK_SEED;
        let bak_json = bak_house.to_json().unwrap();
        let bak = super::sibling_path(&path, ".bak");
        std::fs::write(&bak, &bak_json).unwrap();
        std::fs::write(&path, b"not-json-at-all").unwrap();

        let loaded = HouseName::load_from_path(&path);
        assert_eq!(loaded.seed, BAK_SEED);
        assert_eq!(loaded.display_name(), "Bak House");

        let mut fresh = HouseName::default();
        fresh.confirm("Later Save");
        assert_eq!(fresh.seed, 0);
        fresh.persist_to_path(&path);

        assert_eq!(std::fs::read(&bak).unwrap(), bak_json.as_bytes());
        let again = HouseName::load_from_path(&path);
        assert_eq!(again.seed, BAK_SEED);
        assert_eq!(again.display_name(), "Later Save");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn parsed_main_seed_zero_reuses_bak_seed() {
        let dir = scratch("main-zero");
        let path = dir.join("powrush_house.json");
        let mut live = HouseName::default();
        live.confirm("Live");
        assert_eq!(live.seed, 0);
        std::fs::write(&path, live.to_json().unwrap()).unwrap();
        let mut bak_house = HouseName::default();
        bak_house.confirm("Backup");
        bak_house.seed = 77;
        std::fs::write(super::sibling_path(&path, ".bak"), bak_house.to_json().unwrap()).unwrap();

        let mut fresh = HouseName::default();
        fresh.confirm("Live");
        assert_eq!(fresh.seed, 0);
        fresh.persist_to_path(&path);
        let loaded = HouseName::load_from_path(&path);
        assert_eq!(loaded.seed, 77);
        assert_eq!(loaded.display_name(), "Live");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
