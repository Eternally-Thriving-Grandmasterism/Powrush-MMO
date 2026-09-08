//! Hour pack — Slice 22 (v23.2.29) + Hour three civic (v23.2.35)
//!
//! One file remembers the Charter door and the civic book.
//! Old SpaceSession-only and hour-two-only JSON still load.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::embassy::Embassy;
use crate::fabricator::Fabricator;
use crate::infra_spill::InfraWitness;
use crate::ledger_bind::{ContractState, LedgerBoard};
use crate::space_law::SpaceSession;
use crate::vertical_factory::VerticalFactory;

/// On-disk hour state. Flattened session keeps 23.2.28 files readable.
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
    pub fabricator: Fabricator,
    #[serde(default)]
    pub embassy: Embassy,
    #[serde(default)]
    pub complete: bool,
    #[serde(default)]
    pub hour_three_complete: bool,
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

    /// Hour three is a house latch. Once the book is held, a hex stub embassy
    /// (not seated) must not clear it — travel writes hex climate, not the house book.
    pub fn mark_hour_three(&mut self) {
        if self.hour_three_complete {
            return;
        }
        self.hour_three_complete = self.fabricator.pack.unlocked() && self.embassy.seated;
    }

    /// Hex stub embassy must not replace the house seat or drop the book flag.
    /// `prior` is the on-disk house pack before this persist.
    pub fn keep_house_book_over_hex_stub(&mut self, prior: &HourTwoPack) {
        if prior.hour_three_complete {
            self.hour_three_complete = true;
        }
        if prior.embassy.seated && !self.embassy.seated {
            self.embassy = prior.embassy.clone();
        }
        if prior.complete {
            self.complete = true;
        }
    }

    pub fn line(&self, door_ready: bool) -> &'static str {
        if self.hour_three_complete {
            "Hour three held · the book is yours"
        } else if self.complete {
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
        assert!(!pack.hour_three_complete);
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

    #[test]
    fn proof_pack_and_seat_mark_hour_three() {
        let mut pack = HourTwoPack::default();
        assert_eq!(pack.fabricator.craft_next(), "planted");
        assert_eq!(pack.fabricator.craft_next(), "crafted");
        assert_eq!(pack.fabricator.craft_next(), "unlocked");
        pack.embassy.ensure_lamp(&pack.fabricator.pack);
        assert_eq!(pack.embassy.request_seat(), "seated");
        pack.mark_hour_three();
        assert!(pack.hour_three_complete);
        assert_eq!(pack.line(true), "Hour three held · the book is yours");
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
        let mut p = pack;
        p.mark_complete();
        assert!(!p.complete);
        p.mark_hour_three();
        assert!(!p.hour_three_complete);
    }

    #[test]
    fn hex_stub_embassy_does_not_unlatch_house_book() {
        let prior = crate::stranger_loop_proof::hour_three_held_fixture();
        assert!(prior.hour_three_complete);
        assert!(prior.embassy.seated);
        let mut live = prior.clone();
        live.embassy = crate::embassy::Embassy {
            lamp_live: false,
            seated: false,
            ..Default::default()
        };
        live.mark_hour_three();
        assert!(
            live.hour_three_complete,
            "mark_hour_three is a house latch — stub seat must not drop the book"
        );
        live.keep_house_book_over_hex_stub(&prior);
        assert!(live.embassy.seated);
        assert_eq!(live.embassy, prior.embassy);
        assert!(live.complete);
    }
}
