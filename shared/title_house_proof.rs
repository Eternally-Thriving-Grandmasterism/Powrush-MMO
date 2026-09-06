//! S0/S2 title + Continue + House name proof (v23.2.50)
//!
//! Reuses stranger_loop_proof fixtures. No login wall. Lethal false until L3.
//! Contact: info@Rathor.ai

use crate::house_name::{continue_cue, local_persist_present, HouseName, UNNAMED};
use crate::stranger_loop_proof::{
    climate_week_fixture, hour_three_held_fixture, hour_two_held_fixture, peace_fixture,
};
use crate::hour_two::HourTwoPack;
use crate::shard_climate::ShardClimate;
use crate::shard_standing::ShardStanding;

/// First run: Play reaches Hands without House name.
pub fn first_run_may_enter_hands() -> bool {
    let house = HouseName::default();
    !house.blocks_hands() && !house.resolved
}

/// Continue path: hour pack + climate + standing + house label restore.
pub fn continue_restore_bundle() -> (HourTwoPack, ShardClimate, ShardStanding, HouseName) {
    let pack = hour_three_held_fixture();
    let (climate, standing, _week) = climate_week_fixture();
    let mut house = HouseName::default();
    house.confirm("Continue House");
    (pack, climate, standing, house)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_run_no_name_wall_reaches_hands() {
        assert!(first_run_may_enter_hands());
        let house = HouseName::default();
        assert_eq!(house.display_name(), UNNAMED);
        // No Continue cue forces a name — Play is free
        assert!(continue_cue(false, &house).is_none());
    }

    #[test]
    fn continue_restores_book_climate_standing_path_flags() {
        let (pack, climate, standing, house) = continue_restore_bundle();
        assert!(pack.complete);
        assert!(pack.hour_three_complete);
        assert_eq!(pack.line(true), "Hour three held · the book is yours");

        let pack2 = HourTwoPack::from_json(&serde_json::to_string(&pack).unwrap());
        let climate2 = ShardClimate::from_json(&climate.to_json().unwrap()).unwrap();
        let standing2 = ShardStanding::from_json(&standing.to_json().unwrap()).unwrap();
        let house2 = HouseName::from_json(&house.to_json().unwrap()).unwrap();

        assert!(pack2.complete);
        assert!(pack2.hour_three_complete);
        assert!(climate2.harmony > 0.0);
        assert!(!standing2.declared_lethal);
        assert_eq!(house2.display_name(), "Continue House");
        let cue = continue_cue(pack2.complete, &house2).unwrap();
        assert!(cue.contains("Continue House"));
        assert!(cue.contains("yard remembers"));
        assert!(local_persist_present(true, true, true, house2.resolved));
    }

    #[test]
    fn lethal_still_false_until_l3_on_fresh_peace() {
        let (pack, _c, standing, _w) = peace_fixture();
        assert!(!standing.declared_lethal);
        assert!(!pack.hour_three_complete);
        let house = HouseName::default();
        assert!(!house.blocks_hands());
        // Hour-two held, still no lethal without book+L3
        let h2 = hour_two_held_fixture();
        let (_c2, standing2, _) = climate_week_fixture();
        assert!(h2.complete);
        assert!(!h2.hour_three_complete);
        assert!(!standing2.declared_lethal);
        assert!(!standing2.declare_lethal(false));
    }

    #[test]
    fn skip_keeps_progress_unnamed() {
        let pack = hour_two_held_fixture();
        let mut house = HouseName::default();
        house.skip();
        assert_eq!(house.display_name(), UNNAMED);
        assert!(pack.complete);
        // Skip must not erase hour-two held
        let again = HourTwoPack::from_json(&serde_json::to_string(&pack).unwrap());
        assert!(again.complete);
        assert_eq!(again.line(true), "Hour two held · the yard remembers");
    }
}
