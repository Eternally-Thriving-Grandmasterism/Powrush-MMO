//! powrush-shard — parked hex binary (F7 / v23.2.56)
//!
//! Offline dry-apply + snapshot write. `--listen` accepted but parked.
//! Soft cap 32 Houses (shared). Not wired into powrush-client.
//! Contact: info@Rathor.ai

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use shared::hex_protocol::Snapshot;
use shared::hex_shard_apply::{
    apply_jsonl, fixture_snapshot, listen_is_parked, load_snapshot_json, soft_cap_houses,
    write_snapshot_json, ApplyOutcome, ShardLedger, LISTEN_PARKED_MSG, SNAPSHOT_FILE,
};

#[derive(Debug, Default)]
struct Args {
    hex: String,
    listen: Option<String>,
    data: PathBuf,
    dry_apply: Option<PathBuf>,
    help: bool,
}

fn print_help() {
    eprintln!(
        "\
powrush-shard — parked hex ledger binary (not default door)

USAGE:
  powrush-shard --hex <id> --data <dir> [--dry-apply <events.jsonl>] [--listen <addr>]

FLAGS:
  --hex <id>              Hex id (e.g. hex_local_0)
  --data <dir>            Data directory (ledger_snapshot.json lives here)
  --dry-apply <jsonl>     Apply JSONL verb events offline; write snapshot
  --listen <addr>         Accepted but PARKED / not enabled (no WS bind)
  -h, --help              Show this help

Soft cap: {} Houses per hex (documented; seat beyond soft-rejects).
Listen: parked — prints clear notice; does not bind a port.
Default client door remains: cargo run -p powrush-client (no shard dep).",
        soft_cap_houses()
    );
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut args = Args {
        hex: String::new(),
        listen: None,
        data: PathBuf::from("./data/hex_local_0"),
        dry_apply: None,
        help: false,
    };
    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "-h" | "--help" => {
                args.help = true;
                i += 1;
            }
            "--hex" => {
                i += 1;
                args.hex = argv
                    .get(i)
                    .cloned()
                    .ok_or_else(|| "--hex needs a value".into())?;
                i += 1;
            }
            "--listen" => {
                i += 1;
                args.listen = Some(
                    argv.get(i)
                        .cloned()
                        .ok_or_else(|| "--listen needs addr".into())?,
                );
                i += 1;
            }
            "--data" => {
                i += 1;
                args.data = PathBuf::from(
                    argv.get(i)
                        .cloned()
                        .ok_or_else(|| "--data needs a path".into())?,
                );
                i += 1;
            }
            "--dry-apply" => {
                i += 1;
                args.dry_apply = Some(PathBuf::from(
                    argv.get(i)
                        .cloned()
                        .ok_or_else(|| "--dry-apply needs a path".into())?,
                ));
                i += 1;
            }
            other => return Err(format!("unknown arg: {other}")),
        }
    }
    Ok(args)
}

fn load_or_fresh(hex: &str, data: &Path) -> ShardLedger {
    let path = data.join(SNAPSHOT_FILE);
    if path.is_file() {
        match fs::read_to_string(&path) {
            Ok(raw) => match load_snapshot_json(&raw) {
                Ok(snap) => {
                    eprintln!("powrush-shard: loaded snapshot {}", path.display());
                    return ShardLedger::from_snapshot(hex, snap, 0);
                }
                Err(e) => eprintln!("powrush-shard: snapshot parse fail ({e}); using fresh"),
            },
            Err(e) => eprintln!("powrush-shard: snapshot read fail ({e}); using fresh"),
        }
    }
    ShardLedger::from_snapshot(hex, fixture_snapshot(hex), 0)
}

fn write_snapshot(data: &Path, snap: &Snapshot) -> Result<(), String> {
    fs::create_dir_all(data).map_err(|e| e.to_string())?;
    let path = data.join(SNAPSHOT_FILE);
    let raw = write_snapshot_json(snap).map_err(|e| e.to_string())?;
    fs::write(&path, raw).map_err(|e| e.to_string())?;
    eprintln!("powrush-shard: wrote {}", path.display());
    Ok(())
}

fn main() -> ExitCode {
    let argv: Vec<String> = env::args().collect();
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("powrush-shard: {e}");
            print_help();
            return ExitCode::from(2);
        }
    };
    if args.help {
        print_help();
        return ExitCode::SUCCESS;
    }
    if args.hex.is_empty() {
        eprintln!("powrush-shard: --hex is required");
        print_help();
        return ExitCode::from(2);
    }

    debug_assert!(listen_is_parked());

    if let Some(addr) = &args.listen {
        eprintln!("{LISTEN_PARKED_MSG}");
        eprintln!("powrush-shard: requested listen addr was {addr} (ignored; no bind)");
        if args.dry_apply.is_none() {
            eprintln!(
                "powrush-shard: tip — pass --dry-apply <events.jsonl> for offline apply + snapshot write"
            );
            return ExitCode::SUCCESS;
        }
    }

    let Some(jsonl_path) = &args.dry_apply else {
        eprintln!("powrush-shard: nothing to do — pass --dry-apply <events.jsonl>");
        eprintln!(
            "powrush-shard: soft cap {} houses; listen parked; not default client door",
            soft_cap_houses()
        );
        return ExitCode::SUCCESS;
    };

    let mut ledger = load_or_fresh(&args.hex, &args.data);
    let jsonl = match fs::read_to_string(jsonl_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("powrush-shard: cannot read {}: {e}", jsonl_path.display());
            return ExitCode::from(1);
        }
    };

    let outcomes = apply_jsonl(&mut ledger, &jsonl);
    let mut applied = 0u32;
    let mut rejected = 0u32;
    for o in &outcomes {
        match o {
            ApplyOutcome::Applied { seq, op } => {
                applied += 1;
                eprintln!("apply ok seq={seq} op={op}");
            }
            ApplyOutcome::Rejected { code, reason } => {
                rejected += 1;
                eprintln!(
                    "apply reject {} {}",
                    code.as_str(),
                    reason.as_deref().unwrap_or("")
                );
            }
        }
    }

    if let Err(e) = write_snapshot(&args.data, &ledger.snapshot) {
        eprintln!("powrush-shard: write failed: {e}");
        return ExitCode::from(1);
    }

    eprintln!(
        "powrush-shard: done hex={} applied={applied} rejected={rejected} last_seq={} soft_cap={}",
        args.hex,
        applied,
        rejected,
        ledger.last_seq,
        soft_cap_houses()
    );
    ExitCode::SUCCESS
}
