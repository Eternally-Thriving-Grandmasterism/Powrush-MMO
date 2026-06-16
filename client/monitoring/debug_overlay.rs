// client/monitoring/debug_overlay.rs
// Unified Debug Overlay for Powrush-MMO (v18.37)
// Combines RBE Flow + Performance into one clean, toggleable panel (F2)
// Designed for clarity and the best development experience

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::monitoring::RBEFlowDashboard;

// Marker components
#[derive(Component)] struct DebugRbeAbundance;
#[derive(Component)] struct DebugRbeCreation;
#[derive(Component)] struct DebugRbeRestoration;
#[derive(Component)] struct DebugL2Boost;
#[derive(Component)] struct DebugL3Boost;
#[derive(Component)] struct DebugAlerts;

#[derive(Component)] struct DebugFps;
#[derive(Component)] struct DebugFrameTime;
#[derive(Component)] struct DebugEntities;

#[derive(Component)] struct DebugOverlayContainer;

#[derive(Resource, Default)]
pub struct DebugOverlayVisible(pub bool);

/// Spawns the unified Debug Overlay
pub fn spawn_debug_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DebugOverlayVisible(true));

    commands
        .spawn((
            Node {
                width: Val::Px(460.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(14.0)),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.04, 0.07, 0.95)),
            BorderColor(Color::srgb(0.4, 0.75, 0.95)),
            BorderRadius::all(Val::Px(10.0)),
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(70.0),
                right: Val::Px(25.0),
                ..default()
            },
            Visibility::Visible,
            DebugOverlayContainer,
            Name::new("Debug Overlay"),
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                Text::new("DEBUG OVERLAY (F2 to toggle)"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.9, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(6.0)),
                    ..default()
                },
            ));

            // ========== RBE FLOW SECTION ==========
            parent.spawn((
                Text::new("── RBE FLOW ──"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.85, 1.0)),
                Node {
                    margin: UiRect::top(Val::Px(4.0)).bottom(Val::Px(4.0)),
                    ..default()
                },
            ));

            parent.spawn((Text::new("Abundance: 0"), DebugRbeAbundance));
            parent.spawn((Text::new("Creation: +0.0 /s"), DebugRbeCreation));
            parent.spawn((Text::new("Restoration: +0.0 /s"), DebugRbeRestoration));

            parent.spawn((Text::new("L2: Inactive"), DebugL2Boost));
            parent.spawn((Text::new("L3: Inactive"), DebugL3Boost));
            parent.spawn((Text::new("Alerts: None"), DebugAlerts));

            // ========== PERFORMANCE SECTION ==========
            parent.spawn((
                Text::new("── PERFORMANCE ──"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.95, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(8.0)).bottom(Val::Px(4.0)),
                    ..default()
                },
            ));

            parent.spawn((Text::new("FPS: 0.0"), DebugFps));
            parent.spawn((Text::new("Frame Time: 0.00 ms"), DebugFrameTime));
            parent.spawn((Text::new("Entities: 0"), DebugEntities));
        });
}

/// Toggles the entire Debug Overlay with F2
pub fn toggle_debug_overlay(
    mut visibility: ResMut<DebugOverlayVisible>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<DebugOverlayContainer>>,
) {
    if keyboard.just_pressed(KeyCode::F2) {
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

/// Updates all debug information
pub fn update_debug_overlay(
    rbe_dashboard: Res<RBEFlowDashboard>,
    diagnostics: Res<DiagnosticsStore>,
    world: &World,
    mut abundance_q: Query<&mut Text, With<DebugRbeAbundance>>,
    mut creation_q: Query<&mut Text, With<DebugRbeCreation>>,
    mut restoration_q: Query<&mut Text, With<DebugRbeRestoration>>,
    mut l2_q: Query<&mut Text, With<DebugL2Boost>>,
    mut l3_q: Query<&mut Text, With<DebugL3Boost>>,
    mut alerts_q: Query<&mut Text, With<DebugAlerts>>,
    mut fps_q: Query<&mut Text, With<DebugFps>>,
    mut frame_time_q: Query<&mut Text, With<DebugFrameTime>>,
    mut entities_q: Query<&mut Text, With<DebugEntities>>,
) {
    // RBE Flow
    if let Ok(mut text) = abundance_q.get_single_mut() {
        text.0 = format!("Abundance: {:.0}", rbe_dashboard.server_abundance);
    }
    if let Ok(mut text) = creation_q.get_single_mut() {
        text.0 = format!("Creation: +{:.1} /s", rbe_dashboard.abundance_creation_rate);
    }
    if let Ok(mut text) = restoration_q.get_single_mut() {
        text.0 = format!("Restoration: +{:.1} /s", rbe_dashboard.abundance_restoration_rate);
    }
    if let Ok(mut text) = l2_q.get_single_mut() {
        text.0 = if rbe_dashboard.l2_boost_active {
            format!("L2: Active ×{:.2}", rbe_dashboard.l2_multiplier)
        } else {
            "L2: Inactive".to_string()
        };
    }
    if let Ok(mut text) = l3_q.get_single_mut() {
        text.0 = if rbe_dashboard.abundance_boost_active {
            format!("L3: Active ×{:.2}", rbe_dashboard.restoration_multiplier)
        } else {
            "L3: Inactive".to_string()
        };
    }
    if let Ok(mut text) = alerts_q.get_single_mut() {
        let count = rbe_dashboard.active_alerts.len();
        text.0 = if count > 0 {
            format!("Alerts: {} active", count)
        } else {
            "Alerts: None".to_string()
        };
    }

    // Performance
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.average()) {
        if let Ok(mut text) = fps_q.get_single_mut() {
            text.0 = format!("FPS: {:.1}", fps);
        }
    }

    if let Some(ft) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME).and_then(|d| d.average()) {
        if let Ok(mut text) = frame_time_q.get_single_mut() {
            text.0 = format!("Frame Time: {:.2} ms", ft);
        }
    }

    let entity_count = world.entities().len();
    if let Ok(mut text) = entities_q.get_single_mut() {
        text.0 = format!("Entities: {}", entity_count);
    }
}