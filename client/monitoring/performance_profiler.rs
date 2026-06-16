// client/monitoring/performance_profiler.rs
// Performance Profiling Tools for Powrush-MMO (v18.37)
// Toggle with F4 - Clean, readable overlay for the best development & player experience

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct FrameTimeText;

#[derive(Component)]
struct EntityCountText;

#[derive(Component)]
struct PerformanceContainer;

#[derive(Resource, Default)]
pub struct PerformanceOverlayVisible(pub bool);

/// Spawns the Performance Profiling Overlay
pub fn spawn_performance_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PerformanceOverlayVisible(true));

    commands
        .spawn((
            Node {
                width: Val::Px(280.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 0.9)),
            BorderColor(Color::srgb(0.4, 0.7, 0.5)),
            BorderRadius::all(Val::Px(8.0)),
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(70.0),
                left: Val::Px(25.0),
                ..default()
            },
            Visibility::Visible,
            PerformanceContainer,
            Name::new("Performance Overlay"),
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                Text::new("PERFORMANCE"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.95, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(6.0)),
                    ..default()
                },
            ));

            parent.spawn((Text::new("FPS: 0"), FpsText));
            parent.spawn((Text::new("Frame Time: 0.00 ms"), FrameTimeText));
            parent.spawn((Text::new("Entities: 0"), EntityCountText));
        });
}

/// Toggles the performance overlay with F4
pub fn toggle_performance_overlay(
    mut visibility: ResMut<PerformanceOverlayVisible>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<PerformanceContainer>>,
) {
    if keyboard.just_pressed(KeyCode::F4) {
        visibility.0 = !visibility.0;

        for mut vis in &mut query {
            *vis = if visibility.0 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

/// Updates performance metrics from Bevy diagnostics
pub fn update_performance_overlay(
    diagnostics: Res<DiagnosticsStore>,
    world: &World, // to count entities
    mut fps_q: Query<&mut Text, With<FpsText>>,
    mut frame_time_q: Query<&mut Text, With<FrameTimeText>>,
    mut entity_q: Query<&mut Text, With<EntityCountText>>,
) {
    // FPS
    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.average())
    {
        if let Ok(mut text) = fps_q.get_single_mut() {
            text.0 = format!("FPS: {:.1}", fps);
        }
    }

    // Frame Time
    if let Some(frame_time) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|d| d.average())
    {
        if let Ok(mut text) = frame_time_q.get_single_mut() {
            text.0 = format!("Frame Time: {:.2} ms", frame_time);
        }
    }

    // Entity Count (simple but useful)
    let entity_count = world.entities().len();
    if let Ok(mut text) = entity_q.get_single_mut() {
        text.0 = format!("Entities: {}", entity_count);
    }
}