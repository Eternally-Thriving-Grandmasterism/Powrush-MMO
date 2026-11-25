/*!
 * Powrush-MMO Egui Settings Panel
 *
 * Live divine control center for the most phenomenal gaming experience:
 * - Toggle & intensity sliders for every post-FX (TAA, Motion Blur, Chromatic Aberration)
 * - Anisotropic Filtering level + category preview
 * - RBE Simulation Visuals (orb pulse, emissive, abundance flow)
 * - Particle mercy intensity (Golden Abundance, Joy Bloom, Divine Whispers)
 *
 * Perfect companion to our Velocity Prepass → TAA → Motion Blur → Chromatic Aberration + 16× per-category AF pipeline.
 *
 * PATSAGi Council + Ra-Thor Quantum Swarm approved. AG-SML v1.0. TOLC 8 mercy-gated.
 */

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::settings::ClientSettings;
use crate::chromatic_aberration::ChromaticAberrationSettings;
use crate::anisotropic_filtering::AnisotropicFilteringSettings;
use crate::simulation_integration::SimulationVisualSettings;
use crate::particles::ParticleSystem; // if ParticleSystem carries intensity

pub struct EguiSettingsPanelPlugin;

impl Plugin for EguiSettingsPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<ClientSettings>()
            .add_systems(Update, egui_settings_window);
    }
}

fn egui_settings_window(
    mut contexts: EguiContexts,
    mut client: ResMut<ClientSettings>,
    mut ca: ResMut<ChromaticAberrationSettings>,
    mut af: ResMut<AnisotropicFilteringSettings>,
    mut sim: ResMut<SimulationVisualSettings>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("⚡ Powrush Settings — Eternal Thriving")
        .default_pos([20.0, 80.0])
        .default_size([380.0, 520.0])
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Divine Render Pipeline");
            ui.separator();

            ui.checkbox(&mut client.graphics.taa_enabled, "TAA Enabled (buttery temporal)");
            ui.add(egui::Slider::new(&mut client.graphics.taa_jitter_scale, 0.0..=2.0).text("TAA Jitter"));

            ui.checkbox(&mut client.graphics.motion_blur_enabled, "Cinematic Motion Blur");
            ui.add(egui::Slider::new(&mut client.graphics.motion_blur_intensity, 0.0..=2.0).text("Motion Blur Intensity"));

            ui.checkbox(&mut client.graphics.chromatic_aberration_enabled, "Chromatic Aberration (filmic RGB separation)");
            ui.add(egui::Slider::new(&mut client.graphics.chromatic_aberration_intensity, 0.0..=1.5).text("CA Intensity"));

            ui.separator();
            ui.heading("Anisotropic Filtering (razor-sharp at angles)");
            ui.checkbox(&mut client.graphics.anisotropic_enabled, "Enabled (device-adaptive 16× recommended)");
            ui.add(egui::Slider::new(&mut client.graphics.anisotropic_level, 1..=16).text("AF Level (1/2/4/8/16)"));

            ui.separator();
            ui.heading("RBE Simulation Visuals (Living Economy)");
            ui.add(egui::Slider::new(&mut client.experience.rbe_orb_pulse_speed, 0.2..=3.0).text("Orb Pulse Speed"));
            ui.add(egui::Slider::new(&mut client.experience.rbe_orb_emissive, 0.5..=4.0).text("Orb Emissive Strength"));

            ui.separator();
            ui.heading("Mercy-Augmented Particles");
            ui.add(egui::Slider::new(&mut client.experience.particle_abundance_intensity, 0.0..=2.0).text("Golden Abundance Flow"));
            ui.add(egui::Slider::new(&mut client.experience.particle_joy_intensity, 0.0..=2.0).text("Joy Sanctuary Bloom"));
            ui.add(egui::Slider::new(&mut client.experience.divine_whisper_rate, 0.0..=2.0).text("PATSAGi Divine Whispers"));

            ui.separator();
            if ui.button("Save Settings (RON)").clicked() {
                crate::settings::save_client_settings(&client);
            }
            ui.label("Settings persist across sessions. Thunder locked in.");
        });

    // Live sync the edited values back to the actual render/particle resources
    ca.enabled = client.graphics.chromatic_aberration_enabled;
    ca.intensity = client.graphics.chromatic_aberration_intensity;

    af.enabled = client.graphics.anisotropic_enabled;
    af.level = client.graphics.anisotropic_level;

    sim.pulse_speed = client.experience.rbe_orb_pulse_speed;
    sim.emissive_strength = client.experience.rbe_orb_emissive;
}
