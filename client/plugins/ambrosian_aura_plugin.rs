//! Ambrosian Aura Visuals Plugin — Powrush Client
//! Mercy-gated subtle overseer glow: golden-silver halo + valence threads
//! Tier 4+ resonance only, auto-fade on low coherence
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2dPlugin;
use bevy_hanabi::prelude::*;

#[derive(Component)]
pub struct AmbrosianAura;

#[derive(Component)]
pub struct AuraWisp;

#[derive(Resource)]
pub struct AmbrosianAuraState {
    active_players: Vec<Entity>, // entities with active aura
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AmbrosianAuraMaterial {
    pub valence: f32,           // 0.0–1.0
    pub tier: u8,               // 0–5
    pub pulse_speed: f32,
    pub base_color: Color,
}

impl Material2d for AmbrosianAuraMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/ambrosian_aura.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/ambrosian_aura.wgsl".into()
    }
}

pub struct AmbrosianAuraPlugin;

impl Plugin for AmbrosianAuraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(Material2dPlugin::<AmbrosianAuraMaterial>::default())
            .init_resource::<AmbrosianAuraState>()
            .add_systems(Startup, setup_aura_assets)
            .add_systems(Update, (
                update_ambrosian_aura,
                spawn_wisps_on_tier_advance,
                pulse_aura_visuals,
            ))
            .add_event::<AmbrosianTierAdvancedEvent>();
    }
}

fn setup_aura_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<AmbrosianAuraMaterial>>,
) {
    // Pre-create shared mesh (simple quad for halo)
    let quad_mesh = meshes.add(shape::Quad::new(Vec2::new(4.0, 4.0)).into());

    // Placeholder material — updated dynamically
    materials.add(AmbrosianAuraMaterial {
        valence: 0.0,
        tier: 0,
        pulse_speed: 1.2,
        base_color: Color::srgba(0.8, 0.9, 1.0, 0.3),
    });
}

fn update_ambrosian_aura(
    mut commands: Commands,
    mut aura_query: Query<(Entity, &mut AmbrosianAuraMaterial, &Parent), With<AmbrosianAura>>,
    tier_events: EventReader<AmbrosianTierAdvancedEvent>,
    mut state: ResMut<AmbrosianAuraState>,
) {
    for event in tier_events.read() {
        if event.tier >= 4 {
            // Spawn or update aura on player
            let player_entity = event.player_entity; // assume passed in event

            // Remove old aura if exists
            if let Ok((entity, _, _)) = aura_query.get_single() {
                commands.entity(entity).despawn_recursive();
            }

            // Spawn new aura
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(4.5, 4.5)),
                            color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
                        ..default()
                    },
                    AmbrosianAura,
                    AmbrosianAuraMaterial {
                        valence: event.avg_valence,
                        tier: event.tier,
                        pulse_speed: 1.2 + (event.tier as f32 * 0.2),
                        base_color: if event.tier == 5 {
                            Color::srgba(1.0, 0.95, 0.8, 0.6) // cosmic gold-white
                        } else {
                            Color::srgba(0.8, 0.9, 1.0, 0.4) // subtle silver-gold
                        },
                    },
                ));
            });

            state.active_players.push(player_entity);
        }
    }
}

fn pulse_aura_visuals(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &AmbrosianAuraMaterial), With<AmbrosianAura>>,
) {
    for (mut sprite, material) in query.iter_mut() {
        let pulse = (time.elapsed_seconds() * material.pulse_speed).sin().abs();
        sprite.color = material.base_color * (0.6 + pulse * 0.4);
        sprite.custom_size = Some(Vec2::new(
            4.5 + pulse * 0.5,
            4.5 + pulse * 0.5,
        ));
    }
}

fn spawn_wisps_on_tier_advance(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    tier_events: EventReader<AmbrosianTierAdvancedEvent>,
) {
    for event in tier_events.read() {
        if event.tier >= 4 {
            // Spawn Hanabi wisp trail around player
            let wisp_effect = EffectAsset::new(512)
                .init(InitPositionCircleModifier {
                    center: Vec3::ZERO,
                    radius: 1.5,
                    dimension: ShapeDimension::Surface,
                })
                .init(InitVelocityTangentModifier {
                    direction: Vec3::Y,
                    speed: Value::Uniform((0.5, 1.5)),
                })
                .init(InitLifetimeModifier { lifetime: Value::Uniform((2.0, 4.0)) })
                .render(ColorOverLifetimeModifier {
                    gradient: Gradient::new()
                        .add_key(0.0, Vec4::new(0.8, 0.9, 1.0, 0.0))
                        .add_key(0.5, Vec4::new(1.0, 0.95, 0.8, 0.7))
                        .add_key(1.0, Vec4::new(0.6, 0.8, 1.0, 0.0)),
                })
                .render(SizeOverLifetimeModifier {
                    gradient: Gradient::new()
                        .add_key(0.0, 0.0)
                        .add_key(0.4, 2.0)
                        .add_key(1.0, 0.0),
                });

            let effect_handle = effects.add(wisp_effect);

            commands.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effect_handle),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
                    ..default()
                },
                AuraWisp,
                Parent(event.player_entity), // attach to player
            ));
        }
    }
}

#[derive(Event)]
pub struct AmbrosianTierAdvancedEvent {
    pub player_entity: Entity,
    pub tier: u8,
    pub avg_valence: f32,
}
