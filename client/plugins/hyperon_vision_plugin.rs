//! Hyperon Vision Rendering Plugin — Powrush Client
//! Renders symbolic cosmic visions from Ra-Thor / Hyperon bridge
//! Mercy-gated, non-blocking, valence-tinted display
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

use bevy::prelude::*;
use bevy::text::{Text, TextSection, TextStyle};
use bevy::window::PrimaryWindow;
use std::time::Duration;

#[derive(Component)]
struct VisionOverlay;

#[derive(Component)]
struct VisionGlyph;

#[derive(Component)]
struct VisionText;

#[derive(Component)]
struct VisionTimer(Timer);

#[derive(Resource)]
struct VisionState {
    active_vision: Option<HyperonVisionData>,
}

#[derive(Clone)]
struct HyperonVisionData {
    seed: String,
    narrative: String,
    valence: f32,
    path: Vec<String>, // symbolic atom path
}

pub struct HyperonVisionPlugin;

impl Plugin for HyperonVisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<VisionState>()
            .add_systems(Startup, setup_vision_overlay)
            .add_systems(Update, (
                handle_vision_events,
                update_vision_display,
                dismiss_vision_on_input,
            ));
    }
}

fn setup_vision_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Full-screen overlay root (invisible by default)
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            visibility: Visibility::Hidden,
            ..default()
        },
        VisionOverlay,
    ))
    .with_children(|parent| {
        // Narrative text crawl
        parent.spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/mercy.ttf"), // placeholder — replace with canonical font
                            font_size: 32.0,
                            color: Color::srgba(0.9, 0.95, 1.0, 0.9),
                        },
                    }],
                    linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
                    ..default()
                },
                style: Style {
                    width: Val::Percent(80.0),
                    max_width: Val::Px(1200.0),
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
                ..default()
            },
            VisionText,
        ));

        // Glyph particle system placeholder (expand later with bevy_hanabi or custom)
        parent.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(1.0, 0.9, 0.6, 0.0),
                    custom_size: Some(Vec2::new(200.0, 200.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::ZERO),
                visibility: Visibility::Hidden,
                ..default()
            },
            VisionGlyph,
        ));
    });
}

fn handle_vision_events(
    mut vision_events: EventReader<HyperonVisionEvent>,
    mut vision_state: ResMut<VisionState>,
    mut overlay_query: Query<&mut Visibility, With<VisionOverlay>>,
    mut text_query: Query<&mut Text, With<VisionText>>,
    mut glyph_query: Query<&mut Sprite, With<VisionGlyph>>,
) {
    for event in vision_events.read() {
        let vision = event.vision.clone();

        // Activate overlay
        if let Ok(mut vis) = overlay_query.get_single_mut() {
            *vis = Visibility::Visible;
        }

        // Update narrative text
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = vision.narrative.clone();
            text.sections[0].style.color = Color::srgba(0.9, 0.95, 1.0, 0.9 + (vision.valence * 0.1));
        }

        // Glyph color pulse based on valence
        if let Ok(mut sprite) = glyph_query.get_single_mut() {
            sprite.color = Color::srgba(
                1.0,
                0.9 + (vision.valence * 0.1),
                0.6 + (vision.valence * 0.4),
                0.6 + (vision.valence * 0.4),
            );
        }

        vision_state.active_vision = Some(vision);

        // Auto-dismiss after 12 seconds or on input
        // (handled in separate system)
    }
}

fn update_vision_display(
    time: Res<Time>,
    mut query: Query<(&mut VisionTimer, &mut Visibility), With<VisionOverlay>>,
    vision_state: Res<VisionState>,
) {
    if let Ok((mut timer, mut vis)) = query.get_single_mut() {
        if vision_state.active_vision.is_some() {
            if timer.0.finished() {
                *vis = Visibility::Hidden;
            } else {
                timer.0.tick(time.delta());
            }
        }
    }
}

fn dismiss_vision_on_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut vision_state: ResMut<VisionState>,
    mut overlay_query: Query<&mut Visibility, With<VisionOverlay>>,
) {
    if vision_state.active_vision.is_none() {
        return;
    }

    if keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
        || mouse.just_pressed(MouseButton::Right)
    {
        vision_state.active_vision = None;
        if let Ok(mut vis) = overlay_query.get_single_mut() {
            *vis = Visibility::Hidden;
        }
    }
}

// Event bridge from HyperonVisionIntegration (JS → Rust via WASM or native bridge)
#[derive(Event)]
pub struct HyperonVisionEvent {
    pub vision: HyperonVisionData,
}

// Placeholder data struct (expand with real deserialization)
#[derive(Clone)]
pub struct HyperonVisionData {
    pub seed: String,
    pub narrative: String,
    pub valence: f32,
    pub path: Vec<String>,
}
