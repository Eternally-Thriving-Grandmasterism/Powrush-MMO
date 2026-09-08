//! Lived-hour Charter tutorial — Slice 3 (v23.2.7) + door hint (v23.2.28) + pack (v23.2.29)
//!
//! Q on Frontier: found House, then extractor → depot → hauler → two stops → arrival.
//! After-D3: Q plate shows Seal · … when house seals are dressed (heritage string only).
//! Peace slab speaks Tab only after a first-hour allocate. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hour_two::HourTwoPack;
use shared::space_law::{CharterKind, HexFlag, SpaceSession};
use shared::vertical_factory::VerticalFactory;

use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::input::{InputMapSet, PlayerInput};
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::title_screen::HouseLabel;
use shared::pause_ledger_face::{lethal_sign_row, q_plate_seal_line, HEX_ADMITS_HARM};

/// Client wrap. Shared `VerticalFactory` stays Bevy-free (same as HourSacred / SpaceSession).
#[derive(Resource, Debug, Clone)]
pub struct FactoryYard {
    pub factory: VerticalFactory,
}

impl Default for FactoryYard {
    fn default() -> Self {
        if let Some(raw) = read_hour_two_json() {
            return Self {
                factory: HourTwoPack::from_json(&raw).factory,
            };
        }
        Self {
            factory: VerticalFactory::default(),
        }
    }
}

#[derive(Component)]
struct FactorySlabRoot;
#[derive(Component)]
struct FactorySlabText;

pub struct VerticalFactoryPlugin;

impl Plugin for VerticalFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FactoryYard>()
            .add_systems(Startup, spawn_factory_slab)
            .add_systems(Update, (handle_factory_q, update_factory_slab).after(InputMapSet));
    }
}

fn spawn_factory_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(16.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(520.0),
                    margin: UiRect::left(Val::Px(-260.0)),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.06, 0.90).into(),
                border_color: Color::srgba(0.70, 0.88, 0.55, 0.45).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            FactorySlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.90, 0.98, 0.82),
                        ..default()
                    },
                ),
                FactorySlabText,
            ));
        });
}

fn handle_factory_q(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_input: Res<PlayerInput>,
    mut hour: ResMut<HourSacred>,
    mut yard: ResMut<FactoryYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    // Q / pad West — house / factory face (INPUT_CANON sheets).
    if !(keyboard.just_pressed(soft_play_bindings::BUILD_WHEEL) || player_input.sheet_q) {
        return;
    }
    if hour.hex() == HexFlag::Peace {
        return;
    }
    if hour.session.charter_id.is_none() {
        yard.factory.found_house();
        hour.session.charter_id = Some("house-local".into());
        hour.session.kind = CharterKind::House;
        return;
    }
    if !hour.charter_skin_live() {
        return;
    }
    // Slice 7: after the crate arrives, Q is the fabricator wheel.
    if yard.factory.tutorial_complete() {
        return;
    }
    let step = yard.factory.advance();
    if step == "arrived" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstArrival,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_factory_slab(
    hour: Res<HourSacred>,
    yard: Res<FactoryYard>,
    evidence: Option<Res<crate::infra_spill::EvidenceYard>>,
    ledger: Option<Res<crate::ledger_bind::LedgerYard>>,
    bind: Option<Res<LivedHourBind>>,
    house_label: Option<Res<HouseLabel>>,
    mut root: Query<&mut Visibility, With<FactorySlabRoot>>,
    mut text_q: Query<&mut Text, With<FactorySlabText>>,
) {
    let ready = bind
        .as_ref()
        .map(|b| {
            SpaceSession::hour_two_door_ready(b.hour.allocation.flow, b.hour.allocation.reserve)
        })
        .unwrap_or(false);
    let settled = hour.complete
        || ledger
            .as_ref()
            .and_then(|l| l.board.open())
            .map(|c| c.state == shared::ledger_bind::ContractState::Settled)
            .unwrap_or(false);
    let declared = bind
        .as_ref()
        .map(|b| b.standing.declared_lethal)
        .unwrap_or(false);
    let pack = HourTwoPack {
        session: hour.session.clone(),
        factory: yard.factory.clone(),
        witness: evidence
            .map(|e| e.witness.clone())
            .unwrap_or_default(),
        board: ledger.map(|l| l.board.clone()).unwrap_or_default(),
        fabricator: Default::default(),
        embassy: Default::default(),
        complete: hour.complete,
        hour_three_complete: hour.hour_three_complete,
    };
    let show = hour.hex() != HexFlag::Peace || ready || pack.complete;
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
    let mut line = if pack.complete || hour.hex() == HexFlag::Peace || hour.session.peace_visitor_on_frontier()
    {
        pack.line(ready).to_string()
    } else {
        yard.factory.slab_line()
    };
    // After-D3 comfort: show current seal on Q plate when dressed.
    if let Some(hl) = house_label.as_ref() {
        if let Some(seal) = q_plate_seal_line(&hl.house) {
            if !line.contains("Seal ·") {
                line = format!("{line} · {seal}");
            }
        }
    }
    // L1 hex sign — Settled + book only. Wait copy stays on Ledger/Settings, not Q.
    if settled && hour.hour_three_complete {
        let sign = lethal_sign_row(true, true, true, declared);
        if !line.contains(HEX_ADMITS_HARM) {
            line = format!("{line} · {sign}");
        }
    }
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

    #[test]
    fn peace_does_not_found() {
        let hour = HourSacred {
            session: SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = FactoryYard {
            factory: VerticalFactory::default(),
        };
        assert!(!yard.factory.founded);
    }

    #[test]
    fn q_plate_seal_line_when_dressed() {
        use shared::house_name::{HouseName, SEAL_WELL};
        let mut house = HouseName::default();
        house.skip();
        assert!(q_plate_seal_line(&house).is_none());
        house.set_seals(&[SEAL_WELL]);
        house.confirm_seals();
        assert_eq!(q_plate_seal_line(&house).as_deref(), Some("Seal · Well"));
    }

    #[test]
    fn q_plate_hex_sign_only_after_settled_and_book() {
        use shared::pause_ledger_face::{HEX_ADMITS_HARM_OFF, LEDGER_WAITS};
        assert_eq!(
            lethal_sign_row(false, false, false, false),
            shared::pause_ledger_face::NOT_YOUR_CHARTER
        );
        assert_eq!(lethal_sign_row(true, false, true, false), LEDGER_WAITS);
        assert_eq!(
            lethal_sign_row(true, true, true, false),
            HEX_ADMITS_HARM_OFF
        );
        assert_eq!(lethal_sign_row(true, true, true, true), HEX_ADMITS_HARM);
    }
}
