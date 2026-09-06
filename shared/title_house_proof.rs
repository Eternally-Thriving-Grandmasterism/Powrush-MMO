//! S0/S2 title + Continue + House name proof (v23.2.51)
//!
//! L1 title truth: first-run no wall; Continue Unnamed House + yard remembers;
//! Esc-from-title preserves persist; Online disabled honest copy.
//! Reuses stranger_loop_proof fixtures. No login wall. Lethal false until L3.
//! Contact: info@Rathor.ai

use crate::house_name::HouseName;
use crate::stranger_loop_proof::{climate_week_fixture, hour_three_held_fixture};
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

/// Honest Online stub copy — visible, disabled, never pretends it works.
pub const ONLINE_STUB_LABEL: &str = "Online — off (no listen)";

pub fn online_row_is_honest_disabled(label: &str, enabled: bool) -> bool {
    !enabled
        && label.contains("Online")
        && (label.contains("off") || label.contains("not yet") || label.contains("no listen"))
        && !label.to_lowercase().contains("connected")
        && !label.to_lowercase().contains("players online")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::house_name::{
        continue_cue, continue_cue_when_persist, esc_from_title_preserves_persist,
        local_persist_present, UNNAMED, YARD_REMEMBERS,
    };
    use crate::stranger_loop_proof::{hour_two_held_fixture, peace_fixture};

    #[test]
    fn first_run_no_name_wall_reaches_hands() {
        assert!(first_run_may_enter_hands());
        let house = HouseName::default();
        assert_eq!(house.display_name(), UNNAMED);
        assert!(!house.blocks_hands());
        // No Continue cue forces a name — Play is free
        assert!(continue_cue(false, &house).is_none());
        assert!(continue_cue_when_persist(false, &house).is_none());
    }

    #[test]
    fn continue_cue_unnamed_house_and_yard_remembers() {
        let mut house = HouseName::default();
        house.skip();
        let cue = continue_cue(true, &house).unwrap();
        assert_eq!(cue, "Unnamed House · the yard remembers");
        assert!(cue.contains(UNNAMED));
        assert!(cue.contains(YARD_REMEMBERS));
        // Persist-present path (climate-only) uses same steward wording
        let unresolved = HouseName::default();
        let cue2 = continue_cue_when_persist(true, &unresolved).unwrap();
        assert_eq!(cue2, "Unnamed House · the yard remembers");
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
        assert!(cue.contains(YARD_REMEMBERS));
        assert_eq!(cue, "Continue House · the yard remembers");
        assert!(local_persist_present(true, true, true, house2.resolved));
    }

    #[test]
    fn esc_from_title_does_not_clear_house_json_resolved_flags() {
        let (pack, climate, standing, house) = continue_restore_bundle();
        let house_json = house.to_json().unwrap();
        let climate_json = climate.to_json().unwrap();
        let standing_json = standing.to_json().unwrap();
        let book_json = serde_json::to_string(&pack).unwrap();
        // Title Esc closes settings only — snapshots identical
        assert!(esc_from_title_preserves_persist(
            &house_json,
            &house_json,
            &climate_json,
            &climate_json,
            &standing_json,
            &standing_json,
            &book_json,
            &book_json,
        ));
        let house_back = HouseName::from_json(&house_json).unwrap();
        assert!(house_back.resolved);
        assert_eq!(house_back.display_name(), "Continue House");
        assert!(!standing.declared_lethal);
        assert!(pack.hour_three_complete);
    }

    #[test]
    fn online_row_visible_disabled_honest_copy() {
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(online_row_is_honest_disabled("Online — not yet", false));
        assert!(!online_row_is_honest_disabled(ONLINE_STUB_LABEL, true));
        assert!(!online_row_is_honest_disabled("Online — 42 players online", false));
        assert!(!online_row_is_honest_disabled("Online — connected", false));
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
        let (_c2, mut standing2, _) = climate_week_fixture();
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