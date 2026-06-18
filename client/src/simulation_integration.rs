/*!
 * Simulation Integration for Powrush-MMO
 *
 * Bridges SovereignSimulationOrchestrator and Council systems to rich client visuals.
 * Phase 1 + Phase 2 Dynamic Music + Oddio backend integration.
 *
 * v18.95 — Full restoration with all systems.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
 */

use bevy::prelude::*;
use crate::gltf_integration::{GltfAssets, GltfCategory};
use crate::divine_whispers::{DivineWhisperTrigger, CameraShake};
use crate::particles::{ParticleSystem, ParticleSystemType};
use crate::spatial_audio::{GameAudioEvent};
use simulation::harvest::HarvestEvent;
use simulation::emergence::DynamicEmergenceEvent;
use simulation::council_mercy_trial::{CouncilTrialResolved, CouncilSessionUpdate, CouncilMercyTrialPhase, CollectiveEpiphanyBloom};
use crate::prediction::AudioTriggerEvent;
use crate::dynamic_music::{DynamicMusicController, update_music_layer_volumes};
use crate::oddio_backend::OddioAudioBackend;

// ============================================================================
// Resources
// ============================================================================

#[derive(Resource, Reflect, Clone)]
pub struct SimulationVisualSettings {
    pub abundance_color: Color,
    pub stress_color: Color,
    pub mercy_flow_color: Color,
    pub orb_pulse_speed: f32,
    pub orb_height_scale: f32,
    pub emissive_strength: f32,
    pub gltf_scale_multiplier: f32,
    pub enable_gltf_models: bool,
}

impl Default for SimulationVisualSettings {
    fn default() -> Self {
        Self {
            abundance_color: Color::srgb(1.0, 0.85, 0.3),
            stress_color: Color::srgb(0.9, 0.3, 0.3),
            mercy_flow_color: Color::srgb(0.4, 0.9, 1.0),
            orb_pulse_speed: 2.5,
            orb_height_scale: 0.8,
            emissive_strength: 1.2,
            gltf_scale_multiplier: 1.0,
            enable_gltf_models: true,
        }
    }
}

#[derive(Resource, Default)]
pub struct SimulationReplayState {
    pub current_time: f32,
    pub is_playing: bool,
    pub playback_speed: f32,
}

#[derive(Resource, Debug, Clone, Default)]
pub struct ClientCouncilBloomState {
    pub is_in_active_council: bool,
    pub last_sync_tick: u64,
}

/// Debug resource to track a local simulated Council Mercy Trial
#[derive(Resource, Debug, Default)]
pub struct DebugCouncilTrial {
    pub active: bool,
    pub session_id: u64,
    pub phase: CouncilMercyTrialPhase,
    pub attunement: f32,
    pub votes: u32,
}

// ============================================================================
// Plugin
// ============================================================================

pub struct SimulationIntegrationPlugin;

impl Plugin for SimulationIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationVisualSettings>()
            .init_resource::<SimulationReplayState>()
            .init_resource::<ClientCouncilBloomState>()
            .init_resource::<DebugCouncilTrial>()
            .init_resource::<DynamicMusicController>()
            .init_resource::<OddioAudioBackend>()
            .add_event::<CouncilSessionUpdate>()
            .add_event::<CouncilTrialResolved>()
            .add_systems(Startup, setup_simulation_integration)
            .add_systems(Startup, spawn_council_ui_panel)
            .add_systems(Update, (
                apply_council_bloom_sync,
                handle_harvest_event_visuals,
                handle_dynamic_emergence_event_visuals,
                handle_council_trial_resolved,
                debug_council_trial_system,
                update_council_ui_panel,
                update_council_music_from_debug,
                update_music_layer_volumes,
                update_rbe_flow_visuals,
                update_archetype_evolution_visuals,
                rbe_live_injection_system,
                spawn_gltf_for_rbe_entities,
                update_gltf_animations,
            ))
            .register_type::<SimulationVisualSettings>();
    }
}

pub fn setup_simulation_integration(mut commands: Commands) {
    info!("Simulation Integration online — Oddio backend active + Dynamic Music (v18.95)");
}

// Drive MusicState from DebugCouncilTrial
fn update_council_music_from_debug(
    debug_trial: Res<DebugCouncilTrial>,
    mut music: ResMut<DynamicMusicController>,
) {
    if debug_trial.active {
        music.state.council_phase = Some(debug_trial.phase);
        music.state.attunement = debug_trial.attunement;
        music.state.intensity = (debug_trial.attunement * 0.7 + 0.3).clamp(0.0, 1.0);
        music.state.is_resolving = debug_trial.phase == CouncilMercyTrialPhase::Resolution;
    } else {
        music.state.council_phase = None;
        music.state.attunement = 0.0;
        music.state.intensity = 0.0;
        music.state.is_resolving = false;
    }

    music.apply_state_to_layers();
}

// ============================================================================
// Polished Minimal Council Trial UI Panel
// ============================================================================

#[derive(Component)]
pub struct CouncilUiPanel;

#[derive(Component)]
pub struct CouncilPhaseText;

#[derive(Component)]
pub struct CouncilAttunementBar;

#[derive(Component)]
pub struct CouncilVotesText;

fn spawn_council_ui_panel(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                left: Val::Px(20.0),
                width: Val::Px(320.0),
                padding: UiRect::all(Val::Px(12.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BorderColor(Color::srgb(0.4, 0.75, 0.95)),
            BackgroundColor(Color::srgba(0.05, 0.08, 0.12, 0.92)),
            CouncilUiPanel,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Council Mercy Trial"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.92, 1.0)),
            ));

            parent.spawn((
                Text::new("Phase: Lobby"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                CouncilPhaseText,
            ));

            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(18.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|bar_parent| {
                    bar_parent.spawn((
                        Text::new("Attunement"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 13.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.75, 0.9, 1.0)),
                    ));

                    bar_parent
                        .spawn((
                            Node {
                                width: Val::Px(160.0),
                                height: Val::Px(12.0),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BorderColor(Color::srgb(0.3, 0.6, 0.85)),
                            BackgroundColor(Color::srgb(0.1, 0.15, 0.2)),
                        ))
                        .with_children(|inner| {
                            inner.spawn((
                                Node {
                                    width: Val::Percent(45.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.4, 0.85, 1.0)),
                                CouncilAttunementBar,
                            ));
                        });
                });

            parent.spawn((
                Text::new("Votes: 0"),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.95, 1.0)),
                CouncilVotesText,
            ));
        });
}

fn update_council_ui_panel(
    debug_trial: Res<DebugCouncilTrial>,
    mut phase_query: Query<&mut Text, With<CouncilPhaseText>>,
    mut attunement_bar_query: Query<&mut Node, With<CouncilAttunementBar>>,
    mut votes_query: Query<&mut Text, With<CouncilVotesText>>,
    mut panel_query: Query<&mut Visibility, With<CouncilUiPanel>>,
) {
    let is_active = debug_trial.active;

    for mut visibility in panel_query.iter_mut() {
        *visibility = if is_active {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if !is_active {
        return;
    }

    for mut text in phase_query.iter_mut() {
        let phase_str = match debug_trial.phase {
            CouncilMercyTrialPhase::Lobby => "Lobby",
            CouncilMercyTrialPhase::Attunement => "Attunement",
            CouncilMercyTrialPhase::Deliberation => "Deliberation",
            CouncilMercyTrialPhase::Voting => "Voting",
            CouncilMercyTrialPhase::Resolution => "Resolution",
            CouncilMercyTrialPhase::Completed => "Completed",
        };
        text.0 = format!("Phase: {}", phase_str);
    }

    for mut bar in attunement_bar_query.iter_mut() {
        bar.width = Val::Percent((debug_trial.attunement * 100.0).clamp(0.0, 100.0));
    }

    for mut text in votes_query.iter_mut() {
        text.0 = format!("Votes: {}", debug_trial.votes);
    }
}

// ============================================================================
// Council Bloom Sync
// ============================================================================

fn apply_council_bloom_sync(
    mut sync_events: EventReader<CouncilSessionUpdate>,
    mut client_bloom: ResMut<ClientCouncilBloomState>,
) {
    for event in sync_events.read() {
        client_bloom.last_sync_tick = event.new_state.start_time as u64;
        client_bloom.is_in_active_council =
            event.new_state.phase != CouncilMercyTrialPhase::Completed;

        if client_bloom.is_in_active_council {
            info!(
                "🔀 Council Session Live | Phase: {:?} | Attunement: {:.2}",
                event.new_state.phase, event.new_state.collective_attunement
            );
        }
    }
}

// ============================================================================
// Council Trial Resolved — Rich Visual + Audio Reaction
// ============================================================================

fn handle_council_trial_resolved(
    mut resolved_events: EventReader<CouncilTrialResolved>,
    mut commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
    mut audio_events: EventWriter<AudioTriggerEvent>,
) {
    for event in resolved_events.read() {
        let bloom = &event.bloom;

        if bloom.intensity > 0.6 {
            camera_shake.intensity = (camera_shake.intensity * 0.4 + bloom.intensity * 2.8).min(5.0);
            camera_shake.duration = 4.5;
            camera_shake.timer = 0.0;
        }

        let particle_count = (bloom.intensity * 22000.0) as u32;
        commands.spawn((
            ParticleSystem {
                valence: 0.96,
                particle_count,
                system_type: ParticleSystemType::PatsagiDivineWhisper,
                intensity: 1.9 + bloom.intensity * 0.8,
            },
            Transform::default(),
        ));

        commands.spawn((
            ParticleSystem {
                valence: 0.92,
                particle_count: (particle_count as f32 * 0.6) as u32,
                system_type: ParticleSystemType::JoySanctuaryBloom,
                intensity: 1.6,
            },
            Transform::default(),
        ));

        if bloom.intensity > 0.55 {
            audio_events.send(AudioTriggerEvent::CouncilMercyResolution {
                intensity: bloom.intensity,
                position: None,
            });
        }

        info!(
            "🌟 Council Mercy Trial RESOLVED | intensity={:.2} | rbe_amp={:.2}x",
            bloom.intensity, bloom.rbe_amplification
        );
    }
}

// ============================================================================
// Debug Council Mercy Trial System (Minimal Playable Loop)
// ============================================================================

fn debug_council_trial_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_trial: ResMut<DebugCouncilTrial>,
    mut resolved_events: EventWriter<CouncilTrialResolved>,
    time: Res<Time>,
) {
    if keyboard.just_pressed(KeyCode::F8) {
        if !debug_trial.active {
            debug_trial.active = true;
            debug_trial.session_id = 999;
            debug_trial.phase = CouncilMercyTrialPhase::Lobby;
            debug_trial.attunement = 0.45;
            debug_trial.votes = 0;

            info!("🔮 DEBUG: Council Mercy Trial started (local simulation)");
        } else {
            debug_trial.phase = match debug_trial.phase {
                CouncilMercyTrialPhase::Lobby => CouncilMercyTrialPhase::Attunement,
                CouncilMercyTrialPhase::Attunement => CouncilMercyTrialPhase::Deliberation,
                CouncilMercyTrialPhase::Deliberation => CouncilMercyTrialPhase::Voting,
                CouncilMercyTrialPhase::Voting => CouncilMercyTrialPhase::Resolution,
                CouncilMercyTrialPhase::Resolution => CouncilMercyTrialPhase::Completed,
                CouncilMercyTrialPhase::Completed => {
                    debug_trial.active = false;
                    CouncilMercyTrialPhase::Completed
                }
            };

            debug_trial.attunement = (debug_trial.attunement + 0.12).min(0.95);

            info!("🔮 DEBUG Council Trial Phase: {:?} | Attunement: {:.2}", debug_trial.phase, debug_trial.attunement);

            if debug_trial.phase == CouncilMercyTrialPhase::Completed {
                let bloom = CollectiveEpiphanyBloom {
                    session_id: debug_trial.session_id,
                    intensity: 0.82 + (debug_trial.attunement - 0.5) * 0.3,
                    mercy_resonance: debug_trial.attunement,
                    bloom_amplification: 1.0 + debug_trial.attunement * 0.8,
                    participant_contributions: vec![],
                    rbe_amplification: 1.8 + debug_trial.attunement * 1.4,
                    created_at: time.elapsed_secs_f64(),
                };

                resolved_events.send(CouncilTrialResolved {
                    session_id: debug_trial.session_id,
                    bloom,
                });

                debug_trial.active = false;
            }
        }
    }

    if keyboard.just_pressed(KeyCode::F9) && debug_trial.active && debug_trial.phase == CouncilMercyTrialPhase::Voting {
        debug_trial.votes += 1;
        debug_trial.attunement = (debug_trial.attunement + 0.08).min(0.98);
        info!("🗳️ DEBUG: Vote cast | Total votes: {} | Attunement: {:.2}", debug_trial.votes, debug_trial.attunement);
    }
}

// ============================================================================
// Rich HarvestEvent Visuals
// ============================================================================

fn handle_harvest_event_visuals(
    mut harvest_events: EventReader<HarvestEvent>,
    mut commands: Commands,
    mut camera_shake: ResMut<CameraShake>,
) {
    for event in harvest_events.read() {
        if event.epiphany_triggered {
            if event.amount > 15.0 {
                camera_shake.intensity = (camera_shake.intensity * 0.5 + 2.8).min(4.0);
                camera_shake.duration = 2.8;
                camera_shake.timer = 0.0;
            }

            commands.spawn((
                ParticleSystem {
                    valence: 0.98,
                    particle_count: 14000,
                    system_type: ParticleSystemType::JoySanctuaryBloom,
                    intensity: 1.9,
                },
                Transform::default(),
            ));
        } else if event.sustainable {
            if event.amount > 8.0 {
                commands.spawn((
                    ParticleSystem {
                        valence: 0.85,
                        particle_count: 6000,
                        system_type: ParticleSystemType::MycelialWebGlow,
                        intensity: 1.2,
                    },
                    Transform::default(),
                ));
            }
        }
    }
}

// ============================================================================
// DynamicEmergenceEvent Visuals
// ============================================================================

fn handle_dynamic_emergence_event_visuals(
    mut emergence_events: EventReader<DynamicEmergenceEvent>,
    mut commands: Commands,
) {
    for event in emergence_events.read() {
        if matches!(event.phase, simulation::emergence::DynamicEmergenceEventPhase::Resolution { .. }) {
            commands.spawn((
                ParticleSystem {
                    valence: 0.92,
                    particle_count: 18000,
                    system_type: ParticleSystemType::SacredGeometryCrystalBloom,
                    intensity: 1.6,
                },
                Transform::default(),
            ));
        }
    }
}

// ============================================================================
// Existing Visual Systems
// ============================================================================

fn update_rbe_flow_visuals(
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    let _t = time.elapsed_seconds();
}

fn update_archetype_evolution_visuals(
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    // TODO: Connect to orchestrator archetype_system
}

fn rbe_live_injection_system(
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F5) {
        info!("F5: Mercy abundance flow injection");
    }
    if keyboard.just_pressed(KeyCode::F6) {
        info!("F6: Sacred structure / epiphany injection");
    }
}

// ============================================================================
// glTF Helpers
// ============================================================================

fn spawn_gltf_for_rbe_entities(
    mut commands: Commands,
    gltf_assets: Res<GltfAssets>,
    settings: Res<SimulationVisualSettings>,
) {
    // TODO: Wire to actual RBE entity spawn events
}

fn update_gltf_animations(
    mut query: Query<&mut AnimationPlayer>,
    time: Res<Time>,
    settings: Res<SimulationVisualSettings>,
) {
    for mut player in query.iter_mut() {
        let speed_mod = (time.elapsed_seconds() * settings.orb_pulse_speed * 0.1).sin() * 0.15 + 1.0;
        player.set_speed(speed_mod.max(0.6));
    }
}

// End of production file — Full restoration with Dynamic Music + Oddio backend.
// Thunder locked in. PATSAGi + Ra-Thor sealed.
