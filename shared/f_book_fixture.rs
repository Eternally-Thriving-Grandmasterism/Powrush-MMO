//! F-book — test-only Settled + book fixture (harm row still off).
//!
//! On-disk JSON lives at `tests/fixtures/f-book/data/` (repo root), **not**
//! the stranger door (OS user-data dir / `POWRUSH_USER_DIR`). Reuses house /
//! hour-two / standing / climate / week shapes. `declared_lethal` stays false.
//! Week stays tons + restored. Do not copy these files into the repo `data/`
//! or the stranger user dir.
//!
//! Parent lavapipe (temp book dir; set the lab override so adopt does not
//! land F-book in the OS user dir):
//!   WALK=$(mktemp -d)
//!   mkdir -p "$WALK/data"
//!   cp tests/fixtures/f-book/data/*.json "$WALK/data/"
//!   POWRUSH_USER_DIR="$WALK/data" /path/to/powrush-client
//! Do not click the harm row. Do not retag playable-preview.
//! Contact: info@Rathor.ai

use std::fs;
use std::path::{Path, PathBuf};

use crate::house_name::{HouseName, HOUSE_PATH};
use crate::hour_two::HourTwoPack;
use crate::pause_ledger_face::lethal_sign_row;
use crate::shard_climate::ShardClimate;
use crate::shard_standing::ShardStanding;
use crate::stranger_loop_proof::{climate_week_fixture, hour_three_held_fixture};
use crate::week_audit::WeekAudit;

/// Repo-relative fixture tree (never the default `data/` door).
pub const F_BOOK_REL_DIR: &str = "tests/fixtures/f-book";
/// Directory the parent copies wholesale into a temp cwd `data/`.
pub const F_BOOK_DATA_REL: &str = "tests/fixtures/f-book/data";

pub const F_BOOK_HOUR_TWO_FILE: &str = "powrush_hour_two.json";
pub const F_BOOK_HOUSE_FILE: &str = "powrush_house.json";
pub const F_BOOK_STANDING_FILE: &str = "powrush_shard_standing.json";
pub const F_BOOK_CLIMATE_FILE: &str = "powrush_shard_climate.json";
pub const F_BOOK_WEEK_FILE: &str = "powrush_week_audit.json";

const HOUR_TWO_JSON: &str = include_str!("../tests/fixtures/f-book/data/powrush_hour_two.json");
const HOUSE_JSON: &str = include_str!("../tests/fixtures/f-book/data/powrush_house.json");
const STANDING_JSON: &str =
    include_str!("../tests/fixtures/f-book/data/powrush_shard_standing.json");
const CLIMATE_JSON: &str = include_str!("../tests/fixtures/f-book/data/powrush_shard_climate.json");
const WEEK_JSON: &str = include_str!("../tests/fixtures/f-book/data/powrush_week_audit.json");

/// In-memory Settled + hour-three / book bundle. Lethal stays off.
pub fn f_book_bundle() -> (
    HourTwoPack,
    ShardClimate,
    ShardStanding,
    WeekAudit,
    HouseName,
) {
    let pack = hour_three_held_fixture();
    let (climate, standing, week) = climate_week_fixture();
    let mut house = HouseName::default();
    house.skip();
    house.skip_seals();
    house.skip_heritage();
    (pack, climate, standing, week, house)
}

/// Load the committed on-disk fixture (existing persist shapes).
pub fn load_f_book_disk() -> (
    HourTwoPack,
    ShardClimate,
    ShardStanding,
    WeekAudit,
    HouseName,
) {
    let pack = HourTwoPack::from_json(HOUR_TWO_JSON);
    let climate = ShardClimate::from_json(CLIMATE_JSON).expect("f-book climate");
    let standing = ShardStanding::from_json(STANDING_JSON).expect("f-book standing");
    let week = WeekAudit::from_json(WEEK_JSON).expect("f-book week");
    let house = HouseName::from_json(HOUSE_JSON).expect("f-book house");
    (pack, climate, standing, week, house)
}

/// Write the bundle into `dir` using the same filenames a cwd `data/` uses.
/// Caller supplies the directory — never implied as the repo door.
pub fn write_f_book_data(dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let (pack, climate, standing, week, house) = f_book_bundle();
    fs::write(
        dir.join(F_BOOK_HOUR_TWO_FILE),
        serde_json::to_string_pretty(&pack).expect("pack json"),
    )?;
    fs::write(
        dir.join(F_BOOK_HOUSE_FILE),
        house.to_json().expect("house json"),
    )?;
    fs::write(
        dir.join(F_BOOK_STANDING_FILE),
        standing.to_json().expect("standing json"),
    )?;
    fs::write(
        dir.join(F_BOOK_CLIMATE_FILE),
        climate.to_json().expect("climate json"),
    )?;
    fs::write(dir.join(F_BOOK_WEEK_FILE), week.to_json().expect("week json"))?;
    Ok(())
}

/// Repo-root path for the committed fixture `data/` tree.
pub fn f_book_data_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..").join(F_BOOK_DATA_REL)
}

/// Default stranger persist filenames are not the fixture tree.
pub fn fixture_is_not_default_door() -> bool {
    !F_BOOK_DATA_REL.eq("data")
        && !F_BOOK_REL_DIR.eq("data")
        && !HOUSE_PATH.starts_with(F_BOOK_REL_DIR)
        && HOUSE_PATH.starts_with("data/")
        && !crate::user_persist::is_f_book_fixture_dir(&crate::user_persist::os_user_data_dir())
        && crate::user_persist::persist_file_name(HOUSE_PATH) == F_BOOK_HOUSE_FILE
}

/// Harm row for this fixture: Settled + book, lethal still off.
pub fn f_book_harm_row() -> &'static str {
    let (pack, _c, standing, _w, _h) = load_f_book_disk();
    lethal_sign_row(
        pack.ledger_settled() && pack.complete,
        pack.hour_three_complete,
        pack.session.charter_skin_live(),
        standing.declared_lethal,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::house_name::UNNAMED;
    use crate::pause_ledger_face::{HEX_ADMITS_HARM_OFF, LEDGER_WAITS, NOT_YOUR_CHARTER};
    use crate::stranger_loop_proof::peace_fixture;

    #[test]
    fn fixture_is_settled_and_book_held() {
        let (pack, _c, standing, _w, house) = load_f_book_disk();
        assert!(pack.complete, "hour two / Settled flag");
        assert!(pack.ledger_settled(), "Ledger contract Settled");
        assert!(pack.hour_three_complete, "hour three / book held");
        assert!(pack.fabricator.pack.unlocked());
        assert!(pack.embassy.seated);
        assert!(pack.session.charter_skin_live());
        assert_eq!(pack.line(true), "Hour three held · the book is yours");
        assert!(house.resolved);
        assert_eq!(house.display_name(), UNNAMED);
        assert!(house.seals_resolved);
        assert!(!standing.declared_lethal);
    }

    #[test]
    fn harm_row_is_off_line() {
        assert_eq!(f_book_harm_row(), HEX_ADMITS_HARM_OFF);
        let (pack, _c, standing, _w, _h) = load_f_book_disk();
        assert!(pack.complete);
        assert!(pack.hour_three_complete);
        assert!(!standing.declared_lethal);
        assert_eq!(
            lethal_sign_row(true, true, true, standing.declared_lethal),
            HEX_ADMITS_HARM_OFF
        );
        assert_ne!(f_book_harm_row(), NOT_YOUR_CHARTER);
        assert_ne!(f_book_harm_row(), LEDGER_WAITS);
        assert!(!HEX_ADMITS_HARM_OFF.to_lowercase().contains("combat"));
        assert!(!HEX_ADMITS_HARM_OFF.contains("kill"));
    }

    #[test]
    fn declared_lethal_stays_false_no_ton_mint() {
        let (pack, climate, standing, week, _h) = load_f_book_disk();
        assert!(!standing.declared_lethal);
        assert_eq!(standing.tariff_paid, 0);
        assert_eq!(week.tons_moved, climate.tons_moved);
        assert_eq!(week.restored_count, climate.restored_count);
        let line = week.slab_line();
        assert!(line.contains("tons"), "got {line}");
        assert!(line.contains("restored"), "got {line}");
        assert!(!line.to_lowercase().contains("kill"));
        assert!(!line.to_lowercase().contains("mint"));
        // Loading the fixture does not call declare / on_lethal_declare.
        assert!(pack.session.charter_id.is_some());
        let standing2 = standing.clone();
        assert!(!standing2.declared_lethal);
        assert_eq!(standing2.tariff_paid, 0);
    }

    #[test]
    fn default_boot_does_not_load_the_fixture() {
        assert!(fixture_is_not_default_door());
        assert_eq!(HOUSE_PATH, "data/powrush_house.json");
        assert!(!F_BOOK_DATA_REL.starts_with("data/powrush"));
        assert!(!HOUSE_PATH.contains("f-book"));
        assert!(!HOUSE_PATH.contains("fixtures"));

        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root");
        let door = root.join("data");
        assert!(
            !door.join(F_BOOK_HOUR_TWO_FILE).exists(),
            "stranger data/ must not hold the F-book hour pack"
        );
        assert!(
            !door.join(F_BOOK_HOUSE_FILE).exists(),
            "stranger data/ must not hold the F-book house"
        );
        assert!(
            !door.join(F_BOOK_STANDING_FILE).exists(),
            "stranger data/ must not hold the F-book standing"
        );
        assert!(
            !door.join(F_BOOK_CLIMATE_FILE).exists(),
            "stranger data/ must not hold the F-book climate"
        );
        assert!(
            !door.join(F_BOOK_WEEK_FILE).exists(),
            "stranger data/ must not hold the F-book week"
        );

        let fixture_dir = f_book_data_path();
        assert!(fixture_dir.join(F_BOOK_HOUR_TWO_FILE).exists());
        assert!(fixture_dir.join(F_BOOK_HOUSE_FILE).exists());
        assert!(fixture_dir.join(F_BOOK_STANDING_FILE).exists());

        let (peace_pack, _c, peace_standing, _w) = peace_fixture();
        assert!(!peace_pack.complete);
        assert!(!peace_pack.hour_three_complete);
        assert!(!peace_standing.declared_lethal);
        let fresh = HourTwoPack::default();
        assert!(!fresh.complete);
        assert!(!fresh.hour_three_complete);
        assert!(!fresh.ledger_settled());
        let first_house = HouseName::default();
        assert!(!first_house.resolved);
        assert_eq!(
            lethal_sign_row(false, false, false, false),
            NOT_YOUR_CHARTER
        );
    }

    #[test]
    fn committed_json_matches_existing_shapes() {
        let (pack, climate, standing, week, house) = load_f_book_disk();
        let (live_pack, live_c, live_s, live_w, live_h) = f_book_bundle();
        assert_eq!(pack.complete, live_pack.complete);
        assert_eq!(pack.hour_three_complete, live_pack.hour_three_complete);
        assert_eq!(pack.ledger_settled(), live_pack.ledger_settled());
        assert_eq!(pack.embassy.seated, live_pack.embassy.seated);
        assert_eq!(climate.tons_moved, live_c.tons_moved);
        assert_eq!(climate.restored_count, live_c.restored_count);
        assert_eq!(standing.declared_lethal, live_s.declared_lethal);
        assert_eq!(standing.tariff_paid, live_s.tariff_paid);
        assert_eq!(week.tons_moved, live_w.tons_moved);
        assert_eq!(week.restored_count, live_w.restored_count);
        assert_eq!(house.resolved, live_h.resolved);
        assert_eq!(house.display_name(), live_h.display_name());
        assert!(HOUR_TWO_JSON.contains("hour_three_complete"));
        assert!(STANDING_JSON.contains("declared_lethal"));
        assert!(!STANDING_JSON.contains("\"declared_lethal\": true"));
    }

    #[test]
    fn generate_committed_fixture_when_asked() {
        if std::env::var("F_BOOK_WRITE").ok().as_deref() != Some("1") {
            return;
        }
        write_f_book_data(&f_book_data_path()).expect("write committed f-book");
    }

    #[test]
    fn write_helper_targets_caller_dir_not_repo_data() {
        let dir = std::env::temp_dir().join(format!(
            "powrush-f-book-{}-{}",
            std::process::id(),
            "proof"
        ));
        write_f_book_data(&dir).expect("write temp fixture");
        let raw = fs::read_to_string(dir.join(F_BOOK_HOUR_TWO_FILE)).unwrap();
        let pack = HourTwoPack::from_json(&raw);
        assert!(pack.complete && pack.hour_three_complete);
        let standing =
            ShardStanding::from_json(&fs::read_to_string(dir.join(F_BOOK_STANDING_FILE)).unwrap())
                .unwrap();
        assert!(!standing.declared_lethal);
        let _ = fs::remove_dir_all(&dir);
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root");
        assert!(!root.join("data").join(F_BOOK_HOUR_TWO_FILE).exists());
    }
}
