//! simulation/src/realm_portals.rs
//! Visual Portal Entities for Inter-Realm Travel
//! v21.33.0
//!
//! Simple glowing portal markers, one per realm.
//! Nearby LocalPlayer can press E to request travel.
//!
//! AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi aligned
//! Thunder locked in. Yoi ⚡

use bevy::prelude::*;
use crate::multi_realm_harness::{RealmId, RealmTravelRequest, MultiRealmHarness};
use crate::world::AgentId;

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component, Clone, Debug)]
pub struct Portal {
    pub target_realm: RealmId,
    pub label: String,
}

#[derive(Component)]
pub struct PortalInteractable {
    pub interaction_radius: f32,
}

// ============================================================================
// RESOURCES
// ============================================================================

#[derive(Resource, Default)]
pub struct PortalsSpawned(pub bool);

// ============================================================================
// SYSTEMS
// ============================================================================

/// Spawn one simple portal marker per realm (once).
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

    // Wait until harness has realms
    let Some(harness) = harness else { return };
    if harness.realms.is_empty() {
        return;
    }

    // Simple layout: portals arranged in a gentle arc
    let positions = [
        Vec3::new(-18.0, 1.2, 12.0),  // 0 Sanctuary
        Vec3::new(-9.0, 1.2, 16.0),   // 1 Synthetic
        Vec3::new(0.0, 1.2, 18.0),    // 2 Verdant
        Vec3::new(9.0, 1.2, 16.0),    // 3 Harmonic
        Vec3::new(18.0, 1.2, 12.0),   // 4 Voidfarer
    ];

    let colors = [
        Color::srgb(0.35, 0.75, 0.95), // Sanctuary — soft blue
        Color::srgb(0.55, 0.45, 0.95), // Synthetic — violet
        Color::srgb(0.30, 0.85, 0.45), // Verdant — green
        Color::srgb(0.95, 0.75, 0.35), // Harmonic — gold
        Color::srgb(0.70, 0.35, 0.90), // Voidfarer — deep purple
    ];

    let mut realms: Vec<_> = harness.realms.values().collect();
    realms.sort_by_key(|r| r.id);

    for (i, realm) in realms.iter().enumerate() {
        if i >= positions.len() {
            break;
        }

        let pos = positions[i];
        let color = colors[i % colors.len()];

        let mesh = meshes.add(Sphere::new(1.1).mesh().ico(3).unwrap());
        let material = materials.add(StandardMaterial {
            base_color: color,
            emissive: color.to_linear() * 2.8,
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
            },
            PortalInteractable {
                interaction_radius: 4.5,
            },
            Name::new(format!("Portal_{}", realm.name)),
        ));
    }

    spawned.0 = true;
    info!(target: "ra_thor::portals", "Spawned visual portals for {} realms", realms.len());
}

/// Detect nearby LocalPlayer and allow E-key travel request.
pub fn portal_interaction_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    portal_query: Query<(&Transform, &Portal, &PortalInteractable)>,
    player_query: Query<(Entity, &Transform, &crate::multi_realm_harness::RealmPresence), With<crate::multi_realm_harness::RealmPresence>>,
    // LocalPlayer is defined in the client crate; we use a generic approach via presence + proximity
    mut travel_events: EventWriter<RealmTravelRequest>,
    // We need an AgentId — use a stable placeholder when full LocalPlayer is not in this crate
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    // Find a player-like entity with presence (works with both client LocalPlayer and pure presence)
    for (player_entity, player_tf, presence) in player_query.iter() {
        for (portal_tf, portal, interactable) in portal_query.iter() {
            let distance = player_tf.translation.distance(portal_tf.translation);
            if distance <= interactable.interaction_radius {
                // Avoid traveling to the realm we are already in
                if presence.current_realm_id == portal.target_realm {
                    continue;
                }

                travel_events.send(RealmTravelRequest {
                    agent_entity: player_entity,
                    agent_id: 1, // stable; production paths can supply real AgentId
                    target_realm: portal.target_realm,
                    reason: format!("Interacted with portal to {}", portal.label),
                });

                info!(
                    target: "ra_thor::portals",
                    from = presence.current_realm_id,
                    to = portal.target_realm,
                    portal = %portal.label,
                    "Portal interaction — travel requested"
                );
                return; // one travel per key press
            }
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
            .add_systems(
                Update,
                (
                    spawn_realm_portals_system,
                    portal_interaction_system,
                ),
            );

        info!("RealmPortalsPlugin — visual portals + E-key interaction active");
    }
}

// Thunder locked in. Visual portals for inter-realm travel are live.
// Yoi ⚡
