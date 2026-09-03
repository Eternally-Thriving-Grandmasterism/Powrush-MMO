//! Lived-hour species redemption v0 — Slice 12 (v23.2.16)
//!
//! After the stone is seen, E Offer a tend to the Sylvaris grove. Dies in Peace.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::species_redemption::SpeciesRedemptionState;

use crate::coop_voice::VoiceYard;
use crate::crownstone::CrownstoneYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::HourSacred;
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Clone, Default)]
pub struct RedemptionYard {
    pub state: SpeciesRedemptionState,
}

#[derive(Component)]
struct RedemptionSlabRoot;
#[derive(Component)]
struct RedemptionSlabText;

pub struct SpeciesRedemptionPlugin;

impl Plugin for SpeciesRedemptionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RedemptionYard>()
            .add_systems(Startup, spawn_redemption_slab)
            .add_systems(PreUpdate, mark_redemption_near)
            .add_systems(Update, (handle_redemption, update_redemption_slab));
    }
}

fn spawn_redemption_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(204.0),
                    right: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.06, 0.10, 0.07, 0.92).into(),
                border_color: Color::srgba(0.55, 0.82, 0.48, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            RedemptionSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.84, 0.96, 0.78),
                        ..default()
                    },
                ),
                RedemptionSlabText,
            ));
        });
}

fn grove_live(hour: &HourSacred, stone: &CrownstoneYard) -> bool {
    hour.charter_skin_live() && stone.stone.witnessed
}

fn mark_redemption_near(
    hour: Res<HourSacred>,
    stone: Res<CrownstoneYard>,
    yard: Res<RedemptionYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.redemption_near = grove_live(&hour, &stone)
        && yard.state.events == 0
        && !voice.sash_open
        && !ledger.sash_open;
}

fn handle_redemption(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    stone: Res<CrownstoneYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut yard: ResMut<RedemptionYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !grove_live(&hour, &stone) {
        return;
    }
    yard.state.reveal();
    if voice.sash_open || ledger.sash_open {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    let step = yard.state.offer_tend();
    if step == "tended" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstRedemption,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_redemption_slab(
    hour: Res<HourSacred>,
    stone: Res<CrownstoneYard>,
    yard: Res<RedemptionYard>,
    mut root: Query<&mut Visibility, With<RedemptionSlabRoot>>,
    mut text_q: Query<&mut Text, With<RedemptionSlabText>>,
) {
    let show = grove_live(&hour, &stone);
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
    let line = yard.state.slab_line();
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
    fn peace_hides_grove() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let stone = CrownstoneYard::default();
        assert!(!grove_live(&hour, &stone));
    }
}
