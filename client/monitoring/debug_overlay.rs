// client/monitoring/debug_overlay.rs
// Unified Debug Overlay for Powrush-MMO (v18.37)
// Includes FPS + Frame Time Graphs

use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::monitoring::RBEFlowDashboard;

// RBE Flow markers
#[derive(Component)] struct DebugRbeAbundance;
#[derive(Component)] struct DebugRbeCreation;
#[derive(Component)] struct DebugRbeRestoration;
#[derive(Component)] struct DebugL2Boost;
#[derive(Component)] struct DebugL3Boost;
#[derive(Component)] struct DebugAlerts;

// Performance markers
#[derive(Component)] struct DebugFps;
#[derive(Component)] struct DebugFrameTime;
#[derive(Component)] struct DebugEntities;

// Graph markers
#[derive(Component)] struct FpsGraphContainer;
#[derive(Component)] struct FrameTimeGraphContainer;
#[derive(Component)] struct FpsBar { index: usize }
#[derive(Component)] struct FrameTimeBar { index: usize }

#[derive(Component)] struct DebugOverlayContainer;

#[derive(Resource, Default)]
pub struct DebugOverlayVisible(pub bool);

#[derive(Resource)]
pub struct FpsHistory {
    pub values: Vec<f32>,
    pub max_samples: usize,
}

impl Default for FpsHistory {
    fn default() -> Self {
        Self { values: Vec::with_capacity(90), max_samples: 90 }
    }
}

impl FpsHistory {
    pub fn push(&mut self, fps: f32) {
        if self.values.len() >= self.max_samples { self.values.remove(0); }
        self.values.push(fps);
    }
    pub fn max_fps(&self) -> f32 { self.values.iter().copied().fold(0.0, f32::max) }
}

#[derive(Resource)]
pub struct FrameTimeHistory {
    pub values: Vec<f32>,
    pub max_samples: usize,
}

impl Default for FrameTimeHistory {
    fn default() -> Self {
        Self { values: Vec::with_capacity(90), max_samples: 90 }
    }
}

impl FrameTimeHistory {
    pub fn push(&mut self, frame_time_ms: f32) {
        if self.values.len() >= self.max_samples { self.values.remove(0); }
        self.values.push(frame_time_ms);
    }
    pub fn max_frame_time(&self) -> f32 { self.values.iter().copied().fold(0.0, f32::max) }
}

pub fn spawn_debug_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DebugOverlayVisible(true));
    commands.insert_resource(FpsHistory::default());
    commands.insert_resource(FrameTimeHistory::default());

    commands
        .spawn((
            Node {
                width: Val::Px(480.0),
                height: Val::Auto,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(14.0)),
                row_gap: Val::Px(6.0),
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
            parent.spawn((Text::new("DEBUG (F2)"), TextFont { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 15.0, ..default() }, TextColor(Color::srgb(0.6, 0.9, 1.0))));

            // RBE FLOW
            parent.spawn((Text::new("RBE FLOW"), TextFont { font_size: 12.0, ..default() }, TextColor(Color::srgb(0.7, 0.85, 1.0))));
            parent.spawn((Text::new("Abundance: 0"), DebugRbeAbundance));
            parent.spawn((Text::new("Creation: +0.0 /s"), DebugRbeCreation));
            parent.spawn((Text::new("Restoration: +0.0 /s"), DebugRbeRestoration));
            parent.spawn((Text::new("L2: Inactive"), DebugL2Boost));
            parent.spawn((Text::new("L3: Inactive"), DebugL3Boost));
            parent.spawn((Text::new("Alerts: None"), DebugAlerts));

            // PERFORMANCE
            parent.spawn((Text::new("PERFORMANCE"), TextFont { font_size: 12.0, ..default() }, TextColor(Color::srgb(0.6, 0.95, 0.7))));
            parent.spawn((Text::new("FPS: 0.0"), DebugFps));
            parent.spawn((Text::new("Frame Time: 0.00 ms"), DebugFrameTime));
            parent.spawn((Text::new("Entities: 0"), DebugEntities));

            // FPS Graph
            parent.spawn((Text::new("FPS History"), TextFont { font_size: 11.0, ..default() }, TextColor(Color::srgb(0.8, 0.9, 0.8))));
            parent.spawn((Node { width: Val::Percent(100.0), height: Val::Px(50.0), flex_direction: FlexDirection::Row, align_items: AlignItems::FlexEnd, column_gap: Val::Px(1.0), ..default() }, BackgroundColor(Color::srgba(0.1, 0.1, 0.12, 0.8)), FpsGraphContainer))
                .with_children(|g| { for i in 0..90 { g.spawn((Node { width: Val::Px(2.0), height: Val::Px(4.0), ..default() }, BackgroundColor(Color::srgb(0.3, 0.8, 0.4)), FpsBar { index: i })); } });

            // Frame Time Graph
            parent.spawn((Text::new("Frame Time History (ms)"), TextFont { font_size: 11.0, ..default() }, TextColor(Color::srgb(0.8, 0.9, 0.8))));
            parent.spawn((Node { width: Val::Percent(100.0), height: Val::Px(50.0), flex_direction: FlexDirection::Row, align_items: AlignItems::FlexEnd, column_gap: Val::Px(1.0), ..default() }, BackgroundColor(Color::srgba(0.1, 0.1, 0.12, 0.8)), FrameTimeGraphContainer))
                .with_children(|g| { for i in 0..90 { g.spawn((Node { width: Val::Px(2.0), height: Val::Px(4.0), ..default() }, BackgroundColor(Color::srgb(0.85, 0.5, 0.3)), FrameTimeBar { index: i })); } });
        });
}

pub fn toggle_debug_overlay(
    mut visibility: ResMut<DebugOverlayVisible>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<DebugOverlayContainer>>,
) {
    if keyboard.just_pressed(KeyCode::F2) {
        visibility.0 = !visibility.0;
        for mut vis in &mut query { *vis = if visibility.0 { Visibility::Visible } else { Visibility::Hidden }; }
    }
}

pub fn update_debug_overlay(
    rbe_dashboard: Res<RBEFlowDashboard>,
    diagnostics: Res<DiagnosticsStore>,
    world: &World,
    mut fps_history: ResMut<FpsHistory>,
    mut frame_time_history: ResMut<FrameTimeHistory>,
    mut abundance_q: Query<&mut Text, With<DebugRbeAbundance>>,
    mut creation_q: Query<&mut Text, With<DebugRbeCreation>>,
    mut restoration_q: Query<&mut Text, With<DebugRbeRestoration>>,
    mut l2_q: Query<&mut Text, With<DebugL2Boost>>,
    mut l3_q: Query<&mut Text, With<DebugL3Boost>>,
    mut alerts_q: Query<&mut Text, With<DebugAlerts>>,
    mut fps_q: Query<&mut Text, With<DebugFps>>,
    mut frame_time_q: Query<&mut Text, With<DebugFrameTime>>,
    mut entities_q: Query<&mut Text, With<DebugEntities>>,
    mut fps_bars: Query<(&mut Node, &FpsBar, &mut BackgroundColor)>,
    mut ft_bars: Query<(&mut Node, &FrameTimeBar, &mut BackgroundColor)>,
) {
    // RBE Flow
    if let Ok(mut text) = abundance_q.get_single_mut() { text.0 = format!("Abundance: {:.0}", rbe_dashboard.server_abundance); }
    if let Ok(mut text) = creation_q.get_single_mut() { text.0 = format!("Creation: +{:.1} /s", rbe_dashboard.abundance_creation_rate); }
    if let Ok(mut text) = restoration_q.get_single_mut() { text.0 = format!("Restoration: +{:.1} /s", rbe_dashboard.abundance_restoration_rate); }
    if let Ok(mut text) = l2_q.get_single_mut() { text.0 = if rbe_dashboard.l2_boost_active { format!("L2: Active ×{:.2}", rbe_dashboard.l2_multiplier) } else { "L2: Inactive".to_string() }; }
    if let Ok(mut text) = l3_q.get_single_mut() { text.0 = if rbe_dashboard.abundance_boost_active { format!("L3: Active ×{:.2}", rbe_dashboard.restoration_multiplier) } else { "L3: Inactive".to_string() }; }
    if let Ok(mut text) = alerts_q.get_single_mut() { text.0 = if rbe_dashboard.active_alerts.len() > 0 { format!("Alerts: {} active", rbe_dashboard.active_alerts.len()) } else { "Alerts: None".to_string() }; }

    // Performance metrics
    let current_fps = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|d| d.average()).unwrap_or(0.0);
    if let Ok(mut text) = fps_q.get_single_mut() { text.0 = format!("FPS: {:.1}", current_fps); }

    let current_frame_time = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME).and_then(|d| d.average()).unwrap_or(0.0);
    if let Ok(mut text) = frame_time_q.get_single_mut() { text.0 = format!("Frame Time: {:.2} ms", current_frame_time); }

    if let Ok(mut text) = entities_q.get_single_mut() { text.0 = format!("Entities: {}", world.entities().len()); }

    // Update histories
    fps_history.push(current_fps as f32);
    frame_time_history.push(current_frame_time as f32);

    // FPS Graph
    let max_fps = fps_history.max_fps().max(30.0);
    for (mut node, bar, mut color) in &mut fps_bars {
        if let Some(&val) = fps_history.values.get(bar.index) {
            let norm = (val / max_fps).clamp(0.0, 1.0);
            node.height = Val::Px(4.0 + norm * 46.0);
            color.0 = if val < 30.0 { Color::srgb(0.9, 0.3, 0.3) } else if val < 50.0 { Color::srgb(0.95, 0.85, 0.3) } else { Color::srgb(0.3, 0.85, 0.4) };
        }
    }

    // Frame Time Graph (lower is better)
    let max_ft = frame_time_history.max_frame_time().max(16.0);
    for (mut node, bar, mut color) in &mut ft_bars {
        if let Some(&val) = frame_time_history.values.get(bar.index) {
            let norm = (val / max_ft).clamp(0.0, 1.0);
            node.height = Val::Px(4.0 + norm * 46.0);
            color.0 = if val > 25.0 { Color::srgb(0.9, 0.3, 0.3) } else if val > 16.7 { Color::srgb(0.95, 0.85, 0.3) } else { Color::srgb(0.3, 0.85, 0.4) };
        }
    }
}