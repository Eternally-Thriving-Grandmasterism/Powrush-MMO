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

// --- CARD LOOT-LEGIBLE-1 — shown standing, named finds, vendor law ---------

/// Shown standing of a tool, read from Temper `+N` and Lumen `a/b` only (F2).
/// Ward kind is a label, never a number. No roll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Standing {
    Worn,
    Kept,
    Proven,
    Storied,
}

/// Pure standing fn. Out-of-range `+N` reads as 0, mirroring `lumen_slots`.
pub fn shown_standing(temper: u8, seated: u8, slots: u8) -> Standing {
    let n = if temper > TEMPER_MAX { 0 } else { temper };
    match n + seated.min(slots) {
        0..=2 => Standing::Worn,
        3..=5 => Standing::Kept,
        6..=8 => Standing::Proven,
        _ => Standing::Storied,
    }
}

/// One colour-blind-legible face per standing: word, shape, plain sRGB (F6).
/// PROPOSAL for COURT; Sherif may veto. In-game only.
pub struct StandingFace {
    pub standing: Standing,
    pub word: &'static str,
    pub shape: &'static str,
    pub rgb: [f32; 3],
}

pub const STANDING_FACES: [StandingFace; 4] = [
    StandingFace { standing: Standing::Worn, word: "Worn", shape: "dot", rgb: [0.42, 0.40, 0.38] },
    StandingFace { standing: Standing::Kept, word: "Kept", shape: "triangle", rgb: [0.36, 0.62, 0.86] },
    StandingFace { standing: Standing::Proven, word: "Proven", shape: "diamond", rgb: [0.92, 0.70, 0.30] },
    StandingFace { standing: Standing::Storied, word: "Storied", shape: "star", rgb: [1.0, 0.97, 0.86] },
];

/// WCAG relative luminance (piecewise sRGB at 0.04045, Rec.709 weights). Approximate;
/// no colour-vision simulation is claimed.
pub fn standing_luminance(rgb: [f32; 3]) -> f32 {
    let lin = |c: f32| if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
    0.2126 * lin(rgb[0]) + 0.7152 * lin(rgb[1]) + 0.0722 * lin(rgb[2])
}

/// One named find (F5): one card, not a table. No weights, no odds.
pub struct NamedFind {
    pub name: &'static str,
    pub sentence: &'static str,
    pub role: &'static str,
    pub temper: u8,
    pub seated: u8,
    pub ward: Option<WardKind>,
}

impl NamedFind {
    pub fn standing(&self) -> Standing {
        shown_standing(self.temper, self.seated, lumen_slots(self.temper))
    }
}

/// Named finds, PROPOSAL for COURT. Drop weighting belongs to LOOT-DROPS-1.
pub const NAMED_FINDS: [NamedFind; 6] = [
    NamedFind { name: "Yard-Worn Tend Hook", sentence: "It tended the first well this yard restored.", role: "tend", temper: 3, seated: 1, ward: Some(WardKind::Shell) },
    NamedFind { name: "Steady Climate Pick", sentence: "It holds the weather line while a well recovers.", role: "climate", temper: 4, seated: 0, ward: None },
    NamedFind { name: "Dawn-Fair Lane Crate", sentence: "It carries the first take and keeps the contest fair.", role: "carry", temper: 5, seated: 1, ward: Some(WardKind::Dawn) },
    NamedFind { name: "Neighbour's Mend Spindle", sentence: "It mends the well next door before its own.", role: "mend", temper: 6, seated: 1, ward: Some(WardKind::Reserve) },
    NamedFind { name: "Quiet Harmony Loom", sentence: "It weaves the week's tending into one calm pattern.", role: "weave", temper: 8, seated: 2, ward: Some(WardKind::Current) },
    NamedFind { name: "Bookkeeper's Hook", sentence: "It keeps an embassy page in its Lumen.", role: "record", temper: 9, seated: 3, ward: Some(WardKind::Book) },
];

/// Vendor law: anything a vendor ever offers stands strictly below every named
/// find. There is no conversion between finds, tools or any currency.
pub const VENDOR_STANDING_CEILING: Standing = Standing::Worn;

// --- CARD UPGRADE-STAKES-1 — inactive Bold/Careful upgrade meter ----------
// Rules from the PATSAGi councils' 1:25 AM ET ruling under Sherif's design
// delegation (not Sherif's own words); Sherif may veto. Earned materials only,
// a fail never loses the item or a Lumen, and a full meter guarantees the
// upgrade. Nothing outside tests reads these. The existing resting lock is
// banked as TEMPER-RESTING-FIX-1; binding cost to a real material is banked as
// UPGRADE-COST-WIRE-1.

/// Basis points in one whole: odds and rolls live in 0..10000.
pub const UPGRADE_BPS: u16 = 10_000;
/// Meter units for a guaranteed upgrade. Proposed word: "Progress".
pub const UPGRADE_METER_FULL: u16 = 6;
pub const UPGRADE_PROGRESS_WORD: &str = "Progress";

/// Player-picked stance per attempt. Proposed words: "Bold", "Careful".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeStance {
    Bold,
    Careful,
}

impl UpgradeStance {
    pub const ALL: [UpgradeStance; 2] = [UpgradeStance::Bold, UpgradeStance::Careful];
    pub const fn word(self) -> &'static str {
        ["Bold", "Careful"][self as usize]
    }
    /// Odds per attempt in basis points.
    pub const fn odds_bps(self) -> u16 {
        [5_000, 2_500][self as usize]
    }
    /// Shown odds as a plain whole percent; equals the odds used.
    pub const fn odds_pct(self) -> u16 {
        self.odds_bps() / 100
    }
    /// Abstract earned-material units per attempt; spent from no stock here.
    pub const fn cost(self) -> u16 {
        [37, 30][self as usize]
    }
    /// Meter units a failed attempt adds.
    pub const fn fill(self) -> u16 {
        [2, 3][self as usize]
    }
    /// Most attempts to an upgrade: fails until the meter is full, then one sure attempt.
    pub const fn worst_case_attempts(self) -> u16 {
        UPGRADE_METER_FULL.div_ceil(self.fill()) + 1
    }
}

/// Upgrade meter, kept apart from `TemperedItem` so its saved shape is unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UpgradeMeter {
    pub progress: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeOutcome {
    /// Already at `TEMPER_MAX`; nothing spent.
    AtMax,
    /// Upgrade lands; the caller hands off as `apply_temper(item, true)`.
    Upgraded { cost: u16 },
    /// Materials spent, meter filled; the item is untouched.
    MeterFilled { cost: u16, progress: u16 },
}

/// One attempt with an injected roll in 0..10000. Never calls `can_temper` or
/// `apply_temper`, so a fail never sets `resting`.
pub fn attempt_upgrade(meter: &mut UpgradeMeter, item: &TemperedItem, stance: UpgradeStance, roll: u16) -> UpgradeOutcome {
    if item.temper >= TEMPER_MAX {
        return UpgradeOutcome::AtMax;
    }
    let cost = stance.cost();
    if meter.progress >= UPGRADE_METER_FULL || roll < stance.odds_bps() {
        meter.progress = 0;
        return UpgradeOutcome::Upgraded { cost };
    }
    meter.progress = (meter.progress + stance.fill()).min(UPGRADE_METER_FULL);
    UpgradeOutcome::MeterFilled { cost, progress: meter.progress }
}

/// Exact expected cost to an upgrade from an empty meter, as (numerator,
/// denominator): cost x sum over k < worst case of (1 - p)^k, scaled by 10000.
pub const fn upgrade_expected_cost(stance: UpgradeStance) -> (u128, u128) {
    let (w, q, b) = (stance.worst_case_attempts() as u32, (UPGRADE_BPS - stance.odds_bps()) as u128, UPGRADE_BPS as u128);
    let (mut k, mut num) = (0, 0u128);
    while k < w {
        num += q.pow(k) * b.pow(w - 1 - k);
        k += 1;
    }
    (stance.cost() as u128 * num, b.pow(w - 1))
}

const _: () = {
    let ((bn, bd), (cn, cd)) = (upgrade_expected_cost(UpgradeStance::Bold), upgrade_expected_cost(UpgradeStance::Careful));
    assert!(bn * cd == cn * bd);
    assert!(UpgradeStance::Bold.odds_bps() > UpgradeStance::Careful.odds_bps());
    assert!(UpgradeStance::Bold.cost() > UpgradeStance::Careful.cost());
    assert!(UpgradeStance::Bold.fill() < UpgradeStance::Careful.fill());
};

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

    // --- CARD LOOT-LEGIBLE-1 -------------------------------------------------

    const WARDS: [Option<WardKind>; 6] = [None, Some(WardKind::Shell), Some(WardKind::Current), Some(WardKind::Reserve), Some(WardKind::Dawn), Some(WardKind::Book)];

    #[test]
    fn loot_legible_standing_reads_only_temper_and_lumen() {
        let mut last = Standing::Worn;
        for n in 0..=TEMPER_MAX {
            let s = shown_standing(n, 0, lumen_slots(n));
            assert!(s >= last, "monotonic in +N at {n}");
            last = s;
            for w in WARDS {
                let mut item = TemperedItem::hands(1, "t");
                item.temper = n;
                item.lumens = (0..lumen_slots(n)).map(|i| Lumen { index: i, ward: w }).collect();
                let seated = item.lumens.iter().filter(|l| l.ward.is_some()).count() as u8;
                let expect = if w.is_some() { lumen_slots(n) } else { 0 };
                assert_eq!(seated, expect);
                assert_eq!(shown_standing(n, seated, lumen_slots(n)), shown_standing(n, expect, lumen_slots(n)));
            }
        }
        assert_eq!(shown_standing(0, 0, 0), Standing::Worn);
        assert_eq!(shown_standing(TEMPER_MAX, 3, 3), Standing::Storied);
        assert_eq!(shown_standing(10, 0, 0), Standing::Worn, "out of range reads as 0");
        assert_eq!(shown_standing(255, 9, 0), Standing::Worn, "seated never exceeds slots");
    }

    #[test]
    fn loot_legible_named_finds_are_valid_unique_one_sentence() {
        assert!(NAMED_FINDS.len() <= 6);
        for (i, f) in NAMED_FINDS.iter().enumerate() {
            assert!(f.temper <= TEMPER_MAX && f.seated <= lumen_slots(f.temper), "{}", f.name);
            assert!(NAMED_FINDS[..i].iter().all(|g| g.name != f.name), "dup {}", f.name);
            let marks = f.sentence.chars().filter(|c| matches!(c, '.' | '!' | '?')).count();
            assert!(marks == 1 && f.sentence.ends_with('.'), "{}", f.sentence);
            assert!(!f.role.is_empty() && !f.role.contains(' '));
        }
    }

    #[test]
    fn loot_legible_vendor_ceiling_below_every_find() {
        let min = NAMED_FINDS.iter().map(NamedFind::standing).min().expect("finds");
        assert!(VENDOR_STANDING_CEILING < min, "{VENDOR_STANDING_CEILING:?} < {min:?}");
        assert_eq!(min, Standing::Kept);
        let src = include_str!("temper.rs").split("#[cfg(test)]").next().expect("prefix");
        for banned in ["fn convert", "fn exchange", "fn trade", "fn buy", "fn sell_"] {
            assert!(!src.contains(banned), "no conversion fn: {banned}");
        }
    }

    #[test]
    fn loot_legible_faces_distinct_word_shape_luminance() {
        for (i, a) in STANDING_FACES.iter().enumerate() {
            assert_eq!(a.standing as usize, i);
            for b in &STANDING_FACES[i + 1..] {
                assert_ne!(a.word, b.word);
                assert_ne!(a.shape, b.shape);
                let gap = (standing_luminance(a.rgb) - standing_luminance(b.rgb)).abs();
                assert!(gap >= 0.15, "{} vs {} luminance gap {gap}", a.word, b.word);
            }
        }
        assert!(standing_luminance([1.0, 1.0, 1.0]) > 0.999 && standing_luminance([0.0; 3]) == 0.0);
    }

    #[test]
    fn loot_legible_player_copy_is_honest_and_never_says_loot() {
        let mut copy: Vec<&str> = STANDING_FACES.iter().flat_map(|f| [f.word, f.shape]).collect();
        for f in &NAMED_FINDS {
            copy.extend([f.name, f.sentence, f.role]);
        }
        for s in copy {
            assert!(temper_copy_is_honest(s), "{s}");
            let low = s.to_lowercase();
            for w in ["loot", "gold", "market", "price", "vendor"] {
                assert!(!low.contains(w), "{s} says {w}");
            }
        }
    }

    #[test]
    fn upgrade_stakes_equal_expected_cost_exact() {
        let (bn, bd) = upgrade_expected_cost(UpgradeStance::Bold);
        let (cn, cd) = upgrade_expected_cost(UpgradeStance::Careful);
        // Bold: 37 x (1 + 1/2 + 1/4 + 1/8) = 69.375; Careful: 30 x (1 + 3/4 + 9/16) = 69.375.
        assert_eq!(bn * cd, cn * bd, "tolerance zero");
        assert_eq!((bn * 1000 / bd, cn * 1000 / cd), (69_375, 69_375));
        assert_eq!(UpgradeStance::ALL.map(|s| s.worst_case_attempts()), [4, 3]);
    }

    #[test]
    fn upgrade_stakes_fail_streak_never_rests_and_meter_guarantees() {
        for s in UpgradeStance::ALL {
            let (mut item, mut meter) = (hook(3), UpgradeMeter::default());
            let before = item.clone();
            for n in 1..s.worst_case_attempts() {
                let out = attempt_upgrade(&mut meter, &item, s, UPGRADE_BPS - 1);
                assert_eq!(out, UpgradeOutcome::MeterFilled { cost: s.cost(), progress: (n * s.fill()).min(UPGRADE_METER_FULL) });
                assert_eq!(item, before, "fail leaves item, Lumens and resting alone");
            }
            assert_eq!(meter.progress, UPGRADE_METER_FULL);
            assert_eq!(attempt_upgrade(&mut meter, &item, s, UPGRADE_BPS - 1), UpgradeOutcome::Upgraded { cost: s.cost() });
            apply_temper(&mut item, true);
            assert!(!item.resting && item.temper == 4 && meter.progress == 0);
        }
    }

    #[test]
    fn upgrade_stakes_roll_is_injected_and_max_is_refused() {
        for s in UpgradeStance::ALL {
            let mut meter = UpgradeMeter::default();
            assert_eq!(attempt_upgrade(&mut meter, &hook(0), s, s.odds_bps() - 1), UpgradeOutcome::Upgraded { cost: s.cost() });
            assert!(matches!(attempt_upgrade(&mut meter, &hook(0), s, s.odds_bps()), UpgradeOutcome::MeterFilled { .. }));
            let mut full = UpgradeMeter { progress: UPGRADE_METER_FULL };
            assert_eq!(attempt_upgrade(&mut full, &hook(TEMPER_MAX), s, 0), UpgradeOutcome::AtMax);
            assert_eq!(full.progress, UPGRADE_METER_FULL, "refusal spends nothing");
        }
    }

    #[test]
    fn upgrade_stakes_shown_odds_and_words_are_honest() {
        assert_eq!(UpgradeStance::ALL.map(|s| (s.word(), s.odds_pct())), [("Bold", 50), ("Careful", 25)]);
        assert!(UpgradeStance::ALL.iter().all(|s| s.odds_pct() * 100 == s.odds_bps()));
        for w in [UpgradeStance::Bold.word(), UpgradeStance::Careful.word(), UPGRADE_PROGRESS_WORD] {
            assert!(temper_copy_is_honest(w), "{w}");
        }
    }
}
