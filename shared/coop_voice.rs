//! Co-op Voice — Slice 4 (v23.2.8)
//!
//! Quorum card on the beacon. Flat org votes in-world, not only Discord.
//! Local graph. Live multi-seat AOI waits on the parked server.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceTopic {
    Reserve,
    War,
    Flag,
}

impl VoiceTopic {
    pub fn label(self) -> &'static str {
        match self {
            VoiceTopic::Reserve => "reserve",
            VoiceTopic::War => "war",
            VoiceTopic::Flag => "flag",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuorumCard {
    pub topic: VoiceTopic,
    pub motion: String,
    pub aye: u8,
    pub nay: u8,
    pub seats: u8,
    pub voters: Vec<String>,
    pub settled: Option<bool>,
}

impl QuorumCard {
    pub fn tutorial() -> Self {
        Self {
            topic: VoiceTopic::Reserve,
            motion: "Open the depot to the lane".into(),
            aye: 0,
            nay: 0,
            seats: 1,
            voters: Vec::new(),
            settled: None,
        }
    }

    pub fn needed(&self) -> u8 {
        (self.seats / 2) + 1
    }

    pub fn vote(&mut self, who: &str, aye: bool) -> &'static str {
        if self.settled.is_some() {
            return "idle";
        }
        if self.voters.iter().any(|v| v == who) {
            return "already";
        }
        self.voters.push(who.to_string());
        if aye {
            self.aye = self.aye.saturating_add(1);
        } else {
            self.nay = self.nay.saturating_add(1);
        }
        if self.aye >= self.needed() {
            self.settled = Some(true);
            return "carried";
        }
        if self.nay >= self.needed() {
            self.settled = Some(false);
            return "failed";
        }
        if aye {
            "aye"
        } else {
            "nay"
        }
    }

    pub fn line(&self) -> String {
        match self.settled {
            Some(true) => format!("Carried · {} — {}", self.topic.label(), self.motion),
            Some(false) => format!("Failed · {} — {}", self.topic.label(), self.motion),
            None => format!(
                "Voice · {} — {} · E aye · 2 nay · {}/{} seats",
                self.topic.label(),
                self.motion,
                self.voters.len(),
                self.seats
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CoopVoice {
    pub cards: Vec<QuorumCard>,
    pub last_line: String,
}

impl CoopVoice {
    pub fn ensure_tutorial(&mut self) {
        if self.cards.is_empty() {
            self.cards.push(QuorumCard::tutorial());
            self.last_line = "Beacon live — a Voice card waits".into();
        }
    }

    pub fn open_card(&self) -> Option<&QuorumCard> {
        self.cards.iter().find(|c| c.settled.is_none())
    }

    pub fn open_card_mut(&mut self) -> Option<&mut QuorumCard> {
        self.cards.iter_mut().find(|c| c.settled.is_none())
    }

    pub fn vote_local(&mut self, aye: bool) -> &'static str {
        let Some(card) = self.open_card_mut() else {
            self.last_line = "No Voice card on the beacon".into();
            return "idle";
        };
        let step = card.vote("local", aye);
        self.last_line = card.line();
        step
    }

    pub fn sash_line(&self) -> String {
        if let Some(card) = self.open_card() {
            card.line()
        } else if let Some(last) = self.cards.last() {
            last.line()
        } else {
            "G Voice · no card yet".into()
        }
    }

    pub fn beacon_line(&self) -> String {
        self.sash_line()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_seat_aye_carries() {
        let mut v = CoopVoice::default();
        v.ensure_tutorial();
        assert_eq!(v.vote_local(true), "carried");
        assert_eq!(v.open_card(), None);
        assert!(v.last_line.contains("Carried"));
        assert!(v.cards[0].settled == Some(true));
    }

    #[test]
    fn nay_fails_the_motion() {
        let mut v = CoopVoice::default();
        v.ensure_tutorial();
        assert_eq!(v.vote_local(false), "failed");
        assert_eq!(v.cards[0].settled, Some(false));
    }

    #[test]
    fn cannot_double_vote() {
        let mut card = QuorumCard {
            topic: VoiceTopic::Flag,
            motion: "Raise the yard flag".into(),
            aye: 0,
            nay: 0,
            seats: 3,
            voters: Vec::new(),
            settled: None,
        };
        assert_eq!(card.vote("a", true), "aye");
        assert_eq!(card.vote("a", true), "already");
        assert_eq!(card.aye, 1);
    }

    #[test]
    fn no_card_is_idle() {
        let mut v = CoopVoice::default();
        assert_eq!(v.vote_local(true), "idle");
    }
}
