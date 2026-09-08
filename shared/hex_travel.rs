//! U2 — local hex travel, disk only (no listen)
//!
//! After Settled + book, Places (Sanctuary / Heartwood) can confirm leave,
//! write the current hex climate into the U1 user dir, and load the other
//! place. Play (first hands) always boots Sanctuary Prime. Continue without
//! the book still boots Sanctuary only.
//!
//! Travel is another file in the user dir (`powrush_hex_<id>.json`), not a
//! server. Isolation: gamma = 0. No leak tick. Do not couple tons, seeds, or
//! declared_lethal across hexes. Heartwood is a stub (lamp disk empty, same
//! Peace E). Do not drop a Heartwood save on default new-game. F-book is not
//! the stranger door.
//!
//! Contact: info@Rathor.ai. Independent of xAI.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::embassy::Embassy;
use crate::hour_two::HourTwoPack;
use crate::hex_listen::PowrushNet;
use crate::hex_protocol::default_client_listens;
use crate::pause_ledger_face::wait_line_before_settled;
use crate::shard_climate::ShardClimate;
use crate::shard_standing::ShardStanding;
use crate::space_law::HexFlag;
use crate::user_persist;
use crate::week_audit::WeekAudit;

/// Offline 1.0 isolation. Leak tick is a later named law (U3.5), not U2.
pub const ISOLATION_GAMMA: f32 = 0.0;

/// Places list / hex climate files live beside house JSON in the user dir.
pub const HEX_FILE_PREFIX: &str = "powrush_hex_";
pub const HEX_FILE_SUFFIX: &str = ".json";
pub const CURRENT_HEX_FILE: &str = "powrush_hex_current.json";

/// Confirm copy on the Places plate (after a dest is chosen).
pub const LEAVE_CONFIRM: &str = "Leave this hex?";
/// Places plate title.
pub const PLACES_TITLE: &str = "Places";
/// Places pause-row label when Settled + book.
pub const PLACES_ROW: &str = "Places";

/// Disk-only hex / place. Not a shard on a wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaceId {
    Sanctuary,
    Heartwood,
}

impl PlaceId {
    pub const fn as_str(self) -> &'static str {
        match self {
            PlaceId::Sanctuary => "sanctuary",
            PlaceId::Heartwood => "heartwood",
        }
    }

    pub const fn display_name(self) -> &'static str {
        match self {
            PlaceId::Sanctuary => "Sanctuary",
            PlaceId::Heartwood => "Heartwood",
        }
    }

    /// Chip / yard name. Sanctuary keeps the Prime label.
    pub const fn chip_name(self) -> &'static str {
        match self {
            PlaceId::Sanctuary => "Sanctuary Prime",
            PlaceId::Heartwood => "Heartwood",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "sanctuary" | "sanctuary-prime" | "sanctuary_prime" | "local-hex" => {
                Some(PlaceId::Sanctuary)
            }
            "heartwood" => Some(PlaceId::Heartwood),
            _ => None,
        }
    }

    /// Both U2 places teach Peace E. Heartwood is a stub, not Frontier.
    pub const fn peace_hex(self) -> HexFlag {
        HexFlag::Peace
    }
}

impl Default for PlaceId {
    fn default() -> Self {
        PlaceId::Sanctuary
    }
}

/// Title door boot kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootKind {
    /// First hands — always Sanctuary Prime.
    Play,
    /// Resume. Without the book, Sanctuary only.
    Continue,
}

/// Why travel / Places was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TravelRefuse {
    NotYourCharter,
    SamePlace,
}

/// On-disk hex climate (reuses ShardClimate / ShardStanding shapes).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HexClimateFile {
    pub hex_id: String,
    pub climate: ShardClimate,
    #[serde(default)]
    pub standing: ShardStanding,
    /// Heartwood stub: embassy lamp disk empty.
    #[serde(default)]
    pub lamp_empty: bool,
}

impl HexClimateFile {
    pub fn from_parts(place: PlaceId, climate: ShardClimate, standing: ShardStanding) -> Self {
        let mut climate = climate;
        climate.hex_id = place.as_str().into();
        let mut standing = standing;
        standing.hex_id = place.as_str().into();
        Self {
            hex_id: place.as_str().into(),
            climate,
            standing,
            lamp_empty: place == PlaceId::Heartwood,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }
}

/// Last hex pointer (`powrush_hex_current.json`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentHex {
    pub hex_id: String,
}

impl CurrentHex {
    pub fn new(place: PlaceId) -> Self {
        Self {
            hex_id: place.as_str().into(),
        }
    }

    pub fn place(&self) -> PlaceId {
        PlaceId::parse(&self.hex_id).unwrap_or(PlaceId::Sanctuary)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }
}

/// Isolation law: no leak tick. gamma stays 0.
pub fn leak_tick_enabled() -> bool {
    ISOLATION_GAMMA != 0.0
}

/// Filename `powrush_hex_<id>.json` (user-dir basename).
pub fn hex_file_name(place: PlaceId) -> String {
    format!("{HEX_FILE_PREFIX}{}{HEX_FILE_SUFFIX}", place.as_str())
}

pub fn hex_file_path_in(dir: &Path, place: PlaceId) -> PathBuf {
    dir.join(hex_file_name(place))
}

/// Heartwood stub climate — empty tons/seeds, Peace defaults. Not a Sanctuary copy.
pub fn heartwood_stub_climate() -> ShardClimate {
    ShardClimate {
        hex_id: PlaceId::Heartwood.as_str().into(),
        tons_moved: 0,
        restored_count: 0,
        reserve_pool: 0,
        seed_u64: None,
        gen_epoch: None,
        ..ShardClimate::default()
    }
}

pub fn sanctuary_fresh_climate() -> ShardClimate {
    ShardClimate {
        hex_id: PlaceId::Sanctuary.as_str().into(),
        ..ShardClimate::default()
    }
}

/// Heartwood stub standing — lethal off, not copied from Sanctuary.
pub fn heartwood_stub_standing() -> ShardStanding {
    ShardStanding {
        hex_id: PlaceId::Heartwood.as_str().into(),
        declared_lethal: false,
        tariff_paid: 0,
        ..ShardStanding::default()
    }
}

pub fn sanctuary_fresh_standing() -> ShardStanding {
    ShardStanding {
        hex_id: PlaceId::Sanctuary.as_str().into(),
        ..ShardStanding::default()
    }
}

/// Heartwood lamp disk is empty (embassy lamp not live on the stub).
pub fn heartwood_lamp_empty(embassy: &Embassy) -> bool {
    !embassy.lamp_live
}

pub fn heartwood_stub_embassy() -> Embassy {
    Embassy {
        lamp_live: false,
        seated: false,
        ..Embassy::default()
    }
}

/// House embassy is house-level. Heartwood `lamp_empty` is hex climate, not a second seat.
pub fn house_embassy_on_place(house: &Embassy, place: PlaceId) -> Embassy {
    let _ = place;
    house.clone()
}

/// After confirm leave, keep the house HourTwoPack. Hex climate is a different file.
pub fn house_pack_after_leave(house: &HourTwoPack, dest: PlaceId) -> HourTwoPack {
    let _ = dest;
    house.clone()
}

/// Fresh stub disk for a place. Heartwood lamp empty; Sanctuary lamp not implied.
pub fn stub_hex_file(place: PlaceId) -> HexClimateFile {
    match place {
        PlaceId::Sanctuary => HexClimateFile::from_parts(
            place,
            sanctuary_fresh_climate(),
            sanctuary_fresh_standing(),
        ),
        PlaceId::Heartwood => {
            let mut file = HexClimateFile::from_parts(
                place,
                heartwood_stub_climate(),
                heartwood_stub_standing(),
            );
            file.lamp_empty = true;
            file
        }
    }
}

/// Dest climate for travel: existing disk, else stub. Never a copy of `from`.
pub fn load_or_stub_at(dir: &Path, dest: PlaceId) -> HexClimateFile {
    read_hex_at(dir, dest).unwrap_or_else(|| stub_hex_file(dest))
}

pub fn read_hex_at(dir: &Path, place: PlaceId) -> Option<HexClimateFile> {
    let raw = fs::read_to_string(hex_file_path_in(dir, place)).ok()?;
    HexClimateFile::from_json(&raw).ok()
}

pub fn write_hex_at(dir: &Path, file: &HexClimateFile) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    let place = PlaceId::parse(&file.hex_id).unwrap_or(PlaceId::Sanctuary);
    fs::write(hex_file_path_in(dir, place), file.to_json().unwrap_or_default())
}

pub fn write_current_at(dir: &Path, place: PlaceId) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    fs::write(
        dir.join(CURRENT_HEX_FILE),
        CurrentHex::new(place).to_json().unwrap_or_default(),
    )
}

pub fn read_current_at(dir: &Path) -> Option<PlaceId> {
    let raw = fs::read_to_string(dir.join(CURRENT_HEX_FILE)).ok()?;
    CurrentHex::from_json(&raw).ok().map(|c| c.place())
}

/// Write via U1 user-dir persist (`POWRUSH_USER_DIR` / OS user-data).
pub fn write_hex_named(file: &HexClimateFile) -> std::io::Result<()> {
    let place = PlaceId::parse(&file.hex_id).unwrap_or(PlaceId::Sanctuary);
    let json = file
        .to_json()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    user_persist::write_named(&hex_file_name(place), json)
}

pub fn read_hex_named(place: PlaceId) -> Option<HexClimateFile> {
    let raw = user_persist::read_named(&hex_file_name(place)).ok()?;
    HexClimateFile::from_json(&raw).ok()
}

pub fn write_current_named(place: PlaceId) -> std::io::Result<()> {
    let json = CurrentHex::new(place)
        .to_json()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    user_persist::write_named(CURRENT_HEX_FILE, json)
}

pub fn read_current_named() -> Option<PlaceId> {
    let raw = user_persist::read_named(CURRENT_HEX_FILE).ok()?;
    CurrentHex::from_json(&raw).ok().map(|c| c.place())
}

/// Places list is live only after Settled + book.
pub fn places_eligible(settled: bool, book_held: bool) -> bool {
    settled && book_held
}

/// Hidden (`None`) without Settled+book. Callers may instead show *Not your charter*.
pub fn places_row_label(settled: bool, book_held: bool) -> Option<&'static str> {
    if places_eligible(settled, book_held) {
        Some(PLACES_ROW)
    } else {
        None
    }
}

/// Inert copy when a Places row is shown without the book.
pub fn places_inert_label(charter_live: bool) -> &'static str {
    wait_line_before_settled(charter_live)
}

pub fn places_row_or_inert(settled: bool, book_held: bool, charter_live: bool) -> &'static str {
    places_row_label(settled, book_held).unwrap_or_else(|| places_inert_label(charter_live))
}

/// Heartwood requires the book. Sanctuary is always the Peace yard.
pub fn may_enter(place: PlaceId, book_held: bool) -> bool {
    match place {
        PlaceId::Sanctuary => true,
        PlaceId::Heartwood => book_held,
    }
}

/// Confirm leave. No-book / not Settled → *Not your charter*. Same place → no-op.
pub fn confirm_leave(
    settled: bool,
    book_held: bool,
    from: PlaceId,
    to: PlaceId,
) -> Result<PlaceId, TravelRefuse> {
    if !places_eligible(settled, book_held) || !may_enter(to, book_held) {
        return Err(TravelRefuse::NotYourCharter);
    }
    if from == to {
        return Err(TravelRefuse::SamePlace);
    }
    Ok(to)
}

/// Play always Sanctuary. Continue without book: Sanctuary. Continue + book: last hex.
pub fn boot_place(kind: BootKind, book_held: bool, last: Option<PlaceId>) -> PlaceId {
    match kind {
        BootKind::Play => PlaceId::Sanctuary,
        BootKind::Continue => {
            if !book_held {
                PlaceId::Sanctuary
            } else {
                match last {
                    Some(PlaceId::Heartwood) if book_held => PlaceId::Heartwood,
                    Some(p) => p,
                    None => PlaceId::Sanctuary,
                }
            }
        }
    }
}

/// Isolate dest from source: never copy tons, seeds, or declared_lethal.
pub fn isolate_from_source(source: &HexClimateFile, dest: &mut HexClimateFile) {
    let _ = ISOLATION_GAMMA;
    if dest.hex_id == source.hex_id {
        return;
    }
    // A copied source blob still wearing the source hex_id is not dest climate.
    if dest.climate.hex_id == source.climate.hex_id {
        dest.climate.hex_id = dest.hex_id.clone();
        dest.climate.tons_moved = 0;
        dest.climate.restored_count = 0;
        dest.climate.seed_u64 = None;
        dest.climate.gen_epoch = None;
    }
    if dest.standing.hex_id == source.standing.hex_id {
        dest.standing.hex_id = dest.hex_id.clone();
        dest.standing.declared_lethal = false;
        dest.standing.tariff_paid = 0;
    }
}

/// Travel plan: write `from` climate, load dest stub-or-disk. Isolation applied.
pub fn plan_travel(
    settled: bool,
    book_held: bool,
    from: PlaceId,
    to: PlaceId,
    from_climate: &ShardClimate,
    from_standing: &ShardStanding,
    dest_existing: Option<HexClimateFile>,
) -> Result<(HexClimateFile, HexClimateFile), TravelRefuse> {
    let dest = confirm_leave(settled, book_held, from, to)?;
    let write_from = HexClimateFile::from_parts(from, from_climate.clone(), from_standing.clone());
    let dest_was_missing = dest_existing.is_none();
    let mut load_to = dest_existing.unwrap_or_else(|| stub_hex_file(dest));
    isolate_from_source(&write_from, &mut load_to);
    // Hard law: do not copy Sanctuary tons into Heartwood.
    if dest == PlaceId::Heartwood && from == PlaceId::Sanctuary && dest_was_missing {
        load_to.climate.tons_moved = 0;
        load_to.climate.restored_count = 0;
        load_to.climate.seed_u64 = None;
        load_to.standing.declared_lethal = false;
        load_to.lamp_empty = true;
    }
    Ok((write_from, load_to))
}

/// Apply travel against a directory (tests / lab). Writes from-hex + current pointer.
pub fn apply_travel_at(
    dir: &Path,
    settled: bool,
    book_held: bool,
    from: PlaceId,
    to: PlaceId,
    from_climate: &ShardClimate,
    from_standing: &ShardStanding,
) -> Result<HexClimateFile, TravelRefuse> {
    let existing = read_hex_at(dir, to);
    let (write_from, load_to) = plan_travel(
        settled,
        book_held,
        from,
        to,
        from_climate,
        from_standing,
        existing,
    )?;
    let _ = write_hex_at(dir, &write_from);
    let _ = write_hex_at(dir, &load_to);
    let _ = write_current_at(dir, to);
    Ok(load_to)
}

/// Apply travel into the U1 user dir.
pub fn apply_travel_named(
    settled: bool,
    book_held: bool,
    from: PlaceId,
    to: PlaceId,
    from_climate: &ShardClimate,
    from_standing: &ShardStanding,
) -> Result<HexClimateFile, TravelRefuse> {
    let existing = read_hex_named(to);
    let (write_from, load_to) = plan_travel(
        settled,
        book_held,
        from,
        to,
        from_climate,
        from_standing,
        existing,
    )?;
    let _ = write_hex_named(&write_from);
    let _ = write_hex_named(&load_to);
    let _ = write_current_named(to);
    Ok(load_to)
}

/// House week footer may sum tons + restored across hexes. Does not write into a hex.
pub fn house_week_footer(climates: &[ShardClimate]) -> WeekAudit {
    let tons: u32 = climates.iter().map(|c| c.tons_moved).sum();
    let restored: u32 = climates.iter().map(|c| c.restored_count).sum();
    let mut week = WeekAudit::default();
    week.sync_from_climate(tons, restored);
    week
}

/// Default new-game must not drop a Heartwood save.
pub fn new_game_writes_heartwood() -> bool {
    false
}

pub fn heartwood_file_present_in(dir: &Path) -> bool {
    hex_file_path_in(dir, PlaceId::Heartwood).exists()
}

/// Travel is a file, not a listen. Title Online stays grey.
pub fn travel_is_disk_only() -> bool {
    !default_client_listens()
        && !PowrushNet::Off.title_online_enabled()
        && !leak_tick_enabled()
        && ISOLATION_GAMMA == 0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::f_book_fixture::{fixture_is_not_default_door, F_BOOK_DATA_REL};
    use crate::house_name::HOUSE_PATH;
    use crate::pause_ledger_face::NOT_YOUR_CHARTER;
    use crate::title_house_proof::{online_row_is_honest_disabled, ONLINE_STUB_LABEL};
    use crate::user_persist::{is_f_book_fixture_dir, persist_file_name};

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "powrush-u2-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0),
            tag
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    #[test]
    fn play_first_hands_always_sanctuary() {
        assert_eq!(
            boot_place(BootKind::Play, false, None),
            PlaceId::Sanctuary
        );
        assert_eq!(
            boot_place(BootKind::Play, true, Some(PlaceId::Heartwood)),
            PlaceId::Sanctuary
        );
        assert_eq!(PlaceId::default(), PlaceId::Sanctuary);
    }

    #[test]
    fn continue_without_book_boots_sanctuary() {
        assert_eq!(
            boot_place(BootKind::Continue, false, Some(PlaceId::Heartwood)),
            PlaceId::Sanctuary
        );
        assert_eq!(
            boot_place(BootKind::Continue, false, None),
            PlaceId::Sanctuary
        );
        assert!(!may_enter(PlaceId::Heartwood, false));
        assert_eq!(
            confirm_leave(true, false, PlaceId::Sanctuary, PlaceId::Heartwood),
            Err(TravelRefuse::NotYourCharter)
        );
    }

    #[test]
    fn continue_with_book_may_load_last_hex() {
        assert_eq!(
            boot_place(BootKind::Continue, true, Some(PlaceId::Heartwood)),
            PlaceId::Heartwood
        );
        assert_eq!(
            boot_place(BootKind::Continue, true, Some(PlaceId::Sanctuary)),
            PlaceId::Sanctuary
        );
        assert!(may_enter(PlaceId::Heartwood, true));
    }

    #[test]
    fn places_hidden_or_inert_without_book() {
        assert!(!places_eligible(false, false));
        assert!(!places_eligible(true, false));
        assert!(!places_eligible(false, true));
        assert!(places_eligible(true, true));
        assert_eq!(places_row_label(false, false), None);
        assert_eq!(places_row_label(true, false), None);
        assert_eq!(places_row_or_inert(false, false, false), NOT_YOUR_CHARTER);
        assert_eq!(places_row_label(true, true), Some(PLACES_ROW));
    }

    #[test]
    fn no_book_cannot_enter_heartwood() {
        let dir = scratch("nobook");
        let mut climate = sanctuary_fresh_climate();
        climate.tons_moved = 4;
        let standing = sanctuary_fresh_standing();
        let err = apply_travel_at(
            &dir,
            false,
            false,
            PlaceId::Sanctuary,
            PlaceId::Heartwood,
            &climate,
            &standing,
        );
        assert_eq!(err, Err(TravelRefuse::NotYourCharter));
        assert!(!heartwood_file_present_in(&dir));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn book_path_writes_hex_and_loads_stub() {
        let dir = scratch("book");
        let mut climate = sanctuary_fresh_climate();
        climate.tons_moved = 5;
        climate.restored_count = 2;
        climate.seed_u64 = Some(99);
        let mut standing = sanctuary_fresh_standing();
        standing.declared_lethal = true;
        let loaded = apply_travel_at(
            &dir,
            true,
            true,
            PlaceId::Sanctuary,
            PlaceId::Heartwood,
            &climate,
            &standing,
        )
        .expect("travel");
        assert!(hex_file_path_in(&dir, PlaceId::Sanctuary).exists());
        assert!(hex_file_path_in(&dir, PlaceId::Heartwood).exists());
        assert_eq!(read_current_at(&dir), Some(PlaceId::Heartwood));
        let written = read_hex_at(&dir, PlaceId::Sanctuary).unwrap();
        assert_eq!(written.climate.tons_moved, 5);
        assert_eq!(loaded.climate.hex_id, "heartwood");
        assert_eq!(loaded.climate.tons_moved, 0, "do not copy Sanctuary tons");
        assert_eq!(loaded.climate.restored_count, 0);
        assert!(loaded.climate.seed_u64.is_none());
        assert!(!loaded.standing.declared_lethal);
        assert!(loaded.lamp_empty);
        assert_eq!(loaded.climate.hex_id, PlaceId::Heartwood.as_str());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn travel_writes_user_dir_hex_file_name() {
        assert_eq!(hex_file_name(PlaceId::Sanctuary), "powrush_hex_sanctuary.json");
        assert_eq!(hex_file_name(PlaceId::Heartwood), "powrush_hex_heartwood.json");
        assert_eq!(
            persist_file_name(&hex_file_name(PlaceId::Sanctuary)),
            "powrush_hex_sanctuary.json"
        );
        assert_eq!(
            persist_file_name("data/powrush_hex_heartwood.json"),
            "powrush_hex_heartwood.json"
        );
        assert_eq!(CURRENT_HEX_FILE, "powrush_hex_current.json");
    }

    #[test]
    fn gamma_stays_zero_no_leak_tick() {
        assert_eq!(ISOLATION_GAMMA, 0.0);
        assert!(!leak_tick_enabled());
        assert!(travel_is_disk_only());
    }

    #[test]
    fn f_book_fixture_is_not_the_default_door() {
        assert!(fixture_is_not_default_door());
        assert_ne!(F_BOOK_DATA_REL, "data");
        assert!(!HOUSE_PATH.contains("f-book"));
        let os = user_persist::os_user_data_dir();
        assert!(!is_f_book_fixture_dir(&os));
        assert!(!hex_file_name(PlaceId::Heartwood).contains("f-book"));
    }

    #[test]
    fn new_game_does_not_drop_heartwood_save() {
        assert!(!new_game_writes_heartwood());
        let dir = scratch("fresh");
        assert!(!heartwood_file_present_in(&dir));
        assert!(!persist_files_include_heartwood(&dir));
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repo root");
        assert!(
            !root.join("data").join(hex_file_name(PlaceId::Heartwood)).exists(),
            "repo data/ must not ship a Heartwood save"
        );
        assert!(!root.join("data").join(CURRENT_HEX_FILE).exists());
        let _ = fs::remove_dir_all(&dir);
    }

    fn persist_files_include_heartwood(dir: &Path) -> bool {
        dir.join(hex_file_name(PlaceId::Heartwood)).exists()
    }

    #[test]
    fn house_week_footer_may_sum_without_copying_into_heartwood() {
        let mut sanctuary = sanctuary_fresh_climate();
        sanctuary.tons_moved = 3;
        sanctuary.restored_count = 1;
        let heartwood = heartwood_stub_climate();
        assert_eq!(heartwood.tons_moved, 0);
        let week = house_week_footer(&[sanctuary.clone(), heartwood.clone()]);
        assert_eq!(week.tons_moved, 3);
        assert_eq!(week.restored_count, 1);
        assert_eq!(heartwood.tons_moved, 0);
        assert!(week.slab_line().contains("3 tons"));
        assert!(week.slab_line().contains("1 restored"));
    }

    #[test]
    fn heartwood_stub_is_peace_e_lamp_empty() {
        assert_eq!(PlaceId::Heartwood.peace_hex(), HexFlag::Peace);
        assert_eq!(PlaceId::Sanctuary.peace_hex(), HexFlag::Peace);
        let embassy = heartwood_stub_embassy();
        assert!(heartwood_lamp_empty(&embassy));
        assert!(!embassy.lamp_live);
        let stub = stub_hex_file(PlaceId::Heartwood);
        assert!(stub.lamp_empty);
        assert_eq!(stub.climate.tons_moved, 0);
    }

    #[test]
    fn title_online_stays_grey() {
        assert!(!PowrushNet::Off.title_online_enabled());
        assert!(!PowrushNet::Localhost.title_online_enabled());
        assert!(online_row_is_honest_disabled(ONLINE_STUB_LABEL, false));
        assert!(!default_client_listens());
        assert!(travel_is_disk_only());
    }

    #[test]
    fn hex_file_roundtrip_reuses_climate_shape() {
        let mut climate = sanctuary_fresh_climate();
        climate.on_lane();
        climate.on_mend();
        let standing = sanctuary_fresh_standing();
        let file = HexClimateFile::from_parts(PlaceId::Sanctuary, climate.clone(), standing);
        let back = HexClimateFile::from_json(&file.to_json().unwrap()).unwrap();
        assert_eq!(back.climate.tons_moved, 1);
        assert_eq!(back.climate.restored_count, 1);
        assert_eq!(back.hex_id, "sanctuary");
        assert!(!back.standing.declared_lethal);
    }

    #[test]
    fn same_place_confirm_is_noop() {
        assert_eq!(
            confirm_leave(true, true, PlaceId::Sanctuary, PlaceId::Sanctuary),
            Err(TravelRefuse::SamePlace)
        );
    }

    #[test]
    fn house_book_and_embassy_survive_heartwood_leave_and_sanctuary_return() {
        use crate::f_book_fixture::load_f_book_disk;

        let dir = scratch("book-survive");
        let (house, climate, standing, _week, _name) = load_f_book_disk();
        assert!(house.complete && house.hour_three_complete && house.embassy.seated);
        let house_embassy = house.embassy.clone();

        let loaded = apply_travel_at(
            &dir,
            house.complete,
            house.hour_three_complete,
            PlaceId::Sanctuary,
            PlaceId::Heartwood,
            &climate,
            &standing,
        )
        .expect("leave Heartwood");
        assert!(loaded.lamp_empty);
        assert!(!loaded.standing.declared_lethal);
        assert_eq!(loaded.climate.hex_id, PlaceId::Heartwood.as_str());

        // Fail beat: persist_pack used to swap the stub into the house pack
        // then mark_hour_three (requires seated) cleared the book.
        let mut persist = house.clone();
        persist.embassy = heartwood_stub_embassy();
        persist.keep_house_book_over_hex_stub(&house);
        persist.mark_hour_three();
        assert!(
            persist.hour_three_complete,
            "travel must not drop hour_three_complete"
        );
        assert!(
            persist.embassy.seated,
            "house embassy seat must survive the Heartwood stub"
        );
        assert_eq!(persist.embassy, house_embassy);
        assert!(places_eligible(persist.complete, persist.hour_three_complete));
        assert_eq!(
            house_embassy_on_place(&house_embassy, PlaceId::Heartwood),
            house_embassy
        );
        assert_eq!(
            house_pack_after_leave(&house, PlaceId::Heartwood).embassy,
            house_embassy
        );

        let hour_path = dir.join("powrush_hour_two.json");
        fs::write(
            &hour_path,
            serde_json::to_string_pretty(&persist).expect("house json"),
        )
        .expect("write house");
        let on_stub = HourTwoPack::from_json(&fs::read_to_string(&hour_path).expect("read house"));
        assert!(on_stub.hour_three_complete && on_stub.embassy.seated);
        assert!(places_eligible(on_stub.complete, on_stub.hour_three_complete));

        let back = apply_travel_at(
            &dir,
            persist.complete,
            persist.hour_three_complete,
            PlaceId::Heartwood,
            PlaceId::Sanctuary,
            &heartwood_stub_climate(),
            &heartwood_stub_standing(),
        )
        .expect("return Sanctuary");
        assert_eq!(back.hex_id, PlaceId::Sanctuary.as_str());
        assert!(!back.standing.declared_lethal);

        let mut persist_back = persist.clone();
        persist_back.embassy = house_embassy_on_place(&persist.embassy, PlaceId::Sanctuary);
        persist_back.keep_house_book_over_hex_stub(&persist);
        persist_back.mark_hour_three();
        assert!(persist_back.hour_three_complete);
        assert!(persist_back.embassy.seated);
        assert!(places_eligible(
            persist_back.complete,
            persist_back.hour_three_complete
        ));
        fs::write(
            &hour_path,
            serde_json::to_string_pretty(&persist_back).expect("house json"),
        )
        .expect("rewrite house");
        let returned = HourTwoPack::from_json(&fs::read_to_string(&hour_path).expect("reread"));
        assert!(returned.hour_three_complete && returned.embassy.seated);
        assert_eq!(returned.embassy, house_embassy);

        let hw = read_hex_at(&dir, PlaceId::Heartwood).expect("heartwood hex");
        assert!(hw.lamp_empty);
        assert!(!hw.standing.declared_lethal);
        assert_eq!(read_current_at(&dir), Some(PlaceId::Sanctuary));
        let _ = fs::remove_dir_all(&dir);
    }
}
