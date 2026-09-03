//! Fabricator + Proof Pack — Slice 7 (v23.2.11)
//!
//! Two recipes: MendSpool (repair) and LaneCrate (logi). Not +DPS.
//! Proof Pack unlocks when both have run. Local graph.
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
            "Proof Pack · repair + logi unlocked".into()
        } else {
            format!(
                "Proof Pack · repair {} · logi {}",
                if self.repair { "yes" } else { "no" },
                if self.logi { "yes" } else { "no" }
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fabricator {
    pub planted: bool,
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

impl Fabricator {
    pub fn plant(&mut self) -> &'static str {
        if self.planted {
            return "idle";
        }
        if self.reserve < PLACE_COST {
            self.last_line = "Reserve too thin to plant a fabricator".into();
            return "starved";
        }
        self.reserve -= PLACE_COST;
        self.planted = true;
        self.last_line = "Fabricator live — Q MendSpool, then LaneCrate".into();
        "planted"
    }

    pub fn craft(&mut self, recipe: Recipe) -> &'static str {
        if !self.planted {
            self.last_line = "Plant the fabricator first".into();
            return "unplanted";
        }
        if self.reserve < CRAFT_COST {
            self.last_line = "Reserve too thin to run a recipe".into();
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
                self.last_line = "MendSpool ran — the right to mend".into();
            }
            Recipe::LaneCrate => {
                self.pack.logi = true;
                self.last_line = "LaneCrate ran — the lane holds a crate".into();
            }
        }
        if self.pack.unlocked() {
            self.last_line = "The graph unlocked — repair and logi".into();
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
            return "Q plant a fabricator (after the crate arrives)".into();
        }
        format!(
            "Q next recipe · reserve {:.1} · {}",
            self.reserve,
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
        assert!(f.last_line.contains("graph unlocked"));
    }

    #[test]
    fn recipes_are_not_dps() {
        for name in [Recipe::MendSpool.label(), Recipe::LaneCrate.label()] {
            let n = name.to_lowercase();
            assert!(!n.contains("dps"));
            assert!(!n.contains("damage"));
            assert!(!n.contains("kill"));
        }
    }

    #[test]
    fn cannot_craft_before_plant() {
        let mut f = Fabricator::default();
        assert_eq!(f.craft(Recipe::MendSpool), "unplanted");
        assert!(!f.pack.repair);
    }
}
