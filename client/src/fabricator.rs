//! Lived-hour fabricator — Slice 7 (v23.2.11)
//!
//! After the crate arrives, Q plants a fabricator then runs MendSpool and LaneCrate.
//! Dies in Peace. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::fabricator::Fabricator;
use shared::space_law::HexFlag;

use crate::hour_sacred::HourSacred;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};
use crate::vertical_factory::FactoryYard;

#[derive(Resource, Debug, Clone, Default)]
pub struct FabricatorYard {
    pub fab: Fabricator,
}

#[derive(Component)]
struct FabSlabRoot;
#[derive(Component)]
struct FabSlabText;

pub struct FabricatorPlugin;

impl Plugin for FabricatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FabricatorYard>()
            .add_systems(Startup, spawn_fab_slab)
            .add_systems(Update, (handle_fab_q, update_fab_slab));
    }
}

fn spawn_fab_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(88.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(520.0),
                    margin: UiRect::left(Val::Px(-260.0)),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.07, 0.06, 0.10, 0.90).into(),
                border_color: Color::srgba(0.78, 0.70, 0.92, 0.45).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            FabSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.92, 0.88, 1.0),
                        ..default()
                    },
                ),
                FabSlabText,
            ));
        });
}

fn handle_fab_q(
    keyboard: Res<ButtonInput<KeyCode>>,
    hour: Res<HourSacred>,
    factory: Res<FactoryYard>,
    mut yard: ResMut<FabricatorYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !keyboard.just_pressed(soft_play_bindings::BUILD_WHEEL) {
        return;
    }
    if hour.hex() == HexFlag::Peace || !hour.charter_skin_live() {
        return;
    }
    if !factory.factory.tutorial_complete() {
        return;
    }
    let step = yard.fab.craft_next();
    if step == "unlocked" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstProofPack,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_fab_slab(
    hour: Res<HourSacred>,
    factory: Res<FactoryYard>,
    yard: Res<FabricatorYard>,
    mut root: Query<&mut Visibility, With<FabSlabRoot>>,
    mut text_q: Query<&mut Text, With<FabSlabText>>,
) {
    let show = hour.charter_skin_live() && factory.factory.tutorial_complete();
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
    let line = yard.fab.slab_line();
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
    fn peace_does_not_plant() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let yard = FabricatorYard::default();
        assert!(!yard.fab.planted);
    }
}
