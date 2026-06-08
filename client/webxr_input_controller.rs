//! client/webxr_input_controller.rs
//! Production-grade WebXR Controller Input Handling
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use bevy::prelude::*;
use web_sys::{XrInputSource, XrSession, XrFrame};
use glam::{Vec3, Quat};
use crate::client_game_loop::ClientInput;
use crate::rbe_client_ui_sync::RbeUiSync;

#[derive(Component)]
pub struct WebXrController {
 pub hand: String, // "left" or "right"
 pub grip_pose: Option<Vec3>,
 pub trigger_pressed: bool,
 pub squeeze_pressed: bool,
}

pub struct WebXrInputControllerPlugin;

impl Plugin for WebXrInputControllerPlugin {
 fn build(&self, app: &mut App) {
 app.add_systems(Update, process_webxr_controller_input);
 }
}

fn process_webxr_controller_input(
 mut query: Query<(&mut WebXrController, &mut ClientGameLoop)>,
 webxr_state: Res<WebXrState>,
 mut rbe_ui: Query<&mut RbeUiSync>,
) {
 let session = match &webxr_state.session {
 Some(s) => s,
 None => return,
 };

 // Get latest frame and input sources
 if let Some(frame) = &webxr_state.frame {
 let inputs = session.get_input_sources();
 for input in inputs {
 let hand = input.hand().unwrap_or("unknown".into());
 let grip_pose = input.grip_space().and_then(|space| frame.get_pose(&space, None));

 if let Some(pose) = grip_pose {
 let position = Vec3::new(pose.position.x, pose.position.y, pose.position.z);
 let rotation = Quat::from_xyzw(pose.orientation.x, pose.orientation.y, pose.orientation.z, pose.orientation.w);

 // Map controller input to game actions
 let trigger = input.gamepad_button("trigger").map(|b| b.pressed).unwrap_or(false);
 let squeeze = input.gamepad_button("squeeze").map(|b| b.pressed).unwrap_or(false);

 for (mut controller, mut game_loop) in query.iter_mut() {
 if controller.hand == hand {
 controller.grip_pose = Some(position);
 controller.trigger_pressed = trigger;
 controller.squeeze_pressed = squeeze;

 // Example: trigger = harvest / interact
 if trigger {
 let input = ClientInput {
 sequence: 0,
 movement: Vec3::ZERO,
 rotation_delta: rotation,
 };
 game_loop.update(0.016, input);
 }
 }
 }
 }
 }
 }
}

// Extension for easy integration
pub trait WebXrAppExt {
 fn with_webxr_input(self) -> Self;
}

impl WebXrAppExt for App {
 fn with_webxr_input(mut self) -> Self {
 self.add_plugin(WebXrInputControllerPlugin)
 }
}
