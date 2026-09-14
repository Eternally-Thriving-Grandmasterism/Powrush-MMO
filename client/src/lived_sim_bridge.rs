/*!
 * Lived Sim Bridge — v23.1.0
 *
 * One JSON tick so simulation/ and Ra-Thor can see the human hour.
 * Discrete action lines append beside the aggregate tick.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use serde::Serialize;
use std::io::Write;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::flow_weather::{FlowBand, FlowWeather};
use crate::harvest_feel::SoftRbePool;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::local_human_sim::LocalHumanSim;
use crate::player_lineage::{Lineage, PlayerLineage};

const TICK_PATH: &str = "data/powrush_lived_tick.json";
const EVENTS_PATH: &str = "data/powrush_lived_events.jsonl";
const PERIOD: f32 = 1.0;

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

fn sim_alias(lineage: Lineage) -> &'static str {
    match lineage {
        Lineage::Human => "Terran",
        Lineage::Cydruid => "Verdant",
        Lineage::Quellorian => "Harmonic",
        Lineage::Draek => "Voidfarer",
        Lineage::Ambrosian => "Synthetic",
    }
}

fn band_name(band: FlowBand) -> &'static str {
    match band {
        FlowBand::Rise => "rise",
        FlowBand::Flow => "flow",
        FlowBand::Boredom => "boredom",
        FlowBand::Anxiety => "anxiety",
    }
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

/// Append one discrete human action for lattice read (ingest stays opt-in elsewhere).
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

/// Tap-E take → lived-hour satchel + climate persist + discrete event.
pub fn sync_lived_hour_take(
    bind: Option<&mut crate::lived_hour_bind::LivedHourBind>,
    bridge: Option<&LivedSimBridge>,
    now: f64,
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
        emit_lived_event(bridge.session_id, now, "take", outcome, emit_valence);
    }
}

/// Hold-E care tend → climate/standing persist + discrete event (no satchel take).
pub fn sync_lived_hour_tend(
    bind: Option<&mut crate::lived_hour_bind::LivedHourBind>,
    bridge: Option<&LivedSimBridge>,
    now: f64,
    valence: Option<f32>,
) {
    let Some(bind) = bind else {
        return;
    };
    bind.care_tend();
    bind.last_line = "care tend — the node breathes".to_string();
    if let Some(bridge) = bridge {
        emit_lived_event(
            bridge.session_id,
            now,
            "tend",
            bind.last_line.clone(),
            valence,
        );
    }
}

pub struct LivedSimBridgePlugin;

impl Plugin for LivedSimBridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LivedSimBridge>()
            .add_systems(Update, write_lived_tick);
    }
}

fn write_lived_tick(
    time: Res<Time>,
    mut bridge: ResMut<LivedSimBridge>,
    pool: Res<SoftRbePool>,
    lineage: Res<PlayerLineage>,
    realm: Res<SoftPlayerRealm>,
    weather: Res<FlowWeather>,
    harvest: Res<FirstHarvestEpiphany>,
    sim: Res<LocalHumanSim>,
) {
    bridge.accum += time.delta_seconds();
    if bridge.accum < PERIOD {
        return;
    }
    bridge.accum = 0.0;
    let now = time.elapsed_seconds_f64();
    let tick = LivedTick {
        schema: "powrush_lived_tick_v1",
        lineage_classic: lineage.current.name(),
        lineage_sim: sim_alias(lineage.current),
        vitality: pool.vitality,
        harmony: pool.harmony,
        joy: pool.joy,
        harvests: harvest.harvests_this_session.max(pool.harvests),
        tends: harvest.tends_this_session.max(pool.tends),
        realm: realm.current,
        flow_band: band_name(weather.band),
        flow_chain: weather.chain,
        inhaling: weather.inhaling(now),
        pocket: sim.pocket,
        first_harvest_lived: harvest.first_harvest_lived,
        elapsed: now,
    };
    if let Ok(json) = serde_json::to_string_pretty(&tick) {
        let _ = shared::user_persist::write_named(TICK_PATH, json);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
