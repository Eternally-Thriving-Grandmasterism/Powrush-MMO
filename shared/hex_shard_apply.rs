//! F7 — parked hex-shard ledger apply helpers (v23.2.56)
//!
//! Pure offline apply: load one hex snapshot, apply verb events, write snapshot.
//! Soft cap: 32 Houses (documented). Reuses `hex_protocol` reject helpers.
//! F8: loopback listen lives in powrush-shard + hex_listen (not client default door).
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hex_protocol::{
    default_client_listens, reject_declare_lethal_before_book,
    reject_declare_lethal_before_settled, reject_stale_seq, reject_take_on_tired, BookFlags,
    Envelope, Op, Presence, RejectCode, Snapshot, SnapshotClimate, SnapshotHouse,
    SnapshotStanding, SnapshotWeek, WellFace, PROTOCOL_ID, PROTOCOL_REV,
};

/// Soft cap on seated Houses per hex shard (steward law; not a hard crash).
pub const SOFT_CAP_HOUSES: usize = 32;

/// Legacy parked string (F7). F8 loopback listen is live — prefer LISTEN_LOOPBACK_MSG.
pub const LISTEN_PARKED_MSG: &str =
    "powrush-shard: --listen is parked / not enabled (WS listen not wired yet)";

/// Honest copy when loopback listen is active (F8).
pub const LISTEN_LOOPBACK_MSG: &str =
    "powrush-shard: listening on loopback only (refuse 0.0.0.0 / non-loopback)";

/// Snapshot filename under `--data`.
pub const SNAPSHOT_FILE: &str = "ledger_snapshot.json";

/// In-memory shard ledger for one hex.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShardLedger {
    pub hex_id: String,
    pub snapshot: Snapshot,
    pub last_seq: u64,
    pub presence: Presence,
    pub well: WellFace,
}

impl ShardLedger {
    pub fn fresh(hex_id: &str) -> Self {
        let mut snap = Snapshot::default();
        snap.climate.hex_id = hex_id.into();
        snap.standing.hex_id = hex_id.into();
        snap.house = SnapshotHouse {
            house_id: format!("{hex_id}-house0"),
            house_name: "Unnamed House".into(),
        };
        Self {
            hex_id: hex_id.into(),
            snapshot: snap,
            last_seq: 0,
            presence: Presence::default(),
            well: WellFace::Idle,
        }
    }

    pub fn from_snapshot(hex_id: &str, snapshot: Snapshot, last_seq: u64) -> Self {
        Self {
            hex_id: hex_id.into(),
            snapshot,
            last_seq,
            presence: Presence::default(),
            well: WellFace::Idle,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }

    pub fn snapshot_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.snapshot)
    }
}

/// Result of applying one verb.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyOutcome {
    Applied { seq: u64, op: String },
    Rejected { code: RejectCode, reason: Option<String> },
}

/// Seat a house into presence — soft-cap aware (reject when full).
pub fn seat_house(ledger: &mut ShardLedger, house: &str) -> Result<(), RejectCode> {
    if ledger.presence.houses.iter().any(|h| h == house) {
        return Ok(());
    }
    if ledger.presence.houses.len() >= SOFT_CAP_HOUSES {
        return Err(RejectCode::NoPack); // soft-full: reuse NO_PACK as "no seat room" for v1 parked
    }
    ledger.presence.houses.push(house.into());
    Ok(())
}

/// Soft-cap documentation helper (tests + CLI banner).
pub fn soft_cap_houses() -> usize {
    SOFT_CAP_HOUSES
}

/// F7 parked flag — F8 returns false (loopback listen unparked in powrush-shard).
pub fn listen_is_parked() -> bool {
    false
}

/// Default client door still does not listen (Core law; shard bin stays parked).
pub fn client_default_door_listens() -> bool {
    default_client_listens()
}

/// Apply one verb to the ledger (offline / dry-apply path).
pub fn apply_verb(
    ledger: &mut ShardLedger,
    op: Op,
    seq: u64,
    house: &str,
    body: &Value,
) -> ApplyOutcome {
    if let Err(code) = reject_stale_seq(seq, ledger.last_seq) {
        return ApplyOutcome::Rejected {
            code,
            reason: Some(format!("seq {seq} <= last {}", ledger.last_seq)),
        };
    }

    match op {
        Op::Tend => {
            let c = &mut ledger.snapshot.climate;
            c.harmony = (c.harmony + 0.02).min(1.0);
            c.stress = (c.stress - 0.01).max(0.0);
            c.regen = (c.regen + 0.005).min(1.0);
            ledger.well = WellFace::Tended;
            let w = &mut ledger.snapshot.week;
            w.restored_count = w.restored_count.saturating_add(1);
            finish_apply(ledger, seq, house, op)
        }
        Op::Take => {
            if let Err(code) = reject_take_on_tired(ledger.well) {
                return ApplyOutcome::Rejected {
                    code,
                    reason: Some("well tired".into()),
                };
            }
            let c = &mut ledger.snapshot.climate;
            c.tons_moved = c.tons_moved.saturating_add(1);
            c.stress = (c.stress + 0.03).min(1.0);
            c.harmony = (c.harmony - 0.01).max(0.0);
            ledger.snapshot.week.tons_moved = c.tons_moved;
            ledger.well = WellFace::Resting;
            finish_apply(ledger, seq, house, op)
        }
        Op::DeclareLethal => {
            if let Err(code) =
                reject_declare_lethal_before_book(ledger.snapshot.book.hour_three_held)
            {
                return ApplyOutcome::Rejected {
                    code,
                    reason: Some("book not held".into()),
                };
            }
            if let Err(code) =
                reject_declare_lethal_before_settled(ledger.snapshot.book.hour_two_held)
            {
                return ApplyOutcome::Rejected {
                    code,
                    reason: Some("not your charter".into()),
                };
            }
            // Hex sign only — tariff may dent this hex climate. Week stays tons + restored.
            let c = &mut ledger.snapshot.climate;
            c.stress = (c.stress + 0.14).min(1.0);
            c.harmony = (c.harmony - 0.16).max(0.0);
            ledger.snapshot.standing.declared_lethal = true;
            finish_apply(ledger, seq, house, op)
        }
        Op::ClearLethal => {
            ledger.snapshot.standing.declared_lethal = false;
            finish_apply(ledger, seq, house, op)
        }
        Op::NameHouse => {
            let name = body
                .get("house_name")
                .and_then(|v| v.as_str())
                .unwrap_or("Unnamed House");
            ledger.snapshot.house.house_name = name.into();
            if !house.is_empty() {
                ledger.snapshot.house.house_id = house.into();
            }
            finish_apply(ledger, seq, house, op)
        }
        Op::Mend => {
            let c = &mut ledger.snapshot.climate;
            c.harmony = (c.harmony + 0.03).min(1.0);
            c.restored_count = c.restored_count.saturating_add(1);
            ledger.snapshot.week.restored_count = c.restored_count;
            finish_apply(ledger, seq, house, op)
        }
        Op::Lane | Op::Flow | Op::Reserve | Op::Bind | Op::RequestSeat | Op::SnapshotReq
        | Op::Hello => {
            // Parked-bin accepts these as no-op seq advances (protocol kinds known).
            if op == Op::RequestSeat {
                if let Err(code) = seat_house(ledger, house) {
                    return ApplyOutcome::Rejected {
                        code,
                        reason: Some(format!(
                            "soft cap {SOFT_CAP_HOUSES} houses reached"
                        )),
                    };
                }
            }
            finish_apply(ledger, seq, house, op)
        }
    }
}

fn finish_apply(ledger: &mut ShardLedger, seq: u64, house: &str, op: Op) -> ApplyOutcome {
    ledger.last_seq = seq;
    let _ = seat_house(ledger, house);
    ApplyOutcome::Applied {
        seq,
        op: op.as_str().into(),
    }
}

/// Apply a wire envelope offline (proto check + verb).
pub fn apply_envelope(ledger: &mut ShardLedger, env: &Envelope) -> ApplyOutcome {
    if env.check_proto(PROTOCOL_REV).is_err() {
        return ApplyOutcome::Rejected {
            code: RejectCode::Proto,
            reason: Some(format!("want {PROTOCOL_ID} rev {PROTOCOL_REV}")),
        };
    }
    let op = match env.kind.as_str() {
        "tend" => Op::Tend,
        "take" => Op::Take,
        "flow" => Op::Flow,
        "reserve" => Op::Reserve,
        "mend" => Op::Mend,
        "lane" => Op::Lane,
        "bind" => Op::Bind,
        "declare_lethal" => Op::DeclareLethal,
        "clear_lethal" => Op::ClearLethal,
        "name_house" => Op::NameHouse,
        "request_seat" => Op::RequestSeat,
        "snapshot_req" => Op::SnapshotReq,
        "hello" => Op::Hello,
        other => {
            return ApplyOutcome::Rejected {
                code: RejectCode::Proto,
                reason: Some(format!("unknown kind {other}")),
            };
        }
    };
    apply_verb(ledger, op, env.seq, &env.house, &env.body)
}

/// One JSONL event: either a full Envelope or `{op,seq,house,body?}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonlEvent {
    #[serde(default)]
    pub op: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub seq: Option<u64>,
    #[serde(default)]
    pub house: Option<String>,
    #[serde(default)]
    pub hex: Option<String>,
    #[serde(default)]
    pub body: Option<Value>,
    #[serde(default)]
    pub v: Option<u32>,
    #[serde(default)]
    pub pid: Option<String>,
    #[serde(default)]
    pub ts_ms: Option<u64>,
}

impl JsonlEvent {
    pub fn into_envelope(self, default_hex: &str) -> Result<Envelope, String> {
        if self.v.is_some() || self.pid.is_some() {
            // Treat as full-ish envelope fields.
            let kind = self
                .kind
                .or(self.op)
                .ok_or_else(|| "missing kind/op".to_string())?;
            return Ok(Envelope {
                v: self.v.unwrap_or(1),
                pid: self.pid.unwrap_or_else(|| PROTOCOL_ID.into()),
                kind,
                hex: self.hex.unwrap_or_else(|| default_hex.into()),
                house: self.house.unwrap_or_else(|| "local-house".into()),
                seq: self.seq.unwrap_or(0),
                ts_ms: self.ts_ms.unwrap_or(0),
                body: self.body.unwrap_or(Value::Null),
            });
        }
        let kind = self
            .op
            .or(self.kind)
            .ok_or_else(|| "missing op/kind".to_string())?;
        let seq = self.seq.ok_or_else(|| "missing seq".to_string())?;
        Ok(Envelope::new(
            kind,
            self.hex.unwrap_or_else(|| default_hex.into()),
            self.house.unwrap_or_else(|| "local-house".into()),
            seq,
            self.ts_ms.unwrap_or(0),
            self.body.unwrap_or(Value::Null),
        ))
    }
}

/// Apply all non-empty JSONL lines; returns outcomes in order.
pub fn apply_jsonl(ledger: &mut ShardLedger, jsonl: &str) -> Vec<ApplyOutcome> {
    let mut out = Vec::new();
    for (i, line) in jsonl.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match serde_json::from_str::<JsonlEvent>(line) {
            Ok(ev) => match ev.into_envelope(&ledger.hex_id) {
                Ok(env) => out.push(apply_envelope(ledger, &env)),
                Err(e) => out.push(ApplyOutcome::Rejected {
                    code: RejectCode::Proto,
                    reason: Some(format!("line {}: {e}", i + 1)),
                }),
            },
            Err(e) => out.push(ApplyOutcome::Rejected {
                code: RejectCode::Proto,
                reason: Some(format!("line {}: {e}", i + 1)),
            }),
        }
    }
    out
}

/// Load snapshot JSON (ledger Snapshot shape).
pub fn load_snapshot_json(raw: &str) -> Result<Snapshot, serde_json::Error> {
    serde_json::from_str(raw)
}

/// Write snapshot JSON.
pub fn write_snapshot_json(snap: &Snapshot) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(snap)
}

/// Fixture snapshot used by offline tests.
pub fn fixture_snapshot(hex_id: &str) -> Snapshot {
    Snapshot {
        climate: SnapshotClimate {
            hex_id: hex_id.into(),
            harmony: 0.55,
            stress: 0.15,
            regen: 0.08,
            reserve_pool: 0,
            restored_count: 0,
            tons_moved: 0,
        },
        standing: SnapshotStanding {
            hex_id: hex_id.into(),
            peace: 0.72,
            harmony: 0.55,
            consumption: 0.12,
            steward: 0.40,
            human_hybrid_heat: 0.0,
            declared_lethal: false,
            tariff_paid: 0,
        },
        week: SnapshotWeek {
            week_id: "local-week".into(),
            tons_moved: 0,
            restored_count: 0,
        },
        book: BookFlags {
            hour_two_held: false,
            hour_three_held: false,
        },
        house: SnapshotHouse {
            house_id: "h-fixture".into(),
            house_name: "Unnamed House".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_tend_and_take_offline_to_fixture() {
        let mut ledger = ShardLedger::from_snapshot("hex_local_0", fixture_snapshot("hex_local_0"), 0);
        let tend = apply_verb(
            &mut ledger,
            Op::Tend,
            1,
            "h-fixture",
            &Value::Null,
        );
        assert!(matches!(tend, ApplyOutcome::Applied { seq: 1, .. }));
        assert!(ledger.snapshot.climate.harmony > 0.55);
        assert_eq!(ledger.snapshot.week.restored_count, 1);

        // After tend, well is Tended — take should succeed once, then resting blocks.
        let take = apply_verb(&mut ledger, Op::Take, 2, "h-fixture", &Value::Null);
        assert!(matches!(take, ApplyOutcome::Applied { seq: 2, .. }));
        assert_eq!(ledger.snapshot.climate.tons_moved, 1);
        assert_eq!(ledger.last_seq, 2);

        let take2 = apply_verb(&mut ledger, Op::Take, 3, "h-fixture", &Value::Null);
        assert_eq!(
            take2,
            ApplyOutcome::Rejected {
                code: RejectCode::NoTake,
                reason: Some("well tired".into()),
            }
        );
    }

    #[test]
    fn lethal_before_book_rejected() {
        let mut ledger = ShardLedger::from_snapshot("hex_local_0", fixture_snapshot("hex_local_0"), 0);
        assert!(!ledger.snapshot.book.hour_three_held);
        let r = apply_verb(
            &mut ledger,
            Op::DeclareLethal,
            1,
            "h-fixture",
            &Value::Null,
        );
        assert_eq!(
            r,
            ApplyOutcome::Rejected {
                code: RejectCode::NoBook,
                reason: Some("book not held".into()),
            }
        );
        assert!(!ledger.snapshot.standing.declared_lethal);

        ledger.snapshot.book.hour_three_held = true;
        let no_settle = apply_verb(
            &mut ledger,
            Op::DeclareLethal,
            1,
            "h-fixture",
            &Value::Null,
        );
        assert_eq!(
            no_settle,
            ApplyOutcome::Rejected {
                code: RejectCode::NotCharter,
                reason: Some("not your charter".into()),
            }
        );
        assert!(!ledger.snapshot.standing.declared_lethal);

        ledger.snapshot.book.hour_two_held = true;
        let tons_before = ledger.snapshot.climate.tons_moved;
        let week_tons_before = ledger.snapshot.week.tons_moved;
        let ok = apply_verb(
            &mut ledger,
            Op::DeclareLethal,
            1,
            "h-fixture",
            &Value::Null,
        );
        assert!(matches!(ok, ApplyOutcome::Applied { .. }));
        assert!(ledger.snapshot.standing.declared_lethal);
        assert_eq!(ledger.snapshot.climate.tons_moved, tons_before);
        assert_eq!(ledger.snapshot.week.tons_moved, week_tons_before);
        assert!(ledger.snapshot.climate.stress > 0.15);
        assert!(ledger.snapshot.climate.harmony < 0.55);
    }

    #[test]
    fn client_default_still_no_listen() {
        assert!(!client_default_door_listens());
        assert!(!default_client_listens());
        assert!(!listen_is_parked()); // F8: shard loopback listen live; client door still quiet
        assert_eq!(soft_cap_houses(), 32);
        assert!(LISTEN_LOOPBACK_MSG.contains("loopback"));
        assert!(LISTEN_PARKED_MSG.contains("parked")); // retained for archaeology
    }

    #[test]
    fn soft_cap_32_houses_documented() {
        let mut ledger = ShardLedger::fresh("hex_cap");
        for i in 0..SOFT_CAP_HOUSES {
            assert!(seat_house(&mut ledger, &format!("h{i}")).is_ok());
        }
        assert_eq!(ledger.presence.houses.len(), SOFT_CAP_HOUSES);
        assert_eq!(seat_house(&mut ledger, "overflow"), Err(RejectCode::NoPack));
    }

    #[test]
    fn apply_jsonl_tend_writes_snapshot_shape() {
        let mut ledger = ShardLedger::from_snapshot("hex_local_0", fixture_snapshot("hex_local_0"), 0);
        let jsonl = r#"{"op":"tend","seq":1,"house":"h-fixture"}
{"op":"mend","seq":2,"house":"h-fixture"}
"#;
        let outs = apply_jsonl(&mut ledger, jsonl);
        assert_eq!(outs.len(), 2);
        assert!(matches!(outs[0], ApplyOutcome::Applied { .. }));
        assert!(matches!(outs[1], ApplyOutcome::Applied { .. }));
        let raw = write_snapshot_json(&ledger.snapshot).unwrap();
        let back = load_snapshot_json(&raw).unwrap();
        assert_eq!(back.climate.hex_id, "hex_local_0");
        assert!(back.week.restored_count >= 1);
        assert!(raw.contains("climate"));
        assert!(!raw.contains("n_online"));
    }

    #[test]
    fn stale_seq_rejected() {
        let mut ledger = ShardLedger::from_snapshot("hex_local_0", fixture_snapshot("hex_local_0"), 5);
        let r = apply_verb(&mut ledger, Op::Tend, 5, "h", &Value::Null);
        assert!(matches!(
            r,
            ApplyOutcome::Rejected {
                code: RejectCode::StaleSeq,
                ..
            }
        ));
    }
}
