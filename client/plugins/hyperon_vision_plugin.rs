//! Hyperon Vision Rendering Plugin v1.1 — with Hanabi Glyph Particles
//! Mercy-gated cosmic display: glyphs + particle bursts + narrative
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use std::time::Duration;

#[derive(Component)]
struct VisionOverlay;

#[derive(Component)]
struct VisionGlyphParticle;

#[derive(Component)]
struct VisionText;

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

#[derive(Event)]
pub struct HyperonVisionEvent {
    pub vision: HyperonVisionData,
}

pub struct HyperonVisionPlugin;

impl Plugin for HyperonVisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<VisionState>()
            .add_plugins(HanabiPlugin)
            .add_event::<HyperonVisionEvent>()
            .add_systems(Startup, (setup_vision_overlay, setup_glyph_particle_effect))
            .add_systems(Update, (
                handle_vision_events,
                update_vision_display,
                dismiss_vision_on_input,
                animate_glyph_particles,
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
            background_color: Color::srgba(0.02, 0.05, 0.12, 0.0).into(), // subtle dark base
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
                            font: asset_server.load("fonts/mercy.ttf"), // replace with actual font
                            font_size: 36.0,
                            color: Color::srgba(0.9, 0.95, 1.0, 0.0),
                        },
                    }],
                    linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
                    ..default()
                },
                style: Style {
                    width: Val::Percent(80.0),
                    max_width: Val::Px(1400.0),
                    margin: UiRect::all(Val::Px(60.0)),
                    ..default()
                },
                ..default()
            },
            VisionText,
        ));
    });
}

fn setup_glyph_particle_effect(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Hanabi effect: golden sparks + soft trails + aurora wisps
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.0, 0.9, 0.6, 0.0));
    color_gradient.add_key(0.3, Vec4::new(1.0, 0.95, 0.7, 0.8));
    color_gradient.add_key(0.7, Vec4::new(0.8, 0.9, 1.0, 0.6));
    color_gradient.add_key(1.0, Vec4::new(0.6, 0.8, 1.0, 0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, 0.0);
    size_gradient.add_key(0.3, 12.0);
    size_gradient.add_key(0.7, 8.0);
    size_gradient.add_key(1.0, 0.0);

    let effect = EffectAsset::new(1024)
        .init(InitPositionCircleModifier {
            center: Vec3::ZERO,
            radius: 20.0,
            dimension: ShapeDimension::Surface,
        })
        .init(InitVelocityCircleModifier {
            center: Vec3::ZERO,
            radius: 60.0,
            dimension: ShapeDimension::Volume,
        })
        .init(InitLifetimeModifier { lifetime: Value::Uniform((1.2, 2.8)) })
        .render(ColorOverLifetimeModifier { gradient: color_gradient })
        .render(SizeOverLifetimeModifier { gradient: size_gradient })
        .render(ParticleTextureModifier { texture: None }); // add glyph texture later

    let effect_handle = effects.add(effect);

    // Spawn invisible emitter (activated on vision)
    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        VisionGlyphParticle,
        Visibility::Hidden,
    ));
}

fn handle_vision_events(
    mut vision_events: EventReader<HyperonVisionEvent>,
    mut vision_state: ResMut<VisionState>,
    mut overlay_query: Query<&mut Visibility, With<VisionOverlay>>,
    mut glyph_query: Query<(&mut Visibility, &mut ParticleEffect), With<VisionGlyphParticle>>,
    mut text_query: Query<&mut Text, With<VisionText>>,
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
            text.sections[0].style.color = Color::srgba(0.9, 0.95, 1.0, 0.7 + (vision.valence * 0.3));
        }

        // Activate & tune particle system
        if let Ok((mut vis, mut effect)) = glyph_query.get_single_mut() {
            *vis = Visibility::Visible;

            // Valence-based tuning (intensity, color, speed)
            if let Some(effect) = effect.effect_mut() {
                effect.set_simulation_speed(0.8 + vision.valence * 0.4);
                // More advanced tuning possible via modifiers
            }
        }

        vision_state.active_vision = Some(vision);
    }
}

fn animate_glyph_particles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Visibility), With<VisionGlyphParticle>>,
    vision_state: Res<VisionState>,
) {
    if let Ok((mut transform, mut vis)) = query.get_single_mut() {
        if vision_state.active_vision.is_none() {
            *vis = Visibility::Hidden;
            return;
        }

        // Slow rotation + subtle pulse
        transform.rotate_local_y(time.delta_seconds() * 0.15);
        transform.scale = Vec3::splat(1.0 + (time.elapsed_seconds() * 2.0).sin() * 0.1);
    }
}

fn dismiss_vision_on_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut vision_state: ResMut<VisionState>,
    mut overlay_query: Query<&mut Visibility, With<VisionOverlay>>,
    mut glyph_query: Query<&mut Visibility, With<VisionGlyphParticle>>,
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
        if let Ok(mut g_vis) = glyph_query.get_single_mut() {
            *g_vis = Visibility::Hidden;
        }
    }
}
