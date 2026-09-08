//! F1 — shared hex shard protocol types (v23.2.54)
//!
//! protocol_id `powrush.hex.v1` · protocol_rev `1`.
//! Pure serde shapes + reject helpers. Default: no listen socket / no WS client.
//! F8: outbound WS gated by POWRUSH_NET=localhost (see hex_listen).
//! No server unpark. Default remains offline client authority.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Wire protocol identity. Mismatch → [`RejectCode::Proto`].
pub const PROTOCOL_ID: &str = "powrush.hex.v1";
/// Wire revision. Bump only with a steward PROTOCOL.md change.
pub const PROTOCOL_REV: u32 = 1;
/// Envelope `v` for this rev.
pub const ENVELOPE_V: u32 = 1;

/// JSON envelope — every message on the wire (when net exists).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub v: u32,
    pub pid: String,
    pub kind: String,
    pub hex: String,
    pub house: String,
    pub seq: u64,
    pub ts_ms: u64,
    pub body: serde_json::Value,
}

impl Envelope {
    pub fn new(kind: impl Into<String>, hex: impl Into<String>, house: impl Into<String>, seq: u64, ts_ms: u64, body: serde_json::Value) -> Self {
        Self {
            v: ENVELOPE_V,
            pid: PROTOCOL_ID.into(),
            kind: kind.into(),
            hex: hex.into(),
            house: house.into(),
            seq,
            ts_ms,
            body,
        }
    }

    /// Pid / rev / envelope version check. Wrong → PROTO.
    pub fn check_proto(&self, peer_rev: u32) -> Result<(), RejectCode> {
        if self.v != ENVELOPE_V || self.pid != PROTOCOL_ID || peer_rev != PROTOCOL_REV {
            return Err(RejectCode::Proto);
        }
        Ok(())
    }
}

/// Client → shard op kinds (also used offline as local intent tags).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Op {
    Tend,
    Take,
    Flow,
    Reserve,
    Mend,
    Lane,
    Bind,
    DeclareLethal,
    ClearLethal,
    NameHouse,
    RequestSeat,
    SnapshotReq,
    Hello,
}

impl Op {
    pub fn as_str(self) -> &'static str {
        match self {
            Op::Tend => "tend",
            Op::Take => "take",
            Op::Flow => "flow",
            Op::Reserve => "reserve",
            Op::Mend => "mend",
            Op::Lane => "lane",
            Op::Bind => "bind",
            Op::DeclareLethal => "declare_lethal",
            Op::ClearLethal => "clear_lethal",
            Op::NameHouse => "name_house",
            Op::RequestSeat => "request_seat",
            Op::SnapshotReq => "snapshot_req",
            Op::Hello => "hello",
        }
    }
}

/// Shard → client reply kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShardKind {
    Apply,
    Reject,
    Snapshot,
    Week,
    Presence,
    HelloOk,
    HelloNo,
}

impl ShardKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ShardKind::Apply => "apply",
            ShardKind::Reject => "reject",
            ShardKind::Snapshot => "snapshot",
            ShardKind::Week => "week",
            ShardKind::Presence => "presence",
            ShardKind::HelloOk => "hello_ok",
            ShardKind::HelloNo => "hello_no",
        }
    }
}

/// Reject codes — steward-locked set for v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RejectCode {
    StaleSeq,
    NoTake,
    NotCharter,
    NoBook,
    NoPack,
    BadReserve,
    Teleport,
    Proto,
    CopyDenied,
}

impl RejectCode {
    pub fn as_str(self) -> &'static str {
        match self {
            RejectCode::StaleSeq => "STALE_SEQ",
            RejectCode::NoTake => "NO_TAKE",
            RejectCode::NotCharter => "NOT_CHARTER",
            RejectCode::NoBook => "NO_BOOK",
            RejectCode::NoPack => "NO_PACK",
            RejectCode::BadReserve => "BAD_RESERVE",
            RejectCode::Teleport => "TELEPORT",
            RejectCode::Proto => "PROTO",
            RejectCode::CopyDenied => "COPY_DENIED",
        }
    }
}

/// Reject body on the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RejectBody {
    pub code: RejectCode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Hello body — client proposes rev; shard answers hello_ok / hello_no.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HelloBody {
    pub protocol_rev: u32,
    #[serde(default)]
    pub consent_copy: bool,
}

/// Presence = houses array only. Never carry client-side n_online.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Presence {
    pub houses: Vec<String>,
}

/// Book flags inside a ledger snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BookFlags {
    #[serde(default)]
    pub hour_two_held: bool,
    #[serde(default)]
    pub hour_three_held: bool,
}

/// House face inside a ledger snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotHouse {
    pub house_id: String,
    pub house_name: String,
}

impl Default for SnapshotHouse {
    fn default() -> Self {
        Self {
            house_id: "local-hex".into(),
            house_name: "Unnamed House".into(),
        }
    }
}

/// Climate slice of the ledger snapshot (wire-stable fields).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotClimate {
    pub hex_id: String,
    pub harmony: f32,
    pub stress: f32,
    pub regen: f32,
    pub reserve_pool: u32,
    pub restored_count: u32,
    pub tons_moved: u32,
}

impl Default for SnapshotClimate {
    fn default() -> Self {
        Self {
            hex_id: "local-hex".into(),
            harmony: 0.55,
            stress: 0.15,
            regen: 0.08,
            reserve_pool: 0,
            restored_count: 0,
            tons_moved: 0,
        }
    }
}

/// Standing slice of the ledger snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotStanding {
    pub hex_id: String,
    pub peace: f32,
    pub harmony: f32,
    pub consumption: f32,
    pub steward: f32,
    pub human_hybrid_heat: f32,
    #[serde(default)]
    pub declared_lethal: bool,
    #[serde(default)]
    pub tariff_paid: u32,
}

impl Default for SnapshotStanding {
    fn default() -> Self {
        Self {
            hex_id: "local-hex".into(),
            peace: 0.72,
            harmony: 0.55,
            consumption: 0.12,
            steward: 0.40,
            human_hybrid_heat: 0.0,
            declared_lethal: false,
            tariff_paid: 0,
        }
    }
}

/// Week slice of the ledger snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotWeek {
    pub week_id: String,
    pub tons_moved: u32,
    pub restored_count: u32,
}

impl Default for SnapshotWeek {
    fn default() -> Self {
        Self {
            week_id: "local-week".into(),
            tons_moved: 0,
            restored_count: 0,
        }
    }
}

/// Full ledger snapshot — climate / standing / week / book / house.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Snapshot {
    pub climate: SnapshotClimate,
    pub standing: SnapshotStanding,
    pub week: SnapshotWeek,
    pub book: BookFlags,
    pub house: SnapshotHouse,
}

/// Well / node tiredness for take gating (local or shard-side).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WellFace {
    Idle,
    Glowing,
    Tended,
    Resting,
    Stressed,
}

impl WellFace {
    pub fn is_tired(self) -> bool {
        matches!(self, WellFace::Resting | WellFace::Stressed)
    }
}

/// declare_lethal before book → NO_BOOK.
pub fn reject_declare_lethal_before_book(hour_three_held: bool) -> Result<(), RejectCode> {
    if hour_three_held {
        Ok(())
    } else {
        Err(RejectCode::NoBook)
    }
}

/// declare_lethal before Settled (hour two held) → NOT_CHARTER.
pub fn reject_declare_lethal_before_settled(hour_two_held: bool) -> Result<(), RejectCode> {
    if hour_two_held {
        Ok(())
    } else {
        Err(RejectCode::NotCharter)
    }
}

/// take on tired well → NO_TAKE.
pub fn reject_take_on_tired(face: WellFace) -> Result<(), RejectCode> {
    if face.is_tired() {
        Err(RejectCode::NoTake)
    } else {
        Ok(())
    }
}

/// protocol_rev / pid mismatch → PROTO.
pub fn reject_rev_mismatch(peer_pid: &str, peer_rev: u32) -> Result<(), RejectCode> {
    if peer_pid != PROTOCOL_ID || peer_rev != PROTOCOL_REV {
        Err(RejectCode::Proto)
    } else {
        Ok(())
    }
}

/// Join without copy consent → COPY_DENIED.
pub fn reject_copy_without_consent(consent_copy: bool) -> Result<(), RejectCode> {
    if consent_copy {
        Ok(())
    } else {
        Err(RejectCode::CopyDenied)
    }
}

/// seq must be strictly greater than last applied.
pub fn reject_stale_seq(seq: u64, last_applied: u64) -> Result<(), RejectCode> {
    if seq <= last_applied {
        Err(RejectCode::StaleSeq)
    } else {
        Ok(())
    }
}

/// request_seat without Proof Pack → NO_PACK.
pub fn reject_seat_without_pack(pack_unlocked: bool) -> Result<(), RejectCode> {
    if pack_unlocked {
        Ok(())
    } else {
        Err(RejectCode::NoPack)
    }
}

/// F1 law: default client does not listen; server stays parked.
pub fn default_client_listens() -> bool {
    false
}

pub fn server_unparked_v1() -> bool {
    false
}

/// Env mirror — POWRUSH_NET default off (honest offline).
/// F8: only `localhost` / `loopback` / `127.0.0.1` enable outbound WS intent.
/// Legacy on/true/yes do **not** open sockets (Settings/env door is localhost).
pub fn net_enabled() -> bool {
    match std::env::var("POWRUSH_NET") {
        Ok(v) => {
            let t = v.trim().to_ascii_lowercase();
            matches!(t.as_str(), "localhost" | "loopback" | "127.0.0.1")
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_declare_lethal_before_book_is_no_book() {
        assert_eq!(
            reject_declare_lethal_before_book(false),
            Err(RejectCode::NoBook)
        );
        assert_eq!(reject_declare_lethal_before_book(true), Ok(()));
        assert_eq!(RejectCode::NoBook.as_str(), "NO_BOOK");
        assert_eq!(
            reject_declare_lethal_before_settled(false),
            Err(RejectCode::NotCharter)
        );
        assert_eq!(reject_declare_lethal_before_settled(true), Ok(()));
        assert_eq!(RejectCode::NotCharter.as_str(), "NOT_CHARTER");
    }

    #[test]
    fn reject_take_on_tired_is_no_take() {
        assert_eq!(
            reject_take_on_tired(WellFace::Resting),
            Err(RejectCode::NoTake)
        );
        assert_eq!(
            reject_take_on_tired(WellFace::Stressed),
            Err(RejectCode::NoTake)
        );
        assert_eq!(reject_take_on_tired(WellFace::Glowing), Ok(()));
        assert_eq!(RejectCode::NoTake.as_str(), "NO_TAKE");
    }

    #[test]
    fn presence_has_no_n_online_field() {
        let p = Presence {
            houses: vec!["Unnamed House".into(), "Oak".into()],
        };
        let raw = serde_json::to_string(&p).expect("ser");
        assert!(raw.contains("houses"));
        assert!(!raw.contains("n_online"));
        let back: Presence = serde_json::from_str(&raw).expect("de");
        assert_eq!(back.houses.len(), 2);
        // Client must not invent a count field on the struct.
        let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert!(v.get("n_online").is_none());
    }

    #[test]
    fn rev_mismatch_is_proto() {
        assert_eq!(
            reject_rev_mismatch("powrush.hex.v1", 2),
            Err(RejectCode::Proto)
        );
        assert_eq!(
            reject_rev_mismatch("other.pid", PROTOCOL_REV),
            Err(RejectCode::Proto)
        );
        assert_eq!(reject_rev_mismatch(PROTOCOL_ID, PROTOCOL_REV), Ok(()));
        let env = Envelope::new("hello", "local-hex", "h1", 1, 0, serde_json::json!({}));
        assert_eq!(env.check_proto(99), Err(RejectCode::Proto));
        assert_eq!(env.check_proto(PROTOCOL_REV), Ok(()));
        assert_eq!(RejectCode::Proto.as_str(), "PROTO");
    }

    #[test]
    fn envelope_round_trip_op_and_reject() {
        let body = serde_json::json!({ "protocol_rev": 1, "consent_copy": true });
        let env = Envelope::new(Op::Hello.as_str(), "local-hex", "local-house", 1, 1_700_000_000_000, body);
        let raw = serde_json::to_string(&env).unwrap();
        let back: Envelope = serde_json::from_str(&raw).unwrap();
        assert_eq!(back.pid, PROTOCOL_ID);
        assert_eq!(back.kind, "hello");
        assert_eq!(back.v, ENVELOPE_V);

        let rej = RejectBody {
            code: RejectCode::CopyDenied,
            reason: Some("no consent".into()),
        };
        let rraw = serde_json::to_string(&rej).unwrap();
        assert!(rraw.contains("COPY_DENIED"));
        let rback: RejectBody = serde_json::from_str(&rraw).unwrap();
        assert_eq!(rback.code, RejectCode::CopyDenied);
    }

    #[test]
    fn default_net_path_no_listen_no_unpark() {
        assert!(!default_client_listens());
        assert!(!server_unparked_v1());
        // Without POWRUSH_NET=localhost, net stays off.
        std::env::remove_var("POWRUSH_NET");
        assert!(!net_enabled());
    }

    #[test]
    fn snapshot_shape_has_ledger_parts() {
        let s = Snapshot::default();
        let raw = serde_json::to_value(&s).unwrap();
        for key in ["climate", "standing", "week", "book", "house"] {
            assert!(raw.get(key).is_some(), "missing {key}");
        }
        assert!(raw.get("n_online").is_none());
    }

    #[test]
    fn reject_helpers_cover_pack_seq_copy() {
        assert_eq!(reject_seat_without_pack(false), Err(RejectCode::NoPack));
        assert_eq!(reject_stale_seq(3, 3), Err(RejectCode::StaleSeq));
        assert_eq!(reject_stale_seq(4, 3), Ok(()));
        assert_eq!(reject_copy_without_consent(false), Err(RejectCode::CopyDenied));
        assert_eq!(reject_copy_without_consent(true), Ok(()));
    }

    #[test]
    fn op_and_shard_kind_wire_names() {
        assert_eq!(Op::DeclareLethal.as_str(), "declare_lethal");
        assert_eq!(Op::SnapshotReq.as_str(), "snapshot_req");
        assert_eq!(ShardKind::HelloOk.as_str(), "hello_ok");
        assert_eq!(ShardKind::HelloNo.as_str(), "hello_no");
        assert_eq!(ShardKind::Presence.as_str(), "presence");
    }
}
