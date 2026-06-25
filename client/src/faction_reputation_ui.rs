/*!
 * Faction Reputation Visual UI
 *
 * Displays the player's current standing/reputation with factions they belong to.
 * Uses egui for a clean, divine, mercy-aligned interface.
 *
 * Future: Wire to replicated FactionStanding components from server.
 *
 * AG-SML v1.0 | TOLC 8 + PATSAGi Council approved
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

/// Resource holding the local player's faction reputation data.
/// This will later be populated from network replication.
#[derive(Resource, Default, Clone)]
pub struct FactionReputationUIState {
    pub factions: Vec<FactionReputationEntry>,
}

#[derive(Clone, Debug)]
pub struct FactionReputationEntry {
    pub faction_id: u64,
    pub faction_name: String,
    pub standing: f32,      // 0.0 - 5.0+
    pub description: String,
}

pub struct FactionReputationUIPlugin;

impl Plugin for FactionReputationUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<FactionReputationUIState>()
            .add_systems(Update, faction_reputation_window);
    }
}

fn faction_reputation_window(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<FactionReputationUIState>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("⚡ Faction Reputation — Eternal Standing")
        .default_pos([420.0, 80.0])
        .default_size([320.0, 280.0])
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Your Standing in the Eternal Flow");
            ui.separator();

            if ui_state.factions.is_empty() {
                ui.label("You have not yet aligned with any factions.");
                ui.label("Participate in ToFaction distributions to begin building reputation.");
                ui.add_space(8.0);

                if ui.button("Initialize Sample Factions (Dev)").clicked() {
                    ui_state.factions = vec![
                        FactionReputationEntry {
                            faction_id: 1,
                            faction_name: "The Radiant Accord".to_string(),
                            standing: 2.8,
                            description: "Builders of abundance and mercy".to_string(),
                        },
                        FactionReputationEntry {
                            faction_id: 2,
                            faction_name: "The Silent Veil".to_string(),
                            standing: 1.4,
                            description: "Guardians of hidden knowledge".to_string(),
                        },
                    ];
                }
            } else {
                for entry in &ui_state.factions {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.strong(&entry.faction_name);
                            ui.label(format!("(ID: {})", entry.faction_id));
                        });

                        ui.label(&entry.description);

                        // Standing progress bar
                        let progress = (entry.standing / 5.0).clamp(0.0, 1.0);
                        ui.add(egui::ProgressBar::new(progress)
                            .text(format!("Standing: {:.1} / 5.0", entry.standing)));

                        // Color indicator
                        let color = if entry.standing >= 3.5 {
                            egui::Color32::from_rgb(80, 200, 120)  // Green - highly respected
                        } else if entry.standing >= 2.0 {
                            egui::Color32::from_rgb(200, 180, 80)  // Gold - respected
                        } else {
                            egui::Color32::from_rgb(180, 140, 80)  // Warm - neutral
                        };
                        ui.colored_label(color, if entry.standing >= 3.5 { "Highly Respected" } else if entry.standing >= 2.0 { "Respected" } else { "Neutral" });
                    });
                    ui.add_space(6.0);
                }
            }

            ui.separator();
            ui.small("Standing increases when you participate in faction distributions.");
            ui.small("Higher standing = greater rewards from ProportionalToStanding.");
        });
}
