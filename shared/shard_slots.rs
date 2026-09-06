//! U — divergent shard save-slots (v23.2.43)
//!
//! Two hex files can diverge: thrive vs poor. Local only.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::shard_climate::ShardClimate;
use crate::shard_standing::ShardStanding;
use crate::week_audit::WeekAudit;

/// Named local shard slot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShardSlot {
    pub slot_id: String,
    pub display_name: String,
    pub climate: ShardClimate,
    pub standing: ShardStanding,
    pub week: WeekAudit,
}

impl ShardSlot {
    pub fn thrive(slot_id: &str) -> Self {
        let mut climate = ShardClimate {
            hex_id: slot_id.into(),
            harmony: 0.82,
            stress: 0.08,
            regen: 0.18,
            ..Default::default()
        };
        climate.hex_id = slot_id.into();
        let mut standing = ShardStanding {
            hex_id: slot_id.into(),
            peace: 0.85,
            harmony: 0.82,
            consumption: 0.10,
            steward: 0.70,
            ..Default::default()
        };
        standing.hex_id = slot_id.into();
        let mut week = WeekAudit::default();
        week.sync_from_climate(climate.tons_moved, climate.restored_count);
        Self {
            slot_id: slot_id.into(),
            display_name: "thrive".into(),
            climate,
            standing,
            week,
        }
    }

    pub fn poor(slot_id: &str) -> Self {
        let mut climate = ShardClimate {
            hex_id: slot_id.into(),
            harmony: 0.28,
            stress: 0.72,
            regen: 0.03,
            ..Default::default()
        };
        climate.hex_id = slot_id.into();
        let mut standing = ShardStanding {
            hex_id: slot_id.into(),
            peace: 0.32,
            harmony: 0.28,
            consumption: 0.68,
            steward: 0.22,
            ..Default::default()
        };
        standing.hex_id = slot_id.into();
        let mut week = WeekAudit::default();
        week.sync_from_climate(climate.tons_moved, climate.restored_count);
        Self {
            slot_id: slot_id.into(),
            display_name: "poor".into(),
            climate,
            standing,
            week,
        }
    }

    pub fn path_climate(&self) -> String {
        format!("data/shards/{}/powrush_shard_climate.json", self.slot_id)
    }

    pub fn path_standing(&self) -> String {
        format!("data/shards/{}/powrush_shard_standing.json", self.slot_id)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }
}

/// Two divergent local worlds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShardBank {
    pub active: String,
    pub thrive: ShardSlot,
    pub poor: ShardSlot,
}

impl Default for ShardBank {
    fn default() -> Self {
        Self {
            active: "thrive".into(),
            thrive: ShardSlot::thrive("hex-thrive"),
            poor: ShardSlot::poor("hex-poor"),
        }
    }
}

impl ShardBank {
    pub fn active_slot(&self) -> &ShardSlot {
        if self.active == "poor" {
            &self.poor
        } else {
            &self.thrive
        }
    }

    pub fn select(&mut self, which: &str) {
        if which == "poor" || which == "thrive" {
            self.active = which.into();
        }
    }

    pub fn diverge_line(&self) -> String {
        format!(
            "shards · thrive stress {:.2} · poor stress {:.2}",
            self.thrive.climate.stress, self.poor.climate.stress
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
    fn two_slots_diverge() {
        let bank = ShardBank::default();
        assert!(bank.thrive.climate.harmony > bank.poor.climate.harmony);
        assert!(bank.poor.climate.stress > bank.thrive.climate.stress);
        assert_ne!(bank.thrive.path_climate(), bank.poor.path_climate());
    }

    #[test]
    fn select_poor_then_thrive() {
        let mut bank = ShardBank::default();
        bank.select("poor");
        assert_eq!(bank.active_slot().display_name, "poor");
        bank.select("thrive");
        assert_eq!(bank.active_slot().display_name, "thrive");
    }

    #[test]
    fn json_roundtrip() {
        let bank = ShardBank::default();
        let loaded = ShardBank::from_json(&bank.to_json().unwrap()).unwrap();
        assert_eq!(loaded.thrive.slot_id, "hex-thrive");
        assert_eq!(loaded.poor.slot_id, "hex-poor");
    }
}
