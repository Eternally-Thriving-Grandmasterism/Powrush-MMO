//! Lived-hour Hybrid Matrix v0 — Slice 13 (v23.2.17)
//!
//! After a tend is offered, E Attune. Stability 1. Dies in Peace.
//! Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::hybrid_matrix::HybridMatrix;

use crate::coop_voice::VoiceYard;
use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::HourSacred;
use crate::ledger_bind::LedgerYard;
use crate::soft_play_bindings;
use crate::species_redemption::RedemptionYard;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Clone, Default)]
pub struct HybridYard {
    pub matrix: HybridMatrix,
}

#[derive(Component)]
struct HybridSlabRoot;
#[derive(Component)]
struct HybridSlabText;

pub struct HybridMatrixPlugin;

impl Plugin for HybridMatrixPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HybridYard>()
            .add_systems(Startup, spawn_hybrid_slab)
            .add_systems(PreUpdate, mark_hybrid_near)
            .add_systems(Update, (handle_hybrid, update_hybrid_slab));
    }
}

fn spawn_hybrid_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(244.0),
                    right: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.08, 0.11, 0.92).into(),
                border_color: Color::srgba(0.70, 0.78, 0.92, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            HybridSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.88, 0.90, 0.98),
                        ..default()
                    },
                ),
                HybridSlabText,
            ));
        });
}

fn hybrid_live(hour: &HourSacred, red: &RedemptionYard) -> bool {
    hour.charter_skin_live() && red.state.events > 0
}

fn mark_hybrid_near(
    hour: Res<HourSacred>,
    red: Res<RedemptionYard>,
    yard: Res<HybridYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.hybrid_near = hybrid_live(&hour, &red)
        && !yard.matrix.attuned
        && !voice.sash_open
        && !ledger.sash_open;
}

fn handle_hybrid(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    red: Res<RedemptionYard>,
    voice: Res<VoiceYard>,
    ledger: Res<LedgerYard>,
    mut yard: ResMut<HybridYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !hybrid_live(&hour, &red) {
        return;
    }
    yard.matrix.reveal();
    if voice.sash_open || ledger.sash_open {
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::INTERACT) {
        return;
    }
    let step = yard.matrix.attune();
    if step == "attuned" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstHybrid,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_hybrid_slab(
    hour: Res<HourSacred>,
    red: Res<RedemptionYard>,
    yard: Res<HybridYard>,
    mut root: Query<&mut Visibility, With<HybridSlabRoot>>,
    mut text_q: Query<&mut Text, With<HybridSlabText>>,
) {
    let show = hybrid_live(&hour, &red);
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
    let line = yard.matrix.slab_line();
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
    fn peace_hides_hybrid() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let red = RedemptionYard::default();
        assert!(!hybrid_live(&hour, &red));
    }
}
