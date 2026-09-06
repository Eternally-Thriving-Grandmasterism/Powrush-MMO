//! Pause / Ledger face — S3 (v23.2.52) + stranger-pass wait line (v23.2.61)
//!
//! House name · week tons + restored · lethal only if already declared.
//! Shown on I (satchel) and/or L (Ledger sash). No second HUD.
//! Before charter / Bind-only pre-Settled: never a blank panel — wait line.
//! No talent tree · no peer count · no fake online.
//! Contact: info@Rathor.ai

use crate::house_name::{HouseName, UNNAMED};
use crate::week_audit::WeekAudit;

/// Same clause standing uses when lethal is already declared.
pub const LETHAL_DECLARED_LINE: &str = "lethal is declared — the hex will remember";

/// Pre-charter L press — honest refuse, never blank.
pub const NOT_YOUR_CHARTER: &str = "Not your charter";

/// Bind-only / pre-Settled L press — ledger not yours yet.
pub const LEDGER_WAITS: &str = "the ledger waits";

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

/// One-line wait copy when L opens before Settled / without charter.
/// Prefer *Not your charter* off-charter; *the ledger waits* once Bind skin is live but unsettled.
pub fn wait_line_before_settled(charter_live: bool) -> &'static str {
    if charter_live {
        LEDGER_WAITS
    } else {
        NOT_YOUR_CHARTER
    }
}

/// L sash body: full house+week face when charter skin is live (Bind or Settled);
/// wait line when L opens without charter — never blank.
/// `settled` reserved for callers that want Bind-only wait copy via `wait_line_before_settled`.
pub fn ledger_sash_body(
    charter_live: bool,
    settled: bool,
    house: &HouseName,
    week: &WeekAudit,
    declared_lethal: bool,
) -> String {
    let _ = settled;
    if charter_live {
        face_from(house, week, declared_lethal)
    } else {
        wait_line_before_settled(false).to_string()
    }
}

/// Bind-only / pre-Settled panel body: wait line (never blank) until Settled.
pub fn bind_only_before_settled_body(charter_live: bool, settled: bool) -> Option<&'static str> {
    if settled {
        None
    } else {
        Some(wait_line_before_settled(charter_live))
    }
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

    #[test]
    fn pre_settled_wait_line_never_blank() {
        assert_eq!(wait_line_before_settled(false), NOT_YOUR_CHARTER);
        assert_eq!(wait_line_before_settled(true), LEDGER_WAITS);
        let mut house = HouseName::default();
        house.skip();
        let early = ledger_sash_body(false, false, &house, &week(0, 0), false);
        assert_eq!(early, NOT_YOUR_CHARTER);
        assert!(!early.is_empty());
        // Bind-only before Settled: dedicated wait copy (never blank).
        assert_eq!(
            bind_only_before_settled_body(true, false),
            Some(LEDGER_WAITS)
        );
        assert_eq!(
            bind_only_before_settled_body(false, false),
            Some(NOT_YOUR_CHARTER)
        );
        assert_eq!(bind_only_before_settled_body(true, true), None);
        // Charter live (Bind or Settled) → L2 face with Unnamed + week 0/0.
        let face = ledger_sash_body(true, false, &house, &week(0, 0), false);
        assert!(face.contains(UNNAMED));
        assert!(face.contains("0 tons"));
        assert!(face.contains("0 restored"));
        assert!(!face.contains("lethal"));
        let settled = ledger_sash_body(true, true, &house, &week(0, 0), false);
        assert!(settled.contains(UNNAMED));
        assert!(!settled.contains("lethal"));
    }
}
