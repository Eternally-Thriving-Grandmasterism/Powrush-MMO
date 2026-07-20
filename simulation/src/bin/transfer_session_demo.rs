//! Lightweight demo: accumulate live transfer counters without full world stack.
//!
//! ```bash
//! cargo run -p powrush-simulation --bin transfer_session_demo
//! cargo run -p powrush-simulation --bin transfer_session_demo -- --ticks 100 --out /tmp/powrush_live.json
//! ```
//!
//! Output is Ra-Thor `powrush_telemetry_v1` JSON for
//! `reality-thriving-transfer` / `kardashev-orchestration` ingest.
//! Contact: info@Rathor.ai

use powrush_simulation::telemetry::TelemetryCollector;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ticks: u64 = args
        .iter()
        .position(|a| a == "--ticks")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(48);
    let out = args
        .iter()
        .position(|a| a == "--out")
        .and_then(|i| args.get(i + 1))
        .cloned();

    println!("Powrush → Ra-Thor live transfer session demo");
    println!("ticks={} label=harness_live_demo", ticks);

    let mut collector = TelemetryCollector::new("harness_live_demo");
    for tick in 1..=ticks {
        let mercy = 0.9 + ((tick % 7) as f32) * 0.05;
        let participants = if tick % 5 == 0 { 4 } else { 1 };
        let epiphany = if tick % 11 == 0 { 1 } else { 0 };
        let harvest = if tick % 3 == 0 { 2 } else { 0 };
        let err = tick % 29 == 0;
        collector.record_tick_result(tick, mercy, participants, epiphany, harvest, err);
    }

    let json = collector
        .export_transfer_json()
        .expect("export powrush_telemetry_v1");
    println!("{}", json);

    if let Some(path) = out {
        collector
            .write_transfer_json_to(&path)
            .expect("write transfer json");
        println!("wrote {}", path);
    }

    let t = collector.transfer_telemetry();
    println!(
        "summary: hours={:.4} rbe_q={:.3} peaceful={:.3} collab={} abundance={:.3}",
        t.gameplay_hours,
        t.rbe_decision_quality_avg,
        t.peaceful_resolution_rate,
        t.collaboration_events,
        t.abundance_velocity_signals
    );
}
