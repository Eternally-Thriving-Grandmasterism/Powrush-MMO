//! P1 stranger-loop persist / flag proof (v23.2.47)
//!
//! Fixture + JSON round-trip. No new verbs. Peace boot stays non-lethal.
//! Contact: info@Rathor.ai

use crate::hour_two::HourTwoPack;
use crate::shard_climate::ShardClimate;
use crate::shard_standing::ShardStanding;
use crate::space_law::{CharterKind, HexFlag};
use crate::week_audit::WeekAudit;

/// Peace-hour / fresh fixture — lethal must stay false.
pub fn peace_fixture() -> (
    HourTwoPack,
    ShardClimate,
    ShardStanding,
    WeekAudit,
) {
    (
        HourTwoPack::default(),
        ShardClimate::default(),
        ShardStanding::default(),
        WeekAudit::default(),
    )
}

/// Scripted session after Hour two Settled (Charter + spill + Bind).
pub fn hour_two_held_fixture() -> HourTwoPack {
    let mut pack = HourTwoPack::default();
    assert!(pack.session.take_frontier_ridge());
    pack.session.charter_id = Some("house-local".into());
    pack.session.kind = CharterKind::House;
    pack.factory.found_house();
    pack.witness.ensure_offline_extractor();
    pack.witness.seen = true;
    pack.board.ensure_i2("local-i2");
    for _ in 0..8 {
        if pack.board.act_local() == "settled" {
            break;
        }
    }
    pack.mark_complete();
    pack
}

/// Scripted session after Proof Pack + Embassy seat (the book).
pub fn hour_three_held_fixture() -> HourTwoPack {
    let mut pack = hour_two_held_fixture();
    assert_eq!(pack.fabricator.craft_next(), "planted");
    assert_eq!(pack.fabricator.craft_next(), "crafted");
    assert_eq!(pack.fabricator.craft_next(), "unlocked");
    pack.embassy.ensure_lamp(&pack.fabricator.pack);
    assert_eq!(pack.embassy.request_seat(), "seated");
    pack.mark_hour_three();
    pack
}

/// Climate + standing + week after care / flow / mend / lane (disk shape).
pub fn climate_week_fixture() -> (ShardClimate, ShardStanding, WeekAudit) {
    let mut climate = ShardClimate::default();
    let mut standing = ShardStanding::default();
    climate.on_care_tend();
    standing.on_care_tend();
    climate.on_flow();
    standing.on_flow();
    climate.on_mend();
    standing.on_mend();
    climate.on_lane();
    standing.on_lane();
    let mut week = WeekAudit::default();
    week.sync_from_climate(climate.tons_moved, climate.restored_count);
    (climate, standing, week)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peace_fresh_fixture_lethal_false() {
        let (pack, climate, standing, week) = peace_fixture();
        assert_eq!(pack.session.hex, HexFlag::Peace);
        assert!(!pack.complete);
        assert!(!pack.hour_three_complete);
        assert!(!standing.declared_lethal);
        assert!(climate.harmony > 0.0);
        assert!(climate.stress >= 0.0);
        assert_eq!(week.tons_moved, 0);
        assert_eq!(week.restored_count, 0);
        let line = week.slab_line();
        assert!(!line.to_lowercase().contains("kill"));
    }

    #[test]
    fn hour_two_held_roundtrip() {
        let pack = hour_two_held_fixture();
        assert!(pack.complete);
        assert!(pack.ledger_settled());
        assert_eq!(pack.line(true), "Hour two held · the yard remembers");
        assert!(!pack.hour_three_complete);
        let raw = serde_json::to_string_pretty(&pack).unwrap();
        let loaded = HourTwoPack::from_json(&raw);
        assert!(loaded.complete);
        assert!(!loaded.hour_three_complete);
        assert!(loaded.session.charter_skin_live());
        assert_eq!(loaded.line(true), "Hour two held · the yard remembers");
    }

    #[test]
    fn hour_three_book_held_roundtrip() {
        let pack = hour_three_held_fixture();
        assert!(pack.complete);
        assert!(pack.hour_three_complete);
        assert!(pack.fabricator.pack.unlocked());
        assert!(pack.embassy.seated);
        assert_eq!(pack.line(true), "Hour three held · the book is yours");
        let raw = serde_json::to_string_pretty(&pack).unwrap();
        let loaded = HourTwoPack::from_json(&raw);
        assert!(loaded.hour_three_complete);
        assert!(loaded.complete);
        assert_eq!(loaded.line(true), "Hour three held · the book is yours");
    }

    #[test]
    fn climate_blob_harmony_stress_roundtrip() {
        let (climate, _standing, _week) = climate_week_fixture();
        assert!(climate.harmony > 0.0);
        assert!(climate.stress >= 0.0);
        let raw = climate.to_json().unwrap();
        assert!(raw.contains("harmony"));
        assert!(raw.contains("stress"));
        let loaded = ShardClimate::from_json(&raw).unwrap();
        assert!((loaded.harmony - climate.harmony).abs() < 1e-6);
        assert!((loaded.stress - climate.stress).abs() < 1e-6);
        assert_eq!(loaded.tons_moved, climate.tons_moved);
        assert_eq!(loaded.restored_count, climate.restored_count);
    }

    #[test]
    fn standing_lethal_false_until_l3() {
        let (_c, mut standing, _w) = climate_week_fixture();
        assert!(!standing.declared_lethal);
        // Quit/rerun shape before book
        let raw = standing.to_json().unwrap();
        let mut loaded = ShardStanding::from_json(&raw).unwrap();
        assert!(!loaded.declared_lethal);
        assert!(!loaded.declare_lethal(false));
        assert!(!loaded.declared_lethal);
        // After book only
        assert!(loaded.declare_lethal(true));
        assert!(loaded.declared_lethal);
        let raw2 = loaded.to_json().unwrap();
        let again = ShardStanding::from_json(&raw2).unwrap();
        assert!(again.declared_lethal);
        assert_eq!(again.tariff_paid, 1);
    }

    #[test]
    fn week_tons_plus_restored_not_kills() {
        let (climate, _s, week) = climate_week_fixture();
        assert!(week.tons_moved >= 1);
        assert!(week.restored_count >= 1);
        assert_eq!(week.tons_moved, climate.tons_moved);
        assert_eq!(week.restored_count, climate.restored_count);
        let line = week.slab_line();
        assert!(line.contains("tons"));
        assert!(line.contains("restored"));
        assert!(!line.to_lowercase().contains("kill"));
        let loaded = WeekAudit::from_json(&week.to_json().unwrap()).unwrap();
        assert_eq!(loaded.tons_moved, week.tons_moved);
        assert_eq!(loaded.restored_count, week.restored_count);
    }

    #[test]
    fn quit_rerun_keeps_flags_full_path() {
        let pack = hour_three_held_fixture();
        let (climate, standing, week) = climate_week_fixture();
        assert!(!standing.declared_lethal);

        let pack2 = HourTwoPack::from_json(&serde_json::to_string(&pack).unwrap());
        let climate2 = ShardClimate::from_json(&climate.to_json().unwrap()).unwrap();
        let standing2 = ShardStanding::from_json(&standing.to_json().unwrap()).unwrap();
        let week2 = WeekAudit::from_json(&week.to_json().unwrap()).unwrap();

        assert!(pack2.complete);
        assert!(pack2.hour_three_complete);
        assert!(climate2.harmony > 0.0);
        assert!(!standing2.declared_lethal);
        assert!(week2.tons_moved + week2.restored_count >= 2);
        // Peace boot of a *fresh* pack still silent on lethal
        let fresh = HourTwoPack::default();
        let fresh_standing = ShardStanding::default();
        assert!(!fresh.hour_three_complete);
        assert!(!fresh_standing.declared_lethal);
        assert_eq!(fresh.session.hex, HexFlag::Peace);
    }
}
