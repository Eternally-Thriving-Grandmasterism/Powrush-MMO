//! Lived-hour Co-op Voice — Slice 4 (v23.2.8)
//!
//! G opens Voice (quorum cards). E on the beacon votes aye; Digit2 nay.
//! Dies in Peace. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::coop_voice::CoopVoice;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::HourSacred;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

/// Client wrap. Shared `CoopVoice` stays Bevy-free (HourSacred / FactoryYard pattern).
#[derive(Resource, Debug, Clone, Default)]
pub struct VoiceYard {
    pub voice: CoopVoice,
    pub sash_open: bool,
}

#[derive(Component)]
struct VoiceSlabRoot;
#[derive(Component)]
struct VoiceSlabText;

pub struct CoopVoicePlugin;

impl Plugin for CoopVoicePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoiceYard>()
            .add_systems(Startup, spawn_voice_slab)
            .add_systems(PreUpdate, mark_beacon_voice)
            .add_systems(Update, (handle_voice, update_voice_slab));
    }
}

fn spawn_voice_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(88.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(560.0),
                    margin: UiRect::left(Val::Px(-280.0)),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.06, 0.07, 0.10, 0.92).into(),
                border_color: Color::srgba(0.72, 0.86, 0.98, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            VoiceSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.88, 0.94, 1.0),
                        ..default()
                    },
                ),
                VoiceSlabText,
            ));
        });
}

fn mark_beacon_voice(
    hour: Res<HourSacred>,
    yard: Res<VoiceYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.beacon_voice = hour.charter_skin_live()
        && yard.sash_open
        && yard.voice.open_card().is_some();
}

fn handle_voice(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    mut yard: ResMut<VoiceYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if hour.hex() == shared::space_law::HexFlag::Peace {
        yard.sash_open = false;
        return;
    }
    if !hour.charter_skin_live() {
        yard.sash_open = false;
        return;
    }
    yard.voice.ensure_tutorial();
    if keyboard.just_pressed(soft_play_bindings::SASH) {
        yard.sash_open = !yard.sash_open;
        return;
    }
    if !yard.sash_open {
        return;
    }
    let aye = keyboard.just_pressed(soft_play_bindings::INTERACT)
        || keyboard.just_pressed(KeyCode::Digit1);
    let nay = keyboard.just_pressed(KeyCode::Digit2);
    if !aye && !nay {
        return;
    }
    let step = yard.voice.vote_local(aye);
    if step == "carried" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstVoice,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_voice_slab(
    hour: Res<HourSacred>,
    yard: Res<VoiceYard>,
    mut root: Query<&mut Visibility, With<VoiceSlabRoot>>,
    mut text_q: Query<&mut Text, With<VoiceSlabText>>,
) {
    let show = hour.charter_skin_live() && yard.sash_open;
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
    let line = yard.voice.beacon_line();
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
    fn peace_keeps_sash_closed() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = VoiceYard::default();
        assert!(!yard.sash_open);
        assert!(yard.voice.cards.is_empty());
    }
}
