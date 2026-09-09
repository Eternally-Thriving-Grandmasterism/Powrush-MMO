//! Lived-hour Ledger — Slice 6 + S3 face + stranger wait line (v23.2.61)
//!
//! L opens the board. E Bind then escort. Digit3 / Settings confirm the hex sign.
//! Soft cue after book: sash may append "this hex admits harm · off". Never Peace boot.
//! only after Hour three held. Default win is Bind. No F-key. Peace silent.
//! L2 face when Settled: House · week tons+restored · lethal only if declared.
//! Bind-only / pre-Settled: *Not your charter* / *the ledger waits* — never blank.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hex_travel::{house_week_footer, read_hex_named, PlaceId, LOCAL_HEXES};
use shared::hour_two::HourTwoPack;
use shared::ledger_bind::{ContractState, LedgerBoard};
use shared::pause_ledger_face::{
    house_week_line, ledger_sash_body, lethal_sign_row, HEX_ADMITS_HARM_OFF,
};
use shared::shard_climate::ShardClimate;
use shared::week_audit::WeekAudit;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hex_travel::HexTravelState;
use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::infra_spill::EvidenceYard;
use crate::input::{InputMapSet, PlayerInput};
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::title_screen::{HouseLabel, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};
use crate::ui_above_world::{LivedUiPlate, LIVED_UI_Z_LEDGER};

#[derive(Resource, Debug, Clone)]
pub struct LedgerYard {
    pub board: LedgerBoard,
    pub sash_open: bool,
}

impl Default for LedgerYard {
    fn default() -> Self {
        if let Some(raw) = read_hour_two_json() {
            return Self {
                board: HourTwoPack::from_json(&raw).board,
                sash_open: false,
            };
        }
        Self {
            board: LedgerBoard::default(),
            sash_open: false,
        }
    }
}

/// The House bill the Ledger plate speaks: the live room summed with the other
/// Offline rooms that persist a week. Threshold rides the Heartwood file, so
/// there is no fourth room to read. Read-only — summing never writes a hex, so
/// isolation stays 0.
#[derive(Resource, Debug, Clone, Default)]
pub struct HouseWeekBill {
    pub week: WeekAudit,
}

impl HouseWeekBill {
    /// `House week · Σtons · Σrestored` — not the single-yard week wording.
    pub fn line(&self) -> String {
        house_week_line(&self.week)
    }
}

#[derive(Component)]
struct LedgerSlabRoot;
#[derive(Component)]
struct LedgerSlabText;

pub struct LedgerBindPlugin;

impl Plugin for LedgerBindPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LedgerYard>()
            .init_resource::<HouseWeekBill>()
            .add_systems(Startup, spawn_ledger_slab)
            .add_systems(PreUpdate, (mark_ledger_bind, refresh_house_week_bill))
            .add_systems(
                Update,
                (handle_ledger, stamp_complete, update_ledger_slab).after(InputMapSet),
            );
    }
}

fn spawn_ledger_slab(mut commands: Commands) {
    // Opaque plate + Global z — Title contrast law; soft GPU must not alpha into fog.
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(16.0),
                    left: Val::Px(16.0),
                    width: Val::Px(560.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: Color::srgb(0.70, 0.78, 0.92).into(),
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(LIVED_UI_Z_LEDGER),
                ..default()
            },
            LedgerSlabRoot,
            LivedUiPlate,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: TITLE_TEXT_PRIMARY,
                        ..default()
                    },
                ),
                LedgerSlabText,
            ));
        });
}

fn mark_ledger_bind(
    hour: Res<HourSacred>,
    yard: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.ledger_bind = hour.charter_skin_live() && yard.sash_open;
}

fn handle_ledger(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_input: Res<PlayerInput>,
    mut hour: ResMut<HourSacred>,
    evidence: Res<EvidenceYard>,
    mut yard: ResMut<LedgerYard>,
    mut bind: ResMut<LivedHourBind>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    // L / pad North always toggles sash — pre-Settled shows wait line (never blank panel).
    if keyboard.just_pressed(soft_play_bindings::LEDGER) || player_input.sheet_l {
        yard.sash_open = !yard.sash_open;
        return;
    }
    if !hour.charter_skin_live() {
        // Keep sash visible with wait copy; no Bind acts until charter.
        return;
    }
    let hash = evidence
        .witness
        .pack
        .as_ref()
        .map(|p| p.hash.clone())
        .unwrap_or_else(|| "local-i2".into());
    yard.board.ensure_i2(hash);
    if !yard.sash_open {
        return;
    }
    // L1 hex sign / Digit3 — confirm only after Settled + book. Reuses standing.
    // Peace hour never reaches here. No new Peace key. Sanctuary E unchanged.
    if keyboard.just_pressed(KeyCode::Digit3) {
        let settled = yard
            .board
            .open()
            .map(|c| c.state == ContractState::Settled)
            .unwrap_or(false)
            || hour.complete;
        if bind.standing.declared_lethal {
            if bind.standing.clear_lethal() {
                bind.refresh_climate_slab();
                bind.persist();
            }
            return;
        }
        if !bind
            .standing
            .confirm_hex_sign(settled, hour.hour_three_complete)
        {
            // Book / Settled missing — plate already shows wait / not your charter.
            return;
        }
        hour.session.warrant.x = hour.session.warrant.x.max(10.0);
        let _paid = bind.climate.on_lethal_declare();
        bind.refresh_climate_slab();
        bind.persist();
        return;
    }
    let go = keyboard.just_pressed(soft_play_bindings::INTERACT)
        || keyboard.just_pressed(KeyCode::Digit1);
    if !go {
        return;
    }
    let step = yard.board.act_local();
    if step == "settled" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstBind,
            time.elapsed_seconds_f64(),
        );
    }
}

fn stamp_complete(
    mut hour: ResMut<HourSacred>,
    evidence: Res<EvidenceYard>,
    yard: Res<LedgerYard>,
) {
    if hour.complete {
        return;
    }
    let settled = yard
        .board
        .open()
        .map(|c| c.state == ContractState::Settled)
        .unwrap_or(false);
    if hour.charter_skin_live() && evidence.witness.seen && settled {
        hour.complete = true;
    }
}

/// Quiet Ledger hint after Hour three. Empty string before the book.
pub fn lethal_soft_clause(hour_three_held: bool, already_lethal: bool) -> &'static str {
    if hour_three_held && !already_lethal {
        " · this hex admits harm · off"
    } else {
        ""
    }
}

/// Re-read the room files only when the live room or the yard moved.
fn refresh_house_week_bill(
    bind: Res<LivedHourBind>,
    travel: Option<Res<HexTravelState>>,
    mut bill: ResMut<HouseWeekBill>,
) {
    let travel_moved = travel.as_ref().map(|t| t.is_changed()).unwrap_or(false);
    if !bill.is_added() && !bind.is_changed() && !travel_moved {
        return;
    }
    let week = match travel.as_ref() {
        Some(travel) => house_week_bill_from_disk(travel.current, &bind.climate),
        None => bind.week.clone(),
    };
    if bill.week != week {
        bill.week = week;
    }
}

fn update_ledger_slab(
    hour: Res<HourSacred>,
    yard: Res<LedgerYard>,
    bind: Res<LivedHourBind>,
    bill: Res<HouseWeekBill>,
    house_label: Res<HouseLabel>,
    mut root: Query<&mut Visibility, With<LedgerSlabRoot>>,
    mut text_q: Query<&mut Text, With<LedgerSlabText>>,
) {
    // Show whenever L sash is open — wait line before Settled, full face after.
    let show = yard.sash_open;
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    if !show {
        return;
    }
    let settled = yard
        .board
        .open()
        .map(|c| c.state == ContractState::Settled)
        .unwrap_or(false)
        || hour.complete;
    let charter = hour.charter_skin_live();
    // The face speaks the yard the body stands in. The House bill is its own line.
    let face = ledger_sash_body(
        charter,
        settled,
        &house_label.house,
        &yard_week(&bind.climate),
        bind.standing.declared_lethal,
    );
    let line = if !charter {
        // Pre-charter: one wait line — never blank.
        face
    } else if !settled {
        // Bind-only before Settled: wait line + Bind sash (still never blank).
        use shared::pause_ledger_face::wait_line_before_settled;
        let wait = wait_line_before_settled(true);
        let face = with_house_week(&face, &bill);
        let sash = yard.board.sash_line();
        format!("{wait}\n{face}\n{sash}")
    } else {
        let face = with_house_week(&face, &bill);
        // Soft discoverability only after the book.
        let mut sash = yard.board.sash_line();
        let sign = lethal_sign_row(
            true,
            hour.hour_three_complete,
            true,
            bind.standing.declared_lethal,
        );
        let clause = lethal_soft_clause(hour.hour_three_complete, bind.standing.declared_lethal);
        if !clause.is_empty() && !sash.contains("this hex admits harm") {
            sash = format!("{sash}{clause}");
        }
        format!("{face}\n{sign}\n{sash}")
    };
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
        }
    }
}

/// Week for the yard underfoot only — never the House bill.
fn yard_week(climate: &ShardClimate) -> WeekAudit {
    let mut week = WeekAudit::default();
    week.sync_from_climate(climate.tons_moved, climate.restored_count);
    week
}

/// Yard face, then the summed House bill on its own line.
fn with_house_week(face: &str, bill: &HouseWeekBill) -> String {
    format!("{face}\n{}", bill.line())
}

/// The House bill as the plate computes it: the live room plus whichever other
/// room files are on disk. A room with no file, or one still at 0/0, adds
/// nothing — two nonzero rooms are enough.
pub fn house_week_bill_from_disk(current: PlaceId, current_climate: &ShardClimate) -> WeekAudit {
    house_week_from_rooms(current, current_climate, read_hex_named)
}

fn house_week_from_rooms(
    current_place: PlaceId,
    current_climate: &ShardClimate,
    mut load_room: impl FnMut(PlaceId) -> Option<shared::hex_travel::HexClimateFile>,
) -> WeekAudit {
    let climates: Vec<_> = LOCAL_HEXES
        .into_iter()
        .filter_map(|place| {
            if place == current_place {
                Some(current_climate.clone())
            } else {
                load_room(place).map(|file| file.climate)
            }
        })
        .collect();
    house_week_footer(&climates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::HexFlag;

    /// L sash open on Sanctuary with the book, painted by the real system.
    fn painted_ledger_plate(yard_climate: ShardClimate, bill: HouseWeekBill) -> String {
        use bevy::MinimalPlugins;
        use shared::house_name::HouseName;
        use shared::space_law::{CharterKind, SpaceSession};

        let mut house = HouseName::default();
        house.skip();
        house.skip_seals();
        house.skip_heritage();

        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(HourSacred {
            session: SpaceSession {
                charter_id: Some("house-local".into()),
                hex: HexFlag::Frontier,
                kind: CharterKind::House,
                ..Default::default()
            },
            complete: true,
            hour_three_complete: true,
        });
        app.insert_resource(LedgerYard {
            board: LedgerBoard::default(),
            sash_open: true,
        });
        app.insert_resource(LivedHourBind {
            hour: shared::climate_node::LivedHour::new_demo(),
            climate: yard_climate,
            standing: shared::shard_standing::ShardStanding::default(),
            week: WeekAudit::default(),
            last_line: String::new(),
            guidance_hidden: false,
            focus_id: None,
            climate_slab: None,
        });
        app.insert_resource(bill);
        app.insert_resource(HouseLabel {
            house,
            persist_present: true,
            hour_two_held: true,
            book_held: true,
            settings_open: false,
            draft: String::new(),
            naming_offered: true,
            seals_offered: true,
        });
        app.add_systems(Startup, spawn_ledger_slab);
        app.add_systems(Update, update_ledger_slab);
        app.update();

        let world = app.world_mut();
        let mut q = world.query_filtered::<&Text, With<LedgerSlabText>>();
        q.iter(world)
            .next()
            .and_then(|t| t.sections.first())
            .map(|s| s.value.clone())
            .expect("ledger slab text")
    }

    /// End of the failed walk, on the plate: Sanctuary 1/1 underfoot while the
    /// House bill adds Heartwood in. L must speak both, and they must differ.
    #[test]
    fn sanctuary_l_paints_house_line_that_outgrows_the_yard() {
        let mut sanctuary = shared::hex_travel::sanctuary_fresh_climate();
        sanctuary.tons_moved = 1;
        sanctuary.restored_count = 1;

        let mut summed = WeekAudit::default();
        summed.sync_from_climate(3, 2);

        let plate = painted_ledger_plate(sanctuary, HouseWeekBill { week: summed });

        assert!(
            plate.contains("this week · 1 tons · 1 restored"),
            "yard week stays honest: {plate}"
        );
        assert!(
            plate.contains("House week · 3 tons · 2 restored"),
            "L speaks the summed House bill: {plate}"
        );
        assert!(!plate.to_lowercase().contains("market"));
        assert!(!plate.to_lowercase().contains("online"));
    }

    #[test]
    fn digit3_needs_hour_three_flag() {
        let hour = HourSacred {
            session: shared::space_law::SpaceSession::default(),
            complete: true,
            hour_three_complete: false,
        };
        assert!(!hour.hour_three_complete);
    }

    #[test]
    fn peace_keeps_ledger_closed() {
        let hour = HourSacred {
            session: shared::space_law::SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = LedgerYard {
            board: LedgerBoard::default(),
            sash_open: false,
        };
        assert!(!yard.sash_open);
        assert!(yard.board.contracts.is_empty());
    }

    #[test]
    fn lethal_soft_clause_only_after_book() {
        assert_eq!(lethal_soft_clause(false, false), "");
        assert_eq!(
            lethal_soft_clause(true, false),
            " · this hex admits harm · off"
        );
        assert_eq!(lethal_soft_clause(true, true), "");
        assert!(!lethal_soft_clause(true, false)
            .to_lowercase()
            .contains("combat"));
        assert!(!lethal_soft_clause(true, false).contains("kill"));
        assert!(lethal_soft_clause(true, false).contains(HEX_ADMITS_HARM_OFF));
    }

    #[test]
    fn ledger_face_strings_house_week_no_peer() {
        use shared::house_name::HouseName;
        use shared::pause_ledger_face::{face_is_steward_honest, ledger_sash_body};
        use shared::week_audit::WeekAudit;
        let mut house = HouseName::default();
        house.skip();
        let mut week = WeekAudit::default();
        week.sync_from_climate(2, 4);
        let face = ledger_sash_body(true, true, &house, &week, false);
        assert!(face.contains("Unnamed House"));
        assert!(face.contains("2 tons"));
        assert!(face.contains("4 restored"));
        assert!(!face.contains("lethal"));
        assert!(face_is_steward_honest(&face));
        assert!(!face.to_lowercase().contains("peer"));
    }

    /// The walked fail: Heartwood 0/0, Sanctuary 1/1, and L said "1 tons ·
    /// 1 restored" — the yard, indistinguishable from a sum. After a Tend in
    /// both rooms the House line must add them and read differently.
    #[test]
    fn sanctuary_house_line_adds_two_tended_rooms_and_is_not_the_yard() {
        use shared::hex_travel::stub_hex_file;

        let mut sanctuary = stub_hex_file(PlaceId::Sanctuary).climate;
        sanctuary.tons_moved = 1;
        sanctuary.restored_count = 1;

        let mut heartwood = stub_hex_file(PlaceId::Heartwood);
        heartwood.climate.tons_moved = 2;
        heartwood.climate.restored_count = 1;

        // Standing on Sanctuary, Depths never visited (no file on disk).
        let bill = HouseWeekBill {
            week: house_week_from_rooms(PlaceId::Sanctuary, &sanctuary, |place| {
                (place == PlaceId::Heartwood).then(|| heartwood.clone())
            }),
        };

        assert_eq!(bill.week.tons_moved, 3, "1 Sanctuary + 2 Heartwood");
        assert_eq!(bill.week.restored_count, 2, "1 Sanctuary + 1 Heartwood");
        assert_eq!(bill.line(), "House week · 3 tons · 2 restored");

        let yard = yard_week(&sanctuary);
        assert_eq!(yard.slab_line(), "this week · 1 tons · 1 restored");
        assert_ne!(
            bill.line(),
            yard.slab_line(),
            "the House line must not be a clone of the yard"
        );

        // Both lines land on the plate, and the House bill is its own line.
        let plate = with_house_week(&yard.slab_line(), &bill);
        assert!(plate.contains("this week · 1 tons · 1 restored"));
        assert!(plate.contains("House week · 3 tons · 2 restored"));
        assert!(plate.lines().count() >= 2);
        assert!(!plate.to_lowercase().contains("market"));

        // A quiet third room is not a gate: an untended Depths file adds 0 and
        // the same A+B stands. Two nonzero rooms are enough to prove the sum.
        let quiet_depths = stub_hex_file(PlaceId::Depths);
        assert_eq!(quiet_depths.climate.tons_moved, 0);
        let with_quiet_depths =
            house_week_from_rooms(PlaceId::Sanctuary, &sanctuary, |place| match place {
                PlaceId::Heartwood => Some(heartwood.clone()),
                PlaceId::Depths => Some(quiet_depths.clone()),
                PlaceId::Sanctuary => None,
            });
        assert_eq!(with_quiet_depths, bill.week);
    }

    #[test]
    fn sanctuary_ledger_footer_sums_persisted_offline_rooms() {
        use shared::hex_travel::stub_hex_file;

        let mut sanctuary = stub_hex_file(PlaceId::Sanctuary).climate;
        sanctuary.tons_moved = 2;
        sanctuary.restored_count = 1;

        let mut heartwood = stub_hex_file(PlaceId::Heartwood);
        heartwood.climate.tons_moved = 3;
        heartwood.climate.restored_count = 4;

        let mut depths = stub_hex_file(PlaceId::Depths);
        depths.climate.tons_moved = 5;
        depths.climate.restored_count = 6;

        let week = house_week_from_rooms(PlaceId::Sanctuary, &sanctuary, |place| match place {
            PlaceId::Sanctuary => None,
            PlaceId::Heartwood => Some(heartwood.clone()),
            PlaceId::Depths => Some(depths.clone()),
        });

        assert_eq!(week.tons_moved, 10);
        assert_eq!(week.restored_count, 11);
        assert_eq!(week.slab_line(), "this week · 10 tons · 11 restored");
        assert_eq!(LOCAL_HEXES.len(), 3, "Threshold persists on Heartwood");
    }

    /// Depths counts once it persists a week; no fourth room is ever read.
    #[test]
    fn three_offline_rooms_add_and_no_fifth_place_is_invented() {
        use shared::hex_travel::stub_hex_file;

        let mut sanctuary = stub_hex_file(PlaceId::Sanctuary).climate;
        sanctuary.tons_moved = 1;
        sanctuary.restored_count = 1;

        let mut heartwood = stub_hex_file(PlaceId::Heartwood);
        heartwood.climate.tons_moved = 2;
        heartwood.climate.restored_count = 1;

        let mut depths = stub_hex_file(PlaceId::Depths);
        depths.climate.tons_moved = 4;
        depths.climate.restored_count = 3;

        let mut asked = Vec::new();
        let week = house_week_from_rooms(PlaceId::Sanctuary, &sanctuary, |place| {
            asked.push(place);
            match place {
                PlaceId::Heartwood => Some(heartwood.clone()),
                PlaceId::Depths => Some(depths.clone()),
                PlaceId::Sanctuary => None,
            }
        });

        assert_eq!(week.tons_moved, 7);
        assert_eq!(week.restored_count, 5);
        assert_eq!(
            asked,
            vec![PlaceId::Heartwood, PlaceId::Depths],
            "only the rooms that persist a week are read"
        );
    }

    #[test]
    fn house_footer_uses_live_room_instead_of_stale_disk_copy() {
        use shared::hex_travel::stub_hex_file;

        let mut live_heartwood = stub_hex_file(PlaceId::Heartwood).climate;
        live_heartwood.tons_moved = 7;
        live_heartwood.restored_count = 2;

        let mut stale_heartwood = stub_hex_file(PlaceId::Heartwood);
        stale_heartwood.climate.tons_moved = 99;
        stale_heartwood.climate.restored_count = 99;

        let week = house_week_from_rooms(PlaceId::Heartwood, &live_heartwood, |place| {
            (place == PlaceId::Heartwood).then(|| stale_heartwood.clone())
        });

        assert_eq!(week.tons_moved, 7);
        assert_eq!(week.restored_count, 2);
    }

    #[test]
    fn ledger_face_lethal_only_when_declared() {
        use shared::house_name::HouseName;
        use shared::pause_ledger_face::{ledger_sash_body, LETHAL_DECLARED_LINE};
        use shared::week_audit::WeekAudit;
        let mut house = HouseName::default();
        house.confirm("River House");
        let mut week = WeekAudit::default();
        week.sync_from_climate(1, 0);
        let quiet = ledger_sash_body(true, true, &house, &week, false);
        assert!(!quiet.contains(LETHAL_DECLARED_LINE));
        let lethal = ledger_sash_body(true, true, &house, &week, true);
        assert!(lethal.contains(LETHAL_DECLARED_LINE));
        assert!(lethal.contains("River House"));
        assert!(lethal.contains("1 tons"));
    }

    #[test]
    fn pre_settled_l_shows_wait_line_not_blank() {
        use shared::house_name::HouseName;
        use shared::pause_ledger_face::{
            bind_only_before_settled_body, ledger_sash_body, wait_line_before_settled,
            LEDGER_WAITS, NOT_YOUR_CHARTER,
        };
        use shared::week_audit::WeekAudit;
        let mut house = HouseName::default();
        house.skip();
        let week = WeekAudit::default();
        let early = ledger_sash_body(false, false, &house, &week, false);
        assert_eq!(early, NOT_YOUR_CHARTER);
        assert!(!early.is_empty());
        assert_eq!(
            bind_only_before_settled_body(true, false),
            Some(LEDGER_WAITS)
        );
        assert_eq!(wait_line_before_settled(false), NOT_YOUR_CHARTER);
        // Charter live keeps L2 face (Unnamed + week 0/0); lethal absent.
        let face = ledger_sash_body(true, false, &house, &week, false);
        assert!(face.contains("Unnamed House"));
        assert!(face.contains("0 tons"));
        assert!(face.contains("0 restored"));
        assert!(!face.contains("lethal"));
        let settled0 = ledger_sash_body(true, true, &house, &week, false);
        assert!(settled0.contains("Unnamed House"));
        assert!(!settled0.contains("lethal"));
    }

    #[test]
    fn l1_confirm_requires_settled_and_book_no_ton_mint() {
        use shared::pause_ledger_face::{lethal_sign_row, HEX_ADMITS_HARM, LEDGER_WAITS};
        use shared::shard_climate::ShardClimate;
        use shared::shard_standing::ShardStanding;
        let mut standing = ShardStanding::default();
        let mut climate = ShardClimate::default();
        climate.tons_moved = 4;
        climate.reserve_pool = 2;
        assert!(!standing.confirm_hex_sign(false, true));
        assert!(!standing.confirm_hex_sign(true, false));
        assert_eq!(lethal_sign_row(true, false, true, false), LEDGER_WAITS);
        assert!(standing.confirm_hex_sign(true, true));
        let paid = climate.on_lethal_declare();
        assert_eq!(paid, 1);
        assert_eq!(climate.tons_moved, 4);
        assert!(standing.declared_lethal);
        assert_eq!(lethal_sign_row(true, true, true, true), HEX_ADMITS_HARM);
        // Sanctuary Use stays E — confirm is Digit3 / Settings row, not default Use.
        assert_eq!(soft_play_bindings::INTERACT, KeyCode::KeyE);
        assert_ne!(KeyCode::Digit3, soft_play_bindings::INTERACT);
    }
}
