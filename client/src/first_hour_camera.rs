/*!
 * First-hour camera glance — sovereignty first (v21.99.1)
 *
 * If the human is idle for a breath and has not harvested yet,
 * the camera eases a few degrees toward the nearest glowing node.
 * Any WASD / look intent cancels instantly. Never hijacks the stick.
 *
 * PATSAGi ruling: camera assist must not steal sovereignty.
 * CARD FLESH-CAMERA-PLACE — glance runs in PostUpdate, before
 * TransformPropagate, and the punch scale wins over the hint.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::prelude::*;

use crate::first_harvest_epiphany::FirstHarvestEpiphany;
use crate::first_session_guidance::{FirstSessionGuidance, GuidanceObjective};
use crate::human_presence::punch_scale_for_graphics;
use crate::local_settings::{LocalFeedbackFeel, LocalMeshLodFeel};
use crate::mercy_harvest_nodes::{MercyHarvestNode, NearbyMercyNode};
use shared::local_settings::GraphicsPreset;

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
        app.init_resource::<FirstHourGlance>().add_systems(
            PostUpdate,
            glance_toward_nearest_node.before(TransformSystem::TransformPropagate),
        );
    }
}

/// Slerp weight for one glance. `punch_scale` is already graphics-capped.
fn glance_blend(punch_scale: f32) -> f32 {
    GLANCE_BLEND * punch_scale
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
    feedback: Option<Res<LocalFeedbackFeel>>,
    mesh_lod: Option<Res<LocalMeshLodFeel>>,
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

    // Punch wins. Missing feel, or a scale of 0 (reduced motion), never writes a camera.
    let Some(feedback) = feedback else {
        return;
    };
    let preset = mesh_lod
        .map(|f| f.preset)
        .unwrap_or(GraphicsPreset::Medium);
    let punch_scale = punch_scale_for_graphics(feedback.camera_punch_scale, preset);
    if punch_scale == 0.0 {
        return;
    }
    let blend = glance_blend(punch_scale);

    for mut cam in &mut cameras {
        let to = target - cam.translation;
        if to.length_squared() < 0.01 {
            continue;
        }
        let desired = cam.looking_at(target, Vec3::Y).rotation;
        // Limit how far we pull from current facing. Rotation only.
        let blended = cam.rotation.slerp(desired, blend);
        let delta = cam.rotation.angle_between(blended);
        if delta > MAX_YAW_HINT {
            // already close enough to a strong turn — leave sovereignty
            continue;
        }
        cam.rotation = blended;
    }
}

#[cfg(test)]
mod tests {
    use super::{glance_blend, GLANCE_BLEND};
    use crate::human_presence::punch_scale_for_graphics;
    use crate::local_settings::LocalFeedbackFeel;
    use shared::local_settings::{GraphicsPreset, LocalSettings};

    #[test]
    fn reduced_motion_on_via_feedback_feel_gives_blend_zero() {
        let mut settings = LocalSettings::peace_defaults();
        settings.toggle_reduced_motion();
        assert!(settings.reduced_motion);
        let feel = LocalFeedbackFeel::from_settings(&settings);
        for preset in [GraphicsPreset::Low, GraphicsPreset::Medium, GraphicsPreset::High] {
            let scale = punch_scale_for_graphics(feel.camera_punch_scale, preset);
            assert_eq!(glance_blend(scale), 0.0);
        }
    }

    #[test]
    fn scale_one_medium_is_exactly_glance_blend() {
        let scale = punch_scale_for_graphics(1.0, GraphicsPreset::Medium);
        assert_eq!(glance_blend(scale), GLANCE_BLEND);
    }

    #[test]
    fn low_at_one_is_positive_and_within_glance_blend() {
        let scale = punch_scale_for_graphics(1.0, GraphicsPreset::Low);
        let blend = glance_blend(scale);
        assert!(blend > 0.0);
        assert!(blend <= GLANCE_BLEND);
    }

    #[test]
    fn helper_never_returns_more_than_glance_blend() {
        let samples = [
            0.0,
            0.45,
            1.0,
            -1.0,
            2.0,
            8.0,
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ];
        for preset in GraphicsPreset::ALL {
            for raw in samples {
                let blend = glance_blend(punch_scale_for_graphics(raw, preset));
                assert!(
                    blend <= GLANCE_BLEND,
                    "preset {preset:?} raw {raw} blend {blend}"
                );
                assert!(blend.is_finite() && blend >= 0.0);
            }
        }
        assert!(glance_blend(0.0) <= GLANCE_BLEND);
        assert_eq!(glance_blend(1.0), GLANCE_BLEND);
    }
}
