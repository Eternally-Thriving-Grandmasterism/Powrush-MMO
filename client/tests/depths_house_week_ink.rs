//! P4 — Depths restored ink must land in the House week sum.
//!
//! Fail this was meant to catch: restore looks fine on the Depths slab, but
//! Sanctuary L still speaks Sanctuary-only because Depths stayed 0/0 or never
//! flushed. Own binary so POWRUSH_USER_DIR is this process alone.

use powrush_client::depths_landing::restore_depths_hex;
use powrush_client::ledger_bind::house_week_bill_from_disk;
use powrush_client::lived_hour_bind::LivedHourBind;

use shared::climate_node::LivedHour;
use shared::hex_travel::{
    apply_travel_named, depths_stub_climate, depths_stub_standing, read_hex_named,
    sanctuary_fresh_climate, sanctuary_fresh_standing, write_hex_named, HexClimateFile, PlaceId,
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
fn depths_restore_then_leave_adds_to_sanctuary_house_week() {
    let dir = std::env::temp_dir().join(format!("powrush-p4-depths-ink-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    std::env::set_var(shared::user_persist::USER_DIR_OVERRIDE_ENV, &dir);

    let mut sanctuary = sanctuary_fresh_climate();
    sanctuary.tons_moved = 1;
    sanctuary.restored_count = 1;
    write_hex_named(&HexClimateFile::from_parts(
        PlaceId::Sanctuary,
        sanctuary.clone(),
        sanctuary_fresh_standing(),
    ))
    .expect("sanctuary file");

    // Before Depths ink: House equals the yard.
    let before = house_week_bill_from_disk(PlaceId::Sanctuary, &sanctuary);
    assert_eq!(before.tons_moved, 1);
    assert_eq!(before.restored_count, 1);
    assert_eq!(house_week_line(&before), "House week · 1 tons · 1 restored");

    // Enter Depths the way Places does: flush Sanctuary, load Depths stub.
    let loaded = apply_travel_named(
        true,
        true,
        PlaceId::Sanctuary,
        PlaceId::Depths,
        &sanctuary,
        &sanctuary_fresh_standing(),
    )
    .expect("enter Depths");
    assert_eq!(loaded.hex_id, "depths");
    assert_eq!(loaded.climate.restored_count, 0);

    let mut bind = bind_on(loaded.climate, loaded.standing);
    assert!(restore_depths_hex(&mut bind).expect("restore write"));
    assert!(bind.climate.restored_count >= 1, "live U_w after restore");

    let depths_after_restore = read_hex_named(PlaceId::Depths).expect("depths file after restore");
    assert!(
        depths_after_restore.climate.restored_count >= 1,
        "Depths file must carry U_w before leave"
    );
    assert_eq!(depths_after_restore.climate.tons_moved, 0, "restore is not a Take");

    // Leave Depths → Sanctuary: save-on-exit flushes the Depths file again.
    let back = apply_travel_named(
        true,
        true,
        PlaceId::Depths,
        PlaceId::Sanctuary,
        &bind.climate,
        &bind.standing,
    )
    .expect("leave Depths");
    assert_eq!(back.hex_id, "sanctuary");

    let depths_after_leave = read_hex_named(PlaceId::Depths).expect("depths flushed on leave");
    assert!(
        depths_after_leave.climate.restored_count >= 1,
        "leave must not wipe Depths U_w back to 0"
    );

    let sanctuary_live = back.climate;
    let after = house_week_bill_from_disk(PlaceId::Sanctuary, &sanctuary_live);
    assert_eq!(after.tons_moved, 1, "Sanctuary tons only; Depths added no haul");
    assert_eq!(
        after.restored_count, 2,
        "1 Sanctuary + 1 Depths restored — House must outgrow the yard"
    );
    assert_eq!(house_week_line(&after), "House week · 1 tons · 2 restored");
    assert_eq!(yard_line(&sanctuary_live), "this week · 1 tons · 1 restored");
    assert_ne!(
        house_week_line(&after),
        yard_line(&sanctuary_live),
        "House line must not equal Sanctuary-only after Depths inked"
    );

    // Market never in the sum — LOCAL_HEXES stays three Peace rooms.
    assert_eq!(shared::hex_travel::LOCAL_HEXES.len(), 3);
    assert!(!depths_is_market_guard());

    let _ = std::fs::remove_dir_all(&dir);
    std::env::remove_var(shared::user_persist::USER_DIR_OVERRIDE_ENV);
}

fn depths_is_market_guard() -> bool {
    shared::hex_travel::depths_is_market()
}

#[test]
fn depths_stub_climate_matches_peace_landing() {
    let c = depths_stub_climate();
    let s = depths_stub_standing();
    assert_eq!(c.hex_id, "depths");
    assert_eq!(s.hex_id, "depths");
    assert_eq!(c.restored_count, 0);
    assert_eq!(c.tons_moved, 0);
}
