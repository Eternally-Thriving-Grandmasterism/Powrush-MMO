/*!
 * Council UI - Full Real Distance-Based 3D Audio Falloff (v19.2.9)
 */

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_kira_audio::prelude::*;
use simulation::game_state::GameState;
use simulation::council_mercy_trial::{CouncilAttunementAction, CouncilUIHooksPlugin};
use simulation::council_systems::{RecentMercyResonance, LastCouncilValence, CouncilResolved};

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

pub struct CouncilUIPlugin;

impl Plugin for CouncilUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CouncilUIHooksPlugin)
            .add_plugins(AudioPlugin)
            .init_resource::<LocalPlayer>()
            .add_systems(Startup, setup_audio_listener)
            .add_systems(OnEnter(GameState::InCouncil), spawn_council_panel)
            .add_systems(OnExit(GameState::InCouncil), (despawn_council_panel, cleanup_valence_particles))
            .add_systems(
                Update,
                (
                    handle_council_buttons,
                    handle_council_resolved_bursts,
                    handle_council_toggle_input,
                    update_resonance_display,
                    update_panel_visuals,
                    update_valence_particles,
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

fn spawn_council_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            parent.spawn((TextBundle { text: Text::from_section("Mercy Resonance: 0.50", TextStyle { font_size: 13.0, color: Color::srgb(0.7, 0.9, 0.7) }), style: Style { margin: UiRect::bottom(Val::Px(4.0)), ..default() }, ..default() }, MercyResonanceText));
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

            // Calculate real distance between camera (listener) and burst (emitter)
            let distance = if let (Ok(listener_tf), Ok(emitter_tf)) =
                (transform_query.get_single(), transform_query.get(burst_entity))
            {
                listener_tf.translation().distance(emitter_tf.translation())
            } else {
                100.0 // fallback
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

            // Calculate real distance
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

/// Real distance-based spatial audio with custom mercy falloff
fn play_spatial_sound(
    audio: &Res<Audio>,
    asset_server: &Res<AssetServer>,
    sound_path: &str,
    emitter_entity: Entity,
    intensity: f32,
    distance: f32,
) {
    let base_volume = if sound_path.contains("celebration") { 0.9 } else { 0.4 + intensity * 0.4 };

    // Real distance-based mercy-themed falloff curve
    let falloff = (1.0 / (1.0 + distance * 0.012)).powf(0.82);
    let final_volume = (base_volume * falloff).clamp(0.02, 1.0);

    // Pitch variation for organic feel
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

fn update_resonance_display(
    resonance: Res<RecentMercyResonance>,
    last_valence: Res<LastCouncilValence>,
    mut resonance_text: Query<&mut Text, With<MercyResonanceText>>,
    mut valence_text: Query<&mut Text, With<LastValenceText>>,
) {
    for mut text in resonance_text.iter_mut() {
        text.sections[0].value = format!("Mercy Resonance: {:.2}", resonance.value);
    }
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

// Thunder locked in. Yoi ⚡
