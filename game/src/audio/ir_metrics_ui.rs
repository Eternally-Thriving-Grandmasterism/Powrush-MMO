/*!
 * Debug UI for IR Truncation Metrics
 *
 * Simple on-screen debug panel using Bevy UI.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::audio::ir_metrics::IrTruncationMetrics;

#[derive(Resource, Default)]
pub struct ShowIrMetrics {
    pub enabled: bool,
}

/// Toggles the IR metrics debug UI when F9 is pressed
pub fn toggle_ir_metrics_ui(
    keyboard: Res<Input<KeyCode>>,
    mut show: ResMut<ShowIrMetrics>,
) {
    if keyboard.just_pressed(KeyCode::F9) {
        show.enabled = !show.enabled;
    }
}

/// Renders a simple debug panel showing IR truncation metrics
pub fn ir_metrics_debug_ui(
    mut commands: Commands,
    show: Res<ShowIrMetrics>,
    metrics: Res<IrTruncationMetrics>,
    mut query: Query<(Entity, &mut Text), With<IrMetricsUiMarker>>,
) {
    if !show.enabled {
        // Remove existing UI if disabled
        for (entity, _) in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    let snapshot = metrics.snapshot();

    let text = format!(
        "IR Truncation Metrics\n\
        Loader:      {}\n\
        Fallbacks:   {}\n\
        Post-Proc:   {}\n\
        Skipped:     {}\n\
        Total:       {}",
        snapshot.truncations_in_loader,
        snapshot.async_fallbacks,
        snapshot.truncations_in_post_processor,
        snapshot.truncations_skipped,
        snapshot.total_truncation_attempts
    );

    if let Ok((_, mut text_comp)) = query.get_single_mut() {
        text_comp.sections[0].value = text;
    } else {
        // Spawn the debug UI
        commands.spawn((
            TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.9, 0.95, 1.0),
                        ..default()
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    right: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.15, 0.85).into(),
                ..default()
            },
            IrMetricsUiMarker,
        ));
    }
}

#[derive(Component)]
struct IrMetricsUiMarker;
