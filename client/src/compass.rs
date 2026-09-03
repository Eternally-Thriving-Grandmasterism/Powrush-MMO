//! Lived-hour Compass — Slice 14 (v23.2.18)
//!
//! Tells at live W 20 and 60. Peace silent. No extra key. Contact: info@Rathor.ai

use bevy::prelude::*;

use shared::compass;

use crate::hour_sacred::HourSacred;
use crate::thriving_moments::{fire_thriving, ThrivingKind, ThrivingMoments};

#[derive(Resource, Debug, Default)]
pub struct CompassYard {
    pub last: Option<&'static str>,
    pub fired: bool,
}

#[derive(Component)]
struct CompassSlabRoot;
#[derive(Component)]
struct CompassSlabText;

pub struct CompassPlugin;

impl Plugin for CompassPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CompassYard>()
            .add_systems(Startup, spawn_compass_slab)
            .add_systems(Update, (update_compass, update_compass_slab));
    }
}

fn spawn_compass_slab(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(92.0),
                    right: Val::Px(16.0),
                    width: Val::Px(420.0),
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                    justify_content: JustifyContent::FlexStart,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                background_color: Color::srgba(0.07, 0.08, 0.10, 0.90).into(),
                border_color: Color::srgba(0.62, 0.78, 0.88, 0.45).into(),
                visibility: Visibility::Hidden,
                ..default()
            },
            CompassSlabRoot,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.82, 0.90, 0.96),
                        ..default()
                    },
                ),
                CompassSlabText,
            ));
        });
}

fn update_compass(
    hour: Res<HourSacred>,
    mut yard: ResMut<CompassYard>,
    mut moments: ResMut<ThrivingMoments>,
    time: Res<Time>,
) {
    let line = compass::tell(&hour.session.warrant, hour.hex());
    if line.is_some() && yard.last.is_none() && !yard.fired {
        fire_thriving(
            &mut moments,
            ThrivingKind::FirstCompass,
            time.elapsed_seconds_f64(),
        );
        yard.fired = true;
    }
    yard.last = line;
}

fn update_compass_slab(
    yard: Res<CompassYard>,
    mut root: Query<&mut Visibility, With<CompassSlabRoot>>,
    mut text_q: Query<&mut Text, With<CompassSlabText>>,
) {
    let show = yard.last.is_some();
    for mut vis in &mut root {
        *vis = if show {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    let Some(line) = yard.last else {
        return;
    };
    for mut text in &mut text_q {
        if let Some(s) = text.sections.get_mut(0) {
            if s.value != line {
                s.value = line.to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::space_law::HexFlag;

    #[test]
    fn peace_hides_compass() {
        let hour = HourSacred::default();
        assert_eq!(hour.hex(), HexFlag::Peace);
        assert_eq!(compass::tell(&hour.session.warrant, hour.hex()), None);
    }
}
