//! Embassy seat / eligibility after Proof Pack (CREDIT_RESERVE · H-2026-09-11-D3)
//!
//! Still-frame: Proof Pack unlocked → embassy lamp live → E Request seat →
//! seated with blueprints. Civic end of the manufacture chain — eligibility
//! after proof, not Online lobby, Market stall, gold, or +DPS.
//! One local seat. Lamp waits on the pack. Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::fabricator::{ProofPack, Recipe};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BlueprintBook {
    pub entries: Vec<Recipe>,
}

impl BlueprintBook {
    pub fn from_pack(pack: &ProofPack) -> Self {
        let mut entries = Vec::new();
        if pack.repair {
            entries.push(Recipe::MendSpool);
        }
        if pack.logi {
            entries.push(Recipe::LaneCrate);
        }
        Self { entries }
    }

    /// Civic eligibility book after manufacture proof — not a Market catalog.
    pub fn line(&self) -> String {
        if self.entries.is_empty() {
            "Blueprints · none yet · civic book waits on Proof Pack".into()
        } else {
            let names: Vec<_> = self.entries.iter().map(|r| r.label()).collect();
            format!(
                "Blueprints · civic eligibility · {}",
                names.join(" + ")
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Embassy {
    pub lamp_live: bool,
    pub seated: bool,
    pub book: BlueprintBook,
    pub last_line: String,
}

/// Embassy copy is civic eligibility after manufacture proof — never gold /
/// Market / sell / price / ticker / DPS / kill / Online.
pub fn embassy_copy_is_honest(s: &str) -> bool {
    let low = s.to_lowercase();
    !low.contains("gold")
        && !low.contains("market")
        && !low.contains("price")
        && !low.contains("sell")
        && !low.contains("ticker")
        && !low.contains("dps")
        && !low.contains("damage")
        && !low.contains("kill")
        && !low.contains("online")
}

impl Embassy {
    pub fn ensure_lamp(&mut self, pack: &ProofPack) {
        if !pack.unlocked() {
            return;
        }
        self.lamp_live = true;
        self.book = BlueprintBook::from_pack(pack);
        if self.last_line.is_empty() {
            self.last_line =
                "Embassy lamp · E Request seat · civic after manufacture proof".into();
        }
    }

    pub fn request_seat(&mut self) -> &'static str {
        if !self.lamp_live {
            return "idle";
        }
        if self.seated {
            return "idle";
        }
        self.seated = true;
        self.last_line = format!(
            "Seated — one embassy seat · civic eligibility · {}",
            self.book.line()
        );
        "seated"
    }

    pub fn slab_line(&self) -> String {
        if !self.lamp_live {
            return "Embassy lamp waits on the Proof Pack · civic seat after manufacture"
                .into();
        }
        if self.seated {
            return self.last_line.clone();
        }
        self.last_line.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fabricator::Fabricator;

    fn unlocked_pack() -> ProofPack {
        let mut fab = Fabricator::default();
        fab.craft_next();
        fab.craft_next();
        fab.craft_next();
        assert!(fab.pack.unlocked());
        fab.pack
    }

    #[test]
    fn pack_unlocks_blueprints_and_seat() {
        let pack = unlocked_pack();
        let mut e = Embassy::default();
        e.ensure_lamp(&pack);
        assert!(e.lamp_live);
        assert_eq!(e.book.entries.len(), 2);
        assert!(e.last_line.contains("civic after manufacture proof"));
        assert_eq!(e.request_seat(), "seated");
        assert!(e.seated);
        assert!(e.last_line.contains("civic eligibility"));
        assert!(e.last_line.contains("MendSpool"));
        assert!(e.last_line.contains("LaneCrate"));
        assert_eq!(e.request_seat(), "idle", "one seat only");
        assert!(embassy_copy_is_honest(&e.last_line));
        assert!(embassy_copy_is_honest(&e.slab_line()));
        assert!(embassy_copy_is_honest(&e.book.line()));
    }

    #[test]
    fn no_seat_before_pack() {
        let mut e = Embassy::default();
        e.ensure_lamp(&ProofPack::default());
        assert!(!e.lamp_live);
        assert_eq!(e.request_seat(), "idle");
        let wait = e.slab_line();
        assert!(wait.contains("waits on the Proof Pack"));
        assert!(wait.contains("civic seat after manufacture"));
        assert!(embassy_copy_is_honest(&wait));
    }

    /// Still-frame: Proof Pack → lamp → Request seat → seated with blueprints
    /// as civic eligibility — never gold / Market / Online / DPS.
    #[test]
    fn embassy_seat_refuses_gold_market_online_dps() {
        let pack = unlocked_pack();
        let mut e = Embassy::default();
        let mut samples: Vec<String> = vec![
            BlueprintBook::default().line(),
            e.slab_line(),
        ];

        e.ensure_lamp(&pack);
        samples.push(e.last_line.clone());
        samples.push(e.slab_line());
        samples.push(e.book.line());
        assert!(e.book.line().contains("civic eligibility"));
        assert!(e.last_line.contains("Request seat"));

        assert_eq!(e.request_seat(), "seated");
        samples.push(e.last_line.clone());
        samples.push(e.slab_line());
        samples.push(e.book.line());

        assert!(e.seated);
        assert!(e.last_line.contains("one embassy seat"));
        assert!(e.last_line.contains("civic eligibility"));
        assert!(e.slab_line().contains("civic eligibility"));

        for sample in &samples {
            let low = sample.to_lowercase();
            assert!(!low.contains("gold"), "got {sample}");
            assert!(!low.contains("market"), "got {sample}");
            assert!(!low.contains("price"), "got {sample}");
            assert!(!low.contains("sell"), "got {sample}");
            assert!(!low.contains("ticker"), "got {sample}");
            assert!(!low.contains("dps"), "got {sample}");
            assert!(!low.contains("damage"), "got {sample}");
            assert!(!low.contains("kill"), "got {sample}");
            assert!(!low.contains("online"), "got {sample}");
            assert!(embassy_copy_is_honest(sample), "got {sample}");
        }
    }

    #[test]
    fn seat_is_civic_eligibility_not_lobby() {
        let pack = unlocked_pack();
        let mut e = Embassy::default();
        e.ensure_lamp(&pack);
        let lamp = e.slab_line();
        assert!(lamp.contains("civic after manufacture proof"), "got {lamp}");
        e.request_seat();
        let seated = e.slab_line();
        assert!(seated.contains("civic eligibility"), "got {seated}");
        assert!(e.book.line().contains("civic eligibility"));
        assert!(!lamp.to_lowercase().contains("online"));
        assert!(!seated.to_lowercase().contains("lobby"));
    }
}
