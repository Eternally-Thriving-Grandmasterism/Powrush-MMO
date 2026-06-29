/*!
 * Council UI - Full Real Distance-Based 3D Audio Falloff (v19.2.9)
 * 
 * One label (Mercy Resonance) fully migrated to TextAtlasCache cached blitting.
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_kira_audio::prelude::*;
use image::RgbImage;
use simulation::game_state::GameState;
use simulation::council_mercy_trial::{CouncilAttunementAction, CouncilUIHooksPlugin};
use simulation::council_systems::{RecentMercyResonance, LastCouncilValence, CouncilResolved};

use crate::engine::ui::{TextAtlasCache, SimpleBitmapFont, draw_pre_rendered_text};

#[derive(Component)]
pub struct CouncilPanel;

#[derive(Component)]
struct CouncilAttunementButton {
    attunement_delta: f32,
}

#[derive(Component)]
struct MercyResonanceText;

#[derive(Component)]
struct LastValenceText;

#[derive(Component)]
struct ValenceParticles;

#[derive(Component)]
struct ValenceBurst;

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

/// Holds the cached image handle for a label
#[derive(Component)]
struct CachedLabelImage(pub Handle<Image>);

pub struct CouncilUIPlugin;

impl Plugin for CouncilUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CouncilUIHooksPlugin)
            .add_plugins(AudioPlugin)
            .init_resource::<LocalPlayer>()
            .insert_resource(TextAtlasCache::with_pixel_weigher(512))
            .add_systems(Startup, setup_audio_listener)
            .add_systems(OnEnter(GameState::InCouncil), spawn_council_panel)
            .add_systems(OnExit(GameState::InCouncil), (despawn_council_panel, cleanup_valence_particles))
            .add_systems(
                Update,
                (
                    handle_council_buttons,
                    handle_council_resolved_bursts,
                    handle_council_toggle_input,
                    update_resonance_display, // Still used for Last Valence (not yet migrated)
                    update_panel_visuals,
                    update_valence_particles,
                    update_council_text_cache,
                    update_mercy_resonance_image, // NEW: Cached blitting for Mercy Resonance
                )
                .run_if(in_state(GameState::InCouncil)),
            );
    }
}

fn setup_audio_listener(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera>>,
) {
    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).insert((AudioListener, Velocity::default()));
    }
}

fn spawn_council_panel(mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>) {
    // Create initial placeholder image for Mercy Resonance
    let placeholder = Image::from_dynamic(
        image::DynamicImage::ImageRgb8(RgbImage::new(200, 20)),
        true,
    );
    let resonance_handle = images.add(placeholder);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(15.0),
                    left: Val::Percent(2.0),
                    width: Val::Px(340.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                background_color: Color::srgba(0.08, 0.06, 0.12, 0.96).into(),
                border_color: Color::srgb(0.6, 0.5, 0.9).into(),
                ..default()
            },
            CouncilPanel,
            StateScoped(GameState::InCouncil),
            Name::new("CouncilPanel"),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle { text: Text::from_section("COUNCIL OF MERCY", TextStyle { font: asset_server.load("fonts/FiraSans-Bold.ttf"), font_size: 18.0, color: Color::srgb(0.85, 0.75, 1.0) }), style: Style { margin: UiRect::bottom(Val::Px(8.0)), ..default() }, ..default() });

            // MIGRATED: Mercy Resonance now uses cached blitting via UiImage
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(resonance_handle.clone()),
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        margin: UiRect::bottom(Val::Px(4.0)),
                        ..default()
                    },
                    ..default()
                },
                MercyResonanceText,
                CachedLabelImage(resonance_handle),
            ));

            parent.spawn((TextBundle { text: Text::from_section("Last Valence: --", TextStyle { font_size: 13.0, color: Color::srgb(0.9, 0.85, 0.6) }), style: Style { margin: UiRect::bottom(Val::Px(10.0)), ..default() }, ..default() }, LastValenceText));

            create_attunement_button(parent, &asset_server, "Focus Deeply", 0.25);
            create_attunement_button(parent, &asset_server, "Vote with Conviction", 0.45);
            create_attunement_button(parent, &asset_server, "Meditate in Harmony", 0.35);
            create_attunement_button(parent, &asset_server, "Offer Grace", 0.55);

            parent.spawn(TextBundle { text: Text::from_section("F2 to leave  •  Your attunement shapes the RBE", TextStyle { font_size: 11.0, color: Color::srgb(0.7, 0.65, 0.85) }), style: Style { margin: UiRect::top(Val::Px(12.0)), ..default() }, ..default() });
        });
}

fn create_attunement_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, label: &str, delta: f32) {
    parent.spawn((ButtonBundle { style: Style { width: Val::Percent(100.0), padding: UiRect::all(Val::Px(8.0)), margin: UiRect::bottom(Val::Px(6.0)), justify_content: JustifyContent::Center, ..default() }, background_color: Color::srgb(0.25, 0.2, 0.35).into(), ..default() }, CouncilAttunementButton { attunement_delta: delta }))
        .with_children(|btn| { btn.spawn(TextBundle { text: Text::from_section(label, TextStyle { font: asset_server.load("fonts/FiraSans-Regular.ttf"), font_size: 14.0, color: Color::WHITE }), ..default() }); });
}

// ... (handle_council_buttons, handle_council_resolved_bursts, spawn_*_burst, play_spatial_sound remain unchanged) ...

fn handle_council_buttons(
    mut interaction_query: Query<(&Interaction, &CouncilAttunementButton), Changed<Interaction>>,
    mut events: EventWriter<CouncilAttunementAction>,
    local_player: Res<crate::local_player::LocalPlayer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    transform_query: Query<&GlobalTransform>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            events.send(CouncilAttunementAction {
                player_id: local_player.id,
                attunement_delta: button.attunement_delta,
            });

            let burst_entity = spawn_valence_burst(&mut commands, button.attunement_delta);

            let distance = if let (Ok(listener_tf), Ok(emitter_tf)) =
                (transform_query.get_single(), transform_query.get(burst_entity))
            {
                listener_tf.translation().distance(emitter_tf.translation())
            } else {
                100.0
            };

            play_spatial_sound(&audio, &asset_server, "sounds/council_burst.ogg", burst_entity, button.attunement_delta, distance);
        }
    }
}

fn handle_council_resolved_bursts(
    mut events: EventReader<CouncilResolved>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    transform_query: Query<&GlobalTransform>,
) {
    for event in events.read() {
        if event.success && event.valence_score > 0.7 {
            let burst_entity = spawn_celebration_burst(&mut commands, event.valence_score);

            let distance = if let (Ok(listener_tf), Ok(emitter_tf)) =
                (transform_query.get_single(), transform_query.get(burst_entity))
            {
                listener_tf.translation().distance(emitter_tf.translation())
            } else {
                100.0
            };

            play_spatial_sound(&audio, &asset_server, "sounds/council_celebration.ogg", burst_entity, event.valence_score, distance);
        }
    }
}

fn spawn_valence_burst(commands: &mut Commands, strength: f32) -> Entity {
    // ... (unchanged for brevity)
    let intensity = strength.clamp(0.1, 1.0);
    let mut effect = ParticleEffect::default();
    effect
        .init(InitPositionSphereModifier { center: Vec3::ZERO, radius: 25.0, ..default() })
        .init(InitVelocitySphereModifier { center: Vec3::ZERO, speed: 45.0 * intensity, ..default() })
        .init(InitLifetimeModifier { lifetime: 0.8 })
        .update(LinearDragModifier { drag: 2.0 })
        .render(ColorOverLifetimeModifier {
            gradient: Gradient::from_colors([
                Color::srgba(0.7, 0.6, 1.0, 0.9),
                Color::srgba(0.5, 0.4, 0.95, 0.0),
            ]),
        })
        .render(SizeOverLifetimeModifier { gradient: Gradient::constant(Vec2::splat(4.0)) });

    commands.spawn((
        ParticleEffectBundle {
            effect,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ValenceBurst,
        AudioEmitter::default(),
        Velocity(Vec3::new(0.0, 12.0, 0.0) * intensity),
        Name::new("CouncilValenceBurst"),
    )).id()
}

fn spawn_celebration_burst(commands: &mut Commands, valence: f32) -> Entity {
    // ... (unchanged)
    let intensity = valence.clamp(0.6, 1.0);
    let mut effect = ParticleEffect::default();
    effect
        .init(InitPositionSphereModifier { center: Vec3::ZERO, radius: 120.0, ..default() })
        .init(InitVelocitySphereModifier { center: Vec3::ZERO, speed: 80.0 * intensity, ..default() })
        .init(InitLifetimeModifier { lifetime: 1.6 })
        .update(LinearDragModifier { drag: 1.2 })
        .render(ColorOverLifetimeModifier {
            gradient: Gradient::from_colors([
                Color::srgba(0.9, 0.85, 1.0, 1.0),
                Color::srgba(0.6, 0.5, 0.95, 0.0),
            ]),
        })
        .render(SizeOverLifetimeModifier { gradient: Gradient::constant(Vec2::splat(6.0)) });

    commands.spawn((
        ParticleEffectBundle {
            effect,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ValenceBurst,
        AudioEmitter::default(),
        Velocity(Vec3::new(0.0, 25.0, 0.0) * intensity),
        Name::new("CouncilCelebrationBurst"),
    )).id()
}

fn play_spatial_sound(
    audio: &Res<Audio>,
    asset_server: &Res<AssetServer>,
    sound_path: &str,
    emitter_entity: Entity,
    intensity: f32,
    distance: f32,
) {
    // ... (unchanged)
    let base_volume = if sound_path.contains("celebration") { 0.9 } else { 0.4 + intensity * 0.4 };
    let falloff = (1.0 / (1.0 + distance * 0.012)).powf(0.82);
    let final_volume = (base_volume * falloff).clamp(0.02, 1.0);
    let pitch_variation = 0.95 + (intensity * 0.1) + rand::random::<f32>() * 0.04;
    audio.play(asset_server.load(sound_path))
        .with_volume(final_volume)
        .with_playback_rate(pitch_variation)
        .with_emitter(emitter_entity);
}

fn handle_council_toggle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::F2) && current_state.get() == &GameState::InCouncil {
        next_state.set(GameState::InGame);
    }
}

// Actual cached draw calls (already computing atlases)
fn update_council_text_cache(
    text_cache: Res<TextAtlasCache>,
    resonance: Res<RecentMercyResonance>,
    last_valence: Res<LastCouncilValence>,
) {
    let font = SimpleBitmapFont::new();

    let _resonance_atlas = text_cache.get_or_render(
        &font,
        &format!("Mercy Resonance: {:.2}", resonance.value),
        [100, 255, 150],
    );

    let _valence_atlas = text_cache.get_or_render(
        &font,
        &format!("Last Valence: {:.2}", last_valence.value),
        [255, 220, 100],
    );
}

// NEW: Migrate Mercy Resonance label to cached blitting
fn update_mercy_resonance_image(
    text_cache: Res<TextAtlasCache>,
    resonance: Res<RecentMercyResonance>,
    mut query: Query<(&mut UiImage, &CachedLabelImage), With<MercyResonanceText>>,
    mut images: ResMut<Assets<Image>>,
) {
    let font = SimpleBitmapFont::new();

    for (mut ui_image, cached) in query.iter_mut() {
        // Get (or create) the cached atlas
        let atlas = text_cache.get_or_render(
            &font,
            &format!("Mercy Resonance: {:.2}", resonance.value),
            [100, 255, 150],
        );

        // Convert RgbImage to Bevy Image
        let bevy_image = Image::from_dynamic(
            image::DynamicImage::ImageRgb8(atlas),
            true,
        );

        // Update the UI image
        if let Some(handle) = images.get_mut(&cached.0) {
            *handle = bevy_image;
        } else {
            ui_image.0 = images.add(bevy_image);
        }
    }
}

// Last Valence still uses Bevy Text (not migrated yet)
fn update_resonance_display(
    last_valence: Res<LastCouncilValence>,
    mut valence_text: Query<&mut Text, With<LastValenceText>>,
) {
    for mut text in valence_text.iter_mut() {
        text.sections[0].value = format!("Last Valence: {:.2}", last_valence.value);
    }
}

fn update_panel_visuals(
    resonance: Res<RecentMercyResonance>,
    mut panel_query: Query<&mut BorderColor, With<CouncilPanel>>,
) {
    let intensity = 0.4 + resonance.value * 0.6;
    for mut border in panel_query.iter_mut() {
        *border = Color::srgb(0.5 * intensity, 0.4 * intensity, 0.9 * intensity).into();
    }
}

fn update_valence_particles(
    resonance: Res<RecentMercyResonance>,
    last_valence: Res<LastCouncilValence>,
    mut commands: Commands,
    mut particle_query: Query<(Entity, &mut ValenceParticles, &mut ParticleEffect)>,
    time: Res<Time>,
) {
    // ... (unchanged)
    let intensity = (resonance.value + last_valence.value) * 0.5;

    if intensity > 0.55 {
        if particle_query.is_empty() {
            let mut effect = ParticleEffect::default();
            effect
                .init(InitPositionSphereModifier { center: Vec3::ZERO, radius: 80.0, ..default() })
                .init(InitVelocitySphereModifier { center: Vec3::ZERO, speed: 12.0, ..default() })
                .init(InitLifetimeModifier { lifetime: 2.5 })
                .update(LinearDragModifier { drag: 0.8 })
                .render(ColorOverLifetimeModifier {
                    gradient: Gradient::from_colors([
                        Color::srgba(0.6, 0.5, 0.95, 0.6),
                        Color::srgba(0.4, 0.3, 0.9, 0.0),
                    ]),
                })
                .render(SizeOverLifetimeModifier { gradient: Gradient::constant(Vec2::splat(3.0)) });

            commands.spawn((
                ParticleEffectBundle { effect, transform: Transform::from_xyz(0.0, 0.0, 0.0), ..default() },
                ValenceParticles,
                Name::new("CouncilValenceParticles"),
            ));
        }
    } else if !particle_query.is_empty() {
        for (entity, _, _) in particle_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn cleanup_valence_particles(
    mut commands: Commands,
    particle_query: Query<Entity, With<ValenceParticles>>,
) {
    for entity in particle_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_council_panel() {
    info!("Exiting Council UI");
}

// Mercy Resonance label successfully migrated to TextAtlasCache cached blitting.