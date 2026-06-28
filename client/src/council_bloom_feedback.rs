/*!
 * Council Bloom Feedback — Toast-style notifications
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::replication::CouncilBloomReceived;

/// A single toast notification
#[derive(Clone)]
pub struct BloomToast {
    pub message: String,
    pub attunement: f32,
    pub timer: Timer,
    pub alpha: f32,
}

/// Resource holding active bloom toasts
#[derive(Resource, Default)]
pub struct BloomToasts {
    pub toasts: Vec<BloomToast>,
}

#[derive(Event, Clone, Debug)]
pub struct CouncilBloomNotification {
    pub message: String,
    pub attunement: f32,
}

pub struct CouncilBloomFeedbackPlugin;

impl Plugin for CouncilBloomFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>()
           .add_event::<CouncilBloomNotification>()
           .init_resource::<BloomToasts>()
           .add_systems(Update, (
               receive_bloom_notifications,
               update_toasts,
               draw_toast_ui,
           ).chain());
    }
}

/// Convert CouncilBloomReceived into a notification toast
fn receive_bloom_notifications(
    mut bloom_events: EventReader<CouncilBloomReceived>,
    mut notification_writer: EventWriter<CouncilBloomNotification>,
    mut toasts: ResMut<BloomToasts>,
) {
    for event in bloom_events.read() {
        if event.payload.bloom_activated {
            let toast = BloomToast {
                message: format!(
                    "Council Bloom Activated — {:.0}% Attunement",
                    event.payload.collective_attunement_score * 100.0
                ),
                attunement: event.payload.collective_attunement_score,
                timer: Timer::from_seconds(4.5, TimerMode::Once),
                alpha: 1.0,
            };

            toasts.toasts.push(toast);

            // Also emit the general notification event
            notification_writer.send(CouncilBloomNotification {
                message: "Council Bloom".to_string(),
                attunement: event.payload.collective_attunement_score,
            });
        }
    }
}

/// Update toast lifetimes and fade
fn update_toasts(
    time: Res<Time>,
    mut toasts: ResMut<BloomToasts>,
) {
    let mut i = 0;
    while i < toasts.toasts.len() {
        let toast = &mut toasts.toasts[i];
        toast.timer.tick(time.delta());

        // Fade out in the last 1.2 seconds
        if toast.timer.remaining_secs() < 1.2 {
            toast.alpha = (toast.timer.remaining_secs() / 1.2).clamp(0.0, 1.0);
        }

        if toast.timer.just_finished() {
            toasts.toasts.remove(i);
        } else {
            i += 1;
        }
    }
}

/// Draw toast-style popups in bottom-right corner
fn draw_toast_ui(
    mut contexts: EguiContexts,
    mut toasts: ResMut<BloomToasts>,
) {
    let ctx = contexts.ctx_mut();

    let screen_rect = ctx.screen_rect();
    let toast_width = 280.0;
    let start_y = screen_rect.max.y - 20.0;

    for (i, toast) in toasts.toasts.iter().enumerate() {
        let y_offset = start_y - (i as f32 * 70.0);

        egui::Window::new(format!("bloom_toast_{}", i))
            .fixed_pos(egui::pos2(screen_rect.max.x - toast_width - 20.0, y_offset))
            .fixed_size(egui::vec2(toast_width, 60.0))
            .frame(egui::Frame::window(&ctx.style()).fill(egui::Color32::from_rgba_unmultiplied(20, 30, 25, (toast.alpha * 220.0) as u8)))
            .title_bar(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.colored_label(
                        egui::Color32::from_rgb(120, 255, 160),
                        &toast.message,
                    );
                    ui.label(format!("Attunement: {:.1}%", toast.attunement * 100.0));
                });
            });
    }
}
