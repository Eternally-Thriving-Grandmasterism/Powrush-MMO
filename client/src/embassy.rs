//! Lived-hour Embassy — Slice 8 (v23.2.12)
//!
//! After the Proof Pack, the lamp is live. E Request seat. Dies in Peace.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::embassy::Embassy;
use shared::hour_two::HourTwoPack;

use crate::coop_voice::VoiceYard;
use crate::fabricator::FabricatorYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::{read_hour_two_json, HourSacred};
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use shared::fabricator::ProofPack;
use shared::space_law::HexFlag;

#[derive(Resource, Debug, Clone)]
pub struct EmbassyYard {
    pub embassy: Embassy,
}

impl Default for EmbassyYard {
    fn default() -> Self {
        if let Some(raw) = read_hour_two_json() {
            return Self {
                embassy: HourTwoPack::from_json(&raw).embassy,
            };
        }
        Self {
            embassy: Embassy::default(),
        }
    }
}

#[derive(Component)]
struct EmbassySlabRoot;
#[derive(Component)]
struct EmbassySlabText;

pub struct EmbassyPlugin;

impl Plugin for EmbassyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EmbassyYard>()
            .add_systems(Startup, spawn_embassy_slab)
            .add_systems(PreUpdate, mark_embassy_lamp)
            .add_systems(Update, (handle_embassy, update_embassy_slab));
    }
}

fn spawn_embassy_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(124.0),
                    right: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.07, 0.12, 0.92).into(),
                border_color: Color::srgba(0.82, 0.74, 0.95, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            EmbassySlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.94, 0.90, 1.0),
                        ..default()
                    },
                ),
                EmbassySlabText,
            ));
        });
}

fn on_heartwood_stub(travel: Option<&crate::hex_travel::HexTravelState>) -> bool {
    travel
        .map(|t| t.current == shared::hex_travel::PlaceId::Heartwood)
        .unwrap_or(false)
}

fn mark_embassy_lamp(
    hour: Res<HourSacred>,
    yard: Res<EmbassyYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    // Heartwood embassy lamp is display-only. Spatial refuse is shared/heartwood_lamp.
    if on_heartwood_stub(travel.as_deref()) {
        epi.embassy_lamp = false;
        return;
    }
    epi.embassy_lamp = hour.complete
        && hour.charter_skin_live()
        && yard.embassy.lamp_live
        && !yard.embassy.seated
        && !voice.sash_open
        && !ledger.sash_open;
}

/// Embassy lamp after Proof Pack. E Request seat. Peace / no pack is a no-op.
pub fn try_request_embassy_seat(
    hour: &HourSacred,
    pack: &ProofPack,
    embassy: &mut Embassy,
) -> Option<&'static str> {
    if hour.hex() == HexFlag::Peace || !hour.complete || !hour.charter_skin_live() {
        return None;
    }
    embassy.ensure_lamp(pack);
    if !embassy.lamp_live || embassy.seated {
        return None;
    }
    Some(embassy.request_seat())
}

fn handle_embassy(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    fab: Res<FabricatorYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut yard: ResMut<EmbassyYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    // Heartwood stub: same Peace E (tend), not Embassy seat.
    if on_heartwood_stub(travel.as_deref()) {
        return;
    }
    if hour.complete && hour.charter_skin_live() {
        yard.embassy.ensure_lamp(&fab.fab.pack);
    }
    if voice.sash_open || ledger.sash_open {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    let Some(step) = try_request_embassy_seat(&hour, &fab.fab.pack, &mut yard.embassy) else {
        return;
    };
    if step == "seated" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstEmbassy,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_embassy_slab(
    hour: Res<HourSacred>,
    yard: Res<EmbassyYard>,
    travel: Option<Res<crate::hex_travel::HexTravelState>>,
    mut root: Query<&mut Visibility, With<EmbassySlabRoot>>,
    mut text_q: Query<&mut Text, With<EmbassySlabText>>,
) {
    let show = !on_heartwood_stub(travel.as_deref())
        && hour.complete
        && hour.charter_skin_live()
        && yard.embassy.lamp_live;
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
    let line = yard.embassy.slab_line();
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
    fn peace_hides_lamp() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = EmbassyYard::default();
        assert!(!yard.embassy.lamp_live);
        let mut embassy = yard.embassy.clone();
        assert!(try_request_embassy_seat(&hour, &ProofPack::default(), &mut embassy).is_none());
    }

    /// Playtest H3-SEAT: lamp after Proof Pack, E Request seat, Hour three held.
    #[test]
    fn h3_seat_e_requests_book() {
        use crate::hour_sacred::{
            try_mark_hour_three_held, try_mark_hour_two_held, try_plant_house, try_ridge_tab,
        };
        use shared::fabricator::Fabricator;
        use shared::space_law::SpaceSession;
        use shared::vertical_factory::VerticalFactory;

        let mut hour = HourSacred {
            session: SpaceSession::default(),
            complete: false,
            hour_three_complete: false,
        };
        assert!(try_ridge_tab(&mut hour, true));
        let mut factory = VerticalFactory::default();
        assert!(try_plant_house(&mut hour, &mut factory));
        assert!(try_mark_hour_two_held(&mut hour, true, true));

        let mut fab = Fabricator::default();
        assert_eq!(fab.craft_next(), "planted");
        assert_eq!(fab.craft_next(), "crafted");
        assert_eq!(fab.craft_next(), "unlocked");
        assert!(fab.pack.unlocked());

        let mut embassy = Embassy::default();
        assert_eq!(
            try_request_embassy_seat(&hour, &fab.pack, &mut embassy),
            Some("seated")
        );
        assert!(embassy.lamp_live);
        assert!(embassy.seated);
        assert!(try_mark_hour_three_held(&mut hour, fab.pack.unlocked(), embassy.seated));
        assert!(hour.hour_three_complete);
        assert_eq!(
            shared::hour_two::HourTwoPack {
                complete: true,
                hour_three_complete: true,
                ..Default::default()
            }
            .line(true),
            "Hour three held · the book is yours"
        );
    }
}
