//! U12 — the second well's ink, on disk.
//!
//! The re-walk fail was a Heartwood hex file still at 0/0 after a pipe Tend,
//! which left the House bill unable to see the second room. This runs the
//! walk against a real user dir: Tend on Heartwood, then read the files back
//! and add them the way the Ledger plate does.
//!
//! Own test binary so `POWRUSH_USER_DIR` is this process's alone.

use powrush_client::heartwood_lip::ink_room_tend;
use powrush_client::ledger_bind::house_week_bill_from_disk;
use powrush_client::lived_hour_bind::LivedHourBind;

use shared::climate_node::LivedHour;
use shared::hex_travel::{
    heartwood_stub_climate, heartwood_stub_standing, read_hex_named, sanctuary_fresh_climate,
    sanctuary_fresh_standing, write_hex_named, HexClimateFile, PlaceId,
};
use shared::pause_ledger_face::house_week_line;
use shared::week_audit::WeekAudit;

fn bind_on(
    climate: shared::shard_climate::ShardClimate,
    standing: shared::shard_standing::ShardStanding,
) -> LivedHourBind {
    LivedHourBind {
        hour: LivedHour::new_demo(),
        climate,
        standing,
        week: WeekAudit::default(),
        last_line: String::new(),
        guidance_hidden: false,
        focus_id: None,
        climate_slab: None,
    }
}

fn yard_line(climate: &shared::shard_climate::ShardClimate) -> String {
    let mut week = WeekAudit::default();
    week.sync_from_climate(climate.tons_moved, climate.restored_count);
    week.slab_line()
}

#[test]
fn tend_on_heartwood_inks_its_file_and_sanctuary_l_adds_the_two_rooms() {
    let dir = std::env::temp_dir().join(format!("powrush-u12-ink-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &dir);

    // Sanctuary already carries the walk's ink: 1 ton, 1 restored.
    let mut sanctuary = sanctuary_fresh_climate();
    sanctuary.tons_moved = 1;
    sanctuary.restored_count = 1;
    write_hex_named(&HexClimateFile::from_parts(
        PlaceId::Sanctuary,
        sanctuary.clone(),
        sanctuary_fresh_standing(),
    ))
    .expect("sanctuary file");

    // Depths is never visited — it must not gate the sum.
    assert!(read_hex_named(PlaceId::Depths).is_none());

    // Standing on Heartwood with a blank room, one Tend at the pipe.
    let mut bind = bind_on(heartwood_stub_climate(), heartwood_stub_standing());
    assert_eq!(bind.climate.restored_count, 0);
    ink_room_tend(PlaceId::Heartwood, &mut bind);

    // The ink is on Heartwood's own file, without waiting for a leave.
    let heartwood_file = read_hex_named(PlaceId::Heartwood).expect("heartwood file written");
    assert_eq!(heartwood_file.hex_id, "heartwood");
    assert_eq!(heartwood_file.climate.restored_count, 1, "second well ink");
    assert_eq!(heartwood_file.climate.tons_moved, 0, "a Tend is not a haul");
    assert!(
        heartwood_file.climate.restored_count > 0 || heartwood_file.climate.tons_moved > 0,
        "the hex file must not be blank after a Tend"
    );

    // Sanctuary's file was not touched by tending Heartwood — no leak.
    let sanctuary_file = read_hex_named(PlaceId::Sanctuary).expect("sanctuary file");
    assert_eq!(sanctuary_file.climate.tons_moved, 1);
    assert_eq!(sanctuary_file.climate.restored_count, 1);

    // Back on Sanctuary: the House bill adds both rooms, the yard line does not.
    let bill = house_week_bill_from_disk(PlaceId::Sanctuary, &sanctuary);
    assert_eq!(bill.tons_moved, 1, "1 Sanctuary + 0 Heartwood");
    assert_eq!(bill.restored_count, 2, "1 Sanctuary + 1 Heartwood");

    let spoken_house = house_week_line(&bill);
    let spoken_yard = yard_line(&sanctuary);
    assert_eq!(spoken_house, "House week · 1 tons · 2 restored");
    assert_eq!(spoken_yard, "this week · 1 tons · 1 restored");
    assert_ne!(
        spoken_house, spoken_yard,
        "the House line must outgrow the yard once two rooms have ink"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
