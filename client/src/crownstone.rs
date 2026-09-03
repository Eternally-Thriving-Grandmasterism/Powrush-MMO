//! Lived-hour Crownstone set-piece — Slice 11 (v23.2.15)
//!
//! After the embassy seat, E Witness. Path stays Unset. Dies in Peace.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::crownstone::CrownstoneState;

use crate::coop_voice::VoiceYard;
use crate::embassy::EmbassyYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::HourSacred;
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Clone, Default)]
pub struct CrownstoneYard {
    pub stone: CrownstoneState,
}

#[derive(Component)]
struct CrownstoneSlabRoot;
#[derive(Component)]
struct CrownstoneSlabText;

pub struct CrownstonePlugin;

impl Plugin for CrownstonePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CrownstoneYard>()
            .add_systems(Startup, spawn_crownstone_slab)
            .add_systems(PreUpdate, mark_crownstone_near)
            .add_systems(Update, (handle_crownstone, update_crownstone_slab));
    }
}

fn spawn_crownstone_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(164.0),
                    right: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.10, 0.06, 0.14, 0.92).into(),
                border_color: Color::srgba(0.78, 0.52, 0.95, 0.55).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            CrownstoneSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.94, 0.84, 1.0),
                        ..default()
                    },
                ),
                CrownstoneSlabText,
            ));
        });
}

fn stone_live(hour: &HourSacred, embassy: &EmbassyYard) -> bool {
    hour.charter_skin_live() && embassy.embassy.seated
}

fn mark_crownstone_near(
    hour: Res<HourSacred>,
    yard: Res<CrownstoneYard>,
    embassy: Res<EmbassyYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.crownstone_near = stone_live(&hour, &embassy)
        && !yard.stone.witnessed
        && !voice.sash_open
        && !ledger.sash_open;
}

fn handle_crownstone(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    embassy: Res<EmbassyYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut yard: ResMut<CrownstoneYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !stone_live(&hour, &embassy) {
        return;
    }
    yard.stone.reveal();
    if voice.sash_open || ledger.sash_open {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    let step = yard.stone.witness();
    if step == "witnessed" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstCrownstone,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_crownstone_slab(
    hour: Res<HourSacred>,
    embassy: Res<EmbassyYard>,
    yard: Res<CrownstoneYard>,
    mut root: Query<&mut Visibility, With<CrownstoneSlabRoot>>,
    mut text_q: Query<&mut Text, With<CrownstoneSlabText>>,
) {
    let show = stone_live(&hour, &embassy);
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
    let line = yard.stone.slab_line();
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
    fn peace_hides_stone() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let embassy = EmbassyYard::default();
        assert!(!stone_live(&hour, &embassy));
    }
}
