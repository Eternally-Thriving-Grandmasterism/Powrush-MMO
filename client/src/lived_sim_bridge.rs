/*!
 * Lived Sim Bridge — v23.1.0
 *
 * One JSON tick so simulation/ and Ra-Thor can see the human hour.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::flow_weather::{FlowBand, FlowWeather};
use crate::harvest_feel::SoftRbePool;
use crate::living_practice_loop::SoftPlayerRealm;
use crate::local_human_sim::LocalHumanSim;
use crate::player_lineage::{Lineage, PlayerLineage};

const TICK_PATH: &str = "data/powrush_lived_tick.json";
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

#[derive(Resource, Debug, Default)]
pub struct LivedSimBridge {
    accum: f32,
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
    let path = PathBuf::from(TICK_PATH);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(&tick) {
        let _ = fs::write(path, json);
    }
}
