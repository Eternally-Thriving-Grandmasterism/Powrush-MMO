//! Pause / Ledger face — S3 (v23.2.52)
//!
//! House name · week tons + restored · lethal only if already declared.
//! Shown on I (satchel) and/or L (Ledger sash). No second HUD.
//! No talent tree · no peer count · no fake online.
//! Contact: info@Rathor.ai

use crate::house_name::{HouseName, UNNAMED};
use crate::week_audit::WeekAudit;

/// Same clause standing uses when lethal is already declared.
pub const LETHAL_DECLARED_LINE: &str = "lethal is declared — the hex will remember";

/// Compose the Pause/Ledger face lines (house · week · optional lethal).
pub fn face_lines(house_display: &str, week: &WeekAudit, declared_lethal: bool) -> String {
    let house = {
        let t = house_display.trim();
        if t.is_empty() {
            UNNAMED
        } else {
            t
        }
    };
    let mut out = String::new();
    out.push_str(house);
    out.push('\n');
    out.push_str(&week.slab_line());
    if declared_lethal {
        out.push('\n');
        out.push_str(LETHAL_DECLARED_LINE);
    }
    out
}

/// Face from persisted HouseName + week audit + lethal flag.
pub fn face_from(house: &HouseName, week: &WeekAudit, declared_lethal: bool) -> String {
    face_lines(house.display_name(), week, declared_lethal)
}

/// Steward-honest: no peer / online / talent / kill chrome.
pub fn face_is_steward_honest(face: &str) -> bool {
    let low = face.to_lowercase();
    !low.contains("peer")
        && !low.contains("online")
        && !low.contains("players")
        && !low.contains("talent")
        && !low.contains("kill")
        && !low.contains("dps")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::house_name::HouseName;
    use crate::week_audit::WeekAudit;

    fn week(tons: u32, restored: u32) -> WeekAudit {
        let mut w = WeekAudit::default();
        w.sync_from_climate(tons, restored);
        w
    }

    #[test]
    fn face_includes_house_and_tons_restored() {
        let mut house = HouseName::default();
        house.confirm("Keep Yard");
        let face = face_from(&house, &week(3, 2), false);
        assert!(face.contains("Keep Yard"), "got {face}");
        assert!(face.contains("3 tons"), "got {face}");
        assert!(face.contains("2 restored"), "got {face}");
        assert!(face.contains("this week"), "got {face}");
        assert!(!face.contains(LETHAL_DECLARED_LINE));
        assert!(face_is_steward_honest(&face));
    }

    #[test]
    fn unnamed_house_when_skipped() {
        let mut house = HouseName::default();
        house.skip();
        let face = face_from(&house, &week(0, 0), false);
        assert!(face.starts_with(UNNAMED), "got {face}");
        assert!(face.contains("0 tons"));
        assert!(face.contains("0 restored"));
        assert!(!face.contains(LETHAL_DECLARED_LINE));
    }

    #[test]
    fn lethal_absent_when_false() {
        let face = face_lines(UNNAMED, &week(1, 1), false);
        assert!(!face.contains("lethal"));
        assert!(!face.contains(LETHAL_DECLARED_LINE));
        assert!(face_is_steward_honest(&face));
    }

    #[test]
    fn lethal_present_when_declared() {
        let face = face_lines("Named Keep", &week(4, 1), true);
        assert!(face.contains("Named Keep"));
        assert!(face.contains("4 tons"));
        assert!(face.contains("1 restored"));
        assert!(face.contains(LETHAL_DECLARED_LINE));
        assert!(face_is_steward_honest(&face));
    }

    #[test]
    fn no_peer_count_string() {
        let face = face_lines(UNNAMED, &week(2, 5), true);
        assert!(!face.to_lowercase().contains("peer"));
        assert!(!face.to_lowercase().contains("online"));
        assert!(!face.to_lowercase().contains("players"));
        assert!(!face.to_lowercase().contains("talent"));
        assert!(face_is_steward_honest(&face));
    }

    #[test]
    fn empty_house_display_becomes_unnamed() {
        let face = face_lines("   ", &week(0, 1), false);
        assert!(face.starts_with(UNNAMED), "got {face}");
    }
}
