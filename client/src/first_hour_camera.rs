/*!
 * First-hour camera glance — sovereignty first (v21.99.1)
 *
 * If the human is idle for a breath and has not harvested yet,
 * the camera eases a few degrees toward the nearest glowing node.
 * Any WASD / look intent cancels instantly. Never hijacks the stick.
 * CARD CAM-FEEL-PASS-1 — an RMB orbit drag or a mouse scroll cancels
 * the glance the same way WASD does, so the hint does not fight the orbit.
 *
 * PATSAGi ruling: camera assist must not steal sovereignty.
 * CARD FLESH-CAMERA-PLACE — glance runs in PostUpdate, before
 * TransformPropagate, and the punch scale wins over the hint.
 * Contact: info@Rathor.ai | Yoi ⚡
 */

use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
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
        app.init_resource::<FirstHourGlance>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .add_systems(
                PostUpdate,
                glance_toward_nearest_node.before(TransformSystems::Propagate),
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

/// Right button held with mouse motion. A bare click is not an orbit drag.
fn orbit_drag(buttons: &ButtonInput<MouseButton>, motion: &AccumulatedMouseMotion) -> bool {
    if !buttons.pressed(MouseButton::Right) {
        return false;
    }
    let d = motion.delta;
    d.x.is_finite() && d.y.is_finite() && d.length_squared() > 0.0
}

/// Wheel movement. Zoom uses y; any non-zero wheel delta is a mouse scroll.
fn mouse_scroll(scroll: &AccumulatedMouseScroll) -> bool {
    let d = scroll.delta;
    (d.x.is_finite() && d.x != 0.0) || (d.y.is_finite() && d.y != 0.0)
}

fn look_intent(
    keyboard: &ButtonInput<KeyCode>,
    buttons: &ButtonInput<MouseButton>,
    motion: &AccumulatedMouseMotion,
    scroll: &AccumulatedMouseScroll,
) -> bool {
    moving(keyboard) || orbit_drag(buttons, motion) || mouse_scroll(scroll)
}

fn glance_toward_nearest_node(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
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

    if look_intent(&keyboard, &mouse_buttons, &mouse_motion, &mouse_scroll) {
        glance.idle_secs = 0.0;
        glance.active = false;
        return;
    }

    glance.idle_secs += time.delta_secs();
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
    use super::{
        glance_blend, glance_toward_nearest_node, FirstHourGlance, GLANCE_BLEND, IDLE_BEFORE_GLANCE,
    };
    use crate::first_harvest_epiphany::FirstHarvestEpiphany;
    use crate::first_session_guidance::FirstSessionGuidance;
    use crate::human_presence::punch_scale_for_graphics;
    use crate::local_settings::LocalFeedbackFeel;
    use crate::mercy_harvest_nodes::NearbyMercyNode;
    use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
    use bevy::prelude::*;
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

    fn armed_glance() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .init_resource::<FirstHarvestEpiphany>()
            .init_resource::<FirstSessionGuidance>()
            .init_resource::<NearbyMercyNode>()
            .init_resource::<FirstHourGlance>()
            .add_systems(Update, glance_toward_nearest_node);
        {
            let mut nearby = app.world_mut().resource_mut::<NearbyMercyNode>();
            nearby.nodes_exist = true;
            let mut glance = app.world_mut().resource_mut::<FirstHourGlance>();
            glance.idle_secs = IDLE_BEFORE_GLANCE + 0.5;
            glance.active = true;
        }
        app
    }

    fn assert_cancelled(app: &App) {
        let glance = app.world().resource::<FirstHourGlance>();
        assert!(!glance.active, "glance stayed active");
        assert_eq!(glance.idle_secs, 0.0);
    }

    fn assert_held(app: &App) {
        let glance = app.world().resource::<FirstHourGlance>();
        assert!(glance.active, "glance dropped without look intent");
        assert!(glance.idle_secs >= IDLE_BEFORE_GLANCE);
    }

    /// An RMB drag clears the glance. A bare right-button hold does not.
    #[test]
    fn glance_cancels_on_rmb_drag() {
        let mut drag = armed_glance();
        {
            let world = drag.world_mut();
            world
                .resource_mut::<ButtonInput<MouseButton>>()
                .press(MouseButton::Right);
            world.resource_mut::<AccumulatedMouseMotion>().delta = Vec2::new(12.0, -4.0);
        }
        drag.update();
        assert_cancelled(&drag);

        let mut held = armed_glance();
        held.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Right);
        held.update();
        assert_held(&held);
    }

    /// A mouse scroll (zoom) clears the glance the same way a drag does.
    #[test]
    fn glance_cancels_on_mouse_scroll() {
        let mut app = armed_glance();
        {
            let mut scroll = app.world_mut().resource_mut::<AccumulatedMouseScroll>();
            scroll.unit = MouseScrollUnit::Line;
            scroll.delta = Vec2::new(0.0, 1.0);
        }
        app.update();
        assert_cancelled(&app);
    }
}
