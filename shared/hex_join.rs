//! F2–F4 — join / drop / presence / offline fallback helpers (v23.2.55)
//!
//! Authority rules beside `hex_protocol` shapes. No listen socket. No WS.
//! No server unpark. Default remains offline client authority.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::hex_protocol::{
    reject_copy_without_consent, reject_declare_lethal_before_book, reject_rev_mismatch,
    reject_take_on_tired, BookFlags, Envelope, HelloBody, Presence, RejectCode, ShardKind,
    Snapshot, SnapshotClimate, WellFace, PROTOCOL_ID, PROTOCOL_REV,
};

/// Who owns the ledger right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    /// Client owns L0 disk. Online UI grey.
    Offline,
    /// Shard owns apply/reject/snapshot after hello_ok.
    Online,
}

impl Default for AuthorityMode {
    fn default() -> Self {
        AuthorityMode::Offline
    }
}

/// Outcome of a join / hello attempt.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JoinOutcome {
    /// hello_ok after consent — shard authority; local yard remains.
    Joined { local_yard_remains: bool },
    /// Player cancelled before consent — stay offline.
    CancelledStayOffline,
    /// COPY_DENIED / hello_no — stay offline.
    CopyDeniedStayOffline { code: RejectCode },
    /// PROTO — stay offline.
    ProtoStayOffline,
}

/// Leave or net-drop result: always offline with a certified snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeaveOutcome {
    pub mode: AuthorityMode,
    pub snapshot: Snapshot,
    pub login_wall: bool,
}

/// Conceptual L0 disk fixture for disconnect / book survival tests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiskFixture {
    pub snapshot: Snapshot,
    pub book: BookFlags,
    pub authority: AuthorityMode,
}

impl DiskFixture {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }
}

/// Event-log style climate lineage for one hex (no silent merge).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HexEventLog {
    pub hex_id: String,
    pub lineage: String,
    pub climate: SnapshotClimate,
    pub events: Vec<String>,
}

impl HexEventLog {
    pub fn thrive(hex_id: &str) -> Self {
        Self {
            hex_id: hex_id.into(),
            lineage: "thrive".into(),
            climate: SnapshotClimate {
                hex_id: hex_id.into(),
                harmony: 0.82,
                stress: 0.08,
                regen: 0.18,
                reserve_pool: 4,
                restored_count: 3,
                tons_moved: 12,
            },
            events: vec!["tend".into(), "mend".into(), "lane".into(), "week_thrive".into()],
        }
    }

    pub fn poor(hex_id: &str) -> Self {
        Self {
            hex_id: hex_id.into(),
            lineage: "poor".into(),
            climate: SnapshotClimate {
                hex_id: hex_id.into(),
                harmony: 0.28,
                stress: 0.72,
                regen: 0.03,
                reserve_pool: 0,
                restored_count: 0,
                tons_moved: 40,
            },
            events: vec!["take".into(), "take".into(), "stress_spike".into(), "week_poor".into()],
        }
    }
}

/// Attempt join. Cancel / deny / proto → offline. Consent + ok → online; yard remains.
pub fn attempt_join(consent_copy: bool, cancelled: bool, peer_rev: u32) -> JoinOutcome {
    if cancelled {
        return JoinOutcome::CancelledStayOffline;
    }
    if reject_rev_mismatch(PROTOCOL_ID, peer_rev).is_err() {
        return JoinOutcome::ProtoStayOffline;
    }
    match reject_copy_without_consent(consent_copy) {
        Ok(()) => JoinOutcome::Joined {
            local_yard_remains: true,
        },
        Err(code) => JoinOutcome::CopyDeniedStayOffline { code },
    }
}

/// Authority after a join outcome.
pub fn authority_after_join(outcome: &JoinOutcome) -> AuthorityMode {
    match outcome {
        JoinOutcome::Joined { .. } => AuthorityMode::Online,
        JoinOutcome::CancelledStayOffline
        | JoinOutcome::CopyDeniedStayOffline { .. }
        | JoinOutcome::ProtoStayOffline => AuthorityMode::Offline,
    }
}

/// hello_no body with COPY_DENIED → offline.
pub fn hello_no_keeps_offline(code: RejectCode) -> AuthorityMode {
    let _ = code;
    AuthorityMode::Offline
}

/// Leave or net drop: last certified snapshot, offline, no login wall.
pub fn leave_or_drop(last_certified: Snapshot) -> LeaveOutcome {
    LeaveOutcome {
        mode: AuthorityMode::Offline,
        snapshot: last_certified,
        login_wall: false,
    }
}

/// Disconnect mid-tend: in-flight tend dropped; disk keeps last snapshot + book.
pub fn disconnect_mid_tend(disk: &DiskFixture) -> DiskFixture {
    DiskFixture {
        snapshot: disk.snapshot.clone(),
        book: disk.book.clone(),
        authority: AuthorityMode::Offline,
    }
}

/// Presence must be shard-authored houses only. Client-supplied n_online is rejected.
pub fn presence_from_shard(houses: Vec<String>) -> Presence {
    Presence { houses }
}

/// Returns Err if a JSON presence payload tries to carry client-authored n_online.
pub fn reject_client_authored_n_online(raw: &str) -> Result<Presence, &'static str> {
    let v: serde_json::Value = serde_json::from_str(raw).map_err(|_| "bad_json")?;
    if v.get("n_online").is_some() {
        return Err("client_cannot_author_n_online");
    }
    let p: Presence = serde_json::from_value(v).map_err(|_| "bad_presence")?;
    Ok(p)
}

/// Never merge two divergent hex histories silently.
pub fn merge_hex_histories_silent(a: &HexEventLog, b: &HexEventLog) -> Result<HexEventLog, &'static str> {
    if a.hex_id == b.hex_id && a.lineage != b.lineage {
        return Err("never_merge_two_hex_histories_silently");
    }
    if a == b {
        return Ok(a.clone());
    }
    Err("never_merge_two_hex_histories_silently")
}

/// F2–F4 law mirrors: still no listen / no unpark.
pub fn join_slice_listens() -> bool {
    false
}

pub fn join_slice_unparks_server() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex_protocol::{RejectBody, SnapshotHouse, SnapshotStanding, SnapshotWeek};

    #[test]
    fn reject_declare_lethal_before_book_is_no_book() {
        assert_eq!(
            reject_declare_lethal_before_book(false),
            Err(RejectCode::NoBook)
        );
        assert_eq!(reject_declare_lethal_before_book(true), Ok(()));
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
    }

    #[test]
    fn presence_payload_rejects_client_n_online() {
        let honest = r#"{"houses":["Oak","Unnamed House"]}"#;
        let p = reject_client_authored_n_online(honest).expect("honest");
        assert_eq!(p.houses.len(), 2);
        assert!(!serde_json::to_string(&p).unwrap().contains("n_online"));

        let fake = r#"{"houses":["Oak"],"n_online":99}"#;
        assert_eq!(
            reject_client_authored_n_online(fake),
            Err("client_cannot_author_n_online")
        );

        let shard = presence_from_shard(vec!["Oak".into()]);
        let raw = serde_json::to_value(&shard).unwrap();
        assert!(raw.get("n_online").is_none());
    }

    #[test]
    fn rev_mismatch_is_proto() {
        assert_eq!(
            reject_rev_mismatch(PROTOCOL_ID, 2),
            Err(RejectCode::Proto)
        );
        assert_eq!(reject_rev_mismatch(PROTOCOL_ID, 1), Ok(()));
        let outcome = attempt_join(true, false, 99);
        assert_eq!(outcome, JoinOutcome::ProtoStayOffline);
        assert_eq!(authority_after_join(&outcome), AuthorityMode::Offline);
    }

    #[test]
    fn disconnect_mid_tend_keeps_last_snapshot_and_book() {
        let mut snap = Snapshot::default();
        snap.book = BookFlags {
            hour_two_held: true,
            hour_three_held: true,
        };
        snap.house = SnapshotHouse {
            house_id: "local-hex".into(),
            house_name: "Oak".into(),
        };
        snap.week = SnapshotWeek {
            week_id: "w1".into(),
            tons_moved: 7,
            restored_count: 2,
        };
        snap.standing = SnapshotStanding {
            declared_lethal: false,
            ..SnapshotStanding::default()
        };
        let disk = DiskFixture {
            snapshot: snap.clone(),
            book: snap.book.clone(),
            authority: AuthorityMode::Online,
        };
        // Conceptual tend in flight — never applied; disconnect.
        let after = disconnect_mid_tend(&disk);
        assert_eq!(after.authority, AuthorityMode::Offline);
        assert!(after.book.hour_two_held);
        assert!(after.book.hour_three_held);
        assert_eq!(after.snapshot.week.tons_moved, 7);
        assert_eq!(after.snapshot.house.house_name, "Oak");

        let raw = after.to_json().expect("ser");
        let loaded = DiskFixture::from_json(&raw).expect("de");
        assert_eq!(loaded.book.hour_three_held, true);
        assert_eq!(loaded.authority, AuthorityMode::Offline);
        assert!(!leave_or_drop(loaded.snapshot.clone()).login_wall);
        assert_eq!(leave_or_drop(loaded.snapshot).mode, AuthorityMode::Offline);
    }

    #[test]
    fn two_event_logs_diverge_climate_thrive_vs_poor() {
        let thrive = HexEventLog::thrive("local-hex");
        let poor = HexEventLog::poor("local-hex");
        assert_eq!(thrive.hex_id, poor.hex_id);
        assert!(thrive.climate.harmony > poor.climate.harmony);
        assert!(poor.climate.stress > thrive.climate.stress);
        assert_ne!(thrive.events, poor.events);
        assert_eq!(
            merge_hex_histories_silent(&thrive, &poor),
            Err("never_merge_two_hex_histories_silently")
        );
        // Same lineage round-trip ok.
        assert_eq!(
            merge_hex_histories_silent(&thrive, &thrive).unwrap().lineage,
            "thrive"
        );
        let raw_t = serde_json::to_string(&thrive).unwrap();
        let raw_p = serde_json::to_string(&poor).unwrap();
        let back_t: HexEventLog = serde_json::from_str(&raw_t).unwrap();
        let back_p: HexEventLog = serde_json::from_str(&raw_p).unwrap();
        assert!(back_t.climate.harmony > back_p.climate.harmony);
    }

    #[test]
    fn copy_denied_hello_no_keeps_offline() {
        let denied = attempt_join(false, false, PROTOCOL_REV);
        assert_eq!(
            denied,
            JoinOutcome::CopyDeniedStayOffline {
                code: RejectCode::CopyDenied
            }
        );
        assert_eq!(authority_after_join(&denied), AuthorityMode::Offline);
        assert_eq!(
            hello_no_keeps_offline(RejectCode::CopyDenied),
            AuthorityMode::Offline
        );

        let cancelled = attempt_join(true, true, PROTOCOL_REV);
        assert_eq!(cancelled, JoinOutcome::CancelledStayOffline);
        assert_eq!(authority_after_join(&cancelled), AuthorityMode::Offline);

        let ok = attempt_join(true, false, PROTOCOL_REV);
        assert_eq!(
            ok,
            JoinOutcome::Joined {
                local_yard_remains: true
            }
        );
        assert_eq!(authority_after_join(&ok), AuthorityMode::Online);

        let rej = RejectBody {
            code: RejectCode::CopyDenied,
            reason: Some("no consent".into()),
        };
        let env = Envelope::new(
            ShardKind::HelloNo.as_str(),
            "local-hex",
            "local-house",
            1,
            0,
            serde_json::to_value(&rej).unwrap(),
        );
        assert_eq!(env.kind, "hello_no");
        assert!(serde_json::to_string(&env).unwrap().contains("COPY_DENIED"));
    }

    #[test]
    fn join_hello_body_consent_round_trip() {
        let body = HelloBody {
            protocol_rev: PROTOCOL_REV,
            consent_copy: true,
        };
        let raw = serde_json::to_string(&body).unwrap();
        let back: HelloBody = serde_json::from_str(&raw).unwrap();
        assert!(back.consent_copy);
        assert_eq!(back.protocol_rev, 1);
    }

    #[test]
    fn join_slice_no_listen_no_unpark() {
        assert!(!join_slice_listens());
        assert!(!join_slice_unparks_server());
        assert_eq!(AuthorityMode::default(), AuthorityMode::Offline);
    }
}
