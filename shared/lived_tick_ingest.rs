//! L3 — optional lived-tick ingest for Ra-Thor (v23.2.53)
//!
//! Versioned snapshot the lattice may read later. **Off by default.**
//! Env: `POWRUSH_INGEST=off` (default) · `on` / `1` / `true` to enable.
//! When off, write helpers no-op — default boot never calls a hard write path.
//! Soft-fail I/O only. No Ra-Thor path dep. Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::climate_node::LivedHour;
use crate::house_name::{HouseName, UNNAMED};
use crate::hour_two::HourTwoPack;
use crate::shard_climate::ShardClimate;
use crate::shard_standing::ShardStanding;
use crate::week_audit::WeekAudit;

/// Ra-Thor-facing lived tick. Same path docs already name for lattice read.
pub const LIVED_TICK_INGEST_PATH: &str = "data/powrush_lived_tick.json";
pub const HOUR_TWO_DISK: &str = "data/powrush_hour_two.json";
pub const SCHEMA: &str = "powrush_lived_tick_v1";

/// Env flag. Default off — Mode B offline stays quiet for lattice ingest.
pub fn ingest_enabled() -> bool {
    match std::env::var("POWRUSH_INGEST") {
        Ok(v) => {
            let t = v.trim().to_ascii_lowercase();
            matches!(t.as_str(), "on" | "1" | "true" | "yes")
        }
        Err(_) => false,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HourFlags {
    #[serde(default)]
    pub hour_two_held: bool,
    #[serde(default)]
    pub hour_three_held: bool,
    #[serde(default)]
    pub satchel_count: u32,
    #[serde(default)]
    pub flow: u32,
    #[serde(default)]
    pub reserve: u32,
}

/// Versioned composite tick — house · climate · standing · week · hour flags.
/// Nested `hour` keeps Mode B resume when ingest owns the file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivedTickIngest {
    pub schema: String,
    pub house_id: String,
    pub house_name: String,
    pub climate: ShardClimate,
    pub standing: ShardStanding,
    pub week: WeekAudit,
    pub hour_flags: HourFlags,
    /// Nested LivedHour so Continuity still loads when ingest wrote the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<LivedHour>,
}

impl Default for LivedTickIngest {
    fn default() -> Self {
        Self {
            schema: SCHEMA.into(),
            house_id: "local-hex".into(),
            house_name: UNNAMED.into(),
            climate: ShardClimate::default(),
            standing: ShardStanding::default(),
            week: WeekAudit::default(),
            hour_flags: HourFlags::default(),
            hour: None,
        }
    }
}

impl LivedTickIngest {
    pub fn from_parts(
        house: &HouseName,
        climate: &ShardClimate,
        standing: &ShardStanding,
        week: &WeekAudit,
        hour: &LivedHour,
        hour_two_held: bool,
        hour_three_held: bool,
    ) -> Self {
        let house_id = if climate.hex_id.trim().is_empty() {
            "local-hex".into()
        } else {
            climate.hex_id.clone()
        };
        Self {
            schema: SCHEMA.into(),
            house_id,
            house_name: house.display_name().to_string(),
            climate: climate.clone(),
            standing: standing.clone(),
            week: week.clone(),
            hour_flags: HourFlags {
                hour_two_held,
                hour_three_held,
                satchel_count: hour.satchel.count() as u32,
                flow: hour.allocation.flow,
                reserve: hour.allocation.reserve,
            },
            hour: Some(hour.clone()),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

}

fn soft_hour_flags() -> (bool, bool) {
    let Ok(raw) = fs::read_to_string(HOUR_TWO_DISK) else {
        return (false, false);
    };
    let pack = HourTwoPack::from_json(&raw);
    (pack.complete, pack.hour_three_complete)
}

/// Soft-write lattice tick when ingest is on. No-op when off. Never panics.
pub fn soft_write_if_enabled(
    climate: &ShardClimate,
    standing: &ShardStanding,
    week: &WeekAudit,
    hour: &LivedHour,
) -> bool {
    if !ingest_enabled() {
        return false;
    }
    let house = HouseName::load_or_default();
    let (h2, h3) = soft_hour_flags();
    let tick = LivedTickIngest::from_parts(&house, climate, standing, week, hour, h2, h3);
    soft_write_tick(&tick)
}

/// Soft-write a prepared tick (tests / explicit path). Soft-fail I/O.
pub fn soft_write_tick(tick: &LivedTickIngest) -> bool {
    if let Some(parent) = Path::new(LIVED_TICK_INGEST_PATH).parent() {
        let _ = fs::create_dir_all(parent);
    }
    match tick.to_json() {
        Ok(json) => fs::write(LIVED_TICK_INGEST_PATH, json).is_ok(),
        Err(_) => false,
    }
}

/// Soft-write to an explicit path (unit tests; does not consult the flag).
pub fn soft_write_tick_to(path: &Path, tick: &LivedTickIngest) -> bool {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    match tick.to_json() {
        Ok(json) => fs::write(path, json).is_ok(),
        Err(_) => false,
    }
}

/// Write only when flag is on, to an explicit path (tests for flag gating).
pub fn soft_write_if_enabled_to(
    path: &Path,
    climate: &ShardClimate,
    standing: &ShardStanding,
    week: &WeekAudit,
    hour: &LivedHour,
    house: &HouseName,
    hour_two_held: bool,
    hour_three_held: bool,
) -> bool {
    if !ingest_enabled() {
        return false;
    }
    let tick = LivedTickIngest::from_parts(
        house,
        climate,
        standing,
        week,
        hour,
        hour_two_held,
        hour_three_held,
    );
    soft_write_tick_to(path, &tick)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::house_name::HouseName;
    use std::sync::Mutex;

    // Env mutation is process-global — serialize flag tests.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_ingest_env(val: Option<&str>, f: impl FnOnce()) {
        let _g = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let prev = std::env::var("POWRUSH_INGEST").ok();
        match val {
            Some(v) => std::env::set_var("POWRUSH_INGEST", v),
            None => std::env::remove_var("POWRUSH_INGEST"),
        }
        f();
        match prev {
            Some(v) => std::env::set_var("POWRUSH_INGEST", v),
            None => std::env::remove_var("POWRUSH_INGEST"),
        }
    }

    #[test]
    fn default_flag_is_off() {
        with_ingest_env(None, || {
            assert!(!ingest_enabled());
        });
        with_ingest_env(Some("off"), || {
            assert!(!ingest_enabled());
        });
        with_ingest_env(Some("0"), || {
            assert!(!ingest_enabled());
        });
    }

    #[test]
    fn flag_on_variants() {
        for v in ["on", "1", "true", "TRUE", "yes"] {
            with_ingest_env(Some(v), || {
                assert!(ingest_enabled(), "expected on for {v}");
            });
        }
    }

    #[test]
    fn default_off_writes_nothing() {
        with_ingest_env(None, || {
            let dir = std::env::temp_dir().join(format!(
                "powrush_ingest_off_{}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            let path = dir.join("powrush_lived_tick.json");
            let house = HouseName::default();
            let wrote = soft_write_if_enabled_to(
                &path,
                &ShardClimate::default(),
                &ShardStanding::default(),
                &WeekAudit::default(),
                &LivedHour::new_demo(),
                &house,
                false,
                false,
            );
            assert!(!wrote);
            assert!(!path.exists(), "default off must not create tick file");
            let _ = fs::remove_dir_all(&dir);
        });
    }

    #[test]
    fn when_on_tick_json_round_trips_key_fields() {
        with_ingest_env(Some("on"), || {
            let dir = std::env::temp_dir().join(format!(
                "powrush_ingest_on_{}",
                std::process::id()
            ));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            let path = dir.join("powrush_lived_tick.json");

            let mut house = HouseName::default();
            house.confirm("Keep Yard");
            let mut climate = ShardClimate::default();
            climate.harmony = 0.77;
            climate.stress = 0.22;
            climate.tons_moved = 3;
            climate.restored_count = 2;
            climate.hex_id = "hex-keep".into();
            let mut standing = ShardStanding::default();
            standing.declared_lethal = true;
            standing.peace = 0.4;
            let mut week = WeekAudit::default();
            week.sync_from_climate(3, 2);
            let hour = LivedHour::new_demo();

            let wrote = soft_write_if_enabled_to(
                &path,
                &climate,
                &standing,
                &week,
                &hour,
                &house,
                true,
                false,
            );
            assert!(wrote);
            let raw = fs::read_to_string(&path).expect("tick written");
            let loaded = LivedTickIngest::from_json(&raw).expect("parse tick");
            assert_eq!(loaded.schema, SCHEMA);
            assert_eq!(loaded.house_name, "Keep Yard");
            assert_eq!(loaded.house_id, "hex-keep");
            assert!((loaded.climate.harmony - 0.77).abs() < f32::EPSILON);
            assert!((loaded.climate.stress - 0.22).abs() < f32::EPSILON);
            assert_eq!(loaded.climate.tons_moved, 3);
            assert_eq!(loaded.climate.restored_count, 2);
            assert!(loaded.standing.declared_lethal);
            assert_eq!(loaded.week.tons_moved, 3);
            assert_eq!(loaded.week.restored_count, 2);
            assert!(loaded.hour_flags.hour_two_held);
            assert!(!loaded.hour_flags.hour_three_held);
            assert!(loaded.hour.is_some());
            let _ = fs::remove_dir_all(&dir);
        });
    }

    #[test]
    fn path_matches_lattice_suggestion() {
        assert_eq!(LIVED_TICK_INGEST_PATH, "data/powrush_lived_tick.json");
    }
}
