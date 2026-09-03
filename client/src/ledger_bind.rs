//! Lived-hour Ledger — Slice 6 (v23.2.10)
//!
//! L opens the board. E Bind then escort. Digit3 opts DeclaredLethal (tariff).
//! Default win is Bind. No F-key. Dies in Peace. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::ledger_bind::LedgerBoard;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::hour_sacred::HourSacred;
use crate::infra_spill::EvidenceYard;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Clone, Default)]
pub struct LedgerYard {
    pub board: LedgerBoard,
    pub sash_open: bool,
}

#[derive(Component)]
struct LedgerSlabRoot;
#[derive(Component)]
struct LedgerSlabText;

pub struct LedgerBindPlugin;

impl Plugin for LedgerBindPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LedgerYard>()
            .add_systems(Startup, spawn_ledger_slab)
            .add_systems(PreUpdate, mark_ledger_bind)
            .add_systems(Update, (handle_ledger, update_ledger_slab));
    }
}

fn spawn_ledger_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(16.0),
                    left: Val::Px(16.0),
                    width: Val::Px(560.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.06, 0.09, 0.92).into(),
                border_color: Color::srgba(0.70, 0.78, 0.92, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            LedgerSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.88, 0.92, 1.0),
                        ..default()
                    },
                ),
                LedgerSlabText,
            ));
        });
}

fn mark_ledger_bind(
    hour: Res<HourSacred>,
    yard: Res<LedgerYard>,
    mut epi: ResMut<FirstHarvestEpiphany>,
) {
    epi.ledger_bind = hour.charter_skin_live() && yard.sash_open;
}

fn handle_ledger(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hour: ResMut<HourSacred>,
    evidence: Res<EvidenceYard>,
    mut yard: ResMut<LedgerYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !hour.charter_skin_live() {
        yard.sash_open = false;
        return;
    }
    let hash = evidence
        .witness
        .pack
        .as_ref()
        .map(|p| p.hash.clone())
        .unwrap_or_else(|| "local-i2".into());
    yard.board.ensure_i2(hash);
    if keyboard.just_pressed(soft_play_bindings::LEDGER) {
        yard.sash_open = !yard.sash_open;
        return;
    }
    if !yard.sash_open {
        return;
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        let step = yard.board.opt_lethal_local();
        if step == "lethal" {
            hour.session.warrant.x = hour.session.warrant.x.max(10.0);
        }
        return;
    }
    let go = keyboard.just_pressed(soft_play_bindings::INTERACT)
        || keyboard.just_pressed(KeyCode::Digit1);
    if !go {
        return;
    }
    let step = yard.board.act_local();
    if step == "settled" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstBind,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_ledger_slab(
    hour: Res<HourSacred>,
    yard: Res<LedgerYard>,
    mut root: Query<&mut Visibility, With<LedgerSlabRoot>>,
    mut text_q: Query<&mut Text, With<LedgerSlabText>>,
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
    let line = yard.board.sash_line();
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
    fn peace_keeps_ledger_closed() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = LedgerYard::default();
        assert!(!yard.sash_open);
        assert!(yard.board.contracts.is_empty());
    }
}
