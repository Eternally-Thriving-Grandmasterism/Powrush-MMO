/*!
 * Lived Sim Bridge — v23.1.0
 *
 * Discrete action lines append to `data/powrush_lived_events.jsonl`.
 * Session persist (`LivedHour` / ingest overlay) owns
 * `data/powrush_lived_tick.json` via LivedHourBind::persist.
 * Do not 1Hz-dump a different telemetry schema onto that path —
 * Continue cannot resume it.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use serde::Serialize;
use std::io::Write;

use shared::climate_node::LivedHour;
use shared::lived_tick_ingest::LivedTickIngest;

const TICK_PATH: &str = "data/powrush_lived_tick.json";
const EVENTS_PATH: &str = "data/powrush_lived_events.jsonl";
const PERIOD: f32 = 1.0;

#[cfg(test)]
#[derive(Serialize)]
struct LivedTick {
    schema: &'static str,
    lineage_classic: &'static str,
    lineage_sim: &'static str,
    vitality: f32,
    harmony: f32,
    joy: f32,
    harvests: u32,
    tends: u32,
    realm: Option<u8>,
    flow_band: &'static str,
    flow_chain: f32,
    inhaling: bool,
    pocket: u32,
    first_harvest_lived: bool,
    elapsed: f64,
}

#[derive(Serialize)]
struct LivedEvent {
    schema: &'static str,
    session_id: u64,
    tick: f64,
    action: &'static str,
    outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    valence: Option<f32>,
}

/// Session persist owns `powrush_lived_tick.json`. A 1Hz sim dump is a
/// different schema (`LivedTick` without `nodes`/`satchel`/`allocation`)
/// and Continue falls back to `LivedHour::new_demo()`.
pub fn sim_telemetry_may_overwrite_session_tick() -> bool {
    false
}

/// Resume path used by `LivedHourBind::load_or_demo` — hour blob or ingest overlay.
pub fn resumable_hour_from_tick_raw(raw: &str) -> Option<LivedHour> {
    if let Ok(tick) = LivedTickIngest::from_json(raw) {
        if let Some(hour) = tick.hour {
            return Some(hour);
        }
    }
    LivedHour::from_json(raw).ok()
}

#[derive(Resource, Debug)]
pub struct LivedSimBridge {
    accum: f32,
    pub session_id: u64,
}

impl Default for LivedSimBridge {
    fn default() -> Self {
        let session_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(1);
        Self {
            accum: 0.0,
            session_id,
        }
    }
}

/// Append one discrete human action for Ra-Thor / lattice read (off by default ingest).
pub fn emit_lived_event(
    session_id: u64,
    tick: f64,
    action: &'static str,
    outcome: impl Into<String>,
    valence: Option<f32>,
) {
    let event = LivedEvent {
        schema: "powrush_lived_event_v1",
        session_id,
        tick,
        action,
        outcome: outcome.into(),
        valence,
    };
    let Ok(line) = serde_json::to_string(&event) else {
        return;
    };
    let path = shared::user_persist::persist_path(EVENTS_PATH);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(file, "{line}");
    }
}

/// Peace E take/tend → lived-hour satchel + climate persist + discrete event.
pub fn sync_lived_hour_use(
    bind: Option<&mut crate::lived_hour_bind::LivedHourBind>,
    bridge: Option<&LivedSimBridge>,
    now: f64,
    action: &'static str,
    valence: Option<f32>,
) {
    let Some(bind) = bind else {
        return;
    };
    let result = bind.tend_nearest();
    let (outcome, emit_valence) = match &result {
        shared::climate_node::TendResult::Taken { item } => (
            format!("tended node {}", item.node_id),
            valence,
        ),
        shared::climate_node::TendResult::NoTake { reason } => ((*reason).to_string(), None),
    };
    if let Some(bridge) = bridge {
        emit_lived_event(bridge.session_id, now, action, outcome, emit_valence);
    }
}

pub struct LivedSimBridgePlugin;

impl Plugin for LivedSimBridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivedSimBridge>()
            .add_systems(Update, write_lived_tick);
    }
}

fn write_lived_tick(time: Res<Time>, mut bridge: ResMut<LivedSimBridge>) {
    bridge.accum += time.delta_seconds();
    if bridge.accum < PERIOD {
        return;
    }
    bridge.accum = 0.0;
    // Continuity: LivedHourBind::persist owns TICK_PATH.
    // Discrete take / tend / allocate already append EVENTS_PATH.
    if sim_telemetry_may_overwrite_session_tick() {
        return;
    }
    let _ = TICK_PATH;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_telemetry() -> String {
        serde_json::to_string_pretty(&LivedTick {
            schema: "powrush_lived_tick_v1",
            lineage_classic: "Human",
            lineage_sim: "Terran",
            vitality: 1.2,
            harmony: 0.4,
            joy: 0.1,
            harvests: 1,
            tends: 1,
            realm: Some(0),
            flow_band: "flow",
            flow_chain: 0.0,
            inhaling: false,
            pocket: 0,
            first_harvest_lived: true,
            elapsed: 12.0,
        })
        .unwrap()
    }

    #[test]
    fn event_schema_carries_session_action_outcome_valence() {
        let event = LivedEvent {
            schema: "powrush_lived_event_v1",
            session_id: 42,
            tick: 1.5,
            action: "take",
            outcome: "tended node 1".into(),
            valence: Some(1.2),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"session_id\":42"));
        assert!(json.contains("\"action\":\"take\""));
        assert!(json.contains("\"outcome\":\"tended node 1\""));
        assert!(json.contains("\"valence\":1.2"));
    }

    #[test]
    fn one_hz_sim_tick_must_not_clobber_session_persist() {
        assert!(!sim_telemetry_may_overwrite_session_tick());
        assert_eq!(TICK_PATH, crate::lived_hour_bind::LIVED_TICK_PATH);
        assert_eq!(TICK_PATH, shared::lived_tick_ingest::LIVED_TICK_INGEST_PATH);
    }

    #[test]
    fn lived_hour_blob_resumes_satchel() {
        let mut hour = LivedHour::new_demo();
        assert!(matches!(
            hour.tend(1),
            shared::climate_node::TendResult::Taken { .. }
        ));
        assert_eq!(hour.satchel.count(), 1);
        let json = hour.to_json().unwrap();
        let loaded = resumable_hour_from_tick_raw(&json).expect("hour blob");
        assert_eq!(loaded.satchel.count(), 1);
        assert_eq!(loaded.allocation.flow, 0);
    }

    #[test]
    fn sim_telemetry_is_not_resumable_and_would_drop_satchel() {
        let telemetry = sample_telemetry();
        assert!(
            resumable_hour_from_tick_raw(&telemetry).is_none(),
            "1Hz LivedTick dump must not parse as Continuity"
        );
        assert!(LivedHour::from_json(&telemetry).is_err());
    }
}
