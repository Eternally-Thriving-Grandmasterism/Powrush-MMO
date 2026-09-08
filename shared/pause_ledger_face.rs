//! Pause / Ledger face — S3 (v23.2.52) + stranger-pass wait line (v23.2.61)
//!
//! House name · seals (when dressed) · week tons + restored · lethal only if declared.
//! Shown on I (satchel) and/or L (Ledger sash) — and Q plate reuses seal caption.
//! No second HUD.
//! Before charter / Bind-only pre-Settled: never a blank panel — wait line.
//! No talent tree · no peer count · no fake online.
//! Contact: info@Rathor.ai

use crate::house_name::{HouseName, UNNAMED};
use crate::week_audit::WeekAudit;

/// Same clause standing uses when lethal is already declared.
pub const LETHAL_DECLARED_LINE: &str = "lethal is declared — the hex will remember";

/// L1 hex sign (Settings / Q / Ledger confirm). Not a weapon. Default off.
pub const HEX_ADMITS_HARM: &str = "this hex admits harm";

/// Confirm row while Settled + book and still off.
pub const HEX_ADMITS_HARM_OFF: &str = "this hex admits harm · off";

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
/// When seals resolved, inserts Seal · … (heritage string) under the house name.
pub fn face_from(house: &HouseName, week: &WeekAudit, declared_lethal: bool) -> String {
    let mut out = String::new();
    out.push_str(house.display_name());
    if let Some(dress) = house.dress_line_for_plate() {
        out.push('\n');
        out.push_str(&dress);
    }
    out.push('\n');
    out.push_str(&week.slab_line());
    if declared_lethal {
        out.push('\n');
        out.push_str(LETHAL_DECLARED_LINE);
    }
    out
}

/// Seal caption for the Q founding plate (same dress line as Pause face).
pub fn q_plate_seal_line(house: &HouseName) -> Option<String> {
    house.dress_line_for_plate()
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

/// Settled + Hour three / book held — the only moment the hex sign may confirm.
pub fn lethal_sign_eligible(settled: bool, book_held: bool) -> bool {
    settled && book_held
}

/// Settings / Q / Ledger confirm row. Wait copy when Settled+book not held.
/// Declared → "this hex admits harm". Else off. Never a combat verb.
pub fn lethal_sign_row(
    settled: bool,
    book_held: bool,
    charter_live: bool,
    declared: bool,
) -> &'static str {
    if !lethal_sign_eligible(settled, book_held) {
        wait_line_before_settled(charter_live)
    } else if declared {
        HEX_ADMITS_HARM
    } else {
        HEX_ADMITS_HARM_OFF
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
    fn face_and_q_plate_show_seal_when_dressed() {
        let mut house = HouseName::default();
        house.confirm("Keep Yard");
        house.set_seals(&["well"]);
        house.confirm_seals();
        house.set_heritage("human");
        let face = face_from(&house, &week(1, 0), false);
        assert!(face.contains("Keep Yard"), "got {face}");
        assert!(face.contains("Seal · Well"), "got {face}");
        assert!(face.contains("human"), "got {face}");
        assert!(face.contains("1 tons"), "got {face}");
        assert_eq!(
            q_plate_seal_line(&house).as_deref(),
            Some("Seal · Well · human")
        );
        assert!(face_is_steward_honest(&face));
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

    #[test]
    fn l1_sign_default_off_requires_settled_and_book() {
        assert!(!lethal_sign_eligible(false, false));
        assert!(!lethal_sign_eligible(true, false));
        assert!(!lethal_sign_eligible(false, true));
        assert!(lethal_sign_eligible(true, true));
        assert_eq!(
            lethal_sign_row(false, false, false, false),
            NOT_YOUR_CHARTER
        );
        assert_eq!(lethal_sign_row(true, false, true, false), LEDGER_WAITS);
        assert_eq!(lethal_sign_row(false, true, true, false), LEDGER_WAITS);
        assert_eq!(
            lethal_sign_row(true, true, true, false),
            HEX_ADMITS_HARM_OFF
        );
        assert_eq!(lethal_sign_row(true, true, true, true), HEX_ADMITS_HARM);
        assert!(!HEX_ADMITS_HARM_OFF.to_lowercase().contains("combat"));
        assert!(!HEX_ADMITS_HARM.contains("kill"));
    }
}
