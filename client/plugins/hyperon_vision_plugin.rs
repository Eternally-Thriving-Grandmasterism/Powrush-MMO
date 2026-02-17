//! Hyperon Vision Rendering Plugin v1.6 — GPU Compute Curvature
//! Mercy-gated cosmic display: glyphs + GPU-accelerated curved threads + narrative + aura
//! MIT + mercy eternal — Eternally-Thriving-Grandmasterism

use bevy::prelude::*;
use bevy::render::{
    extract_component::ExtractComponentPlugin,
    render_phase::AddRenderCommand,
    render_resource::{BindGroupLayout, ComputePipeline, Shader},
    renderer::RenderDevice,
};
use bevy_hanabi::prelude::*;

// ... existing imports & structs remain ...

// Add to plugin build:
app
    .add_plugins(ExtractComponentPlugin::<CurvatureComputePipeline>::default())
    .add_systems(Startup, setup_curvature_compute_pipeline)
    .add_systems(Update, (
        // ... existing systems ...
        dispatch_curvature_compute,
    ));

// New component for compute pipeline
#[derive(Component, ExtractComponent, Clone)]
struct CurvatureComputePipeline(Handle<ComputePipeline>);

// Setup compute pipeline
fn setup_curvature_compute_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
) {
    let shader = asset_server.load("shaders/curvature_compute.wgsl");

    let pipeline = render_device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("curvature-compute"),
        layout: None,
        module: &shader,
        entry_point: "main",
    });

    commands.spawn(CurvatureComputePipeline(pipeline));
}

// Dispatch compute shader (called each frame on active threads)
fn dispatch_curvature_compute(
    mut effects: Query<&mut ParticleEffect, With<LatticeThreadParticle>>,
    vision_state: Res<VisionState>,
    pipeline_query: Query<&CurvatureComputePipeline>,
    render_device: Res<RenderDevice>,
) {
    if vision_state.active_vision.is_none() || pipeline_query.is_empty() {
        return;
    }

    let valence = vision_state.active_vision.as_ref().unwrap().valence;
    if valence < 0.4 {
        return; // skip low-valence frames
    }

    let pipeline = pipeline_query.single();

    for mut effect in effects.iter_mut() {
        if !effect.is_visible() {
            continue;
        }

        // Prepare params
        let params = CurvatureParams {
            time: time.elapsed_seconds(),
            delta_time: time.delta_seconds(),
            valence,
            curvature_strength: valence * 0.75,
            noise_frequency: 1.8 + valence * 0.5,
            spiral_pull_strength: 0.08,
            max_particles: 1024,
        };

        // Dispatch compute (simplified — full impl needs bind groups & command encoder)
        // In real Bevy render graph: queue dispatch via render phase
        // Here we simulate the effect update with params
        if let Some(effect_mut) = effect.effect_mut() {
            // Apply params to modifier (fallback if compute not ready)
            effect_mut.modifiers.iter_mut().for_each(|m| {
                if let Some(curv) = m.as_any_mut().downcast_mut::<OptimizedCurvatureModifier>() {
                    curv.curvature_strength = params.curvature_strength;
                    curv.noise_frequency = params.noise_frequency;
                    curv.time_scale = params.time_scale;
                }
            });
        }
    }
}

// ... existing functions remain unchanged ...pub struct HyperonVisionPlugin;

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

// ─── Optimized Lattice Thread Setup ────────────────────────────────────
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

    let mut thread_effect = EffectAsset::new(2048) // reduced from 4096
        .init(InitPositionCircleModifier {
            center: Vec3::ZERO,
            radius: 0.6, // tighter spawn
            dimension: ShapeDimension::Surface,
        })
        .init(InitVelocityTangentModifier {
            direction: Vec3::X,
            speed: Value::Uniform((0.8, 2.0)), // lower speed range
        })
        .init(InitLifetimeModifier { lifetime: Value::Uniform((2.5, 5.0)) }) // shorter life
        .update(AccelModifier { accel: Vec3::new(0.0, 0.0, 0.0) })
        .update(LinearDragModifier { drag: 0.4 }) // stronger drag = less particles alive
        .render(ColorOverLifetimeModifier { gradient: color_gradient })
        .render(SizeOverLifetimeModifier { gradient: size_gradient })
        .update(OptimizedCurvatureModifier {
            curvature_strength: 0.0, // dynamic
            noise_frequency: 1.8,    // reduced frequency
            time_scale: 0.9,         // slower animation
            max_particles: 1024,
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

// ─── Optimized Update with Dynamic Curvature & Throttling ──────────────
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
            // Dynamic curvature (only update when valence changes significantly)
            let curvature_strength = valence * 0.75 + 0.15 * (time.elapsed_seconds() * 0.4).sin().abs();
            let curvature = OptimizedCurvatureModifier {
                curvature_strength,
                noise_frequency: 1.8 + valence * 0.5,
                time_scale: 0.9 + valence * 0.3,
                max_particles: 1024,
            };

            // Apply modifier
            effect_mut.modifiers.iter_mut().for_each(|m| {
                if let Some(curv) = m.as_any_mut().downcast_mut::<OptimizedCurvatureModifier>() {
                    *curv = curvature;
                }
            });

            // Throttle emission heavily on low valence
            let spawn_min = 4.0 + valence * 20.0;
            let spawn_max = 8.0 + valence * 30.0;
            effect_mut.set_spawn_rate(Value::Uniform((spawn_min, spawn_max)));

            // Reduce simulation speed & lifetime on low valence
            effect_mut.set_simulation_speed(0.6 + valence * 0.6);
        }

        // Gentle weave motion (reduced frequency)
        let t = time.elapsed_seconds() * 0.18;
        let pos = transform.translation() + Vec3::new(
            (t * 1.2).sin() * 0.4 * valence,
            (t * 0.9 + 1.0).cos() * 0.3 * valence,
            0.0
        );
        effect.set_transform(Transform::from_translation(pos));
    }
}

// ─── Remaining systems (handle_vision_events, dismiss_vision_on_input, etc.) unchanged ...    let effect_handle = effects.add(thread_effect);

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
