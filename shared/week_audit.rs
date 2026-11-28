//! Phase S — week audit preview (v23.2.39)
//!
//! Week score = tons + restored. Local clock only. No kills.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// SAVE-VERSION-1. Stamp this build writes as `"v"`. No `"v"` on disk is v0 (tag-era file).
pub const WEEK_AUDIT_V: u16 = 1;

/// Local week bucket. Ra-Thor may read; it does not write.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeekAudit {
    /// SAVE-VERSION-1. Absent reads as 0 (v0); `from_json` migrates it to `WEEK_AUDIT_V`.
    #[serde(default)] pub v: u16,
    /// Local calendar week key (YYYY-Www or day bucket)
    pub week_id: String,
    pub tons_moved: u32,
    pub restored_count: u32,
    pub updated_at: u64,
}

impl Default for WeekAudit {
    fn default() -> Self {
        Self {
            v: WEEK_AUDIT_V,
            week_id: "local-week".into(),
            tons_moved: 0,
            restored_count: 0,
            updated_at: 0,
        }
    }
}

impl WeekAudit {
    pub fn sync_from_climate(&mut self, tons: u32, restored: u32) {
        self.tons_moved = tons;
        self.restored_count = restored;
        self.updated_at = self.updated_at.saturating_add(1);
    }

    /// One honest yard slab. Not a war HUD.
    /// Deliberately "this week" — the House bill face lives in pause_ledger_face
    /// as "House week" so a stranger can tell bill from yard.
    pub fn slab_line(&self) -> String {
        let (tons, restored) = self.tons_and_restored();
        format!("this week · {tons} tons · {restored} restored")
    }

    /// Score legs only — tons + restored. Never kills, gold, or Market.
    pub fn tons_and_restored(&self) -> (u32, u32) {
        (self.tons_moved, self.restored_count)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str::<Self>(raw)?.migrate()
    }

    /// SAVE-VERSION-1. v0 → v1 is stamp-only: v1 is the v0 fields plus `v`. A newer stamp is refused.
    fn migrate(mut self) -> Result<Self, serde_json::Error> {
        match self.v {
            0 => self.v = 1,
            WEEK_AUDIT_V => {}
            _ => return Err(serde::de::Error::custom("week audit stamp is newer than this build")),
        }
        Ok(self)
    }

    /// SAVE-VERSION-1. True when `raw` carries a stamp newer than this build. Do not write over it.
    pub fn stamp_is_newer(raw: &str) -> bool {
        serde_json::from_str::<serde_json::Value>(raw).ok().and_then(|j| j.get("v")?.as_u64()).is_some_and(|v| v > u64::from(WEEK_AUDIT_V))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slab_names_tons_and_restored() {
        let mut w = WeekAudit::default();
        w.sync_from_climate(3, 2);
        let line = w.slab_line();
        assert!(line.contains("3 tons"));
        assert!(line.contains("2 restored"));
        assert!(line.starts_with("this week"));
        assert_eq!(w.tons_and_restored(), (3, 2));
        let low = line.to_lowercase();
        assert!(!low.contains("kill"));
        assert!(!low.contains("gold"));
        assert!(!low.contains("market"));
        assert!(!low.contains("price"));
        assert!(!low.contains("sell"));
    }

    #[test]
    fn json_roundtrip() {
        let mut w = WeekAudit::default();
        w.sync_from_climate(1, 4);
        let loaded = WeekAudit::from_json(&w.to_json().unwrap()).unwrap();
        assert_eq!(loaded.tons_moved, 1);
        assert_eq!(loaded.restored_count, 4);
    }
}
