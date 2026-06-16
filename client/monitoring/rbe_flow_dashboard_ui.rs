// client/monitoring/rbe_flow_dashboard_ui.rs
// In-Game RBE Flow Dashboard UI with Toggle (v18.37)
// Designed for clarity, transparency, and the best player experience

use bevy::prelude::*;
use crate::monitoring::RBEFlowDashboard;

// Marker components
#[derive(Component)]
struct AbundanceText;

#[derive(Component)]
struct CreationRateText;

#[derive(Component)]
struct RestorationRateText;

#[derive(Component)]
struct L2BoostText;

#[derive(Component)]
struct L3BoostText;

#[derive(Component)]
struct AlertsText;

#[derive(Component)]
struct RbeDashboardContainer;

/// Resource to control dashboard visibility
#[derive(Resource, Default)]
pub struct RbeDashboardVisible(pub bool);

/// Spawns the RBE Flow Dashboard UI
pub fn spawn_rbe_flow_dashboard_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RbeDashboardVisible(true));

    commands
        .spawn((
            Node {
                width: Val::Px(420.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(14.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.06, 0.10, 0.94)),
            BorderColor(Color::srgb(0.35, 0.65, 0.95)),
            BorderRadius::all(Val::Px(10.0)),
            Style {
                position_type: PositionType::Absolute,
                top: Val::Px(70.0),
                right: Val::Px(25.0),
                ..default()
            },
            Visibility::Visible,
            RbeDashboardContainer,
            Name::new("RBE Flow Dashboard Container"),
        ))
        .with_children(|parent| {
            // Header
            parent.spawn((
                Text::new("RBE FLOW DASHBOARD"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 17.0,
                    ..default()
                },
                TextColor(Color::srgb(0.65, 0.88, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(6.0)),
                    ..default()
                },
            ));

            // Metrics
            parent.spawn((Text::new("Abundance: 0"), AbundanceText));
            parent.spawn((Text::new("Creation: +0.0 /s"), CreationRateText));
            parent.spawn((Text::new("Restoration: +0.0 /s"), RestorationRateText));

            // Active Boosts
            parent.spawn((
                Text::new("Active Boosts"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.9, 0.55)),
                Node {
                    margin: UiRect::top(Val::Px(8.0)).bottom(Val::Px(2.0)),
                    ..default()
                },
            ));
            parent.spawn((Text::new("L2: Inactive"), L2BoostText));
            parent.spawn((Text::new("L3: Inactive"), L3BoostText));

            // Recent Alerts
            parent.spawn((
                Text::new("Recent Alerts"),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.75, 0.55)),
                Node {
                    margin: UiRect::top(Val::Px(8.0)).bottom(Val::Px(2.0)),
                    ..default()
                },
            ));
            parent.spawn((Text::new("No active alerts"), AlertsText));
        });
}

/// Toggles the RBE Flow Dashboard visibility when F3 is pressed
pub fn toggle_rbe_flow_dashboard(
    mut visibility: ResMut<RbeDashboardVisible>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<RbeDashboardContainer>>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
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

/// Updates the dashboard content from the resource
pub fn update_rbe_flow_dashboard(
    dashboard: Res<RBEFlowDashboard>,
    mut abundance_q: Query<&mut Text, With<AbundanceText>>,
    mut creation_q: Query<&mut Text, With<CreationRateText>>,
    mut restoration_q: Query<&mut Text, With<RestorationRateText>>,
    mut l2_q: Query<&mut Text, With<L2BoostText>>,
    mut l3_q: Query<&mut Text, With<L3BoostText>>,
    mut alerts_q: Query<&mut Text, With<AlertsText>>,
) {
    if let Ok(mut text) = abundance_q.get_single_mut() {
        text.0 = format!("Abundance: {:.0}", dashboard.server_abundance);
    }

    if let Ok(mut text) = creation_q.get_single_mut() {
        text.0 = format!("Creation: +{:.1} /s", dashboard.abundance_creation_rate);
    }

    if let Ok(mut text) = restoration_q.get_single_mut() {
        text.0 = format!("Restoration: +{:.1} /s", dashboard.abundance_restoration_rate);
    }

    if let Ok(mut text) = l2_q.get_single_mut() {
        if dashboard.l2_boost_active {
            text.0 = format!("L2: Active ×{:.2}", dashboard.l2_multiplier);
        } else {
            text.0 = "L2: Inactive".to_string();
        }
    }

    if let Ok(mut text) = l3_q.get_single_mut() {
        if dashboard.abundance_boost_active {
            text.0 = format!("L3: Active ×{:.2}", dashboard.restoration_multiplier);
        } else {
            text.0 = "L3: Inactive".to_string();
        }
    }

    if let Ok(mut text) = alerts_q.get_single_mut() {
        let total_active = dashboard.active_alerts.len();
        let l2_count = dashboard.l2_alerts.len();

        if total_active > 0 {
            text.0 = format!("Active Alerts: {} (L2: {})", total_active, l2_count);
        } else {
            text.0 = "No active alerts".to_string();
        }
    }
}