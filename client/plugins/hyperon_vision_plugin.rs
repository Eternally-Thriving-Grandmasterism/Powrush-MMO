//! Hyperon Vision Rendering Plugin v1.4 — Curvature Modifier Expansion
//! Mercy-gated cosmic display: glyphs + advanced curved threads + narrative + aura
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use std::time::Duration;

// ... existing structs & imports remain ...

// Custom Hanabi modifier for valence-driven curvature
#[derive(Clone, Copy, Default)]
struct CurvatureModifier {
    curvature_strength: f32,    // 0.0–1.0 (valence-scaled)
    noise_frequency: f32,
    time_scale: f32,
}

impl Modifier for CurvatureModifier {
    fn apply(&self, particle: &mut Particle, delta_time: f32) {
        let t = particle.age * self.time_scale;
        let noise = noise::perlin::Perlin::new(0)
            .get([particle.position.x as f64 * self.noise_frequency as f64, t as f64])
            as f32 * 0.5 + 0.5;

        // Curvature vector (arc-like bend)
        let bend_dir = Vec3::new(
            noise.sin() * self.curvature_strength,
            noise.cos() * self.curvature_strength * 0.7,
            0.0
        );

        // Apply to velocity
        particle.velocity += bend_dir * delta_time * 20.0;

        // Subtle pull toward center for spiral feel on high valence
        if self.curvature_strength > 0.6 {
            let to_center = -particle.position.normalize_or_zero() * 0.1 * self.curvature_strength;
            particle.velocity += to_center * delta_time;
        }
    }
}

// In setup_lattice_thread_effect — add modifier
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

    let mut thread_effect = EffectAsset::new(4096)
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
        .update(LinearDragModifier { drag: 0.3 })
        .render(ColorOverLifetimeModifier { gradient: color_gradient })
        .render(SizeOverLifetimeModifier { gradient: size_gradient });

    // Add custom curvature modifier
    thread_effect = thread_effect.update(CurvatureModifier {
        curvature_strength: 0.0, // dynamic in update
        noise_frequency: 2.5,
        time_scale: 1.2,
    });

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

// Enhanced update with dynamic curvature
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
            // Dynamic curvature strength
            let curvature = CurvatureModifier {
                curvature_strength: valence * 0.8 + 0.2 * (time.elapsed_seconds() * 0.5).sin().abs(),
                noise_frequency: 2.5 + valence * 1.0,
                time_scale: 1.2 + valence * 0.6,
            };

            // Apply modifier override (Hanabi supports runtime modifier updates)
            effect_mut.modifiers.iter_mut().for_each(|m| {
                if let Some(curv) = m.as_any_mut().downcast_mut::<CurvatureModifier>() {
                    *curv = curvature;
                }
            });

            // Emission rate & speed scaling
            effect_mut.set_spawn_rate(Value::Uniform((
                8.0 + valence * 60.0,
                15.0 + valence * 80.0
            )));
            effect_mut.set_simulation_speed(0.7 + valence * 0.8);
        }

        // Gentle weave motion
        let t = time.elapsed_seconds() * 0.25;
        let pos = transform.translation() + Vec3::new(
            (t * 1.5).sin() * 0.6 * valence,
            (t * 1.1 + 1.0).cos() * 0.5 * valence,
            0.0
        );
        effect.set_transform(Transform::from_translation(pos));
    }
}

// ... existing functions (handle_vision_events, dismiss_vision_on_input, etc.) unchanged ...
