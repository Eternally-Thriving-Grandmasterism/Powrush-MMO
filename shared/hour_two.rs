//! Hour-two pack — Slice 22 (v23.2.29)
//!
//! One file remembers the Charter door: hex, House, offline extractor,
//! and Ledger Bind/Escort. Old `powrush_hour_two.json` (SpaceSession-only)
//! still loads. No Embassy. Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::infra_spill::InfraWitness;
use crate::ledger_bind::{ContractState, LedgerBoard};
use crate::space_law::SpaceSession;
use crate::vertical_factory::VerticalFactory;

/// On-disk hour-two state. Flattened session keeps 23.2.28 files readable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HourTwoPack {
    #[serde(flatten)]
    pub session: SpaceSession,
    #[serde(default)]
    pub factory: VerticalFactory,
    #[serde(default)]
    pub witness: InfraWitness,
    #[serde(default)]
    pub board: LedgerBoard,
    #[serde(default)]
    pub complete: bool,
}

impl HourTwoPack {
    pub fn from_json(raw: &str) -> Self {
        if let Ok(pack) = serde_json::from_str::<HourTwoPack>(raw) {
            return pack;
        }
        if let Ok(session) = serde_json::from_str::<SpaceSession>(raw) {
            return Self {
                session,
                ..Default::default()
            };
        }
        Self::default()
    }

    pub fn ledger_settled(&self) -> bool {
        self.board
            .open()
            .map(|c| c.state == ContractState::Settled)
            .unwrap_or(false)
    }

    /// Hour two is held when the House is live, the spill was seen, and Bind settled.
    pub fn mark_complete(&mut self) {
        self.complete = self.session.charter_skin_live()
            && self.witness.seen
            && self.ledger_settled();
    }

    pub fn line(&self, door_ready: bool) -> &'static str {
        if self.complete {
            "Hour two held · the yard remembers"
        } else if self.session.charter_skin_live() && self.witness.seen {
            "L Ledger · E Bind then escort"
        } else {
            self.session.hour_two_line(door_ready)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space_law::{CharterKind, HexFlag};

    #[test]
    fn old_session_json_still_loads() {
        let raw = r#"{"charter_id":"house-local","hex":"Frontier","kind":"House","warrant":{"h":0.0,"i":0.0,"c":0.0,"f":0.0,"x":0.0,"repair":0.0,"return_cargo":0.0,"council":0.0,"tend_spill":0.0}}"#;
        let pack = HourTwoPack::from_json(raw);
        assert_eq!(pack.session.hex, HexFlag::Frontier);
        assert_eq!(pack.session.charter_id.as_deref(), Some("house-local"));
        assert!(pack.session.charter_skin_live());
        assert!(!pack.complete);
        assert!(!pack.factory.founded);
    }

    #[test]
    fn settle_marks_hour_two_held() {
        let mut pack = HourTwoPack::default();
        assert_eq!(pack.line(true), "Tab Charter — the ridge is open");
        assert!(pack.session.take_frontier_ridge());
        pack.session.charter_id = Some("house-local".into());
        pack.session.kind = CharterKind::House;
        pack.factory.found_house();
        pack.witness.ensure_offline_extractor();
        pack.witness.seen = true;
        pack.board.ensure_i2("local-i2");
        assert_eq!(pack.act_until_settled(), "settled");
        pack.mark_complete();
        assert!(pack.complete);
        assert_eq!(pack.line(true), "Hour two held · the yard remembers");
    }

    impl HourTwoPack {
        fn act_until_settled(&mut self) -> &'static str {
            let mut last = "idle";
            for _ in 0..8 {
                last = self.board.act_local();
                if last == "settled" {
                    break;
                }
            }
            last
        }
    }

    #[test]
    fn peace_json_is_not_complete() {
        let pack = HourTwoPack::from_json("{}");
        assert_eq!(pack.session.hex, HexFlag::Peace);
        pack.clone().mark_complete();
        let mut p = pack;
        p.mark_complete();
        assert!(!p.complete);
    }
}
