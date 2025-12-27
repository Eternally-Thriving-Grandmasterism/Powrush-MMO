use bevy::prelude::*;

#[derive(Component)]
pub struct MercyTooltip {
    message: String,
    suggestion: String,
}

pub struct UITooltipPlugin;

impl Plugin for UITooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tooltip_spawn_system)
           .add_systems(Update, tooltip_animation_system);
    }
}

fn tooltip_spawn_system(
    mut commands: Commands,
    errors: Query<&LatticeError, Added<LatticeError>>,
) {
    for error in &errors {
        commands.spawn((
            MercyTooltip {
                message: error.message.clone(),
                suggestion: "Share mercy — lattice grows".to_string(),
            },
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(200.0),
                    top: Val::Px(100.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.2, 0.9)),
                ..default()
            },
            TextBundle::from_sections([
                TextSection::new(&error.message, TextStyle { font_size: 32.0, color: Color::WHITE, ..default() }),
                TextSection::new("\nMercy suggestion: Share resources", TextStyle { font_size: 24.0, color: Color::CYAN, ..default() }),
            ]),
        ));
    }
}

fn tooltip_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut tooltips: Query<(Entity, &mut BackgroundColor, &mut Text)>,
) {
    for (entity, mut bg, mut text) in tooltips.iter_mut() {
        let pulse = (time.elapsed_seconds() * 2.0).sin() * 0.1 + 0.8;
        bg.0 = Color::rgba(0.1, 0.1, 0.2, pulse);
        text.sections[1].style.color = Color::CYAN.with_a(pulse);
    }
}
