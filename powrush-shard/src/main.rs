//! powrush-shard — loopback WS + offline dry-apply (F8 / v23.2.57)
//!
//! `--listen 127.0.0.1:7788` accepts JSON WS (hello → hello_ok/hello_no;
//! tend/take/flow/reserve via shared hex_shard_apply). Refuse public bind.
//! Soft cap 32 Houses. Not wired into powrush-client default door.
//! Contact: info@Rathor.ai

mod listen;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use shared::hex_listen::{validate_listen_bind, ListenRefuse, DEFAULT_LOCALHOST_LISTEN};
use shared::hex_protocol::Snapshot;
use shared::hex_shard_apply::{
    apply_jsonl, client_default_door_listens, fixture_snapshot, listen_is_parked, load_snapshot_json,
    soft_cap_houses, write_snapshot_json, ApplyOutcome, ShardLedger, LISTEN_LOOPBACK_MSG,
    SNAPSHOT_FILE,
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
powrush-shard — hex ledger + loopback WS (not default client door)

USAGE:
  powrush-shard --hex <id> --data <dir> [--listen 127.0.0.1:7788] [--dry-apply <events.jsonl>]

FLAGS:
  --hex <id>              Hex id (e.g. hex_local_0)
  --data <dir>            Data directory (ledger_snapshot.json lives here)
  --dry-apply <jsonl>     Apply JSONL verb events offline; write snapshot
  --listen <addr>         Bind WS on loopback only (refuse 0.0.0.0 / non-loopback)
  -h, --help              Show this help

Soft cap: {} Houses per hex.
Listen: loopback only — e.g. {DEFAULT_LOCALHOST_LISTEN}
Default client door remains: cargo run -p powrush-client (no shard dep).
Client outbound WS only when POWRUSH_NET=localhost (title Online stays grey).",
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
                    .ok_or_else(|| "--hex needs a value".to_string())?;
                i += 1;
            }
            "--listen" => {
                i += 1;
                args.listen = Some(
                    argv.get(i)
                        .cloned()
                        .ok_or_else(|| "--listen needs addr".to_string())?,
                );
                i += 1;
            }
            "--data" => {
                i += 1;
                args.data = PathBuf::from(
                    argv.get(i)
                        .cloned()
                        .ok_or_else(|| "--data needs a path".to_string())?,
                );
                i += 1;
            }
            "--dry-apply" => {
                i += 1;
                args.dry_apply = Some(PathBuf::from(
                    argv.get(i)
                        .cloned()
                        .ok_or_else(|| "--dry-apply needs a path".to_string())?,
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

fn run_dry_apply(hex: &str, data: &Path, jsonl_path: &Path) -> ExitCode {
    let mut ledger = load_or_fresh(hex, data);
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

    if let Err(e) = write_snapshot(data, &ledger.snapshot) {
        eprintln!("powrush-shard: write failed: {e}");
        return ExitCode::from(1);
    }

    eprintln!(
        "powrush-shard: done hex={hex} applied={applied} rejected={rejected} last_seq={} soft_cap={}",
        ledger.last_seq,
        soft_cap_houses()
    );
    ExitCode::SUCCESS
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

    debug_assert!(!listen_is_parked());
    debug_assert!(!client_default_door_listens());

    if let Some(addr_raw) = &args.listen {
        let addr = match validate_listen_bind(addr_raw) {
            Ok(a) => a,
            Err(e) => {
                let msg = match &e {
                    ListenRefuse::BadAddr(s)
                    | ListenRefuse::PublicOrNonLoopback(s)
                    | ListenRefuse::Wildcard(s) => s.clone(),
                };
                eprintln!("powrush-shard: {msg}");
                eprintln!("{LISTEN_LOOPBACK_MSG}");
                return ExitCode::from(2);
            }
        };
        eprintln!("{LISTEN_LOOPBACK_MSG}");

        if let Some(jsonl_path) = &args.dry_apply {
            let code = run_dry_apply(&args.hex, &args.data, jsonl_path);
            if code != ExitCode::SUCCESS {
                return code;
            }
        }

        let ledger = load_or_fresh(&args.hex, &args.data);
        let data = args.data.clone();
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("powrush-shard: tokio runtime failed: {e}");
                return ExitCode::from(1);
            }
        };
        if let Err(e) = rt.block_on(listen::run_loopback_ws(addr, ledger, data)) {
            eprintln!("powrush-shard: listen ended: {e}");
            return ExitCode::from(1);
        }
        return ExitCode::SUCCESS;
    }

    let Some(jsonl_path) = &args.dry_apply else {
        eprintln!("powrush-shard: nothing to do — pass --listen {DEFAULT_LOCALHOST_LISTEN} and/or --dry-apply <events.jsonl>");
        eprintln!(
            "powrush-shard: soft cap {} houses; loopback listen available; not default client door",
            soft_cap_houses()
        );
        return ExitCode::SUCCESS;
    };

    run_dry_apply(&args.hex, &args.data, jsonl_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::hex_listen::{parse_powrush_net_from, validate_listen_bind, PowrushNet};

    #[test]
    fn listen_bind_loopback_ok_public_refused() {
        assert!(validate_listen_bind("127.0.0.1:7788").is_ok());
        assert!(validate_listen_bind("0.0.0.0:7788").is_err());
    }

    #[test]
    fn client_default_door_still_no_listen() {
        assert!(!client_default_door_listens());
        assert!(!listen_is_parked());
        assert_eq!(parse_powrush_net_from(Some("off")), PowrushNet::Off);
        assert!(!PowrushNet::Off.may_outbound_ws());
    }
}
