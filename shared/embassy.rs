//! Embassy + blueprints — Slice 8 (v23.2.12)
//!
//! After the Proof Pack unlocks, the embassy lamp is live. E Request seat.
//! One local seat. Blueprints copy the unlocked recipes. Not +DPS.
//! Contact: info@Rathor.ai

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

    pub fn line(&self) -> String {
        if self.entries.is_empty() {
            "Blueprints · none yet".into()
        } else {
            let names: Vec<_> = self.entries.iter().map(|r| r.label()).collect();
            format!("Blueprints · {}", names.join(" + "))
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

impl Embassy {
    pub fn ensure_lamp(&mut self, pack: &ProofPack) {
        if !pack.unlocked() {
            return;
        }
        self.lamp_live = true;
        self.book = BlueprintBook::from_pack(pack);
        if self.last_line.is_empty() {
            self.last_line = "Embassy lamp · E Request seat".into();
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
            "Seated — one embassy seat · {}",
            self.book.line()
        );
        "seated"
    }

    pub fn slab_line(&self) -> String {
        if !self.lamp_live {
            return "Embassy lamp waits on the Proof Pack".into();
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

    #[test]
    fn pack_unlocks_blueprints_and_seat() {
        let mut fab = Fabricator::default();
        fab.craft_next();
        fab.craft_next();
        fab.craft_next();
        assert!(fab.pack.unlocked());
        let mut e = Embassy::default();
        e.ensure_lamp(&fab.pack);
        assert!(e.lamp_live);
        assert_eq!(e.book.entries.len(), 2);
        assert_eq!(e.request_seat(), "seated");
        assert!(e.seated);
        assert!(e.last_line.contains("MendSpool"));
        assert!(e.last_line.contains("LaneCrate"));
    }

    #[test]
    fn no_seat_before_pack() {
        let mut e = Embassy::default();
        e.ensure_lamp(&ProofPack::default());
        assert!(!e.lamp_live);
        assert_eq!(e.request_seat(), "idle");
    }
}
