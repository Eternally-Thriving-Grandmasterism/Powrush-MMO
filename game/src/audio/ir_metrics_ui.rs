/*!
 * Debug UI - Extended with Latency & Crossfade Metrics
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

use bevy::prelude::*;
use crate::audio::ir_metrics::IrTruncationMetrics;
use crate::audio::latency_metrics::AudioLatencyMetrics;

#[derive(Resource, Default)]
pub struct ShowIrMetrics {
    pub enabled: bool,
}

pub fn toggle_ir_metrics_ui(
    keyboard: Res<Input<KeyCode>>,
    mut show: ResMut<ShowIrMetrics>,
) {
    if keyboard.just_pressed(KeyCode::F9) {
        show.enabled = !show.enabled;
    }
}

pub fn ir_metrics_debug_ui(
    mut commands: Commands,
    show: Res<ShowIrMetrics>,
    ir_metrics: Res<IrTruncationMetrics>,
    latency_metrics: Res<AudioLatencyMetrics>,
    mut query: Query<(Entity, &mut Text), With<IrMetricsUiMarker>>,
) {
    if !show.enabled {
        for (entity, _) in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    let ir_snap = ir_metrics.snapshot();
    let lat_snap = latency_metrics.snapshot();

    let crossfade_str = match lat_snap.last_crossfade_duration_ms {
        Some(d) => format!("{d} ms"),
        None => "N/A".to_string(),
    };

    let text = format!(
        "IR Truncation\n\
        Loader: {}\n\
        Fallback: {}\n\
        PostProc: {}\n\
        Skipped: {}\n\
        \n\
        Latency\n\
        Avg: {:.1} ms\n\
        Max: {} ms\n\
        Samples: {}\n\
        Last XF: {}",
        ir_snap.truncations_in_loader,
        ir_snap.async_fallbacks,
        ir_snap.truncations_in_post_processor,
        ir_snap.truncations_skipped,
        lat_snap.average_latency_ms,
        lat_snap.max_latency_ms,
        lat_snap.latency_samples,
        crossfade_str
    );

    if let Ok((_, mut text_comp)) = query.get_single_mut() {
        text_comp.sections[0].value = text;
    } else {
        commands.spawn((
            TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font_size: 16.0,
                        color: Color::srgb(0.85, 0.9, 1.0),
                        ..default()
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    right: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.08, 0.12, 0.9).into(),
                ..default()
            },
            IrMetricsUiMarker,
        ));
    }
}

#[derive(Component)]
struct IrMetricsUiMarker;
