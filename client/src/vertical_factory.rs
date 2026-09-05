//! Lived-hour Charter tutorial — Slice 3 (v23.2.7) + door hint (v23.2.28) + pack (v23.2.29)
//!
//! Q on Frontier: found House, then extractor → depot → hauler → two stops → arrival.
//! Peace slab speaks Tab only after a first-hour allocate. Contact: info@Rathor.ai

use std::fs;

use bevy::prelude::*;

use shared::hour_two::HourTwoPack;
use shared::space_law::{CharterKind, HexFlag, SpaceSession};
use shared::vertical_factory::VerticalFactory;

use crate::hour_sacred::{HourSacred, HOUR_TWO_PATH};
use crate::lived_hour_bind::LivedHourBind;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

/// Client wrap. Shared `VerticalFactory` stays Bevy-free (same as HourSacred / SpaceSession).
#[derive(Resource, Debug, Clone)]
pub struct FactoryYard {
    pub factory: VerticalFactory,
}

impl Default for FactoryYard {
    fn default() -> Self {
        if let Ok(raw) = fs::read_to_string(HOUR_TWO_PATH) {
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
            .add_systems(Update, (handle_factory_q, update_factory_slab));
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
    mut hour: ResMut<HourSacred>,
    mut yard: ResMut<FactoryYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !keyboard.just_pressed(soft_play_bindings::BUILD_WHEEL) {
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
    mut root: Query<&mut Visibility, With<FactorySlabRoot>>,
    mut text_q: Query<&mut Text, With<FactorySlabText>>,
) {
    let ready = bind
        .map(|b| {
            SpaceSession::hour_two_door_ready(b.hour.allocation.flow, b.hour.allocation.reserve)
        })
        .unwrap_or(false);
    let pack = HourTwoPack {
        session: hour.session.clone(),
        factory: yard.factory.clone(),
        witness: evidence
            .map(|e| e.witness.clone())
            .unwrap_or_default(),
        board: ledger.map(|l| l.board.clone()).unwrap_or_default(),
        complete: hour.complete,
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
    let line = if pack.complete || hour.hex() == HexFlag::Peace || hour.session.peace_visitor_on_frontier()
    {
        pack.line(ready).to_string()
    } else {
        yard.factory.slab_line()
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

    #[test]
    fn peace_does_not_found() {
        let hour = HourSacred {
            session: SpaceSession::default(),
            complete: false,
        };
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = FactoryYard {
            factory: VerticalFactory::default(),
        };
        assert!(!yard.factory.founded);
    }
}
