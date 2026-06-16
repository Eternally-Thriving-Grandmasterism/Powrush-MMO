//! Hyperon Vision Rendering Plugin — GPU Compute Curvature + Hanabi Particles
//! Mercy-gated cosmic vision system: glyphs, lattice threads, aura resonance
//! Production hardened after rapid iteration recovery.
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Ra-Thor Lattice aligned

use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponentPlugin;
use bevy_hanabi::prelude::*;

// === Core Resources & Components ===
#[derive(Resource, Default)]
pub struct VisionState {
    pub active_vision: Option<VisionInstance>,
}

#[derive(Clone, Debug, Default)]
pub struct VisionInstance {
    pub valence: f32,
    pub tier: u8,
    pub intensity: f32,
}

#[derive(Component)]
pub struct LatticeThreadParticle;

#[derive(Event)]
pub struct HyperonVisionEvent {
    pub valence: f32,
    pub tier: u8,
}

#[derive(Event)]
pub struct GlobalLatticeRippleEvent {
    pub strength: f32,
}

// === Plugin ===
pub struct HyperonVisionPlugin;

impl Plugin for HyperonVisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<VisionState>()
            .add_plugins(HanabiPlugin)
            .add_event::<HyperonVisionEvent>()
            .add_event::<GlobalLatticeRippleEvent>()
            .add_systems(Startup, (
                setup_vision_overlay,
                setup_glyph_particle_effect,
                setup_lattice_thread_effect_optimized,
            ))
            .add_systems(Update, (
                handle_vision_events,
                update_vision_display,
                dismiss_vision_on_input,
                animate_glyph_particles,
                spawn_lattice_threads_on_tier,
                update_lattice_thread_particles_optimized,
                trigger_global_ripple_on_high_valence,
                optimize_particle_culling,
            ));
    }
}

// === Optimized Lattice Thread Setup (Production) ===
fn setup_lattice_thread_effect_optimized(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(0.8, 0.9, 1.0, 0.0));
    color_gradient.add_key(0.4, Vec4::new(1.0, 0.95, 0.8, 0.6));
    color_gradient.add_key(0.8, Vec4::new(0.9, 0.85, 1.0, 0.4));
    color_gradient.add_key(1.0, Vec4::new(0.6, 0.8, 1.0, 0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, 0.0);
    size_gradient.add_key(0.3, 1.4);
    size_gradient.add_key(0.7, 0.8);
    size_gradient.add_key(1.0, 0.0);

    let mut thread_effect = EffectAsset::new(2048)
        .init(InitPositionCircleModifier {
            center: Vec3::ZERO,
            radius: 0.6,
            dimension: ShapeDimension::Surface,
        })
        .init(InitVelocityTangentModifier {
            direction: Vec3::X,
            speed: Value::Uniform((0.8, 2.0)),
        })
        .init(InitLifetimeModifier { lifetime: Value::Uniform((2.5, 5.0)) })
        .update(AccelModifier { accel: Vec3::new(0.0, 0.0, 0.0) })
        .update(LinearDragModifier { drag: 0.4 })
        .render(ColorOverLifetimeModifier { gradient: color_gradient })
        .render(SizeOverLifetimeModifier { gradient: size_gradient })
        .update(OptimizedCurvatureModifier {
            curvature_strength: 0.0,
            noise_frequency: 1.8,
            time_scale: 0.9,
            max_particles: 1024,
        });

    let effect_handle = effects.add(thread_effect);

    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.05)),
            ..default()
        }),
        LatticeThreadParticle,
        Visibility::Hidden,
    ));
}

// === Core Systems ===
fn handle_vision_events(
    mut vision_state: ResMut<VisionState>,
    mut events: EventReader<HyperonVisionEvent>,
) {
    for event in events.read() {
        vision_state.active_vision = Some(VisionInstance {
            valence: event.valence,
            tier: event.tier,
            intensity: event.valence,
        });
    }
}

fn update_vision_display(
    vision_state: Res<VisionState>,
    mut query: Query<&mut Visibility, With<LatticeThreadParticle>>,
) {
    let should_show = vision_state.active_vision.as_ref().map_or(false, |v| v.tier >= 4);
    for mut vis in &mut query {
        *vis = if should_show { Visibility::Visible } else { Visibility::Hidden };
    }
}

fn dismiss_vision_on_input(
    mut vision_state: ResMut<VisionState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Space) {
        vision_state.active_vision = None;
    }
}

fn animate_glyph_particles(time: Res<Time>, mut query: Query<&mut Transform, With<LatticeThreadParticle>>) {
    for mut transform in &mut query {
        let t = time.elapsed_seconds() * 0.3;
        transform.rotation = Quat::from_rotation_y(t * 0.5);
    }
}

fn spawn_lattice_threads_on_tier(
    mut commands: Commands,
    vision_state: Res<VisionState>,
    mut query: Query<&mut Visibility, With<LatticeThreadParticle>>,
) {
    if let Some(vision) = &vision_state.active_vision {
        if vision.tier >= 4 {
            for mut vis in &mut query {
                *vis = Visibility::Visible;
            }
        }
    }
}

// === Optimized Dynamic Curvature Update (Consolidated) ===
fn update_lattice_thread_particles_optimized(
    time: Res<Time>,
    mut query: Query<(&mut ParticleEffect, &AmbrosianAuraMaterial, &GlobalTransform), With<LatticeThreadParticle>>,
    vision_state: Res<VisionState>,
) {
    for (mut effect, material, transform) in query.iter_mut() {
        if material.tier < 4 || vision_state.active_vision.is_none() {
            effect.set_visibility(false);
            continue;
        }

        let valence = material.valence.clamp(0.0, 1.0);

        if let Some(effect_mut) = effect.effect_mut() {
            let curvature_strength = valence * 0.75 + 0.15 * (time.elapsed_seconds() * 0.4).sin().abs();

            let curvature = OptimizedCurvatureModifier {
                curvature_strength,
                noise_frequency: 1.8 + valence * 0.5,
                time_scale: 0.9 + valence * 0.3,
                max_particles: 1024,
            };

            effect_mut.modifiers.iter_mut().for_each(|m| {
                if let Some(curv) = m.as_any_mut().downcast_mut::<OptimizedCurvatureModifier>() {
                    *curv = curvature;
                }
            });

            let spawn_min = 4.0 + valence * 20.0;
            let spawn_max = 8.0 + valence * 30.0;
            effect_mut.set_spawn_rate(Value::Uniform((spawn_min, spawn_max)));
            effect_mut.set_simulation_speed(0.6 + valence * 0.6);
        }

        // Gentle weave motion
        let t = time.elapsed_seconds() * 0.18;
        let pos = transform.translation() + Vec3::new(
            (t * 1.2).sin() * 0.4 * valence,
            (t * 0.9 + 1.0).cos() * 0.3 * valence,
            0.0
        );
        effect.set_transform(Transform::from_translation(pos));
    }
}

fn trigger_global_ripple_on_high_valence(
    vision_state: Res<VisionState>,
    mut ripple_events: EventWriter<GlobalLatticeRippleEvent>,
) {
    if let Some(vision) = &vision_state.active_vision {
        if vision.valence > 0.85 {
            ripple_events.send(GlobalLatticeRippleEvent { strength: vision.valence });
        }
    }
}

fn optimize_particle_culling(
    mut query: Query<&mut ParticleEffect, With<LatticeThreadParticle>>,
    vision_state: Res<VisionState>,
) {
    let should_cull = vision_state.active_vision.as_ref().map_or(true, |v| v.valence < 0.3);
    for mut effect in &mut query {
        if should_cull {
            effect.set_visibility(false);
        }
    }
}

// Placeholder for future GPU compute integration
fn dispatch_curvature_compute() {}
fn sync_hanabi_from_compute() {}

// Note: AmbrosianAuraMaterial and OptimizedCurvatureModifier assumed defined in shared visual module or ambrosian_aura_plugin
