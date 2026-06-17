//! client/rbe_ui_feedback.rs
//! Production-grade Bevy UI for RBE Harvest Feedback
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use crate::rbe_client_ui_sync::RbeUiSync;

#[derive(Component)]
pub struct HarvestFeedbackText;

pub struct RbeUiFeedbackPlugin;

impl Plugin for RbeUiFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_harvest_feedback_ui)
            .add_systems(Update, update_harvest_feedback_ui);
    }
}

fn spawn_harvest_feedback_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.7)),
            ..default()
        },
        HarvestFeedbackText,
    )).with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"), // replace with your font
                    font_size: 24.0,
                    color: Color::rgb(0.9, 0.9, 0.3),
                },
            ),
            ..default()
        });
    });

    println!("RBE Harvest Feedback UI spawned");
}

fn update_harvest_feedback_ui(
    mut query: Query<(&mut Text, &mut Visibility), With<HarvestFeedbackText>>,
    rbe_ui: Query<&RbeUiSync>,
) {
    let Ok((mut text, mut visibility)) = query.get_single_mut() else { return; };

    for ui_sync in rbe_ui.iter() {
        if let Some(feedback) = &ui_sync.last_harvest_feedback {
            text.sections[0].value = feedback.clone();
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}