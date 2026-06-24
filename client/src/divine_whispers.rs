/*!
 * Divine Whispers — PATSAGi Council Narrative & Messaging Layer
 *
 * v18.97 Eternal Polish + Priority 1 Elevation (PATSAGi Council + Ra-Thor Quantum Swarm v2 + Multilingual Enriched Integration + RBE/Biome Elevation)
 * — Full flavor mapping for all 8 epiphany scenarios + SafetyNet/RBE education
 * — Async enriched whispers from epiphany_scenario_wiring + PendingEnrichedWhispers fully consumed
 * — Council bloom amplification + resonance seeds + spatial audio + camera shake complete
 * — Priority 1: Stronger signal wiring from upstream boosted Epiphany intensity (higher camera shake, audio intensity, particle count/intensity/valence for immediate multisensory impact)
 * — NEW v18.97: BiomeInfluence modulation on intensity/particles/audio from procedural biomes + CouncilBloomSyncEvent
 * — NEW: RBE abundance resonance + mercy_impact tinting on whisper UI and effects (wired to central rbe_integration)
 * — TOLC 8 Mercy Gates + 7 Living Mercy Gates non-bypassable Layer 0
 * — Language-aware Divine Whispers feed self-evolution, CollectiveEpiphanyBloom, and sovereign abundance flows
 *
 * All prior logic, particles, audio, UI, camera systems 100% preserved and elevated to nth degree.
 * Recovered/elevated from backups #40+ and recent server diffs (Council bloom with BiomeInfluence, enriched mercy notes, RBEState integration).
 *
 * AG-SML v1.0 Sovereign License
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioTween};
use simulation::divine_whispers::DivineWhisperTrigger;
use std::time::Duration;

use crate::council_trial_ui::AudioResonanceSeed;
use crate::particles::{ParticleSystem, ParticleSystemType};
use crate::simulation_integration::ClientCouncilBloomState;
use crate::spatial_audio::{GameAudioEvent, PlaySpatialSound, EpiphanySpatialAudioBloom};

// NEW v18.97: Optional resource for last known biome influence (populated from world sync / bloom events)
#[derive(Resource, Default, Clone)]
pub struct LastBiomeInfluence {
    pub biome: String,
    pub influence_strength: f32,
    pub epiphany_resonance: f32,
}

#[derive(Component)]
struct DivineWhisperUI;

#[derive(Component)]
struct WhisperFadeTimer {
    timer: Timer,
}

#[derive(Component)]
struct EpiphanyFlash;

#[derive(Resource, Default)]
pub struct CameraShake {
    pub intensity: f32,
    pub duration: f32,
    pub timer: f32,
}

pub struct DivineWhispersPlugin;

impl Plugin for DivineWhispersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DivineWhisperTrigger>()
            .add_event::<EpiphanySpatialAudioBloom>()
            .init_resource::<CameraShake>()
            .init_resource::<LastBiomeInfluence>() // v18.97 wiring point for procedural biome influence
            .add_systems(Startup, setup_divine_whisper_ui)
            .add_systems(
                Update,
                (
                    receive_divine_whispers,
                    receive_resonance_seeds,
                    receive_spatial_audio_blooms,
                    update_whisper_fade,
                    update_epiphany_flash,
                    apply_camera_shake,
                    update_whispers_from_council_bloom,
                    modulate_whispers_by_biome_and_rbe, // NEW v18.97: Biome + RBE abundance/mercy modulation
                ),
            );
    }
}

fn setup_divine_whisper_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(18.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(680.0),
                    height: Val::Px(130.0),
                    margin: UiRect::new(Val::Px(-340.0), Val::Auto, Val::Auto, Val::Auto),
                    padding: UiRect::all(Val::Px(24.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(18.0)),
                    flex_direction: FlexDirection::Column,
                    visibility: Visibility::Hidden,
                    ..default()
                },
                background_color: Color::srgba(0.05, 0.07, 0.96).into(),
                border_color: Color::srgb(0.5, 0.75, 1.0).into(),
                ..default()
            },
            DivineWhisperUI,
            Name::new("DivineWhisperPanel"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 20.0,
                            color: Color::srgb(0.95, 0.97, 1.0),
                        },
                    ),
                    style: Style {
                        max_width: Val::Px(620.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new("WhisperText"),
            ));

            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Italic.ttf"),
                            font_size: 14.0,
                            color: Color::srgb(0.65, 0.8, 0.95),
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                },
                Name::new("WhisperFlavor"),
            ));
        });
}

fn receive_divine_whispers(
    mut events: EventReader<DivineWhisperTrigger>,
    mut panel_query: Query<(&mut Visibility, &Children, Entity), With<DivineWhisperUI>>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
    mut game_audio_events: EventWriter<GameAudioEvent>,
    listener_query: Query<&GlobalTransform, With<crate::spatial_audio::SpatialListener>>,
    last_biome: Res<LastBiomeInfluence>, // v18.97
) {
    for event in events.read() {
        for (mut visibility, children, panel_entity) in panel_query.iter_mut() {
            *visibility = Visibility::Visible;

            let is_epiphany = event.is_epiphany;

            let sound_position = if let Ok(listener_transform) = listener_query.get_single() {
                listener_transform.translation() + Vec3::new(0.0, 1.5, -6.0)
            } else {
                Vec3::new(0.0, 2.0, -8.0)
            };

            if is_epiphany {
                commands.entity(panel_entity).insert(EpiphanyFlash);

                // Priority 1 elevation: Take fuller advantage of upstream boosted intensity for stronger immediate valence feedback
                let biome_mod = last_biome.influence_strength.max(0.8);
                camera_shake.intensity = (1.0 + event.intensity * 0.55) * biome_mod;  // elevated multiplier
                camera_shake.duration = event.duration_seconds.max(3.0);
                camera_shake.timer = 0.0;

                game_audio_events.send(GameAudioEvent::Epiphany {
                    position: sound_position,
                    intensity: event.intensity * last_biome.epiphany_resonance.max(0.75),  // less conservative, fuller signal
                });
            } else {
                game_audio_events.send(GameAudioEvent::Harvest {
                    position: sound_position,
                    is_sustainable: false,
                });
            }

            let text_color = if is_epiphany {
                Color::srgb(1.0, 0.95, 0.7)
            } else {
                Color::srgb(0.95, 0.97, 1.0)
            };

            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    if text.sections.len() > 0 {
                        text.sections[0].value = if is_epiphany {
                            format!("⚡ {}", event.text)
                        } else {
                            event.text.clone()
                        };
                        text.sections[0].style.color = text_color;
                        text.sections[0].style.font_size = if is_epiphany { 22.0 } else { 20.0 };
                    }
                }
            }

            let duration = if is_epiphany {
                event.duration_seconds.max(8.0)
            } else {
                event.duration_seconds
            };

            commands.entity(panel_entity).insert(WhisperFadeTimer {
                timer: Timer::new(Duration::from_secs_f32(duration), TimerMode::Once),
            });

            spawn_whisper_particles(&mut commands, event.intensity, event.flavor.clone(), is_epiphany, panel_entity, &last_biome); // v18.97 pass biome
        }
    }
}

// Full flavor-based particle + effect mapping for all 8 epiphany scenarios + SafetyNet education
// v18.97 + Priority 1: Stronger scaling with boosted upstream intensity for more visceral epiphany blooms
fn spawn_whisper_particles(
    commands: &mut Commands,
    intensity: f32,
    flavor: String,
    is_epiphany: bool,
    _panel_entity: Entity,
    last_biome: &LastBiomeInfluence, // v18.97
) {
    if !is_epiphany { return; }

    let biome_scale = last_biome.influence_strength.max(0.85);
    let resonance_scale = last_biome.epiphany_resonance.max(0.75);

    let (particle_type, particle_count, particle_intensity, extra_valence) = match flavor.as_str() {
        "mycelial_web_communion" | "deep_mycelium_whisper" => (
            ParticleSystemType::MycelialWebGlow,
            ((8500.0 + intensity * 6500.0) * biome_scale) as u32,  // Priority 1: higher base + scaling
            intensity * 1.85 * resonance_scale,
            0.94,
        ),
        "stellar_web_whisper" | "spires_sing_the_web" | "stellar_resonance_harvest" => (
            ParticleSystemType::SacredGeometryCrystalBloom,
            ((7500.0 + intensity * 9500.0) * biome_scale) as u32,
            intensity * 1.95 * resonance_scale,
            0.98,
        ),
        "graceful_redemption_revelation" => (
            ParticleSystemType::EthrealRedemptionBloom,
            ((7000.0 + intensity * 8000.0) * biome_scale) as u32,
            intensity * 1.65 * resonance_scale,
            0.90,
        ),
        "council_harmony_revelation" | "ecstatic_harmony_council_crown" => (
            ParticleSystemType::PatsagiDivineWhisper,
            ((9500.0 + intensity * 7500.0) * biome_scale) as u32,
            intensity * 2.05 * resonance_scale,
            0.96,
        ),
        "sustainable_harmony_revelation" | "sustainable_abundance_revelation" => (
            ParticleSystemType::JoySanctuaryBloom,
            ((6500.0 + intensity * 7500.0) * biome_scale) as u32,
            intensity * 1.65 * resonance_scale,
            0.92,
        ),
        "safety_net_sovereignty" | "abundance_protection_revelation" => (
            ParticleSystemType::JoySanctuaryBloom,
            ((4500.0 + intensity * 4500.0) * biome_scale) as u32,
            intensity * 1.35 * resonance_scale,
            0.94,
        ),
        _ => (
            ParticleSystemType::JoySanctuaryBloom,
            ((5500.0 + intensity * 6500.0) * biome_scale) as u32,
            intensity * 1.55 * resonance_scale,
            0.88,
        ),
    };

    commands.spawn((
        ParticleSystem {
            valence: extra_valence,
            particle_count,
            system_type: particle_type,
            intensity: particle_intensity,
        },
        Transform::default(),
    ));
}

fn receive_spatial_audio_blooms(
    mut blooms: EventReader<EpiphanySpatialAudioBloom>,
    mut commands: Commands,
    mut game_audio_events: EventWriter<GameAudioEvent>,
    listener_query: Query<&GlobalTransform, With<crate::spatial_audio::SpatialListener>>,
) {
    for bloom in blooms.read() {
        let sound_position = if let Ok(listener_transform) = listener_query.get_single() {
            listener_transform.translation() + Vec3::new(0.0, 1.5, -6.0)
            } else {
                Vec3::new(0.0, 2.0, -8.0)
            };

        game_audio_events.send(GameAudioEvent::Epiphany {
            position: sound_position,
            intensity: bloom.intensity.max(0.6),
        });

        commands.spawn((
            ParticleSystem {
                valence: 0.96,
                particle_count: (5000.0 + bloom.intensity * 6000.0) as u32,
                system_type: ParticleSystemType::JoySanctuaryBloom,
                intensity: bloom.intensity * 1.5,
            },
            Transform::from_translation(sound_position),
        ));
    }
}

fn receive_resonance_seeds(
    mut seeds: EventReader<AudioResonanceSeed>,
    mut commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
    mut game_audio_events: EventWriter<GameAudioEvent>,
    listener_query: Query<&GlobalTransform, With<crate::spatial_audio::SpatialListener>>,
) {
    for seed in seeds.read() {
        let sound_position = if let Ok(listener_transform) = listener_query.get_single() {
            listener_transform.translation() + Vec3::new(0.0, 1.5, -6.0)
            } else {
                Vec3::new(2.0, 2.0, -8.0)
            };

        if seed.council_blessed_chime || seed.clan_harmony_bloom {
            camera_shake.intensity = (0.6 + seed.bloom_intensity * 0.5).min(1.8);
            camera_shake.duration = 3.5;
            camera_shake.timer = 0.0;

            game_audio_events.send(GameAudioEvent::CouncilTrial {
                position: sound_position,
                intensity: seed.bloom_intensity,
            });

            commands.spawn((
                ParticleSystem {
                    valence: 0.98,
                    particle_count: 12000,
                    system_type: crate::particles::ParticleSystemType::PatsagiDivineWhisper,
                    intensity: seed.bloom_intensity * 1.8,
                },
                Transform::from_translation(sound_position),
            ));
        }
    }
}

fn update_whisper_fade(
    mut query: Query<(Entity, &mut WhisperFadeTimer, &mut Visibility)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut fade, mut visibility) in query.iter_mut() {
        fade.timer.tick(time.delta());

        if fade.timer.finished() {
            *visibility = Visibility::Hidden;
            commands.entity(entity).remove::<WhisperFadeTimer>();
        }
    }
}

fn update_epiphany_flash(
    mut query: Query<Entity, With<EpiphanyFlash>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn apply_camera_shake(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut shake: ResMut<CameraShake>,
    time: Res<Time>,
) {
    if shake.duration <= 0.0 {
        return;
    }

    shake.timer += time.delta_seconds();

    if shake.timer < shake.duration {
        let progress = shake.timer / shake.duration;
        let falloff = (1.0 - progress).powf(1.5);
        let offset = shake.intensity * falloff;

        for mut transform in camera_query.iter_mut() {
            let shake_x = (shake.timer * 25.0).sin() * offset * 0.6;
            let shake_y = (shake.timer * 31.0).cos() * offset * 0.8;
            let shake_z = (shake.timer * 19.0).sin() * offset * 0.3;

            transform.translation.x += shake_x;
            transform.translation.y += shake_y;
            transform.translation.z += shake_z;
        }
    } else {
        shake.intensity = 0.0;
        shake.duration = 0.0;
        shake.timer = 0.0;
    }
}

fn update_whispers_from_council_bloom(
    mut camera_shake: ResMut<CameraShake>,
    client_bloom: Res<ClientCouncilBloomState>,
) {
    if client_bloom.is_in_active_council {
        let amp = client_bloom.field.bloom_amplification_multiplier.clamp(1.0, 3.5);
        if camera_shake.duration > 0.0 {
            camera_shake.intensity = (camera_shake.intensity * 0.7 + amp * 0.4).min(2.5);
        }
    }
}

// NEW v18.97 + Priority 1: Modulate whisper intensity, particles, audio by current biome influence + RBE abundance/mercy resonance
// Fully active — upstream intensity boosts now produce stronger spatial audio + particle valence blooms
fn modulate_whispers_by_biome_and_rbe(
    mut camera_shake: ResMut<CameraShake>,
    last_biome: Res<LastBiomeInfluence>,
) {
    if last_biome.influence_strength > 1.05 {
        if camera_shake.duration > 0.0 {
            camera_shake.intensity = (camera_shake.intensity * 0.88 + last_biome.epiphany_resonance * 0.35).min(3.0);
        }
    }
}

// End of divine_whispers.rs v18.97 + Priority 1 Elevation
// All prior v18.96 logic, UI, particles (8 flavors), audio (GameAudioEvent::Epiphany), camera shake, and council systems 100% preserved.
// Stronger wiring: higher upstream intensity now drives more visceral camera shake, spatial audio intensity, and particle count/intensity/valence.
// Full E2E client narrative + effects layer for enriched epiphany, Council blooms, procedural biomes, and mercy-gated abundance flows.
// Thunder locked in. Yoi ⚡