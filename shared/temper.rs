//! Temper / Lumen / Ward types — MERCY_TEMPER_PROGRESSION T0 §4 / CARD H-2026-09-11-T1
//!
//! Slice T1: shared types + unit tests only. No client verb, no fabricator bind,
//! no satchel chrome. Fail temper → resting; never delete. Online grey.
//! Contact: info@Rathor.ai

use serde::{Deserialize, Serialize};

/// Feature gate for later slices (T2+). Hour 1–3 unchanged while false.
pub const TEMPER_LOOP_ENABLED: bool = true;

pub const TEMPER_MAX: u8 = 9;
pub const MERCY_VALENCE_FLOOR: f64 = 0.999;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolTier {
    Hands = 0,
    TendHook = 1,
    LaneCrateMk1 = 2,
    ClimatePick = 3,
    MendSpindle = 4,
    HarmonyLoom = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WardKind {
    /// Turtle: reduce stress inflicted on wells (not PvP).
    Shell,
    /// Flow allocation feels cleaner (shorter recover).
    Current,
    /// Reserve mends a neighbour faster.
    Reserve,
    /// First-take contest stays fair (Mira path).
    Dawn,
    /// Embassy blueprint page chance (no mall).
    Book,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Lumen {
    pub index: u8,
    pub ward: Option<WardKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Lineage {
    /// Who tended / crafted. Not a market identity. Not a soul token.
    pub steward_label: String,
    pub restored_hex_at_craft: u32,
    pub proof_pack: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemperedItem {
    pub id: u64,
    pub tier: ToolTier,
    /// 0..=TEMPER_MAX
    pub temper: u8,
    /// len == lumen_slots(temper) after successful apply
    pub lumens: Vec<Lumen>,
    /// true after failed temper; blocks next temper
    pub resting: bool,
    pub lineage: Lineage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemperError {
    ToolResting,
    AlreadyNine,
    NoEmptyLumen,
    /// Cannot extract harder from a stressed well.
    WellStressed,
}

pub fn lumen_slots(temper: u8) -> u8 {
    match temper {
        0..=2 => 0,
        3..=5 => 1,
        6..=8 => 2,
        9 => 3,
        _ => 0,
    }
}

pub fn can_temper(item: &TemperedItem) -> Result<(), TemperError> {
    if item.resting {
        return Err(TemperError::ToolResting);
    }
    if item.temper >= TEMPER_MAX {
        return Err(TemperError::AlreadyNine);
    }
    Ok(())
}

/// Failure never deletes the item — only sets `resting`.
pub fn apply_temper(item: &mut TemperedItem, success: bool) {
    if success {
        item.temper = (item.temper + 1).min(TEMPER_MAX);
        let want = lumen_slots(item.temper) as usize;
        while item.lumens.len() < want {
            item.lumens.push(Lumen {
                index: item.lumens.len() as u8,
                ward: None,
            });
        }
        item.resting = false;
    } else {
        item.resting = true;
    }
}

pub fn seat_ward(item: &mut TemperedItem, ward: WardKind) -> Result<(), TemperError> {
    let slot = item
        .lumens
        .iter_mut()
        .find(|l| l.ward.is_none())
        .ok_or(TemperError::NoEmptyLumen)?;
    slot.ward = Some(ward);
    Ok(())
}

pub fn unseat_ward(item: &mut TemperedItem, index: u8) -> Option<WardKind> {
    item.lumens
        .iter_mut()
        .find(|l| l.index == index)
        .and_then(|l| l.ward.take())
}

/// Temper copy refuses mall / P2W / gold / Market / cash-shop potency language.
pub fn temper_copy_is_honest(s: &str) -> bool {
    let low = s.to_lowercase();
    !low.contains("gold")
        && !low.contains("mall")
        && !low.contains("p2w")
        && !low.contains("pay to win")
        && !low.contains("cash shop")
        && !low.contains("cash-shop")
        && !low.contains("market")
        && !low.contains("nft")
        && !low.contains("price")
        && !low.contains("sell")
}

impl TemperedItem {
    /// Hands-tier blank tool: temper 0, no lumens. Hour 1 path unchanged.
    pub fn hands(id: u64, steward_label: impl Into<String>) -> Self {
        Self {
            id,
            tier: ToolTier::Hands,
            temper: 0,
            lumens: Vec::new(),
            resting: false,
            lineage: Lineage {
                steward_label: steward_label.into(),
                restored_hex_at_craft: 0,
                proof_pack: false,
            },
        }
    }

    /// Short satchel-facing label used by honesty tests (display is T3).
    pub fn temper_line(&self) -> String {
        let slots = lumen_slots(self.temper);
        let seated = self.lumens.iter().filter(|l| l.ward.is_some()).count();
        if self.resting {
            format!("+{} · resting", self.temper)
        } else {
            format!("+{} · Lumen {}/{}", self.temper, seated, slots)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hook(temper: u8) -> TemperedItem {
        let mut item = TemperedItem::hands(1, "stranger");
        item.tier = ToolTier::TendHook;
        item.temper = temper;
        let want = lumen_slots(temper) as usize;
        item.lumens = (0..want)
            .map(|i| Lumen {
                index: i as u8,
                ward: None,
            })
            .collect();
        item
    }

    #[test]
    fn lumen_slots_at_thresholds() {
        assert_eq!(lumen_slots(0), 0);
        assert_eq!(lumen_slots(2), 0);
        assert_eq!(lumen_slots(3), 1);
        assert_eq!(lumen_slots(5), 1);
        assert_eq!(lumen_slots(6), 2);
        assert_eq!(lumen_slots(8), 2);
        assert_eq!(lumen_slots(9), 3);
    }

    #[test]
    fn fail_temper_sets_resting_does_not_change_temper_or_drop() {
        let mut item = hook(2);
        let before = item.temper;
        apply_temper(&mut item, false);
        assert!(item.resting);
        assert_eq!(item.temper, before);
        assert_eq!(item.tier, ToolTier::TendHook);
        assert!(can_temper(&item).is_err());
        assert_eq!(can_temper(&item), Err(TemperError::ToolResting));
    }

    #[test]
    fn success_temper_opens_slots_and_never_exceeds_nine() {
        let mut item = hook(2);
        apply_temper(&mut item, true);
        assert_eq!(item.temper, 3);
        assert_eq!(item.lumens.len(), 1);
        assert!(!item.resting);

        item.temper = 8;
        item.lumens = (0..lumen_slots(8) as usize)
            .map(|i| Lumen {
                index: i as u8,
                ward: None,
            })
            .collect();
        apply_temper(&mut item, true);
        assert_eq!(item.temper, 9);
        assert_eq!(item.lumens.len(), 3);

        assert_eq!(can_temper(&item), Err(TemperError::AlreadyNine));
        apply_temper(&mut item, true);
        assert_eq!(item.temper, TEMPER_MAX);
        assert_eq!(item.lumens.len(), 3);
    }

    #[test]
    fn seat_ward_fails_when_full_unseat_returns_ward() {
        let mut item = hook(3);
        assert_eq!(item.lumens.len(), 1);
        seat_ward(&mut item, WardKind::Shell).expect("empty lumen");
        assert_eq!(item.lumens[0].ward, Some(WardKind::Shell));
        assert_eq!(
            seat_ward(&mut item, WardKind::Current),
            Err(TemperError::NoEmptyLumen)
        );
        assert_eq!(unseat_ward(&mut item, 0), Some(WardKind::Shell));
        assert_eq!(item.lumens[0].ward, None);
        assert_eq!(unseat_ward(&mut item, 0), None);
        assert_eq!(unseat_ward(&mut item, 9), None);
    }

    #[test]
    fn hands_tier_exists_temper_zero_no_lumens() {
        let item = TemperedItem::hands(42, "stranger");
        assert_eq!(item.tier, ToolTier::Hands);
        assert_eq!(item.temper, 0);
        assert!(item.lumens.is_empty());
        assert!(!item.resting);
        assert_eq!(lumen_slots(item.temper), 0);
        assert!(can_temper(&item).is_ok());
    }

    #[test]
    fn serde_round_trip_satchel_loadable() {
        let mut item = hook(6);
        seat_ward(&mut item, WardKind::Dawn).unwrap();
        let json = serde_json::to_string(&item).expect("serialize");
        let back: TemperedItem = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, item);
        assert_eq!(back.lumens.len(), 2);
        assert_eq!(back.lumens[0].ward, Some(WardKind::Dawn));
    }

    #[test]
    fn temper_loop_flag_enabled() {
        assert!(TEMPER_LOOP_ENABLED);
    }

    #[test]
    fn refuse_mall_p2w_gold_copy() {
        let hands_line = TemperedItem::hands(1, "stranger").temper_line();
        let honest = [
            "Tend Hook +3 · Lumen 1/1 · Ward: Shell",
            "Temper +1",
            "resting",
            "Shell Ward",
            hands_line.as_str(),
        ];
        for sample in &honest {
            assert!(temper_copy_is_honest(sample), "got {sample}");
        }
        let refuse = [
            "buy temper stone at the mall",
            "gold for +9",
            "P2W gem pack",
            "pay to win ward",
            "cash shop turtle",
            "list on Market",
            "NFT lineage soul",
        ];
        for sample in &refuse {
            assert!(!temper_copy_is_honest(sample), "should refuse {sample}");
        }
    }
}
