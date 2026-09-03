//! Lived-hour Charter tutorial — Slice 3 (v23.2.7)
//!
//! Q on Frontier: found House, then extractor → depot → hauler → two stops → arrival.
//! Dies in Peace. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::space_law::{CharterKind, HexFlag};
use shared::vertical_factory::VerticalFactory;

use crate::hour_sacred::HourSacred;
use crate::soft_play_bindings;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Component)]
struct FactorySlabRoot;
#[derive(Component)]
struct FactorySlabText;

pub struct VerticalFactoryPlugin;

impl Plugin for VerticalFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VerticalFactory>()
            .add_systems(Startup, spawn_factory_slab)
            .add_systems(Update, (handle_factory_q, update_factory_slab));
    }
}

fn spawn_factory_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(16.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(520.0),
                    margin: UiRect::left(Val::Px(-260.0)),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.06, 0.90).into(),
                border_color: Color::srgba(0.70, 0.88, 0.55, 0.45).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            FactorySlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.90, 0.98, 0.82),
                        ..default()
                    },
                ),
                FactorySlabText,
            ));
        });
}

fn handle_factory_q(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut hour: ResMut<HourSacred>,
    mut factory: ResMut<VerticalFactory>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    if !keyboard.just_pressed(soft_play_bindings::BUILD_WHEEL) {
        return;
    }
    if hour.hex() == HexFlag::Peace {
        return;
    }
    if hour.session.charter_id.is_none() {
        factory.found_house();
        hour.session.charter_id = Some("house-local".into());
        hour.session.kind = CharterKind::House;
        return;
    }
    if !hour.charter_skin_live() {
        return;
    }
    let step = factory.advance();
    if step == "arrived" {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstArrival,
            time.elapsed_seconds_f64(),
        );
    }
}

fn update_factory_slab(
    hour: Res<HourSacred>,
    factory: Res<VerticalFactory>,
    mut root: Query<&mut Visibility, With<FactorySlabRoot>>,
    mut text_q: Query<&mut Text, With<FactorySlabText>>,
) {
    let show = hour.hex() != HexFlag::Peace;
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
    let line = factory.slab_line();
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
    fn peace_does_not_found() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        let f = VerticalFactory::default();
        assert!(!f.founded);
    }
}
