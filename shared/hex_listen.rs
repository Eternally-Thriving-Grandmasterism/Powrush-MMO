//! F8 — loopback-only listen bind + hello/session helpers (v23.2.57)
//!
//! Pure rules for `powrush-shard --listen`. No TLS theatre. No public bind.
//! Presence = real seated houses only. Contact: info@Rathor.ai

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::hex_join::{authority_after_join, attempt_join, leave_or_drop, AuthorityMode, LeaveOutcome};
use crate::hex_protocol::{
    Envelope, HelloBody, Presence, RejectBody, RejectCode, ShardKind, Snapshot, PROTOCOL_ID,
    PROTOCOL_REV,
};
use crate::hex_shard_apply::{apply_envelope, seat_house, ApplyOutcome, ShardLedger};

/// Default steward localhost listen (loopback only).
pub const DEFAULT_LOCALHOST_LISTEN: &str = "127.0.0.1:7788";

/// Why a listen bind was refused.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListenRefuse {
    BadAddr(String),
    PublicOrNonLoopback(String),
    Wildcard(String),
}

impl ListenRefuse {
    pub fn as_str(&self) -> &str {
        match self {
            ListenRefuse::BadAddr(s)
            | ListenRefuse::PublicOrNonLoopback(s)
            | ListenRefuse::Wildcard(s) => s.as_str(),
        }
    }
}

/// Parse and validate `--listen` — loopback only. Refuse `0.0.0.0` / `::` / non-loopback.
pub fn validate_listen_bind(raw: &str) -> Result<SocketAddr, ListenRefuse> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(ListenRefuse::BadAddr("empty listen addr".into()));
    }
    // Reject wildcard spellings before parse (parse may map 0.0.0.0 ok).
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("0.0.0.0")
        || lower.starts_with("[::]")
        || lower.starts_with("::")
        || lower.starts_with("[::0]")
        || lower.starts_with("*")
    {
        return Err(ListenRefuse::Wildcard(format!(
            "refusing public/wildcard bind {trimmed} — loopback only"
        )));
    }
    let addr: SocketAddr = trimmed
        .parse()
        .map_err(|e| ListenRefuse::BadAddr(format!("bad listen addr {trimmed}: {e}")))?;
    if !ip_is_loopback(addr.ip()) {
        return Err(ListenRefuse::PublicOrNonLoopback(format!(
            "refusing non-loopback bind {} — loopback only",
            addr.ip()
        )));
    }
    Ok(addr)
}

fn ip_is_loopback(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_loopback() || v4 == Ipv4Addr::LOCALHOST,
        IpAddr::V6(v6) => v6.is_loopback() || v6 == Ipv6Addr::LOCALHOST,
    }
}

/// Listen is unparked for loopback in F8 (still not the client default door).
pub fn listen_loopback_enabled() -> bool {
    true
}

/// POWRUSH_NET env — F8 wire door. Default off. Outbound WS only for `localhost`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PowrushNet {
    #[default]
    Off,
    /// Client may open outbound WS to a loopback shard. Title Online stays grey.
    Localhost,
}

impl PowrushNet {
    pub fn as_str(self) -> &'static str {
        match self {
            PowrushNet::Off => "off",
            PowrushNet::Localhost => "localhost",
        }
    }

    /// Outbound WS intent — only localhost. Never opens a listen socket.
    pub fn may_outbound_ws(self) -> bool {
        matches!(self, PowrushNet::Localhost)
    }

    /// Client never listens (shard bin is the listen door).
    pub fn opens_listen_socket(self) -> bool {
        false
    }

    /// Title Online row stays disabled/grey even when Localhost.
    pub fn title_online_enabled(self) -> bool {
        false
    }
}

/// Parse `POWRUSH_NET`. Unknown / empty / missing → Off. Only `localhost` enables outbound.
pub fn parse_powrush_net_from(raw: Option<&str>) -> PowrushNet {
    match raw {
        None => PowrushNet::Off,
        Some(v) => {
            let t = v.trim().to_ascii_lowercase();
            match t.as_str() {
                "localhost" | "loopback" | "127.0.0.1" => PowrushNet::Localhost,
                "off" | "0" | "false" | "no" | "" => PowrushNet::Off,
                // Legacy "on"/"true" do NOT open sockets in F8 — Settings/env door is localhost.
                _ => PowrushNet::Off,
            }
        }
    }
}

pub fn parse_powrush_net() -> PowrushNet {
    parse_powrush_net_from(std::env::var("POWRUSH_NET").ok().as_deref())
}

pub fn client_may_outbound_ws() -> bool {
    parse_powrush_net().may_outbound_ws()
}

pub fn client_opens_listen_socket() -> bool {
    parse_powrush_net().opens_listen_socket()
}

/// Default shard URL when POWRUSH_NET=localhost (hint only; client may override later).
pub fn default_localhost_shard_url() -> &'static str {
    "ws://127.0.0.1:7788"
}

/// Hello session reply for one inbound hello envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelloSession {
    Ok {
        house: String,
        presence: Presence,
    },
    No {
        code: RejectCode,
        reason: Option<String>,
    },
}

/// Handle JSON hello → hello_ok / hello_no.
/// v1: a second local House on the same hex may hello_ok (minimal copy-with-consent).
pub fn handle_hello(ledger: &mut ShardLedger, env: &Envelope) -> HelloSession {
    if env.check_proto(PROTOCOL_REV).is_err() {
        return HelloSession::No {
            code: RejectCode::Proto,
            reason: Some(format!("want {PROTOCOL_ID} rev {PROTOCOL_REV}")),
        };
    }
    if env.kind != "hello" {
        return HelloSession::No {
            code: RejectCode::Proto,
            reason: Some(format!("expected hello got {}", env.kind)),
        };
    }
    let body: HelloBody = match serde_json::from_value(env.body.clone()) {
        Ok(b) => b,
        Err(_) => HelloBody {
            protocol_rev: PROTOCOL_REV,
            consent_copy: false,
        },
    };
    let outcome = attempt_join(body.consent_copy, false, body.protocol_rev);
    match authority_after_join(&outcome) {
        AuthorityMode::Online => {
            if let Err(code) = seat_house(ledger, &env.house) {
                return HelloSession::No {
                    code,
                    reason: Some("soft cap reached".into()),
                };
            }
            // Minimal copy-with-consent: adopt house face when empty-ish / first seat.
            if !env.house.is_empty() {
                ledger.snapshot.house.house_id = env.house.clone();
            }
            HelloSession::Ok {
                house: env.house.clone(),
                presence: ledger.presence.clone(),
            }
        }
        AuthorityMode::Offline => {
            let code = match outcome {
                crate::hex_join::JoinOutcome::CopyDeniedStayOffline { code } => code,
                crate::hex_join::JoinOutcome::ProtoStayOffline => RejectCode::Proto,
                _ => RejectCode::CopyDenied,
            };
            HelloSession::No {
                code,
                reason: Some("hello_no".into()),
            }
        }
    }
}

/// Build wire reply envelope for hello session.
pub fn hello_reply_envelope(hex: &str, house: &str, session: &HelloSession, ts_ms: u64) -> Envelope {
    match session {
        HelloSession::Ok { presence, .. } => Envelope::new(
            ShardKind::HelloOk.as_str(),
            hex,
            house,
            0,
            ts_ms,
            json!({ "presence": presence, "protocol_rev": PROTOCOL_REV }),
        ),
        HelloSession::No { code, reason } => Envelope::new(
            ShardKind::HelloNo.as_str(),
            hex,
            house,
            0,
            ts_ms,
            serde_json::to_value(RejectBody {
                code: *code,
                reason: reason.clone(),
            })
            .unwrap_or(json!({})),
        ),
    }
}

/// Apply a client op envelope; return apply / reject wire envelope. Persist caller writes snapshot.
pub fn apply_client_op(ledger: &mut ShardLedger, env: &Envelope, ts_ms: u64) -> Envelope {
    let outcome = apply_envelope(ledger, env);
    match outcome {
        ApplyOutcome::Applied { seq, op } => Envelope::new(
            ShardKind::Apply.as_str(),
            &ledger.hex_id,
            &env.house,
            seq,
            ts_ms,
            json!({ "op": op, "seq": seq }),
        ),
        ApplyOutcome::Rejected { code, reason } => Envelope::new(
            ShardKind::Reject.as_str(),
            &ledger.hex_id,
            &env.house,
            env.seq,
            ts_ms,
            serde_json::to_value(RejectBody { code, reason }).unwrap_or(json!({})),
        ),
    }
}

/// Presence count = real houses.len() only — never invent.
pub fn presence_house_count(presence: &Presence) -> usize {
    presence.houses.len()
}

/// Unseat a house on client drop; book/house snapshot stay intact.
pub fn client_drop(ledger: &mut ShardLedger, house: &str) -> LeaveOutcome {
    ledger.presence.houses.retain(|h| h != house);
    leave_or_drop(ledger.snapshot.clone())
}

/// Snapshot + book survive drop (fixture helper).
pub fn drop_keeps_book_and_house(before: &Snapshot, after_drop: &LeaveOutcome) -> bool {
    after_drop.mode == AuthorityMode::Offline
        && !after_drop.login_wall
        && after_drop.snapshot.book == before.book
        && after_drop.snapshot.house == before.house
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex_protocol::{WellFace, ENVELOPE_V};
    use crate::hex_shard_apply::fixture_snapshot;

    #[test]
    fn refuse_public_and_wildcard_bind() {
        assert!(matches!(
            validate_listen_bind("0.0.0.0:7788"),
            Err(ListenRefuse::Wildcard(_))
        ));
        assert!(matches!(
            validate_listen_bind("[::]:7788"),
            Err(ListenRefuse::Wildcard(_))
        ));
        assert!(matches!(
            validate_listen_bind("8.8.8.8:7788"),
            Err(ListenRefuse::PublicOrNonLoopback(_))
        ));
        assert!(matches!(
            validate_listen_bind("192.168.1.10:7788"),
            Err(ListenRefuse::PublicOrNonLoopback(_))
        ));
    }

    #[test]
    fn accept_loopback_bind() {
        let a = validate_listen_bind("127.0.0.1:7788").expect("loopback");
        assert!(a.ip().is_loopback());
        assert_eq!(a.port(), 7788);
        let b = validate_listen_bind("127.0.0.1:0").expect("ephemeral");
        assert!(b.ip().is_loopback());
    }

    #[test]
    fn powrush_net_off_default_no_sockets() {
        assert_eq!(parse_powrush_net_from(None), PowrushNet::Off);
        assert_eq!(parse_powrush_net_from(Some("off")), PowrushNet::Off);
        assert_eq!(parse_powrush_net_from(Some("ON")), PowrushNet::Off);
        assert_eq!(parse_powrush_net_from(Some("true")), PowrushNet::Off);
        let off = PowrushNet::Off;
        assert!(!off.may_outbound_ws());
        assert!(!off.opens_listen_socket());
        assert!(!off.title_online_enabled());
    }

    #[test]
    fn powrush_net_localhost_outbound_only_title_grey() {
        let n = parse_powrush_net_from(Some("localhost"));
        assert_eq!(n, PowrushNet::Localhost);
        assert!(n.may_outbound_ws());
        assert!(!n.opens_listen_socket());
        assert!(!n.title_online_enabled());
        assert_eq!(default_localhost_shard_url(), "ws://127.0.0.1:7788");
    }

    #[test]
    fn hello_ok_seats_house_presence_real() {
        let mut ledger = ShardLedger::from_snapshot("hex_local_0", fixture_snapshot("hex_local_0"), 0);
        let env = Envelope::new(
            "hello",
            "hex_local_0",
            "h-a",
            1,
            1,
            json!({ "protocol_rev": 1, "consent_copy": true }),
        );
        let s = handle_hello(&mut ledger, &env);
        assert!(matches!(s, HelloSession::Ok { .. }));
        assert_eq!(presence_house_count(&ledger.presence), 1);
        assert_eq!(ledger.presence.houses, vec!["h-a".to_string()]);

        // Second local House on same hex may hello_ok (v1).
        let env2 = Envelope::new(
            "hello",
            "hex_local_0",
            "h-b",
            2,
            2,
            json!({ "protocol_rev": 1, "consent_copy": true }),
        );
        let s2 = handle_hello(&mut ledger, &env2);
        assert!(matches!(s2, HelloSession::Ok { .. }));
        assert_eq!(presence_house_count(&ledger.presence), 2);
    }

    #[test]
    fn hello_no_without_consent() {
        let mut ledger = ShardLedger::from_snapshot("hex_local_0", fixture_snapshot("hex_local_0"), 0);
        let env = Envelope::new(
            "hello",
            "hex_local_0",
            "h-a",
            1,
            1,
            json!({ "protocol_rev": 1, "consent_copy": false }),
        );
        let s = handle_hello(&mut ledger, &env);
        assert!(matches!(
            s,
            HelloSession::No {
                code: RejectCode::CopyDenied,
                ..
            }
        ));
        assert_eq!(presence_house_count(&ledger.presence), 0);
    }

    #[test]
    fn take_on_glowing_applies_once() {
        let mut ledger = ShardLedger::from_snapshot("hex_local_0", fixture_snapshot("hex_local_0"), 0);
        ledger.well = WellFace::Glowing;
        let env = Envelope {
            v: ENVELOPE_V,
            pid: PROTOCOL_ID.into(),
            kind: "take".into(),
            hex: "hex_local_0".into(),
            house: "h-fixture".into(),
            seq: 1,
            ts_ms: 1,
            body: json!({}),
        };
        let reply = apply_client_op(&mut ledger, &env, 1);
        assert_eq!(reply.kind, "apply");
        assert_eq!(ledger.snapshot.climate.tons_moved, 1);
    }

    #[test]
    fn take_on_tired_rejects_no_take() {
        let mut ledger = ShardLedger::from_snapshot("hex_local_0", fixture_snapshot("hex_local_0"), 0);
        ledger.well = WellFace::Resting;
        let env = Envelope::new("take", "hex_local_0", "h-fixture", 1, 1, json!({}));
        let reply = apply_client_op(&mut ledger, &env, 1);
        assert_eq!(reply.kind, "reject");
        let body: RejectBody = serde_json::from_value(reply.body).unwrap();
        assert_eq!(body.code, RejectCode::NoTake);
    }

    #[test]
    fn kill_shard_mid_session_client_offline_house_intact() {
        let mut ledger = ShardLedger::from_snapshot("hex_local_0", fixture_snapshot("hex_local_0"), 0);
        ledger.snapshot.book.hour_two_held = true;
        ledger.snapshot.house.house_name = "Oak".into();
        let _ = seat_house(&mut ledger, "h-oak");
        let before = ledger.snapshot.clone();
        let leave = client_drop(&mut ledger, "h-oak");
        assert!(drop_keeps_book_and_house(&before, &leave));
        assert!(!ledger.presence.houses.iter().any(|h| h == "h-oak"));
        assert_eq!(leave.snapshot.house.house_name, "Oak");
        assert!(leave.snapshot.book.hour_two_held);
    }

    #[test]
    fn listen_loopback_unparked_client_door_still_quiet() {
        assert!(listen_loopback_enabled());
        assert!(!PowrushNet::Off.opens_listen_socket());
        assert!(!PowrushNet::Localhost.opens_listen_socket());
    }
}
