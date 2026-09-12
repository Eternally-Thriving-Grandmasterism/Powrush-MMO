//! Fabricator manufacture leg — Proof Pack (CREDIT_RESERVE · H-2026-09-11-D2)
//! MERCY_TEMPER_PROGRESSION T2 · CARD H-2026-09-11-T2 — Tend Hook bind
//!
//! Still-frame: plant → MendSpool (repair) → LaneCrate (logi) → Proof Pack.
//! T2: Tend Hook recipe (1 MendSpool) + Temper +1 care via shared::temper.
//! Civic proof, not gear power. Reserve spent here is repair-rights credit —
//! never gold, sell, price, ticker, or Market. Recipes are not +DPS.
//! TEMPER_LOOP may stay off (Hour 1–3); recipe stays hidden until the flag.
//! Online grey. Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

use crate::temper::{
    apply_temper, can_temper, TemperedItem, TemperError, ToolTier, TEMPER_LOOP_ENABLED,
};

const PLACE_COST: f32 = 1.0;
const CRAFT_COST: f32 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Recipe {
    MendSpool,
    LaneCrate,
    /// T2 care tool — 1 MendSpool. Hidden while `TEMPER_LOOP_ENABLED` is false.
    TendHook,
}

impl Recipe {
    pub fn label(self) -> &'static str {
        match self {
            Recipe::MendSpool => "MendSpool",
            Recipe::LaneCrate => "LaneCrate",
            Recipe::TendHook => "Tend Hook",
        }
    }

    /// Civic role on the manufacture leg — repair, logi, or care; never DPS.
    pub fn civic_role(self) -> &'static str {
        match self {
            Recipe::MendSpool => "repair",
            Recipe::LaneCrate => "logi",
            Recipe::TendHook => "care",
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
    /// MendSpool care stock for Tend Hook. Civic `pack.repair` stays intact.
    #[serde(default)]
    pub spool_stock: u8,
    /// Last Tend Hook produced by `craft_tend_hook` / `Recipe::TendHook`.
    #[serde(default)]
    pub last_tempered: Option<TemperedItem>,
}

impl Default for Fabricator {
    fn default() -> Self {
        Self {
            planted: false,
            reserve: 2.0,
            pack: ProofPack::default(),
            last_line: String::new(),
            spool_stock: 0,
            last_tempered: None,
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
        && !low.contains("mall")
        && !low.contains("p2w")
        && !low.contains("pay to win")
}

/// T2 visibility: Tend Hook stays hidden while the temper loop flag is off.
pub fn tend_hook_recipe_visible() -> bool {
    TEMPER_LOOP_ENABLED
}

/// Temper +1 care. Never spends on a Stressed well. Fail → resting via temper APIs.
pub fn temper_care(
    item: &mut TemperedItem,
    well_stressed: bool,
    success: bool,
) -> Result<(), TemperError> {
    if well_stressed {
        return Err(TemperError::WellStressed);
    }
    can_temper(item)?;
    apply_temper(item, success);
    Ok(())
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
        if recipe == Recipe::TendHook {
            return match self.craft_tend_hook(1, "stranger") {
                Ok(_) => "crafted",
                Err(status) => status,
            };
        }
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
            Recipe::TendHook => unreachable!("Tend Hook handled above"),
            _ => {}
        }
        self.reserve -= CRAFT_COST;
        match recipe {
            Recipe::MendSpool => {
                self.pack.repair = true;
                self.spool_stock = self.spool_stock.saturating_add(1);
                self.last_line = "MendSpool ran — repair-rights to mend".into();
            }
            Recipe::LaneCrate => {
                self.pack.logi = true;
                self.last_line = "LaneCrate ran — the lane holds a crate".into();
            }
            Recipe::TendHook => unreachable!("Tend Hook handled above"),
        }
        if self.pack.unlocked() {
            self.last_line =
                "Proof Pack unlocked — manufacture: repair + logi".into();
            return "unlocked";
        }
        "crafted"
    }

    /// Craft a Tend Hook from 1 MendSpool. Cost is the care spool only — no gold.
    /// Hidden while `TEMPER_LOOP_ENABLED` is false. Does not clear Proof Pack flags.
    pub fn craft_tend_hook(
        &mut self,
        id: u64,
        steward_label: impl Into<String>,
    ) -> Result<TemperedItem, &'static str> {
        self.craft_tend_hook_gated(tend_hook_recipe_visible(), id, steward_label)
    }

    fn craft_tend_hook_gated(
        &mut self,
        visible: bool,
        id: u64,
        steward_label: impl Into<String>,
    ) -> Result<TemperedItem, &'static str> {
        if !visible {
            self.last_line = "Tend Hook recipe rests · Temper loop grey".into();
            return Err("hidden");
        }
        if !self.planted {
            self.last_line = "Plant the fabricator first".into();
            return Err("unplanted");
        }
        if self.spool_stock == 0 {
            self.last_line = "Tend Hook needs 1 MendSpool".into();
            return Err("missing_spool");
        }
        self.spool_stock -= 1;
        let mut item = TemperedItem::hands(id, steward_label);
        item.tier = ToolTier::TendHook;
        item.lineage.proof_pack = self.pack.unlocked();
        self.last_line = "Tend Hook crafted · care · Temper +0".into();
        self.last_tempered = Some(item.clone());
        Ok(item)
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
    use crate::temper::temper_copy_is_honest;

    #[test]
    fn plant_then_two_recipes_unlock_pack() {
        let mut f = Fabricator::default();
        assert_eq!(f.craft_next(), "planted");
        assert_eq!(f.craft_next(), "crafted");
        assert!(f.pack.repair);
        assert_eq!(f.spool_stock, 1);
        assert_eq!(f.craft_next(), "unlocked");
        assert!(f.pack.unlocked());
        assert_eq!(f.spool_stock, 1, "LaneCrate must not consume MendSpool stock");
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
        for name in [
            Recipe::MendSpool.label(),
            Recipe::LaneCrate.label(),
            Recipe::TendHook.label(),
        ] {
            let n = name.to_lowercase();
            assert!(!n.contains("dps"));
            assert!(!n.contains("damage"));
            assert!(!n.contains("kill"));
            assert!(manufacture_copy_is_honest(name));
        }
        assert_eq!(Recipe::MendSpool.civic_role(), "repair");
        assert_eq!(Recipe::LaneCrate.civic_role(), "logi");
        assert_eq!(Recipe::TendHook.civic_role(), "care");
    }

    #[test]
    fn cannot_craft_before_plant() {
        let mut f = Fabricator::default();
        assert_eq!(f.craft(Recipe::MendSpool), "unplanted");
        assert!(!f.pack.repair);
        assert_eq!(f.spool_stock, 0);
    }

    /// Manufacture still-frame: plant → MendSpool → LaneCrate → Proof Pack.
    /// Credit is repair-rights Reserve — never gold / Market / sell / DPS.
    #[test]
    fn manufacture_leg_refuses_gold_market_dps() {
        let mut f = Fabricator::default();
        let mut samples: Vec<String> = vec![
            Recipe::MendSpool.label().into(),
            Recipe::LaneCrate.label().into(),
            Recipe::TendHook.label().into(),
            Recipe::MendSpool.civic_role().into(),
            Recipe::LaneCrate.civic_role().into(),
            Recipe::TendHook.civic_role().into(),
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
            assert!(!low.contains("mall"), "got {sample}");
            assert!(!low.contains("p2w"), "got {sample}");
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

    #[test]
    fn tend_hook_hidden_while_temper_loop_off() {
        assert!(!TEMPER_LOOP_ENABLED);
        assert!(!tend_hook_recipe_visible());
        let mut f = Fabricator::default();
        assert_eq!(f.plant(), "planted");
        assert_eq!(f.craft(Recipe::MendSpool), "crafted");
        assert_eq!(f.spool_stock, 1);
        assert_eq!(f.craft_tend_hook(7, "stranger"), Err("hidden"));
        assert_eq!(f.craft(Recipe::TendHook), "hidden");
        assert_eq!(f.spool_stock, 1, "hidden recipe must not spend spool");
        assert!(f.last_tempered.is_none());
        assert!(f.last_line.contains("Temper loop grey"), "got {}", f.last_line);
        assert!(manufacture_copy_is_honest(&f.last_line));
        assert!(temper_copy_is_honest(&f.last_line));
        // Proof Pack path still intact
        assert_eq!(f.craft(Recipe::LaneCrate), "unlocked");
        assert!(f.pack.unlocked());
    }

    #[test]
    fn tend_hook_rejects_missing_spool_when_visible() {
        let mut f = Fabricator::default();
        assert_eq!(f.plant(), "planted");
        assert_eq!(
            f.craft_tend_hook_gated(true, 1, "stranger"),
            Err("missing_spool")
        );
        assert!(f.last_line.contains("MendSpool"), "got {}", f.last_line);
        assert!(manufacture_copy_is_honest(&f.last_line));
        assert!(f.pack.repair == false);
    }

    #[test]
    fn tend_hook_crafts_tempered_item_from_one_spool() {
        let mut f = Fabricator::default();
        assert_eq!(f.plant(), "planted");
        assert_eq!(f.craft(Recipe::MendSpool), "crafted");
        assert!(f.pack.repair);
        assert_eq!(f.spool_stock, 1);

        let item = f
            .craft_tend_hook_gated(true, 42, "stranger")
            .expect("Tend Hook from 1 MendSpool");
        assert_eq!(item.tier, ToolTier::TendHook);
        assert_eq!(item.temper, 0);
        assert!(item.lumens.is_empty());
        assert!(!item.resting);
        assert_eq!(item.id, 42);
        assert_eq!(item.lineage.steward_label, "stranger");
        assert_eq!(f.spool_stock, 0);
        assert!(f.pack.repair, "Proof Pack repair flag must stay");
        assert!(!f.pack.logi);
        assert_eq!(f.last_tempered.as_ref(), Some(&item));
        assert!(f.last_line.contains("Tend Hook"), "got {}", f.last_line);
        assert!(f.last_line.contains("Temper +0"), "got {}", f.last_line);
        assert!(manufacture_copy_is_honest(&f.last_line));
        assert!(temper_copy_is_honest(&f.last_line));
        assert!(temper_copy_is_honest(&item.temper_line()));

        // LaneCrate / Proof Pack still work after Tend Hook
        assert_eq!(f.craft(Recipe::LaneCrate), "unlocked");
        assert!(f.pack.unlocked());
    }

    #[test]
    fn temper_care_plus_one_and_refuses_stressed_well() {
        let mut f = Fabricator::default();
        f.plant();
        f.craft(Recipe::MendSpool);
        let mut item = f
            .craft_tend_hook_gated(true, 3, "steward")
            .expect("hook");
        assert_eq!(
            temper_care(&mut item, true, true),
            Err(TemperError::WellStressed)
        );
        assert_eq!(item.temper, 0);
        assert!(!item.resting);

        temper_care(&mut item, false, true).expect("Temper +1 care");
        assert_eq!(item.temper, 1);
        assert!(!item.resting);
        assert!(item.lumens.is_empty());

        temper_care(&mut item, false, false).expect("fail sets resting");
        assert!(item.resting);
        assert_eq!(item.temper, 1);
        assert_eq!(
            temper_care(&mut item, false, true),
            Err(TemperError::ToolResting)
        );
    }

    #[test]
    fn tend_hook_copy_refuses_mall_p2w_gold() {
        let honest = [
            Recipe::TendHook.label(),
            Recipe::TendHook.civic_role(),
            "Tend Hook crafted · care · Temper +0",
            "Tend Hook recipe rests · Temper loop grey",
            "Tend Hook needs 1 MendSpool",
            "Temper +1",
        ];
        for sample in &honest {
            assert!(manufacture_copy_is_honest(sample), "got {sample}");
            assert!(temper_copy_is_honest(sample), "got {sample}");
        }
        let refuse = [
            "Tend Hook gold pack",
            "mall temper stone",
            "P2W Tend Hook",
            "pay to win care",
        ];
        for sample in &refuse {
            assert!(
                !manufacture_copy_is_honest(sample) || !temper_copy_is_honest(sample),
                "should refuse {sample}"
            );
        }
    }
}
