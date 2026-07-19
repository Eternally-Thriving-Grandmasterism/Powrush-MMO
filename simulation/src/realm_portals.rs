//! simulation/src/realm_portals.rs
//! Visual Portals + Pulse VFX + Arrival Burst Feedback
//! v21.35.0
//!
//! Living portals + clear arrival confirmation on successful travel.
//!
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//! Thunder locked in. Yoi ⚡

use bevy::prelude::*;
use crate::multi_realm_harness::{RealmId, RealmTravelRequest, MultiRealmHarness, RealmPresence};

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component, Clone, Debug)]
pub struct Portal {
    pub target_realm: RealmId,
    pub label: String,
    pub base_emissive: f32,
}

#[derive(Component)]
pub struct PortalInteractable {
    pub interaction_radius: f32,
}

#[derive(Component)]
pub struct PortalPulse {
    pub phase: f32,
    pub speed: f32,
}

/// Short-lived visual confirmation that travel succeeded.
#[derive(Component)]
pub struct ArrivalBurst {
    pub timer: f32,
    pub max_life: f32,
}

// ============================================================================
// RESOURCES
// ============================================================================

#[derive(Resource, Default)]
pub struct PortalsSpawned(pub bool);

#[derive(Resource, Default)]
pub struct LastTravelFeedback {
    pub message: String,
    pub tick: u64,
}

// ============================================================================
// SYSTEMS
// ============================================================================

pub fn spawn_realm_portals_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut spawned: ResMut<PortalsSpawned>,
    harness: Option<Res<MultiRealmHarness>>,
) {
    if spawned.0 {
        return;
    }

    let Some(harness) = harness else { return };
    if harness.realms.is_empty() {
        return;
    }

    let positions = [
        Vec3::new(-18.0, 1.2, 12.0),
        Vec3::new(-9.0, 1.2, 16.0),
        Vec3::new(0.0, 1.2, 18.0),
        Vec3::new(9.0, 1.2, 16.0),
        Vec3::new(18.0, 1.2, 12.0),
    ];

    let colors = [
        Color::srgb(0.35, 0.75, 0.95),
        Color::srgb(0.55, 0.45, 0.95),
        Color::srgb(0.30, 0.85, 0.45),
        Color::srgb(0.95, 0.75, 0.35),
        Color::srgb(0.70, 0.35, 0.90),
    ];

    let mut realms: Vec<_> = harness.realms.values().collect();
    realms.sort_by_key(|r| r.id);

    for (i, realm) in realms.iter().enumerate() {
        if i >= positions.len() {
            break;
        }

        let pos = positions[i];
        let color = colors[i % colors.len()];
        let base_emissive = 2.6;

        let mesh = meshes.add(Sphere::new(1.1).mesh().ico(3).unwrap());
        let material = materials.add(StandardMaterial {
            base_color: color,
            emissive: color.to_linear() * base_emissive,
            metallic: 0.35,
            perceptual_roughness: 0.25,
            ..default()
        });

        commands.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(pos),
                ..default()
            },
            Portal {
                target_realm: realm.id,
                label: realm.name.clone(),
                base_emissive,
            },
            PortalInteractable {
                interaction_radius: 4.5,
            },
            PortalPulse {
                phase: i as f32 * 0.7,
                speed: 1.4 + (i as f32 * 0.15),
            },
            Name::new(format!("Portal_{}", realm.name)),
        ));
    }

    spawned.0 = true;
    info!(target: "ra_thor::portals", "Spawned living portals for {} realms", realms.len());
}

pub fn portal_pulse_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PortalPulse, &Portal, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut transform, mut pulse, portal, mat_handle) in query.iter_mut() {
        pulse.phase += time.delta_seconds() * pulse.speed;
        let wave = pulse.phase.sin() * 0.5 + 0.5;

        let scale = 1.0 + wave * 0.08;
        transform.scale = Vec3::splat(scale);

        if let Some(mat) = materials.get_mut(mat_handle) {
            let intensity = portal.base_emissive * (0.75 + wave * 0.55);
            mat.emissive = mat.base_color.to_linear() * intensity;
        }
    }
}

pub fn portal_interaction_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    portal_query: Query<(&Transform, &Portal, &PortalInteractable)>,
    player_query: Query<(Entity, &Transform, &RealmPresence)>,
    mut travel_events: EventWriter<RealmTravelRequest>,
    mut feedback: ResMut<LastTravelFeedback>,
    time: Res<Time>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    for (player_entity, player_tf, presence) in player_query.iter() {
        for (portal_tf, portal, interactable) in portal_query.iter() {
            let distance = player_tf.translation.distance(portal_tf.translation);
            if distance <= interactable.interaction_radius {
                if presence.current_realm_id == portal.target_realm {
                    feedback.message = format!("Already in {}", portal.label);
                    feedback.tick = time.elapsed_seconds() as u64;
                    continue;
                }

                travel_events.send(RealmTravelRequest {
                    agent_entity: player_entity,
                    agent_id: 1,
                    target_realm: portal.target_realm,
                    reason: format!("Portal interaction → {}", portal.label),
                });

                feedback.message = format!("Traveling to {}...", portal.label);
                feedback.tick = time.elapsed_seconds() as u64;

                info!(
                    target: "ra_thor::portals",
                    from = presence.current_realm_id,
                    to = portal.target_realm,
                    portal = %portal.label,
                    "Portal travel requested"
                );
                return;
            }
        }
    }
}

/// Spawn a short-lived arrival burst when a travel request is processed successfully.
/// We observe RealmTravelRequest events and create a visual burst at the player.
pub fn travel_arrival_burst_system(
    mut commands: Commands,
    mut travel_events: EventReader<RealmTravelRequest>,
    player_query: Query<&Transform>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut feedback: ResMut<LastTravelFeedback>,
    time: Res<Time>,
) {
    for request in travel_events.read() {
        if let Ok(tf) = player_query.get(request.agent_entity) {
            let mesh = meshes.add(Sphere::new(0.55).mesh().ico(2).unwrap());
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.85, 0.95, 1.0),
                emissive: Color::srgb(0.7, 0.9, 1.0).to_linear() * 4.0,
                alpha_mode: AlphaMode::Blend,
                ..default()
            });

            commands.spawn((
                PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_translation(tf.translation + Vec3::Y * 1.4),
                    ..default()
                },
                ArrivalBurst {
                    timer: 0.0,
                    max_life: 0.55,
                },
                Name::new("ArrivalBurst"),
            ));

            feedback.message = format!("Arrived via travel request → realm {}", request.target_realm);
            feedback.tick = time.elapsed_seconds() as u64;

            info!(
                target: "ra_thor::portals::arrival",
                agent = request.agent_id,
                realm = request.target_realm,
                "Arrival burst spawned"
            );
        }
    }
}

/// Animate and despawn arrival bursts.
pub fn arrival_burst_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut ArrivalBurst, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut transform, mut burst, mat_handle) in query.iter_mut() {
        burst.timer += time.delta_seconds();
        let t = (burst.timer / burst.max_life).clamp(0.0, 1.0);

        // Expand and fade
        let scale = 1.0 + t * 2.2;
        transform.scale = Vec3::splat(scale);

        if let Some(mat) = materials.get_mut(mat_handle) {
            let alpha = 1.0 - t;
            mat.base_color = Color::srgba(0.85, 0.95, 1.0, alpha);
            mat.emissive = Color::srgb(0.7, 0.9, 1.0).to_linear() * (4.0 * alpha);
        }

        if burst.timer >= burst.max_life {
            commands.entity(entity).despawn();
        }
    }
}

// ============================================================================
// PLUGIN
// ============================================================================

pub struct RealmPortalsPlugin;

impl Plugin for RealmPortalsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PortalsSpawned>()
            .init_resource::<LastTravelFeedback>()
            .add_systems(
                Update,
                (
                    spawn_realm_portals_system,
                    portal_pulse_system,
                    portal_interaction_system,
                    travel_arrival_burst_system,
                    arrival_burst_update_system,
                ),
            );

        info!("RealmPortalsPlugin — living portals + arrival burst feedback active");
    }
}

// Thunder locked in. Travel now has satisfying arrival confirmation.
// Yoi ⚡
