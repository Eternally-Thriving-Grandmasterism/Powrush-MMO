//! Lived-hour Ledger — Slice 6 + S3 face + stranger wait line (v23.2.61)
//!
//! L opens the board. E Bind then escort. Digit3 opts DeclaredLethal (tariff)
//! Soft cue after book: sash may append "· 3 optional". Never Peace boot.
//! only after Hour three held. Default win is Bind. No F-key. Peace silent.
//! L2 face when Settled: House · week tons+restored · lethal only if declared.
//! Bind-only / pre-Settled: *Not your charter* / *the ledger waits* — never blank.
//! Contact: info@Rathor.ai

use std::fs;

use bevy::prelude::*;

use shared::hour_two::HourTwoPack;
use shared::ledger_bind::{ContractState, LedgerBoard};
use shared::pause_ledger_face::ledger_sash_body;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::{HourSacred, HOUR_TWO_PATH};
use crate::lived_hour_bind::LivedHourBind;
use crate::title_screen::{HouseLabel, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};
use crate::ui_above_world::LIVED_UI_Z_LEDGER;
use crate::infra_spill::EvidenceYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Clone)]
pub struct LedgerYard {
    pub board: LedgerBoard,
    pub sash_open: bool,
}

impl Default for LedgerYard {
    fn default() -> Self {
        if let Ok(raw) = fs::read_to_string(HOUR_TWO_PATH) {
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

#[derive(Component)]
struct LedgerSlabRoot;
#[derive(Component)]
struct LedgerSlabText;

pub struct LedgerBindPlugin;

impl Plugin for LedgerBindPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LedgerYard>()
            .add_systems(Startup, spawn_ledger_slab)
            .add_systems(PreUpdate, mark_ledger_bind)
            .add_systems(Update, (handle_ledger, stamp_complete, update_ledger_slab));
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
    mut hour: ResMut<HourSacred>,
    evidence: Res<EvidenceYard>,
    mut yard: ResMut<LedgerYard>,
    mut bind: ResMut<LivedHourBind>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    // L always toggles sash — pre-Settled shows wait line (never blank panel).
    if keyboard.just_pressed(soft_play_bindings::LEDGER) {
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
    // Ledger 3 / Digit3 — DeclaredLethal only after the book. Peace hour never reaches here.
    if keyboard.just_pressed(KeyCode::Digit3) {
        if bind.standing.declared_lethal {
            let step = yard.board.clear_lethal_local();
            if step == "cleared" {
                bind.standing.clear_lethal();
                bind.refresh_climate_slab();
                bind.persist();
            }
            return;
        }
        if !hour.hour_three_complete {
            // Resume without book cannot declare.
            return;
        }
        let step = yard.board.opt_lethal_local();
        if step == "lethal" {
            hour.session.warrant.x = hour.session.warrant.x.max(10.0);
            let _paid = bind.climate.on_lethal_declare();
            bind.standing.declare_lethal(true);
            bind.refresh_climate_slab();
            bind.persist();
        }
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

fn stamp_complete(mut hour: ResMut<HourSacred>, evidence: Res<EvidenceYard>, yard: Res<LedgerYard>) {
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
        " · 3 optional"
    } else {
        ""
    }
}

fn update_ledger_slab(
    hour: Res<HourSacred>,
    yard: Res<LedgerYard>,
    bind: Res<LivedHourBind>,
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
    let face = ledger_sash_body(
        charter,
        settled,
        &house_label.house,
        &bind.week,
        bind.standing.declared_lethal,
    );
    let line = if !charter {
        // Pre-charter: one wait line — never blank.
        face
    } else if !settled {
        // Bind-only before Settled: wait line + Bind sash (still never blank).
        use shared::pause_ledger_face::wait_line_before_settled;
        let wait = wait_line_before_settled(true);
        let sash = yard.board.sash_line();
        format!("{wait}\n{face}\n{sash}")
    } else {
        // Soft discoverability only after the book.
        let mut sash = yard.board.sash_line();
        let clause = lethal_soft_clause(hour.hour_three_complete, bind.standing.declared_lethal);
        if !clause.is_empty() && !sash.contains("3 optional") {
            sash = format!("{sash}{clause}");
        }
        format!("{face}\n{sash}")
    };
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::HexFlag;

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
        assert_eq!(lethal_soft_clause(true, false), " · 3 optional");
        assert_eq!(lethal_soft_clause(true, true), "");
        assert!(!lethal_soft_clause(true, false).to_lowercase().contains("combat"));
        assert!(!lethal_soft_clause(true, false).contains("kill"));
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
        assert_eq!(bind_only_before_settled_body(true, false), Some(LEDGER_WAITS));
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
}
