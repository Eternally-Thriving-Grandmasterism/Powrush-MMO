//! Lived-hour Embassy — Slice 8 (v23.2.12)
//!
//! After the Proof Pack, the lamp is live. E Request seat. Dies in Peace.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::embassy::Embassy;

use crate::coop_voice::VoiceYard;
use crate::fabricator::FabricatorYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::HourSacred;
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Clone, Default)]
pub struct EmbassyYard {
    pub embassy: Embassy,
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

fn mark_embassy_lamp(
    hour: Res<HourSacred>,
    yard: Res<EmbassyYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.embassy_lamp = hour.charter_skin_live()
        && yard.embassy.lamp_live
        && !yard.embassy.seated
        && !voice.sash_open
        && !ledger.sash_open;
}

fn handle_embassy(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    fab: Res<FabricatorYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut yard: ResMut<EmbassyYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !hour.charter_skin_live() {
        return;
    }
    yard.embassy.ensure_lamp(&fab.fab.pack);
    if !yard.embassy.lamp_live {
        return;
    }
    if voice.sash_open || ledger.sash_open {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    if yard.embassy.seated {
        return;
    }
    let step = yard.embassy.request_seat();
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
    mut root: Query<&mut Visibility, With<EmbassySlabRoot>>,
    mut text_q: Query<&mut Text, With<EmbassySlabText>>,
) {
    let show = hour.charter_skin_live() && yard.embassy.lamp_live;
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
    }
}
