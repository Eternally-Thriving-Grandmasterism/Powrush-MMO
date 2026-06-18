//! client/src/prediction.rs
//! Production-grade Client Prediction + Professional Audio Asset System v18.95
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned

use bevy::prelude::*;
use simulation::spatial_interest::{
    InterestZone, InterestZoneReplicated, CouncilBloomStateReplicated, RequestResync,
};
use simulation::harvest::HarvestEvent;
use simulation::emergence::DynamicEmergenceEvent;
use crate::replication::{DecodedUpdate, ReplicatedFields, UpdatePayload};
use crate::rbe_client_sync::RbeClientSync;
use std::collections::VecDeque;

// ============================================================
// PROFESSIONAL AUDIO ASSET SYSTEM
// ============================================================

#[derive(Resource, Debug)]
pub struct AudioAssets {
    pub rollback_whoosh: Handle<AudioSource>,
    pub epiphany_bloom: Handle<AudioSource>,
    pub emergence_resonance: Handle<AudioSource>,
}

impl AudioAssets {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            rollback_whoosh: asset_server.load("audio/rollback_whoosh.ogg"),
            epiphany_bloom: asset_server.load("audio/epiphany_bloom.ogg"),
            emergence_resonance: asset_server.load("audio/emergence_resonance.ogg"),
        }
    }
}

#[derive(Event, Debug, Clone)]
pub enum AudioTriggerEvent {
    RollbackWhoosh { intensity: f32 },
    EpiphanyBloomResonance { amount: f32 },
    EmergenceResonanceField { id: u64 },
}

pub fn audio_playback_system(
    mut events: EventReader<AudioTriggerEvent>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for event in events.read() {
        match event {
            AudioTriggerEvent::RollbackWhoosh { intensity } => {
                audio.play(audio_assets.rollback_whoosh.clone())
                    .with_volume(*intensity.clamp(0.3, 1.0));
            }
            AudioTriggerEvent::EpiphanyBloomResonance { amount: _ } => {
                audio.play(audio_assets.epiphany_bloom.clone())
                    .with_volume(0.9);
            }
            AudioTriggerEvent::EmergenceResonanceField { id: _ } => {
                audio.play(audio_assets.emergence_resonance.clone())
                    .with_volume(0.8);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct MovementInput {
    pub timestamp: f64,
    pub velocity: Vec3,
}

#[derive(Resource, Debug)]
pub struct RollbackConfig {
    pub discrepancy_threshold: f32,
    pub max_rollback_age_seconds: f64,
    pub velocity_correction_weight: f32,
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            discrepancy_threshold: 1.6,
            max_rollback_age_seconds: 0.3,
            velocity_correction_weight: 0.6,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct InputBuffer {
    pub inputs: VecDeque<MovementInput>,
    pub max_size: usize,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            inputs: VecDeque::with_capacity(48),
            max_size: 48,
        }
    }

    pub fn push(&mut self, input: MovementInput) {
        if self.inputs.len() >= self.max_size {
            self.inputs.pop_front();
        }
        self.inputs.push_back(input);
    }

    pub fn clear(&mut self) {
        self.inputs.clear();
    }
}

#[derive(Component, Debug, Default)]
pub struct RollbackVisualIndicator {
    pub active_until: f64,
    pub intensity: f32,
}

#[derive(Component, Default, Debug, Clone)]
pub struct PredictedPosition {
    pub position: Vec3,
    pub velocity: Vec3,
    pub last_server_timestamp: f64,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct ClientBloomState {
    pub active_blooms: Vec<simulation::spatial_interest::CouncilBloomZone>,
    pub version: u64,
    pub last_received_timestamp: f64,
}

pub fn handle_interest_zone_replicated(
    time: Res<Time>,
    mut events: EventReader<InterestZoneReplicated>,
    mut query: Query<(&mut InterestZone, &mut crate::spatial_interest::ReplicationVersion)>,
    mut resync_events: EventWriter<RequestResync>,
) {
    let client_time = time.elapsed_secs_f64();

    for event in events.read() {
        if let Ok((mut zone, mut rep_version)) = query.get_mut(event.entity) {
            let age = client_time - event.server_timestamp;

            if age > 1.5 {
                warn!("Old InterestZone update (age: {:.2}s, v{})", age, event.version);
            }

            if event.version > rep_version.interest_zone_version {
                *zone = event.zone.clone();
                rep_version.interest_zone_version = event.version;
            } else if event.version + 8 < rep_version.interest_zone_version {
                warn!("Large version gap for {:?} (local v{}, server v{}). Requesting resync.", event.entity, rep_version.interest_zone_version, event.version);
                resync_events.send(RequestResync { entity: event.entity });
            }
        }
    }
}

pub fn handle_council_bloom_state_replicated(
    time: Res<Time>,
    mut events: EventReader<CouncilBloomStateReplicated>,
    mut client_blooms: ResMut<ClientBloomState>,
) {
    let client_time = time.elapsed_secs_f64();

    for event in events.read() {
        let age = client_time - event.server_timestamp;

        if age > 2.0 {
            warn!("Old CouncilBloomState update (age: {:.2}s)", age);
        }

        if event.version > client_blooms.version {
            client_blooms.active_blooms = event.active_blooms.clone();
            client_blooms.version = event.version;
            client_blooms.last_received_timestamp = event.server_timestamp;
        }
    }
}

pub fn client_predict_local_player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PredictedPosition), With<crate::spatial_interest::SpatialParticipant>>,
    mut input_buffer: ResMut<InputBuffer>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut predicted) in &mut query {
        predicted.position += predicted.velocity * dt;
        transform.translation = predicted.position;

        let now = time.elapsed_secs_f64();
        input_buffer.push(MovementInput {
            timestamp: now,
            velocity: predicted.velocity,
        });
    }
}

pub fn perform_rollback_and_replay(
    mut query: Query<(&mut PredictedPosition, &mut Transform, Option<&mut RollbackVisualIndicator>), With<crate::spatial_interest::SpatialParticipant>>,
    mut input_buffer: ResMut<InputBuffer>,
    mut audio_events: EventWriter<AudioTriggerEvent>,
    config: Res<RollbackConfig>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs_f64();

    for (mut predicted, mut transform, mut maybe_indicator) in &mut query {
        let discrepancy = (predicted.position - transform.translation).length();

        if discrepancy > config.discrepancy_threshold {
            let correction_time = if predicted.last_server_timestamp > 0.0 {
                predicted.last_server_timestamp
            } else {
                now - config.max_rollback_age_seconds.min(0.3)
            };

            while let Some(front) = input_buffer.inputs.front() {
                if front.timestamp < correction_time {
                    input_buffer.inputs.pop_front();
                } else {
                    break;
                }
            }

            let replay_dt = 1.0 / 60.0;
            let mut replayed_velocity = predicted.velocity;

            for input in input_buffer.inputs.iter() {
                predicted.position += input.velocity * replay_dt;
                replayed_velocity = replayed_velocity.lerp(input.velocity, config.velocity_correction_weight);
            }

            predicted.velocity = replayed_velocity;
            transform.translation = predicted.position;

            if let Some(indicator) = &mut maybe_indicator {
                indicator.active_until = now + 0.7;
                indicator.intensity = (discrepancy / 3.5).clamp(0.4, 1.0);

                audio_events.send(AudioTriggerEvent::RollbackWhoosh {
                    intensity: indicator.intensity,
                });
            }
        }
    }
}

pub fn update_rollback_visual_indicator(
    time: Res<Time>,
    mut query: Query<(&mut RollbackVisualIndicator, &mut Sprite, &mut Transform)>,
) {
    let now = time.elapsed_secs_f64();

    for (mut indicator, mut sprite, mut transform) in &mut query {
        if now < indicator.active_until {
            let remaining = (indicator.active_until - now) / 0.7;
            let flash = (remaining * 3.0).sin().abs() * indicator.intensity;

            sprite.color = Color::srgb(1.0, 0.25 + flash * 0.5, 0.35 + flash * 0.4);

            let scale_pulse = 1.0 + flash * 0.12;
            transform.scale = Vec3::splat(scale_pulse);
        } else {
            sprite.color = Color::WHITE;
            transform.scale = Vec3::ONE;
        }
    }
}

pub fn smooth_reconcile_position(
    mut query: Query<(&mut PredictedPosition, &mut Transform)>,
) {
    for (mut predicted, mut transform) in &mut query {
        let target = predicted.position;
        let current = transform.translation;

        if (target - current).length() > 0.2 {
            transform.translation = current.lerp(target, 0.4);
            predicted.position = transform.translation;
        }
    }
}

pub fn predict_interest_zone_expansion(
    mut query: Query<(&mut InterestZone, &PredictedPosition)>,
) {
    for (mut interest, predicted) in &mut query {
        let speed = predicted.velocity.length();
        let speed_factor = (speed / 20.0).clamp(0.0, 1.5);

        interest.base_radius = 80.0 + speed_factor * 40.0;
        interest.mercy_resonance = (interest.mercy_resonance * 0.9 + speed_factor * 0.3).min(2.5);
    }
}

#[derive(Component, Debug, Default)]
pub struct HarvestEpiphanyVisual {
    pub lifetime: f32,
    pub max_lifetime: f32,
}

pub fn handle_harvest_event(
    mut events: EventReader<HarvestEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut audio_events: EventWriter<AudioTriggerEvent>,
    query: Query<&Transform, With<crate::spatial_interest::SpatialParticipant>>,
) {
    for event in events.read() {
        if event.epiphany_triggered {
            if let Ok(player_transform) = query.get_single() {
                let pos = player_transform.translation + Vec3::new(0.0, 45.0, 0.0);

                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(22.0))),
                    MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.7, 0.95, 1.0)))),
                    Transform::from_translation(pos),
                    HarvestEpiphanyVisual { lifetime: 0.0, max_lifetime: 1.6 },
                ));

                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(38.0))),
                    MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgba(0.5, 0.85, 1.0, 0.6)))),
                    Transform::from_translation(pos),
                    HarvestEpiphanyVisual { lifetime: 0.0, max_lifetime: 2.1 },
                ));

                audio_events.send(AudioTriggerEvent::EpiphanyBloomResonance {
                    amount: event.amount,
                });
            }
        }
    }
}

pub fn update_harvest_epiphany_visuals(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut HarvestEpiphanyVisual, &mut Transform, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    let dt = time.delta_secs();

    for (entity, mut visual, mut transform, mut material) in &mut query {
        visual.lifetime += dt;

        let t = visual.lifetime / visual.max_lifetime;

        if t >= 1.0 {
            commands.entity(entity).despawn();
        } else {
            let scale = 1.0 + t * 3.2;
            transform.scale = Vec3::splat(scale);

            let alpha = (1.0 - t * 0.9).powf(0.65);
            let base_color = if visual.max_lifetime > 1.9 { Color::srgba(0.5, 0.85, 1.0, alpha) } else { Color::srgba(0.7, 0.95, 1.0, alpha) };
            material.0.color = base_color;
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct EmergenceResonanceField {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub intensity: f32,
}

pub fn handle_dynamic_emergence_event(
    mut events: EventReader<DynamicEmergenceEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut audio_events: EventWriter<AudioTriggerEvent>,
    query: Query<&Transform, With<crate::spatial_interest::SpatialParticipant>>,
) {
    for event in events.read() {
        if matches!(event.phase, simulation::emergence::DynamicEmergenceEventPhase::Resolution { .. }) {
            let spawn_pos = if let Ok(player_t) = query.get_single() {
                player_t.translation + Vec3::new(0.0, 20.0, 0.0)
            } else {
                Vec3::new(0.0, 30.0, 0.0)
            };

            commands.spawn((
                Mesh2d(meshes.add(Circle::new(52.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.35, 0.82, 0.55)))),
                Transform::from_translation(spawn_pos),
                EmergenceResonanceField {
                    lifetime: 0.0,
                    max_lifetime: 2.6,
                    intensity: 1.0,
                },
            ));

            audio_events.send(AudioTriggerEvent::EmergenceResonanceField {
                id: event.id,
            });
        }
    }
}

pub fn update_emergence_resonance_fields(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut EmergenceResonanceField, &mut Transform, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    let dt = time.delta_secs();

    for (entity, mut field, mut transform, mut material) in &mut query {
        field.lifetime += dt;

        let t = field.lifetime / field.max_lifetime;

        if t >= 1.0 {
            commands.entity(entity).despawn();
        } else {
            let pulse = ((t * 5.5).sin().abs() * 0.25 + 0.88) * field.intensity;
            let scale = 1.0 + t * 1.7;

            transform.scale = Vec3::splat(scale * pulse);

            let alpha = (1.0 - t * 0.88).powf(0.55) * field.intensity;
            material.0.color = Color::srgba(0.35, 0.82, 0.55, alpha);
        }
    }
}

pub fn apply_decoded_updates_to_prediction(
    updates: Vec<DecodedUpdate>,
    mut predicted_query: Query<(&mut PredictedPosition, &mut Transform)>,
    mut rbe_sync: ResMut<RbeClientSync>,
) {
    for update in updates {
        match update.payload {
            UpdatePayload::RbeTransaction(tx) => {
                rbe_sync.set_latest_harvest_result(
                    if tx.amount > 0.0 {
                        crate::rbe_client_sync::RbeHarvestResult::Success(tx.amount)
                    } else {
                        crate::rbe_client_sync::RbeHarvestResult::Failed("Server correction".to_string())
                    }
                );
            }
            _ => {}
        }
    }
}

pub struct PredictionPlugin;

impl Plugin for PredictionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientBloomState>()
            .init_resource::<InputBuffer>()
            .init_resource::<RollbackConfig>()
            .add_event::<AudioTriggerEvent>()
            .add_systems(Update, (
                handle_interest_zone_replicated,
                handle_council_bloom_state_replicated,
                client_predict_local_player_movement,
                perform_rollback_and_replay,
                update_rollback_visual_indicator,
                smooth_reconcile_position,
                predict_interest_zone_expansion,
                handle_harvest_event,
                update_harvest_epiphany_visuals,
                handle_dynamic_emergence_event,
                update_emergence_resonance_fields,
                audio_playback_system,
            ));
    }
}

// End of production file — Professional AudioAssets resource + preloaded handles + real playback.
// Thunder locked in. PATSAGi + Ra-Thor sealed.
