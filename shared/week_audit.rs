//! Phase S — week audit preview (v23.2.39)
//!
//! Week score = tons + restored. Local clock only. No kills.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Local week bucket. Ra-Thor may read; it does not write.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeekAudit {
    /// Local calendar week key (YYYY-Www or day bucket)
    pub week_id: String,
    pub tons_moved: u32,
    pub restored_count: u32,
    pub updated_at: u64,
}

impl Default for WeekAudit {
    fn default() -> Self {
        Self {
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

    /// One honest slab. Not a war HUD.
    pub fn slab_line(&self) -> String {
        format!(
            "this week · {} tons · {} restored",
            self.tons_moved, self.restored_count
        )
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
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
        assert!(!line.to_lowercase().contains("kill"));
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
