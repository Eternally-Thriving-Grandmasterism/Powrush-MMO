//! Lived-hour skirmish well — Slice 15 (v23.2.19)
//!
//! E contests the first well. Dawn after loss. Lives in Peace. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::skirmish_well::{SkirmishWell, WellHold, CONTEST_REACH, WELL_ANCHORS};

use crate::coop_voice::VoiceYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::human_presence::SoftPresence;
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

const HOLD_SECS: f64 = 6.0;

#[derive(Resource, Debug, Clone)]
pub struct WellYard {
    pub well: SkirmishWell,
    pub hold_until: f64,
}

impl Default for WellYard {
    fn default() -> Self {
        Self {
            well: SkirmishWell::default(),
            hold_until: 0.0,
        }
    }
}

#[derive(Component)]
struct WellSlabRoot;
#[derive(Component)]
struct WellSlabText;

pub struct SkirmishWellPlugin;

impl Plugin for SkirmishWellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WellYard>()
            .add_systems(Startup, spawn_well_slab)
            .add_systems(PreUpdate, mark_well_near)
            .add_systems(Update, (pressure_hold, handle_well, update_well_slab));
    }
}

fn spawn_well_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(132.0),
                    left: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.07, 0.09, 0.08, 0.92).into(),
                border_color: Color::srgba(0.55, 0.78, 0.62, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            WellSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.82, 0.96, 0.84),
                        ..default()
                    },
                ),
                WellSlabText,
            ));
        });
}

fn near_first_well(presence: &SoftPresence) -> bool {
    let (x, y, z) = WELL_ANCHORS[0];
    presence.position.distance(Vec3::new(x, y, z)) <= CONTEST_REACH
}

fn mark_well_near(
    presence: Res<SoftPresence>,
    yard: Res<WellYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.well_near = near_first_well(&presence)
        && yard.well.wants_interact()
        && !voice.sash_open
        && !ledger.sash_open
        && !epi.peace_visitor;
}

fn pressure_hold(time: Res<Time>, mut yard: ResMut<WellYard>) {
    if yard.well.hold != WellHold::Human {
        return;
    }
    if time.elapsed_seconds_f64() < yard.hold_until {
        return;
    }
    let _ = yard.well.traveler_answers();
}

fn handle_well(
    keyboard: Res<ButtonInput<KeyCode>>,
    presence: Res<SoftPresence>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    epi: Res<FirstHarvestEpiphany>,
    mut yard: ResMut<WellYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !near_first_well(&presence) {
        return;
    }
    if voice.sash_open || ledger.sash_open || epi.peace_visitor {
        return;
    }
    yard.well.reveal();
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    let step = yard.well.act();
    if step == "won" {
        yard.hold_until = time.elapsed_seconds_f64() + HOLD_SECS;
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstWell,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_well_slab(
    presence: Res<SoftPresence>,
    yard: Res<WellYard>,
    mut root: Query<&mut Visibility, With<WellSlabRoot>>,
    mut text_q: Query<&mut Text, With<WellSlabText>>,
) {
    let show = near_first_well(&presence);
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
    let line = yard.well.slab_line();
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
    fn far_from_spawn_is_not_near() {
        let p = SoftPresence::default();
        assert!(!near_first_well(&p));
    }
}
