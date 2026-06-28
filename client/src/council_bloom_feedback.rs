/*!
 * Council Bloom Feedback — Severity-aware Dramatic Entrance Animations
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::replication::CouncilBloomReceived;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BloomSeverity {
    Gentle,
    Normal,
    Strong,
    Epiphany,
}

impl BloomSeverity {
    pub fn from_attunement(attunement: f32, amplification: f32) -> Self {
        if attunement > 0.88 && amplification > 1.8 { BloomSeverity::Epiphany }
        else if attunement > 0.78 || amplification > 1.5 { BloomSeverity::Strong }
        else if attunement > 0.6 { BloomSeverity::Normal }
        else { BloomSeverity::Gentle }
    }

    pub fn accent_color(&self) -> egui::Color32 {
        match self {
            BloomSeverity::Epiphany => egui::Color32::from_rgb(255, 215, 100),
            BloomSeverity::Strong   => egui::Color32::from_rgb(120, 255, 160),
            BloomSeverity::Normal   => egui::Color32::from_rgb(100, 200, 255),
            BloomSeverity::Gentle   => egui::Color32::from_rgb(180, 200, 180),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            BloomSeverity::Epiphany => "✦",
            BloomSeverity::Strong   => "❖",
            BloomSeverity::Normal   => "◈",
            BloomSeverity::Gentle   => "◦",
        }
    }

    pub fn duration(&self) -> f32 {
        match self {
            BloomSeverity::Epiphany => 6.5,
            BloomSeverity::Strong   => 5.0,
            _ => 4.0,
        }
    }

    /// How far the toast slides in from the right
    pub fn slide_distance(&self) -> f32 {
        match self {
            BloomSeverity::Epiphany => 160.0,
            BloomSeverity::Strong   => 110.0,
            BloomSeverity::Normal   => 80.0,
            BloomSeverity::Gentle   => 55.0,
        }
    }

    /// How long the entrance animation takes
    pub fn entrance_duration(&self) -> f32 {
        match self {
            BloomSeverity::Epiphany => 0.58,
            BloomSeverity::Strong   => 0.45,
            BloomSeverity::Normal   => 0.35,
            BloomSeverity::Gentle   => 0.28,
        }
    }
}

#[derive(Clone)]
pub struct BloomToast {
    pub message: String,
    pub attunement: f32,
    pub severity: BloomSeverity,
    pub timer: Timer,
    pub alpha: f32,
    pub entrance_progress: f32,
}

#[derive(Resource, Default)]
pub struct BloomToasts {
    pub toasts: Vec<BloomToast>,
}

pub struct CouncilBloomFeedbackPlugin;

impl Plugin for CouncilBloomFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>()
           .init_resource::<BloomToasts>()
           .add_systems(Update, (
               receive_bloom_notifications,
               update_toasts,
               draw_toast_ui,
           ).chain());
    }
}

fn receive_bloom_notifications(
    mut bloom_events: EventReader<CouncilBloomReceived>,
    mut toasts: ResMut<BloomToasts>,
) {
    for event in bloom_events.read() {
        if event.payload.bloom_activated {
            let severity = BloomSeverity::from_attunement(
                event.payload.collective_attunement_score,
                event.payload.bloom_amplification_multiplier,
            );

            let toast = BloomToast {
                message: format!("Council Bloom — {:.0}% Attunement", event.payload.collective_attunement_score * 100.0),
                attunement: event.payload.collective_attunement_score,
                severity,
                timer: Timer::from_seconds(severity.duration(), TimerMode::Once),
                alpha: 1.0,
                entrance_progress: 0.0,
            };

            toasts.toasts.push(toast);
        }
    }
}

fn update_toasts(time: Res<Time>, mut toasts: ResMut<BloomToasts>) {
    let mut i = 0;
    while i < toasts.toasts.len() {
        let toast = &mut toasts.toasts[i];
        toast.timer.tick(time.delta());

        // Advance entrance animation using severity-specific duration
        if toast.entrance_progress < 1.0 {
            let duration = toast.severity.entrance_duration();
            toast.entrance_progress += time.delta_seconds() / duration;
            toast.entrance_progress = toast.entrance_progress.min(1.0);
        }

        // Fade out near end of life
        if toast.timer.remaining_secs() < 1.5 {
            toast.alpha = (toast.timer.remaining_secs() / 1.5).clamp(0.0, 1.0);
        }

        if toast.timer.just_finished() {
            toasts.toasts.remove(i);
        } else {
            i += 1;
        }
    }
}

fn draw_toast_ui(mut contexts: EguiContexts, toasts: Res<BloomToasts>) {
    let ctx = contexts.ctx_mut();
    let screen_rect = ctx.screen_rect();
    let toast_width = 300.0;

    for (i, toast) in toasts.toasts.iter().enumerate() {
        let y = screen_rect.max.y - 25.0 - (i as f32 * 78.0);

        let entrance = toast.entrance_progress;
        let slide = toast.severity.slide_distance();
        let slide_offset = egui::lerp(slide..=0.0, entrance);

        // Extra dramatic scale for Epiphany
        let scale = if toast.severity == BloomSeverity::Epiphany {
            egui::lerp(0.82..=1.0, entrance)
        } else {
            1.0
        };

        let entrance_alpha = egui::lerp(0.0..=1.0, entrance);
        let final_alpha = toast.alpha * entrance_alpha;

        let accent = toast.severity.accent_color();

        let frame = egui::Frame::window(&ctx.style())
            .fill(egui::Color32::from_rgba_unmultiplied(18, 26, 22, (final_alpha * 235.0) as u8))
            .stroke(egui::Stroke::new(1.5, accent))
            .rounding(egui::Rounding::same(8.0));

        let x_pos = screen_rect.max.x - toast_width - 18.0 + slide_offset;

        // For Epiphany we fake a subtle scale by adjusting width slightly
        let effective_width = toast_width * scale;

        egui::Window::new(format!("bloom_toast_{}", i))
            .fixed_pos(egui::pos2(x_pos, y))
            .fixed_size(egui::vec2(effective_width, 68.0))
            .frame(frame)
            .title_bar(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(accent, toast.severity.icon());
                    ui.vertical(|ui| {
                        ui.colored_label(accent, &toast.message);
                        ui.label(format!("Attunement: {:.1}%", toast.attunement * 100.0));
                    });
                });
            });
    }
}
