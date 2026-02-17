//! Hyperon Vision Rendering Plugin v1.3 — Enhanced Thread Weaving, Global Ripple, Optimized Particles
//! Mercy-gated cosmic display: glyphs + advanced threads + narrative + aura + ripple
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use std::time::Duration;

// ─── Components ────────────────────────────────────────────────────────
#[derive(Component)]
struct VisionOverlay;

#[derive(Component)]
struct VisionGlyphParticle;

#[derive(Component)]
struct LatticeThreadParticle;

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
    path: Vec<String>,
}

#[derive(Event)]
pub struct HyperonVisionEvent {
    pub vision: HyperonVisionData,
}

#[derive(Event)]
pub struct GlobalLatticeRippleEvent {
    pub intensity: f32,
}

// ─── Plugin ────────────────────────────────────────────────────────────
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
                setup_lattice_thread_effect,
            ))
            .add_systems(Update, (
                handle_vision_events,
                update_vision_display,
                dismiss_vision_on_input,
                animate_glyph_particles,
                spawn_lattice_threads_on_tier,
                update_lattice_thread_particles_enhanced,
                trigger_global_ripple_on_high_valence,
                optimize_particle_culling,
            ));
    }
}

// ─── Setup Functions (unchanged from v1.2 except thread effect) ────────
fn setup_vision_overlay(/* ... */) { /* unchanged */ }

fn setup_glyph_particle_effect(/* ... */) { /* unchanged */ }

fn setup_lattice_thread_effect(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(0.8, 0.9, 1.0, 0.0));
    color_gradient.add_key(0.4, Vec4::new(1.0, 0.95, 0.8, 0.7));
    color_gradient.add_key(0.8, Vec4::new(0.9, 0.85, 1.0, 0.5));
    color_gradient.add_key(1.0, Vec4::new(0.6, 0.8, 1.0, 0.0));

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, 0.0);
    size_gradient.add_key(0.2, 1.8);
    size_gradient.add_key(0.6, 1.2);
    size_gradient.add_key(1.0, 0.0);

    let thread_effect = EffectAsset::new(4096) // increased capacity for weaving
        .init(InitPositionCircleModifier {
            center: Vec3::ZERO,
            radius: 0.8,
            dimension: ShapeDimension::Surface,
        })
        .init(InitVelocityTangentModifier {
            direction: Vec3::X,
            speed: Value::Uniform((1.2, 3.0)),
        })
        .init(InitLifetimeModifier { lifetime: Value::Uniform((4.0, 8.0)) })
        .update(AccelModifier { accel: Vec3::new(0.0, 0.0, 0.0) })
        .update(LinearDragModifier { drag: 0.3 }) // gentle slowing for weave feel
        .render(ColorOverLifetimeModifier { gradient: color_gradient })
        .render(SizeOverLifetimeModifier { gradient: size_gradient })
        .render(ParticleTextureModifier { texture: None }); // add thread texture later

    let effect_handle = effects.add(thread_effect);

    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.05)),
            ..default()
        },
        LatticeThreadParticle,
        Visibility::Hidden,
    ));
}

// ─── Enhanced Thread Weaving Patterns ──────────────────────────────────
fn update_lattice_thread_particles_enhanced(
    time: Res<Time>,
    mut query: Query<(&mut ParticleEffect, &AmbrosianAuraMaterial, &GlobalTransform), With<LatticeThreadParticle>>,
    vision_state: Res<VisionState>,
) {
    for (mut effect, material, transform) in query.iter_mut() {
        if material.tier < 4 || vision_state.active_vision.is_none() {
            effect.set_visibility(false);
            continue;
        }

        let valence = material.valence;

        if let Some(effect_mut) = effect.effect_mut() {
            // Valence-scaled emission rate & speed
            effect_mut.set_spawn_rate(Value::Uniform((
                8.0 + valence * 60.0,
                15.0 + valence * 80.0
            )));
            effect_mut.set_simulation_speed(0.7 + valence * 0.8);

            // Curvature noise for organic weaving
            let t = time.elapsed_seconds() * 0.4;
            let curvature = valence * 0.15 * (t.sin() * 0.5 + 0.5);
            // Apply via custom modifier or simulation space tweak (expand later)

            // Direction influenced by player velocity (placeholder)
            // effect_mut.set_velocity_direction(/* player velocity */);
        }

        // Gentle global weave motion
        let t = time.elapsed_seconds() * 0.25;
        let pos = transform.translation() + Vec3::new(
            (t * 1.5).sin() * 0.6 * valence,
            (t * 1.1 + 1.0).cos() * 0.5 * valence,
            0.0
        );
        effect.set_transform(Transform::from_translation(pos));
    }
}

// ─── Global Lattice Ripple Effects (on high collective valence) ────────
fn trigger_global_ripple_on_high_valence(
    mut ripple_events: EventWriter<GlobalLatticeRippleEvent>,
    vision_state: Res<VisionState>,
) {
    if let Some(vision) = &vision_state.active_vision {
        if vision.valence >= 0.96 {
            ripple_events.send(GlobalLatticeRippleEvent {
                intensity: vision.valence * 0.8,
            });
        }
    }
}

// ─── Performance Optimization: Particle Culling ────────────────────────
fn optimize_particle_culling(
    mut query: Query<(&mut ParticleEffect, &Visibility), With<LatticeThreadParticle>>,
    camera_query: Query<&Camera, With<Camera>>,
) {
    let camera_pos = camera_query.single().world_position(); // placeholder

    for (mut effect, vis) in query.iter_mut() {
        let dist = (effect.transform.translation() - camera_pos).length();
        if dist > 50.0 {
            effect.set_visibility(false);
        } else {
            effect.set_visibility(true);
        }
    }
}

// ... existing handle_vision_events, dismiss_vision_on_input, etc. unchanged ...
