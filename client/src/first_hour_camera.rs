/*!
 * First-hour camera glance — sovereignty first (v21.99.1)
 *
 * If the human is idle for a breath and has not harvested yet,
 * the camera eases a few degrees toward the nearest glowing node.
 * Any WASD / look intent cancels instantly. Never hijacks the stick.
 *
 * PATSAGi ruling: camera assist must not steal sovereignty.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::first_session_guidance::{FirstSessionGuidance, GuidanceObjective};
use crate::mercy_harvest_nodes::{MercyHarvestNode, NearbyMercyNode};

const IDLE_BEFORE_GLANCE: f32 = 1.35;
const GLANCE_BLEND: f32 = 0.045;
const MAX_YAW_HINT: f32 = 0.18; // radians ~10°

#[derive(Resource, Debug)]
pub struct FirstHourGlance {
    pub idle_secs: f32,
    pub active: bool,
}

impl Default for FirstHourGlance {
    fn default() -> Self {
        Self {
            idle_secs: 0.0,
            active: false,
        }
    }
}

pub struct FirstHourCameraPlugin;

impl Plugin for FirstHourCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FirstHourGlance>()
            .add_systems(Update, glance_toward_nearest_node);
    }
}

fn moving(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.pressed(KeyCode::KeyW)
        || keyboard.pressed(KeyCode::KeyA)
        || keyboard.pressed(KeyCode::KeyS)
        || keyboard.pressed(KeyCode::KeyD)
        || keyboard.pressed(KeyCode::ArrowUp)
        || keyboard.pressed(KeyCode::ArrowDown)
        || keyboard.pressed(KeyCode::ArrowLeft)
        || keyboard.pressed(KeyCode::ArrowRight)
        || keyboard.pressed(KeyCode::Space)
}

fn glance_toward_nearest_node(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    nearby: Res<NearbyMercyNode>,
    harvest: Res<FirstHarvestEpiphany>,
    guidance: Res<FirstSessionGuidance>,
    mut glance: ResMut<FirstHourGlance>,
    nodes: Query<&GlobalTransform, With<MercyHarvestNode>>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
) {
    if harvest.first_harvest_lived || guidance.dismissed {
        glance.active = false;
        glance.idle_secs = 0.0;
        return;
    }
    let approach = matches!(
        guidance.objective,
        GuidanceObjective::MoveAround
            | GuidanceObjective::ApproachGlowingNode
            | GuidanceObjective::HarvestWithInteract
    );
    if !approach || !nearby.nodes_exist {
        glance.active = false;
        return;
    }

    if moving(&keyboard) {
        glance.idle_secs = 0.0;
        glance.active = false;
        return;
    }

    glance.idle_secs += time.delta_seconds();
    if glance.idle_secs < IDLE_BEFORE_GLANCE {
        return;
    }
    glance.active = true;

    let Some(entity) = nearby.entity else {
        return;
    };
    let Ok(node_tf) = nodes.get(entity) else {
        return;
    };
    let target = node_tf.translation();

    for mut cam in &mut cameras {
        let to = target - cam.translation;
        if to.length_squared() < 0.01 {
            continue;
        }
        let desired = cam.looking_at(target, Vec3::Y).rotation;
        // Limit how far we pull from current facing
        let blended = cam.rotation.slerp(desired, GLANCE_BLEND);
        let delta = cam.rotation.angle_between(blended);
        if delta > MAX_YAW_HINT {
            // already close enough to a strong turn — leave sovereignty
            continue;
        }
        cam.rotation = blended;
    }
}
