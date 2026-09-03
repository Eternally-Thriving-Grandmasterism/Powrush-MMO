//! Lived-hour War week — Slice 9 (v23.2.13)
//!
//! Tab Chart declares the week. Score is tons + restored. Dies in Peace.
//! No combat key. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::space_law::HexFlag;
use shared::war_week::WarWeek;

use crate::fabricator::FabricatorYard;
use crate::hour_sacred::HourSacred;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Clone, Default)]
pub struct WarYard {
    pub week: WarWeek,
    pub sash_open: bool,
}

#[derive(Component)]
struct WarSlabRoot;
#[derive(Component)]
struct WarSlabText;

pub struct WarWeekPlugin;

impl Plugin for WarWeekPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WarYard>()
            .add_systems(Startup, spawn_war_slab)
            .add_systems(Update, (handle_war_chart, ingest_graph, update_war_slab));
    }
}

fn spawn_war_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(52.0),
                    right: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.09, 0.06, 0.92).into(),
                border_color: Color::srgba(0.70, 0.88, 0.45, 0.50).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            WarSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.88, 0.98, 0.72),
                        ..default()
                    },
                ),
                WarSlabText,
            ));
        });
}

fn handle_war_chart(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    mut yard: ResMut<WarYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if hour.hex() == HexFlag::Peace || !hour.charter_skin_live() {
        yard.sash_open = false;
        return;
    }
    if !keyboard.just_pressed(soft_play_bindings::CHART) {
        return;
    }
    yard.sash_open = !yard.sash_open;
    if yard.sash_open && !yard.week.declared {
        let step = yard.week.declare();
        if step == "declared" {
            fire_thriving(
                &mut moments,
                ThrivingKind::FirstWarWeek,
                time.elapsed_seconds_f64(),
            );
        }
    }
}

fn ingest_graph(fab: Res<FabricatorYard>, mut yard: ResMut<WarYard>) {
    let tons = if fab.fab.pack.logi { 1.0 } else { 0.0 };
    let restored = if fab.fab.pack.repair { 1 } else { 0 };
    yard.week.ingest(tons, restored);
}

fn update_war_slab(
    hour: Res<HourSacred>,
    yard: Res<WarYard>,
    mut root: Query<&mut Visibility, With<WarSlabRoot>>,
    mut text_q: Query<&mut Text, With<WarSlabText>>,
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
    let line = yard.week.line();
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
    fn peace_does_not_declare() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = WarYard::default();
        assert!(!yard.week.declared);
        assert!(!yard.sash_open);
    }
}
