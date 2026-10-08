//! Lived-hour Charter tutorial — Slice 3 (v23.2.7) + door hint (v23.2.28) + pack (v23.2.29)
//!
//! Q on Frontier: found House, then extractor → depot → hauler → two stops → arrival.
//! After-D3: Q plate shows Seal · … when house seals are dressed (heritage string only).
//! Peace slab speaks Tab only after a first-hour allocate.
//!
//! CARD FLESH-FACTORY-SLAB — the existing factory `slab_line` may name the Place
//! (`HexTravelState::chip_name`). Absent travel keeps each slab byte for byte.
//! No gold. No Market. No second HUD.
//! Peak memory, cited: walked · tended · week was the bill · yard remembered.
//!
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hour_two::HourTwoPack;
use shared::space_law::{HexFlag, SpaceSession};
use shared::vertical_factory::VerticalFactory;

use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::hud_anchor_registry::{HudSlab, FACTORY};
use crate::title_screen::{TITLE_BORDER, TITLE_PLATE_BG, TITLE_TEXT_PRIMARY};
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::input::{InputMapSet, PlayerInput};
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::title_screen::HouseLabel;
use shared::pause_ledger_face::{lethal_sign_row, q_plate_seal_line, HEX_ADMITS_HARM};

/// CARD FLESH-FACTORY-SLAB — `{place} · {slab}` when a chip is present.
/// `None` returns the bare slab. One string. No second widget.
fn factory_slab_line(bare: &str, place: Option<&str>) -> String {
    match place {
        Some(place) => format!("{place} · {bare}"),
        None => bare.to_string(),
    }
}

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
                    top: FACTORY.top(),
                    left: FACTORY.left(),
                    width: Val::Px(520.0),
                    margin: FACTORY.margin(),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: TITLE_PLATE_BG.into(),
                border_color: TITLE_BORDER.into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            FactorySlabRoot,
            HudSlab(FACTORY.id),
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
    if crate::hour_sacred::try_plant_house(&mut hour, &mut yard.factory) {
        return;
    }
    if hour.hex() == HexFlag::Peace || !hour.charter_skin_live() {
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
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
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
        factory_slab_line(
            &yard.factory.slab_line(),
            travel.as_ref().map(|state| state.chip_name()),
        )
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

    /// CARD HUD-ANCHOR-REGISTRY-2B — Factory position is the registry, Style is the coded literal.
    #[test]
    fn factory_style_byte_identical_to_coded_place() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_systems(Startup, spawn_factory_slab);
        app.update();
        let mut q = app
            .world_mut()
            .query_filtered::<&Style, With<FactorySlabRoot>>();
        let style = q.single(app.world()).clone();
        let coded = Style {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Percent(50.0),
            width: Val::Px(520.0),
            margin: UiRect::left(Val::Px(-260.0)),
            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
            justify_content: JustifyContent::Center,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        };
        assert_eq!(style, coded);
        assert_eq!(style.top, FACTORY.top());
        assert_eq!(style.left, FACTORY.left());
        assert_eq!(style.margin, FACTORY.margin());
        assert_eq!(style.width, Val::Px(FACTORY.width));
    }

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

    /// Playtest H2-Q: the Q key in Peace is a no-op for founding.
    #[test]
    fn q_key_on_peace_does_not_found() {
        use bevy::input::keyboard::{Key, KeyboardInput};
        use bevy::input::ButtonState;
        use bevy::input::InputPlugin as BevyInputPlugin;
        use crate::input::PlayerInput;
        use crate::thriving_moments::ThrivingMoments;

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(BevyInputPlugin);
        app.insert_resource(HourSacred {
            session: SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        });
        app.insert_resource(FactoryYard {
            factory: VerticalFactory::default(),
        });
        app.insert_resource(PlayerInput::default());
        app.insert_resource(ThrivingMoments::default());
        app.add_systems(Update, handle_factory_q);
        app.update();

        let window = Entity::PLACEHOLDER;
        app.world_mut().send_event(KeyboardInput {
            key_code: soft_play_bindings::BUILD_WHEEL,
            logical_key: Key::Character("q".into()),
            state: ButtonState::Pressed,
            window,
        });
        app.update();

        assert_eq!(app.world().resource::<HourSacred>().hex(), HexFlag::Peace);
        assert!(!app.world().resource::<FactoryYard>().factory.founded);
        assert!(!app.world().resource::<HourSacred>().charter_skin_live());
    }

    /// Playtest H2-Q: Q on the visitor ridge plants house-local.
    #[test]
    fn q_key_off_peace_plants_house() {
        use bevy::input::keyboard::{Key, KeyboardInput};
        use bevy::input::ButtonState;
        use bevy::input::InputPlugin as BevyInputPlugin;
        use crate::hour_sacred::try_ridge_tab;
        use crate::input::PlayerInput;
        use crate::thriving_moments::ThrivingMoments;

        let mut hour = HourSacred {
            session: SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert!(try_ridge_tab(&mut hour, true));

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(BevyInputPlugin);
        app.insert_resource(hour);
        app.insert_resource(FactoryYard {
            factory: VerticalFactory::default(),
        });
        app.insert_resource(PlayerInput::default());
        app.insert_resource(ThrivingMoments::default());
        app.add_systems(Update, handle_factory_q);
        app.update();

        let window = Entity::PLACEHOLDER;
        app.world_mut().send_event(KeyboardInput {
            key_code: soft_play_bindings::BUILD_WHEEL,
            logical_key: Key::Character("q".into()),
            state: ButtonState::Pressed,
            window,
        });
        app.update();

        let hour = app.world().resource::<HourSacred>();
        assert!(hour.charter_skin_live());
        assert_eq!(hour.session.charter_id.as_deref(), Some("house-local"));
        assert!(app.world().resource::<FactoryYard>().factory.founded);
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

    /// CARD FLESH-FACTORY-SLAB — Place prefixes the Q slab; no travel keeps it exact.
    #[test]
    fn flesh_factory_slab_names_chip_or_keeps_slab() {
        use shared::hex_travel::PlaceId;

        let plant = "Q plant a House stake (Frontier)";
        let unfounded = VerticalFactory::default();
        assert_eq!(unfounded.slab_line(), plant);
        assert_eq!(factory_slab_line(&unfounded.slab_line(), None), plant);

        let mut founded = VerticalFactory::default();
        founded.found_house();
        let next = "Q next · reserve 3 · nodes 0 · stops 0";
        assert_eq!(founded.slab_line(), next);
        assert_eq!(factory_slab_line(&founded.slab_line(), None), next);

        let cases = [
            (PlaceId::Sanctuary, "Sanctuary Prime"),
            (PlaceId::Heartwood, "Heartwood"),
            (PlaceId::Depths, "Depths"),
        ];
        for (id, name) in cases {
            assert_eq!(id.chip_name(), name);
            let travel = crate::hex_travel::HexTravelState { current: id };
            assert_eq!(travel.chip_name(), id.chip_name());
            let chip = travel.chip_name();
            assert_eq!(
                factory_slab_line(plant, Some(chip)),
                format!("{name} · {plant}")
            );
            assert_eq!(
                factory_slab_line(next, Some(chip)),
                format!("{name} · {next}")
            );
        }

        for sample in [
            factory_slab_line(plant, Some(PlaceId::Sanctuary.chip_name())),
            factory_slab_line(next, Some(PlaceId::Heartwood.chip_name())),
            factory_slab_line(plant, Some(PlaceId::Depths.chip_name())),
            factory_slab_line(plant, None),
            factory_slab_line(next, None),
        ] {
            let low = sample.to_lowercase();
            assert!(!low.contains("gold"), "{sample}");
            assert!(!low.contains("market"), "{sample}");
            assert!(!low.contains("xp"), "{sample}");
            assert!(!low.contains("hud"), "{sample}");
            assert!(!low.contains("online"), "{sample}");
            assert!(!sample.contains("Threshold"), "{sample}");
        }
    }
}
