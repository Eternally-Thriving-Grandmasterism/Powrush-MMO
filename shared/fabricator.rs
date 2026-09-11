//! Fabricator manufacture leg — Proof Pack (CREDIT_RESERVE · H-2026-09-11-D2)
//!
//! Still-frame: plant → MendSpool (repair) → LaneCrate (logi) → Proof Pack.
//! Civic proof, not gear power. Reserve spent here is repair-rights credit —
//! never gold, sell, price, ticker, or Market. Recipes are not +DPS.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

const PLACE_COST: f32 = 1.0;
const CRAFT_COST: f32 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Recipe {
    MendSpool,
    LaneCrate,
}

impl Recipe {
    pub fn label(self) -> &'static str {
        match self {
            Recipe::MendSpool => "MendSpool",
            Recipe::LaneCrate => "LaneCrate",
        }
    }

    /// Civic role on the manufacture leg — repair or logi, never DPS.
    pub fn civic_role(self) -> &'static str {
        match self {
            Recipe::MendSpool => "repair",
            Recipe::LaneCrate => "logi",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProofPack {
    pub repair: bool,
    pub logi: bool,
}

impl ProofPack {
    pub fn unlocked(&self) -> bool {
        self.repair && self.logi
    }

    pub fn line(&self) -> String {
        if self.unlocked() {
            "Proof Pack · manufacture unlocked (repair + logi)".into()
        } else {
            format!(
                "Proof Pack · manufacture · repair {} · logi {}",
                if self.repair { "yes" } else { "no" },
                if self.logi { "yes" } else { "no" }
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fabricator {
    pub planted: bool,
    /// Repair-rights credit (Reserve). Not gold / sell / price.
    pub reserve: f32,
    pub pack: ProofPack,
    pub last_line: String,
}

impl Default for Fabricator {
    fn default() -> Self {
        Self {
            planted: false,
            reserve: 2.0,
            pack: ProofPack::default(),
            last_line: String::new(),
        }
    }
}

/// Manufacture-leg copy stays civic proof + repair-rights — never gold / Market / DPS.
pub fn manufacture_copy_is_honest(s: &str) -> bool {
    let low = s.to_lowercase();
    !low.contains("gold")
        && !low.contains("market")
        && !low.contains("price")
        && !low.contains("sell")
        && !low.contains("ticker")
        && !low.contains("dps")
        && !low.contains("damage")
        && !low.contains("kill")
}

impl Fabricator {
    /// Reserve as repair-rights credit — never a gold balance.
    pub fn reserve_credit_line(&self) -> String {
        format!("Reserve {:.1} · repair-rights credit", self.reserve)
    }

    pub fn plant(&mut self) -> &'static str {
        if self.planted {
            return "idle";
        }
        if self.reserve < PLACE_COST {
            self.last_line = "Repair-rights Reserve too thin to plant a fabricator".into();
            return "starved";
        }
        self.reserve -= PLACE_COST;
        self.planted = true;
        self.last_line =
            "Fabricator live — manufacture: MendSpool, then LaneCrate".into();
        "planted"
    }

    pub fn craft(&mut self, recipe: Recipe) -> &'static str {
        if !self.planted {
            self.last_line = "Plant the fabricator first".into();
            return "unplanted";
        }
        if self.reserve < CRAFT_COST {
            self.last_line = "Repair-rights Reserve too thin to run a recipe".into();
            return "starved";
        }
        match recipe {
            Recipe::MendSpool if self.pack.repair => return "idle",
            Recipe::LaneCrate if self.pack.logi => return "idle",
            _ => {}
        }
        self.reserve -= CRAFT_COST;
        match recipe {
            Recipe::MendSpool => {
                self.pack.repair = true;
                self.last_line = "MendSpool ran — repair-rights to mend".into();
            }
            Recipe::LaneCrate => {
                self.pack.logi = true;
                self.last_line = "LaneCrate ran — the lane holds a crate".into();
            }
        }
        if self.pack.unlocked() {
            self.last_line =
                "Proof Pack unlocked — manufacture: repair + logi".into();
            return "unlocked";
        }
        "crafted"
    }

    pub fn craft_next(&mut self) -> &'static str {
        if !self.planted {
            return self.plant();
        }
        if !self.pack.repair {
            return self.craft(Recipe::MendSpool);
        }
        if !self.pack.logi {
            return self.craft(Recipe::LaneCrate);
        }
        self.last_line = self.pack.line();
        "idle"
    }

    pub fn slab_line(&self) -> String {
        if !self.planted {
            return "Q plant a fabricator · Reserve repair-rights".into();
        }
        format!(
            "Q next recipe · {} · {}",
            self.reserve_credit_line(),
            self.pack.line()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plant_then_two_recipes_unlock_pack() {
        let mut f = Fabricator::default();
        assert_eq!(f.craft_next(), "planted");
        assert_eq!(f.craft_next(), "crafted");
        assert!(f.pack.repair);
        assert_eq!(f.craft_next(), "unlocked");
        assert!(f.pack.unlocked());
        assert!(
            f.last_line.contains("Proof Pack unlocked"),
            "got {}",
            f.last_line
        );
        assert!(f.last_line.contains("manufacture"), "got {}", f.last_line);
        assert!(manufacture_copy_is_honest(&f.last_line));
    }

    #[test]
    fn recipes_are_not_dps() {
        for name in [Recipe::MendSpool.label(), Recipe::LaneCrate.label()] {
            let n = name.to_lowercase();
            assert!(!n.contains("dps"));
            assert!(!n.contains("damage"));
            assert!(!n.contains("kill"));
            assert!(manufacture_copy_is_honest(name));
        }
        assert_eq!(Recipe::MendSpool.civic_role(), "repair");
        assert_eq!(Recipe::LaneCrate.civic_role(), "logi");
    }

    #[test]
    fn cannot_craft_before_plant() {
        let mut f = Fabricator::default();
        assert_eq!(f.craft(Recipe::MendSpool), "unplanted");
        assert!(!f.pack.repair);
    }

    /// Manufacture still-frame: plant → MendSpool → LaneCrate → Proof Pack.
    /// Credit is repair-rights Reserve — never gold / Market / sell / DPS.
    #[test]
    fn manufacture_leg_refuses_gold_market_dps() {
        let mut f = Fabricator::default();
        let mut samples: Vec<String> = vec![
            Recipe::MendSpool.label().into(),
            Recipe::LaneCrate.label().into(),
            Recipe::MendSpool.civic_role().into(),
            Recipe::LaneCrate.civic_role().into(),
            f.slab_line(),
            f.reserve_credit_line(),
            f.pack.line(),
        ];

        assert_eq!(f.plant(), "planted");
        samples.push(f.last_line.clone());
        samples.push(f.slab_line());
        samples.push(f.reserve_credit_line());

        assert_eq!(f.craft(Recipe::MendSpool), "crafted");
        samples.push(f.last_line.clone());
        assert!(f.last_line.contains("repair-rights"), "got {}", f.last_line);

        assert_eq!(f.craft(Recipe::LaneCrate), "unlocked");
        samples.push(f.last_line.clone());
        samples.push(f.pack.line());
        samples.push(f.slab_line());
        samples.push(f.reserve_credit_line());

        assert!(f.pack.unlocked());
        assert!(f.pack.line().contains("manufacture"), "got {}", f.pack.line());
        assert!(
            f.reserve_credit_line().contains("repair-rights"),
            "got {}",
            f.reserve_credit_line()
        );
        assert!(
            f.slab_line().contains("repair-rights"),
            "got {}",
            f.slab_line()
        );

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
            assert!(manufacture_copy_is_honest(sample), "got {sample}");
        }
    }

    #[test]
    fn reserve_reads_as_repair_rights_credit() {
        let f = Fabricator::default();
        let line = f.reserve_credit_line();
        assert!(line.contains("Reserve"), "got {line}");
        assert!(line.contains("repair-rights"), "got {line}");
        assert!(line.contains("credit"), "got {line}");
        assert!(manufacture_copy_is_honest(&line));
        assert!(!line.to_lowercase().contains("gold"));
    }
}
