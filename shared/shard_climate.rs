//! Phase Q — solo shard climate ledger (v23.2.36)
//!
//! One hex remembers thriving vs tired. Same verbs as the lived hour.
//! Visible face stays well speech (Idle / Glowing / Tended / Resting / Stressed).
//! Persist: data/powrush_shard_climate.json
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Native one-hex climate. Ra-Thor may read; it does not write.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShardClimate {
    pub hex_id: String,
    /// 0..1 thriving signal
    pub harmony: f32,
    /// 0..1 tired signal
    pub stress: f32,
    /// Passive recovery rate (not a second HUD)
    pub regen: f32,
    /// Repair-rights pool. Never loot.
    pub reserve_pool: u32,
    pub restored_count: u32,
    /// Tons only after LaneCrate has run
    pub tons_moved: u32,
    pub updated_at: u64,
}

impl Default for ShardClimate {
    fn default() -> Self {
        Self {
            hex_id: "local-hex".into(),
            harmony: 0.55,
            stress: 0.15,
            regen: 0.08,
            reserve_pool: 0,
            restored_count: 0,
            tons_moved: 0,
            updated_at: 0,
        }
    }
}

impl ShardClimate {
    pub fn clamp_fields(&mut self) {
        self.harmony = self.harmony.clamp(0.0, 1.0);
        self.stress = self.stress.clamp(0.0, 1.0);
        self.regen = self.regen.clamp(0.0, 1.0);
    }

    fn touch(&mut self) {
        self.updated_at = self.updated_at.saturating_add(1);
        self.clamp_fields();
    }

    /// Hold-E care tend: stress ↓ harmony ↑ regen ↑
    pub fn on_care_tend(&mut self) {
        self.stress = (self.stress - 0.08).clamp(0.0, 1.0);
        self.harmony = (self.harmony + 0.06).clamp(0.0, 1.0);
        self.regen = (self.regen + 0.03).clamp(0.0, 1.0);
        self.touch();
    }

    /// Tap E on Glowing: satchel already filled; stress ↑ small
    pub fn on_glowing_take(&mut self) {
        self.stress = (self.stress + 0.06).clamp(0.0, 1.0);
        self.harmony = (self.harmony - 0.02).clamp(0.0, 1.0);
        self.touch();
    }

    /// Take attempted on Resting/Stressed: no-take + extra stress
    pub fn on_tired_refuse(&mut self) {
        self.stress = (self.stress + 0.10).clamp(0.0, 1.0);
        self.harmony = (self.harmony - 0.04).clamp(0.0, 1.0);
        self.touch();
    }

    /// R 1 flow: stress ↓; Resting leans Glowing (node side elsewhere)
    pub fn on_flow(&mut self) {
        self.stress = (self.stress - 0.12).clamp(0.0, 1.0);
        self.harmony = (self.harmony + 0.05).clamp(0.0, 1.0);
        self.regen = (self.regen + 0.02).clamp(0.0, 1.0);
        self.touch();
    }

    /// R 2 reserve: reserve_pool ↑; well may cool one tick
    pub fn on_reserve(&mut self) {
        self.reserve_pool = self.reserve_pool.saturating_add(1);
        self.stress = (self.stress - 0.03).clamp(0.0, 1.0);
        self.touch();
    }

    /// MendSpool: restored_count ↑ toward Tended
    pub fn on_mend(&mut self) {
        self.restored_count = self.restored_count.saturating_add(1);
        self.harmony = (self.harmony + 0.04).clamp(0.0, 1.0);
        self.stress = (self.stress - 0.05).clamp(0.0, 1.0);
        self.touch();
    }

    /// LaneCrate: tons_moved ↑
    pub fn on_lane(&mut self) {
        self.tons_moved = self.tons_moved.saturating_add(1);
        self.touch();
    }

    /// Optional slab — not a second HUD.
    pub fn slab_line(&self) -> Option<&'static str> {
        if self.stress >= 0.55 {
            Some("the well is tired")
        } else if self.harmony >= 0.60 && self.stress <= 0.30 {
            Some("the yard is circulating")
        } else {
            None
        }
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
    fn tend_save_load_keeps_stress() {
        let mut c = ShardClimate::default();
        let before = c.stress;
        c.on_care_tend();
        assert!(c.stress < before);
        let stress = c.stress;
        let raw = c.to_json().unwrap();
        let loaded = ShardClimate::from_json(&raw).unwrap();
        assert!((loaded.stress - stress).abs() < 1e-6);
        assert_eq!(loaded.hex_id, "local-hex");
    }

    #[test]
    fn glowing_take_raises_stress() {
        let mut c = ShardClimate::default();
        let before = c.stress;
        c.on_glowing_take();
        assert!(c.stress > before);
    }

    #[test]
    fn tired_refuse_extra_stress() {
        let mut c = ShardClimate {
            stress: 0.4,
            ..Default::default()
        };
        c.on_tired_refuse();
        assert!(c.stress >= 0.5);
    }

    #[test]
    fn flow_lowers_stress() {
        let mut c = ShardClimate {
            stress: 0.7,
            ..Default::default()
        };
        c.on_flow();
        assert!(c.stress < 0.7);
    }

    #[test]
    fn reserve_raises_pool() {
        let mut c = ShardClimate::default();
        c.on_reserve();
        assert_eq!(c.reserve_pool, 1);
        c.on_reserve();
        assert_eq!(c.reserve_pool, 2);
    }

    #[test]
    fn mend_and_lane() {
        let mut c = ShardClimate::default();
        c.on_mend();
        c.on_lane();
        assert_eq!(c.restored_count, 1);
        assert_eq!(c.tons_moved, 1);
    }

    #[test]
    fn slab_lines_are_one_sentence() {
        let tired = ShardClimate {
            stress: 0.7,
            ..Default::default()
        };
        assert_eq!(tired.slab_line(), Some("the well is tired"));
        let good = ShardClimate {
            harmony: 0.7,
            stress: 0.1,
            ..Default::default()
        };
        assert_eq!(good.slab_line(), Some("the yard is circulating"));
    }
}
