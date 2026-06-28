/*!
 * Council Bloom Feedback — History Panel
 * Shows recent Council Blooms with severity, attunement and time.
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::replication::CouncilBloomReceived;

// ... existing BloomSeverity, BloomToast, etc. ...

/// A single entry in the bloom history
#[derive(Clone, Debug)]
pub struct BloomHistoryEntry {
    pub timestamp: f64,           // seconds since startup
    pub message: String,
    pub attunement: f32,
    pub severity: BloomSeverity,
}

/// Resource storing recent Council Bloom history
#[derive(Resource, Default)]
pub struct BloomHistory {
    pub entries: Vec<BloomHistoryEntry>,
    pub max_entries: usize,
}

impl Default for BloomHistory {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 12,
        }
    }
}

impl BloomHistory {
    pub fn add(&mut self, entry: BloomHistoryEntry) {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }
}

pub struct CouncilBloomFeedbackPlugin;

impl Plugin for CouncilBloomFeedbackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CouncilBloomReceived>()
           .init_resource::<BloomToasts>()
           .init_resource::<BloomHistory>()
           .add_systems(Update, (
               receive_bloom_notifications,
               record_bloom_to_history,
               update_toasts,
               draw_toast_ui,
               draw_bloom_history_panel,
           ).chain());
    }
}

/// Records every activated bloom into the history
fn record_bloom_to_history(
    mut bloom_events: EventReader<CouncilBloomReceived>,
    time: Res<Time>,
    mut history: ResMut<BloomHistory>,
) {
    for event in bloom_events.read() {
        if event.payload.bloom_activated {
            let severity = BloomSeverity::from_attunement(
                event.payload.collective_attunement_score,
                event.payload.bloom_amplification_multiplier,
            );

            let entry = BloomHistoryEntry {
                timestamp: time.elapsed_seconds_f64(),
                message: format!(
                    "Council Bloom — {:.0}% Attunement",
                    event.payload.collective_attunement_score * 100.0
                ),
                attunement: event.payload.collective_attunement_score,
                severity,
            };

            history.add(entry);
        }
    }
}

/// Draws a persistent Bloom History panel (bottom-left by default)
fn draw_bloom_history_panel(
    mut contexts: EguiContexts,
    history: Res<BloomHistory>,
    time: Res<Time>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Council Bloom History")
        .default_pos(egui::pos2(20.0, 300.0))
        .default_size(egui::vec2(260.0, 280.0))
        .resizable(true)
        .collapsible(true)
        .frame(egui::Frame::window(&ctx.style()).fill(egui::Color32::from_rgba_unmultiplied(15, 22, 18, 245)))
        .show(ctx, |ui| {
            ui.label("Recent Council Blooms");
            ui.separator();

            if history.entries.is_empty() {
                ui.label("No blooms recorded yet.");
                return;
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in history.entries.iter().rev() {
                    let age = time.elapsed_seconds_f64() - entry.timestamp;
                    let time_str = if age < 60.0 {
                        format!("{:.0}s ago", age)
                    } else {
                        format!("{:.1}m ago", age / 60.0)
                    };

                    ui.horizontal(|ui| {
                        ui.colored_label(entry.severity.accent_color(), entry.severity.icon());
                        ui.vertical(|ui| {
                            ui.colored_label(entry.severity.accent_color(), &entry.message);
                            ui.label(format!("Attunement: {:.1}%  •  {}", entry.attunement * 100.0, time_str));
                        });
                    });

                    ui.separator();
                }
            });
        });
}
