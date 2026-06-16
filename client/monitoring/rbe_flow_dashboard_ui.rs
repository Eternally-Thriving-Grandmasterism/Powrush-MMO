// client/monitoring/rbe_flow_dashboard_ui.rs
// In-Game RBE Flow Dashboard UI (v18.37)
// Designed for clarity, transparency, and the best player experience

use bevy::prelude::*;
use crate::monitoring::RBEFlowDashboard;

/// Spawns the RBE Flow Dashboard UI
pub fn spawn_rbe_flow_dashboard_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Px(420.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.92)),
            BorderColor(Color::srgb(0.3, 0.6, 0.9)),
            BorderRadius::all(Val::Px(8.0)),
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                right: Val::Px(20.0),
                ..default()
            },
            Name::new("RBE Flow Dashboard"),
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                Text::new("RBE FLOW DASHBOARD"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.85, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
            ));

            // Metrics Section
            parent.spawn(Text::new("Current Abundance: 124,850"));
            parent.spawn(Text::new("Creation Rate: +42.3 / sec"));
            parent.spawn(Text::new("Restoration Rate: +18.7 / sec"));

            // Active Boosts Section
            parent.spawn((
                Text::new("Active Boosts"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.6)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)).bottom(Val::Px(4.0)),
                    ..default()
                },
            ));
            parent.spawn(Text::new("• L2 Supportive Boost ×1.25"));
            parent.spawn(Text::new("• L3 Recovery Boost ×1.50"));

            // Recent Alerts Section
            parent.spawn((
                Text::new("Recent Alerts"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.7, 0.5)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)).bottom(Val::Px(4.0)),
                    ..default()
                },
            ));
            parent.spawn(Text::new("• [L2] High Safety Net Trigger Frequency"));
            parent.spawn(Text::new("• [L3] Persistent Scarcity Signal"));
        });
}

/// System that updates the RBE Flow Dashboard from the resource
pub fn update_rbe_flow_dashboard(
    dashboard: Res<RBEFlowDashboard>,
    mut query: Query<&mut Text, With<Name>>,
) {
    // TODO: Dynamically update text values from dashboard resource
    // For now this is a static foundation. We will wire live data next.
}